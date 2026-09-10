<?php

declare(strict_types=1);

/**
 * Runs PHPUnit inside the native binary (interpreted PHPUnit and tests, compiled Psalm):
 *
 *   psalm-native --typephp-run typephp/run-tests.php [phpunit options]
 *
 * The binary must be built from a project file generated with
 * `gen-vendor-build.php --open-world`, so that test classes can extend or
 * mock the compiled classes.
 */

ini_set('memory_limit', '-1');

$argv = $_SERVER['argv'];
// psalm-native --typephp-run typephp/run-tests.php <options> => phpunit <options>
array_splice($argv, 0, 3, ['phpunit']);
$_SERVER['argv'] = $argv;
$_SERVER['argc'] = count($argv);

// Composer "files" autoload entries whose functions are compiled into the
// binary must not be loaded again.
$vendor = dirname(__DIR__) . '/vendor';
$files = require $vendor . '/composer/autoload_files.php';
foreach ($files as $identifier => $file) {
    $code = file_get_contents($file);
    $namespace = preg_match('/^namespace\s+([^;\s]+)\s*;/m', $code, $m) ? $m[1] . '\\' : '';
    if (preg_match_all('/^\s*function\s+(\w+)\s*\(/m', $code, $m) === 0) {
        continue;
    }
    foreach ($m[1] as $function) {
        if (function_exists($namespace . $function)) {
            $GLOBALS['__composer_autoload_files'][$identifier] = true;
            break;
        }
    }
}

require $vendor . '/phpunit/phpunit/phpunit';
