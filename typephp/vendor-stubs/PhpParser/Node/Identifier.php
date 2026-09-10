<?php

declare(strict_types=1);

namespace PhpParser\Node;

class Identifier extends \PhpParser\NodeAbstract implements \Stringable
{
    public string $name;
    private static array $specialClassNames = array (
  'self' => true,
  'parent' => true,
  'static' => true,
);
    public function __construct(string $name, array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
    public function toString(): string
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
    public function getType(): string
    {
        return '';
    }
}
