<?php

/**
 * convert.php --root DIR --facts FILE [--seeds FILE] [--report FILE] [--apply]
 *
 * Solves the facts of FilePlugin / BodyPlugin (one Psalm run) and converts the chosen string slots to interned
 * ids in one pass:
 *
 *  1. candidates: every slot path the facts declare string-typed;
 *  2. groups: ties (overrides, promotions, array flows, foreach / destructuring, by-reference arguments, callbacks,
 *     receiver unions) join slot paths that must convert together; a stop anywhere in a group stops it;
 *  3. seeds: names (member-name tokens, class-string / lowercase-string types, the seed file) start groups;
 *     a group whose every incoming value is a converted slot or a name literal converts too (to a fixpoint);
 *  4. edits: every junction whose two sides differ gets `Interner::intern` / `Interner::lookup` (the OrNull
 *     variants when nullable), name literals meeting a converted slot become `Sym::` constants (src) or
 *     `Interner::intern('..')` (tests), comparisons are made in one domain, declarations and docblocks say `int`.
 *
 * Every name string value becomes the id of the same string: the program means the same thing.
 */

declare(strict_types=1);

namespace Psalm\Tools\IdConvert;

require_once __DIR__ . '/TypeStr.php';

$opts = getopt('', ['root:', 'facts:', 'seeds:', 'report:', 'apply', 'explain:', 'dump-conv:']);
$root = rtrim((string) ($opts['root'] ?? ''), '/') . '/';
$facts_file = (string) ($opts['facts'] ?? '');
if ($root === '/' || $facts_file === '') {
    fwrite(STDERR, "usage: convert.php --root DIR --facts FILE [--seeds FILE] [--report FILE] [--apply]\n");
    exit(2);
}

$s = new Solver($root);
$s->load($facts_file);
if (isset($opts['seeds'])) {
    $s->loadSeeds((string) $opts['seeds']);
}
$s->solve();
if (isset($opts['dump-conv'])) {
    file_put_contents((string) $opts['dump-conv'], implode("\n", array_keys($s->conv)) . "\n");
}
if (isset($opts['explain'])) {
    foreach ((array) $opts['explain'] as $x) {
        echo $s->explain((string) $x);
    }
    exit(0);
}
$edits = $s->edits();
$report = $s->report();
if (isset($opts['report'])) {
    file_put_contents((string) $opts['report'], $report);
} else {
    echo $report;
}
if (isset($opts['apply'])) {
    (new Applier($root, $s))->apply($edits);
}

final class Solver
{
    /** @var array<string, array<string, mixed>> slot base => info */
    public array $slots = [];
    /** @var array<string, bool> candidate slot path => nullable */
    public array $cand = [];
    /** @var array<string, string> union-find parent */
    private array $uf = [];
    /** @var array<string, string> slot path => why stopped */
    public array $stopped = [];
    /** @var array<string, string> group root => why seeded */
    public array $seeded = [];
    /** @var array<string, true> converted slot paths */
    public array $conv = [];
    /** @var list<array<string, mixed>> */
    private array $flows = [];
    /** @var list<array<string, mixed>> */
    private array $cmps = [];
    /** @var list<array<string, mixed>> */
    private array $ties = [];
    /** @var list<array<string, mixed>> */
    private array $stops = [];
    /** @var array<string, true> */
    private array $known = [];
    /** @var array<string, true> locals never read */
    private array $unread = [];
    /** @var list<array<string, mixed>> */
    private array $notes = [];
    /** @var array<string, array<int, string>> fn => idx => param slot */
    private array $fn_params = [];
    /** @var list<array{string, string}> */
    private array $tiefns = [];
    /** @var array<string, list<string>> candidate paths below each slot path prefix */
    private array $below = [];
    /** @var list<string> errors of the facts run */
    public array $errors = [];
    /** @var array<string, list<string>> */
    private array $extra_seeds = [];
    /** @var array<string, int> */
    public array $stats = [];

    public function __construct(public string $root)
    {
    }

    public function load(string $file): void
    {
        $seen = [];
        $h = fopen($file, 'r');
        while (($line = fgets($h)) !== false) {
            if (isset($seen[$line])) {
                continue;
            }
            $seen[$line] = true;
            $r = json_decode($line, true);
            if (!is_array($r)) {
                continue;
            }
            if (($r['kind'] ?? null) === 'error') {
                $this->errors[] = $r['msg'] . ' in ' . ($r['file'] ?? '?');
                continue;
            }
            switch ($r['k'] ?? '') {
                case 'slot':
                    $this->addSlot($r);
                    break;
                case 'body':
                    $this->slots[$r['s']]['body'][] = $r['paths'];
                    break;
                case 'tie':
                    $this->ties[] = $r;
                    break;
                case 'tiefn':
                    $this->tiefns[] = [$r['a'], $r['b']];
                    break;
                case 'stop':
                    $this->stops[] = $r;
                    break;
                case 'unread':
                    $this->unread[$r['s']] = true;
                    break;
                case 'known':
                    $this->known[$r['fn']] = true;
                    break;
                case 'f':
                    $this->flows[] = $this->lowered($r, $r);
                    break;
                case 'cmp':
                    foreach ($r['leaves'] as $i => [$side, $leaf]) {
                        $r['leaves'][$i][1] = $this->lowered($leaf, $r);
                    }
                    $this->cmps[] = $r;
                    break;
                default:
                    $this->notes[] = $r;
            }
        }
    }

    /** @var list<array<string, mixed>> `strtolower(x)` calls around a lowercase slot read: gone where x is an id */
    private array $lowers = [];

    /**
     * A leaf `strtolower(x)` of a slot read: of a lowercase slot it is the slot's own value (the call goes when the
     * slot holds ids); otherwise the slot is read as a string and the call's result is a foreign string.
     *
     * @param array<string, mixed> $leaf
     * @param array<string, mixed> $fact
     * @return array<string, mixed>
     */
    private function lowered(array $leaf, array $fact): array
    {
        $lower = $leaf['src']['lower'] ?? null;
        if ($lower === null) {
            return $leaf;
        }
        unset($leaf['src']['lower']);
        // names are case-sensitive (pzoom): a lowered name that stays an id is the name itself
        $dst = (string) ($leaf['dst'] ?? '');
        if ($dst !== '' && $dst !== 'str' && $dst !== 'interp') {
            $this->lowers[] = ['file' => $fact['file'], 's' => $leaf['src']['s'] ?? null, 'dst' => $dst] + $lower;
            $leaf['lower'] = $lower;
            return $leaf;
        }
        if ($leaf['src']['k'] === 's') {
            if ($lower['lc']) {
                $this->lowers[] = ['file' => $fact['file'], 's' => $leaf['src']['s'], 'dst' => null] + $lower;
                return $leaf;
            }
            $this->flows[] = ['k' => 'f', 'dst' => 'str', 'src' => $leaf['src'], 'r' => $lower['arg'], 'nl' => false,
                'file' => $fact['file'], 'fnscope' => $fact['fnscope'] ?? ''];
            $leaf['src'] = ['k' => 'x'];
            $leaf['sc'] = true;
        }
        return $leaf;
    }

    /** @param array<string, mixed> $r */
    private function addSlot(array $r): void
    {
        $base = $r['s'];
        $cur = $this->slots[$base] ?? null;
        if ($cur === null || !isset($cur['paths'])) {
            $this->slots[$base] = $r + ($cur ?? []);
        } else {
            // seen again (a trait method per using class, a re-analysis): the paths every sighting agrees on
            $both = array_intersect_key($cur['paths'], $r['paths']);
            foreach ($both as $k => $nl) {
                $both[$k] = $nl || $r['paths'][$k];
            }
            $this->slots[$base]['paths'] = $both;
            $decl = $cur['decl'] ?? [];
            foreach ($r['decl'] ?? [] as $d) {
                if (!in_array($d, $decl, true)) {
                    $decl[] = $d;
                }
            }
            $this->slots[$base]['decl'] = $decl;
        }
        if (isset($r['fn'], $r['idx']) && str_starts_with($base, 'P:')) {
            $this->fn_params[$r['fn']][$r['idx']] = $base;
        }
    }

    public function loadSeeds(string $file): void
    {
        foreach (file($file, FILE_IGNORE_NEW_LINES) ?: [] as $l) {
            $l = trim($l);
            if ($l === '' || $l[0] === '#') {
                continue;
            }
            $this->extra_seeds[$l][] = 'seed-file';
        }
    }

    // ------------------------------------------------------------------------------------------------ union-find

    private function find(string $x): string
    {
        $r = $x;
        while (($this->uf[$r] ?? $r) !== $r) {
            $r = $this->uf[$r];
        }
        while ($x !== $r) {
            $n = $this->uf[$x] ?? $x;
            $this->uf[$x] = $r;
            $x = $n;
        }
        return $r;
    }

