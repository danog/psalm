<?php

/**
 * fix.php ROOT ISSUES.json: one round of boundary fixes on a tree textual.php converted, from Psalm's report on it
 * (run by an unconverted Psalm). Where an id meets a string it adds `Interner::lookup`, where a string meets an id
 * `Interner::intern` (a literal becomes its Sym constant), and a literal wrongly made a Sym goes back.
 * Prints how many sites it fixed; run Psalm and this again until nothing is left to fix.
 */

declare(strict_types=1);

require_once __DIR__ . '/TypeStr.php';
require_once __DIR__ . '/names.php';

use Psalm\Tools\IdConvert\TypeStr;

$root = rtrim($argv[1], '/') . '/';
CanonicalNames::init($root);
$issues = json_decode((string) file_get_contents($argv[2]), true);
/** A message without what differs between the two checked trees: their paths (closure names embed them). */
function pristineMessage(string $m): string
{
    return (string) preg_replace('~/\S*?/src/psalm/(\S*?):\d+:\d+:-:closure~', 'src/psalm/$1:closure', $m);
}

// only what the conversion introduced: the issues an unconverted tree has too (env ID_CONVERT_PRISTINE) stay
$pristine_file = (string) getenv('ID_CONVERT_PRISTINE');
if ($pristine_file !== '' && is_file($pristine_file)) {
    $seen_pristine = [];
    foreach (json_decode((string) file_get_contents($pristine_file), true) ?: [] as $pi) {
        $pk = $pi['file_name'] . "\0" . $pi['type'] . "\0" . pristineMessage($pi['message']);
        $seen_pristine[$pk] = ($seen_pristine[$pk] ?? 0) + 1;
    }
    $issues = array_values(array_filter($issues, static function (array $ci) use (&$seen_pristine): bool {
        $ck = $ci['file_name'] . "\0" . $ci['type'] . "\0" . pristineMessage($ci['message']);
        if (($seen_pristine[$ck] ?? 0) > 0) {
            $seen_pristine[$ck]--;
            return false;
        }
        return true;
    }));
}

/** @var array<string, string> Sym constant => literal */
$sym = [];
if (is_file($root . 'src/Psalm/Internal/Sym.php')) {
    preg_match_all('/`(.*?)` \*\/\n    public const (\w+) =/', (string) file_get_contents($root . 'src/Psalm/Internal/Sym.php'), $m, PREG_SET_ORDER);
    foreach ($m as [, $v, $c]) {
        $sym[$c] = $v;
    }
}
/** @var array<string, string> literal => Sym constant (new ones are added) */
$sym_by_value = array_flip($sym);

$stringish = '(?:[a-z]+-)*string(?:<[^>]*>)?|\'[^\']*\'|""';
$intish = 'int|positive-int|non-negative-int|int<[^>]*>|-?\d+';

/** @var array<string, list<array{int, int, string, string}>> file => [from, to, kind, arg] */
$edits = [];
/** @var array<string, string> the sources as Psalm read them */
$sources = [];
$add = static function (string $file, int $from, int $to, string $kind, string $arg = '') use (&$edits): void {
    $edits[$file][] = [$from, $to, $kind, $arg];
};
// a report older than the tree (an edit applied since): its issues are skipped
$issues = array_values(array_filter($issues, static function (array $i) use (&$sources): bool {
    if ($i['file_path'] === '') {
        return false;
    }
    $sources[$i['file_path']] ??= (string) @file_get_contents($i['file_path']);
    return substr($sources[$i['file_path']], $i['from'], $i['to'] - $i['from']) === $i['selected_text'];
}));

foreach ($issues as $i) {
    if ($i['severity'] !== 'error') {
        continue;
    }
    $file = $i['file_path'];
    $msg = $i['message'];
    $type = $i['type'];
    $from = $i['from'];
    $to = $i['to'];
    $nullable = false;

    // an argument, a property value, a return value, a concatenated value
    if (in_array($type, ['InvalidArgument', 'PossiblyInvalidArgument', 'InvalidScalarArgument', 'ArgumentTypeCoercion'], true)
        && preg_match('/expects (.+?), but (?:possibly different type )?(.+?) provided/', $msg, $m)
    ) {
        [$want, $got] = [$m[1], $m[2]];
        $nullable = str_contains($got, 'null');
    } elseif ($type === 'InvalidPropertyAssignmentValue'
        && preg_match("/with declared type '(.+?)' cannot be assigned type '(.+?)'/", $msg, $m)
    ) {
        [$want, $got] = [$m[1], $m[2]];
        $nullable = str_contains($got, 'null');
        // Psalm selects the assigned property: the value is after its `=`
        $src = $sources[$file];
        if (preg_match('/\G\s*=(?![=>])\s*/', $src, $em, 0, $to)) {
            $vstart = $to + strlen($em[0]);
            $depth = 0;
            for ($vend = $vstart; $vend < strlen($src); $vend++) {
                $c = $src[$vend];
                if ($c === '(' || $c === '[' || $c === '{') {
                    $depth++;
                } elseif ($c === ')' || $c === ']' || $c === '}') {
                    $depth--;
                } elseif ($c === ';' && $depth === 0) {
                    break;
                }
            }
            $from = $vstart;
            $to = $vend;
            // the empty name is 0
            if (trim(substr($src, $from, $to - $from)) === "''" && preg_match('/^(?:null\|)?int(?:\|null)?$/', strtolower($want))) {
                $edits[$file][] = [$from, $to, 'replace', '0'];
                continue;
            }
        }
    } elseif (in_array($type, ['InvalidReturnStatement'], true)
        && preg_match("/inferred type '(.+?)' does not match the declared return type '(.+?)'/", $msg, $m)
    ) {
        [$got, $want] = [$m[1], $m[2]];
        $nullable = str_contains($got, 'null');
    } elseif ($type === 'IdString' && preg_match('/^(\S+) .* (built|interpolated|heredoc-interpolated) into a string$/s', $msg, $m)) {
        // an id built into a string (IdStringPlugin): its name is
        if ($m[2] === 'interpolated') {
            // split out of the double-quoted string: `"...{$x}..."` -> `"..." . Interner::lookup($x) . "..."`
            $src = $sources[$file];
            [$f2, $t2] = $src[$from - 1] === '{' && $src[$to] === '}' ? [$from - 1, $to + 1] : [$from, $to];
            $fn = str_contains($m[1], 'null') ? 'lookupOrNull' : 'lookup';
            $edits[$file][] = [$f2, $t2, 'replace', '" . Interner::' . $fn . '(' . substr($src, $from, $to - $from) . ') . "'];
            continue;
        }
        if ($m[2] !== 'built') {
            continue;
        }
        [$want, $got] = ['string', $m[1]];
        $nullable = str_contains($got, 'null');
    } elseif (in_array($type, ['InvalidOperand', 'PossiblyInvalidOperand'], true)
        && preg_match('/Cannot concatenate with a (.+)$/', $msg, $m)
    ) {
        [$want, $got] = ['string', $m[1]];
        $nullable = str_contains($got, 'null');
    } elseif ($type === 'PossiblyUndefinedStringArrayOffset'
        && preg_match("/^Possibly undefined array offset '('[A-Za-z_\\\\][A-Za-z0-9_\\\\]*')' is risky given expected type 'int'/", $msg, $m)
    ) {
        // a name literal indexing an id-keyed map: its id
        $src = $sources[$file];
        $at = strrpos(substr($src, $from, $to - $from), '[' . $m[1] . ']');
        if ($at !== false) {
            $add($file, $from + $at + 1, $from + $at + 1 + strlen($m[1]), 'intern');
        }
        continue;
    } elseif ($type === 'InvalidArrayOffset'
        && (preg_match('/using a ([\w-]+) offset, expecting (\w+)/', $msg, $m)
            || (preg_match("/using offset value of '.*', expecting (int)/", $msg, $mm) && ($m = [0, 'string', 'int'])))
    ) {
        $m[1] = str_contains($m[1], 'string') ? 'string' : $m[1];
        // the key of the fetch: the last `[...]` of the range
        $src = (string) file_get_contents($file);
        $text = substr($src, $from, $to - $from);
        if (!str_ends_with(rtrim($text), ']')) {
            continue;
        }
        $close = $from + strlen(rtrim($text)) - 1;
        $depth = 0;
        for ($k = $close; $k > $from; $k--) {
            if ($src[$k] === ']') {
                $depth++;
            } elseif ($src[$k] === '[') {
                $depth--;
                if ($depth === 0) {
                    break;
                }
            }
        }
        if ($k <= $from || $k + 1 >= $close) {
            continue;
        }
        if ($m[1] === 'string' && $m[2] === 'int') {
            $add($file, $k + 1, $close, 'intern');
        } elseif ($m[1] === 'int' && $m[2] === 'string') {
            $add($file, $k + 1, $close, 'lookup');
        }
        continue;
    } else {
        continue;
    }
    $want = strtolower($want);
    $got = strtolower($got);
    $wantInt = (bool) preg_match('/^(?:null\|)?(?:' . $intish . ')(?:\|null)?$/', $want);
    // the top-level atoms of the wanted type (generic parameters stripped)
    $top = $want;
    while (($stripped = (string) preg_replace('/<[^<>]*>|\{[^{}]*\}|\([^()]*\)/', '', $top)) !== $top) {
        $top = $stripped;
    }
    $wantStr = (bool) preg_match('/^(?:null\|)?(?:' . $stringish . ')(?:\|null)?$/', $want)
        // a union taking a string (a name, an object, an array) but no int
        || ((bool) preg_match('/(^|\|)(?:(?:non-empty-|non-falsy-|lowercase-|class-|callable-)*string)(\||$)/', $top) && !preg_match('/(^|\|)(?:' . $intish . '|array-key|mixed|scalar)(\||$)/', $top));
    $gotInt = (bool) preg_match('/^(?:null\|)?(?:' . $intish . ')(?:\|(?:null|' . $intish . '))*$/', $got);
    $gotStr = (bool) preg_match('/^(?:null\|)?(?:' . $stringish . '|[\w\\\\]+::class)(?:\|(?:null|' . $stringish . '|[\w\\\\]+::class))*$/', $got);
    if ($wantInt && preg_match('/^([\w\\\\]+)::class$/', $m[2] ?? '', $cm) && $type !== 'InvalidReturnStatement') {
        // `X::class` meeting an id: the name's Sym constant (the message spells the resolved name)
        $add($file, $from, $to, 'sym', ltrim($cm[1], '\\'));
        continue;
    }
    if ($wantInt && $gotStr) {
        $add($file, $from, $to, 'intern', $nullable ? 'OrNull' : '');
    } elseif ($wantStr && $gotInt) {
        $add($file, $from, $to, 'lookup', $nullable ? 'OrNull' : '');
    } elseif (($paths = arrayPaths($want, $got, $stringish, $intish)) !== null) {
        foreach ($paths as $fn => $ps) {
            if ($ps !== []) {
                $add($file, $from, $to, 'at', $fn . ':' . implode(',', $ps));
            }
        }
    } elseif (false && preg_match('/^(?:non-empty-)?(array|list)<(.+)>$/', $want, $w) && preg_match('/^(?:non-empty-)?(array|list)<(.+)>$/', $got, $g)) {
        // arrays: keys / values of different kinds
        $wk = $w[1] === 'list' ? 'int' : trim(explode(',', $w[2], 2)[0]);
        $gk = $g[1] === 'list' ? 'int' : trim(explode(',', $g[2], 2)[0]);
        $wv = trim($w[1] === 'list' ? $w[2] : (explode(',', $w[2], 2)[1] ?? ''));
        $gv = trim($g[1] === 'list' ? $g[2] : (explode(',', $g[2], 2)[1] ?? ''));
        $fns = [];
        if (preg_match('/^(' . $intish . ')$/', $wk) && preg_match('/^(' . $stringish . '|array-key|int\|string|string\|int)$/', $gk)) {
            $fns[] = 'internKeys';
        } elseif (preg_match('/^(' . $stringish . ')$/', $wk) && preg_match('/^(' . $intish . ')$/', $gk)) {
            $fns[] = 'lookupKeys';
        }
        if (preg_match('/^(' . $intish . ')(\|null)?$/', $wv) && preg_match('/^(' . $stringish . ')(\|null)?$/', $gv)) {
            $fns[] = 'internList';
        } elseif (preg_match('/^(' . $stringish . ')(\|null)?$/', $wv) && preg_match('/^(' . $intish . ')(\|null)?$/', $gv)) {
            $fns[] = 'lookupList';
        }
        foreach ($fns as $fn) {
            $add($file, $from, $to, 'call', $fn);
        }
    }
}

