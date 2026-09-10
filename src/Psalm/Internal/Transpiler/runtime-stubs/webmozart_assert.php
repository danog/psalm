<?php

declare(strict_types=1);

namespace Webmozart\Assert;

use InvalidArgumentException;

/**
 * Runtime stub of webmozart/assert: the assertions Psalm uses, throwing InvalidArgumentException.
 */
final class Assert
{
    /** @psalm-assert !false $value */
    public static function notFalse(mixed $value, string $message = ''): void
    {
        if ($value === false) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a value other than false.');
        }
    }

    /** @psalm-assert !null $value */
    public static function notNull(mixed $value, string $message = ''): void
    {
        if ($value === null) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a value other than null.');
        }
    }

    /** @psalm-assert null $value */
    public static function null(mixed $value, string $message = ''): void
    {
        if ($value !== null) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected null.');
        }
    }

    public static function eq(mixed $value, mixed $expect, string $message = ''): void
    {
        if ($value != $expect) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a value equal to the expected one.');
        }
    }

    public static function same(mixed $value, mixed $expect, string $message = ''): void
    {
        if ($value !== $expect) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a value identical to the expected one.');
        }
    }

    /** @psalm-assert string $value */
    public static function string(mixed $value, string $message = ''): void
    {
        if (!is_string($value)) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a string.');
        }
    }

    /** @psalm-assert int $value */
    public static function integer(mixed $value, string $message = ''): void
    {
        if (!is_int($value)) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected an integer.');
        }
    }

    /** @psalm-assert bool $value */
    public static function boolean(mixed $value, string $message = ''): void
    {
        if (!is_bool($value)) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a boolean.');
        }
    }

    /** @psalm-assert array $value */
    public static function isArray(mixed $value, string $message = ''): void
    {
        if (!is_array($value)) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected an array.');
        }
    }

    /** @psalm-assert non-empty-string $value */
    public static function stringNotEmpty(mixed $value, string $message = ''): void
    {
        if (!is_string($value) || $value === '') {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a non-empty string.');
        }
    }

    public static function keyExists(mixed $array, string|int $key, string $message = ''): void
    {
        if (!is_array($array) || !array_key_exists($key, $array)) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected the key ' . $key . ' to exist.');
        }
    }

    public static function true(mixed $value, string $message = ''): void
    {
        if ($value !== true) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a value to be true.');
        }
    }

    public static function false(mixed $value, string $message = ''): void
    {
        if ($value !== false) {
            throw new InvalidArgumentException($message !== '' ? $message : 'Expected a value to be false.');
        }
    }
}
