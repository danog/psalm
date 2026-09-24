<?php

declare(strict_types=1);

namespace Psalm\Type\Atomic;

/**
 * The memoized getKey() / getId(true) / getId(false) strings of an Atomic, allocated on the first memoized call:
 * one optional handle on the atomic instead of three string slots (most atomics are copied and compared far more
 * often than their strings are asked for).
 *
 * @internal
 */
final class IdMemo
{
    public ?string $key = null;
    public ?string $id = null;
    public ?string $inexact_id = null;
}
