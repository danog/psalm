<?php

declare(strict_types=1);

namespace Psalm;

use Psalm\Storage\UnserializeMemoryUsageSuppressionTrait;

/**
 * @api
 */
final class Aliases
{
    use UnserializeMemoryUsageSuppressionTrait;

    public ?int $namespace_first_stmt_start = null;

    public ?int $uses_start = null;

    public ?int $uses_end = null;

    /**
     * @param array<int, string> $uses
     * @param array<int, non-empty-string> $functions
     * @param array<string, string> $constants
     * @param array<int, string> $uses_flipped
     * @param array<int, string> $functions_flipped
     * @param array<string, string> $constants_flipped
     * @internal
     * @psalm-mutation-free
     */
    public function __construct(
        public ?string $namespace = null,
        public array $uses = [],
        public array $functions = [],
        public array $constants = [],
        public array $uses_flipped = [],
        public array $functions_flipped = [],
        public array $constants_flipped = [],
    ) {
    }
}
