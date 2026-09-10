<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class Ternary extends \PhpParser\Node\Expr
{
    public \PhpParser\Node\Expr $cond;
    public ?\PhpParser\Node\Expr $if = null;
    public \PhpParser\Node\Expr $else;
    public function __construct(\PhpParser\Node\Expr $cond, ?\PhpParser\Node\Expr $if, \PhpParser\Node\Expr $else, array $attributes = array (
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