// a tuple returned with ids where its docblock says strings: the docblock's entries become int
foreach ($issues as $i) {
    if ($i['type'] !== 'InvalidReturnStatement'
        || !preg_match("/inferred type '(?:false\|)?((?:list|array)\{.+\})(?:\|false)?' does not match the declared return type '(?:false\|)?((?:list|array)\{.+\})(?:\|false)?'/", $i['message'], $rm)
    ) {
        continue;
    }
    try {
        $got = TypeStr::parse($rm[1])['atoms'][0] ?? null;
        $want = TypeStr::parse($rm[2])['atoms'][0] ?? null;
    } catch (Throwable) {
        continue;
    }
    if (($got['kind'] ?? '') !== 'shape' || ($want['kind'] ?? '') !== 'shape' || count($got['items'] ?? []) !== count($want['items'] ?? [])) {
        continue;
    }
    $flip = [];
    foreach ($want['items'] as $k => $wu) {
        $wt = strtolower(substr($rm[2], $wu['start'], $wu['end'] - $wu['start']));
        $gt = strtolower(substr($rm[1], $got['items'][$k]['start'], $got['items'][$k]['end'] - $got['items'][$k]['start']));
        if (preg_match('/^(?:null\|)?(?:' . $stringish . ')(?:\|null)?$/', trim($wt)) && preg_match('/^(?:null\|)?(?:' . $intish . ')(?:\|null)?$/', trim($gt))) {
            $flip[] = $k;
        }
    }
    if ($flip === []) {
        continue;
    }
    // the function's docblock `@return` / `@psalm-return`
    $src = $sources[$i['file_path']];
    $fn = strrpos(substr($src, 0, $i['from']), 'function ');
    $doc_end = $fn === false ? false : strrpos(substr($src, 0, $fn), '*/');
    $doc_start = $doc_end === false ? false : strrpos(substr($src, 0, $doc_end), '/**');
    if ($doc_start === false || trim((string) preg_replace('/#\[.*?\]|public|private|protected|static|final|abstract/s', '', substr($src, $doc_end + 2, $fn - $doc_end - 2))) !== '') {
        continue;
    }
    $doc = substr($src, $doc_start, $doc_end - $doc_start);
    foreach (['@psalm-return', '@return'] as $tag) {
        if (!preg_match('/' . preg_quote($tag, '/') . '\s+/', $doc, $tm, PREG_OFFSET_CAPTURE)) {
            continue;
        }
        $tstart = $tm[0][1] + strlen($tm[0][0]);
        $flat = (string) preg_replace_callback('/\n\s*\*/', static fn(array $x): string => str_repeat(' ', strlen($x[0])), substr($doc, $tstart));
        try {
            $u = TypeStr::parse($flat);
        } catch (Throwable) {
            continue;
        }
        $shape = null;
        foreach ($u['atoms'] as $atom) {
            if (($atom['kind'] ?? '') === 'shape') {
                $shape = $atom;
            }
        }
        if ($shape === null || count($shape['items'] ?? []) !== count($want['items'])) {
            continue;
        }
        foreach ($flip as $k) {
            $item = $shape['items'][$k];
            $text = substr($flat, $item['start'], $item['end'] - $item['start']);
            $edits[$i['file_path']][] = [$doc_start + $tstart + $item['start'], $doc_start + $tstart + $item['end'], 'replace',
                str_contains($text, 'null') || str_starts_with(trim($text), '?') ? '?int' : 'int'];
        }
    }
}

// a ternary / `?:` / `??` whose branches are of both kinds: the branch of the wrong kind is wrapped
foreach ($issues as $i) {
    $msg = $i['message'];
    if (!in_array($i['type'], ['InvalidReturnStatement', 'InvalidArgument', 'PossiblyInvalidArgument', 'InvalidPropertyAssignmentValue'], true)) {
        continue;
    }
    if (preg_match("/inferred type '(.+?)' does not match the declared return type '(.+?)'/", $msg, $m)) {
        [$got, $want] = [$m[1], $m[2]];
    } elseif (preg_match('/expects (.+?), but (?:possibly different type )?(.+?) provided/', $msg, $m)) {
        [$want, $got] = [$m[1], $m[2]];
    } elseif (preg_match("/with declared type '(.+?)' cannot be assigned type '(.+?)'/", $msg, $m)) {
        [$want, $got] = [$m[1], $m[2]];
    } else {
        continue;
    }
    $want = strtolower($want);
    $got = strtolower($got);
    $mixed = preg_match('/(^|\|)(?:' . $intish . ')(\||$)/', $got) && preg_match('/(^|\|)(?:' . $stringish . ')(\||$)/', $got);
    $wstr = (bool) preg_match('/^(?:null\|)?(?:' . $stringish . ')(?:\|null)?$/', $want);
    $wint = (bool) preg_match('/^(?:null\|)?(?:' . $intish . ')(?:\|null)?$/', $want);
    if (!$mixed || (!$wstr && !$wint)) {
        continue;
    }
    foreach (branches($i['selected_text']) as [$b, $off]) {
        $bt = trim($b);
        $lead = strlen($b) - strlen(ltrim($b));
        $read = (bool) preg_match('/^\$\w+(?:->\w+)*$/', $bt) && !str_contains($bt, 'Interner::');
        $stringy = (bool) preg_match("/^'|^\"|\s\.\s|^(?:strtolower|substr|str_\w+|trim|ltrim|rtrim|sprintf|implode|Interner::lookup)\b/", $bt);
        $at = $i['from'] + $off + $lead;
        if ($wstr && $read) {
            $edits[$i['file_path']][] = [$at, $at + strlen($bt), 'lookup', ''];
        } elseif ($wint && $stringy && !str_starts_with($bt, 'Interner::lookup')) {
            $edits[$i['file_path']][] = [$at, $at + strlen($bt), 'intern', ''];
        }
    }
}

/**
 * The value branches of a top-level `c ? a : b` / `a ?: b` / `a ?? b` (with their offsets); [] when not one.
 *
 * @return list<array{string, int}>
 */
function branches(string $e): array
{
    $depth = 0;
    $q = null;
    $colon = null;
    $coal = null;
    for ($k = 0; $k < strlen($e); $k++) {
        $c = $e[$k];
        if ($c === "'" || $c === '"') {
            for ($k++; $k < strlen($e) && $e[$k] !== $c; $k++) {
                if ($e[$k] === '\\') {
                    $k++;
                }
            }
            continue;
        }
        if ($c === '(' || $c === '[' || $c === '{') {
            $depth++;
        } elseif ($c === ')' || $c === ']' || $c === '}') {
            $depth--;
        } elseif ($depth === 0 && $c === '?' && ($e[$k + 1] ?? '') === '?' && $coal === null && $q === null) {
            $coal = $k;
            $k++;
        } elseif ($depth === 0 && $c === '?' && ($e[$k + 1] ?? '') !== '-' && $q === null && $coal === null) {
            $q = $k;
        } elseif ($depth === 0 && $c === ':' && ($e[$k + 1] ?? '') !== ':' && ($e[$k - 1] ?? '') !== ':' && $q !== null && $colon === null) {
            $colon = $k;
        }
    }
    if ($coal !== null) {
        return [[substr($e, 0, $coal), 0], [substr($e, $coal + 2), $coal + 2]];
    }
    if ($q !== null && $colon !== null) {
        $a = substr($e, $q + 1, $colon - $q - 1);
        $out = trim($a) === '' ? [[substr($e, 0, $q), 0]] : [[$a, $q + 1]];
        $out[] = [substr($e, $colon + 1), $colon + 1];
        return $out;
    }
    return [];
}

