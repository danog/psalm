<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr\AssignOp;

class Coalesce extends \PhpParser\Node\Expr\AssignOp
{
    public function getType(): string
    {
        return '';
    }
}
