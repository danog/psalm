<?php

declare(strict_types=1);

namespace Psalm\Tests;

use Psalm\Context;

/**
 * Narrow probes for inference a compiled build gets wrong: each one names the single thing it
 * measures, and the whole set is reported at once so one run tells the whole story.
 */
final class CompiledProbeTest extends TestCase
{
    public function testEmptyArrayAndTemplateInference(): void
    {
        $file_path = self::$src_dir_path . 'somefile.php';

        $this->addFile(
            $file_path,
            '<?php
                /** @return array<string> */
                function getStrings(): array { return []; }

                /**
                 * @template TKey as array-key
                 * @template TValue
                 */
                final class Coll {
                    /** @param array<TKey, TValue> $items */
                    public function __construct(public array $items) {}
                }

                $empty = [];
                $strings = getStrings();
                $coll = new Coll([]);
                $filled = new Coll(["a" => 1]);
                $sorted = [];
                sort($sorted);
                $combined = array_combine(getStrings(), getStrings());',
        );

        $context = new Context();
        $this->analyzeFile($file_path, $context);

        $actual = [];

        foreach (['$empty', '$strings', '$coll', '$filled', '$sorted', '$combined'] as $var_id) {
            $actual[$var_id] = isset($context->vars_in_scope[$var_id])
                ? (string) $context->vars_in_scope[$var_id]
                : 'absent';
        }

        $this->assertSame(
            [
                '$empty' => 'array<never, never>',
                '$strings' => 'array<array-key, string>',
                '$coll' => 'Coll<never, never>',
                '$filled' => 'Coll<string, int>',
                '$sorted' => 'array<never, never>',
                '$combined' => 'array<string, string>|false',
            ],
            $actual,
        );
    }
    public function testIntersectionAndStubbedClasses(): void
    {
        $file_path = self::$src_dir_path . 'somefile2.php';

        $this->addFile(
            $file_path,
            '<?php
                interface IA {}
                interface IB { public function foo(): void; }

                function make(): IA { throw new RuntimeException("x"); }

                $a = make();
                $narrowed = null;

                if ($a instanceof IB) {
                    $narrowed = $a;
                }

                $iso = DateTime::ISO8601;
                $countable = new ArrayObject([1, 2]);',
        );

        $context = new Context();
        $this->analyzeFile($file_path, $context);

        $actual = [];

        foreach (['$narrowed', '$iso', '$countable'] as $var_id) {
            $actual[$var_id] = isset($context->vars_in_scope[$var_id])
                ? (string) $context->vars_in_scope[$var_id]
                : 'absent';
        }

        $this->assertSame(
            [
                '$narrowed' => 'IA&IB|null',
                '$iso' => 'string',
                '$countable' => 'ArrayObject<int<0, 1>, int>',
            ],
            $actual,
        );
    }
    /**
     * A variable the try block definitely assigns is defined afterwards: the analyzer clears the
     * `possibly undefined from try` flag once it knows every catch leaves or the body completed.
     */
    public function testTryBlockAssignmentsAreDefinedAfterwards(): void
    {
        $file_path = self::$src_dir_path . 'somefile3.php';

        $this->addFile(
            $file_path,
            '<?php
                try {
                    $withFinally = "a";
                } finally {
                }

                try {
                    $withLeavingCatch = "b";
                } catch (Exception $e) {
                    throw new RuntimeException("x");
                }

                try {
                    $plain = "c";
                } catch (Exception $e) {
                }',
        );

        $context = new Context();
        $this->analyzeFile($file_path, $context);

        $actual = [];

        foreach (['$withFinally', '$withLeavingCatch', '$plain'] as $var_id) {
            $type = $context->vars_in_scope[$var_id] ?? null;
            $actual[$var_id] = $type === null
                ? 'absent'
                : ($type->possibly_undefined ? 'maybe' : 'defined')
                    . '/' . ($type->possibly_undefined_from_try ? 'from-try' : 'not-from-try');
        }

        $this->assertSame(
            [
                '$withFinally' => 'defined/not-from-try',
                '$withLeavingCatch' => 'defined/not-from-try',
                '$plain' => 'maybe/from-try',
            ],
            $actual,
        );
    }
}
