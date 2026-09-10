<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Label extends \PhpParser\Node\Stmt
{
    public \PhpParser\Node\Identifier $name;
    public function __construct($name, array $attributes = array (
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
