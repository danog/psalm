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
     * @param lowercase-string $id
     */
    private function addNodeSimple(
        string $id,
        string $label
    ): DataFlowNode {
        if (isset($this->nodes[$id])) {
            $this->nodes[$id];
        }
        return $this->nodes[$id] = DataFlowNode::make(
            $id,
            $label,
            null
        );
    }

    public function addClassConstOrPropertyNode(string $id): DataFlowNode {
        $f = explode('::', $id, 2);
        $class = $this->addClassLikeNode($f[1]);
        $member = $this->addNodeSimple($id, 'member');
        $this->addPath(
            $class,
            $member,
            'member-of-class',
        );
        return $member;
    }


    /**
     * @param lowercase-string $function_id
     */
    public function addFunctionLikeNode(string $function_id): DataFlowNode {
        $f = explode('::', $function_id, 2);
        if (count($f) === 1) {
            return $this->addNodeSimple('function-'.$function_id, 'function');
        }
        $class = $this->addClassLikeNode($f[1]);
        $method = $this->addNodeSimple($function_id, 'method');
        $this->addPath(
            $class,
            $method,
            'method-of-class',
        );
        return $method;
    }

    /**
     * @param lowercase-string $fq_class_name_lc
     */
    public function addClassLikeNode(string $fq_class_name_lc): DataFlowNode
    {
        return $this->addNodeSimple($fq_class_name_lc, 'class');
    }

    /**
     * @param lowercase-string $fq_class_name_lc
     */
    public function addMethodReferenceToClass(string $calling_function_id, string $fq_class_name_lc): void
    {
        $caller = $this->addFunctionLikeNode($calling_function_id);
        $class = $this->addClassLikeNode($fq_class_name_lc);
        $this->addPath(
            $class,
            $caller,
            'method-call',
        );
    }

    /**
     * @param lowercase-string $fq_class_name_lc
     */
    public function addNonMethodReferenceToClass(string $source_file, string $fq_class_name_lc): void
    {
        $src = $this->addNodeSimple(
            $source_file,
            'file',
        );
        $class = $this->addClassLikeNode($fq_class_name_lc);
        $this->addPath(
            $class,
            $src,
            'nonmethod-reference',
        );
    }

    public function addMethodReferenceToClassMember(string $calling_function_id, string $member): void
    {
        $caller = $this->addFunctionLikeNode($calling_function_id);
        $class = $this->addClassConstOrPropertyNode($member);
        $this->addPath(
            $class,
            $caller,
            'member-usage',
        );
    }

    public function addFileReferenceToClassMember(string $source_file, string $member): void
    {
        $src = $this->addNodeSimple($source_file, 'file');
        $class = $this->addClassConstOrPropertyNode($member);
        $this->addPath(
            $class,
            $src,
            'nonmethod-usage',
        );
    }

    /**
     * @param lowercase-string $fq_class_name_lc
     */
    public function markClassAsUsed(
        string $fq_class_name_lc,
    ): void {
        $this->addPath(
            $this->addClassLikeNode($fq_class_name_lc),
            $this->addNodeSimple('', 'psalm-api'),
            'psalm-api',
        );
    }

    /**
     * @param lowercase-string $function_id
     */
    public function markMethodAsUsed(
        string $function_id,
    ): void {
        $this->addPath(
            $this->addFunctionLikeNode($function_id),
            $this->addNodeSimple('', 'psalm-api'),
            'psalm-api',
        );
    }

    /**
     * @param lowercase-string $name_lc
     */
    public function isNodeUsed(string $name_lc): bool
    {
        if (!isset($this->nodes[$name_lc])) {
            return false;
        }

        if (str_ends_with($name_lc, 'setpossiblyundefined')) {
            var_dump('here');
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
            if ($path->type === 'psalm-api' || $path->type === 'nonmethod-reference') {
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
