<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class Isset_ extends \PhpParser\Node\Expr
{
    public array $vars;
    public function __construct(array $vars, array $attributes = array (
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
