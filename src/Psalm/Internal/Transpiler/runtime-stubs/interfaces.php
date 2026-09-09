<?php

declare(strict_types=1);

interface JsonSerializable
{
    public function jsonSerialize(): mixed;
}

interface Countable
{
    public function count(): int;
}

/**
 * @template TKey
 * @template TValue
 */
interface ArrayAccess
{
    /** @param TKey $offset */
    public function offsetExists(mixed $offset): bool;

    /**
     * @param TKey $offset
     * @return TValue
     */
    public function offsetGet(mixed $offset): mixed;

    /**
     * @param TKey|null $offset
     * @param TValue $value
     */
    public function offsetSet(mixed $offset, mixed $value): void;

    /** @param TKey $offset */
    public function offsetUnset(mixed $offset): void;
}

interface UnitEnum
{
    /** @return list<static> */
    public static function cases(): array;
}

interface BackedEnum extends UnitEnum
{
    public static function from(int|string $value): static;

    public static function tryFrom(int|string $value): ?static;
}
