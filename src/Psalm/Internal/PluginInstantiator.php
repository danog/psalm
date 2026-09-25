<?php

declare(strict_types=1);

namespace Psalm\Internal;

use Psalm\Exception\ConfigException;
use Psalm\Plugin\HookInterface;
use Psalm\Plugin\PluginInterface;

use function class_exists;
use function ltrim;

/**
 * Builds a plugin from its class name.
 *
 * The interpreted analyzer autoloads the class like any other class, or, for a `<plugin filename>`
 * entry, loads the file the config names. The compiled analyzer cannot load code at runtime; its
 * copy of this class (a transpiler runtime stub) answers from a table the transpiler generates of
 * every plugin class compiled into the program, so this file is left out of the transpilation.
 *
 * @internal
 */
final class PluginInstantiator
{
    /**
     * @param string|null $path the file a `<plugin filename="...">` entry names, loaded when the
     *                          class is not yet known
     */
    public static function instantiate(string $class, ?string $path = null): PluginInterface|HookInterface
    {
        $class = ltrim($class, '\\');

        if (!class_exists($class) && $path !== null) {
            /** @psalm-suppress UnresolvableInclude */
            require_once $path;
        }

        if (!class_exists($class)) {
            throw new ConfigException('Cannot instantiate plugin class ' . $class . ': the class does not exist');
        }

        $plugin = new $class();

        if (!$plugin instanceof PluginInterface && !$plugin instanceof HookInterface) {
            throw new ConfigException(
                $class . ' is not a plugin: it implements neither PluginInterface nor HookInterface',
            );
        }

        return $plugin;
    }
}
