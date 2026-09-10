<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class Use_ extends \PhpParser\Node\Stmt
{
    public const TYPE_UNKNOWN = 0;
    public const TYPE_NORMAL = 1;
    public const TYPE_FUNCTION = 2;
    public const TYPE_CONSTANT = 3;
    public int $type;
    public array $uses;
    public function __construct(array $uses, int $type = self::TYPE_NORMAL, array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
    public function getType(): string
    {
        return '';
    }
}
