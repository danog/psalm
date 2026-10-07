<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

/**
 * A CodeUseGraph in a form that can cross a process boundary (see CodeUseGraph::toPortable()): nodes by their
 * string form, as interned ids are only valid in the process that interned them.
 *
 * @internal
 */
final class PortableCodeUseGraph
{
    /**
     * @param array<string, array<string, string>> $edges source node => target node => edge type name
     * @param array<string, string> $node_files node => file path
     * @param array<string, MutationInfo> $mutation_info
     * @param list<PortableCodeUseLocation> $locations
     * @psalm-mutation-free
     */
    public function __construct(
        public readonly array $edges,
        public readonly array $node_files,
        public readonly array $mutation_info,
        public readonly array $locations,
    ) {
    }
}
