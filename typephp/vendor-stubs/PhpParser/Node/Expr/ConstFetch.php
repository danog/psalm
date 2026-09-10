<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class ConstFetch extends \PhpParser\Node\Expr
{
    public \PhpParser\Node\Name $name;
    public function __construct(\PhpParser\Node\Name $name, array $attributes = array (
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
