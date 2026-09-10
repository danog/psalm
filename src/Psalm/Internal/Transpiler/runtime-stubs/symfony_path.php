<?php

declare(strict_types=1);

namespace Symfony\Component\Filesystem;

/**
 * Runtime stub of symfony/filesystem's Path helper (the subset Psalm uses).
 */
final class Path
{
    public static function isAbsolute(string $path): bool
    {
        if ($path === '') {
            return false;
        }
        $path = self::stripScheme($path);
        if ($path[0] === '/' || $path[0] === '\\') {
            return true;
        }
        // Windows drive letter
        return strlen($path) > 1 && ctype_alpha($path[0]) && $path[1] === ':';
    }

    public static function isRelative(string $path): bool
    {
        return !self::isAbsolute($path);
    }

    public static function canonicalize(string $path): string
    {
        if ($path === '') {
            return '';
        }
        if (str_starts_with($path, '~')) {
            $home = getenv('HOME');
            $path = ($home === false ? '' : $home) . substr($path, 1);
        }
        $path = str_replace('\\', '/', $path);
        $scheme = '';
        $pos = strpos($path, '://');
        if ($pos !== false) {
            $scheme = substr($path, 0, $pos + 3);
            $path = substr($path, $pos + 3);
        }
        $root = '';
        if ($path !== '' && $path[0] === '/') {
            $root = '/';
            $path = substr($path, 1);
        } elseif (strlen($path) > 1 && ctype_alpha($path[0]) && $path[1] === ':') {
            $root = strtoupper($path[0]) . ':/';
            $path = ltrim(substr($path, 2), '/');
        }
        $parts = [];
        foreach (explode('/', $path) as $part) {
            if ($part === '.' || $part === '') {
                continue;
            }
            if ($part === '..' && $parts !== [] && $parts[count($parts) - 1] !== '..') {
                array_pop($parts);
                continue;
            }
            if ($part === '..' && $root !== '') {
                continue;
            }
            $parts[] = $part;
        }
        return $scheme . $root . implode('/', $parts);
    }

    public static function normalize(string $path): string
    {
        return str_replace('\\', '/', $path);
    }

    public static function getDirectory(string $path): string
    {
        if ($path === '') {
            return '';
        }
        $path = self::canonicalize($path);
        $pos = strrpos($path, '/');
        if ($pos === false) {
            return '';
        }
        if ($pos === 0) {
            return '/';
        }
        return substr($path, 0, $pos);
    }

    public static function join(string ...$paths): string
    {
        $out = '';
        foreach ($paths as $p) {
            if ($p === '') {
                continue;
            }
            if ($out === '') {
                $out = $p;
            } else {
                $out = rtrim($out, '/\\') . '/' . ltrim($p, '/\\');
            }
        }
        return self::canonicalize($out);
    }

    public static function makeAbsolute(string $path, string $basePath): string
    {
        if (self::isAbsolute($path)) {
            return self::canonicalize($path);
        }
        return self::canonicalize(rtrim($basePath, '/\\') . '/' . $path);
    }

    private static function stripScheme(string $path): string
    {
        $pos = strpos($path, '://');
        return $pos === false ? $path : substr($path, $pos + 3);
    }
}
