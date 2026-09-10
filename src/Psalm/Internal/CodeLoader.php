<?php

declare(strict_types=1);

namespace Psalm\Internal;

/**
 * The only place where Psalm loads PHP code at runtime (plugins, project
 * autoloaders, phar metadata).
 *
 * The closed-world native build replaces this class with one that cannot load
 * code, see typephp/overrides/CodeLoader.php.
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
        return true;
    }

    /**
     * @psalm-suppress UnresolvableInclude
     * @psalm-pure
     */
    public static function requireFile(string $path): mixed
    {
        return require $path;
    }

    /**
     * @psalm-suppress UnresolvableInclude
     * @psalm-pure
     */
    public static function requireFileOnce(string $path): mixed
    {
        return require_once $path;
    }
}
