<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class Yield_ extends \PhpParser\Node\Expr
{
    public ?\PhpParser\Node\Expr $key = null;
    public ?\PhpParser\Node\Expr $value = null;
    public function __construct(?\PhpParser\Node\Expr $value = NULL, ?\PhpParser\Node\Expr $key = NULL, array $attributes = array (
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
