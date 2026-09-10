<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

abstract class BinaryOp extends \PhpParser\Node\Expr
{
    public \PhpParser\Node\Expr $left;
    public \PhpParser\Node\Expr $right;
    public function __construct(\PhpParser\Node\Expr $left, \PhpParser\Node\Expr $right, array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
    public abstract function getOperatorSigil(): string;
}