// a tuple docblock (InvalidReturnType selects the docblock type itself): entries holding ids become int
foreach ($issues as $i) {
    if ($i['type'] !== 'InvalidReturnType'
        || !preg_match("/is incorrect, got '(?:false\|)?((?:list|array)\{.+\})(?:\|false)?'$/", $i['message'], $gm)
        || !inDocblock($sources[$i['file_path']], $i['from'])
    ) {
        continue;
    }
    $text = $i['selected_text'];
    $flat = (string) preg_replace_callback('/\n\s*\*/', static fn(array $x): string => str_repeat(' ', strlen($x[0])), $text);
    try {
        $got = TypeStr::parse($gm[1])['atoms'][0] ?? null;
        $decl = null;
        foreach (TypeStr::parse($flat)['atoms'] as $atom) {
            if (($atom['kind'] ?? '') === 'shape') {
                $decl = $atom;
            }
        }
    } catch (Throwable) {
        continue;
    }
    if ($decl === null || ($got['kind'] ?? '') !== 'shape' || count($got['items']) !== count($decl['items'])) {
        continue;
    }
    foreach ($decl['items'] as $k => $du) {
        $dt = strtolower(trim(substr($flat, $du['start'], $du['end'] - $du['start'])));
        $gt = strtolower(trim(substr($gm[1], $got['items'][$k]['start'], $got['items'][$k]['end'] - $got['items'][$k]['start'])));
        if (preg_match('/^(?:null\|)?(?:' . $stringish . ')(?:\|null)?$/', $dt) && preg_match('/^(?:null\|)?(?:' . $intish . ')(?:\|null)?$/', $gt)) {
            $edits[$i['file_path']][] = [$i['from'] + $du['start'], $i['from'] + $du['end'], 'replace',
                str_contains($dt, 'null') ? 'int|null' : 'int'];
        }
    }
}

// an id variable reassigned from a string function (`$name = substr(...)`): the reassignment interns
foreach ($issues as $i) {
    if ($i['type'] !== 'PossiblyInvalidArgument'
        || !preg_match('/expects (?:int|string), but possibly different type int\|(?:non-empty-|non-falsy-|lowercase-)*string provided/', $i['message'])
        || !preg_match('/^\$\w+$/', $i['selected_text'])
    ) {
        continue;
    }
    $src = $sources[$i['file_path']];
    $var = $i['selected_text'];
    $fn_start = strrpos(substr($src, 0, $i['from']), 'function ');
    if ($fn_start === false) {
        continue;
    }
    $body = substr($src, $fn_start, $i['from'] - $fn_start);
    // the variable and what it copies (`$a = $b;`)
    $vars = [$var];
    for ($d = 0; $d < 3 && preg_match('/' . preg_quote(end($vars), '/') . '\s*=\s*(\$\w+)\s*;/', $body, $cm); $d++) {
        $vars[] = $cm[1];
    }
    foreach ($vars as $v) {
        if (preg_match_all('/' . preg_quote($v, '/') . '\s*=\s*((?:\(string\)\s*)?(?:substr|str_\w+|strtolower|strtoupper|trim|ltrim|rtrim|sprintf|implode|preg_replace|Interner::lookup)\b[^;]*);/', $body, $am, PREG_OFFSET_CAPTURE)) {
            foreach ($am[1] as [$rhs, $off]) {
                $edits[$i['file_path']][] = [$fn_start + $off, $fn_start + $off + strlen($rhs), 'intern', ''];
            }
        }
    }
}

// a string offset on an id (`$name[0]`): the base is looked up
foreach ($issues as $i) {
    if ($i['type'] !== 'InvalidArrayAccess' || !preg_match('/non-array variable (.+) of type (?:null\|)?int(?:\|null)?$/', $i['message'], $am)) {
        continue;
    }
    $text = $i['selected_text'];
    $base = $am[1];
    if ($base !== '' && !str_contains($base, '[') && str_starts_with($text, $base)) {
        $edits[$i['file_path']][] = [$i['from'], $i['from'] + strlen($base), 'lookup', ''];
    } elseif (preg_match('/^(\$\w+)\[/', $base, $rm)) {
        // an element of a string list holding an id: the id was written into it - look it up at the write
        $src = $sources[$i['file_path']];
        $fn = strrpos(substr($src, 0, $i['from']), 'function ');
        if ($fn === false) {
            continue;
        }
        $body = substr($src, $fn, $i['from'] - $fn);
        $roots = [$rm[1]];
        // `$tok = $tokens[$i];`: the list it was copied from
        if (preg_match('/' . preg_quote($rm[1], '/') . '\s*=\s*(\$\w+)\[/', $body, $cm)) {
            $roots[] = $cm[1];
        }
        $whole = substr($src, $fn, (enclosingEnd($src, $fn) ?? strlen($src)) - $fn);
        foreach ($roots as $root_var) {
            if (preg_match_all('/' . preg_quote($root_var, '/') . '(?:\[[^\]]*\])+\s*=(?![=>])\s*(\$\w+)\s*;/', $whole, $wm, PREG_OFFSET_CAPTURE)) {
                foreach ($wm[1] as [$v, $off]) {
                    if (isNameVar($v)) {
                        $edits[$i['file_path']][] = [$fn + $off, $fn + $off + strlen($v), 'lookup', ''];
                    }
                }
            }
        }
    }
}

// `new $x` / `$x::m()` with an id: the class expression is looked up
foreach ($issues as $i) {
    if ($i['type'] !== 'UndefinedClass' || !str_contains($i['message'], 'Type int cannot be called as a class')) {
        continue;
    }
    $text = $i['selected_text'];
    if (preg_match('/^new\s+(\$[\w]+(?:->\w+)*)/', $text, $cm, PREG_OFFSET_CAPTURE)) {
        $at = $i['from'] + $cm[1][1];
        $edits[$i['file_path']][] = [$at, $at + strlen($cm[1][0]), 'paren-lookup', ''];
    } elseif (preg_match('/^(\$[\w]+(?:->\w+)*)::/', $text, $cm, PREG_OFFSET_CAPTURE)) {
        $at = $i['from'] + $cm[1][1];
        $edits[$i['file_path']][] = [$at, $at + strlen($cm[1][0]), 'paren-lookup', ''];
    }
}

// a lowered name compared with an id: names are case-sensitive ids, the id itself compares
foreach ($issues as $i) {
    if (!in_array($i['type'], ['DocblockTypeContradiction', 'TypeDoesNotContainType'], true)
        || !preg_match('/lowercase-string does not contain int|int does not contain (?:non-empty-)?lowercase-string/', $i['message'])
    ) {
        continue;
    }
    $src = $sources[$i['file_path']];
    if (preg_match_all('/strtolower\(\s*Interner::lookup\(/', $i['selected_text'], $lm, PREG_OFFSET_CAPTURE)) {
        foreach ($lm[0] as [$whole, $off]) {
            $start = $i['from'] + $off;
            $inner = $start + strlen($whole);
            // the lookup's `)` and then strtolower's
            $depth = 2;
            $lookup_end = null;
            for ($k = $inner; $k < strlen($src) && $depth > 0; $k++) {
                if ($src[$k] === '(') {
                    $depth++;
                } elseif ($src[$k] === ')') {
                    $depth--;
                    if ($depth === 1 && $lookup_end === null) {
                        $lookup_end = $k;
                    }
                }
            }
            if ($lookup_end === null || trim(substr($src, $lookup_end + 1, $k - 1 - ($lookup_end + 1)), ", \n\t") !== '') {
                continue; // strtolower takes more than the name
            }
            $edits[$i['file_path']][] = [$start, $k, 'replace', trim(substr($src, $inner, $lookup_end - $inner))];
        }
    }
}

// comparisons across the two kinds
foreach ($issues as $i) {
    if (!in_array($i['type'], ['TypeDoesNotContainType', 'DocblockTypeContradiction', 'RedundantCondition', 'RedundantConditionGivenDocblockType'], true)) {
        continue;
    }
    $src = (string) file_get_contents($i['file_path']);
    $text = substr($src, $i['from'], $i['to'] - $i['from']);
    if (!preg_match('/^(.*?)\s*(===|!==|==|!=)\s*(.*)$/s', $text, $cm, PREG_OFFSET_CAPTURE)) {
        continue;
    }
    [$l, $lo] = $cm[1];
    [$r, $ro] = $cm[3];
    $msg = $i['message'];
    // a literal name against an id: its Sym constant
    if (preg_match("/^'([^']*)' cannot be identical to int|^int cannot be identical to '([^']*)'/", $msg)) {
        foreach ([[$l, $lo], [$r, $ro]] as [$side, $off]) {
            if (preg_match("/^'([A-Za-z_\\\\][A-Za-z0-9_\\\\]*)'$/", $side)) {
                $edits[$i['file_path']][] = [$i['from'] + $off, $i['from'] + $off + strlen($side), 'intern', ''];
            }
        }
        continue;
    }
    // an id against a string: the id side is looked up ("A cannot be identical to B": A is the right operand's type)
    if (preg_match('/^int cannot be identical to (?:null\|)?(?:' . $stringish . ')(?:\|null)?$/', $msg)
        && preg_match('/^\$[\w>\-]+$|^[\w:\\\\]+\(.*\)$/s', $r)
    ) {
        $edits[$i['file_path']][] = [$i['from'] + $ro, $i['from'] + $ro + strlen($r), 'lookup', ''];
    } elseif (preg_match('/^(?:null\|)?(?:' . $stringish . ')(?:\|null)? cannot be identical to int$/', $msg)
        && preg_match('/^\$[\w>\-]+$|^[\w:\\\\]+\(.*\)$/s', $l)
    ) {
        $edits[$i['file_path']][] = [$i['from'] + $lo, $i['from'] + $lo + strlen($l), 'lookup', ''];
    }
}

// literals wrongly made Sym constants: compared with what is still a string
foreach ($issues as $i) {
    if (!in_array($i['type'], ['TypeDoesNotContainType', 'DocblockTypeContradiction', 'RedundantCondition'], true)) {
        continue;
    }
    $src = (string) file_get_contents($i['file_path']);
    $text = substr($src, $i['from'], $i['to'] - $i['from']);
    if (preg_match('/(?:===|!==|==|!=)\s*Sym::(\w+)\b/', $text, $m, PREG_OFFSET_CAPTURE) && isset($sym[$m[1][0]])
        && preg_match('/string/', $i['message']) && !preg_match('/^\s*(?:Interner::|\$\w+->(?:value|name)\b)/', $text)
    ) {
        $at = $i['from'] + $m[1][1] - 5;
        $edits[$i['file_path']][] = [$at, $at + 5 + strlen($m[1][0]), 'literal', $sym[$m[1][0]]];
    }
}

