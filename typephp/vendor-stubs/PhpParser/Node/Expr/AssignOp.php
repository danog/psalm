<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

abstract class AssignOp extends \PhpParser\Node\Expr
{
    public \PhpParser\Node\Expr $var;
    public \PhpParser\Node\Expr $expr;
    public function __construct(\PhpParser\Node\Expr $var, \PhpParser\Node\Expr $expr, array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
}
