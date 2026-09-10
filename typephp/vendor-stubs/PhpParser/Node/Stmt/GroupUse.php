<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class GroupUse extends \PhpParser\Node\Stmt
{
    public int $type;
    public \PhpParser\Node\Name $prefix;
    public array $uses;
    public function __construct(\PhpParser\Node\Name $prefix, array $uses, int $type = \PhpParser\Node\Stmt\Use_::TYPE_NORMAL, array $attributes = array (
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
