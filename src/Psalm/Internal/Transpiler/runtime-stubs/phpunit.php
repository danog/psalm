<?php

declare(strict_types=1);

namespace PHPUnit\Framework;

use Throwable;

/**
 * Minimal PHPUnit implementation used by the transpiled test suites.
 * Assertion failures are raised as AssertionFailedError; the generated Rust harness
 * drives fixtures, data providers and expected-exception checks.
 */
class AssertionFailedError extends \Exception
{
}

final class ExpectationFailedException extends AssertionFailedError
{
}

final class SkippedTestError extends \Exception
{
}

final class IncompleteTestError extends \Exception
{
}

abstract class TestCase extends Assert
{
    private ?string $expectedException = null;

    private ?string $expectedExceptionMessage = null;

    private ?string $expectedExceptionMessageRegExp = null;

    private ?int $expectedExceptionCode = null;

    private string $name = '';

    public function __construct(string $name = '')
    {
        $this->name = $name;
    }

    public static function setUpBeforeClass(): void
    {
    }

    public static function tearDownAfterClass(): void
    {
    }

    protected function setUp(): void
    {
    }

    protected function tearDown(): void
    {
    }

    /** Called by the generated harness (fixtures are protected in PHPUnit). */
    public function runSetUp(): void
    {
        $this->setUp();
    }

    public function runTearDown(): void
    {
        $this->tearDown();
    }

    public function getName(): string
    {
        return $this->name;
    }

    public function name(): string
    {
        return $this->name;
    }

    /** @param class-string<Throwable> $exception */
    public function expectException(string $exception): void
    {
        $this->expectedException = $exception;
    }

    public function expectExceptionMessage(string $message): void
    {
        $this->expectedExceptionMessage = $message;
    }

    public function expectExceptionMessageMatches(string $regularExpression): void
    {
        $this->expectedExceptionMessageRegExp = $regularExpression;
    }

    public function expectExceptionCode(int $code): void
    {
        $this->expectedExceptionCode = $code;
    }

    public function expectNotToPerformAssertions(): void
    {
    }

    /** True when the test declared an expected exception. */
    public function expectsException(): bool
    {
        return $this->expectedException !== null || $this->expectedExceptionMessage !== null
            || $this->expectedExceptionMessageRegExp !== null || $this->expectedExceptionCode !== null;
    }

    /** Verifies a thrown exception against the expectations; throws AssertionFailedError on mismatch. */
    public function verifyExpectedException(Throwable $e): void
    {
        if ($this->expectedException !== null && !is_a($e, $this->expectedException)) {
            throw new AssertionFailedError(
                'Failed asserting that exception of type "' . get_class($e) . '" matches expected exception "'
                . $this->expectedException . '". Message was: "' . $e->getMessage() . '"',
            );
        }
        if ($this->expectedExceptionMessage !== null && !str_contains($e->getMessage(), $this->expectedExceptionMessage)) {
            throw new AssertionFailedError(
                'Failed asserting that exception message \'' . $e->getMessage() . '\' contains \''
                . $this->expectedExceptionMessage . '\'.',
            );
        }
        if ($this->expectedExceptionMessageRegExp !== null && !preg_match($this->expectedExceptionMessageRegExp, $e->getMessage())) {
            throw new AssertionFailedError(
                'Failed asserting that exception message \'' . $e->getMessage() . '\' matches \''
                . $this->expectedExceptionMessageRegExp . '\'.',
            );
        }
        if ($this->expectedExceptionCode !== null && $e->getCode() !== $this->expectedExceptionCode) {
            throw new AssertionFailedError('Failed asserting that exception code ' . $e->getCode() . ' is ' . $this->expectedExceptionCode . '.');
        }
    }

    public function expectedExceptionDescription(): string
    {
        return (string) ($this->expectedException ?? $this->expectedExceptionMessage ?? $this->expectedExceptionMessageRegExp ?? '');
    }

    public function markTestSkipped(string $message = ''): never
    {
        throw new SkippedTestError($message);
    }

    public function markTestIncomplete(string $message = ''): never
    {
        throw new IncompleteTestError($message);
    }

    /** @return mixed */
    protected function createMock(string $originalClassName): mixed
    {
        throw new SkippedTestError('mocks are not supported in the Rust port: ' . $originalClassName);
    }

