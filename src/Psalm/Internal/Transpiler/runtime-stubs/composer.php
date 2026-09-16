<?php

/**
 * Composer's autoloader as seen by the compiled program: nothing is autoloaded at runtime (every class is
 * compiled in), so lookups report no file and registration is a no-op. Typed replacement for the vendor
 * class Psalm's config/plugin loading talks to.
 */

namespace Composer\Autoload;

class ClassLoader
{
    /** @var array<string, list<string>> */
    private array $prefixesPsr4 = [];

    /** @param string|list<string> $paths */
    public function addPsr4(string $prefix, string|array $paths, bool $prepend = false): void
    {
        $list = is_string($paths) ? [$paths] : $paths;
        $this->prefixesPsr4[$prefix] = array_merge($this->prefixesPsr4[$prefix] ?? [], $list);
    }

    /** @param string|list<string> $paths */
    public function add(string $prefix, string|array $paths, bool $prepend = false): void
    {
        $this->addPsr4($prefix, $paths, $prepend);
    }

    /** @return array<string, list<string>> */
    public function getPrefixesPsr4(): array
    {
        return $this->prefixesPsr4;
    }

    /** @return string|false */
    public function findFile(string $class): string|false
    {
        return false;
    }

    public function register(bool $prepend = false): void
    {
    }

    public function unregister(): void
    {
    }
}