    /** @var array<string, true> group roots holding a slot that cannot hold ids (set before the flow unions) */
    private array $hard_roots = [];

    /**
     * Joins two slot paths. A tie (a binding that cannot be wrapped) always joins; a flow (which could be
     * wrapped) joins only names, and never a group that cannot hold ids: its neighbours meet it at a boundary.
     */
    private function union(string $a, string $b, bool $tie = false): void
    {
        if (!$tie && (isset($this->unnamed[$a]) || isset($this->unnamed[$b])
            || isset($this->hard_roots[$this->find($a)]) || isset($this->hard_roots[$this->find($b)]))
        ) {
            return;
        }
        $ra = $this->find($a);
        $rb = $this->find($b);
        if ($ra !== $rb) {
            $this->uf[$ra] = $rb;
        }
    }

    /** @return array{string, string} [base, path] */
    public static function split(string $slotpath): array
    {
        $i = strpos($slotpath, '#');
        return $i === false ? [$slotpath, ''] : [substr($slotpath, 0, $i), substr($slotpath, $i)];
    }

    /** @return list<string> the path suffixes q with $x.q a candidate */
    private function subs(string $x): array
    {
        return $this->below[$x] ?? [];
    }

    private function stop(string $x, string $why): void
    {
        if (isset($this->cand[$x]) && !isset($this->stopped[$x])) {
            $this->stopped[$x] = $why;
        }
    }

    private function stopSub(string $x, string $why): void
    {
        foreach ($this->subs($x) as $q) {
            $this->stop($x . $q, $why);
        }
    }

    /** Ties two slot paths: their common candidate subpaths join; a subpath on one side only is stopped. */
    private function tie(string $a, string $b, string $why): void
    {
        if (isset($this->unread[self::split($a)[0]]) || isset($this->unread[self::split($b)[0]])) {
            return;
        }
        $qa = $this->subs($a);
        $qb = $this->subs($b);
        foreach (array_unique([...$qa, ...$qb]) as $q) {
            $x = isset($this->cand[$a . $q]);
            $y = isset($this->cand[$b . $q]);
            if ($x && $y) {
                $this->union($a . $q, $b . $q, true);
            } elseif ($x) {
                $this->stop($a . $q, 'tie-mismatch:' . $why . ':' . $b . $q);
            } elseif ($y) {
                $this->stop($b . $q, 'tie-mismatch:' . $why . ':' . $a . $q);
            }
        }
    }

    // ------------------------------------------------------------------------------------------------ solving