// ---- classes B-I of the remaining errors -------------------------------------------------------------------
foreach ($issues as $i) {
    $file = $i['file_path'];
    $msg = $i['message'];
    $text = $i['selected_text'];
    $from = $i['from'];
    $to = $i['to'];
    $t = $i['type'];
    // B. comparisons: `''` against an id is 0 (the empty name's id); a Sym against a php-parser / other string goes
    // back to its literal; strtolower((string) X) of an id is X
    if (in_array($t, ['TypeDoesNotContainType', 'DocblockTypeContradiction', 'RedundantCondition', 'RedundantConditionGivenDocblockType'], true)) {
        if (preg_match("/^'' cannot be identical to int|^int cannot be identical to ''|'' can never contain int|'' does not contain int/", $msg)
            && preg_match("/(===|!==|==|!=)\s*''|''\s*(===|!==|==|!=)/", $text, $em, PREG_OFFSET_CAPTURE)
        ) {
            $q = strpos($em[0][0], "''");
            $at = $from + $em[0][1] + $q;
            $edits[$file][] = [$at, $at + 2, 'replace', '0'];
            continue;
        }
        if (preg_match('/^\d{6,} cannot be identical to |\d{6,} can never contain (?!int)|is never =int\(\d+\)|^\d{6,} does not contain (?!int)/', $msg)
            && preg_match('/Sym::(\w+)/', $text, $sm, PREG_OFFSET_CAPTURE) && isset($sym[$sm[1][0]])
        ) {
            $at = $from + $sm[0][1];
            $edits[$file][] = [$at, $at + strlen($sm[0][0]), 'literal', $sym[$sm[1][0]]];
            continue;
        }
        if (preg_match('/lowercase-string does not contain int|int for \$\w+ is never =(?:non-empty-)?lowercase-string|does not contain (?:non-empty-)?lowercase-string/', $msg)
            && preg_match('/strtolower\(\s*\(string\)\s*/', $text, $lm, PREG_OFFSET_CAPTURE)
        ) {
            $at = $from + $lm[0][1];
            $inner = $at + strlen($lm[0][0]);
            $src = $sources[$file];
            $depth = 1;
            for ($k = $inner; $k < strlen($src) && $depth > 0; $k++) {
                $depth += $src[$k] === '(' ? 1 : ($src[$k] === ')' ? -1 : 0);
            }
            $edits[$file][] = [$at, $k, 'replace', trim(substr($src, $inner, $k - 1 - $inner))];
            continue;
        }
        // in_array(<id>, [literals]) / literal lists compared with ids
        if (str_contains($msg, 'Operand of type false is always falsy') && preg_match('/^in_array\(/', $text)
            && preg_match('/\[([^\[\]]*)\]/', $text, $am, PREG_OFFSET_CAPTURE)
        ) {
            if (preg_match_all("/'([A-Za-z_\\\\][A-Za-z0-9_\\\\]*)'/", $am[1][0], $lits, PREG_OFFSET_CAPTURE)) {
                foreach ($lits[0] as [$lit, $lo]) {
                    $at = $from + $am[1][1] + $lo;
                    $edits[$file][] = [$at, $at + strlen($lit), 'intern', ''];
                }
            }
            continue;
        }
    }
    // C. an id key into a name map declared with string keys: the map's keys are ids (pzoom keys every name map by
    // StrId) - its property docblock / constant keys flip; a local array of unknown origin looks the key up
    if (in_array($t, ['InvalidArrayOffset', 'PossiblyInvalidArrayOffset'], true)
        && preg_match("/on variable (.+?) using a int offset, expecting (?:(?:non-empty-)?(?:lowercase-)?string|'.*')$/", $msg, $vm)
        && preg_match('/^(?:\$[\w]+->|self::\$|static::\$|[\w\\\\]+::\$?)(\w+)((?:\[[^\]]*\])*)$/', $vm[1], $pm)
        && flipMapKeys($root, $pm[1], str_repeat('#v', substr_count($pm[2], '[')) . '#k', $edits, $sym, $sym_by_value)
    ) {
        continue;
    }
    if (in_array($t, ['InvalidArrayOffset', 'PossiblyInvalidArrayOffset'], true)
        && preg_match("/using a int offset, expecting (?:(?:non-empty-)?(?:lowercase-)?string|'.*')$/", $msg)
        && str_ends_with(rtrim($text), ']')
    ) {
        $src = $sources[$file];
        $close = $from + strlen(rtrim($text)) - 1;
        $depth = 0;
        for ($k = $close; $k > $from; $k--) {
            $depth += $src[$k] === ']' ? 1 : ($src[$k] === '[' ? -1 : 0);
            if ($depth === 0) {
                break;
            }
        }
        if ($k > $from && $k + 1 < $close) {
            $edits[$file][] = [$k + 1, $close, 'lookup', ''];
        }
        continue;
    }
    // D. a php-parser identifier node where an id is wanted: its name, interned
    if (in_array($t, ['InvalidArgument', 'PossiblyInvalidArgument'], true)
        && preg_match('/expects (?:null\|)?int(?:\|null)?, but (?:possibly different type )?(?:null\|)?PhpParser\\\\Node\\\\(?:Identifier|VarLikeIdentifier|Name)(?:\|null)? provided/', $msg)
    ) {
        $edits[$file][] = [$from, $to, 'replace', 'Interner::intern' . (str_contains($msg, 'null') ? 'OrNull(' . $text . '?->name)' : '(' . $text . '->name)')];
        continue;
    }
    // E. a nullable value into intern / lookup: the OrNull variant
    if ($t === 'PossiblyNullArgument' && preg_match('/^Argument 1 of Psalm\\\\Internal\\\\Interner::(intern|lookup)(List|Keys)? cannot be null/', $msg, $nm)) {
        $src = $sources[$file];
        $call = 'Interner::' . $nm[1] . ($nm[2] ?? '') . '(';
        $at = strrpos(substr($src, 0, $from), $call);
        if ($at !== false && ($nm[2] ?? '') === '') {
            $edits[$file][] = [$at, $at + strlen($call), 'replace', 'Interner::' . $nm[1] . 'OrNull('];
        }
        continue;
    }
    // F. a looked-up lowercase name where a lowercase-string is declared: lookupLc
    if ($t === 'ArgumentTypeCoercion' && preg_match('/expects (?:non-empty-)?lowercase-string, but parent type/', $msg)
        && preg_match('/^Interner::lookup\(/', $text)
    ) {
        $edits[$file][] = [$from, $from + strlen('Interner::lookup('), 'replace', 'Interner::lookupLc('];
        continue;
    }
    if ($t === 'ArgumentTypeCoercion' && preg_match('/expects array<lowercase-string, /', $msg) && preg_match('/^Interner::lookupKeys\(/', $text)) {
        $edits[$file][] = [$from, $from + strlen('Interner::lookupKeys('), 'replace', 'Interner::lookupLcKeys('];
        continue;
    }
    // G. stale docblocks: the type Psalm infers / the native one
    if ($t === 'InvalidReturnType' && preg_match("/The declared return type '.+' for .+ is incorrect, got '(.+)'$/", $msg, $gm)
        && preg_match('/^(?:null\|)?int(?:\|null)?$|^\?int$/', $gm[1])
        && preg_match('/^(?:null\||\?)?(?:' . $stringish . ')(?:\|null)?$/', strtolower(trim($text)))
        && inDocblock($sources[$file], $from)
    ) {
        $edits[$file][] = [$from, $to, 'replace', $gm[1]];
        continue;
    }
    if (in_array($t, ['MismatchingDocblockReturnType', 'MismatchingDocblockParamType', 'MismatchingDocblockPropertyType'], true)
        && preg_match("/has (?:incorrect|wrong) (?:return )?type '(.+?)', should be '(.+?)'/", $msg, $gm)
        && preg_match('/^(?:\?|null\|)?int(?:\|null)?$/', $gm[2]) && inDocblock($sources[$file], $from)
    ) {
        $edits[$file][] = [$from, $to, 'replace', $gm[2]];
        continue;
    }
    // H. a by-reference parameter written with ids: its declared type says ids
    if ($t === 'ReferenceConstraintViolation' && preg_match('/^Variable (\$\w+) is limited to values of type (null\|string|string) because it is passed by reference, int(?:\|null)? type found/', $msg, $rm)) {
        $src = $sources[$file];
        $fn = strrpos(substr($src, 0, $from), 'function ');
        if ($fn === false) {
            continue;
        }
        $sig_end = strpos($src, '{', $fn);
        $sig = substr($src, $fn, $sig_end - $fn);
        if (preg_match('/(\?string|string\|null|null\|string|string)(\s*&\s*' . preg_quote($rm[1], '/') . ')\b/', $sig, $pm2, PREG_OFFSET_CAPTURE)) {
            $at = $fn + $pm2[1][1];
            $edits[$file][] = [$at, $at + strlen($pm2[1][0]), 'replace', str_contains($pm2[1][0], 'null') || $pm2[1][0][0] === '?' ? '?int' : 'int'];
        }
        // and its docblock
        $doc_end = strrpos(substr($src, 0, $fn), '*/');
        $doc_start = $doc_end === false ? false : strrpos(substr($src, 0, $doc_end), '/**');
        if ($doc_start !== false && preg_match('/@param\s+(\S+)(\s+&?' . preg_quote($rm[1], '/') . ')\b/', substr($src, $doc_start, $doc_end - $doc_start), $dm, PREG_OFFSET_CAPTURE)) {
            $at = $doc_start + $dm[1][1];
            $edits[$file][] = [$at, $at + strlen($dm[1][0]), 'replace', str_contains($dm[1][0], 'null') ? 'int|null' : 'int'];
        }
        continue;
    }
    // an interner call around a callable (a closure variable passed to array_map): undone
    if (in_array($t, ['InvalidArgument', 'PossiblyInvalidArgument'], true) && preg_match('/expects (?:impure-)?callable/', $msg)
        && preg_match('/^Interner::(?:intern|lookup)(?:OrNull)?\((\$\w+)\)$/', $text, $cm2)
    ) {
        $edits[$file][] = [$from, $to, 'replace', $cm2[1]];
        continue;
    }
    // I. wraps that went wrong: an interned closure / callable, a looked-up by-reference argument
    if (in_array($t, ['InvalidArgument', 'PossiblyInvalidArgument'], true) && preg_match('/expects (?:impure-)?(?:callable|Closure)/i', $msg)
        && preg_match('/^Interner::(?:intern|lookup)(?:OrNull)?\((.*)\)$/s', $text, $um)
    ) {
        $edits[$file][] = [$from, $to, 'replace', $um[1]];
        continue;
    }
    if ($t === 'InvalidPassByReference' && preg_match('/^Interner::(?:intern|lookup)(?:OrNull)?\((.*)\)$/s', $text, $um)) {
        $edits[$file][] = [$from, $to, 'replace', $um[1]];
        continue;
    }
}

