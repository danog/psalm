<?php

declare(strict_types=1);

namespace PhpParser\Node;

class Arg extends \PhpParser\NodeAbstract
{
    public ?\PhpParser\Node\Identifier $name = null;
    public \PhpParser\Node\Expr $value;
    public bool $byRef;
    public bool $unpack;
    public function __construct(\PhpParser\Node\Expr $value, bool $byRef = false, bool $unpack = false, array $attributes = array (
), ?\PhpParser\Node\Identifier $name = NULL)
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
