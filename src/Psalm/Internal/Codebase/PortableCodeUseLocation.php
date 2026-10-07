<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use Psalm\CodeLocation;

/**
 * The location of a reference in a PortableCodeUseGraph.
 *
 * @internal
 */
final class PortableCodeUseLocation
{
    /**
     * @psalm-mutation-free
     */
    public function __construct(
        public readonly string $source_node,
        public readonly string $target_node,
        public readonly string $location_hash,
        public readonly CodeLocation $location,
    ) {
    }
}
