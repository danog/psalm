<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class PostInc extends \PhpParser\Node\Expr
{
    public \PhpParser\Node\Expr $var;
    public function __construct(\PhpParser\Node\Expr $var, array $attributes = array (
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
