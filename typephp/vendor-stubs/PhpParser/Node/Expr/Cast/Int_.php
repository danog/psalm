<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr\Cast;

class Int_ extends \PhpParser\Node\Expr\Cast
{
    public const KIND_INT = 1;
    public const KIND_INTEGER = 2;
    public function getType(): string
    {
        return '';
    }
}
