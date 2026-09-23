<?php

declare(strict_types=1);

namespace Psalm\Internal\Provider;

use InvalidArgumentException;
use LogicException;
use Psalm\Internal\Interner;
use Psalm\Issue\DuplicateClass;
use Psalm\IssueBuffer;
use Psalm\Storage\ClassLikeStorage;

use function array_key_exists;
use function strtolower;

/**
 * @internal
 */
final class ClassLikeStorageProvider
{
    /**
     * Storing this statically is much faster (at least in PHP 7.2.1)
     *
     * @var array<string, ClassLikeStorage>
     */
    private static array $storage = [];

    /**
     * @var array<string, ClassLikeStorage>
     */
    private static array $new_storage = [];

    /**
     * Storages by the exact spelling a lookup used (pzoom resolves names case-sensitively; Psalm keeps its
     * case-insensitive semantics, but a spelling that resolved once resolves again without lowercasing).
     * Emptied whenever the storage map changes.
     *
     * @var array<string, ClassLikeStorage>
     */
    private static array $by_spelling = [];

    /**
     * Storages by the interned declared name (pzoom keys its class-like map by StrId).
     *
     * @var array<int, ClassLikeStorage>
     */
    private static array $by_id = [];

    /**
     * class_alias() targets by the alias's lowercased name (ClassLikes::addClassAlias), so an alias's id
     * resolves to the aliased storage like a differently-cased spelling does.
     *
     * @var array<lowercase-string, string>
     */
    private static array $aliases = [];

    /**
     * The storage a differently-spelled name resolves to (Psalm resolves names case-insensitively for now:
     * the id of the spelling used maps to the storage declared under another casing), or null. Emptied
     * whenever the storage map changes.
     *
     * @var array<int, ClassLikeStorage|null>
     */
    private static array $canonical = [];

    /**
     * @psalm-mutation-free
     */
    public function __construct(public ?ClassLikeStorageCacheProvider $cache = null)
    {
    }

    /**
     * @psalm-mutation-free
     * @throws InvalidArgumentException when class does not exist
     */
    public function get(string $fq_classlike_name): ClassLikeStorage
    {
        /** @psalm-suppress ImpureStaticProperty Used only for caching */
        $known = self::$by_spelling[$fq_classlike_name] ?? null;
        if ($known !== null) {
            return $known;
        }
        $fq_classlike_name_lc = strtolower($fq_classlike_name);
        /** @psalm-suppress ImpureStaticProperty Used only for caching */
        if (!isset(self::$storage[$fq_classlike_name_lc])) {
            throw new InvalidArgumentException('Could not get class storage for ' . $fq_classlike_name_lc);
        }

        /** @psalm-suppress ImpureStaticProperty Used only for caching */
        $storage = self::$storage[$fq_classlike_name_lc];
        /** @psalm-suppress ImpureStaticProperty Used only for caching */
        self::$by_spelling[$fq_classlike_name] = $storage;
        return $storage;
    }

    /**
     * The storage of the class-like whose declared name interns to $id, or, while resolution is
     * case-insensitive, of the one whose lowercased name matches.
     *
     * @psalm-mutation-free
     * @throws InvalidArgumentException when class does not exist
     */
    public function getById(int $id): ClassLikeStorage
    {
        /** @psalm-suppress ImpureStaticProperty, ImpureMethodCall Used only for caching */
        return self::$by_id[$id]
            ?? self::resolveId($id)
            ?? throw new InvalidArgumentException('Could not get class storage for ' . Interner::lookup($id));
    }

    /**
     * @psalm-mutation-free
     */
    public function findById(int $id): ?ClassLikeStorage
    {
        /** @psalm-suppress ImpureStaticProperty, ImpureMethodCall Used only for caching */
        return self::$by_id[$id] ?? self::resolveId($id);
    }

    /**
     * @psalm-mutation-free
     */
    public function hasById(int $id): bool
    {
        /** @psalm-suppress ImpureStaticProperty, ImpureMethodCall Used only for caching */
        return isset(self::$by_id[$id]) || self::resolveId($id) !== null;
    }

    /**
     * @psalm-external-mutation-free
     */
    private static function resolveId(int $id): ?ClassLikeStorage
    {
        if (isset(self::$canonical[$id]) || array_key_exists($id, self::$canonical)) {
            return self::$canonical[$id];
        }
        $lc = strtolower(Interner::lookup($id));
        $storage = self::$storage[$lc] ?? null;
        for ($hops = 0; $storage === null && $hops < 10 && isset(self::$aliases[$lc]); $hops++) {
            $lc = strtolower(self::$aliases[$lc]);
            $storage = self::$storage[$lc] ?? null;
        }
        self::$canonical[$id] = $storage;
        return $storage;
    }