    /** @return mixed */
    protected function createStub(string $originalClassName): mixed
    {
        throw new SkippedTestError('stubs are not supported in the Rust port: ' . $originalClassName);
    }
}

abstract class Assert
{
    /** Evaluates a constraint (the subset of PHPUnit's constraint API used by the ported tests). */
    public static function assertThat(mixed $value, \PHPUnit\Framework\Constraint\Constraint $constraint, string $message = ''): void
    {
        self::$count++;
        $constraint->evaluate($value, $message);
    }

    public static function stringContains(string $needle, bool $ignoreCase = false): \PHPUnit\Framework\Constraint\StringContains
    {
        return new \PHPUnit\Framework\Constraint\StringContains($needle, $ignoreCase);
    }

    private static int $count = 0;

    public static function getCount(): int
    {
        return self::$count;
    }

    public static function resetCount(): void
    {
        self::$count = 0;
    }

    private static function describe(mixed $value): string
    {
        if (is_string($value)) {
            return "'" . $value . "'";
        }
        if (is_object($value)) {
            return get_class($value) . ' Object ' . var_export(get_object_vars($value), true);
        }
        return var_export($value, true);
    }

    public static function fail(string $message = ''): never
    {
        throw new AssertionFailedError($message);
    }

    public static function assertTrue(mixed $condition, string $message = ''): void
    {
        self::$count++;
        if ($condition !== true) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($condition) . ' is true.');
        }
    }

    public static function assertFalse(mixed $condition, string $message = ''): void
    {
        self::$count++;
        if ($condition !== false) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($condition) . ' is false.');
        }
    }

    public static function assertNull(mixed $actual, string $message = ''): void
    {
        self::$count++;
        if ($actual !== null) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($actual) . ' is null.');
        }
    }

    public static function assertNotNull(mixed $actual, string $message = ''): void
    {
        self::$count++;
        if ($actual === null) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that null is not null.');
        }
    }

    public static function assertSame(mixed $expected, mixed $actual, string $message = ''): void
    {
        self::$count++;
        if ($expected !== $actual) {
            throw new AssertionFailedError(
                ($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($actual) . ' is identical to ' . self::describe($expected) . '.',
            );
        }
    }

    public static function assertNotSame(mixed $expected, mixed $actual, string $message = ''): void
    {
        self::$count++;
        if ($expected === $actual) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that two values are not identical.');
        }
    }

    public static function assertEquals(mixed $expected, mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (!self::looselyEqual($expected, $actual)) {
            throw new AssertionFailedError(
                ($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($actual) . ' equals ' . self::describe($expected) . '.',
            );
        }
    }

    public static function assertNotEquals(mixed $expected, mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (self::looselyEqual($expected, $actual)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that two values are not equal.');
        }
    }

    public static function assertEqualsCanonicalizing(mixed $expected, mixed $actual, string $message = ''): void
    {
        if (is_array($expected) && is_array($actual)) {
            sort($expected);
            sort($actual);
        }
        self::assertEquals($expected, $actual, $message);
    }

    /** PHPUnit's comparator: arrays element-wise, objects by class and properties, scalars loosely. */
    private static function looselyEqual(mixed $expected, mixed $actual): bool
    {
        if (is_array($expected) && is_array($actual)) {
            if (count($expected) !== count($actual)) {
                return false;
            }
            foreach ($expected as $k => $v) {
                if (!array_key_exists($k, $actual) || !self::looselyEqual($v, $actual[$k])) {
                    return false;
                }
            }
            return true;
        }
        if (is_object($expected) && is_object($actual)) {
            if (get_class($expected) !== get_class($actual)) {
                return false;
            }
            return self::looselyEqual(get_object_vars($expected), get_object_vars($actual));
        }
        if (is_float($expected) || is_float($actual)) {
            if (!is_numeric($expected) || !is_numeric($actual)) {
                return $expected == $actual;
            }
            return abs((float) $expected - (float) $actual) < 0.0000000001;
        }
        if ($expected === null || $actual === null || is_bool($expected) || is_bool($actual)) {
            return $expected == $actual;
        }
        if (is_string($expected) && is_string($actual)) {
            return $expected === $actual;
        }
        return $expected == $actual;
    }

    public static function assertCount(int $expectedCount, mixed $haystack, string $message = ''): void
    {
        self::$count++;
        $count = is_array($haystack) ? count($haystack) : (is_countable($haystack) ? count($haystack) : -1);
        if ($count !== $expectedCount) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that actual size ' . $count . ' matches expected size ' . $expectedCount . '.');
        }
    }

    public static function assertEmpty(mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (!empty($actual)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($actual) . ' is empty.');
        }
    }

    public static function assertNotEmpty(mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (empty($actual)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that a value is not empty.');
        }
    }

    public static function assertInstanceOf(string $expected, mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (!is_object($actual) || !is_a($actual, $expected)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($actual) . ' is an instance of class "' . $expected . '".');
        }
    }

    public static function assertNotInstanceOf(string $expected, mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (is_object($actual) && is_a($actual, $expected)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that an object is not an instance of class "' . $expected . '".');
        }
    }

    public static function assertIsArray(mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (!is_array($actual)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($actual) . ' is of type array.');
        }
    }

    public static function assertIsString(mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (!is_string($actual)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($actual) . ' is of type string.');
        }
    }

    public static function assertIsInt(mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (!is_int($actual)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($actual) . ' is of type int.');
        }
    }

    public static function assertIsBool(mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (!is_bool($actual)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($actual) . ' is of type bool.');
        }
    }

    public static function assertIsObject(mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (!is_object($actual)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($actual) . ' is of type object.');
        }
    }

    public static function assertIsCallable(mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (!is_callable($actual)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that a value is callable.');
        }
    }

    public static function assertStringContainsString(string $needle, string $haystack, string $message = ''): void
    {
        self::$count++;
        if (!str_contains($haystack, $needle)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . "Failed asserting that '" . $haystack . "' contains \"" . $needle . '".');
        }
    }

    public static function assertStringNotContainsString(string $needle, string $haystack, string $message = ''): void
    {
        self::$count++;
        if (str_contains($haystack, $needle)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . "Failed asserting that '" . $haystack . "' does not contain \"" . $needle . '".');
        }
    }

    public static function assertStringStartsWith(string $prefix, string $string, string $message = ''): void
    {
        self::$count++;
        if (!str_starts_with($string, $prefix)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . "Failed asserting that '" . $string . "' starts with \"" . $prefix . '".');
        }
    }

    public static function assertStringEndsWith(string $suffix, string $string, string $message = ''): void
    {
        self::$count++;
        if (!str_ends_with($string, $suffix)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . "Failed asserting that '" . $string . "' ends with \"" . $suffix . '".');
        }
    }

    public static function assertMatchesRegularExpression(string $pattern, string $string, string $message = ''): void
    {
        self::$count++;
        if (!preg_match($pattern, $string)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . "Failed asserting that '" . $string . "' matches PCRE pattern \"" . $pattern . '".');
        }
    }

    public static function assertDoesNotMatchRegularExpression(string $pattern, string $string, string $message = ''): void
    {
        self::$count++;
        if (preg_match($pattern, $string)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . "Failed asserting that '" . $string . "' does not match PCRE pattern \"" . $pattern . '".');
        }
    }

    public static function assertContains(mixed $needle, iterable $haystack, string $message = ''): void
    {
        self::$count++;
        foreach ($haystack as $item) {
            if ($item === $needle) {
                return;
            }
        }
        throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that an iterable contains ' . self::describe($needle) . '.');
    }

    public static function assertNotContains(mixed $needle, iterable $haystack, string $message = ''): void
    {
        self::$count++;
        foreach ($haystack as $item) {
            if ($item === $needle) {
                throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that an iterable does not contain ' . self::describe($needle) . '.');
            }
        }
    }

    public static function assertArrayHasKey(int|string $key, array $array, string $message = ''): void
    {
        self::$count++;
        if (!array_key_exists($key, $array)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that an array has the key ' . self::describe($key) . '.');
        }
    }

    public static function assertArrayNotHasKey(int|string $key, array $array, string $message = ''): void
    {
        self::$count++;
        if (array_key_exists($key, $array)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that an array does not have the key ' . self::describe($key) . '.');
        }
    }

    public static function assertGreaterThan(mixed $expected, mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (!($actual > $expected)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($actual) . ' is greater than ' . self::describe($expected) . '.');
        }
    }

    public static function assertGreaterThanOrEqual(mixed $expected, mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (!($actual >= $expected)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($actual) . ' is greater than or equal to ' . self::describe($expected) . '.');
        }
    }

    public static function assertLessThan(mixed $expected, mixed $actual, string $message = ''): void
    {
        self::$count++;
        if (!($actual < $expected)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that ' . self::describe($actual) . ' is less than ' . self::describe($expected) . '.');
        }
    }

    public static function assertFileExists(string $filename, string $message = ''): void
    {
        self::$count++;
        if (!file_exists($filename)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that file "' . $filename . '" exists.');
        }
    }

    public static function assertFileDoesNotExist(string $filename, string $message = ''): void
    {
        self::$count++;
        if (file_exists($filename)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that file "' . $filename . '" does not exist.');
        }
    }

    public static function assertDirectoryExists(string $directory, string $message = ''): void
    {
        self::$count++;
        if (!is_dir($directory)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that directory "' . $directory . '" exists.');
        }
    }

    public static function assertStringEqualsFile(string $expectedFile, string $actualString, string $message = ''): void
    {
        self::assertSame((string) file_get_contents($expectedFile), $actualString, $message);
    }

    public static function assertJsonStringEqualsJsonString(string $expectedJson, string $actualJson, string $message = ''): void
    {
        self::assertEquals(json_decode($expectedJson, true), json_decode($actualJson, true), $message);
    }

    public static function assertObjectHasProperty(string $propertyName, object $object, string $message = ''): void
    {
        self::$count++;
        if (!property_exists($object, $propertyName)) {
            throw new AssertionFailedError(($message !== '' ? $message . "\n" : '') . 'Failed asserting that object has property "' . $propertyName . '".');
        }
    }
}

