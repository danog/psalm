<?php

declare(strict_types=1);

namespace PhpParser\Node;

class AttributeGroup extends \PhpParser\NodeAbstract
{
    public array $attrs;
    public function __construct(array $attrs, array $attributes = array (
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
