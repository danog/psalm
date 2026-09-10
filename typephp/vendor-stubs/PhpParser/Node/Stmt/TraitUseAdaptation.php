<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

abstract class TraitUseAdaptation extends \PhpParser\Node\Stmt
{
    public ?\PhpParser\Node\Name $trait = null;
    public \PhpParser\Node\Identifier $method;
}
