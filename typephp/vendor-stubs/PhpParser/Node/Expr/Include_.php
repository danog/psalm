<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class Include_ extends \PhpParser\Node\Expr
{
    public const TYPE_INCLUDE = 1;
    public const TYPE_INCLUDE_ONCE = 2;
    public const TYPE_REQUIRE = 3;
    public const TYPE_REQUIRE_ONCE = 4;
    public \PhpParser\Node\Expr $expr;
    public int $type;
    public function __construct(\PhpParser\Node\Expr $expr, int $type, array $attributes = array (
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
