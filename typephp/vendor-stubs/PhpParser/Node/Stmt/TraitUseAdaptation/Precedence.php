<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt\TraitUseAdaptation;

class Precedence extends \PhpParser\Node\Stmt\TraitUseAdaptation
{
    public array $insteadof;
    public function __construct(\PhpParser\Node\Name $trait, $method, array $insteadof, array $attributes = array (
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
