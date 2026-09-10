<?php

declare(strict_types=1);

namespace PhpParser\Node;

class Name extends \PhpParser\NodeAbstract implements \Stringable
{
    public string $name;
    private static array $specialClassNames = array (
  'self' => true,
  'parent' => true,
  'static' => true,
);
    public function __construct($name, array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
    public function getParts(): array
    {
        return [];
    }
    public function getFirst(): string
    {
        return '';
    }
    public function getLast(): string
    {
        return '';
    }
    public function isUnqualified(): bool
    {
        return false;
    }
    public function isQualified(): bool
    {
        return false;
    }
    public function isFullyQualified(): bool
    {
        return false;
    }
    public function isRelative(): bool
    {
        return false;
    }
    public function toString(): string
    {
        return '';
    }
    public function toCodeString(): string
    {
        return '';
    }
    public function toLowerString(): string
    {
        return '';
    }
    public function isSpecialClassName(): bool
    {
        return false;
    }
    public function __toString(): string
    {
        return '';
    }
    public function slice(int $offset, ?int $length = NULL)
    {
        throw new \RuntimeException('vendor stub');
    }
    public static function concat($name1, $name2, array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    private static function prepareName($name): string
    {
        return '';
    }
    public function getType(): string
    {
        return '';
    }
}
