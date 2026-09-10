<?php

declare(strict_types=1);

namespace PhpParser\Node;

class UseItem extends \PhpParser\NodeAbstract
{
    public int $type;
    public \PhpParser\Node\Name $name;
    public ?\PhpParser\Node\Identifier $alias = null;
    public function __construct(\PhpParser\Node\Name $name, $alias = NULL, int $type = \PhpParser\Node\Stmt\Use_::TYPE_UNKNOWN, array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
    public function getAlias(): \PhpParser\Node\Identifier
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getType(): string
    {
        return '';
    }
}
