<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use Override;
use Psalm\Aliases;
use Psalm\CodeLocation;
use Psalm\Internal\DataFlow\DataFlowNode;
use Psalm\Internal\DataFlow\Path;
use Psalm\Type\Atomic\TString;

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
     * @param lowercase-string $fq_class_name_lc
     */
    public function addMethodReferenceToClass(string $calling_function_id, string $fq_class_name_lc): void
    {
        [, $class] = explode('::', $calling_function_id, 2);
        if (!isset($this->nodes[$class])) {
            $this->nodes[$class] = DataFlowNode::make(
                $class,
                'class',
                null,
            );
        }
        if (!isset($this->nodes[$calling_function_id])) {
            $this->nodes[$calling_function_id] = DataFlowNode::make(
                $calling_function_id,
                'method',
                null,
            );
            $this->addPath(
                $this->nodes[$calling_function_id],
                $this->nodes[$class],
                'method-of-class',
            );
        }
        if (!isset($this->nodes[$fq_class_name_lc])) {
            $this->nodes[$fq_class_name_lc] = DataFlowNode::make(
                $fq_class_name_lc,
                'class',
                null,
            );
        }
        $this->addPath(
            $this->nodes[$calling_function_id],
            $this->nodes[$fq_class_name_lc],
            'method-call',
        );
    }
    /**
     * @param lowercase-string $fq_class_name_lc
     */
    public function addNonMethodReferenceToClass(string $source_file, string $fq_class_name_lc): void
    {
        if (!isset($this->nodes[$source_file])) {
            $this->nodes[$source_file] = DataFlowNode::make(
                $source_file,
                'nonmethod-reference',
                null,
            );
        }
        if (!isset($this->nodes[$fq_class_name_lc])) {
            $this->nodes[$fq_class_name_lc] = DataFlowNode::make(
                $fq_class_name_lc,
                'class',
                null,
            );
        }
        $this->addPath(
            $this->nodes[$source_file],
            $this->nodes[$fq_class_name_lc],
            'nonmethod-reference',
        );
    }

    /**
     * @param lowercase-string $fq_class_name_lc
     */
    public function markClassAsUsed(
        string $fq_class_name_lc,
    ): void {
        if (!isset($this->nodes[$fq_class_name_lc])) {
            $this->nodes[$fq_class_name_lc] = DataFlowNode::make(
                $fq_class_name_lc,
                'class',
                null,
            );
        }
        if (!isset($this->nodes[''])) {
            $this->nodes[''] = DataFlowNode::make(
                '',
                'psalm-api',
                null,
            );
        }
        
        $this->addPath(
            $this->nodes[$fq_class_name_lc],
            $this->nodes[''],
            'psalm-api',
        );
    }

    /**
     * @param lowercase-string $function_id
     */
    public function markMethodAsUsed(
        string $function_id,
    ): void {
        $split = explode('::', $function_id, 2);
        if (count($split) === 1) {
            // Todo handle functions with same name as class
            if (!isset($this->nodes[$function_id])) {
                $this->nodes[$function_id] = DataFlowNode::make(
                    $function_id,
                    'function',
                    null,
                );
            }
        } else {
            $class = $split[0];
            if (!isset($this->nodes[$class])) {
                $this->nodes[$class] = DataFlowNode::make(
                    $class,
                    'class',
                    null,
                );
            }
            if (!isset($this->nodes[$function_id])) {
                $this->nodes[$function_id] = DataFlowNode::make(
                    $function_id,
                    'method',
                    null,
                );
                $this->addPath(
                    $this->nodes[$function_id],
                    $this->nodes[$class],
                    'method-of-class',
                );
            }
        }
        
        if (!isset($this->nodes[''])) {
            $this->nodes[''] = DataFlowNode::make(
                '',
                'psalm-api',
                null,
            );
        }

        $this->addPath(
            $this->nodes[$function_id],
            $this->nodes[''],
            'psalm-api',
        );
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
        int $added_taints = 0,
        int $removed_taints = 0,
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

    /**
     * @param lowercase-string $name_lc
     */
    public function isNodeUsed(string $name_lc): bool
    {
        if (!isset($this->nodes[$name_lc])) {
            return false;
        }
        if ($name_lc === strtolower(Aliases::class)) {
            var_dump("here");
        }

        $node = $this->nodes[$name_lc];

        $visited_source_ids = [];

        $sources = [$node];

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

    public function addGraph(self $other): void
    {
        $this->nodes += $other->nodes;

        foreach ($other->forward_edges as $key => $map) {
            if (!isset($this->forward_edges[$key])) {
                $this->forward_edges[$key] = $map;
            } else {
                $this->forward_edges[$key] += $map;
            }
        }
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
            return $new_child_nodes;
        }

        foreach ($this->forward_edges[$generated_source->id] as $to_id => $path) {
            if ($path->type === 'psalm-api') {
                return null;
            }

            if (isset($visited_source_ids[$to_id])) {
                continue;
            }

            $new_destination = new DataFlowNode(
                $to_id,
                null,
                null,
                $to_id,
            );

            $new_child_nodes[$to_id] = $new_destination;
        }

        return $new_child_nodes;
    }
}
