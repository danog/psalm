<?php

declare(strict_types=1);

namespace PhpParser\Node\Scalar;

class Int_ extends \PhpParser\Node\Scalar
{
    public const KIND_BIN = 2;
    public const KIND_OCT = 8;
    public const KIND_DEC = 10;
    public const KIND_HEX = 16;
    public int $value;
    public function __construct(int $value, array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
    public static function fromString(string $str, array $attributes = array (
), bool $allowInvalidOctal = false): \PhpParser\Node\Scalar\Int_
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getType(): string
    {
        return '';
    }
}
