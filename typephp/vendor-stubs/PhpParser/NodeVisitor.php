<?php

declare(strict_types=1);

namespace PhpParser;

interface NodeVisitor
{
    public const DONT_TRAVERSE_CHILDREN = 1;
    public const STOP_TRAVERSAL = 2;
    public const REMOVE_NODE = 3;
    public const DONT_TRAVERSE_CURRENT_AND_CHILDREN = 4;
    public const REPLACE_WITH_NULL = 5;
    public function beforeTraverse(array $nodes);
    public function enterNode(\PhpParser\Node $node);
    public function leaveNode(\PhpParser\Node $node);
    public function afterTraverse(array $nodes);
}
