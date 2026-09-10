<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class ShellExec extends \PhpParser\Node\Expr
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
