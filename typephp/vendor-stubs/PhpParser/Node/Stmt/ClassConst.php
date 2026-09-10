<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class ClassConst extends \PhpParser\Node\Stmt
{
    public int $flags;
    public array $consts;
    public array $attrGroups;
    public ?\PhpParser\Node $type = null;
    public function __construct(array $consts, int $flags = 0, array $attributes = array (
), array $attrGroups = array (
), ?\PhpParser\Node $type = NULL)
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
    public function isPublic(): bool
    {
        return false;
    }
    public function isProtected(): bool
    {
        return false;
    }
    public function isPrivate(): bool
    {
        return false;
    }
    public function isFinal(): bool
    {
        return false;
    }
    public function getType(): string
    {
        return '';
    }
}
