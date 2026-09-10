<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt\TraitUseAdaptation;

class Alias extends \PhpParser\Node\Stmt\TraitUseAdaptation
{
    public ?int $newModifier = null;
    public ?\PhpParser\Node\Identifier $newName = null;
    public function __construct(?\PhpParser\Node\Name $trait, $method, ?int $newModifier, $newName, array $attributes = array (
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
