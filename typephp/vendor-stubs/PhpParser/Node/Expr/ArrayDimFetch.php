<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class ArrayDimFetch extends \PhpParser\Node\Expr
{
    public \PhpParser\Node\Expr $var;
    public ?\PhpParser\Node\Expr $dim = null;
    public function __construct(\PhpParser\Node\Expr $var, ?\PhpParser\Node\Expr $dim = NULL, array $attributes = array (
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