// ---- cleanup classes (after the id conversion settles) -------------------------------------------------------
foreach ($issues as $i) {
    $file = $i['file_path'];
    $msg = $i['message'];
    $text = $i['selected_text'];
    $from = $i['from'];
    $to = $i['to'];
    $t = $i['type'];
    // an interner call on a value that is both (a display string / a shape key with a name-like name): unwrap
    if ($t === 'PossiblyInvalidArgument'
        && preg_match('/^Argument 1 of Psalm\\\\Internal\\\\Interner::(lookup|intern)(?:OrNull)? expects (?:\?|null\|)?(?:int|string)(?:\|null)?, but possibly different type (.+) provided$/', $msg, $um2)
        && preg_match('/(^|\|)int(\||$)/', $um2[2]) && preg_match('/string/', $um2[2])
    ) {
        $src = $sources[$file];
        $call = strrpos(substr($src, 0, $from), 'Interner::' . $um2[1]);
        if ($call !== false) {
            $open = strpos($src, '(', $call);
            $close = matchParen($src, $open);
            if ($open < $from && $close >= $to && trim(substr($src, $open + 1, $close - $open - 1)) === $text) {
                $edits[$file][] = [$call, $close + 1, 'replace', $text];
            }
        }
        continue;
    }
    // truthiness of a name id (0 is the empty name) / of a name map: explicit, and stable under re-analysis
    if ($t === 'RiskyTruthyFalsyComparison' && !preg_match('/=== 0|!== 0|=== null|!== null|=== \[\]|!== \[\]|&&|\|\|/', $text)
        // `x ?: y` uses x as the value too: left as it is
        && !preg_match('/\G\s*\?:/', $sources[$file], $qm, 0, $to)
        && !preg_match('/(?:!==|===)\s*null\s*(?:&&|\|\|)\s*$|\?\?\s*$/', substr($sources[$file], max(0, $from - 24), min(24, $from)))
    ) {
        $neg = (bool) preg_match('/^!\s*(.+)$/s', $text, $nm);
        $x = trim($neg ? $nm[1] : $text);
        // `empty(x)` is `!x`
        if (preg_match('/^empty\((.*)\)$/s', $x, $em) && matchParen($x, 5) === strlen($x) - 1) {
            $x = trim($em[1]);
            $neg = !$neg;
        }
        if (preg_match('/^Operand of type (null\|)?int(\|null)? contains type int,/', $msg, $tm)) {
            // explicit comparisons keep Psalm's null narrowing
            $nullable = ($tm[1] ?? '') !== '' || ($tm[2] ?? '') !== '';
            $edits[$file][] = [$from, $to, 'replace', $nullable
                ? ($neg ? "($x === null || $x === 0)" : "($x !== null && $x !== 0)")
                : ($neg ? "$x === 0" : "$x !== 0")];
        } elseif (preg_match('/^Operand of type (?:non-empty-)?(?:array|list)<.*>(?:\|null)? contains type/', $msg)) {
            $nullable = str_contains(explode(' contains type', $msg)[0], '|null');
            $edits[$file][] = [$from, $to, 'replace', $nullable
                ? ($neg ? "($x === null || $x === [])" : "($x !== null && $x !== [])")
                : ($neg ? "$x === []" : "$x !== []")];
        }
        continue;
    }
    // a looked-up id used as a key of / appended to a name map still keyed by strings: the map's keys are ids
    if (in_array($t, ['PossiblyInvalidArrayOffset', 'InvalidArrayOffset'], true)
        && preg_match("/on variable (.+?) using a (?:string|non-empty-string|'.*') offset, expecting (?:non-empty-)?lowercase-string$/", $msg, $vm)
        && preg_match('/\[\s*Interner::lookup\(/', $text)
        && preg_match('/^(?:\$[\w]+->|self::\$|static::\$|[\w\\\\]+::\$?)(?:\w+->)*(\w+)((?:\[[^\]]*\])*)$/', $vm[1], $pm)
    ) {
        // lowercase-string keys are lowercased names in Psalm (variable ids are not lowercased): a name map
        flipMapKeys($root, $pm[1], str_repeat('#v', substr_count($pm[2], '[')) . '#k', $edits, $sym, $sym_by_value, true);
        continue;
    }
    if (in_array($t, ['PropertyTypeCoercion', 'InvalidPropertyAssignmentValue'], true)
        && preg_match("/expects 'array<(?:non-empty-)?lowercase-string, |with declared type 'array<(?:non-empty-)?lowercase-string, /", $msg)
        && preg_match('/^(?:\$\w+(?:->\w+)*->|self::\$|static::\$)(\w+)$/', $text, $pm)
        && preg_match('/' . preg_quote($text, '/') . '\[\s*Interner::lookup\(/', $sources[$file])
    ) {
        flipMapKeys($root, $pm[1], '#k', $edits, $sym, $sym_by_value);
        continue;
    }
    // a name map written with ids at a key level declared string: that level's keys are ids (pzoom: StrId keys)
    if (false && in_array($t, ['InvalidPropertyAssignmentValue', 'PossiblyInvalidPropertyAssignmentValue'], true)
        && preg_match('/^(?:[\w\\\\]+::\$|\$\w+(?:->\w+)*->)(\w+) with declared type \'(.+?)\' cannot be (?:possibly )?assigned (?:possibly different )?type \'(.+?)\'$/', $msg, $fm)
    ) {
        $declared_t = strtolower($fm[2]);
        $got_t = strtolower($fm[3]);
        foreach (['#k', '#v#k', '#v#v#k'] as $path) {
            try {
                $dk = implode('|', array_map(static fn(array $a): string => strtolower((string) ($a['name'] ?? '')),
                    array_merge(...array_map(static fn(array $u): array => $u['atoms'], TypeStr::at(TypeStr::parse($declared_t), $path) ?: [['atoms' => []]]))));
                $gk = implode('|', array_map(static fn(array $a): string => strtolower((string) ($a['name'] ?? '')),
                    array_merge(...array_map(static fn(array $u): array => $u['atoms'], TypeStr::at(TypeStr::parse($got_t), $path) ?: [['atoms' => []]]))));
            } catch (Throwable) {
                continue;
            }
            if (preg_match('/^(?:non-empty-)?(?:lowercase-)?string$/', $dk) && preg_match('/(^|\|)int(\||$)/', $gk)
                && preg_match('/string/', $gk)
            ) {
                $recv = preg_match('/^(\$\w+)(?:->\w+)*->' . $fm[1] . '\b/', $msg, $rv) ? (string) preg_replace('/.*->/', '', $rv[0] === '' ? '' : substr($msg, 0, strpos($msg, '->' . $fm[1]))) : '';
                flipMapKeys($root, $fm[1], $path, $edits, $sym, $sym_by_value, true, $recv);
            }
        }
        // no continue: the value / key rules below may apply too
    }
    // precise string types of looked-up names
    if (in_array($t, ['ArgumentTypeCoercion', 'PropertyTypeCoercion', 'LessSpecificReturnStatement', 'MoreSpecificReturnType'], true)
        && preg_match('/^Interner::lookup\(/', $text)
    ) {
        $variant = match (true) {
            (bool) preg_match('/expects (?:\S*\|)?class-string|declared return type \'class-string/', $msg) => 'lookupClass',
            (bool) preg_match('/expects (?:\S*\|)?callable-string/', $msg) => 'lookupCallable',
            (bool) preg_match('/expects (?:non-empty-)?lowercase-string/', $msg) => 'lookupLc',
            (bool) preg_match('/non-empty-string/', $msg) => 'lookupNonEmpty',
            default => null,
        };
        if ($variant !== null) {
            $edits[$file][] = [$from, $from + strlen('Interner::lookup('), 'replace', "Interner::$variant("];
        }
        continue;
    }
    // `$list[] = Interner::lookup($x)` into a list of class-strings
    if ($t === 'PropertyTypeCoercion' && preg_match("/expects '(?:non-empty-)?list<class-string/", $msg)
        && preg_match('/^(?:\$\w+(?:->\w+)*->|self::\$|static::\$)(\w+)$/', $text, $pm)
    ) {
        $src = $sources[$file];
        if (preg_match_all('/' . preg_quote($text, '/') . '\[\]\s*=\s*Interner::lookup\(/', $src, $am, PREG_OFFSET_CAPTURE)) {
            foreach ($am[0] as [$w, $off]) {
                $at = $off + strlen($w) - strlen('Interner::lookup(');
                $edits[$file][] = [$at, $at + strlen('Interner::lookup('), 'replace', 'Interner::lookupClass('];
            }
        }
        continue;
    }
    // a literal named by the message meeting an id: its Sym; `0` meeting a string: ''
    if (in_array($t, ['DocblockTypeContradiction', 'RedundantCondition', 'TypeDoesNotContainType', 'RedundantConditionGivenDocblockType'], true)) {
        if (preg_match("/^'([A-Za-z_\\\\][A-Za-z0-9_\\\\]*)' (?:does not contain|cannot be identical to|can never contain) (?:null\|)?int(?:\|null)?$/", $msg, $lm2)) {
            if (preg_match_all("/'" . preg_quote(str_replace('\\\\', '\\', $lm2[1]), '/') . "'/", $text, $om, PREG_OFFSET_CAPTURE)) {
                foreach ($om[0] as [$lit, $loff]) {
                    $edits[$file][] = [$from + $loff, $from + $loff + strlen($lit), 'intern', ''];
                }
            }
            continue;
        }
        if (preg_match('/^0 (?:does not contain|cannot be identical to|can never contain) (?:[\w-]*string)/', $msg)
            && preg_match('/(===|!==)\s*0\b/', $text, $zm, PREG_OFFSET_CAPTURE)
        ) {
            $at = $from + $zm[0][1] + strlen($zm[0][0]) - 1;
            $edits[$file][] = [$at, $at + 1, 'replace', "''"];
            continue;
        }
        if (str_contains($msg, 'Operand of type false is always falsy')
            && preg_match('/^in_array\(\s*(\$\w+(?:->\w+)*)\s*,\s*\[\s*Sym::/', $text, $im, PREG_OFFSET_CAPTURE)
        ) {
            $at = $from + $im[1][1];
            $edits[$file][] = [$at, $at + strlen($im[1][0]), 'intern', ''];
            continue;
        }
    }
    // a comparison across the two kinds: the string side joins the ids (names are ids everywhere)
    if (in_array($t, ['DocblockTypeContradiction', 'RedundantCondition', 'TypeDoesNotContainType', 'RedundantConditionGivenDocblockType'], true)
        && preg_match('/\bint\b/', $msg) && preg_match('/string|\'/', $msg)
        && preg_match('/^(.+?)\s*(===|!==)\s*(.+)$/s', $text, $cm, PREG_OFFSET_CAPTURE)
        && !preg_match('/&&|\|\|/', $text)
    ) {
        [$l, $lo] = $cm[1];
        [$r, $ro] = $cm[3];
        $kl = sideKind(trim($l));
        $kr = sideKind(trim($r));
        if (trim($r) === '0' && $kl !== 'id') {
            // a `=== 0` on what stayed a string: its empty value
            $edits[$file][] = [$from + $ro, $from + $ro + strlen($r), 'replace', "''"];
        } elseif ($kl === 'str' && $kr === 'id') {
            $edits[$file][] = [$from + $lo, $from + $lo + strlen(rtrim($l)), 'intern', ''];
        } elseif ($kl === 'id' && $kr === 'str') {
            $edits[$file][] = [$from + $ro, $from + $ro + strlen(rtrim($r)), 'intern', ''];
        }
        continue;
    }
    // a lowered name compared with an id (`strtolower($x)` / `strtolower((string) $x)`): names are ids
    if (in_array($t, ['DocblockTypeContradiction', 'RedundantCondition', 'TypeDoesNotContainType', 'RedundantConditionGivenDocblockType'], true)
        && preg_match('/lowercase-string/', $msg) && preg_match('/int/', $msg)
        && preg_match('/\\\\?strtolower\(\s*(?:\(string\)\s*)?/', $text, $lm, PREG_OFFSET_CAPTURE)
    ) {
        $at = $from + $lm[0][1];
        $inner = $at + strlen($lm[0][0]);
        $close = matchParen($sources[$file], $at + strpos($lm[0][0], '('));
        $x = trim(rtrim(substr($sources[$file], $inner, $close - $inner), ", \n\t"));
        // a looked-up id lowered: the id itself
        if (preg_match('/^Interner::lookup(?:OrNull)?\((.*)\)$/s', $x, $xm) && matchParen($x, strpos($x, '(')) === strlen($x) - 1) {
            $x = $xm[1];
        }
        $edits[$file][] = [$at, $close + 1, 'replace', $x];
        continue;
    }
    // a closure mapped over names typed the other way: its parameter follows the array
    if ($t === 'InvalidScalarArgument'
        && preg_match('/^Parameter 1 of closure passed to function array_(?:map|filter) expects (int|string), but (.+) provided$/', $msg, $cm)
        && preg_match('/^(static\s+)?(?:fn|function)\s*\(\s*(int|string)\s+\$/', $text, $pm, PREG_OFFSET_CAPTURE)
    ) {
        $got_int = (bool) preg_match('/^int$/', $cm[2]);
        $got_str = (bool) preg_match('/^(?:[\w-]*string|[\w\\\\]+::class)(?:\|(?:[\w-]*string|[\w\\\\]+::class))*$/', $cm[2]);
        if ($got_int || $got_str) {
            $at = $from + $pm[2][1];
            $edits[$file][] = [$at, $at + strlen($pm[2][0]), 'replace', $got_int ? 'int' : 'string'];
        }
        continue;
    }
    // a nullable value into lookup / intern: the OrNull variant (the argument's call just before it)
    if ($t === 'PossiblyNullArgument' && preg_match('/^Argument 1 of Psalm\\\\Internal\\\\Interner::(intern|lookup)(List|Keys|Lc|Class|NonEmpty|Callable)? cannot be null/', $msg, $nm)) {
        $src = $sources[$file];
        $call = 'Interner::' . $nm[1] . ($nm[2] ?? '') . '(';
        $at = strrpos(substr($src, 0, $from), $call);
        if ($at !== false && ($nm[2] ?? '') === '') {
            $edits[$file][] = [$at, $at + strlen($call), 'replace', 'Interner::' . $nm[1] . 'OrNull('];
        } elseif ($at !== false && !str_contains($text, '?? []')) {
            // a list / keys helper over a possibly-null array: `?? []` (once)
            $edits[$file][] = [$from, $to, 'replace', '(' . $text . ' ?? [])'];
        }
        continue;
    }
}

