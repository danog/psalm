<?php

declare(strict_types=1);

namespace PhpParser\Node;

class ClosureUse extends \PhpParser\NodeAbstract
{
    public \PhpParser\Node\Expr\Variable $var;
    public bool $byRef;
    public function __construct(\PhpParser\Node\Expr\Variable $var, bool $byRef = false, array $attributes = array (
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
