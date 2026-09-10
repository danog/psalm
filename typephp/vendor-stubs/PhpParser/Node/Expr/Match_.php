<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class Match_ extends \PhpParser\Node\Expr
{
    public \PhpParser\Node\Expr $cond;
    public array $arms;
    public function __construct(\PhpParser\Node\Expr $cond, array $arms = array (
), array $attributes = array (
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
