<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

abstract class ClassLike extends \PhpParser\Node\Stmt
{
    public ?\PhpParser\Node\Identifier $name = null;
    public array $stmts;
    public array $attrGroups;
    public ?\PhpParser\Node\Name $namespacedName = null;
    public function getTraitUses(): array
    {
        return [];
    }
    public function getConstants(): array
    {
        return [];
    }
    public function getProperties(): array
    {
        return [];
    }
    public function getProperty(string $name): ?\PhpParser\Node\Stmt\Property
    {
        return null;
    }
    public function getMethods(): array
    {
        return [];
    }
    public function getMethod(string $name): ?\PhpParser\Node\Stmt\ClassMethod
    {
        return null;
    }
}
