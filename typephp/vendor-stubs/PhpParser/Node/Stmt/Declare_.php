<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Declare_ extends \PhpParser\Node\Stmt
{
    public array $declares;
    public ?array $stmts = null;
    public function __construct(array $declares, ?array $stmts = NULL, array $attributes = array (
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
