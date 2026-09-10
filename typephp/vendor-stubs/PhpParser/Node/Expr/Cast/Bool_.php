<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr\Cast;

class Bool_ extends \PhpParser\Node\Expr\Cast
{
    public const KIND_BOOL = 1;
    public const KIND_BOOLEAN = 2;
    public function getType(): string
    {
        return '';
    }
}
