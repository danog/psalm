<?php

declare(strict_types=1);

/**
 * Simplified SPL classes: enough for directory walking as used by php-parser's tests and Psalm.
 */

interface Traversable
{
}

/**
 * @template TKey
 * @template TValue
 */
interface Iterator extends Traversable
{
    /** @return TValue */
    public function current(): mixed;

    /** @return TKey */
    public function key(): mixed;

    public function next(): void;

    public function rewind(): void;

    public function valid(): bool;
}

/**
 * @template TKey
 * @template TValue
 */
interface IteratorAggregate extends Traversable
{
    /** @return Iterator<TKey, TValue> */
    public function getIterator(): Iterator;
}

/**
 * @template TKey
 * @template TValue
 */
interface RecursiveIterator extends Iterator
{
    public function hasChildren(): bool;

    /** @return RecursiveIterator<TKey, TValue>|null */
    public function getChildren(): ?RecursiveIterator;
}

class SplFileInfo implements Stringable
{
    public function __construct(protected string $pathname)
    {
    }

    public function getPathname(): string
    {
        return $this->pathname;
    }

    public function getFilename(): string
    {
        return basename($this->pathname);
    }

    public function getBasename(string $suffix = ''): string
    {
        return basename($this->pathname, $suffix);
    }

    public function getExtension(): string
    {
        $info = pathinfo($this->pathname);
        return $info['extension'] ?? '';
    }

    public function getPath(): string
    {
        return dirname($this->pathname);
    }

    public function getRealPath(): string|false
    {
        $r = realpath($this->pathname);
        return $r === false ? false : $r;
    }

    public function isDir(): bool
    {
        return is_dir($this->pathname);
    }

    public function isFile(): bool
    {
        return is_file($this->pathname);
    }

    public function isReadable(): bool
    {
        return is_readable($this->pathname);
    }

    public function getMTime(): int
    {
        return (int) filemtime($this->pathname);
    }

    public function getSize(): int
    {
        return (int) filesize($this->pathname);
    }

    public function __toString(): string
    {
        return $this->pathname;
    }
}

/**
 * Iterates the entries of one directory (non-recursively) as SplFileInfo objects.
 *
 * @implements Iterator<string, SplFileInfo>
 */
class FilesystemIterator implements Iterator
{
    public const CURRENT_AS_FILEINFO = 0;
    public const CURRENT_AS_SELF = 16;
    public const CURRENT_AS_PATHNAME = 32;
    public const KEY_AS_PATHNAME = 0;
    public const KEY_AS_FILENAME = 256;
    public const FOLLOW_SYMLINKS = 512;
    public const NEW_CURRENT_AND_KEY = 256;
    public const SKIP_DOTS = 4096;
    public const UNIX_PATHS = 8192;

    /** @var list<string> */
    protected array $entries = [];

    protected int $position = 0;

    public function __construct(protected string $directory, protected int $flags = 4096)
    {
        $this->directory = rtrim($directory, '/');
        $names = scandir($this->directory);
        foreach ($names as $name) {
            if ($name === '.' || $name === '..') {
                if ($this->flags & self::SKIP_DOTS) {
                    continue;
                }
            }
            $this->entries[] = $name;
        }
    }

    public function getPath(): string
    {
        return $this->directory;
    }

    public function current(): SplFileInfo
    {
        return new SplFileInfo($this->directory . '/' . $this->entries[$this->position]);
    }

    public function key(): string
    {
        return $this->directory . '/' . $this->entries[$this->position];
    }

    public function next(): void
    {
        $this->position++;
    }

    public function rewind(): void
    {
        $this->position = 0;
    }

    public function valid(): bool
    {
        return $this->position < count($this->entries);
    }
}

/**
 * @implements RecursiveIterator<string, SplFileInfo>
 */
class RecursiveDirectoryIterator extends FilesystemIterator implements RecursiveIterator
{
    public function __construct(string $directory, int $flags = 0)
    {
        parent::__construct($directory, $flags);
    }

    public function hasChildren(bool $allowLinks = false): bool
    {
        $name = $this->entries[$this->position];
        if ($name === '.' || $name === '..') {
            return false;
        }
        return is_dir($this->directory . '/' . $name);
    }

    public function getChildren(): RecursiveDirectoryIterator
    {
        return new RecursiveDirectoryIterator($this->directory . '/' . $this->entries[$this->position], $this->flags);
    }

    public function getSubPathname(): string
    {
        return $this->entries[$this->position];
    }
}

/**
 * Flattens a RecursiveIterator depth-first.
 *
 * @implements Iterator<string, SplFileInfo>
 */
class RecursiveIteratorIterator implements Iterator
{
    public const LEAVES_ONLY = 0;
    public const SELF_FIRST = 1;
    public const CHILD_FIRST = 2;
    public const CATCH_GET_CHILD = 16;

    /** @var list<array{string, SplFileInfo}> */
    private array $items = [];

    private int $position = 0;

    /** @param RecursiveIterator<string, SplFileInfo> $iterator */
    public function __construct(RecursiveIterator $iterator, private int $mode = self::LEAVES_ONLY, int $flags = 0)
    {
        $this->collect($iterator);
    }

    /** @param RecursiveIterator<string, SplFileInfo> $iterator */
    private function collect(RecursiveIterator $iterator): void
    {
        for ($iterator->rewind(); $iterator->valid(); $iterator->next()) {
            $key = $iterator->key();
            $value = $iterator->current();
            $name = basename($key);
            if ($name === '.' || $name === '..') {
                continue;
            }
            if ($iterator->hasChildren()) {
                if ($this->mode === self::SELF_FIRST) {
                    $this->items[] = [$key, $value];
                }
                $children = $iterator->getChildren();
                if ($children !== null) {
                    $this->collect($children);
                }
                if ($this->mode === self::CHILD_FIRST) {
                    $this->items[] = [$key, $value];
                }
            } else {
                $this->items[] = [$key, $value];
            }
        }
    }

    public function current(): SplFileInfo
    {
        return $this->items[$this->position][1];
    }

    public function key(): string
    {
        return $this->items[$this->position][0];
    }

    public function next(): void
    {
        $this->position++;
    }

    public function rewind(): void
    {
        $this->position = 0;
    }

    public function valid(): bool
    {
        return $this->position < count($this->items);
    }
}

/**
 * Filters an iterator's keys/values by a regular expression.
 *
 * @implements Iterator<string, SplFileInfo>
 */
class RegexIterator implements Iterator
{
    public const USE_KEY = 1;
    public const MATCH = 0;

    /** @var list<array{string, SplFileInfo}> */
    private array $items = [];

    private int $position = 0;

    /** @param Iterator<string, SplFileInfo> $iterator */
    public function __construct(Iterator $iterator, string $pattern, int $mode = self::MATCH, int $flags = 0)
    {
        for ($iterator->rewind(); $iterator->valid(); $iterator->next()) {
            $key = $iterator->key();
            $subject = $flags & self::USE_KEY ? $key : (string) $iterator->current();
            if (preg_match($pattern, $subject)) {
                $this->items[] = [$key, $iterator->current()];
            }
        }
    }

    public function current(): SplFileInfo
    {
        return $this->items[$this->position][1];
    }

    public function key(): string
    {
        return $this->items[$this->position][0];
    }

    public function next(): void
    {
        $this->position++;
    }

    public function rewind(): void
    {
        $this->position = 0;
    }

    public function valid(): bool
    {
        return $this->position < count($this->items);
    }
}
