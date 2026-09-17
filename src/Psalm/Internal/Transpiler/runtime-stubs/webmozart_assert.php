<?php

declare(strict_types=1);

namespace Webmozart\Assert;

use InvalidArgumentException;

/**
 * Runtime stub of webmozart/assert: the assertions Psalm uses, throwing InvalidArgumentException.
 */
final class Assert
{
    /**
     * @template T
     * @param T $value
     * @psalm-assert !false $value
     */
    public static function notFalse($value, string $message = ''): void
    {
        if ($value === false) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a value other than false.');
        }
    }

    /**
     * @template T
     * @param T $value
     * @psalm-assert !null $value
     */
    public static function notNull($value, string $message = ''): void
    {
        if ($value === null) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a value other than null.');
        }
    }

    /**
     * @template T
     * @param T $value
     * @psalm-assert null $value
     */
    public static function null($value, string $message = ''): void
    {
        if ($value !== null) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected null.');
        }
    }

    /**
     * @template T
     * @param T $value
     * @param T $expect
     */
    public static function eq($value, $expect, string $message = ''): void
    {
        if ($value != $expect) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a value equal to the expected one.');
        }
    }

    /**
     * @template T
     * @param T $value
     * @param T $expect
     */
    public static function same($value, $expect, string $message = ''): void
    {
        if ($value !== $expect) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a value identical to the expected one.');
        }
    }

    /**
     * @template T
     * @param T $value
     * @psalm-assert string $value
     */
    public static function string($value, string $message = ''): void
    {
        if (!is_string($value)) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a string.');
        }
    }

    /**
     * @template T
     * @param T $value
     * @psalm-assert int $value
     */
    public static function integer($value, string $message = ''): void
    {
        if (!is_int($value)) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected an integer.');
        }
    }

    /**
     * @template T
     * @param T $value
     * @psalm-assert bool $value
     */
    public static function boolean($value, string $message = ''): void
    {
        if (!is_bool($value)) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a boolean.');
        }
    }

    /**
     * @template T
     * @param T $value
     * @psalm-assert array $value
     */
    public static function isArray($value, string $message = ''): void
    {
        if (!is_array($value)) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected an array.');
        }
    }

    /**
     * @template T
     * @param T $value
     * @psalm-assert non-empty-string $value
     */
    public static function stringNotEmpty($value, string $message = ''): void
    {
        if (!is_string($value) || $value === '') {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a non-empty string.');
        }
    }

    /**
     * @template TKey of array-key
     * @template TValue
     * @param array<TKey, TValue> $array
     */
    public static function keyExists(array $array, string|int $key, string $message = ''): void
    {
        if (!array_key_exists($key, $array)) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected the key ' . $key . ' to exist.');
        }
    }

    /**
     * @template T
     * @param T $value
     */
    public static function true($value, string $message = ''): void
    {
        if ($value !== true) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a value to be true.');
        }
    }

    /**
     * @template T
     * @param T $value
     */
    public static function false($value, string $message = ''): void
    {
        if ($value !== false) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a value to be false.');
        }
    }
}
