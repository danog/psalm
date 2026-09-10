<?php

declare(strict_types=1);

namespace XdgBaseDir;

/**
 * Runtime stub of dnoegel/php-xdg-base-dir: XDG base directories from the environment.
 */
class Xdg
{
    public const S_IFDIR = 040000;
    public const S_IRWXO = 00007;
    public const S_IRWXG = 00056;
    public const RUNTIME_DIR_FALLBACK = 'php-xdg-runtime-dir-fallback-';

    public function getHomeDir(): string
    {
        $home = getenv('HOME');
        return $home === false ? '' : $home;
    }

    public function getHomeConfigDir(): string
    {
        $path = getenv('XDG_CONFIG_HOME');
        return $path === false || $path === '' ? $this->getHomeDir() . DIRECTORY_SEPARATOR . '.config' : $path;
    }

    public function getHomeDataDir(): string
    {
        $path = getenv('XDG_DATA_HOME');
        return $path === false || $path === '' ? $this->getHomeDir() . DIRECTORY_SEPARATOR . '.local' . DIRECTORY_SEPARATOR . 'share' : $path;
    }

    public function getHomeCacheDir(): string
    {
        $path = getenv('XDG_CACHE_HOME');
        return $path === false || $path === '' ? $this->getHomeDir() . DIRECTORY_SEPARATOR . '.cache' : $path;
    }

    /** @return list<string> */
    public function getConfigDirs(): array
    {
        $dirs = getenv('XDG_CONFIG_DIRS');
        $list = $dirs === false || $dirs === '' ? ['/etc/xdg'] : explode(':', $dirs);
        return [$this->getHomeConfigDir(), ...$list];
    }

    /** @return list<string> */
    public function getDataDirs(): array
    {
        $dirs = getenv('XDG_DATA_DIRS');
        $list = $dirs === false || $dirs === '' ? ['/usr/local/share', '/usr/share'] : explode(':', $dirs);
        return [$this->getHomeDataDir(), ...$list];
    }

    public function getRuntimeDir(bool $strict = true): string
    {
        $path = getenv('XDG_RUNTIME_DIR');
        if ($path !== false && $path !== '') {
            return $path;
        }
        if ($strict) {
            throw new \RuntimeException('XDG_RUNTIME_DIR was not set');
        }
        return sys_get_temp_dir() . DIRECTORY_SEPARATOR . self::RUNTIME_DIR_FALLBACK . 'user';
    }
}
