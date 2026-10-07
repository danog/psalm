<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use Psalm\CodeLocation;

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
     * @param int                 $intrinsic         the capabilities the body itself requires (a Capabilities bitmask)
     * @param int                 $allowed           the capabilities the function-like is annotated with
     * @param array<string, bool> $callees
     * @param int                 $default_intrinsic the capabilities its parameter defaults require
     * @param array<string, bool> $default_callees   the unannotated function-likes its parameter defaults call
     * @param array<array-key, string>  $suppressed_issues
     */
    public function __construct(
        public int $intrinsic,
        public int $allowed,
        public array $callees,
        public int $default_intrinsic,
        public array $default_callees,
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
