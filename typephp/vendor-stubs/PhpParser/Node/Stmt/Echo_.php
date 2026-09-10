<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Echo_ extends \PhpParser\Node\Stmt
{
    public array $exprs;
    public function __construct(array $exprs, array $attributes = array (
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
