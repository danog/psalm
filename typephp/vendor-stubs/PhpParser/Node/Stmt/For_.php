<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class For_ extends \PhpParser\Node\Stmt
{
    public array $init;
    public array $cond;
    public array $loop;
    public array $stmts;
    public function __construct(array $subNodes = array (
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
