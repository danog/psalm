<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Function_ extends \PhpParser\Node\Stmt implements \PhpParser\Node\FunctionLike
{
    public bool $byRef;
    public \PhpParser\Node\Identifier $name;
    public array $params;
    public ?\PhpParser\Node $returnType = null;
    public array $stmts;
    public array $attrGroups;
    public ?\PhpParser\Node\Name $namespacedName = null;
    public function __construct($name, array $subNodes = array (
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
