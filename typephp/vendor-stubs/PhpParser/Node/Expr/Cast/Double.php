<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr\Cast;

class Double extends \PhpParser\Node\Expr\Cast
{
    public const KIND_DOUBLE = 1;
    public const KIND_FLOAT = 2;
    public const KIND_REAL = 3;
    public function getType(): string
    {
        return '';
    }
}
