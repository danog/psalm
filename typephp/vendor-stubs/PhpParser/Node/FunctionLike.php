<?php

declare(strict_types=1);

namespace PhpParser\Node;

interface FunctionLike extends \PhpParser\Node
{
    public function returnsByRef(): bool;
    public function getParams(): array;
    public function getReturnType();
    public function getStmts(): ?array;
    public function getAttrGroups(): array;
}
