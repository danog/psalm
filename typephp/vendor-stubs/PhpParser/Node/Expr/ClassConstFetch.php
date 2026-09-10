<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class ClassConstFetch extends \PhpParser\Node\Expr
{
    public \PhpParser\Node $class;
    public \PhpParser\Node $name;
    public function __construct(\PhpParser\Node $class, $name, array $attributes = array (
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
