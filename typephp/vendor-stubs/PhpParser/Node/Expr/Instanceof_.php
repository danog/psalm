<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class Instanceof_ extends \PhpParser\Node\Expr
{
    public \PhpParser\Node\Expr $expr;
    public \PhpParser\Node $class;
    public function __construct(\PhpParser\Node\Expr $expr, \PhpParser\Node $class, array $attributes = array (
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
