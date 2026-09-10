<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr;

abstract class CallLike extends \PhpParser\Node\Expr
{
    public abstract function getRawArgs(): array;
    public function isFirstClassCallable(): bool
    {
        return false;
    }
    public function getArgs(): array
    {
        return [];
    }
    public function getArg(string $name, int $position): ?\PhpParser\Node\Arg
    {
        return null;
    }
}
