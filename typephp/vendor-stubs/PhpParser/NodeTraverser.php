<?php

declare(strict_types=1);

namespace PhpParser;

class NodeTraverser implements \PhpParser\NodeTraverserInterface
{
    public const DONT_TRAVERSE_CHILDREN = 1;
    public const STOP_TRAVERSAL = 2;
    public const REMOVE_NODE = 3;
    public const DONT_TRAVERSE_CURRENT_AND_CHILDREN = 4;
    protected array $visitors = array (
);
    protected bool $stopTraversal;
    public function __construct(\PhpParser\NodeVisitor ...$visitors)
    {
        throw new \RuntimeException('vendor stub');
    }
    public function addVisitor(\PhpParser\NodeVisitor $visitor): void
    {
        throw new \RuntimeException('vendor stub');
    }
    public function removeVisitor(\PhpParser\NodeVisitor $visitor): void
    {
        throw new \RuntimeException('vendor stub');
    }
    public function traverse(array $nodes): array
    {
        return [];
    }
    protected function traverseNode(\PhpParser\Node $node): void
    {
        throw new \RuntimeException('vendor stub');
    }
    protected function traverseArray(array $nodes): array
    {
        return [];
    }
    private function ensureReplacementReasonable(\PhpParser\Node $old, \PhpParser\Node $new): void
    {
        throw new \RuntimeException('vendor stub');
    }
}
