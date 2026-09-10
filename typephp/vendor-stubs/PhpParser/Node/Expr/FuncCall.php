<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

class FuncCall extends \PhpParser\Node\Expr\CallLike
{
    public \PhpParser\Node $name;
    public array $args;
    public function __construct(\PhpParser\Node $name, array $args = array (
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
    public function getRawArgs(): array
    {
        return [];
    }
}
