<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use Psalm\CodeLocation;
use Psalm\Storage\Mutations;

/**
 * The mutations performed by an analysed function-like.
 *
 * Was the `MutationInfo` array-shape (`@psalm-type` in MutationLevelResolver); a nominal class avoids the
 * fragile array-shape-union narrowing in the Rust transpiler (partial `['fresh'] =` writes + cache round-trips
 * produced divergent shapes that unioned and then failed a runtime downcast).
 *
 * @internal
 */
final class MutationInfo
{
    /**
     * @param Mutations::LEVEL_*  $intrinsic
     * @param Mutations::LEVEL_*  $allowed
     * @param array<string, bool> $callees
     * @param array<int, string>  $suppressed_issues
     */
    public function __construct(
        public int $intrinsic,
        public int $allowed,
        public array $callees,
        public CodeLocation $location,
        public string $cased_name,
        public array $suppressed_issues,
        public ?string $class,
        public int $start,
        public bool $fresh,
        public bool $report,
    ) {
    }
}
