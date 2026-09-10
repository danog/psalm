<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Nop extends \PhpParser\Node\Stmt
{
    public function getSubNodeNames(): array
    {
        return [];
    }
    public function getType(): string
    {
        return '';
    }
}