namespace PHPUnit\Framework\Constraint;

abstract class Constraint implements \Countable
{
    /**
     * @return bool|null true/false when $returnResult, otherwise throws on mismatch
     */
    public function evaluate(mixed $other, string $description = '', bool $returnResult = false): ?bool
    {
        $success = $this->matches($other);
        if ($returnResult) {
            return $success;
        }
        if (!$success) {
            $this->fail($other, $description);
        }
        return null;
    }

    protected function matches(mixed $other): bool
    {
        return false;
    }

    abstract public function toString(): string;

    protected function failureDescription(mixed $other): string
    {
        return \PHPUnit\Framework\Assert::describeValue($other) . ' ' . $this->toString();
    }

    protected function fail(mixed $other, string $description): never
    {
        $failureDescription = 'Failed asserting that ' . $this->failureDescription($other) . '.';
        if ($description !== '') {
            $failureDescription = $description . "\n" . $failureDescription;
        }
        throw new \PHPUnit\Framework\ExpectationFailedException($failureDescription);
    }

    public function count(): int
    {
        return 1;
    }

    protected function exporter(): \SebastianBergmann\Exporter\Exporter
    {
        return new \SebastianBergmann\Exporter\Exporter();
    }
}

final class StringContains extends Constraint
{
    public function __construct(private string $needle, private bool $ignoreCase = false)
    {
    }

    protected function matches(mixed $other): bool
    {
        $haystack = (string) $other;
        if ($this->ignoreCase) {
            return stripos($haystack, $this->needle) !== false;
        }
        return strpos($haystack, $this->needle) !== false;
    }

    public function toString(): string
    {
        return 'contains "' . $this->needle . '"';
    }
}

namespace SebastianBergmann\Exporter;

/** The parts of the exporter that PHPUnit constraints use for failure messages. */
final class Exporter
{
    public function export(mixed $value, int $indentation = 0): string
    {
        return var_export($value, true);
    }

    public function shortenedExport(mixed $value): string
    {
        if (is_string($value)) {
            $value = str_replace("\n", '', $value);
            if (strlen($value) > 40) {
                $value = substr($value, 0, 30) . '...' . substr($value, -7);
            }
            return "'" . $value . "'";
        }
        if (is_array($value)) {
            return count($value) === 0 ? '[]' : '[...]';
        }
        if (is_object($value)) {
            return get_class($value) . ' Object (...)';
        }
        return var_export($value, true);
    }
}
