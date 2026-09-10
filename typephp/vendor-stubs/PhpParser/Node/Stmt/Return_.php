<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Return_ extends \PhpParser\Node\Stmt
{
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
