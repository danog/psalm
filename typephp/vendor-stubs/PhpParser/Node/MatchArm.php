<?php

declare(strict_types=1);

namespace PhpParser\Node;

class MatchArm extends \PhpParser\NodeAbstract
{
    public ?array $conds = null;
    public \PhpParser\Node\Expr $body;
    public function __construct(?array $conds, \PhpParser\Node\Expr $body, array $attributes = array (
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
