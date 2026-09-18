<?php
declare(strict_types=1);
/**
 * Runs the test classes under PHP the way the generated Rust harness does: construct with the
 * method name, runSetUp(), call the method with the data-set row, runTearDown(), then check the
 * expected-exception state. The vendored PHPUnit cannot do this itself here -- its run() and
 * runBare() are stubbed out, because the harness drives the fixtures.
 *
 * Usage: php run-tests-under-php.php 'Psalm\Tests\TaintTest' ['Psalm\Tests\ClassTest' ...]
 */
require __DIR__ . '/vendor/autoload.php';
\DG\BypassFinals::enable();
\DG\BypassFinals::denyPaths(['*tests/fixtures/DummyProject*']);
ini_set('memory_limit', '-1');
chdir(__DIR__);

spl_autoload_register(static function (string $class): void {
    if (str_starts_with($class, 'Psalm\\Example\\Plugin\\')) {
        $short = substr($class, strrpos($class, '\\') + 1);
        foreach (array_merge(glob(__DIR__ . '/examples/plugins/*.php') ?: [], glob(__DIR__ . '/examples/plugins/*/*/*.php') ?: []) ?: [] as $candidate) {
            if (basename($candidate, '.php') === $short) {
                require $candidate;
                return;
            }
        }
    }
    if (str_starts_with($class, 'Psalm\\Tests\\')) {
        $path = __DIR__ . '/tests/' . str_replace('\\', '/', substr($class, 12)) . '.php';
        if (file_exists($path)) {
            require $path;
        }
    }
});

/** @return list<array{string, list<mixed>}> */
function dataSetsFor(ReflectionMethod $m, string $class): array
{
    $doc = (string) $m->getDocComment();
    if (!preg_match('/@dataProvider\s+([A-Za-z0-9_:\\\\]+)/', $doc, $mm)) {
        return [['', []]];
    }
    $provider = $mm[1];
    $rc = new ReflectionClass($class);
    $pm = $rc->getMethod($provider);
    $rows = $pm->isStatic() ? $pm->invoke(null) : $pm->invoke($rc->newInstanceWithoutConstructor());
    $params = $m->getParameters();
    $names = [];
    foreach ($params as $i => $p) {
        $names[$p->getName()] = $i;
    }
    $out = [];
    foreach ($rows as $name => $row) {
        // a row key that names a parameter binds to it; any other key takes the next free position,
        // which is how the generated harness reads these rows
        $slots = [];
        $next = 0;
        foreach ($row as $key => $value) {
            if (is_string($key) && isset($names[$key])) {
                $slots[$names[$key]] = $value;
                continue;
            }
            while (array_key_exists($next, $slots)) {
                $next++;
            }
            $slots[$next] = $value;
            $next++;
        }
        $args = [];
        foreach ($params as $i => $p) {
            if (array_key_exists($i, $slots)) {
                $args[] = $slots[$i];
                continue;
            }
            if ($p->isDefaultValueAvailable()) {
                $args[] = $p->getDefaultValue();
                continue;
            }
            break;
        }
        $out[] = [(string) $name, $args];
    }
    return $out;
}

$classes = array_slice($argv, 1);
$total_pass = 0;
$total_fail = 0;
$failures = [];

foreach ($classes as $class) {
    if (!class_exists($class)) {
        echo "MISSING CLASS $class\n";
        continue;
    }
    $rc = new ReflectionClass($class);
    if ($rc->isAbstract()) {
        continue;
    }
    $class::setUpBeforeClass();
    foreach ($rc->getMethods(ReflectionMethod::IS_PUBLIC) as $m) {
        if (!str_starts_with($m->getName(), 'test') || $m->isStatic() || $m->getDeclaringClass()->isInterface()) {
            continue;
        }
        foreach (dataSetsFor($m, $class) as [$data_name, $args]) {
            if (str_contains($data_name, 'SKIPPED-') || str_contains($m->getName(), 'SKIPPED-')) {
                continue;
            }
            $label = $class . '::' . $m->getName() . ($data_name === '' ? '' : " [$data_name]");
            $test = new $class($m->getName());
            $test->setDataName($data_name);
            $failure = null;
            try {
                $test->runSetUp();
                $m->invokeArgs($test, $args);
                if ($test->expectsException()) {
                    $failure = 'expected exception ' . $test->expectedExceptionDescription() . ' was not thrown';
                }
                $test->runTearDown();
            } catch (PHPUnit\Framework\SkippedTestError | PHPUnit\Framework\SkippedTestSuiteError) {
                continue;
            } catch (Throwable $e) {
                if ($test->expectsException()) {
                    try {
                        $test->verifyExpectedException($e);
                    } catch (Throwable $v) {
                        $failure = get_class($v) . ': ' . $v->getMessage();
                    }
                } else {
                    $failure = get_class($e) . ': ' . $e->getMessage();
                    if (getenv('PSALM_TEST_TRACE')) {
                        $failure .= "\n" . $e->getTraceAsString();
                    }
                    for ($p = $e->getPrevious(); $p !== null; $p = $p->getPrevious()) {
                        $failure .= ' <- ' . get_class($p) . ': ' . $p->getMessage();
                    }
                }
                try {
                    $test->runTearDown();
                } catch (Throwable) {
                }
            }
            if ($failure === null) {
                $total_pass++;
            } else {
                $total_fail++;
                $failures[] = $label . ': ' . $failure;
            }
        }
    }
    $class::tearDownAfterClass();
}

foreach ($failures as $f) {
    echo "FAIL $f\n";
}
echo "PASS=$total_pass FAIL=$total_fail\n";
