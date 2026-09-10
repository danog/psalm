<?php

declare(strict_types=1);

/**
 * Runs the PHPUnit suite in parallel shards, by default inside the native binary:
 *
 *   php typephp/run-tests-parallel.php --binary /path/to/psalm-native [--shards 12] [--out /tmp/psalm-tests] [tests/...]
 *   php typephp/run-tests-parallel.php --command "php -d memory_limit=-1 vendor/bin/phpunit" ...   (plain PHP)
 *
 * Every shard gets its own PHPUnit configuration listing its test files and a
 * JUnit log; the logs are summarised at the end (failing tests in <out>/failures.txt).
 */

$root = dirname(__DIR__);
$shards = (int) (getenv('NPROC') ?: trim((string) shell_exec('sysctl -n hw.ncpu 2>/dev/null || nproc')));
$out = '/tmp/psalm-tests';
$command = null;
$paths = [];
for ($i = 1; $i < $argc; $i++) {
    switch ($argv[$i]) {
        case '--binary':
            $command = escapeshellarg($argv[++$i]) . ' --typephp-run ' . escapeshellarg($root . '/typephp/run-tests.php');
            break;
        case '--command':
            $command = $argv[++$i];
            break;
        case '--shards':
            $shards = (int) $argv[++$i];
            break;
        case '--out':
            $out = $argv[++$i];
            break;
        default:
            $paths[] = $argv[$i];
    }
}
if ($command === null) {
    fwrite(STDERR, "--binary or --command is required\n");
    exit(2);
}
if ($paths === []) {
    $paths = [$root . '/tests'];
}

$files = [];
foreach ($paths as $path) {
    $path = str_starts_with($path, '/') ? $path : $root . '/' . $path;
    if (is_file($path)) {
        $files[] = $path;
        continue;
    }
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path, FilesystemIterator::SKIP_DOTS));
    foreach ($it as $file) {
        if (str_ends_with($file->getFilename(), 'Test.php')) {
            $files[] = $file->getPathname();
        }
    }
}
sort($files);
// balance by file size (rough proxy for the number of tests), largest first
usort($files, static fn(string $a, string $b): int => filesize($b) <=> filesize($a));
$buckets = array_fill(0, $shards, []);
$load = array_fill(0, $shards, 0);
foreach ($files as $file) {
    $target = array_search(min($load), $load, true);
    $buckets[$target][] = $file;
    $load[$target] += filesize($file);
}

if (!is_dir($out)) {
    mkdir($out, 0777, true);
}
$procs = [];
foreach ($buckets as $n => $bucket) {
    if ($bucket === []) {
        continue;
    }
    $xml = '<?xml version="1.0" encoding="UTF-8"?>' . "\n"
        . '<phpunit bootstrap="' . $root . '/tests/autoload.php" backupGlobals="false" beStrictAboutOutputDuringTests="true"'
        . ' beStrictAboutTodoAnnotatedTests="true" colors="false" executionOrder="random">' . "\n"
        . '  <testsuites><testsuite name="shard-' . $n . '">' . "\n";
    foreach ($bucket as $file) {
        $xml .= '    <file>' . htmlspecialchars($file) . '</file>' . "\n";
    }
    $xml .= '  </testsuite></testsuites>' . "\n"
        . '  <php><const name="__IS_TEST_ENV__" value="1" /></php>' . "\n"
        . '</phpunit>' . "\n";
    file_put_contents("$out/shard-$n.xml", $xml);
    $cmd = $command . ' -c ' . escapeshellarg("$out/shard-$n.xml") . ' --log-junit ' . escapeshellarg("$out/shard-$n-junit.xml")
        . ' > ' . escapeshellarg("$out/shard-$n.log") . ' 2>&1';
    $procs[$n] = proc_open($cmd, [], $pipes, $root);
}
echo count($procs), " shards, ", count($files), " test files\n";
$start = microtime(true);
$exit = [];
foreach ($procs as $n => $proc) {
    $exit[$n] = proc_close($proc);
}
printf("finished in %.0fs\n", microtime(true) - $start);

$totals = ['tests' => 0, 'assertions' => 0, 'errors' => 0, 'failures' => 0, 'skipped' => 0];
$failures = [];
foreach (array_keys($procs) as $n) {
    $junit = "$out/shard-$n-junit.xml";
    if (!is_file($junit) || filesize($junit) === 0) {
        $failures[] = "shard $n: no JUnit log (exit code {$exit[$n]}), see $out/shard-$n.log";
        continue;
    }
    $doc = new DOMDocument();
    $doc->load($junit);
    $suite = $doc->documentElement->firstElementChild;
    foreach (array_keys($totals) as $key) {
        $totals[$key] += (int) $suite->getAttribute($key);
    }
    foreach ($doc->getElementsByTagName('testcase') as $case) {
        foreach (['error', 'failure'] as $kind) {
            foreach ($case->getElementsByTagName($kind) as $problem) {
                $failures[] = $kind . ': ' . $case->getAttribute('class') . '::' . $case->getAttribute('name') . "\n    "
                    . trim(strtok($problem->textContent, "\n"));
            }
        }
    }
}
sort($failures);
file_put_contents("$out/failures.txt", implode("\n", $failures) . "\n");
printf(
    "Tests: %d, Assertions: %d, Errors: %d, Failures: %d, Skipped: %d\n",
    $totals['tests'],
    $totals['assertions'],
    $totals['errors'],
    $totals['failures'],
    $totals['skipped'],
);
echo count($failures), " problems listed in $out/failures.txt\n";
exit($totals['errors'] + $totals['failures'] > 0 ? 1 : 0);
