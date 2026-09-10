<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class ArrowFunction extends \PhpParser\Node\Expr implements \PhpParser\Node\FunctionLike
{
    public bool $static;
    public bool $byRef;
    public array $params = array (
);
    public ?\PhpParser\Node $returnType = null;
    public \PhpParser\Node\Expr $expr;
    public array $attrGroups;
    public function __construct(array $subNodes, array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
    public function returnsByRef(): bool
    {
        return false;
    }
    public function getParams(): array
    {
        return [];
    }
    public function getReturnType()
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getAttrGroups(): array
    {
        return [];
    }
    public function getStmts(): array
    {
        return [];
    }
    public function getType(): string
    {
        return '';
    }
}
