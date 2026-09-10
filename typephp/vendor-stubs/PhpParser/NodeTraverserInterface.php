<?php

declare(strict_types=1);

namespace PhpParser;

interface NodeTraverserInterface
{
    public function addVisitor(\PhpParser\NodeVisitor $visitor): void;
    public function removeVisitor(\PhpParser\NodeVisitor $visitor): void;
    public function traverse(array $nodes): array;
}
