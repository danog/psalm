<?php

declare(strict_types=1);

namespace PhpParser\Node\Expr\Cast;

class Array_ extends \PhpParser\Node\Expr\Cast
{
    public function getType(): string
    {
        return '';
    }
}
