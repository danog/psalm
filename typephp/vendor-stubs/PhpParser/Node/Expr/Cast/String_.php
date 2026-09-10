<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr\Cast;

class String_ extends \PhpParser\Node\Expr\Cast
{
    public const KIND_STRING = 1;
    public const KIND_BINARY = 2;
    public function getType(): string
    {
        return '';
    }
}
