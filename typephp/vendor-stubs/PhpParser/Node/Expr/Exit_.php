<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class Exit_ extends \PhpParser\Node\Expr
{
    public const KIND_EXIT = 1;
    public const KIND_DIE = 2;
    public ?\PhpParser\Node\Expr $expr = null;
    public function __construct(?\PhpParser\Node\Expr $expr = NULL, array $attributes = array (
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
