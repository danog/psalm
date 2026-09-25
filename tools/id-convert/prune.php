<?php

/**
 * Imports Interner / Sym wherever the fix rounds started using them, and drops the Interner helpers the converted tree never calls (the fixer's wrap helpers that no site ended up
 * needing), repeatedly, since a dropped helper can be the only caller of another.
 *
 * php prune.php ROOT
 */

declare(strict_types=1);

$root = rtrim($argv[1], '/') . '/';
$file = $root . 'src/Psalm/Internal/Interner.php';
$others = '';
foreach (['src', 'tests', 'examples', 'bin'] as $dir) {
    if (!is_dir($root . $dir)) {
        continue;
    }
    foreach (new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root . $dir, FilesystemIterator::SKIP_DOTS)) as $f) {
        if (str_ends_with((string) $f, '.php') && realpath((string) $f) !== realpath($file)) {
            $others .= file_get_contents((string) $f);
        }
    }
}
$imported = 0;
foreach (['src', 'tests', 'examples'] as $dir) {
    if (!is_dir($root . $dir)) {
        continue;
    }
    foreach (new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root . $dir, FilesystemIterator::SKIP_DOTS)) as $f) {
        $path = (string) $f;
        if (!str_ends_with($path, '.php')) {
            continue;
        }
        $out = (string) file_get_contents($path);
        $before = $out;
        $ns = preg_match('/^namespace\s+([^;{\s]+)/m', $out, $m) ? $m[1] : '';
        foreach (['Interner', 'Sym'] as $cls) {
            if (!preg_match('/(?<![\w\\\\$>])' . $cls . '::/', $out) || $ns === 'Psalm\\Internal'
                || preg_match('/^use\s+Psalm\\\\Internal\\\\' . $cls . ';/m', $out)
            ) {
                continue;
            }
            $line = "use Psalm\\Internal\\$cls;\n";
            if (preg_match('/^namespace\s+[^;]+;\n/m', $out, $nm, PREG_OFFSET_CAPTURE)) {
                $at = $nm[0][1] + strlen($nm[0][0]);
                $out = substr($out, 0, $at) . "\n" . $line . substr($out, $at);
            } else {
                $out = (string) preg_replace('/(?<![\w\\\\$>])' . $cls . '::/', '\\\\Psalm\\\\Internal\\\\' . $cls . '::', $out);
            }
        }
        if ($out !== $before) {
            file_put_contents($path, $out);
            $imported++;
        }
    }
}
echo "imported Interner/Sym in $imported files\n";
$removed = [];
do {
    $src = (string) file_get_contents($file);
    $changed = false;
    // public/private static helpers with their docblock
    preg_match_all('/\n(    \/\*\*(?:(?!\*\/).)*?\*\/\n)?    (?:public|private) static function (\w+)\(.*?\n    \}\n/s', $src, $ms, PREG_SET_ORDER | PREG_OFFSET_CAPTURE);
    foreach ($ms as $m) {
        $name = $m[2][0];
        if (!preg_match('/^(intern|lookup)\w*(Each|At|Keys|List)$|^mapAt$/', $name)) {
            continue;
        }
        $rest = substr($src, 0, $m[0][1]) . substr($src, $m[0][1] + strlen($m[0][0]));
        $call = '/(?:Interner|self|static)::' . $name . '\b/';
        if (!preg_match($call, $others) && !preg_match($call, $rest)) {
            file_put_contents($file, $rest);
            $removed[] = $name;
            $changed = true;
            break;
        }
    }
} while ($changed);
echo 'pruned ' . count($removed) . ' Interner helpers: ' . implode(', ', $removed) . "\n";
