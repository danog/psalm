<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class BitwiseNot extends \PhpParser\Node\Expr
{
    public \PhpParser\Node\Expr $expr;
    public function __construct(\PhpParser\Node\Expr $expr, array $attributes = array (
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