    public function solve(): void
    {
        // candidates
        foreach ($this->slots as $base => &$info) {
            $paths = $info['paths'] ?? [];
            foreach ($info['body'] ?? [] as $bp) {
                $paths = array_intersect_key($paths, $bp);
            }
            $info['paths'] = $paths;
            foreach ($paths as $p => $nl) {
                $this->cand[$base . $p] = (bool) $nl;
            }
        }
        unset($info);
        foreach ($this->cand as $x => $_) {
            [$base, $path] = self::split($x);
            // every prefix of the path sees it
            $pre = $base;
            $this->below[$pre][] = $path;
            $rest = $path;
            while ($rest !== '') {
                $pre .= substr($rest, 0, 2);
                $rest = substr($rest, 2);
                $this->below[$pre][] = $rest;
            }
        }
        $this->stats['candidates'] = count($this->cand);

        // fixed stops of declarations
        foreach ($this->slots as $base => $info) {
            if (isset($info['stop'])) {
                $this->stopSub($base, (string) $info['stop']);
            }
            if (!empty($info['tpl'])) {
                $this->stopSub($base, 'template');
            }
            if (!empty($info['variadic'])) {
                $this->stopSub($base, 'variadic');
            }
            if (!empty($info['closure']) && !isset($this->known[(string) ($info['fn'] ?? '')])) {
                $this->stopSub($base, 'closure-unknown-caller');
            }
            $m = (string) ($info['member'] ?? '');
            $rfn = str_starts_with($base, 'R:') ? substr($base, (int) strrpos($base, ':') + 1) : '';
            if (($m !== '' && self::stopName($m))
                || ($rfn !== '' && preg_match('/(id|ids|key|keys|string|strings|text|message|path|paths|file|files|phrase|snippet|label|description|dir|html|json|xml|hash|code|version|value)$|^(to|get)(string|text)/', $rfn))
            ) {
                // not a hard stop: such a group is not seeded by names
                foreach ($info['paths'] ?? [] as $p => $_) {
                    $this->unnamed[$base . $p] = true;
                }
            }
            if (str_starts_with($base, 'F:') && self::outputClass((string) ($info['cls'] ?? ''))) {
                $this->stopSub($base, 'output-class');
            }
        }
        // read-only callers (vendor plugins): what they read or write keeps its strings
        $ro = static fn(array $r): bool => str_contains((string) ($r['file'] ?? ''), '/vendor/');
        foreach ($this->slots as $base => $info) {
            if ($ro($info)) {
                $this->stopSub($base, 'read-only-code');
            }
        }
        foreach ($this->flows as $f) {
            if ($ro($f)) {
                if ($f['src']['k'] === 's') {
                    $this->stopSub((string) $f['src']['s'], 'used-by-read-only-code @' . $this->loc($f));
                }
                if ($f['dst'] !== 'str' && $f['dst'] !== 'interp') {
                    $this->stopSub((string) $f['dst'], 'written-by-read-only-code @' . $this->loc($f));
                }
            }
        }
        foreach ($this->cmps as $c) {
            if ($ro($c)) {
                foreach ($c['leaves'] as [$side, $leaf]) {
                    if ($leaf['src']['k'] === 's') {
                        $this->stopSub((string) $leaf['src']['s'], 'used-by-read-only-code @' . $this->loc($c));
                    }
                }
            }
        }
        foreach ($this->stops as $r) {
            $s = (string) $r['s'];
            if (isset($r['fnall'])) {
                $fn = (string) $r['fnall'];
                $this->stopSub('R:' . $fn, $r['why']);
                foreach ($this->fn_params[$fn] ?? [] as $p) {
                    $this->stopSub($p, $r['why']);
                }
                continue;
            }
            $this->stopSub($s, (string) $r['why']);
        }
        foreach ($this->notes as $n) {
            switch ($n['k']) {
                case 'scopeoff':
                    foreach ($this->slots as $base => $_) {
                        if ((str_starts_with($base, 'L:') || str_starts_with($base, 'P:'))
                            && str_starts_with($base, substr($base, 0, 2) . $n['fn'] . '|')
                        ) {
                            $this->stopSub($base, 'scope:' . $n['why']);
                        }
                    }
                    break;
                case 'arrout':
                    foreach ($this->subs($n['s']) as $q) {
                        if ($q !== '#v') {
                            $this->stop($n['s'] . $q, 'arrout:' . $n['why']);
                        }
                    }
                    break;
            }
        }

        // ties
        foreach ($this->ties as $t) {
            $this->tie((string) $t['a'], (string) $t['b'], (string) $t['why']);
        }
        foreach ($this->tiefns as [$a, $b]) {
            $this->tie('R:' . $a, 'R:' . $b, 'receiver-union');
            foreach ($this->fn_params[$a] ?? [] as $i => $pa) {
                $pb = $this->fn_params[$b][$i] ?? null;
                if ($pb !== null) {
                    $this->tie($pa, $pb, 'receiver-union');
                } else {
                    $this->stopSub($pa, 'receiver-union-arity');
                }
            }
        }
        // local array paths that nothing reads (only written): flows into them constrain nothing
        $used = [];
        foreach ($this->flows as $f) {
            if ($f['src']['k'] === 's') {
                $used[(string) $f['src']['s']] = true;
            }
        }
        foreach ($this->cmps as $c) {
            foreach ($c['leaves'] as [$side, $leaf]) {
                if ($leaf['src']['k'] === 's') {
                    $used[(string) $leaf['src']['s']] = true;
                }
            }
        }
        foreach ($this->ties as $t) {
            if (!isset($this->unread[self::split((string) $t['b'])[0]])) {
                $used[(string) $t['a']] = true;
            }
            if (!isset($this->unread[self::split((string) $t['a'])[0]])) {
                $used[(string) $t['b']] = true;
            }
        }
        $this->used = $used;
        // a declaration's default (a constant expression) cannot call the interner: its slots convert together
        foreach ($this->flows as $f) {
            if (!str_starts_with((string) ($f['fnscope'] ?? ''), 'decl@')) {
                continue;
            }
            $dst = (string) $f['dst'];
            if ($dst === 'str' || $dst === 'interp') {
                if ($f['src']['k'] === 's') {
                    $this->stopSub((string) $f['src']['s'], 'default-to-string');
                }
                continue;
            }
            if ($f['src']['k'] === 's') {
                $this->tie((string) $f['src']['s'], $dst, 'default');
                if (isset($this->cand[$f['src']['s']]) !== isset($this->cand[$dst])) {
                    $this->stop((string) $f['src']['s'], 'default-mismatch');
                    $this->stop($dst, 'default-mismatch');
                } elseif (isset($this->cand[$dst])) {
                    $this->union((string) $f['src']['s'], $dst, true);
                }
            } elseif ($f['src']['k'] === 'x') {
                $this->stopSub($dst, 'default-foreign');
            }
        }
        // flows: array-valued flows between slots are ties; scalar ones are junctions
        foreach ($this->flows as $f) {
            $src = $f['src'];
            $dst = (string) $f['dst'];
            $dslot = $dst !== 'str' && $dst !== 'interp';
            if ($src['k'] === 's') {
                $s = (string) $src['s'];
                if ($dslot) {
                    foreach (array_unique([...$this->subs($s), ...$this->subs($dst)]) as $q) {
                        if ($q === '') {
                            continue;
                        }
                        $x = isset($this->cand[$s . $q]);
                        $y = isset($this->cand[$dst . $q]);
                        if ($x && $y) {
                            $this->union($s . $q, $dst . $q, true);
                        } elseif ($x) {
                            if ($this->unusedLocal($dst . $q)) {
                                continue;
                            }
                            // the top level crosses by a wrap; deeper it cannot
                            $this->stop($s . $q, (strlen($q) > 2 ? 'foreign-deep:' : 'flow-into-foreign:') . $dst . ' @' . $this->loc($f));
                        } elseif ($y) {
                            $this->stop($dst . $q, (strlen($q) > 2 ? 'foreign-deep:' : 'flow-from-foreign:') . $s . ' @' . $this->loc($f));
                        }
                    }
                    // a scalar reaching a slot that holds arrays there (or vice versa) cannot happen with sound types
                } else {
                    foreach ($this->subs($s) as $q) {
                        if (strlen($q) > 2) {
                            $this->stop($s . $q, 'foreign-deep @' . $this->loc($f));
                        }
                    }
                }
            } elseif ($src['k'] === 'x' && $dslot) {
                foreach ($this->subs($dst) as $q) {
                    if ($q === '') {
                        continue;
                    }
                    if (!in_array($q, ['#k', '#v'], true)) {
                        $this->stop($dst . $q, 'foreign-deep @' . $this->loc($f));
                    } elseif (!isset($f['ap'][$q])) {
                        $this->stop($dst . $q, 'foreign-array-in @' . $this->loc($f));
                    }
                }
            }
        }
        // comparisons of arrays: the compared arrays convert together
        foreach ($this->cmps as $c) {
            $arrs = [];
            foreach ($c['leaves'] as [$side, $leaf]) {
                if ($leaf['src']['k'] === 's') {
                    $s = (string) $leaf['src']['s'];
                    if (array_diff($this->subs($s), ['']) !== []) {
                        $arrs[] = $s;
                    }
                }
            }
            for ($i = 1; $i < count($arrs); $i++) {
                $this->tie($arrs[0], $arrs[$i], 'cmp');
            }
        }
        foreach ($this->notes as $n) {
            if ($n['k'] === 'heredoc') {
                $this->heredocs[$n['file'] . ':' . $n['r'][0] . ':' . $n['r'][1]] = true;
            }
        }
        foreach ($this->flows as $f) {
            if (($f['src']['k'] ?? '') === 's' && isset($this->heredocs[$f['file'] . ':' . $f['r'][0] . ':' . $f['r'][1]])) {
                $this->stop((string) $f['src']['s'], 'heredoc');
            }
        }

        // ids everywhere: a name moving between two slots makes both ids; only what cannot hold an id stays a
        // string (the rest of the old stops are breakages the type check of the converted tree reports)
        $this->stopped = array_filter($this->stopped, self::hardStop(...));
        foreach ($this->stopped as $x => $_) {
            $this->hard_roots[$this->find($x)] = true;
        }
        $joinable = fn(string $x): bool => isset($this->cand[$x]) && !isset($this->unnamed[$x]) && !isset($this->stopped[$x]);
        foreach ($this->flows as $f) {
            $dst = (string) $f['dst'];
            if ($f['src']['k'] === 's' && $joinable((string) $f['src']['s']) && $joinable($dst)) {
                $this->union((string) $f['src']['s'], $dst);
            }
        }
        foreach ($this->cmps as $c) {
            $slots = [];
            foreach ($c['leaves'] as [$side, $leaf]) {
                if ($leaf['src']['k'] === 's' && !isset($leaf['src']['alts']) && $joinable((string) $leaf['src']['s'])) {
                    $slots[] = (string) $leaf['src']['s'];
                }
            }
            for ($i = 1; $i < count($slots); $i++) {
                $this->union($slots[0], $slots[$i]);
            }
        }

        // groups
        $groups = [];
        foreach ($this->cand as $x => $_) {
            $groups[$this->find($x)][] = $x;
        }
        $gstop = [];
        foreach ($this->stopped as $x => $why) {
            $gstop[$this->find($x)] ??= $x . ': ' . $why;
        }
        $this->stats['groups'] = count($groups);
        $this->stats['stopped-groups'] = count($gstop);

        // seeds
        foreach ($groups as $g => $members) {
            if (isset($gstop[$g])) {
                continue;
            }
            $unnamed = false;
            foreach ($members as $x) {
                $unnamed = $unnamed || isset($this->unnamed[$x]);
            }
            foreach ($members as $x) {
                $why = $this->seedWhy($x);
                if ($why !== null && (!$unnamed || !str_starts_with($why, 'name:'))) {
                    $this->seeded[$g] = $x . ': ' . $why;
                    break;
                }
            }
        }
        // the partition: seeded groups hold ids, stopped / string-named groups hold strings; every other group
        // goes to the side that needs the fewest intern / lookup sites (a minimum cut of the flow graph)
        $conv_groups = $this->seeded;
        foreach ($groups as $g => $members) {
            if (isset($conv_groups[$g]) && !isset($gstop[$g])) {
                foreach ($members as $x) {
                    $this->conv[$x] = true;
                }
            }
        }
        // a constant expression (a default, a constant's value) cannot call the interner: its two ends agree
        do {
            $changed = false;
            foreach ($this->flows as $f) {
                if (!str_starts_with((string) ($f['fnscope'] ?? ''), 'decl@')) {
                    continue;
                }
                $dst = (string) $f['dst'];
                $d = isset($this->cand[$dst]) && isset($this->conv[$dst]);
                $src = $f['src'];
                if ($src['k'] === 's') {
                    $x = (string) $src['s'];
                    $sc = isset($this->cand[$x]) && isset($this->conv[$x]);
                    if ($sc === $d) {
                        continue;
                    }
                    $drop = $sc ? $x : $dst;
                } elseif ($src['k'] === 'x' && $d) {
                    $drop = $dst;
                } else {
                    continue;
                }
                foreach ($groups[$this->find($drop)] ?? [$drop] as $m) {
                    unset($this->conv[$m]);
                }
                $changed = true;
            }
        } while ($changed);
        $this->groups = $groups;
        $this->gstop = $gstop;
        $this->conv_groups = $conv_groups;
        $this->stats['converted'] = count($this->conv);
        $this->stats['converted-groups'] = count(array_filter(array_keys($conv_groups), fn($g) => !isset($gstop[$g])));
    }

