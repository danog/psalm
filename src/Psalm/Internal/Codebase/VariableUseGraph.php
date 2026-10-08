<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use Override;
use Psalm\CodeLocation;
use Psalm\Internal\DataFlow\DataFlowNode;
use Psalm\Internal\DataFlow\Path;

use function abs;
use function array_sum;
use function count;

/**
 * @internal
 */
final class VariableUseGraph extends DataFlowGraph
{
    /**
     * The edges, by the keys (see DataFlowNode::$key) of their origins and destinations
     *
     * @var array<int, array<int, Path>>
     */
    private array $forward_edges = [];

    /** @var array<int, array<int, true>> */
    private array $backward_edges = [];

    /** @var array<int, DataFlowNode> */
    private array $nodes = [];

    /** @var array<int, list<CodeLocation>> */
    private array $origin_locations_by_key = [];

    /**
     * @param ?TaintFlowGraph $taint_flow_graph The taint graph built alongside this one, if any: the
     *                                          speculative specializations of its nodes are removed
     *                                          here (see TaintFlowGraph::withoutSpeculativeSpecialization())
     * @psalm-mutation-free
     */
    public function __construct(
        private readonly ?TaintFlowGraph $taint_flow_graph = null,
    ) {
    }

    /**
     * @psalm-external-mutation-free
     */
    #[Override]
    public function addNode(DataFlowNode $node): void
    {
        $node = $this->taint_flow_graph?->withoutSpeculativeSpecialization($node) ?? $node;

        $this->nodes[$node->key] = $node;
    }

    /**
     * @psalm-external-mutation-free
     */
    #[Override]
    public function addPath(
        DataFlowNode $from,
        DataFlowNode $to,
        string $path_type,
        int $added_taints = 0,
        int $removed_taints = 0,
    ): void {
        if ($this->taint_flow_graph) {
            $from = $this->taint_flow_graph->withoutSpeculativeSpecialization($from);
            $to = $this->taint_flow_graph->withoutSpeculativeSpecialization($to);
        }

        $from_key = $from->key;
        $to_key = $to->key;

        if ($from_key === $to_key) {
            return;
        }

        $length = 0;

        if ($from->code_location
            && $to->code_location
            && $from->code_location->file_path === $to->code_location->file_path
        ) {
            $to_line = $to->code_location->raw_line_number;
            $from_line = $from->code_location->raw_line_number;
            $length = abs($to_line - $from_line);
        }

        $this->backward_edges[$to_key][$from_key] = true;
        $this->forward_edges[$from_key][$to_key] = new Path($path_type, $length);
    }

    /**
     * @psalm-capabilities read-props
     */
    public function isVariableUsed(DataFlowNode $assignment_node): bool
    {
        $visited_source_keys = [];

        $assignment_node = $this->taint_flow_graph?->withoutSpeculativeSpecialization($assignment_node)
            ?? $assignment_node;

        // the nodes reached, by key, with the path types of the flow reaching them
        $sources = [$assignment_node->key => $assignment_node->path_types];

        for ($i = 0; count($sources) && $i < 200; $i++) {
            $new_child_nodes = [];

            foreach ($sources as $source_key => $source_path_types) {
                $visited_source_keys[$source_key] = true;

                if ($this->getChildNodes(
                    $new_child_nodes,
                    $source_key,
                    $source_path_types,
                    $visited_source_keys,
                )) {
                    return true;
                }
            }

            $sources = $new_child_nodes;
        }

        return false;
    }

