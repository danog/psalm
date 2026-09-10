<?php

declare(strict_types=1);

namespace PhpParser\Node;

class StaticVar extends \PhpParser\NodeAbstract
{
    public \PhpParser\Node\Expr\Variable $var;
    public ?\PhpParser\Node\Expr $default = null;
    public function __construct(\PhpParser\Node\Expr\Variable $var, ?\PhpParser\Node\Expr $default = NULL, array $attributes = array (
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
