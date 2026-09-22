<?php

declare(strict_types=1);

namespace Psalm\Internal;

use Psalm\Exception\ConfigException;
use Psalm\Plugin\HookInterface;
use Psalm\Plugin\PluginInterface;

use function ltrim;

/**
 * The compiled program's PluginInstantiator. Nothing can be loaded at runtime, so a plugin is
 * built through the table the transpiler generates of every plugin class compiled into the
 * program (`names::instantiate_plugin`), keyed by class name. A file-based plugin's file is
 * compiled in like any other, so the path a config names is not needed.
 *
 * @internal
 */
final class PluginInstantiator
{
    /**
     * @param string|null $path the file a `<plugin filename="...">` entry names (unused: compiled in)
     */
    public static function instantiate(string $class, ?string $path = null): PluginInterface|HookInterface
    {
        $class = ltrim($class, '\\');
        $plugin = __rt_instantiate_plugin($class);

        if ($plugin === null) {
            throw new ConfigException(
                'Cannot instantiate plugin class ' . $class . ': it is not compiled into this build',
            );
        }

        return $plugin;
    }
}
