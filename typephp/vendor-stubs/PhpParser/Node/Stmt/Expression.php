<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Expression extends \PhpParser\Node\Stmt
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
