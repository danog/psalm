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
}
