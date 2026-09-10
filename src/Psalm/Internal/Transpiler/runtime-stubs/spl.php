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
        return basename($this->getPathname());
    }

    public function getBasename(string $suffix = ''): string
    {
        return basename($this->getPathname(), $suffix);
    }

    public function getExtension(): string
    {
        $info = pathinfo($this->getPathname());
        return $info['extension'] ?? '';
    }

    public function getPath(): string
    {
        return dirname($this->getPathname());
    }

    public function getRealPath(): string|false
    {
        $r = realpath($this->getPathname());
        return $r === false ? false : $r;
    }

    public function isDir(): bool
    {
        return is_dir($this->getPathname());
    }

    public function isFile(): bool
    {
        return is_file($this->getPathname());
    }

    public function isLink(): bool
    {
        return is_link($this->getPathname());
    }

    public function isReadable(): bool
    {
        return is_readable($this->getPathname());
    }

    public function getMTime(): int
    {
        return (int) filemtime($this->getPathname());
    }

    public function getSize(): int
    {
        return (int) filesize($this->getPathname());
    }

    public function __toString(): string
    {
        return $this->getPathname();
    }
}

/**
 * Iterates the entries of one directory; the iterator itself is positioned on the current entry.
 *
 * @implements Iterator<int|string, mixed>
 */
class DirectoryIterator extends SplFileInfo implements Iterator
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

    protected string $directory;

    protected int $flags;

    public function __construct(string $directory, int $flags = 16)
    {
        $this->directory = rtrim($directory, '/');
        $this->flags = $flags;
        parent::__construct($this->directory);
        $names = scandir($this->directory);
        if ($names !== false) {
            foreach ($names as $name) {
                if (($name === '.' || $name === '..') && ($this->flags & self::SKIP_DOTS)) {
                    continue;
                }
                $this->entries[] = $name;
            }
        }
    }

    public function getPathname(): string
    {
        if (!isset($this->entries[$this->position])) {
            return $this->directory;
        }
        return $this->directory . '/' . $this->entries[$this->position];
    }

    public function getFilename(): string
    {
        return $this->entries[$this->position] ?? '';
    }

    public function getPath(): string
    {
        return $this->directory;
    }

    public function isDot(): bool
    {
        $name = $this->entries[$this->position] ?? '';
        return $name === '.' || $name === '..';
    }

    /** @return static|SplFileInfo|string */
    public function current(): mixed
    {
        if ($this->flags & self::CURRENT_AS_PATHNAME) {
            return $this->getPathname();
        }
        if ($this->flags & self::CURRENT_AS_SELF) {
            return $this;
        }
        return new SplFileInfo($this->getPathname());
    }

    /** @return int|string */
    public function key(): mixed
    {
        if ($this->flags & self::KEY_AS_FILENAME) {
            return $this->getFilename();
        }
        return $this->position;
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

    public function seek(int $offset): void
    {
        $this->position = $offset;
    }
}

/**
 * @implements Iterator<string, mixed>
 */
class FilesystemIterator extends DirectoryIterator
{
    public function __construct(string $directory, int $flags = 4096)
    {
        parent::__construct($directory, $flags);
    }

    /** @return string */
    public function key(): mixed
    {
        if ($this->flags & self::KEY_AS_FILENAME) {
            return $this->getFilename();
        }
        return $this->getPathname();
    }

    public function getFlags(): int
    {
        return $this->flags;
    }

    public function setFlags(int $flags): void
    {
        $this->flags = $flags;
    }
}

/**
 * @implements RecursiveIterator<string, mixed>
 */
class RecursiveDirectoryIterator extends FilesystemIterator implements RecursiveIterator
{
    protected string $sub_path = '';

    public function __construct(string $directory, int $flags = 0)
    {
        parent::__construct($directory, $flags);
    }

    public function hasChildren(bool $allowLinks = false): bool
    {
        if ($this->isDot()) {
            return false;
        }
        return is_dir($this->getPathname());
    }

    public function getChildren(): RecursiveDirectoryIterator
    {
        $child = new RecursiveDirectoryIterator($this->getPathname(), $this->flags);
        $child->sub_path = $this->sub_path === '' ? $this->getFilename() : $this->sub_path . '/' . $this->getFilename();
        return $child;
    }