    /**
     * Groups on the id side of a minimum cut. Nodes are groups; an edge per value that would need a wrap if its
     * two ends differed (scalar flows and comparisons between slots); a value meeting a string (a string consumer,
     * a foreign string, an unconverted slot) is an edge to the string side. Seeds hang on the source, stopped and
     * string-named groups on the sink.
     *
     * @param array<string, list<string>> $groups
     * @param array<string, string> $gstop
     * @return array<string, string>
     */
    private function minCut(array $groups, array $gstop): array
    {
        $INF = 1 << 40;
        $id = ['#S' => 0, '#T' => 1];
        foreach ($groups as $g => $_) {
            $id[$g] = count($id);
        }
        $n = count($id);
        $cap = [];
        $add = function (int $a, int $b, int $c, bool $both = true) use (&$cap): void {
            if ($a === $b) {
                return;
            }
            $cap[$a][$b] = ($cap[$a][$b] ?? 0) + $c;
            $cap[$b][$a] = ($cap[$b][$a] ?? 0) + ($both ? $c : 0);
        };
        $node = function (string $x) use ($id): ?int {
            return isset($this->cand[$x]) ? $id[$this->find($x)] : null;
        };
        foreach ($this->flows as $f) {
            $dst = (string) $f['dst'];
            $src = $f['src'];
            $d = ($dst !== 'str' && $dst !== 'interp') ? $node($dst) : null;
            $sn = $src['k'] === 's' ? $node((string) $src['s']) : null;
            if ($src['k'] === 'lit' || $src['k'] === 'null') {
                continue; // a literal is a Sym constant on either side
            }
            if ($sn !== null && $d !== null) {
                $add($sn, $d, 1);
            } elseif ($sn !== null) {
                $add($sn, 1, 1);
            } elseif ($d !== null && ($f['sc'] ?? null) !== false) {
                $add($d, 1, 1);
            }
        }
        foreach ($this->cmps as $c) {
            $nodes = [];
            $strings = 0;
            foreach ($c['leaves'] as [$side, $leaf]) {
                $k = $leaf['src']['k'];
                if ($k === 'lit' || $k === 'null') {
                    continue;
                }
                $x = $k === 's' ? $node((string) $leaf['src']['s']) : null;
                if ($x === null) {
                    $strings++;
                } else {
                    $nodes[] = $x;
                }
            }
            for ($i = 1; $i < count($nodes); $i++) {
                $add($nodes[0], $nodes[$i], 1);
            }
            if ($strings > 0) {
                foreach ($nodes as $x) {
                    $add($x, 1, 1);
                }
            }
        }
        $why = [];
        foreach ($groups as $g => $members) {
            if (isset($gstop[$g])) {
                $add($id[$g], 1, $INF);
                continue;
            }
            $unnamed = false;
            foreach ($members as $x) {
                $unnamed = $unnamed || isset($this->unnamed[$x]);
            }
            if (isset($this->seeded[$g]) && (!$unnamed || !str_contains($this->seeded[$g], ': name:'))) {
                $add(0, $id[$g], $INF, false);
                $why[$g] = $this->seeded[$g];
            } elseif ($unnamed) {
                $add($id[$g], 1, $INF);
            }
        }
        // Dinic
        $flow = 0;
        while (true) {
            $level = array_fill(0, $n, -1);
            $level[0] = 0;
            $q = [0];
            for ($h = 0; $h < count($q); $h++) {
                $u = $q[$h];
                foreach ($cap[$u] ?? [] as $v => $c) {
                    if ($c > 0 && $level[$v] < 0) {
                        $level[$v] = $level[$u] + 1;
                        $q[] = $v;
                    }
                }
            }
            if ($level[1] < 0) {
                break;
            }
            $it = [];
            foreach ($cap as $u => $row) {
                $it[$u] = array_keys($row);
            }
            $ptr = array_fill(0, $n, 0);
            // iterative DFS pushing unit-ish flow along level edges
            while (true) {
                $path = [0];
                $found = false;
                while ($path !== []) {
                    $u = end($path);
                    if ($u === 1) {
                        $found = true;
                        break;
                    }
                    $adv = false;
                    while ($ptr[$u] < count($it[$u] ?? [])) {
                        $v = $it[$u][$ptr[$u]];
                        if (($cap[$u][$v] ?? 0) > 0 && $level[$v] === $level[$u] + 1) {
                            $path[] = $v;
                            $adv = true;
                            break;
                        }
                        $ptr[$u]++;
                    }
                    if (!$adv) {
                        array_pop($path);
                        $level[$u] = -1;
                        if ($path !== []) {
                            $ptr[end($path)]++;
                        }
                    }
                }
                if (!$found) {
                    break;
                }
                $b = PHP_INT_MAX;
                for ($i = 0; $i + 1 < count($path); $i++) {
                    $b = min($b, $cap[$path[$i]][$path[$i + 1]]);
                }
                for ($i = 0; $i + 1 < count($path); $i++) {
                    $cap[$path[$i]][$path[$i + 1]] -= $b;
                    $cap[$path[$i + 1]][$path[$i]] = ($cap[$path[$i + 1]][$path[$i]] ?? 0) + $b;
                }
                $flow += $b;
                if ($flow >= $INF) {
                    break 2;
                }
            }
        }
        $this->stats['cut-sites'] = $flow;
        // the source side
        $seen = [0 => true];
        $q = [0];
        for ($h = 0; $h < count($q); $h++) {
            foreach ($cap[$q[$h]] ?? [] as $v => $c) {
                if ($c > 0 && !isset($seen[$v])) {
                    $seen[$v] = true;
                    $q[] = $v;
                }
            }
        }
        $out = [];
        foreach ($groups as $g => $_) {
            if (isset($seen[$id[$g]]) && !isset($gstop[$g])) {
                $out[$g] = $why[$g] ?? 'cut';
            }
        }
        return $out;
    }

    /** @var array<string, true> */
    private array $heredocs = [];
    /** @var array<string, true> slot paths whose names say they are not identifiers */
    private array $unnamed = [];
    /** @var array<string, list<string>> */
    public array $groups = [];
    /** @var array<string, string> */
    public array $gstop = [];
    /** @var array<string, string> */
    public array $conv_groups = [];

    private const NAME_TOKENS = ['class', 'classes', 'classlike', 'classlikes', 'fqcln', 'fqcn', 'interface', 'interfaces',
        'trait', 'traits', 'enum', 'method', 'methods', 'function', 'functions', 'property', 'properties', 'prop',
        'props', 'constant', 'constants', 'const', 'parent', 'parents', 'mixin', 'mixins', 'extends', 'implements'];

    private const STOP_TOKENS = ['file', 'files', 'path', 'paths', 'dir', 'directory', 'message', 'msg', 'text',
        'code', 'doc', 'docblock', 'description', 'var', 'vars', 'key', 'keys', 'hash', 'snippet', 'content',
        'contents', 'string', 'str', 'prefix', 'suffix', 'pattern', 'regex', 'format', 'type', 'types', 'version',
        'reason', 'comment', 'tag', 'tags', 'label', 'title', 'html', 'json', 'xml', 'sql', 'url', 'uri', 'mode',
        'root', 'cwd', 'signature', 'source', 'stub', 'line', 'offset', 'column', 'id', 'ids', 'output', 'input',
        'php', 'extension', 'issue', 'error', 'value', 'values', 'default', 'expression', 'expr', 'token', 'tokens',
        'char', 'chars', 'selected', 'body', 'replacement', 'replacements', 'template', 'templates', 'annotation',
        'dupe', 'shepherd', 'cache', 'config', 'option', 'options', 'arg', 'argv', 'command', 'binary', 'command'];

    private const HARD_STOPS = ['template', 'magic', 'overrides-external', 'unknown-parent', 'output-class', 'variadic',
        'closure-unknown-caller', 'scope:', 'reference', 'global', 'static', 'catch', 'by-ref:', 'heredoc',
        'read-only', 'used-by-read-only', 'written-by-read-only', 'first-class-callable', 'concat-assign',
        'receiver-union-arity', 'default-', 'foreach-unknown', 'destructure-unknown', 'unpacked-arg', 'callback-param', 'tie-mismatch'];

    /** A reason a slot cannot hold an id at all (not merely a place the converted tree breaks). */
    public static function hardStop(string $why): bool
    {
        foreach (self::HARD_STOPS as $h) {
            if (str_starts_with($why, $h)) {
                return true;
            }
        }
        return false;
    }

    /** Names whose values are not identifiers (paths, messages, source text, composite ids). */
    public static function stopName(string $m): bool
    {
        $tokens = self::tokens($m);
        // a composite id ("A::b", "$var") is no name; otherwise a name word wins over the other words
        if (in_array('id', $tokens, true) || in_array('ids', $tokens, true)) {
            return true;
        }
        if (array_intersect($tokens, [...self::NAME_TOKENS, 'fqcln', 'fqcn', 'name', 'names']) !== []) {
            return false;
        }
        foreach ($tokens as $t) {
            if (in_array($t, self::STOP_TOKENS, true)) {
                return true;
            }
        }
        return false;
    }

    /** @return list<string> */
    private static function tokens(string $m): array
    {
        $m = (string) preg_replace('/(?<=[a-z0-9])(?=[A-Z])/', '_', $m);
        return array_values(array_filter(explode('_', strtolower($m)), static fn(string $t): bool => $t !== ''));
    }