$fixed = 0;
foreach ($edits as $file => $list) {
    $src = (string) file_get_contents($file);
    $test = !str_contains($file, '/src/');
    // widest first at a position; drop exact duplicates; apply back to front
    $uniq = [];
    foreach ($list as $e) {
        $uniq[implode(':', $e)] = $e;
    }
    $list = array_values($uniq);
    usort($list, static fn($a, $b) => [$b[0], $a[1]] <=> [$a[0], $b[1]]);
    $last_from = PHP_INT_MAX;
    foreach ($list as [$from, $to, $kind, $arg]) {
        if ($to > $last_from) {
            continue; // overlaps an edit already made (the next round sees it again)
        }
        $text = substr($src, $from, $to - $from);
        $new = null;
        $um = [];
        if (in_array($kind, ['intern', 'lookup', 'call', 'at', 'paren-lookup'], true)
            && (isWriteTarget($src, $from, $to) || inIsset($src, $from))
        ) {
            continue; // an assignment target / an isset() operand is not a value
        }
        if ($kind === 'intern' && preg_match("/^'([A-Za-z_\\\\][A-Za-z0-9_\\\\]*)'$/", $text, $lm) && !$test) {
            $v = ltrim(stripcslashes($lm[1]), '\\');
            $new = 'Sym::' . symName($v, $sym, $sym_by_value);
        } elseif (($kind === 'intern' || $kind === 'lookup') && str_ends_with(rtrim(substr($src, max(0, $from - 10), min(10, $from))), '...')) {
            // a spread argument: its values
            $new = "Interner::{$kind}List($text)";
        } elseif ($kind === 'intern' || $kind === 'lookup') {
            $new = "Interner::$kind$arg($text)";
        } elseif ($kind === 'paren-lookup') {
            $new = "(Interner::lookup($text))";
        } elseif ($kind === 'replace') {
            $new = $arg;
        } elseif ($kind === 'intern' && $test && preg_match("/^'([A-Za-z_\\\\][A-Za-z0-9_\\\\]*)'$/", $text, $lm)) {
            $new = 'Interner::intern(' . var_export(CanonicalNames::of(stripcslashes($lm[1])), true) . ')';
        } elseif ($kind === 'sym') {
            $new = $test ? "Interner::intern($text)" : 'Sym::' . symName($arg, $sym, $sym_by_value);
        } elseif ($kind === 'call') {
            $new = "Interner::$arg($text)";
        } elseif ($kind === 'at') {
            [$fn, $ps] = explode(':', $arg, 2);
            $new = composeWrap($fn, explode(',', $ps), $text);
        } elseif ($kind === 'literal') {
            $new = var_export($arg, true);
        }
        // the opposite wrap already there (an earlier round's, before the other side changed kind): unwrap it
        if (($kind === 'intern' || $kind === 'lookup')
            && preg_match('/^Interner::' . ($kind === 'intern' ? 'lookup' : 'intern') . '(?:OrNull)?\((.*)\)$/s', $text, $um)
            && matchParen($text, strpos($text, '(')) === strlen($text) - 1
        ) {
            $new = $um[1];
        }
        // never a wrap around an interner call (a round trip, or a site the wrap does not satisfy), nor a closure
        if ($new === null || ($kind !== 'replace' && str_starts_with(ltrim($text, '('), 'Interner::') && !isset($um[1]))
            || preg_match('/^(static\s+)?(fn|function)\s*\(/', $text)
        ) {
            continue;
        }
        $src = substr($src, 0, $from) . $new . substr($src, $to);
        $last_from = $from;
        $fixed++;
    }
    // imports
    $ns = preg_match('/^namespace\s+([^;{\s]+)/m', $src, $nm) ? $nm[1] : '';
    foreach (['Interner', 'Sym'] as $cls) {
        if (preg_match('/(?<![\w\\\\$>])' . $cls . '::/', $src) && $ns !== 'Psalm\\Internal' && $ns !== ''
            && !preg_match('/^use\s+Psalm\\\\Internal\\\\' . $cls . ';/m', $src)
        ) {
            $src = (string) preg_replace('/^(namespace [^;]+;\n)/m', "$1\nuse Psalm\\\\Internal\\\\$cls;\n", $src, 1);
        }
    }
    file_put_contents($file, $src);
}

// round trips: lookup(intern(X)) / intern(lookup(X)) are X; names are case-sensitive (pzoom): no lowering of a
// looked-up name, nor of a string about to be interned
$all = array_keys($edits);
foreach (new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root . 'src', FilesystemIterator::SKIP_DOTS)) as $f) {
    if (str_ends_with((string) $f, '.php')) {
        $all[] = (string) $f;
    }
}
foreach (array_unique($all) as $file) {
    $src = (string) file_get_contents($file);
    $before = $src;
    $src = dropLowering($src);
    $src = keepIdKeys($src, $root);
    if (str_contains($file, '/src/')) {
        $src = (string) preg_replace_callback("/Interner::intern\\('([A-Za-z_\\\\][A-Za-z0-9_\\\\]*)'\\)/", static function (array $m) use (&$sym, &$sym_by_value): string {
            return 'Sym::' . symName(stripcslashes($m[1]), $sym, $sym_by_value);
        }, $src);
    }
    if ($src !== $before) {
        file_put_contents($file, $src);
    }
}
foreach (array_keys($edits) as $file) {
    $src = (string) file_get_contents($file);
    $changed = false;
    while (preg_match('/Interner::(lookup|intern)(OrNull)?\(\s*Interner::(intern|lookup)(OrNull)?\(/', $src, $pm, PREG_OFFSET_CAPTURE)) {
        if ($pm[1][0] === $pm[3][0]) {
            break;
        }
        $start = $pm[0][1];
        $inner = $start + strlen($pm[0][0]);
        $depth = 2;
        $inner_end = null;
        for ($k = $inner; $k < strlen($src) && $depth > 0; $k++) {
            if ($src[$k] === '(') {
                $depth++;
            } elseif ($src[$k] === ')') {
                $depth--;
                if ($depth === 1 && $inner_end === null) {
                    $inner_end = $k;
                }
            }
        }
        if ($inner_end === null || trim(substr($src, $inner_end + 1, $k - 1 - ($inner_end + 1))) !== '') {
            break;
        }
        $src = substr($src, 0, $start) . substr($src, $inner, $inner_end - $inner) . substr($src, $k);
        $changed = true;
    }
    if ($changed) {
        file_put_contents($file, $src);
    }
}

