<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class Array_ extends \PhpParser\Node\Expr
{
    public const KIND_LONG = 1;
    public const KIND_SHORT = 2;
    public array $items;
    public function __construct(array $items = array (
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
