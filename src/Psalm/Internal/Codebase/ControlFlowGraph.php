<?php

namespace Psalm\Internal\Codebase;

use Psalm\CodeLocation;
use Psalm\Internal\ControlFlow\Path;
use Psalm\Internal\ControlFlow\TaintSink;
use Psalm\Internal\ControlFlow\TaintSource;
use Psalm\Internal\ControlFlow\ControlFlowNode;
use Psalm\IssueBuffer;
use Psalm\Issue\TaintedInput;
use function array_merge;
use function count;
use function implode;
use function substr;
use function strlen;
use function array_intersect;
use function array_reverse;

class ControlFlowGraph
{
    /** @var array<string, array<string, Path>> */
    protected $forward_edges = [];

    public function addNode(ControlFlowNode $node) : void
    {
    }

    /**
     * @param array<string> $added_taints
     * @param array<string> $removed_taints
     */
    public function addPath(
        ControlFlowNode $from,
        ControlFlowNode $to,
        string $path_type,
        ?array $added_taints = null,
        ?array $removed_taints = null
    ) : void {
        $from_id = $from->id;
        $to_id = $to->id;

        if ($from_id === $to_id) {
            return;
        }

        $this->forward_edges[$from_id][$to_id] = new Path($path_type, $added_taints, $removed_taints);
    }

    public function isVariableUsed(ControlFlowNode $assignment_node) : bool
    {
        $visited_source_ids = [];

        $sources = [$assignment_node];

        for ($i = 0; count($sources) && $i < 100; $i++) {
            $new_sources = [];

            foreach ($sources as $source) {
                $visited_source_ids[$source->id] = true;

                $child_nodes = $this->getChildNodes(
                    $source,
                    $visited_source_ids
                );

                if ($child_nodes === null) {
                    return true;
                }

                $new_sources = array_merge(
                    $new_sources,
                    $child_nodes
                );
            }

            $sources = $new_sources;
        }

        return false;
    }

    /**
     * @param array<string, bool> $visited_source_ids
     * @return ?array<ControlFlowNode>
     */
    private function getChildNodes(
        ControlFlowNode $generated_source,
        array $visited_source_ids
    ) : ?array {
        $new_sources = [];

        if (!isset($this->forward_edges[$generated_source->id])) {
            return [];
        }

        foreach ($this->forward_edges[$generated_source->id] as $to_id => $path) {
            $path_type = $path->type;

            if ($path->type === 'variable-use'
                || $path->type === 'closure-use'
                || $path->type === 'global-use'
                || $path->type === 'use-inside-instance-property'
                || $path->type === 'use-inside-static-property'
                || $path->type === 'use-inside-call'
                || $path->type === 'use-inside-conditional'
                || $path->type === 'use-inside-isset'
                || $path->type === 'arg'
            ) {
                return null;
            }

            if (isset($visited_source_ids[$to_id])) {
                continue;
            }

            if (self::shouldIgnoreFetch($path_type, 'array', $generated_source->path_types)) {
                continue;
            }

            if (self::shouldIgnoreFetch($path_type, 'property', $generated_source->path_types)) {
                continue;
            }

            $new_destination = new ControlFlowNode($to_id, $to_id, null);
            $new_destination->path_types = array_merge($generated_source->path_types, [$path_type]);

            $new_sources[$to_id] = $new_destination;
        }

        return $new_sources;
    }

    /**
     * @param array<string> $previous_path_types
     *
     * @psalm-pure
     */
    protected static function shouldIgnoreFetch(
        string $path_type,
        string $expression_type,
        array $previous_path_types
    ) : bool {
        $el = \strlen($expression_type);

        if (substr($path_type, 0, $el + 7) === $expression_type . '-fetch-') {
            $fetch_nesting = 0;

            $previous_path_types = array_reverse($previous_path_types);

            foreach ($previous_path_types as $previous_path_type) {
                if ($previous_path_type === $expression_type . '-assignment') {
                    if ($fetch_nesting === 0) {
                        return false;
                    }

                    $fetch_nesting--;
                }

                if (substr($previous_path_type, 0, $el + 6) === $expression_type . '-fetch') {
                    $fetch_nesting++;
                }

                if (substr($previous_path_type, 0, $el + 12) === $expression_type . '-assignment-') {
                    if ($fetch_nesting > 0) {
                        $fetch_nesting--;
                        continue;
                    }

                    if (substr($previous_path_type, $el + 12) === substr($path_type, $el + 7)) {
                        return false;
                    }

                    return true;
                }
            }
        }

        return false;
    }
}
