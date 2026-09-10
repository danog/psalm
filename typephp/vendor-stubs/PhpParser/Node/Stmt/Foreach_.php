<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Foreach_ extends \PhpParser\Node\Stmt
{
    public \PhpParser\Node\Expr $expr;
    public ?\PhpParser\Node\Expr $keyVar = null;
    public bool $byRef;
    public \PhpParser\Node\Expr $valueVar;
    public array $stmts;
    public function __construct(\PhpParser\Node\Expr $expr, \PhpParser\Node\Expr $valueVar, array $subNodes = array (
), array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
    public function getType(): string
    {
        return '';
    }
}
