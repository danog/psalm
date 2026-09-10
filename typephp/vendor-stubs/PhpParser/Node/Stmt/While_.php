<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class While_ extends \PhpParser\Node\Stmt
{
    public \PhpParser\Node\Expr $cond;
    public array $stmts;
    public function __construct(\PhpParser\Node\Expr $cond, array $stmts = array (
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
