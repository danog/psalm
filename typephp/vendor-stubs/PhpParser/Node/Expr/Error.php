<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class Error extends \PhpParser\Node\Expr
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
    public function getType(): string
    {
        return '';
    }
}