    /**
     * @param lowercase-string $alias_name_lc
     * @psalm-external-mutation-free
     */
    public static function addAlias(string $alias_name_lc, string $fq_class_name): void
    {
        self::$aliases[$alias_name_lc] = $fq_class_name;
        self::$canonical = [];
    }

    /**
     * @psalm-mutation-free
     */
    public function has(string $fq_classlike_name): bool
    {
        /** @psalm-suppress ImpureStaticProperty Used only for caching */
        if (isset(self::$by_spelling[$fq_classlike_name])) {
            return true;
        }
        $fq_classlike_name_lc = strtolower($fq_classlike_name);

        /** @psalm-suppress ImpureStaticProperty Used only for caching */
        $storage = self::$storage[$fq_classlike_name_lc] ?? null;
        if ($storage === null) {
            return false;
        }
        /** @psalm-suppress ImpureStaticProperty Used only for caching */
        self::$by_spelling[$fq_classlike_name] = $storage;
        return true;
    }

    public function exhume(string $fq_classlike_name, string $file_path, string $file_contents): ClassLikeStorage
    {
        $fq_classlike_name_lc = strtolower($fq_classlike_name);

        if (isset(self::$storage[$fq_classlike_name_lc])) {
            return self::$storage[$fq_classlike_name_lc];
        }

        if (!$this->cache) {
            throw new LogicException('Cannot exhume when there’s no cache');
        }

        $cached_value = $this->cache->getLatestFromCache($fq_classlike_name_lc, $file_path, $file_contents);

        self::$storage[$fq_classlike_name_lc] = $cached_value;
        Interner::intern($cached_value->name);
        self::$by_id[$cached_value->id] = $cached_value;
        self::$by_spelling = [];
        self::$canonical = [];
        self::$new_storage[$fq_classlike_name_lc] = $cached_value;

        return $cached_value;
    }

    /**
     * @return array<string, ClassLikeStorage>
     * @psalm-external-mutation-free
     */
    public static function getAll(): array
    {
        return self::$storage;
    }

    /**
     * @return array<string, ClassLikeStorage>
     * @psalm-external-mutation-free
     */
    public function getNew(): array
    {
        return self::$new_storage;
    }

    /**
     * @param array<string, ClassLikeStorage> $more
     */
    public function addMore(array $more): void
    {
        foreach ($more as $k => $storage) {
            if (isset(self::$storage[$k])) {
                $duplicate_storage = self::$storage[$k];
                $duplicate_location = $duplicate_storage->location ?? $duplicate_storage->stmt_location;
                $location = $storage->location ?? $storage->stmt_location;
                if ($duplicate_location !== null
                    && $location !== null
                    && $duplicate_location->getHash() !== $location->getHash()
                ) {
                    IssueBuffer::maybeAdd(
                        new DuplicateClass(
                            'Class ' . $k . ' has already been defined'
                            . ' in ' . $location->file_path,
                            $location,
                        ),
                    );

                    //$storage->file_storage->has_visitor_issues = true;

                    $duplicate_storage->has_visitor_issues = true;

                    continue;
                }
            }
            self::$new_storage[$k] = $storage;
            self::$storage[$k] = $storage;
            self::$by_id[$storage->id] = $storage;
            self::$by_spelling = [];
            self::$canonical = [];
        }
    }

    /**
     * @psalm-external-mutation-free
     */
    public function makeNew(string $fq_classlike_name_lc): void
    {
        self::$new_storage[$fq_classlike_name_lc] = self::$storage[$fq_classlike_name_lc];
    }

    /**
     * @psalm-external-mutation-free
     */
    public function create(string $fq_classlike_name): ClassLikeStorage
    {
        $fq_classlike_name_lc = strtolower($fq_classlike_name);

        $storage = new ClassLikeStorage($fq_classlike_name);
        self::$storage[$fq_classlike_name_lc] = $storage;
        self::$by_id[$storage->id] = $storage;
        self::$by_spelling = [];
        self::$canonical = [];
        self::$new_storage[$fq_classlike_name_lc] = $storage;

        return $storage;
    }

    /**
     * @psalm-external-mutation-free
     */
    public function remove(string $fq_classlike_name): void
    {
        $fq_classlike_name_lc = strtolower($fq_classlike_name);

        $existing = self::$storage[$fq_classlike_name_lc] ?? null;
        if ($existing !== null) {
            unset(self::$by_id[$existing->id]);
        }
        unset(self::$storage[$fq_classlike_name_lc]);
        self::$by_spelling = [];
        self::$canonical = [];
    }

    /**
     * @psalm-external-mutation-free
     */
    public static function deleteAll(): void
    {
        self::$storage = [];
        self::$new_storage = [];
        self::$by_spelling = [];
        self::$by_id = [];
        self::$canonical = [];
        self::$aliases = [];
    }

    /**
     * @psalm-external-mutation-free
     */
    public static function populated(): void
    {
        self::$new_storage = [];
    }
}