    private function seedWhy(string $x): ?string
    {
        [$base, $path] = self::split($x);
        foreach ($this->extra_seeds as $pat => $_) {
            if ($pat === $x || $pat === $base || (str_starts_with($pat, '/') && preg_match($pat, $x))) {
                return 'seed-file';
            }
        }
        $info = $this->slots[$base] ?? [];
        if ($path === '' && !empty($info['classy'])) {
            return 'class-string';
        }
        if (strlen($path) > 2) {
            return null; // a name says what a slot, its keys or its values are, not what is nested deeper
        }
        $m = (string) ($info['member'] ?? '');
        if (str_starts_with($base, 'R:')) {
            $m = substr($base, (int) strrpos($base, ':') + 1);
        }
        if ($m === '' || str_starts_with($base, 'E:')) {
            return null;
        }
        $tokens = self::tokens($m);
        $hits = array_intersect($tokens, self::NAME_TOKENS);
        if ($hits === []) {
            return null;
        }
        $last = end($tokens);
        // `…_name(s)`, `…_class`, `fq_…`, `…_lc`: a name
        if (in_array($last, [...self::NAME_TOKENS, 'name', 'names', 'lc', 'fqcln'], true)
            || in_array('name', $tokens, true) || in_array('names', $tokens, true)
        ) {
            return 'name:' . implode('_', $hits);
        }
        return null;
    }

    public static function nameLiteral(string $v): bool
    {
        return (bool) preg_match('/^\\\\?[A-Za-z_\x80-\xff][A-Za-z0-9_\x80-\xff]*(\\\\[A-Za-z_\x80-\xff][A-Za-z0-9_\x80-\xff]*)*$/', $v)
            || $v === '';
    }

    /** Classes whose properties are output (JSON / reports / the language server protocol): their strings stay. */
    private static function outputClass(string $cls): bool
    {
        return str_starts_with($cls, 'psalm\\internal\\languageserver\\') || str_starts_with($cls, 'psalm\\report')
            || $cls === 'psalm\\internal\\analyzer\\issuedata' || str_starts_with($cls, 'psalm\\issue\\')
            || str_starts_with($cls, 'psalm\\internal\\provider\\') && str_contains($cls, 'cache')
            || str_starts_with($cls, 'psalm\\progress\\') || str_starts_with($cls, 'psalm\\internal\\cli\\')
            || str_starts_with($cls, 'psalm\\internal\\pluginmanager\\')
            || str_starts_with($cls, 'psalm\\sourcecontrol\\') || str_starts_with($cls, 'psalm\\codelocation');
    }

    /** Why a slot path is (not) converted: its group, the group's stops and seeds. */
    public function explain(string $x): string
    {
        if (!isset($this->cand[$x])) {
            $near = array_filter(array_keys($this->cand), static fn(string $c): bool => str_starts_with($c, $x));
            return "$x: not a candidate" . ($near !== [] ? ' (candidates below: ' . implode(', ', array_slice($near, 0, 5)) . ')' : '')
                . (isset($this->slots[self::split($x)[0]]) ? ' paths=' . json_encode($this->slots[self::split($x)[0]]['paths'] ?? null) : ' (no slot)') . "\n";
        }
        $g = $this->find($x);
        $members = $this->groups[$g] ?? [$x];
        $o = "$x: " . ($this->isConv($x) ? 'CONVERTED' : 'kept') . ' group of ' . count($members)
            . ($this->seeded[$g] ?? null ? ' seeded by ' . $this->seeded[$g] : '')
            . (isset($this->conv_groups[$g]) && !isset($this->seeded[$g]) ? ' (' . $this->conv_groups[$g] . ')' : '') . "\n";
        foreach ($members as $m) {
            if (isset($this->stopped[$m])) {
                $o .= "   stop $m: {$this->stopped[$m]}\n";
            }
        }
        foreach (array_slice($members, 0, 25) as $m) {
            $o .= "   member $m\n";
        }
        return $o;
    }

    public function isConv(string $x): bool
    {
        return isset($this->conv[$x]);
    }

    // ------------------------------------------------------------------------------------------------ edits

    /**
     * @return list<array<string, mixed>> edits: ['file', 's', 'e', 'kind' => wrap|replace, 'pre', 'post', 'text']
     */
    public function edits(): array
    {
        $out = [];
        $co = false;
        $wrap = function (string $file, array $r, string $fn, bool $nl, ?string $ctx = null) use (&$out, &$co): void {
            // the left operand of `??` may be undefined: `lookupOrNull($x ?? null)`
            $out[] = ['file' => $file, 's' => $r[0], 'e' => $r[1], 'kind' => 'wrap',
                'pre' => 'Interner::' . $fn . ($nl || $co ? 'OrNull' : '') . '(', 'post' => $co ? ' ?? null)' : ')', 'ctx' => $ctx];
        };
        $lit = function (string $file, array $r, string $v, bool $cls) use (&$out): void {
            $out[] = ['file' => $file, 's' => $r[0], 'e' => $r[1], 'kind' => 'lit', 'v' => ltrim($v, '\\'),
                'cls' => $cls];
        };
        foreach ($this->flows as $f) {
            $file = (string) $f['file'];
            $src = $f['src'];
            $dst = (string) $f['dst'];
            $dconv = $dst !== 'str' && $dst !== 'interp' && isset($this->cand[$dst]) && $this->isConv($dst);
            $nl = (bool) ($f['nl'] ?? true);
            $co = (bool) ($f['co'] ?? false);
            if ($src['k'] === 's' && isset($src['alts']) && ($d = $this->dispatch($src, $dst, $dconv, $nl)) !== null) {
                if ($d !== '') {
                    $out[] = ['file' => $file, 's' => $f['r'][0], 'e' => $f['r'][1], 'kind' => 'dispatch', 'text' => $d,
                        'interp' => $dst === 'interp', 'brace' => (bool) ($f['brace'] ?? false)];
                }
                continue;
            }
            switch ($src['k']) {
                case 's':
                    $s = (string) $src['s'];
                    if (isset($f['lower']) && isset($this->cand[$s])) {
                        // strtolower(x) into a slot: gone where the value stays an id; else x is read as a string
                        $sconv = $this->isConv($s);
                        if (!$dconv && !($sconv && $f['lower']['lc'])) {
                            if ($sconv) {
                                $wrap($file, $f['lower']['arg'], 'lookup', false);
                            }
                            break;
                        }
                    }
                    if (isset($this->cand[$s])) {
                        $sconv = $this->isConv($s);
                        if ($sconv && !$dconv) {
                            if ($dst === 'interp') {
                                $out[] = ['file' => $file, 's' => $f['r'][0], 'e' => $f['r'][1], 'kind' => 'interp',
                                    'brace' => (bool) ($f['brace'] ?? false), 'nl' => $nl];
                            } elseif ($dst === 'str' || isset($this->cand[$dst]) || !$this->hasSubs($dst)) {
                                $wrap($file, $f['r'], 'lookup', $nl, $f['ctx'] ?? null);
                            }
                        } elseif (!$sconv && $dconv) {
                            $wrap($file, $f['r'], 'intern', $nl);
                        }
                        break;
                    }
                    if (!$this->hasSubs($s) && $dconv && ($f['sc'] ?? null) !== false) {
                        // a string from a slot that is no candidate (a shape entry, a mixed array): foreign
                        $wrap($file, $f['r'], 'intern', $nl);
                        break;
                    }
                    // an array slot meeting a foreign array / a string consumer: its top-level keys / values cross
                    $this->arrayBoundary($out, $f, $s, $dst, $co);
                    break;
                case 'lit':
                    if ($dconv) {
                        $lit($file, $f['r'], (string) $src['v'], str_starts_with((string) ($f['fnscope'] ?? ''), 'decl@'));
                    }
                    break;
                case 'x':
                    if ($dconv && ($f['sc'] ?? null) !== false) {
                        $wrap($file, $f['r'], 'intern', $nl);
                    } elseif ($dst !== 'str' && $dst !== 'interp') {
                        $this->arrayBoundary($out, $f, null, $dst, $co);
                    }
                    break;
            }
        }
        $co = false;
        foreach ($this->cmps as $c) {
            $conv = false;
            $foreign = false;
            foreach ($c['leaves'] as [$side, $leaf]) {
                $src = $leaf['src'];
                if ($src['k'] === 's' && isset($src['alts']) && $this->dispatch($src, 'str', false, false) !== null) {
                    $conv = true;
                    $foreign = true;
                    continue;
                }
                if ($src['k'] === 's' && isset($this->cand[$src['s']])) {
                    if ($this->isConv((string) $src['s'])) {
                        $conv = true;
                    } else {
                        $foreign = true;
                    }
                } elseif ($src['k'] === 'x' || ($src['k'] === 's')) {
                    $foreign = true;
                }
            }
            if (!$conv) {
                continue;
            }
            $file = (string) $c['file'];
            foreach ($c['leaves'] as [$side, $leaf]) {
                $src = $leaf['src'];
                if ($src['k'] === 's' && isset($src['alts']) && ($d = $this->dispatch($src, 'str', false, false)) !== null) {
                    $out[] = ['file' => $file, 's' => $leaf['r'][0], 'e' => $leaf['r'][1], 'kind' => 'dispatch', 'text' => $d,
                        'interp' => false, 'brace' => false];
                    continue;
                }
                $isconv = $src['k'] === 's' && isset($this->cand[$src['s']]) && $this->isConv((string) $src['s']);
                if ($foreign) {
                    // compared as strings
                    if ($isconv) {
                        $wrap($file, $leaf['r'], 'lookup', (bool) $leaf['nl']);
                    }
                } elseif ($src['k'] === 'lit') {
                    $lit($file, $leaf['r'], (string) $src['v'], false);
                }
            }
        }
        foreach ($this->lowers as $l) {
            $sconv = $l['s'] !== null && isset($this->cand[$l['s']]) && $this->isConv((string) $l['s']);
            $dconv = $l['dst'] !== null && isset($this->cand[$l['dst']]) && $this->isConv((string) $l['dst']);
            // the lowering goes where the value lands in an id slot (names are case-sensitive, as in pzoom), and
            // where an id of a lowercase name is lowered again
            if ($dconv || ($sconv && $l['lc'])) {
                $out[] = ['file' => $l['file'], 's' => $l['call'][0], 'e' => $l['arg'][0], 'kind' => 'replace', 'text' => ''];
                $out[] = ['file' => $l['file'], 's' => $l['arg'][1], 'e' => $l['call'][1], 'kind' => 'replace', 'text' => ''];
            }
        }
        foreach ($this->notes as $n) {
            if ($n['k'] === 'unpack' && $n['ps'] !== []) {
                $pc = array_map(fn(string $p): bool => isset($this->cand[$p]) && $this->isConv($p), $n['ps']);
                $vc = $n['src']['k'] === 's' && isset($this->cand[$n['src']['s'] . '#v']) && $this->isConv($n['src']['s'] . '#v');
                if (count(array_unique($pc)) === 1 && $pc[0] !== $vc) {
                    $out[] = ['file' => $n['file'], 's' => $n['r'][0], 'e' => $n['r'][1], 'kind' => 'wrap',
                        'pre' => 'Interner::' . ($pc[0] ? 'intern' : 'lookup') . 'List(', 'post' => ')'];
                }
            }
            if ($n['k'] === 'spread' && $this->isConv($n['s'] . '#k')) {
                $open = $this->srcAt((string) $n['file'], $n['r'][0], 1) === '[' ? 1 : 6; // `[` or `array(`
                $out[] = ['file' => $n['file'], 's' => $n['r'][0], 'e' => $n['r'][0] + $open, 'kind' => 'replace', 'text' => 'array_replace('];
                foreach ($n['items'] as $at) {
                    $out[] = ['file' => $n['file'], 's' => $at, 'e' => $at + 3, 'kind' => 'replace', 'text' => ''];
                }
                $out[] = ['file' => $n['file'], 's' => $n['r'][1] - 1, 'e' => $n['r'][1], 'kind' => 'replace', 'text' => ')'];
            }
            if ($n['k'] === 'merge' && $this->isConv($n['s'] . '#k')) {
                $out[] = ['file' => $n['file'], 's' => $n['r'][0], 'e' => $n['r'][1], 'kind' => 'rename'];
            }
            if ($n['k'] === 'arrout' && isset($this->cand[$n['s'] . '#v']) && $this->isConv($n['s'] . '#v')) {
                $out[] = ['file' => $n['file'], 's' => $n['r'][0], 'e' => $n['r'][1], 'kind' => 'wrap',
                    'pre' => 'Interner::lookupList(',
                    'post' => ')'];
            }
        }
        // declarations
        foreach ($this->slots as $base => $info) {
            $paths = [];
            foreach ($info['paths'] ?? [] as $p => $_) {
                if ($this->isConv($base . $p)) {
                    $paths[] = $p;
                }
            }
            if ($paths === []) {
                continue;
            }
            foreach ($info['decl'] ?? [] as $d) {
                $text = (string) $d['text'];
                $tedits = [];
                foreach ($paths as $p) {
                    try {
                        foreach (TypeStr::toIntAt($text, $p) as $te) {
                            $tedits[] = $te;
                        }
                    } catch (\Throwable $e) {
                        $this->errors[] = 'type parse: ' . $text . ' (' . $e->getMessage() . ')';
                    }
                }
                if ($tedits === []) {
                    continue;
                }
                $new = TypeStr::apply($text, array_values(array_unique($tedits, SORT_REGULAR)));
                if ($new !== $text) {
                    $out[] = ['file' => $info['file'], 's' => $d['r'][0], 'e' => $d['r'][1], 'kind' => 'replace',
                        'text' => $new, 'old' => $text];
                }
            }
        }
        return $out;
    }

