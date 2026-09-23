<?php

declare(strict_types=1);

namespace Psalm\Internal;

use UnexpectedValueException;

use function hash;
use function unpack;

use const PHP_INT_MAX;

/**
 * Names as ints (pzoom's `StrId` / `Interner`): class-like, function, method, property and template names are
 * interned once and carried as ints in types, storages and identifiers, so every lookup and comparison is an
 * int operation and no lowercased copy is ever built (resolution is case-sensitive, as in pzoom; the
 * case-insensitive fallbacks used for casing hints keep their own lowercase maps).
 *
 * pzoom hands out sequential ids from one shared table. Psalm's workers are forked processes, so an id must
 * mean the same thing in every process without a shared table: it is a 63-bit hash of the string. Only the
 * id -> string table is per process; a worker exports the strings it interned after forking (delta()) and the
 * parent merges them (merge()), and the cache stores the table with the storages.
 *
 * @internal
 */
final class Interner
{
    /** @var array<string, int> */
    private static array $ids = [];

    /** @var array<int, string> */
    private static array $strings = [];

    /** @var list<string> the strings interned in this process since mark() */
    private static array $since_mark = [];

    private static bool $marked = false;

    /**
     * @psalm-external-mutation-free
     */
    public static function intern(string $string): int
    {
        return self::$ids[$string] ?? self::add($string);
    }

    /**
     * @psalm-external-mutation-free
     */
    private static function add(string $string): int
    {
        $id = self::hash($string);
        $existing = self::$strings[$id] ?? null;
        if ($existing !== null && $existing !== $string) {
            throw new UnexpectedValueException('Interned string hash collision: ' . $existing . ' / ' . $string);
        }
        self::$strings[$id] = $string;
        self::$ids[$string] = $id;
        if (self::$marked) {
            self::$since_mark[] = $string;
        }
        return $id;
    }

    /**
     * The id a string interns to, without interning it (the same value in every process).
     *
     * @psalm-pure
     */
    public static function hash(string $string): int
    {
        /** @var array{1: int} $unpacked */
        $unpacked = unpack('q', hash('xxh3', $string, true));
        return $unpacked[1] & PHP_INT_MAX;
    }

    /**
     * @psalm-external-mutation-free
     */
    public static function lookup(int $id): string
    {
        if (!isset(self::$strings[$id])) {
            // the preloaded names (Sym constants) resolve before any Codebase exists
            self::merge(Sym::PRELOADED);
        }
        return self::$strings[$id]
            ?? throw new UnexpectedValueException('Unknown interned id ' . $id);
    }

    /**
     * A forked worker calls this first: delta() then lists what it interned on its own.
     *
     * @psalm-external-mutation-free
     */
    public static function mark(): void
    {
        self::$marked = true;
        self::$since_mark = [];
    }

    /**
     * The strings interned since mark(), for the parent to merge().
     *
     * @return list<string>
     * @psalm-external-mutation-free
     */
    public static function delta(): array
    {
        return self::$since_mark;
    }

    /**
     * @param iterable<string> $strings
     * @psalm-external-mutation-free
     * @psalm-suppress ImpureMethodCall iterating the input
     */
    public static function merge(iterable $strings): void
    {
        foreach ($strings as $string) {
            self::intern($string);
        }
    }
}