    /**
     * @return list<CodeLocation>
     * @psalm-capabilities read-props|write-this-props|write-refs
     */
    public function getOriginLocations(DataFlowNode $assignment_node): array
    {
        $assignment_node = $this->taint_flow_graph?->withoutSpeculativeSpecialization($assignment_node)
            ?? $assignment_node;

        if (isset($this->origin_locations_by_key[$assignment_node->key])) {
            return $this->origin_locations_by_key[$assignment_node->key];
        }

        $visited_child_keys = [];

        $origin_locations = [];

        $child_nodes = [$assignment_node];

        for ($i = 0; count($child_nodes) && $i < 200; $i++) {
            $new_parent_nodes = [];

            foreach ($child_nodes as $child_node) {
                $visited_child_keys[$child_node->key] = true;

                $had_parent_nodes = $this->getParentNodes(
                    $new_parent_nodes,
                    $child_node,
                    $visited_child_keys,
                );

                if (!$had_parent_nodes) {
                    if ($child_node->code_location) {
                        $origin_locations[] = $child_node->code_location;
                    }

                    continue;
                }
            }

            $child_nodes = $new_parent_nodes;
        }

        $this->origin_locations_by_key[$assignment_node->key] = $origin_locations;

        return $origin_locations;
    }

    /**
     * @param list<string> $source_path_types
     * @param array<int, bool> $visited_source_keys
     * @param array<int, list<string>> $child_nodes
     * @param-out array<int, list<string>> $child_nodes
     * @psalm-capabilities write-refs|read-props
     */
    private function getChildNodes(
        array &$child_nodes,
        int $source_key,
        array $source_path_types,
        array $visited_source_keys,
    ): bool {
        if (!isset($this->forward_edges[$source_key])) {
            return false;
        }

        foreach ($this->forward_edges[$source_key] as $to_key => $path) {
            $path_type = $path->type;

            if ($path_type === 'variable-use'
                || $path_type === 'closure-use'
                || $path_type === 'global-use'
                || $path_type === 'use-inside-instance-property'
                || $path_type === 'use-inside-static-property'
                || $path_type === 'use-inside-call'
                || $path_type === 'use-inside-conditional'
                || $path_type === 'use-inside-isset'
                || $path_type === 'arg'
                || $path_type === 'comparison'
            ) {
                return true;
            }

            if (isset($visited_source_keys[$to_key])) {
                continue;
            }

            if (self::shouldIgnoreFetch($path_type, 'arraykey', $source_path_types)) {
                continue;
            }

            if (self::shouldIgnoreFetch($path_type, 'arrayvalue', $source_path_types)) {
                continue;
            }

            if (self::shouldIgnoreFetch($path_type, 'property', $source_path_types)) {
                continue;
            }

            $path_types = $source_path_types;
            $path_types []= $path_type;

            $child_nodes[$to_key] = $path_types;
        }

        return false;
    }

    /**
     * @param array<int, bool> $visited_source_keys
     * @param list<DataFlowNode> $new_parent_nodes
     * @param-out list<DataFlowNode> $new_parent_nodes
     * @psalm-capabilities write-refs|read-props
     */
    private function getParentNodes(
        array &$new_parent_nodes,
        DataFlowNode $destination,
        array $visited_source_keys,
    ): bool {
        if (!isset($this->backward_edges[$destination->key])) {
            return false;
        }

        $had = false;
        foreach ($this->backward_edges[$destination->key] as $from_key => $_) {
            if (isset($visited_source_keys[$from_key])) {
                continue;
            }

            if (isset($this->nodes[$from_key])) {
                $new_parent_nodes[] = $this->nodes[$from_key];
                $had = true;
            }
        }

        return $had;
    }

    /**
     * @return array{int, int, int, float}
     * @psalm-mutation-free
     */
    public function getEdgeStats(): array
    {
        $lengths = 0;

        $destination_counts = [];
        $origin_counts = [];

        foreach ($this->forward_edges as $from_key => $destinations) {
            foreach ($destinations as $to_key => $path) {
                if ($path->length === 0) {
                    continue;
                }

                $lengths += $path->length;

                if (!isset($destination_counts[$to_key])) {
                    $destination_counts[$to_key] = 0;
                }

                $destination_counts[$to_key]++;

                $origin_counts[$from_key] = true;
            }
        }

        $count = array_sum($destination_counts);

        if (!$count) {
            return [0, 0, 0, 0.0];
        }

        $mean = $lengths / $count;

        return [$count, count($origin_counts), count($destination_counts), $mean];
    }
}