    /** @var array<string, true> slot paths read somewhere */
    private array $used = [];

    /** A local's path (or a prefix of it) never read. */
    private function unusedLocal(string $x): bool
    {
        if (!str_starts_with($x, 'L:')) {
            return false;
        }
        [$base, $path] = self::split($x);
        $pre = $base;
        if (isset($this->used[$pre])) {
            return false;
        }
        while ($path !== '') {
            $pre .= substr($path, 0, 2);
            $path = substr($path, 2);
            if (isset($this->used[$pre])) {
                return false;
            }
        }
        return true;
    }

    /** @param array<string, mixed> $f */
    private function loc(array $f): string
    {
        $file = (string) ($f['file'] ?? '?');
        $rel = str_starts_with($file, $this->root) ? substr($file, strlen($this->root)) : $file;
        $line = 0;
        if (isset($f['r'][0]) && is_file($file)) {
            $this->src_cache[$file] ??= (string) file_get_contents($file);
            $line = substr_count($this->src_cache[$file], "\n", 0, min((int) $f['r'][0], strlen($this->src_cache[$file]))) + 1;
        }
        return $rel . ':' . $line;
    }

    /** @var array<string, string> */
    private array $src_cache = [];

    private function srcAt(string $file, int $at, int $len): string
    {
        $this->src_cache[$file] ??= (string) file_get_contents($file);
        return substr($this->src_cache[$file], $at, $len);
    }

    /**
     * A read through a union receiver whose classes' slots decide differently: `(R instanceof A ? X : Y)` with
     * each branch meeting the consumer. Null when the alternatives agree (the plain rules apply); '' when no
     * edit is needed.
     *
     * @param array<string, mixed> $src
     */
    private function dispatch(array $src, string $dst, bool $dconv, bool $nl): ?string
    {
        $decisions = [];
        foreach ($src['alts'] as $slot => $classes) {
            $decisions[$slot] = isset($this->cand[$slot]) && $this->isConv((string) $slot);
        }
        if (count(array_unique($decisions)) <= 1) {
            return null;
        }
        $conv_classes = [];
        foreach ($src['alts'] as $slot => $classes) {
            if ($decisions[$slot]) {
                $conv_classes = [...$conv_classes, ...$classes];
            }
        }
        $recv = (string) $src['recv'];
        $test = implode(' || ', array_map(static fn(string $c): string => $recv . ' instanceof \\' . ltrim($c, '\\'), $conv_classes));
        return $test . "\0" . ($dconv ? 'intern' : 'lookup');
    }

    /**
     * The top-level keys / values of an array crossing between an id slot and a string one (a foreign array, a
     * string consumer): `Interner::internKeys / internList / lookupKeys / lookupList` around the value.
     *
     * @param list<array<string, mixed>> $out
     * @param array<string, mixed> $f
     */
    private function arrayBoundary(array &$out, array $f, ?string $s, string $dst, bool $co): void
    {
        $toStr = $dst === 'str' || $dst === 'interp';
        $in = [];
        $out_paths = [];
        $qs = array_unique([...($s !== null ? $this->subs($s) : array_keys($f['ap'] ?? [])), ...(!$toStr ? $this->subs($dst) : [])]);
        foreach ($qs as $q) {
            if ($q === '') {
                continue;
            }
            $sc = $s !== null && isset($this->cand[$s . $q]) && $this->isConv($s . $q);
            $dc = !$toStr && isset($this->cand[$dst . $q]) && $this->isConv($dst . $q);
            if ($sc === $dc) {
                continue;
            }
            if (!$sc && $s === null && !isset($f['ap'][$q])) {
                continue; // no strings there
            }
            if ($sc) {
                $out_paths[] = $q;
            } else {
                $in[] = $q;
            }
        }
        $pre = '';
        $post = '';
        foreach (['lookup' => $out_paths, 'intern' => $in] as $fn => $paths) {
            if ($paths === []) {
                continue;
            }
            sort($paths);
            if ($paths === ['#v']) {
                $pre = "Interner::{$fn}List(" . $pre;
                $post .= ')';
            } elseif ($paths === ['#k']) {
                $pre = "Interner::{$fn}Keys(" . $pre;
                $post .= ')';
            } else {
                $pre = "Interner::{$fn}At(" . $pre;
                $post .= ', ' . implode(', ', array_map(static fn(string $p): string => var_export($p, true), $paths)) . ')';
            }
        }
        if ($pre === '') {
            return;
        }
        if ($co) {
            $post = ' ?? []' . $post;
        }
        $out[] = ['file' => $f['file'], 's' => $f['r'][0], 'e' => $f['r'][1], 'kind' => 'wrap', 'pre' => $pre, 'post' => $post];
    }

