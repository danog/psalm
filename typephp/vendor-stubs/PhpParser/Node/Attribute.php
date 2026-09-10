<?php

declare(strict_types=1);

namespace PhpParser\Node;

class Attribute extends \PhpParser\NodeAbstract
{
    public \PhpParser\Node\Name $name;
    public array $args;
    public function __construct(\PhpParser\Node\Name $name, array $args = array (
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
