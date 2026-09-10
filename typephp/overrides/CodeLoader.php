<?php

declare(strict_types=1);

namespace Psalm\Internal;

use RuntimeException;

/**
 * Closed-world replacement of Psalm\Internal\CodeLoader for the native build:
 * no PHP code can be loaded at runtime.
 *
 * @internal
 */
final class CodeLoader
{
    /**
     * @psalm-pure
     */
    public static function canLoadCode(): bool
    {
        return false;
    }

    /**
     * @psalm-pure
     */
    public static function requireFile(string $path): mixed
    {
        throw new RuntimeException('The native Psalm binary cannot load PHP code at runtime: ' . $path);
    }

    /**
     * @psalm-pure
     */
    public static function requireFileOnce(string $path): mixed
    {
        throw new RuntimeException('The native Psalm binary cannot load PHP code at runtime: ' . $path);
    }
}
