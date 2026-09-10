<?php

declare(strict_types=1);

namespace Psalm\Internal;

/**
 * The only place where Psalm loads PHP code at runtime (plugins, project
 * autoloaders, phar metadata).
 *
 * Everything Psalm itself needs is compiled into the native build; plugins and
 * stubs of the analysed project are loaded at runtime here, in both builds.
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
