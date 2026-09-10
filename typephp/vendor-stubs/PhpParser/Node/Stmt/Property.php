<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Property extends \PhpParser\Node\Stmt
{
    public int $flags;
    public array $props;
    public ?\PhpParser\Node $type = null;
    public array $attrGroups;
    public array $hooks;
    public function __construct(int $flags, array $props, array $attributes = array (
), ?\PhpParser\Node $type = NULL, array $attrGroups = array (
), array $hooks = array (
))
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
    public function isStatic(): bool
    {
        return false;
    }
    public function isReadonly(): bool
    {
        return false;
    }
    public function isAbstract(): bool
    {
        return false;
    }
    public function isFinal(): bool
    {
        return false;
    }
    public function isPublicSet(): bool
    {
        return false;
    }
    public function isProtectedSet(): bool
    {
        return false;
    }
    public function isPrivateSet(): bool
    {
        return false;
    }
    public function getType(): string
    {
        return '';
    }
}
