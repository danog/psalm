<?php

declare(strict_types=1);

namespace PhpParser\Node\Scalar;

class InterpolatedString extends \PhpParser\Node\Scalar
{
    public array $parts;
    public function __construct(array $parts, array $attributes = array (
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