    private function hasSubs(string $x): bool
    {
        return $this->subs($x) !== [];
    }

    /** @param list<string> $allowed */
    private function hasSubsBeyond(string $x, array $allowed): bool
    {
        foreach ($this->subs($x) as $q) {
            if ($q !== '' && !in_array($q, $allowed, true) && $this->isConv($x . $q)) {
                return true;
            }
        }
        return false;
    }

    // ------------------------------------------------------------------------------------------------ report

    public function report(): string
    {
        $o = "== stats\n";
        foreach ($this->stats as $k => $v) {
            $o .= "$k: $v\n";
        }
        $o .= 'errors: ' . count($this->errors) . "\n";
        foreach (array_slice(array_unique(array_map(static fn(string $e): string => substr($e, 0, 200), $this->errors)), 0, 30) as $e) {
            $o .= "  $e\n";
        }
        $unres = [];
        foreach ($this->notes as $n) {
            if ($n['k'] === 'unres') {
                $unres[$n['m'] . ':' . ($n['name'] ?? '?')] = ($unres[$n['m'] . ':' . ($n['name'] ?? '?')] ?? 0) + 1;
            }
        }
        arsort($unres);
        $o .= "== unresolved members (top)\n";
        foreach (array_slice($unres, 0, 40, true) as $k => $v) {
            $o .= "  $v $k\n";
        }
        // the wrap sites by the operation that forces them
        $lk = [];
        $in = [];
        foreach ($this->flows as $f) {
            $src = $f['src'];
            $dst = (string) $f['dst'];
            $dconv = $dst !== 'str' && $dst !== 'interp' && isset($this->cand[$dst]) && $this->isConv($dst);
            if ($src['k'] === 's' && isset($this->cand[$src['s']]) && $this->isConv((string) $src['s']) && !$dconv) {
                $k = ($dst === 'str' || $dst === 'interp' ? $dst : 'slot') . ' via ' . ($f['via'] ?? '?');
                $lk[$k] = ($lk[$k] ?? 0) + 1;
            } elseif ($dconv && ($src['k'] === 'x' || ($src['k'] === 's' && (!isset($this->cand[$src['s']]) || !$this->isConv((string) $src['s']))))) {
                $k = $src['k'] === 'x' ? 'x:' . ($f['xk'] ?? '?') : 'slot:' . substr((string) $src['s'], 0, 2);
                $in[$k] = ($in[$k] ?? 0) + 1;
            }
        }
        // what keeps the unconverted side of a slot-to-slot boundary a string
        $why = [];
        $ex = [];
        foreach ($this->flows as $f) {
            $src = $f['src'];
            $dst = (string) $f['dst'];
            if ($src['k'] !== 's' || $dst === 'str' || $dst === 'interp' || !isset($this->cand[$src['s']], $this->cand[$dst])) {
                continue;
            }
            $a = $this->isConv((string) $src['s']);
            $b = $this->isConv($dst);
            if ($a === $b) {
                continue;
            }
            $other = $a ? $dst : (string) $src['s'];
            $g = $this->find($other);
            $r = $this->gstop[$g] ?? 'unseeded (cut)';
            $r = preg_replace('/^[^ ]+: /', '', $r);
            $r = preg_replace('/[:@].*$/s', '', $r);
            $why[$r] = ($why[$r] ?? 0) + 1;
            $ex[$r][$this->gstop[$g] ?? $other] = true;
        }
        arsort($why);
        $o .= "== boundaries at unconverted slots, by why they stay strings\n";
        foreach ($why as $k => $v) {
            $o .= "  $v $k\n";
            foreach (array_slice(array_keys($ex[$k]), 0, 4) as $e) {
                $o .= '      e.g. ' . substr($e, 0, 220) . "\n";
            }
        }
        arsort($lk);
        arsort($in);
        $o .= "== lookup sites by consumer (" . array_sum($lk) . ")\n";
        foreach (array_slice($lk, 0, 60, true) as $k => $v) {
            $o .= "  $v $k\n";
        }
        $o .= "== intern sites by source (" . array_sum($in) . ")\n";
        foreach (array_slice($in, 0, 40, true) as $k => $v) {
            $o .= "  $v $k\n";
        }
        $o .= "== converted groups\n";
        foreach ($this->groups as $g => $members) {
            if (isset($this->conv_groups[$g]) && !isset($this->gstop[$g])) {
                sort($members);
                $o .= '+ [' . count($members) . '] ' . ($this->seeded[$g] ?? $this->conv_groups[$g]) . "\n";
                foreach (array_slice($members, 0, 12) as $m) {
                    $o .= "    $m\n";
                }
            }
        }
        $o .= "== stopped seeded-looking groups\n";
        foreach ($this->groups as $g => $members) {
            if (!isset($this->gstop[$g])) {
                continue;
            }
            $why = null;
            foreach ($members as $x) {
                $why = $this->seedWhy($x);
                if ($why !== null) {
                    break;
                }
            }
            if ($why === null) {
                continue;
            }
            $o .= '- [' . count($members) . '] ' . $this->gstop[$g] . "\n";
            foreach (array_slice($members, 0, 6) as $m) {
                $o .= "    $m\n";
            }
        }
        return $o;
    }
}

final class Applier
{
    /** @var array<string, string> literal => Sym constant name */
    private array $sym = [];

    public function __construct(private string $root, private Solver $solver)
    {
    }

    /** @param list<array<string, mixed>> $edits */
    public function apply(array $edits): void
    {
        $by_file = [];
        foreach ($edits as $e) {
            $key = $e['s'] . ':' . $e['e'] . ':' . $e['kind'] . ':' . ($e['pre'] ?? '') . ($e['text'] ?? '');
            $by_file[$e['file']][$key] = $e;
        }
        $n = 0;
        foreach ($by_file as $file => $list) {
            $src = (string) file_get_contents($file);
            $test = !str_starts_with(substr($file, strlen($this->root)), 'src/');
            $new = $this->applyFile($src, array_values($list), $test, $file);
            if ($new !== $src) {
                $new = $this->imports($new, $file);
                file_put_contents($file, $new);
                $n++;
            }
        }
        $this->bootstrap();
        $this->writeSym();
        fwrite(STDERR, "rewrote $n files, " . count($this->sym) . " Sym constants\n");
    }

    /**
     * The interner itself: Interner.php, the preloaded names merged by the Codebase, the table kept beside the
     * cache, and forked workers handing their new strings back to the parent.
     */
    private function bootstrap(): void
    {
        copy(__DIR__ . '/templates/Interner.php', $this->root . 'src/Psalm/Internal/Interner.php');
        $patch = function (string $rel, string $anchor, string $insert, bool $after = true): void {
            $file = $this->root . $rel;
            $src = (string) file_get_contents($file);
            if (substr_count($src, $anchor) !== 1) {
                throw new \RuntimeException("bootstrap anchor not unique in $rel: $anchor");
            }
            $src = str_replace($anchor, $after ? $anchor . $insert : $insert . $anchor, $src);
            file_put_contents($file, $this->imports($src, $file));
        };
        $patch('src/Psalm/Codebase.php', "        if (\$progress === null) {\n            \$progress = new VoidProgress();\n        }\n",
            "        Interner::merge(Sym::PRELOADED);\n        \$cache_directory = \$config->getCacheDirectory();\n"
            . "        if (\$cache_directory !== null) {\n            Interner::persistTo(\$cache_directory);\n        }\n");
        foreach (['InitScannerTask', 'InitAnalyzerTask'] as $t) {
            $patch("src/Psalm/Internal/Fork/$t.php", "Cancellation \$cancellation): mixed\n    {\n", "        Interner::mark();\n\n");
        }
        foreach (['ShutdownScannerTask', 'ShutdownAnalyzerTask'] as $t) {
            $src = (string) file_get_contents($this->root . "src/Psalm/Internal/Fork/$t.php");
            $anchor = preg_match('/        return \[\n/', $src) ? "        return [\n" : '';
            $patch("src/Psalm/Internal/Fork/$t.php", $anchor, "            'interner' => Interner::delta(),\n");
        }
        foreach (['Scanner' => 'PoolData', 'Analyzer' => 'WorkerData'] as $c => $type) {
            $patch("src/Psalm/Internal/Codebase/$c.php", "                \$pool_data = \$pool_data->await();\n",
                "\n                Interner::merge(\$pool_data['interner']);\n");
            $patch("src/Psalm/Internal/Codebase/$c.php", " * @psalm-type  $type = array{\n", " *     interner: list<string>,\n");
        }
    }

