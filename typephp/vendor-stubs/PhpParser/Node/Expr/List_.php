<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class List_ extends \PhpParser\Node\Expr
{
    public const KIND_LIST = 1;
    public const KIND_ARRAY = 2;
    public array $items;
    public function __construct(array $items, array $attributes = array (
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
