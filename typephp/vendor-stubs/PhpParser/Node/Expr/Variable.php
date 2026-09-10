<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class Variable extends \PhpParser\Node\Expr
{
    public $name = NULL;
    public function __construct($name, array $attributes = array (
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
