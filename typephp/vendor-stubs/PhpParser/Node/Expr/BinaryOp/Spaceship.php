<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr\BinaryOp;

class Spaceship extends \PhpParser\Node\Expr\BinaryOp
{
    public function getOperatorSigil(): string
    {
        return '';
    }
    public function getType(): string
    {
        return '';
    }
}