    public function getSubPath(): string
    {
        return $this->sub_path;
    }

    public function getSubPathname(): string
    {
        return $this->sub_path === '' ? $this->getFilename() : $this->sub_path . '/' . $this->getFilename();
    }
}

/**
 * Filters a RecursiveIterator with a callback; children are filtered with the same callback.
 *
 * @implements RecursiveIterator<mixed, mixed>
 */
class RecursiveCallbackFilterIterator implements RecursiveIterator
{
    /** @var callable(mixed, mixed, RecursiveIterator): bool */
    private $callback;

    /** @param callable(mixed, mixed, RecursiveIterator): bool $callback */
    public function __construct(private RecursiveIterator $iterator, callable $callback)
    {
        $this->callback = $callback;
    }

    public function getInnerIterator(): RecursiveIterator
    {
        return $this->iterator;
    }

    public function accept(): bool
    {
        return (bool) ($this->callback)($this->iterator->current(), $this->iterator->key(), $this->iterator);
    }

    private function skip(): void
    {
        while ($this->iterator->valid() && !$this->accept()) {
            $this->iterator->next();
        }
    }

    public function current(): mixed
    {
        return $this->iterator->current();
    }

    public function key(): mixed
    {
        return $this->iterator->key();
    }

    public function next(): void
    {
        $this->iterator->next();
        $this->skip();
    }

    public function rewind(): void
    {
        $this->iterator->rewind();
        $this->skip();
    }

    public function valid(): bool
    {
        return $this->iterator->valid();
    }

    public function hasChildren(): bool
    {
        return $this->iterator->hasChildren();
    }

    public function getChildren(): RecursiveCallbackFilterIterator
    {
        return new RecursiveCallbackFilterIterator($this->iterator->getChildren(), $this->callback);
    }
}

/**
 * Flattens a RecursiveIterator depth-first. Method calls not defined here are proxied by PHP to the
 * current sub-iterator; the stub models the common case (directory iteration) by being positioned on
 * the current entry like a RecursiveDirectoryIterator.
 *
 * @implements Iterator<mixed, mixed>
 */
class RecursiveIteratorIterator extends RecursiveDirectoryIterator
{
    public const LEAVES_ONLY = 0;
    public const SELF_FIRST = 1;
    public const CHILD_FIRST = 2;
    public const CATCH_GET_CHILD = 16;

    /** @var list<array{mixed, mixed, string}> key, value, pathname */
    private array $items = [];

    private int $mode;

    public function __construct(RecursiveIterator $iterator, int $mode = self::LEAVES_ONLY, int $flags = 0)
    {
        $this->mode = $mode;
        $this->directory = '';
        $this->flags = $flags;
        $this->collect($iterator);
    }

    private function collect(RecursiveIterator $iterator): void
    {
        for ($iterator->rewind(); $iterator->valid(); $iterator->next()) {
            $key = $iterator->key();
            $value = $iterator->current();
            $source = $iterator instanceof RecursiveCallbackFilterIterator ? $iterator->getInnerIterator() : $iterator;
            $pathname = $source instanceof SplFileInfo ? $source->getPathname() : (string) $key;
            $name = basename($pathname);
            if ($name === '.' || $name === '..') {
                continue;
            }
            if ($iterator->hasChildren()) {
                if ($this->mode === self::SELF_FIRST) {
                    $this->items[] = [$key, $value, $pathname];
                }
                $this->collect($iterator->getChildren());
                if ($this->mode === self::CHILD_FIRST) {
                    $this->items[] = [$key, $value, $pathname];
                }
            } else {
                $this->items[] = [$key, $value, $pathname];
            }
        }
    }

    public function getPathname(): string
    {
        return isset($this->items[$this->position]) ? $this->items[$this->position][2] : '';
    }

    public function getFilename(): string
    {
        return basename($this->getPathname());
    }

    public function getPath(): string
    {
        return dirname($this->getPathname());
    }

    public function getSubPathname(): string
    {
        return $this->getFilename();
    }

    public function current(): mixed
    {
        return $this->items[$this->position][1];
    }

    public function key(): mixed
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

    public function hasChildren(bool $allowLinks = false): bool
    {
        return false;
    }

    public function getDepth(): int
    {
        return 0;
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
