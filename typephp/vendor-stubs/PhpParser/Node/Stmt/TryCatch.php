<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class TryCatch extends \PhpParser\Node\Stmt
{
    public array $stmts;
    public array $catches;
    public ?\PhpParser\Node\Stmt\Finally_ $finally = null;
    public function __construct(array $stmts, array $catches, ?\PhpParser\Node\Stmt\Finally_ $finally = NULL, array $attributes = array (
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
