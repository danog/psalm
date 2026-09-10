<?php

declare(strict_types=1);

namespace PhpParser\Node;

class DeclareItem extends \PhpParser\NodeAbstract
{
    public \PhpParser\Node\Identifier $key;
    public \PhpParser\Node\Expr $value;
    public function __construct($key, \PhpParser\Node\Expr $value, array $attributes = array (
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