// new Sym constants
$gen = $root . 'bin/generate-sym.php';
$g = (string) file_get_contents($gen);
foreach ($sym_by_value as $v => $c) {
    $line = '    ' . var_export($c, true) . ' => ' . var_export((string) $v, true) . ",\n";
    if (!str_contains($g, $line)) {
        $g = str_replace("\$names = [\n", "\$names = [\n" . $line, $g);
    }
}
file_put_contents($gen, $g);
passthru('php ' . escapeshellarg($gen) . ' > /dev/null');
echo "fixed $fixed sites\n";

function symName(string $v, array &$sym, array &$by_value): string
{
    $v = CanonicalNames::of($v);
    if (isset($by_value[$v])) {
        return $by_value[$v];
    }
    $words = [];
    foreach (explode('\\', $v) as $p) {
        $words[] = strtoupper((string) preg_replace('/(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])/', '_', $p));
    }
    $name = 'C_' . preg_replace('/[^A-Z0-9_]/', '_', implode('__', $words));
    for ($k = 2; isset($sym[$name]); $k++) {
        $name = preg_replace('/_\d+$/', '', $name) . '_' . $k;
    }
    $sym[$name] = $v;
    $by_value[$v] = $name;
    return $name;
}

/**
 * The array paths where the wanted type has ids and the given one strings (intern) or the reverse (lookup).
 *
 * @return ?array{intern: list<string>, lookup: list<string>}
 */
function arrayPaths(string $want, string $got, string $stringish, string $intish): ?array
{
    if (!preg_match('/^(non-empty-)?(array|list|iterable)[<{]/', $want) || !preg_match('/^(non-empty-)?(array|list|iterable)[<{]/', $got)) {
        return null;
    }
    try {
        $w = TypeStr::parse($want);
        $g = TypeStr::parse($got);
    } catch (Throwable) {
        return null;
    }
    $out = ['intern' => [], 'lookup' => []];
    $kind = static function (array $atoms) use ($stringish, $intish): ?string {
        $k = null;
        foreach ($atoms as $a) {
            $n = strtolower((string) ($a['name'] ?? ''));
            if ($a['kind'] === 'literal') {
                $n = str_starts_with((string) $a['name'], "'") || str_starts_with((string) $a['name'], '"') ? 'string' : 'int';
            }
            if ($n === 'null' || $n === '') {
                continue;
            }
            $x = preg_match('/^(?:' . $intish . ')$/', $n) ? 'int' : (preg_match('/^(?:' . $stringish . ')$/', $n) ? 'string' : 'other');
            if ($k !== null && $k !== $x) {
                return 'mixed';
            }
            $k = $x;
        }
        return $k;
    };
    foreach (['#k', '#v', '#v#k', '#v#v', '#v#v#k', '#v#v#v', '#k#v'] as $p) {
        try {
            $wa = array_merge(...array_map(static fn(array $u): array => $u['atoms'], TypeStr::at($w, $p) ?: [['atoms' => []]]));
            $ga = array_merge(...array_map(static fn(array $u): array => $u['atoms'], TypeStr::at($g, $p) ?: [['atoms' => []]]));
        } catch (Throwable) {
            continue;
        }
        $wk = $kind($wa);
        $gk = $kind($ga);
        if ($wk === 'int' && $gk === 'string') {
            $out['intern'][] = $p;
        } elseif ($wk === 'string' && $gk === 'int') {
            $out['lookup'][] = $p;
        }
    }
    return $out['intern'] === [] && $out['lookup'] === [] ? null : $out;
}

/**
 * Flips the keys at a path of the one property or class constant named $name to ids: a property's docblock type, a
 * constant's literal keys (Sym constants). False when the name is declared nowhere or more than once.
 *
 * @param array<string, list<array{int, int, string, string}>> $edits
 */
function flipMapKeys(string $root, string $name, string $path, array &$edits, array &$sym, array &$by_value, bool $lc_keys = false, string $receiver = ''): bool
{
    static $decls = null;
    if ($decls === null) {
        $decls = [];
        $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root . 'src', FilesystemIterator::SKIP_DOTS));
        foreach ($it as $f) {
            if (!str_ends_with((string) $f, '.php')) {
                continue;
            }
            $src = (string) file_get_contents((string) $f);
            if (preg_match_all('/^[ \t]*(?:(?:public|protected|private|static|readonly|final)\s+)+(?:[?\w|\\\\]+\s+)?\$(\w+)\b|^[ \t]*(?:(?:public|protected|private|final)\s+)*const\s+(?:\w+\s+)?(\w+)\s*=/m', $src, $m, PREG_OFFSET_CAPTURE | PREG_SET_ORDER)) {
                foreach ($m as $d) {
                    $n = ($d[2][0] ?? '') !== '' ? $d[2][0] : $d[1][0];
                    $decls[$n][] = [(string) $f, $d[0][1], ($d[2][0] ?? '') !== ''];
                }
            }
        }
    }
    $candidates = $decls[$name] ?? [];
    if (count($candidates) > 1 && $receiver !== '') {
        // the receiver's name says its class (`$template_result` -> TemplateResult.php)
        $want = str_replace('_', '', strtolower(ltrim($receiver, '$')));
        $candidates = array_values(array_filter($candidates, static fn(array $c): bool
            => strtolower(basename($c[0], '.php')) === $want));
    }
    if (count($candidates) !== 1 || (!$lc_keys && notNameMap($name))) {
        return false;
    }
    [$file, $at, $is_const] = $candidates[0];
    $src = (string) file_get_contents($file);
    if ($is_const) {
        // the constant's literal keys (at the top level) become Sym constants
        if ($path !== '#k' || str_contains($file, '/tests/')) {
            return false;
        }
        $open = strpos($src, '[', $at);
        $end = strpos($src, '];', $at);
        if ($open === false || $end === false) {
            return false;
        }
        if (!preg_match_all("/^\s*'([A-Za-z_\\\\][A-Za-z0-9_\\\\-]*)'\s*=>/m", substr($src, $open, $end - $open), $km, PREG_OFFSET_CAPTURE)) {
            return false;
        }
        foreach ($km[1] as [$lit, $off]) {
            $q = $open + $off - 1;
            $edits[$file][] = [$q, $q + strlen($lit) + 2, 'replace', 'Sym::' . symName(stripcslashes($lit), $sym, $by_value)];
        }
        return true;
    }
    // the property's docblock just above it
    $doc_end = strrpos(substr($src, 0, $at), '*/');
    $doc_start = $doc_end === false ? false : strrpos(substr($src, 0, $doc_end), '/**');
    if ($doc_start === false || trim(substr($src, $doc_end + 2, $at - $doc_end - 2)) !== '') {
        return false;
    }
    $doc = substr($src, $doc_start, $doc_end - $doc_start);
    if (!preg_match('/@(?:psalm-)?var\s+/', $doc, $tm, PREG_OFFSET_CAPTURE)) {
        return false;
    }
    $tstart = $tm[0][1] + strlen($tm[0][0]);
    $flat = (string) preg_replace_callback('/\n\s*\*/', static fn(array $x): string => str_repeat(' ', strlen($x[0])), substr($doc, $tstart));
    try {
        $end = TypeStr::parse($flat)['end'];
        $type = substr($flat, 0, $end);
        $te = TypeStr::toIntAt($type, $path);
    } catch (Throwable) {
        return false;
    }
    if ($te === []) {
        return false;
    }
    foreach ($te as [$a, $b, $txt]) {
        $edits[$file][] = [$doc_start + $tstart + $a, $doc_start + $tstart + $b, 'replace', $txt];
    }
    return true;
}

/** Whether an offset lies inside a docblock. */
function inDocblock(string $src, int $at): bool
{
    $open = strrpos(substr($src, 0, $at), '/**');
    if ($open === false) {
        return false;
    }
    $close = strpos($src, '*/', $open);
    return $close !== false && $close > $at;
}

/** A map whose name says its keys are not names (variable ids, files, keys, types...): it keeps string keys. */
function notNameMap(string $name): bool
{
    $words = array_filter(explode('_', strtolower((string) preg_replace('/(?<=[a-z0-9])(?=[A-Z])/', '_', $name))));
    return array_intersect($words, ['id', 'ids', 'var', 'vars', 'file', 'files', 'path', 'paths', 'key', 'keys',
        'type', 'types', 'scope', 'clauses', 'node', 'nodes', 'issue', 'issues', 'message', 'messages', 'code',
        'references', 'reference', 'location', 'locations', 'map', 'maps', 'hashes', 'hash', 'data']) !== [];
}

/** Whether the range [from, to) is (part of) an assignment target: `X = `, `[X, ...] = `, `list(X) = `. */
function isWriteTarget(string $src, int $from, int $to): bool
{
    if (preg_match('/\\G\\s*(?:=(?![=>])|\\.=|\\+=|\\?\\?=)/', $src, $m, 0, $to)) {
        return true;
    }
    // the array an element of is written: `X[k] = `, `X[k][] = `
    $k = $to;
    while (preg_match('/\\G\\s*\\[/', $src, $m, 0, $k)) {
        $depth = 0;
        for ($k += strlen($m[0]) - 1; $k < strlen($src); $k++) {
            $depth += $src[$k] === '[' ? 1 : ($src[$k] === ']' ? -1 : 0);
            if ($depth === 0) {
                break;
            }
        }
        $k++;
        if (preg_match('/\\G\\s*(?:=(?![=>])|\\.=|\\+=|\\?\\?=)/', $src, $m, 0, $k)) {
            return true;
        }
    }
    // inside `[ ... ] =` / `list( ... ) =`
    $depth = 0;
    for ($k = $to; $k < strlen($src) && $k < $to + 2000; $k++) {
        $c = $src[$k];
        if ($c === '(' || $c === '[') {
            $depth++;
        } elseif ($c === ')' || $c === ']') {
            if ($depth === 0) {
                return (bool) preg_match('/\\G\\s*=(?![=>])/', $src, $m, 0, $k + 1)
                    && preg_match('/(?:^|[\\s;{(,])(?:\\[|list\\()\\s*$/', substr($src, max(0, $from - 200), 200)) === 1
                    || (bool) preg_match('/\\G\\s*=(?![=>])/', $src, $m, 0, $k + 1) && isDestructureStart($src, $from);
            }
            $depth--;
        } elseif ($c === ';' || $c === '{') {
            return false;
        }
    }
    return false;
}

