<?php

declare(strict_types=1);

namespace PhpParser\Node;

class UnionType extends \PhpParser\Node\ComplexType
{
    public array $types;
    public function __construct(array $types, array $attributes = array (
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
