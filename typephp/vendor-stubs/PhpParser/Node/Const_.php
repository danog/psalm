<?php

declare(strict_types=1);

namespace PhpParser\Node;

class Const_ extends \PhpParser\NodeAbstract
{
    public \PhpParser\Node\Identifier $name;
    public \PhpParser\Node\Expr $value;
    public ?\PhpParser\Node\Name $namespacedName = null;
    public function __construct($name, \PhpParser\Node\Expr $value, array $attributes = array (
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
