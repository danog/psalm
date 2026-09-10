<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Else_ extends \PhpParser\Node\Stmt
{
    public array $stmts;
    public function __construct(array $stmts = array (
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
