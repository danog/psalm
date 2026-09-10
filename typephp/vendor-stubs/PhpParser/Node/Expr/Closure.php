<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class Closure extends \PhpParser\Node\Expr implements \PhpParser\Node\FunctionLike
{
    public bool $static;
    public bool $byRef;
    public array $params;
    public array $uses;
    public ?\PhpParser\Node $returnType = null;
    public array $stmts;
    public array $attrGroups;
    public function __construct(array $subNodes = array (
), array $attributes = array (
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
    public function getStmts(): array
    {
        return [];
    }
    public function getAttrGroups(): array
    {
        return [];
    }
    public function getType(): string
    {
        return '';
    }
}
