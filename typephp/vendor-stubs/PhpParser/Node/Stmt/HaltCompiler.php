<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class HaltCompiler extends \PhpParser\Node\Stmt
{
    public string $remaining;
    public function __construct(string $remaining, array $attributes = array (
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
