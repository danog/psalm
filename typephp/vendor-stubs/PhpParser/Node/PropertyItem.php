<?php

declare(strict_types=1);

namespace PhpParser\Node;

class PropertyItem extends \PhpParser\NodeAbstract
{
    public \PhpParser\Node\VarLikeIdentifier $name;
    public ?\PhpParser\Node\Expr $default = null;
    public function __construct($name, ?\PhpParser\Node\Expr $default = NULL, array $attributes = array (
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
