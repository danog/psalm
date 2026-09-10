<?php

declare(strict_types=1);

namespace PhpParser\Node;

class Param extends \PhpParser\NodeAbstract
{
    public ?\PhpParser\Node $type = null;
    public bool $byRef;
    public bool $variadic;
    public \PhpParser\Node\Expr $var;
    public ?\PhpParser\Node\Expr $default = null;
    public int $flags;
    public array $attrGroups;
    public array $hooks;
    public function __construct(\PhpParser\Node\Expr $var, ?\PhpParser\Node\Expr $default = NULL, ?\PhpParser\Node $type = NULL, bool $byRef = false, bool $variadic = false, array $attributes = array (
), int $flags = 0, array $attrGroups = array (
), array $hooks = array (
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
    public function isPromoted(): bool
    {
        return false;
    }
    public function isFinal(): bool
    {
        return false;
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
    public function isReadonly(): bool
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
}
