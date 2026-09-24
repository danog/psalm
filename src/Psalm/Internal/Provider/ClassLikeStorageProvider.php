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
     * The storage of the class-like whose declared name interns to $id, or, while resolution is
     * case-insensitive, of the one whose lowercased name matches.
     *
     * @psalm-mutation-free
     * @throws InvalidArgumentException when class does not exist
     */
    public function get(int $id): ClassLikeStorage
    {
        /** @psalm-suppress ImpureStaticProperty, ImpureMethodCall Used only for caching */
        return self::$by_id[$id]
            ?? self::resolveId($id)
            ?? throw new InvalidArgumentException('Could not get class storage for ' . Interner::lookup($id));
    }

    /**
     * @psalm-mutation-free
     */
    public function find(int $id): ?ClassLikeStorage
    {
        /** @psalm-suppress ImpureStaticProperty, ImpureMethodCall Used only for caching */
        return self::$by_id[$id] ?? self::resolveId($id);
    }

    /**
     * @psalm-mutation-free
     */
    public function has(int $id): bool
    {
        /** @psalm-suppress ImpureStaticProperty, ImpureMethodCall Used only for caching */
        return isset(self::$by_id[$id]) || self::resolveId($id) !== null;
    }

    /** @var array<int, int> canonical id by name id (see canonicalId) */
    private static array $canonical_ids = [];

    /**
     * The canonical id of a class-like name (pzoom's resolved StrId): the id of the storage it resolves to
     * (another casing or a class_alias), else the id of the name its aliases lead to. Resolved once per name.
     *
     * @psalm-mutation-free
     */
    public function canonicalId(int $id): int
    {
        /** @psalm-suppress ImpureStaticProperty, ImpureMethodCall Used only for caching */
        return self::$canonical_ids[$id] ??= (self::$by_id[$id] ?? self::resolveId($id))?->id
            ?? self::aliasTarget($id);
    }

    /** @psalm-external-mutation-free */
    private static function aliasTarget(int $id): int
    {
        $name = Interner::lookup($id);
        for ($hops = 0; $hops < 10 && isset(self::$aliases[strtolower($name)]); $hops++) {
            $name = self::$aliases[strtolower($name)];
        }
        return Interner::intern($name);
    }

    /**
     * The storage declared under the name $id interns to (case-insensitively, as PHP declares class-likes), not
     * following class aliases: what registration and duplicate detection need. Lookups resolve aliases (find).
     *
     * @psalm-mutation-free
     */
    public function findDeclared(int $id): ?ClassLikeStorage
    {
        /** @psalm-suppress ImpureStaticProperty Used only for caching */
        return self::$by_id[$id] ?? self::$storage[strtolower(Interner::lookup($id))] ?? null;
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
        self::$canonical_ids = [];
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
        self::$canonical = [];
        self::$canonical_ids = [];
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
            self::$canonical = [];
        self::$canonical_ids = [];
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
        self::$canonical = [];
        self::$canonical_ids = [];
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
        self::$canonical = [];
        self::$canonical_ids = [];
    }

    /**
     * @psalm-external-mutation-free
     */
    public static function deleteAll(): void
    {
        self::$storage = [];
        self::$new_storage = [];
        self::$by_id = [];
        self::$canonical = [];
        self::$canonical_ids = [];
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
