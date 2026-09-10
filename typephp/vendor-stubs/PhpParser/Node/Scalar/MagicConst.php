<?php

declare(strict_types=1);

namespace PhpParser\Node\Scalar;

abstract class MagicConst extends \PhpParser\Node\Scalar
{
    public function __construct(array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
    public abstract function getName(): string;
}
