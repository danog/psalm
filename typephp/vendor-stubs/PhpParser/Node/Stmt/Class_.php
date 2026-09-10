<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Class_ extends \PhpParser\Node\Stmt\ClassLike
{
    public const MODIFIER_PUBLIC = 1;
    public const MODIFIER_PROTECTED = 2;
    public const MODIFIER_PRIVATE = 4;
    public const MODIFIER_STATIC = 8;
    public const MODIFIER_ABSTRACT = 16;
    public const MODIFIER_FINAL = 32;
    public const MODIFIER_READONLY = 64;
    public const VISIBILITY_MODIFIER_MASK = 7;
    public int $flags;
    public ?\PhpParser\Node\Name $extends = null;
    public array $implements;
    public function __construct($name, array $subNodes = array (
), array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
    public function isAbstract(): bool
    {
        return false;
    }
    public function isFinal(): bool
    {
        return false;
    }
    public function isReadonly(): bool
    {
        return false;
    }
    public function isAnonymous(): bool
    {
        return false;
    }
    public function getType(): string
    {
        return '';
    }
}
