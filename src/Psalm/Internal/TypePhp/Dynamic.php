<?php

declare(strict_types=1);

namespace Psalm\Internal\TypePhp;

/**
 * Helpers for code that must also compile with the TypePHP ahead-of-time compiler.
 *
 * TypePHP fixes the native type of a local variable at its first assignment, so a
 * local first assigned an object cannot later hold a sibling class, and cannot be
 * passed by reference. Routing the first assignment through {@see Dynamic::any()}
 * keeps the variable dynamic for the compiler while Psalm still sees the exact type.
 *
 * @internal
 * @psalm-immutable
 */
final class Dynamic
{
    /**
     * Identity function: returns its argument unchanged.
     *
     * @template T
     * @param T $value
     * @return T
     * @psalm-pure
     */
    public static function any(mixed $value): mixed
    {
        return $value;
    }
}
