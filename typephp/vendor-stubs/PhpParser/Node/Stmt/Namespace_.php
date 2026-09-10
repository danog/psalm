<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Namespace_ extends \PhpParser\Node\Stmt
{
    public const KIND_SEMICOLON = 1;
    public const KIND_BRACED = 2;
    public ?\PhpParser\Node\Name $name = null;
    public $stmts = NULL;
    public function __construct(?\PhpParser\Node\Name $name = NULL, ?array $stmts = array (
), array $attributes = array (
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
