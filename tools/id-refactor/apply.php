<?php
// apply.php <jsonl> [--dry]: applies the plugin's edits (each call site's edits atomically; overlapping or duplicate
// sites once), adds missing Sym constants, and prints a summary of the skipped sites by argument shape.
declare(strict_types=1);
$in = $argv[1] ?? exit("usage: apply.php out.jsonl [--dry]\n");
$dry = in_array('--dry', $argv, true);
$sites = []; $syms = []; $skips = []; $manual = []; $errors = 0; $wrapped = 0;
foreach (file($in, FILE_IGNORE_NEW_LINES) as $line) {
    $r = json_decode($line, true);
    if ($r['kind'] === 'edit') { $k = $r['site'] . json_encode($r['edits']); if (!isset($sites[$r['file']][$k])) { $wrapped += $r['wrapped'] ?? 0; } $sites[$r['file']][$k] = $r['edits']; }
    elseif ($r['kind'] === 'sym') { $syms[$r['name']] = $r; }
    elseif ($r['kind'] === 'skip') { $skips[$r['site']] = $r['args']; }
    elseif ($r['kind'] === 'manual') { $manual[$r['site']] = $r['why']; }
    else { $errors++; }
}
$n = 0;
foreach ($sites as $file => $group) {
    $edits = [];
    // one record per call site (Psalm analyzes loop bodies more than once); nested sites keep all their edits
    foreach ($group as $rk => $site_edits) { foreach ($site_edits as $i => $e) { $edits[$rk . '#' . $i] = $e; } }
    usort($edits, fn($a, $b) => [$b[0], $b[1]] <=> [$a[0], $a[1]]);
    $src = file_get_contents($file); $prev = PHP_INT_MAX;
    foreach ($edits as [$s, $e, $t]) {
        if ($e > $prev) { fwrite(STDERR, "overlap in $file at $s\n"); continue; }
        $src = substr($src, 0, $s) . $t . substr($src, $e); $prev = $s; $n++;
    }
    // the Sym / Interner imports, in alphabetical position among the use statements
    foreach (['Sym', 'Interner'] as $imp) {
    if (str_contains($src, $imp . '::') && !preg_match('/^use Psalm\\\\Internal\\\\' . $imp . ';$/m', $src)
        && !preg_match('/^namespace Psalm\\\\Internal;$/m', $src)
    ) {
        $line = "use Psalm\\Internal\\" . $imp . ";";
        preg_match_all('/^use [A-Z][^;(]*;$/m', $src, $m, PREG_OFFSET_CAPTURE);
        $at = null;
        foreach ($m[0] as [$u, $off]) {
            if (strcmp($u, $line) > 0 && !str_starts_with($u, 'use function') && !str_starts_with($u, 'use const')) { $at = $off; break; }
        }
        if ($at === null) {
            $last = null;
            foreach ($m[0] as [$u, $off]) { if (!str_starts_with($u, 'use function') && !str_starts_with($u, 'use const')) { $last = $off + strlen($u) + 1; } }
            $at = $last ?? (strpos($src, "\n", strpos($src, 'namespace ')) + 1);
        }
        $src = substr($src, 0, $at) . $line . "\n" . substr($src, $at);
    }
    }
    if (!$dry) { file_put_contents($file, $src); }
}
// new Sym constants go to bin/generate-sym.php's name list (with their literal) and Sym.php is regenerated,
// so they are preloaded names too
$gen = __DIR__ . '/../../bin/generate-sym.php';
$gen_src = file_get_contents($gen); $added = 0;
$names_end = strpos($gen_src, "\n];", strpos($gen_src, '$names = ['));
$insert = '';
foreach ($syms as $name => $row) {
    if (!is_array($row) || !isset($row['literal'])) { continue; }
    if (preg_match("/'" . preg_quote($name, '/') . "' =>/", $gen_src)) { continue; }
    $insert .= "\n    '" . $name . "' => " . var_export($row['literal'], true) . ',';
    $added++;
}
if (!$dry && $added) {
    $gen_src = substr($gen_src, 0, $names_end) . "\n    // added by tools/id-refactor" . $insert . substr($gen_src, $names_end);
    file_put_contents($gen, $gen_src);
    passthru('php ' . escapeshellarg($gen));
}
$shapes = [];
foreach ($skips as $site => $args) { foreach ($args as $a) { $shapes[($a['node'] ?? $a['why']) . ' : ' . ($a['type'] ?? '')][] = $site; } }
uasort($shapes, fn($a, $b) => count($b) <=> count($a));
printf("manual sites %d, intern() wraps %d\n", count($manual), $wrapped); foreach ($manual as $ms => $mw) { echo "  MANUAL $ms: $mw\n"; }
printf("files %d, edits %d, sym constants added %d, skipped sites %d, plugin errors %d\n", count($sites), $n, $added, count($skips), $errors);
foreach (array_slice($shapes, 0, 25, true) as $shape => $list) { printf("%5d  %s   e.g. %s\n", count($list), $shape, $list[0]); }