    /** @param list<array<string, mixed>> $list */
    private function applyFile(string $src, array $list, bool $test, string $file): string
    {
        // insertions (pos, order, text) and replacements [s, e, text]
        $ins = [];
        $rep = [];
        foreach ($list as $e) {
            switch ($e['kind']) {
                case 'wrap':
                    $pre = $e['pre'];
                    $post = $e['post'];
                    if (($e['ctx'] ?? null) === 'paren') {
                        $pre = '(' . $pre;
                        $post .= ')';
                    } elseif (($e['ctx'] ?? null) === 'brace' && ($src[$e['s'] - 1] ?? '') !== '{') {
                        $pre = '{' . $pre;
                        $post .= '}';
                    }
                    $ins[] = [$e['s'], 1, -$e['e'], $pre];
                    $ins[] = [$e['e'], 0, -$e['s'], $post];
                    break;
                case 'interp':
                    $fn = 'Interner::lookup' . ($e['nl'] ? 'OrNull' : '');
                    if ($e['brace'] && ($src[$e['s'] - 1] ?? '') === '{' && ($src[$e['e']] ?? '') === '}') {
                        $rep[] = [$e['s'] - 1, $e['s'], '" . ' . $fn . '('];
                        $rep[] = [$e['e'], $e['e'] + 1, ') . "'];
                    } else {
                        $ins[] = [$e['s'], 1, -$e['e'], '" . ' . $fn . '('];
                        $ins[] = [$e['e'], 0, -$e['s'], ') . "'];
                    }
                    break;
                case 'dispatch':
                    // (test ? converted-branch : other-branch) around the read
                    [$cond, $fn] = explode("\0", (string) $e['text']);
                    $expr = substr($src, $e['s'], $e['e'] - $e['s']);
                    $conv = $fn === 'lookup' ? 'Interner::lookup(' . $expr . ')' : $expr;
                    $other = $fn === 'lookup' ? $expr : 'Interner::intern(' . $expr . ')';
                    $text = '(' . $cond . ' ? ' . $conv . ' : ' . $other . ')';
                    if ($e['interp']) {
                        if ($e['brace'] && ($src[$e['s'] - 1] ?? '') === '{' && ($src[$e['e']] ?? '') === '}') {
                            $rep[] = [$e['s'] - 1, $e['e'] + 1, '" . ' . $text . ' . "'];
                        } else {
                            $rep[] = [$e['s'], $e['e'], '" . ' . $text . ' . "'];
                        }
                    } else {
                        $rep[] = [$e['s'], $e['e'], $text];
                    }
                    break;
                case 'lit':
                    // a constant expression needs the Sym constant
                    $rep[] = [$e['s'], $e['e'], $this->literal((string) $e['v'], $test && !$e['cls'])];
                    break;
                case 'replace':
                    $rep[] = [$e['s'], $e['e'], (string) $e['text']];
                    break;
                case 'rename':
                    $old = substr($src, $e['s'], $e['e'] - $e['s']);
                    $rep[] = [$e['s'], $e['e'], str_ireplace('array_merge', 'array_replace', $old)];
                    break;
            }
        }
        usort($rep, static fn($a, $b) => $a[0] <=> $b[0]);
        // overlapping replacements: keep the first, report the others
        $clean = [];
        $end = -1;
        foreach ($rep as $r) {
            if ($r[0] < $end) {
                $this->solver->errors[] = "overlapping edit in $file at {$r[0]}";
                continue;
            }
            $clean[] = $r;
            $end = $r[1];
        }
        // events by position: at a position, suffixes (inner first) then prefixes (outer first), then a replacement
        usort($ins, static fn($a, $b) => [$a[0], $a[1], $a[2]] <=> [$b[0], $b[1], $b[2]]);
        $out = '';
        $pos = 0;
        $ri = 0;
        $ii = 0;
        $len = strlen($src);
        while ($pos <= $len) {
            while ($ii < count($ins) && $ins[$ii][0] === $pos) {
                $out .= $ins[$ii][3];
                $ii++;
            }
            if ($ri < count($clean) && $clean[$ri][0] === $pos) {
                $out .= $clean[$ri][2];
                $next = $clean[$ri][1];
                $ri++;
                // insertions strictly inside a replaced range are dropped (reported)
                while ($ii < count($ins) && $ins[$ii][0] < $next) {
                    $this->solver->errors[] = "insertion inside a replacement in $file at {$ins[$ii][0]}";
                    $ii++;
                }
                if ($next === $pos) {
                    continue;
                }
                $pos = $next;
                continue;
            }
            $nextpos = min($ins[$ii][0] ?? PHP_INT_MAX, $clean[$ri][0] ?? PHP_INT_MAX, $len + 1);
            if ($nextpos > $len) {
                $out .= substr($src, $pos);
                break;
            }
            $out .= substr($src, $pos, $nextpos - $pos);
            $pos = $nextpos;
        }
        return $out;
    }

    private function literal(string $v, bool $test): string
    {
        if ($test) {
            return 'Interner::intern(' . var_export($v, true) . ')';
        }
        return 'Sym::' . $this->symName($v);
    }

    private function symName(string $v): string
    {
        if (isset($this->sym[$v])) {
            return $this->sym[$v];
        }
        if ($v === '') {
            return $this->sym[$v] = 'EMPTY';
        }
        $words = [];
        foreach (explode('\\', $v) as $p) {
            $words[] = strtoupper((string) preg_replace('/(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])/', '_', $p));
        }
        $name = 'C_' . preg_replace('/[^A-Z0-9_]/', '_', implode('__', $words));
        $taken = array_flip($this->sym);
        $base = $name;
        for ($i = 2; isset($taken[$name]); $i++) {
            $name = $base . '_' . $i;
        }
        return $this->sym[$v] = $name;
    }

    /** Adds `use Psalm\Internal\Interner;` / `use Psalm\Internal\Sym;` where the new code needs them. */
    private function imports(string $src, string $file): string
    {
        $ns = preg_match('/^namespace\s+([^;{\s]+)/m', $src, $m) ? $m[1] : '';
        foreach (['Interner', 'Sym'] as $cls) {
            if (!preg_match('/(?<![\w\\\\$>])' . $cls . '::/', $src) || $ns === 'Psalm\\Internal') {
                continue;
            }
            $fq = 'Psalm\\Internal\\' . $cls;
            if (preg_match('/^use\s+' . preg_quote($fq, '/') . ';/m', $src)) {
                continue;
            }
            $line = "use $fq;\n";
            // among the class imports, in order
            if (preg_match_all('/^use\s+(?!function\b|const\b)([^;]+);\n/m', $src, $uses, PREG_OFFSET_CAPTURE)) {
                $at = null;
                foreach ($uses[1] as $i => [$name, $off]) {
                    if (strcasecmp($name, $fq) > 0) {
                        $at = $uses[0][$i][1];
                        break;
                    }
                }
                if ($at === null) {
                    $last = end($uses[0]);
                    $at = $last[1] + strlen($last[0]);
                }
                $src = substr($src, 0, $at) . $line . substr($src, $at);
            } elseif ($ns !== '' && preg_match('/^namespace\s+[^;]+;\n/m', $src, $nm, PREG_OFFSET_CAPTURE)) {
                $at = $nm[0][1] + strlen($nm[0][0]);
                $src = substr($src, 0, $at) . "\n" . $line . substr($src, $at);
            } else {
                // no namespace: fully qualified
                $src = (string) preg_replace('/(?<![\w\\\\$>])' . $cls . '::/', '\\\\' . $fq . '::', $src);
            }
        }
        return $src;
    }

    private function writeSym(): void
    {
        $names = $this->sym;
        ksort($names);
        $gen = $this->root . 'bin/generate-sym.php';
        $list = '';
        foreach ($names as $v => $c) {
            $list .= '    ' . var_export($c, true) . ' => ' . var_export((string) $v, true) . ",\n";
        }
        $tpl = (string) file_get_contents(__DIR__ . '/generate-sym.php.tpl');
        file_put_contents($gen, str_replace("    // NAMES\n", $list, $tpl));
        passthru('php ' . escapeshellarg($gen));
    }
}
