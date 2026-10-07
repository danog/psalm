<?php

declare(strict_types=1);

namespace Psalm\Internal;

use function count;

/**
 * Process-wide string interner: dense, sequential int ids for names, as pzoom's `StrId` (pzoom-str): id 0 is
 * the empty string, ids are never reused, and the table only grows.
 *
 * Interning is case-sensitive (names are interned as spelled; callers intern the form they key by).
 *
 * Unlike pzoom, whose analysis threads can only `find()` in a table frozen after the scan, Psalm forms some
 * keys only during analysis (the code-use graph's nodes are interned where a reference is recorded), so
 * `intern()` is also called then. Ids are stable within a process; a forked worker's table diverges from its
 * parent's after the fork, so data crossing a process boundary carries strings, not ids (see
 * CodeUseGraph::toPortable()).
 *
 * @internal
 */
final class Interner
{
    /** @var array<string, int> */
    private static array $ids = ['' => 0];

    /** @var list<string> */
    private static array $strings = [''];

    /**
     * The id of $s, assigned on first use.
     *
     * @psalm-external-mutation-free
     */
    public static function intern(string $s): int
    {
        /** @psalm-suppress ImpureStaticProperty the interning table */
        $id = self::$ids[$s] ?? null;
        if ($id !== null) {
            return $id;
        }

        /** @psalm-suppress ImpureStaticProperty the interning table */
        $id = count(self::$strings);
        /** @psalm-suppress ImpureStaticProperty the interning table */
        self::$strings[] = $s;
        /** @psalm-suppress ImpureStaticProperty the interning table */
        self::$ids[$s] = $id;

        return $id;
    }

    /**
     * The id of $s if it was interned, else 0.
     *
     * @psalm-external-mutation-free
     */
    public static function find(string $s): int
    {
        /** @psalm-suppress ImpureStaticProperty the interning table */
        return self::$ids[$s] ?? 0;
    }

    /**
     * The string an id stands for.
     *
     * @psalm-external-mutation-free
     */
    public static function lookup(int $id): string
    {
        /** @psalm-suppress ImpureStaticProperty the interning table */
        return self::$strings[$id] ?? '';
    }
}
