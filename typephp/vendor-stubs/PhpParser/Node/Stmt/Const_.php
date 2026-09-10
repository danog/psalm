<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Const_ extends \PhpParser\Node\Stmt
{
    public array $consts;
    public array $attrGroups;
    public function __construct(array $consts, array $attributes = array (
), array $attrGroups = array (
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
