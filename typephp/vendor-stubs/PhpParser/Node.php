<?php

declare(strict_types=1);

namespace PhpParser;

interface Node
{
    public function getType(): string;
    public function getSubNodeNames(): array;
    public function getLine(): int;
    public function getStartLine(): int;
    public function getEndLine(): int;
    public function getStartTokenPos(): int;
    public function getEndTokenPos(): int;
    public function getStartFilePos(): int;
    public function getEndFilePos(): int;
    public function getComments(): array;
    public function getDocComment(): ?\PhpParser\Comment\Doc;
    public function setDocComment(\PhpParser\Comment\Doc $docComment): void;
    public function setAttribute(string $key, $value): void;
    public function hasAttribute(string $key): bool;
    public function getAttribute(string $key, $default = NULL);
    public function getAttributes(): array;
    public function setAttributes(array $attributes): void;
}
