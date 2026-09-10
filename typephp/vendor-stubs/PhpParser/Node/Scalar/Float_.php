<?php

declare(strict_types=1);

namespace PhpParser\Node\Scalar;

class Float_ extends \PhpParser\Node\Scalar
{
    public float $value;
    public function __construct(float $value, array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
    public static function fromString(string $str, array $attributes = array (
)): \PhpParser\Node\Scalar\Float_
    {
        throw new \RuntimeException('vendor stub');
    }
    public static function parse(string $str): float
    {
        return 0.0;
    }
    public function getType(): string
    {
        return '';
    }
}
