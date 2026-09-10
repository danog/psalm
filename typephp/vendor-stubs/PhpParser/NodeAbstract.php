<?php

declare(strict_types=1);

namespace PhpParser;

abstract class NodeAbstract implements \PhpParser\Node, \JsonSerializable
{
    protected array $attributes;
    public function __construct(array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getLine(): int
    {
        return 0;
    }
    public function getStartLine(): int
    {
        return 0;
    }
    public function getEndLine(): int
    {
        return 0;
    }
    public function getStartTokenPos(): int
    {
        return 0;
    }
    public function getEndTokenPos(): int
    {
        return 0;
    }
    public function getStartFilePos(): int
    {
        return 0;
    }
    public function getEndFilePos(): int
    {
        return 0;
    }
    public function getComments(): array
    {
        return [];
    }
    public function getDocComment(): ?\PhpParser\Comment\Doc
    {
        return null;
    }
    public function setDocComment(\PhpParser\Comment\Doc $docComment): void
    {
        throw new \RuntimeException('vendor stub');
    }
    public function setAttribute(string $key, $value): void
    {
        throw new \RuntimeException('vendor stub');
    }
    public function hasAttribute(string $key): bool
    {
        return false;
    }
    public function getAttribute(string $key, $default = NULL)
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getAttributes(): array
    {
        return [];
    }
    public function setAttributes(array $attributes): void
    {
        throw new \RuntimeException('vendor stub');
    }
    public function jsonSerialize(): array
    {
        return [];
    }
}
