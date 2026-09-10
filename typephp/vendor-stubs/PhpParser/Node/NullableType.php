<?php

declare(strict_types=1);

namespace PhpParser\Node;

class NullableType extends \PhpParser\Node\ComplexType
{
    public \PhpParser\Node $type;
    public function __construct(\PhpParser\Node $type, array $attributes = array (
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
