<?php

declare(strict_types=1);

namespace Psalm\Internal;

use UnexpectedValueException;

use function array_map;
use function count;
use function file_get_contents;
use function file_put_contents;
use function getmypid;
use function hash;
use function is_array;
use function str_starts_with;
use function substr;
use function is_file;
use function register_shutdown_function;
use function rename;
use function serialize;
use function unpack;
use function unserialize;

use const PHP_INT_MAX;

/**
 * Names as ints (pzoom's `StrId` / `Interner`): class-like, function, method, property and constant names are
 * interned once and carried as ints in types, storages and identifiers, so every lookup and comparison is an
 * int operation.
 *
 * An id must mean the same thing in every process without a shared table (Psalm's workers are forked processes,
 * and the cache outlives a run): it is a 63-bit hash of the string (the empty string is 0, so a name's truthiness
 * is its id's). Only the id -> string table is per process: a worker exports the strings it interned after
 * forking (delta()) and the parent merges them (merge()); with a cache directory the table is saved beside the
 * cached storages and read back when an id from an earlier run is looked up.
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

    private static ?string $persist_file = null;

    private static int $persisted_count = 0;

    /**
     * @psalm-pure
     * @psalm-suppress ImpureStaticProperty, ImpureMethodCall the table only grows; an id never changes meaning
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
        if ($string === '') {
            return 0;
        }
        /** @var array{1: int} $unpacked */
        $unpacked = unpack('q', hash('xxh3', $string, true));
        return ($unpacked[1] & PHP_INT_MAX) ?: 1;
    }

    /**
     * @psalm-pure
     * @psalm-suppress ImpureStaticProperty, ImpureMethodCall the table only grows; an id never changes meaning
     */
    public static function lookup(int $id): string
    {
        return self::$strings[$id] ?? self::lookupMissing($id);
    }

    /**
     * The string of an id interned from a lowercase name (member names, lowercased function ids).
     *
     * @return lowercase-string
     * @psalm-suppress LessSpecificReturnStatement, MoreSpecificReturnType the id was interned from a lowercase string
     * @psalm-pure
     */
    public static function lookupLc(int $id): string
    {
        return self::lookup($id);
    }

    /**
     * An array keyed by the strings of its ids of lowercase names.
     *
     * @template V
     * @param array<int, V> $by_id
     * @return array<lowercase-string, V>
     * @psalm-suppress LessSpecificReturnStatement, MoreSpecificReturnType the ids were interned from lowercase strings
     * @psalm-pure
     */
    public static function lookupLcKeys(array $by_id): array
    {
        return self::lookupKeys($by_id);
    }

    /**
     * The name of an id interned from a class-like name.
     *
     * @return class-string
     * @psalm-suppress LessSpecificReturnStatement, MoreSpecificReturnType the id was interned from a class name
     * @psalm-pure
     */
    public static function lookupClass(int $id): string
    {
        return self::lookup($id);
    }

    /**
     * The name of an id interned from a function name.
     *
     * @return callable-string
     * @psalm-suppress LessSpecificReturnStatement, MoreSpecificReturnType the id was interned from a function name
     * @psalm-pure
     */
    public static function lookupCallable(int $id): string
    {
        return self::lookup($id);
    }

    /**
     * The name of an id of a non-empty name.
     *
     * @return non-empty-string
     * @psalm-suppress LessSpecificReturnStatement, MoreSpecificReturnType the id was interned from a non-empty name
     * @psalm-pure
     */
    public static function lookupNonEmpty(int $id): string
    {
        return self::lookup($id);
    }

    /**
     * The string an optional id stands for.
     *
     * @psalm-pure
     * @return ($id is null ? null : string)
     */
    public static function lookupOrNull(?int $id): ?string
    {
        return $id === null ? null : self::lookup($id);
    }

    /**
     * The id of an optional string.
     *
     * @psalm-pure
     * @return ($string is null ? null : int)
     */
    public static function internOrNull(?string $string): ?int
    {
        return $string === null ? null : self::intern($string);
    }

    /**
     * The ids of an array's string values (null stays null), keys kept: where a foreign array of names enters.
     *
     * @template K of array-key
     * @param array<K, string> $strings
     * @return array<K, int>
     * @psalm-pure
     */
    public static function internList(array $strings): array
    {
        return array_map(self::intern(...), $strings);
    }

    /**
     * An array keyed by the ids of its string keys.
     *
     * @template V
     * @param array<array-key, V> $by_string
     * @return array<int, V>
     * @psalm-pure
     */
    public static function internKeys(array $by_string): array
    {
        $out = [];
        foreach ($by_string as $k => $v) {
            $out[self::intern((string) $k)] = $v;
        }
        return $out;
    }

    /**
     * The strings of an array's id values (null stays null), keys kept: where names leave as strings.
     *
     * @template K of array-key
     * @param array<K, int> $ids
     * @return array<K, string>
     * @psalm-pure
     */
    public static function lookupList(array $ids): array
    {
        return array_map(self::lookup(...), $ids);
    }

    /**
     * An array keyed by the strings of its id keys.
     *
     * @template V
     * @param array<int, V> $by_id
     * @return array<string, V>
     * @psalm-pure
     */
    public static function lookupKeys(array $by_id): array
    {
        $out = [];
        foreach ($by_id as $k => $v) {
            $out[self::lookup($k)] = $v;
        }
        return $out;
    }

    /**
     * Each inner array keyed by the strings of its id keys.
     *
     * @template K of array-key
     * @template V
     * @param array<K, array<int, V>> $a
     * @return array<K, array<string, V>>
     * @psalm-pure
     */
    public static function lookupKeysEach(array $a): array
    {
        return array_map(self::lookupKeys(...), $a);
    }

    /**
     * Each inner array keyed by the ids of its string keys.
     *
     * @template K of array-key
     * @template V
     * @param array<K, array<array-key, V>> $a
     * @return array<K, array<int, V>>
     * @psalm-pure
     */
    public static function internKeysEach(array $a): array
    {
        return array_map(self::internKeys(...), $a);
    }

    /**
     * Each inner list of ids as strings.
     *
     * @template K of array-key
     * @template K2 of array-key
     * @param array<K, array<K2, int>> $a
     * @return array<K, array<K2, string>>
     * @psalm-pure
     */
    public static function lookupListEach(array $a): array
    {
        return array_map(self::lookupList(...), $a);
    }

    /**
     * Each inner list of strings as ids.
     *
     * @template K of array-key
     * @template K2 of array-key
     * @param array<K, array<K2, string>> $a
     * @return array<K, array<K2, int>>
     * @psalm-pure
     */
    public static function internListEach(array $a): array
    {
        return array_map(self::internList(...), $a);
    }

    /**
     * An array with the names at the given paths ("#k" keys, "#v" values, nested "#v#k" ...) interned: where a
     * foreign array of names enters.
     *
     * @param array<array-key, mixed> $a
     * @return array<array-key, mixed>
     * @psalm-pure
     */
    public static function internAt(array $a, string ...$paths): array
    {
        return self::mapAt($a, $paths, true);
    }

    /**
     * An array with the ids at the given paths looked up: where names leave as strings.
     *
     * @param array<array-key, mixed> $a
     * @return array<array-key, mixed>
     * @psalm-pure
     */
    public static function lookupAt(array $a, string ...$paths): array
    {
        return self::mapAt($a, $paths, false);
    }

    /**
     * @param array<array-key, mixed> $a
     * @param list<string> $paths
     * @return array<array-key, mixed>
     * @psalm-pure
     */
    private static function mapAt(array $a, array $paths, bool $intern): array
    {
        $keys = false;
        $values = false;
        $below = [];
        foreach ($paths as $p) {
            if ($p === '#k') {
                $keys = true;
            } elseif ($p === '#v') {
                $values = true;
            } elseif (str_starts_with($p, '#v')) {
                $below[] = substr($p, 2);
            }
        }
        $out = [];
        foreach ($a as $k => $v) {
            if ($keys) {
                $k = $intern ? self::intern((string) $k) : self::lookup((int) $k);
            }
            if ($values && $v !== null) {
                /** @psalm-suppress MixedArgument */
                $v = $intern ? self::intern($v) : self::lookup($v);
            } elseif ($below !== [] && is_array($v)) {
                $v = self::mapAt($v, $below, $intern);
            }
            $out[$k] = $v;
        }
        return $out;
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

    /**
     * Keeps the table beside the cache: ids read back from cached storages resolve in later runs.
     */
    public static function persistTo(string $cache_directory): void
    {
        if (self::$persist_file !== null) {
            return;
        }
        self::$persist_file = $cache_directory . '/interner';
        $pid = (int) getmypid();
        register_shutdown_function(static function () use ($pid): void {
            // the process that registered it (not a forked worker), when it interned something new
            if (getmypid() !== $pid || self::$persist_file === null || count(self::$strings) <= self::$persisted_count) {
                return;
            }
            self::load();
            $tmp = self::$persist_file . '.' . $pid;
            if (@file_put_contents($tmp, serialize(self::$strings)) !== false) {
                @rename($tmp, self::$persist_file);
            }
        });
    }

    private static function load(): void
    {
        if (self::$persist_file === null || !is_file(self::$persist_file)) {
            return;
        }
        /** @psalm-suppress MixedAssignment the serialized table */
        $data = @unserialize((string) @file_get_contents(self::$persist_file));
        if (is_array($data)) {
            /** @var array<int, string> $data */
            self::merge($data);
        }
        self::$persisted_count = count(self::$strings);
    }

    /**
     * An id not in the table yet: a preloaded name (Sym constants resolve before any Codebase exists), or one
     * interned by an earlier run whose storages came from the cache.
     */
    private static function lookupMissing(int $id): string
    {
        self::merge(Sym::PRELOADED);
        if (!isset(self::$strings[$id])) {
            self::load();
        }
        return self::$strings[$id]
            ?? throw new UnexpectedValueException('Unknown interned id ' . $id);
    }
}
