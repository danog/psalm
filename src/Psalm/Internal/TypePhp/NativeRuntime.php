<?php

declare(strict_types=1);

namespace Psalm\Internal\TypePhp;

use ReflectionClass;
use ReflectionFunctionAbstract;

/**
 * In the native (TypePHP) build Psalm's own classes and functions are part of
 * the binary's extension module, so reflection reports them as internal.
 * They must not be mistaken for PHP's predefined symbols.
 *
 * @internal
 */
final class NativeRuntime
{
    private static ?string $extension_name = null;

    /**
     * The name of the extension module the compiled Psalm belongs to, or '' under plain PHP.
     *
     * @psalm-external-mutation-free
     */
    public static function extensionName(): string
    {
        if (self::$extension_name === null) {
            $name = (new ReflectionClass(self::class))->getExtensionName();
            self::$extension_name = $name === false ? '' : $name;
        }

        return self::$extension_name;
    }

    /**
     * Whether the reflected symbol is compiled into the Psalm binary (and so
     * not a real predefined PHP symbol).
     *
     * @psalm-external-mutation-free
     */
    public static function isCompiledIn(ReflectionClass|ReflectionFunctionAbstract $reflection): bool
    {
        $extension_name = self::extensionName();

        return $extension_name !== '' && $reflection->getExtensionName() === $extension_name;
    }
}
