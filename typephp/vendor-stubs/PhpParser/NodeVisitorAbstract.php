<?php

declare(strict_types=1);

namespace PhpParser;

abstract class NodeVisitorAbstract implements \PhpParser\NodeVisitor
{
    public function beforeTraverse(array $nodes)
    {
        throw new \RuntimeException('vendor stub');
    }
    public function enterNode(\PhpParser\Node $node)
    {
        throw new \RuntimeException('vendor stub');
    }
    public function leaveNode(\PhpParser\Node $node)
    {
        throw new \RuntimeException('vendor stub');
    }
    public function afterTraverse(array $nodes)
    {
        throw new \RuntimeException('vendor stub');
    }
}