/** Whether a `[` / `list(` opening a statement-level destructuring encloses $at. */
function isDestructureStart(string $src, int $at): bool
{
    $depth = 0;
    for ($k = $at - 1; $k >= 0 && $k > $at - 2000; $k--) {
        $c = $src[$k];
        if ($c === ')' || $c === ']') {
            $depth++;
        } elseif ($c === '(' || $c === '[') {
            if ($depth === 0) {
                $before = rtrim(substr($src, max(0, $k - 20), $k - max(0, $k - 20)));
                return $c === '[' ? (bool) preg_match('/(?:[;{}]|^)$/', $before) || $before === '' || str_ends_with($before, 'foreach (') || str_ends_with($before, ' as')
                    : str_ends_with($before, 'list');
            }
            $depth--;
        } elseif ($c === ';' || $c === '{' || $c === '}') {
            return false;
        }
    }
    return false;
}

/** The end of the `{ ... }` body of the function starting at $fn. */
function enclosingEnd(string $src, int $fn): ?int
{
    $open = strpos($src, '{', $fn);
    if ($open === false) {
        return null;
    }
    $depth = 0;
    for ($k = $open; $k < strlen($src); $k++) {
        if ($src[$k] === '{') {
            $depth++;
        } elseif ($src[$k] === '}') {
            $depth--;
            if ($depth === 0) {
                return $k;
            }
        }
    }
    return null;
}

/** A variable whose name says it holds a name (the textual pass made it an id). */
function isNameVar(string $v): bool
{
    $w = array_values(array_filter(explode('_', strtolower((string) preg_replace('/(?<=[a-z0-9])(?=[A-Z])/', '_', ltrim($v, '$'))))));
    $names = ['class', 'classes', 'classlike', 'classlikes', 'fqcln', 'fqcn', 'interface', 'interfaces', 'trait', 'traits',
        'enum', 'method', 'methods', 'function', 'functions', 'property', 'properties', 'prop', 'props', 'constant',
        'constants', 'const', 'parent', 'parents', 'mixin'];
    return array_intersect($w, $names) !== [] && in_array(end($w), [...$names, 'name', 'names', 'lc', 'fqcln'], true)
        && array_intersect($w, ['id', 'ids', 'file', 'path', 'type', 'key', 'var', 'string', 'fake']) === [];
}

/**
 * Names are case-sensitive ids (pzoom): `strtolower(Interner::lookup(X))` is `Interner::lookup(X)` and
 * `Interner::intern(strtolower(X))` is `Interner::intern(X)`.
 */
function dropLowering(string $src): string
{
    $patterns = [
        '/(?<![\w:>$\\\\])(\\\\?strtolower)\(\s*(?=Interner::lookup)/',
        '/Interner::intern(?:OrNull)?\(\s*(\\\\?strtolower)\(/',
    ];
    foreach ($patterns as $re) {
        for ($guard = 0; $guard < 10000 && preg_match($re, $src, $m, PREG_OFFSET_CAPTURE); $guard++) {
            $call = $m[1][1];
            $paren = $call + strlen($m[1][0]);
            $depth = 0;
            for ($e = $paren; $e < strlen($src); $e++) {
                $depth += $src[$e] === '(' ? 1 : ($src[$e] === ')' ? -1 : 0);
                if ($depth === 0) {
                    break;
                }
            }
            $inner = trim(rtrim(substr($src, $paren + 1, $e - $paren - 1), ", \n\t"));
            // only class-like names are case-sensitive ids so far (member and function maps are still keyed by
            // lowercase names); a path is no name at all
            $last = preg_match('/(\w+)\)*\s*$/', $inner, $lw) ? strtolower((string) preg_replace('/(?<=[a-z0-9])(?=[A-Z])/', '_', $lw[1])) : '';
            $class_like = (bool) preg_match('/(?:^|_)(?:class|classlike|classes|fqcln|fqcn|interface|trait|parent|self|enum|mixin)(?:_|$)/', $last)
                && !preg_match('/method|function|property|prop|const/', $last)
                // a class storage's name (`$storage->name`, `$dependency->name`, `$class_storage->name`)
                || (bool) preg_match('/(?:storage|dependency|dependent|class\w*|parent\w*|interface\w*)->name\s*\)*\s*$/', $inner);
            if (!$class_like || preg_match('/file|path|dir|extension|ext\b/i', $inner)) {
                // a path (not a name): Psalm keys files case-insensitively; mark it so the pattern moves on
                $src = substr($src, 0, $call) . "\x01" . substr($src, $call + 1);
                continue;
            }
            $src = substr($src, 0, $call) . $inner . substr($src, $e + 1);
        }
    }
    // restore the skipped calls' first letters (`\x01` marked `strtolower` / `\strtolower`)
    return str_replace(["\x01trtolower", "\x01strtolower"], ['strtolower', '\\strtolower'], $src);
}

/**
 * `array_merge(...)` / `[...$a, ...$b]` renumber int keys: over maps keyed by ids they become `array_replace(...)`.
 */
function keepIdKeys(string $src, string $root): string
{
    static $props = null;
    $props ??= array_flip(json_decode((string) @file_get_contents($root . '.id-keyed.json'), true) ?: []);
    if ($props === []) {
        return $src;
    }
    $mentions = static function (string $args) use ($props): bool {
        if (preg_match_all('/(?:->|::\$)(\w+)\b(?!\s*\()/', $args, $m)) {
            foreach ($m[1] as $p) {
                if (isset($props[$p])) {
                    return true;
                }
            }
        }
        return false;
    };
    // array_merge(...)
    $out = '';
    $pos = 0;
    while (preg_match('/(?<![\w:>$\\\\])array_merge\(/', $src, $m, PREG_OFFSET_CAPTURE, $pos)) {
        $at = $m[0][1];
        $open = $at + strlen('array_merge');
        $close = matchParen($src, $open);
        $args = substr($src, $open + 1, $close - $open - 1);
        $out .= substr($src, $pos, $at - $pos) . ($mentions($args) ? 'array_replace(' : 'array_merge(');
        $pos = $open + 1;
    }
    $src = $out . substr($src, $pos);
    // [...$a, ...$b] with only spreads
    return (string) preg_replace_callback('/\[(\s*\.\.\.[^\[\]]+?(?:,\s*\.\.\.[^\[\]]+?)+,?\s*)\]/', static function (array $m) use ($mentions): string {
        if (!$mentions($m[1])) {
            return $m[0];
        }
        $items = array_filter(array_map('trim', explode(',', $m[1])), static fn(string $x): bool => $x !== '');
        if (count(array_filter($items, static fn(string $x): bool => !str_starts_with($x, '...'))) > 0) {
            return $m[0];
        }
        return 'array_replace(' . implode(', ', array_map(static fn(string $x): string => substr($x, 3), $items)) . ')';
    }, $src);
}

function matchParen(string $src, int $open): int
{
    $depth = 0;
    for ($k = $open; $k < strlen($src); $k++) {
        $depth += $src[$k] === '(' ? 1 : ($src[$k] === ')' ? -1 : 0);
        if ($depth === 0) {
            return $k;
        }
    }
    return strlen($src) - 1;
}

/**
 * The typed expression mapping the names at array paths ("#k", "#v", "#v#k"...) of X through intern / lookup:
 * `Interner::lookupKeys(...)`, `Interner::lookupList(...)`, and `array_map` of those for nested levels.
 */
function composeWrap(string $fn, array $paths, string $x): string
{
    $here = [];
    $below = [];
    foreach ($paths as $p) {
        if ($p === '#k' || $p === '#v') {
            $here[] = $p;
        } elseif (str_starts_with($p, '#v')) {
            $below[] = substr($p, 2);
        }
    }
    if ($below !== []) {
        sort($below);
        if ($below === ['#k']) {
            $x = "Interner::{$fn}KeysEach($x)";
        } elseif ($below === ['#v']) {
            $x = "Interner::{$fn}ListEach($x)";
        } else {
            return "Interner::{$fn}At($x, " . implode(', ', array_map(static fn(string $p): string => var_export($p, true), $paths)) . ')';
        }
    }
    if (in_array('#v', $here, true)) {
        $x = "Interner::{$fn}List($x)";
    }
    if (in_array('#k', $here, true)) {
        $x = "Interner::{$fn}Keys($x)";
    }
    return $x;
}

/** Whether a range starting at $at is itself an operand of `isset(...)` / `unset(...)` (not a key inside one). */
function inIsset(string $src, int $at): bool
{
    $before = rtrim(substr($src, max(0, $at - 40), min(40, $at)));
    return (bool) preg_match('/(?:isset|unset)\s*\($/', $before);
}

/** What a comparison operand holds, by its form: 'id', 'str', or null (unknown). */
function sideKind(string $e): ?string
{
    if (preg_match("/^'[^']*'$|^\"[^\"]*\"$|^\(string\)|^Interner::lookup|^\\\\?strtolower\(|\s\.\s|^substr\(|^str_|->toString\(\)$|->name->name$/", $e)) {
        return 'str';
    }
    if (preg_match('/^Interner::intern|^Sym::|^0$/', $e)) {
        return 'id';
    }
    $last = preg_match('/(\$?\w+)\s*\)*$/', $e, $m) ? $m[1] : '';
    if ($last !== '' && (isNameVar($last) || in_array(ltrim($last, '$'), ['value', 'self', 'cased_name', 'defining_fqcln', 'fq_class_name', 'name'], true)) && !str_contains($e, '(')) {
        return 'id';
    }
    return null;
}
