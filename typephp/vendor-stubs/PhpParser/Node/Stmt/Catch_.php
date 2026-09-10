<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Catch_ extends \PhpParser\Node\Stmt
{
    public array $types;
    public ?\PhpParser\Node\Expr\Variable $var = null;
    public array $stmts;
    public function __construct(array $types, ?\PhpParser\Node\Expr\Variable $var = NULL, array $stmts = array (
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
