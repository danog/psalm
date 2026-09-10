<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Continue_ extends \PhpParser\Node\Stmt
{
    public ?\PhpParser\Node\Expr $num = null;
    public function __construct(?\PhpParser\Node\Expr $num = NULL, array $attributes = array (
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
