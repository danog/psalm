<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use Override;
use Psalm\CodeLocation;
use Psalm\Internal\DataFlow\DataFlowNode;
use Psalm\Internal\DataFlow\Path;

use function abs;
use function count;

/**
 * @internal
 */
final class ClassUseGraph extends DataFlowGraph
{
    /** @var array<string, DataFlowNode> */
    private array $nodes = [];

    #[Override]
    public function addNode(DataFlowNode $node): void
    {
        $this->nodes[$node->id] = $node;
    }

    /**
     * @param array<string> $added_taints
     * @param array<string> $removed_taints
     */
    #[Override]
    public function addPath(
        DataFlowNode $from,
        DataFlowNode $to,
        string $path_type,
        ?array $added_taints = null,
        ?array $removed_taints = null,
    ): void {
        $from_id = $from->id;
        $to_id = $to->id;

        if ($from_id === $to_id) {
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

        $this->forward_edges[$from_id][$to_id] = new Path($path_type, $length);
    }

    public function isClassUsed(DataFlowNode $assignment_node): bool
    {
        $visited_source_ids = [];

        $sources = [$assignment_node];

        for ($i = 0; count($sources) && $i < 200; $i++) {
            $new_child_nodes = [];

            foreach ($sources as $source) {
                $visited_source_ids[$source->id] = true;

                $child_nodes = $this->getChildNodes(
                    $source,
                    $visited_source_ids,
                );

                if ($child_nodes === null) {
                    return true;
                }

                $new_child_nodes = [...$new_child_nodes, ...$child_nodes];
            }

            $sources = $new_child_nodes;
        }

        return false;
    }

    /**
     * @param array<string, bool> $visited_source_ids
     * @return array<string, DataFlowNode>|null
     */
    private function getChildNodes(
        DataFlowNode $generated_source,
        array $visited_source_ids,
    ): ?array {
        $new_child_nodes = [];

        if (!isset($this->forward_edges[$generated_source->id])) {
            return [];
        }

        foreach ($this->forward_edges[$generated_source->id] as $to_id => $path) {
            $path_type = $path->type;

            if ($path->type === 'psalm-api') {
                return null;
            }

            if (isset($visited_source_ids[$to_id])) {
                continue;
            }

            $new_destination = new DataFlowNode($to_id, $to_id, null);
            $new_destination->path_types = [...$generated_source->path_types, ...[$path_type]];

            $new_child_nodes[$to_id] = $new_destination;
        }

        return $new_child_nodes;
    }
}
