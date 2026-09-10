<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class InlineHTML extends \PhpParser\Node\Stmt
{
    public string $value;
    public function __construct(string $value, array $attributes = array (
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
