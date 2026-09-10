<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class PropertyFetch extends \PhpParser\Node\Expr
{
    public \PhpParser\Node\Expr $var;
    public \PhpParser\Node $name;
    public function __construct(\PhpParser\Node\Expr $var, $name, array $attributes = array (
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
