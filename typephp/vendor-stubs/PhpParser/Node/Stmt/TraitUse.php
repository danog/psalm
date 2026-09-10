<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class TraitUse extends \PhpParser\Node\Stmt
{
    public array $traits;
    public array $adaptations;
    public function __construct(array $traits, array $adaptations = array (
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
