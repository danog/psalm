<?php

declare(strict_types=1);

namespace PhpParser\Node;

class ArrayItem extends \PhpParser\NodeAbstract
{
    public ?\PhpParser\Node\Expr $key = null;
    public \PhpParser\Node\Expr $value;
    public bool $byRef;
    public bool $unpack;
    public function __construct(\PhpParser\Node\Expr $value, ?\PhpParser\Node\Expr $key = NULL, bool $byRef = false, array $attributes = array (
), bool $unpack = false)
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
