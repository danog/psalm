<?php
// solve.php facts.jsonl [--dry]: from IdFactsPlugin facts, the largest set of (method, string parameter) pairs that can
// become interned ids (a fixed point: every recorded caller passes an id form), and the edits that do it:
//   - the parameter: `string $x` -> `int $x_id` (and its @param);
//   - its uses: in an id position (a converted twin-call argument or an argument to another converted parameter)
//     `$x_id`, elsewhere `Interner::lookup($x_id)` (pzoom's interner.lookup(id));
//   - callers: their argument's id form; twin calls whose class arguments all have id forms: the id twin.
declare(strict_types=1);
$in = $argv[1] ?? exit("usage: solve.php facts.jsonl [--dry]\n");
$dry = in_array('--dry', $argv, true);
$fns = []; $calls = []; $twins = []; $syms = []; $locals = [];
foreach (file($in, FILE_IGNORE_NEW_LINES) as $line) {
    $r = json_decode($line, true);
    switch ($r['kind']) {
        case 'fn': $fns[$r['id']] = $r; break;
        case 'call': $calls[$r['site'] . $r['callee']] = $r; break;
        case 'twin': $twins[$r['site']] = $r; break;
        case 'sym': $syms[$r['name']] = $r['value']; break;
        case 'local': $locals[$r['f'] . '|' . $r['p']] = $r; break;
    }
}
// candidates: eligible parameters of fixed-signature methods
$C = []; $why = [];
foreach ($fns as $id => $f) {
    foreach ($f['params'] as $p => $info) {
        if (!$f['fixed']) { $why['signature not fixed'] = ($why['signature not fixed'] ?? 0) + 1; continue; }
        if ($info['bad'] !== null) { $why['param: ' . $info['bad']] = ($why['param: ' . $info['bad']] ?? 0) + 1; continue; }
        $C["$id|$p"] = true;
    }
}
$why += ['signature not fixed' => 0];
$argOk = static function (array $a) use (&$C, &$locals): bool {
    return $a['k'] === 'edit' || ($a['k'] === 'param' && (isset($C[$a['f'] . '|' . $a['p']]) || isset($locals[$a['f'] . '|' . $a['p']])));
};
// call facts by callee parameter
$callers = [];
foreach ($calls as $c) { foreach ($c['args'] as $p => $a) { $callers[$c['callee'] . '|' . $p][] = [$c, $a]; } }
for ($round = 0; ; $round++) {
    $removed = 0;
    foreach (array_keys($C) as $key) {
        // every caller must pass an id form (an absent argument cannot happen: no defaults)
        foreach ($callers[$key] ?? [] as [$c, $a]) {
            if (!$argOk($a)) { unset($C[$key]); $removed++; $why['caller arg: ' . ($a['k'] === 'param' ? 'var' : ($a['shape'] ?? $a['k']))] = ($why['caller arg: ' . ($a['k'] === 'param' ? 'var' : ($a['shape'] ?? $a['k']))] ?? 0) + 1; continue 2; }
        }
        // and the parameter must reach at least one id position: a twin call that converts, or a converted parameter
        [$fid, $p] = explode('|', $key, 2);
        $useful = false;
        foreach ($twins as $t) {
            $all = true; $mine = false;
            foreach ($t['args'] as $a) {
                if (!$argOk($a)) { $all = false; }
                if ($a['k'] === 'param' && $a['f'] === $fid && $a['p'] === $p) { $mine = true; }
            }
            if ($all && $mine) { $useful = true; break; }
        }
        if (!$useful) {
            foreach ($calls as $c) {
                foreach ($c['args'] as $cp => $a) {
                    if ($a['k'] === 'param' && $a['f'] === $fid && $a['p'] === $p && isset($C[$c['callee'] . '|' . $cp])) { $useful = true; break 2; }
                }
            }
        }
        if (!$useful) { unset($C[$key]); $removed++; $why['no id use'] = ($why['no id use'] ?? 0) + 1; }
    }
    if ($removed === 0) { break; }
}
// edits
$edits = []; // file => [start:end => [s, e, text]]
$add = static function (string $file, array $e) use (&$edits): void { $edits[$file][$e[0] . ':' . $e[1]] = $e; };
$idUse = []; // "file|start" of parameter uses that stand in an id position
foreach ($twins as $t) {
    $all = true;
    foreach ($t['args'] as $a) { if (!$argOk($a)) { $all = false; } }
    if (!$all) { continue; }
    $add($t['file'], $t['rename']);
    foreach ($t['args'] as $a) {
        if ($a['k'] === 'edit') { $add($t['file'], $a['e']); } else { $idUse[$t['file'] . '|' . $a['r'][0]] = true; }
    }
}
foreach ($calls as $c) {
    foreach ($c['args'] as $cp => $a) {
        if (!isset($C[$c['callee'] . '|' . $cp])) { continue; }
        if ($a['k'] === 'edit') { $add($c['file'], $a['e']); } else { $idUse[$c['file'] . '|' . $a['r'][0]] = true; }
        if (isset($a['named'])) { $add($c['file'], [$a['named'][0], $a['named'][1], $cp . '_id']); }
    }
}
$lookups = 0; $locals_used = 0;
foreach ($locals as $key => $l) {
    $used = false;
    foreach ($l['uses'] as [$s, $e]) {
        if (isset($idUse[$l['file'] . '|' . $s])) { $add($l['file'], [$s, $e, '$' . $l['p'] . '_id']); $used = true; }
    }
    if ($used) { $add($l['file'], [$l['insert'][0], $l['insert'][0], $l['insert'][1]]); $locals_used++; }
}
foreach (array_keys($C) as $key) {
    [$fid, $p] = explode('|', $key, 2);
    $f = $fns[$fid]; $info = $f['params'][$p]; $file = $f['file'];
    $add($file, [$info['type'][0], $info['type'][1], 'int']);
    $add($file, [$info['var'][0], $info['var'][1], '$' . $p . '_id']);
    if ($info['doc'] !== null) {
        $add($file, [$info['doc'][0][0], $info['doc'][0][1], 'int']);
        $add($file, [$info['doc'][1][0], $info['doc'][1][1], '$' . $p . '_id']);
    }
    foreach ($info['uses'] as [$s, $e]) {
        if (isset($idUse[$file . '|' . $s])) { $add($file, [$s, $e, '$' . $p . '_id']); }
        else { $add($file, [$s, $e, 'Interner::lookup($' . $p . '_id)']); $lookups++; }
    }
}
$n = 0; $overlaps = 0;
foreach ($edits as $file => $list) {
    usort($list, fn($a, $b) => $b[0] <=> $a[0]);
    $src = file_get_contents($file); $prev = PHP_INT_MAX;
    foreach ($list as [$s, $e, $t]) {
        if ($e > $prev) { $overlaps++; fwrite(STDERR, "overlap $file@$s\n"); continue; }
        $src = substr($src, 0, $s) . $t . substr($src, $e); $prev = $s; $n++;
    }
    foreach (['Sym' => 'Sym::', 'Interner' => 'Interner::'] as $cls => $needle) {
        $line = "use Psalm\\Internal\\$cls;";
        if (!str_contains($src, $needle) || str_contains($src, $line) || preg_match('/^namespace Psalm\\\\Internal;$/m', $src)) { continue; }
        preg_match_all('/^use [A-Z][^;(]*;$/m', $src, $m, PREG_OFFSET_CAPTURE);
        $at = null; $last = null;
        foreach ($m[0] as [$u, $off]) {
            if (str_starts_with($u, 'use function') || str_starts_with($u, 'use const')) { continue; }
            if ($at === null && strcmp($u, $line) > 0) { $at = $off; }
            $last = $off + strlen($u) + 1;
        }
        $at ??= $last ?? (strpos($src, "\n", strpos($src, 'namespace ')) + 1);
        $src = substr($src, 0, $at) . $line . "\n" . substr($src, $at);
    }
    if (!$dry) { file_put_contents($file, $src); }
}
$symfile = __DIR__ . '/../../src/Psalm/Internal/Sym.php'; $sym_src = file_get_contents($symfile); $added = 0;
foreach ($syms as $name => $value) {
    if (!preg_match('/\bSym::' . preg_quote($name) . '\b/', implode('', array_map(fn($l) => implode('', array_column($l, 2)), $edits)))) { continue; }
    if (preg_match('/const ' . preg_quote($name) . '\b/', $sym_src)) { continue; }
    $pos = strrpos($sym_src, '}');
    $sym_src = substr($sym_src, 0, $pos) . "\n    /** generated by tools/id-refactor */\n    public const $name = $value;\n" . substr($sym_src, $pos);
    $added++;
}
if (!$dry && $added) { file_put_contents($symfile, $sym_src); }
arsort($why); foreach (array_slice($why, 0, 12, true) as $k => $v) { printf("  removed %6d  %s\n", $v, $k); }
// report: what still blocks
$block = [];
foreach ($twins as $t) { foreach ($t['args'] as $a) { if (!$argOk($a)) {
    $bwhy = $a["k"] === "param" ? "param not convertible: " . $a["f"] . "|" . $a["p"] : ($a["shape"] ?? "?");
    $block[$bwhy][] = $t["site"];
} } }
uasort($block, fn($a, $b) => count($b) <=> count($a));
printf("locals given an id twin %d\n", $locals_used);
printf("params converted %d (rounds %d), files %d, edits %d, overlaps %d, lookups %d, sym added %d\n", count($C), $round + 1, count($edits), $n, $overlaps, $lookups, $added);
foreach (array_slice($block, 0, 30, true) as $why => $sites) { printf("%5d  %s   e.g. %s\n", count($sites), substr($why, 0, 150), $sites[0]); }
