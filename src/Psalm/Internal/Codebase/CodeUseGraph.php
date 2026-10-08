<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use Closure;
use LogicException;
use Psalm\CodeLocation;
use Psalm\Context;
use Psalm\Internal\Interner;
use Psalm\Storage\Capabilities;
use Psalm\Storage\FunctionLikeStorage;
use Psalm\Storage\MethodStorage;

use function array_intersect_key;
use function array_pop;
use function md5;
use function str_contains;
use function strpos;
use function strrpos;
use function strtolower;
use function substr;

/**
 * A directed graph of "uses" between code elements (classes, function-likes,
 * properties, class constants), used for dead code detection, for reference
 * lookups (language server, `--find-references-to`) and for invalidating
 * cached analysis results in `--diff` mode.
 *
 * Every node is an int packing a node kind and one or two interned names (pzoom's symbol references are
 * `(StrId, StrId)` pairs): `kind << 58 | b << 29 | (a ^ mix(b, kind))`. PHP hashes an int key to itself and
 * picks the bucket from its low bits, so those must vary between nodes: with `b` (0 for single-name nodes)
 * in the low bits nearly every node would land in one bucket chain. A class node is (class), a function-like, return or
 * missing-method node is (function or method id), a property, constant or missing-property node is (class,
 * member), a use-alias node is (alias, file hash), a file node is (file path); the public API root is 0.
 * Nothing is concatenated or lowercased to form a node on the reference path. The old string form
 * (`<kind> <member>`) is only built for the cache, for data crossing a process boundary and for the
 * statement differ's member ids (nodeToString()).
 *
 * An edge `A -> B` means "if A is alive, B is used by A". As in pzoom only the forward edges are recorded
 * while code is analysed; the backward index the queries need is derived from them on demand.
 *
 * Usage is resolved by a reachability search from a set of root nodes
 * (the public API, top-level file code, free functions, which psalm never
 * reports as unused, and code outside of the project), so code that is only
 * referenced from other unused code (including cycles of otherwise
 * unreferenced code) is correctly reported as unused.
 *
 * The mutation info of function-likes, resolved by MutationLevelResolver, is kept here too; it is keyed by the
 * function-like's node name (functionLikeNodeName()), not by graph node ids.
 *
 * @internal
 */
final class CodeUseGraph
{
    public const PUBLIC_API = 0;

    /** A regular use of the target: the target is used if the source is alive. */
    public const EDGE_USE = 1;

    /**
     * A write to a property: the property is not considered used (only
     * reads make a property used), but the edge is tracked for reference
     * lookups and cache invalidation.
     */
    public const EDGE_WRITE = 2;

    /** From the "return value" node of a function-like to the function-like itself. */
    public const EDGE_RETURN = 3;

    /** From a method to the class declaring it. */
    public const EDGE_METHOD = 4;

    /** From the public API root node to a node. */
    public const EDGE_PUBLIC_API = 5;

    /**
     * From an overridden parent or interface method (or its return value) to
     * the overriding method (or its return value): a call to the parent method
     * may end up in the overriding one, but only if the overriding class is
     * itself used, so these edges are only followed once the class of the
     * target is used.
     */
    public const EDGE_OVERRIDE = 6;

    /**
     * Edge types derived from the class storages rather than from the analysed
     * code: recomputed on every run instead of being cached.
     */
    public const STRUCTURAL_EDGES = [
        self::EDGE_PUBLIC_API => true,
        self::EDGE_OVERRIDE => true,
        self::EDGE_METHOD => true,
    ];

    /** The names of the edge types in the cache and in data crossing a process boundary. */
    private const EDGE_NAMES = [
        self::EDGE_USE => 'use',
        self::EDGE_WRITE => 'write',
        self::EDGE_RETURN => 'return',
        self::EDGE_METHOD => 'method',
        self::EDGE_PUBLIC_API => 'public-api',
        self::EDGE_OVERRIDE => 'override',
    ];

    private const EDGE_TYPES = [
        'use' => self::EDGE_USE,
        'write' => self::EDGE_WRITE,
        'return' => self::EDGE_RETURN,
        'method' => self::EDGE_METHOD,
        'public-api' => self::EDGE_PUBLIC_API,
        'override' => self::EDGE_OVERRIDE,
    ];

    private const KIND_CLASS = 1;
    private const KIND_FUNCTION_LIKE = 2;
    private const KIND_RETURN = 3;
    private const KIND_PROPERTY = 4;
    private const KIND_CONSTANT = 5;
    private const KIND_MISSING_METHOD = 6;
    private const KIND_MISSING_PROPERTY = 7;
    private const KIND_USE_ALIAS = 8;
    private const KIND_FILE = 9;

    private const SHIFT_KIND = 58;
    private const SHIFT_B = 29;
    private const MASK = 0x1FFFFFFF;
    /** odd, so mix() is a bijection of (b, kind) modulo 2^29 */
    private const MIX = 0x9E3779B1;

    /**
     * Forward edges: source node => target node => edge type
     *
     * @var array<int, array<int, int>>
     */
    private array $forward_edges = [];

    /**
     * Backward edges (target node => source node => edge type), derived from $forward_edges on demand and kept up
     * to date once built; null until a query needs it.
     *
     * @var array<int, array<int, int>>|null
     */
    private ?array $backward_edges = null;

    /**
     * The file (interned path) each source node was seen in, used to compute file-level
     * references in `--diff` mode and to prune stale edges.
     *
     * @var array<int, int>
     */
    private array $node_files = [];

    /**
     * Node => location hash => location of the references to the node.
     * Only populated when $collect_locations is true.
     *
     * @var array<int, array<string, CodeLocation>>
     */
    private array $locations = [];

    /**
     * Source node => target node => location hash => true. This lets references
     * and their locations be removed together when a source is invalidated.
     *
     * @var array<int, array<int, array<string, true>>>
     */
    private array $source_locations = [];

    /**
     * Target node => location hash => source node => true. A source location
     * can be shared by trait analyses, so it is retained until every source
     * that recorded it has been invalidated.
     *
     * @var array<int, array<string, array<int, true>>>
     */
    private array $location_sources = [];

    /**
     * Set of used nodes, null until resolved.
     *
     * @var array<int, true>|null
     */
    private ?array $used = null;

    /**
     * Cached reverse index of $node_files (file id => nodes), rebuilt on demand.
     *
     * @var array<int, array<int, true>>|null
     */
    private ?array $file_nodes = null;

    /**
     * Cached index of the nodes referencing any member of a class
     * (lowercase class name => nodes), rebuilt on demand.
     *
     * @var array<string, array<int, true>>|null
     */
    private ?array $class_referencing_nodes = null;

    /**
     * The mutations performed by each analysed function-like and the
     * unannotated function-likes it calls, resolved by MutationLevelResolver.
     *
     * @var array<string, MutationInfo>
     */
    private array $mutation_info = [];

    /**
     * The levels resolved from $mutation_info, see {@see self::getMutationLevels()}.
     *
     * @var array<string, int>|null
     */
    private ?array $mutation_levels = null;

    /** @var array<string, int> interned md5 of each file path seen by useAliasNode() */
    private static array $file_path_hash_ids = [];

    /**
     * The nodes formed so far, by name: a repeated reference costs one lookup here.
     *
     * @var array<string, int>
     */
    private static array $class_nodes = [];
    /** @var array<string, int> */
    private static array $function_like_nodes = [];
    /** @var array<string, int> */
    private static array $return_nodes = [];
    /** @var array<string, int> */
    private static array $missing_method_nodes = [];
    /** @var array<string, int> */
    private static array $file_path_nodes = [];
    /** @var array<string, array<string, int>> */
    private static array $property_nodes = [];
    /** @var array<string, array<string, int>> */
    private static array $constant_nodes = [];
    /** @var array<string, array<string, int>> */
    private static array $missing_property_nodes = [];
    /** @var array<string, array<int, int>> */
    private static array $use_alias_nodes = [];

    /** @var array<int, int> owner class (interned lowercase name, 0 for none) of the nodes asked about */
    private static array $owner_class_ids = [];

    /**
     * @psalm-mutation-free
     */
    public function __construct(
        public bool $collect_locations = false,
    ) {
    }

    // Node ids

    /** @psalm-pure */
    private static function pack(int $kind, int $a, int $b): int
    {
        return ($kind << self::SHIFT_KIND) | ($b << self::SHIFT_B) | ($a ^ self::mix($b, $kind));
    }

    /** @psalm-pure */
    private static function mix(int $b, int $kind): int
    {
        return (($b ^ ($kind << 24)) * self::MIX) & self::MASK;
    }

    /** @psalm-pure */
    private static function kindOf(int $node): int
    {
        return $node >> self::SHIFT_KIND;
    }

    /** @psalm-pure */
    private static function firstOf(int $node): int
    {
        $b = ($node >> self::SHIFT_B) & self::MASK;

        return ($node & self::MASK) ^ self::mix($b, $node >> self::SHIFT_KIND);
    }

    /** @psalm-pure */
    private static function secondOf(int $node): int
    {
        return ($node >> self::SHIFT_B) & self::MASK;
    }

    /**
     * @param lowercase-string $fq_class_name_lc
     * @psalm-external-mutation-free
     */
    public static function classNode(string $fq_class_name_lc): int
    {
        /** @psalm-suppress ImpureStaticProperty cache of a pure function */
        return self::$class_nodes[$fq_class_name_lc] ??= self::pack(self::KIND_CLASS, Interner::intern($fq_class_name_lc), 0);
    }

    /**
     * classNode() of the class-like whose lowercase name has the interned id $fq_class_name_lc_id.
     *
     * @psalm-pure
     */
    public static function classNodeOfId(int $fq_class_name_lc_id): int
    {
        return self::pack(self::KIND_CLASS, $fq_class_name_lc_id, 0);
    }

    /**
     * @param lowercase-string $function_id_lc a method id (`class::method`) or a function id
     * @psalm-external-mutation-free
     */
    public static function functionLikeNode(string $function_id_lc): int
    {
        /** @psalm-suppress ImpureStaticProperty cache of a pure function */
        return self::$function_like_nodes[$function_id_lc] ??= self::pack(self::KIND_FUNCTION_LIKE, Interner::intern($function_id_lc), 0);
    }

    /**
     * @param lowercase-string $function_id_lc
     * @psalm-external-mutation-free
     */
    public static function functionLikeReturnNode(string $function_id_lc): int
    {
        /** @psalm-suppress ImpureStaticProperty cache of a pure function */
        return self::$return_nodes[$function_id_lc] ??= self::pack(self::KIND_RETURN, Interner::intern($function_id_lc), 0);
    }

    /**
     * @param lowercase-string $fq_class_name_lc
     * @param string $property_name the property name, without the leading `$`
     * @psalm-external-mutation-free
     */
    public static function propertyNode(string $fq_class_name_lc, string $property_name): int
    {
        /** @psalm-suppress ImpureStaticProperty cache of a pure function */
        return self::$property_nodes[$fq_class_name_lc][$property_name] ??= self::pack(self::KIND_PROPERTY, Interner::intern($fq_class_name_lc), Interner::intern($property_name));
    }

    /**
     * @param lowercase-string $fq_class_name_lc
     * @psalm-external-mutation-free
     */
    public static function classConstantNode(string $fq_class_name_lc, string $const_name): int
    {
        /** @psalm-suppress ImpureStaticProperty cache of a pure function */
        return self::$constant_nodes[$fq_class_name_lc][$const_name] ??= self::pack(self::KIND_CONSTANT, Interner::intern($fq_class_name_lc), Interner::intern($const_name));
    }

    /**
     * @param lowercase-string $method_id_lc
     * @psalm-external-mutation-free
     */
    public static function missingMethodNode(string $method_id_lc): int
    {
        /** @psalm-suppress ImpureStaticProperty cache of a pure function */
        return self::$missing_method_nodes[$method_id_lc] ??= self::pack(self::KIND_MISSING_METHOD, Interner::intern($method_id_lc), 0);
    }

    /**
     * @param lowercase-string $fq_class_name_lc
     * @param string $property_name the property name, without the leading `$`
     * @psalm-external-mutation-free
     */
    public static function missingPropertyNode(string $fq_class_name_lc, string $property_name): int
    {
        /** @psalm-suppress ImpureStaticProperty cache of a pure function */
        return self::$missing_property_nodes[$fq_class_name_lc][$property_name] ??= self::pack(
            self::KIND_MISSING_PROPERTY,
            Interner::intern($fq_class_name_lc),
            Interner::intern($property_name),
        );
    }

    /**
     * A node representing a `use` import alias in a given file: methods
     * referencing the alias get invalidated when the import changes.
     *
     * @psalm-external-mutation-free
     */
    public static function useAliasNode(string $alias, string $file_path): int
    {
        // do NOT change this to hash, it will fail on Windows for whatever reason; computed once per file
        /** @psalm-suppress ImpureStaticProperty cache of a pure function */
        $hash_id = self::$file_path_hash_ids[$file_path] ??= Interner::intern(md5($file_path));

        return self::$use_alias_nodes[$alias][$hash_id] ??= self::pack(self::KIND_USE_ALIAS, Interner::intern($alias), $hash_id);
    }

    /**
     * A node representing the top-level code of a file.
     *
     * @psalm-external-mutation-free
     */
    public static function fileNode(string $file_path): int
    {
        /** @psalm-suppress ImpureStaticProperty cache of a pure function */
        return self::$file_path_nodes[$file_path] ??= self::pack(self::KIND_FILE, Interner::intern($file_path), 0);
    }

    /**
     * The name of a function-like's node (`func <id>`): the key of its mutation info.
     *
     * @param lowercase-string $function_id_lc
     * @psalm-pure
     */
    public static function functionLikeNodeName(string $function_id_lc): string
    {
        return 'func ' . $function_id_lc;
    }

    /**
     * The node name (see functionLikeNodeName()) of a method or named function, from its storage: the key of
     * its mutation info. Closures have no name: their node is derived from their closure id.
     *
     * @psalm-mutation-free
     */
    public static function functionLikeNodeForStorage(FunctionLikeStorage $storage): ?string
    {
        if ($storage instanceof MethodStorage) {
            if ($storage->defining_fqcln === null || $storage->cased_name === null) {
                return null;
            }

            return self::functionLikeNodeName(strtolower($storage->defining_fqcln . '::' . $storage->cased_name));
        }

        if ($storage->cased_name === null) {
            return null;
        }

        return self::functionLikeNodeName(strtolower($storage->cased_name));
    }

    /**
     * The string form of a node (`<kind> <member>`), as stored in the cache.
     *
     * @psalm-external-mutation-free
     */
    public static function nodeToString(int $node): string
    {
        $a = Interner::lookup(self::firstOf($node));
        $b = self::secondOf($node);

        return match (self::kindOf($node)) {
            self::KIND_CLASS => 'class ' . $a,
            self::KIND_FUNCTION_LIKE => 'func ' . $a,
            self::KIND_RETURN => 'return ' . $a,
            self::KIND_PROPERTY => 'property ' . $a . '::$' . Interner::lookup($b),
            self::KIND_CONSTANT => 'const ' . $a . '::' . Interner::lookup($b),
            self::KIND_MISSING_METHOD => 'missing-method ' . $a,
            self::KIND_MISSING_PROPERTY => 'missing-property ' . $a . '::$' . Interner::lookup($b),
            self::KIND_USE_ALIAS => 'use-alias use:' . $a . ':' . Interner::lookup($b),
            self::KIND_FILE => 'file ' . $a,
            default => 'public-api',
        };
    }

    /**
     * The node a string form (see nodeToString()) stands for.
     *
     * @psalm-external-mutation-free
     */
    public static function nodeFromString(string $node_name): int
    {
        $pos = strpos($node_name, ' ');

        if ($pos === false) {
            return self::PUBLIC_API;
        }

        $kind = substr($node_name, 0, $pos);
        $member = substr($node_name, $pos + 1);

        if ($kind === 'class') {
            return self::pack(self::KIND_CLASS, Interner::intern($member), 0);
        }
        if ($kind === 'func') {
            return self::pack(self::KIND_FUNCTION_LIKE, Interner::intern($member), 0);
        }
        if ($kind === 'return') {
            return self::pack(self::KIND_RETURN, Interner::intern($member), 0);
        }
        if ($kind === 'missing-method') {
            return self::pack(self::KIND_MISSING_METHOD, Interner::intern($member), 0);
        }
        if ($kind === 'file') {
            return self::pack(self::KIND_FILE, Interner::intern($member), 0);
        }
        if ($kind === 'use-alias') {
            // `use:<alias>:<file hash>`
            $last = (int) strrpos($member, ':');

            return self::pack(
                self::KIND_USE_ALIAS,
                Interner::intern(substr($member, 4, $last - 4)),
                Interner::intern(substr($member, $last + 1)),
            );
        }

        // `<class>::<constant>` or `<class>::$<property>`
        $separator = (int) strpos($member, '::');
        $class = Interner::intern(substr($member, 0, $separator));

        if ($kind === 'const') {
            return self::pack(self::KIND_CONSTANT, $class, Interner::intern(substr($member, $separator + 2)));
        }

        return self::pack(
            $kind === 'property' ? self::KIND_PROPERTY : self::KIND_MISSING_PROPERTY,
            $class,
            Interner::intern(substr($member, $separator + 3)),
        );
    }

    /**
     * Returns the class member id (`class::member`) for nodes representing class
     * members, in the same format used by the statement differ, or null.
     *
     * @psalm-external-mutation-free
     */
    public static function getMemberId(int $node): ?string
    {
        $kind = self::kindOf($node);
        $a = self::firstOf($node);

        return match ($kind) {
            self::KIND_FUNCTION_LIKE, self::KIND_RETURN, self::KIND_MISSING_METHOD => Interner::lookup($a),
            self::KIND_PROPERTY, self::KIND_MISSING_PROPERTY => Interner::lookup($a) . '::$'
                . Interner::lookup(self::secondOf($node)),
            self::KIND_CONSTANT => Interner::lookup($a) . '::' . Interner::lookup(self::secondOf($node)),
            self::KIND_USE_ALIAS => 'use:' . Interner::lookup($a) . ':' . Interner::lookup(self::secondOf($node)),
            default => null,
        };
    }

    /**
     * The interned lowercase name of the class a node belongs to, 0 for nodes that don't belong to a class
     * (files, use aliases, free functions, the public API root).
     *
     * @psalm-external-mutation-free
     */
    private static function getOwnerClassId(int $node): int
    {
        $kind = self::kindOf($node);

        if ($kind === self::KIND_CLASS || $kind === self::KIND_PROPERTY || $kind === self::KIND_CONSTANT
            || $kind === self::KIND_MISSING_PROPERTY
        ) {
            return self::firstOf($node);
        }

        if ($kind !== self::KIND_FUNCTION_LIKE && $kind !== self::KIND_RETURN && $kind !== self::KIND_MISSING_METHOD) {
            return 0;
        }

        /** @psalm-suppress ImpureStaticProperty cache of a pure function */
        $owner = self::$owner_class_ids[$node] ?? null;

        if ($owner === null) {
            $member = Interner::lookup(self::firstOf($node));
            $separator = strpos($member, '::');
            $owner = $separator === false ? 0 : Interner::intern(substr($member, 0, $separator));
            /** @psalm-suppress ImpureStaticProperty cache of a pure function */
            self::$owner_class_ids[$node] = $owner;
        }

        return $owner;
    }

    /**
     * Returns the lowercase name of the class a node belongs to, or null for
     * nodes that don't belong to a class (files, free functions, roots).
     *
     * @return lowercase-string|null
     * @psalm-external-mutation-free
     */
    public static function getOwnerClass(int $node): ?string
    {
        $owner = self::getOwnerClassId($node);

        if ($owner === 0) {
            return null;
        }

        /** @var lowercase-string */
        return Interner::lookup($owner);
    }

    /**
     * Whether a node is a root of the usage search: a root is always alive.
     *
     * @param Closure[_](int): bool $is_external whether a node belongs to code outside of the project
     * @psalm-external-mutation-free
     */
    private static function isRoot(int $node, Closure $is_external): bool
    {
        if ($node === self::PUBLIC_API) {
            return true;
        }

        $kind = self::kindOf($node);

        if ($kind === self::KIND_FILE) {
            return true;
        }

        // Psalm never reports unused free functions, so they're entry points
        if ($kind === self::KIND_FUNCTION_LIKE && !str_contains(Interner::lookup(self::firstOf($node)), '::')) {
            return true;
        }

        return $is_external($node);
    }

    // Building

    /**
     * Records that $source_node uses $target_node.
     *
     * @psalm-external-mutation-free
     */
    public function addEdge(int $source_node, int $target_node, int $type = self::EDGE_USE): void
    {
        if ($source_node === $target_node) {
            return;
        }

        $existing_type = $this->forward_edges[$source_node][$target_node] ?? null;

        // a write never downgrades a read
        if ($existing_type !== null && ($existing_type === $type || $type === self::EDGE_WRITE)) {
            return;
        }

        $this->forward_edges[$source_node][$target_node] = $type;

        if ($this->backward_edges !== null) {
            $this->backward_edges[$target_node][$source_node] = $type;
        }

        $this->used = null;
        $this->class_referencing_nodes = null;
    }

    /**
     * Records a reference to $target_node from the code element described by
     * $context (the calling method or function, or the class for class-level
     * code), falling back to the top-level code of the file of $location
     * (or $file_path).
     *
     * Nothing is recorded if neither a usable context nor a file is known.
     *
     * @psalm-external-mutation-free
     */
    public function addReference(
        int $target_node,
        ?Context $context,
        ?CodeLocation $location = null,
        int $type = self::EDGE_USE,
        ?string $file_path = null,
    ): void {
        if ($context === null) {
            $this->addReferenceFrom($target_node, null, null, null, $location, $type, $file_path);
            return;
        }

        $source_node = $context->reference_source_node ??= self::scopeNode(
            $context->calling_method_id,
            $context->calling_function_id,
            $context->self,
        );

        $file_path = $location?->file_path ?? $file_path;

        if ($source_node === 0) {
            if ($file_path === null) {
                return;
            }
            $source_node = self::fileNode($file_path);
        }

        $this->addReferenceFromNode($source_node, $target_node, $location, $type, $file_path);
    }

    /**
     * The node references from a scope come from: its method's or function's, else its class's; 0 for none.
     *
     * @param lowercase-string|null $calling_method_id
     * @param lowercase-string|null $calling_function_id
     * @psalm-external-mutation-free
     */
    private static function scopeNode(?string $calling_method_id, ?string $calling_function_id, ?string $self): int
    {
        if ($calling_method_id !== null) {
            return self::functionLikeNode($calling_method_id);
        }
        if ($calling_function_id !== null) {
            return self::functionLikeNode($calling_function_id);
        }
        if ($self !== null) {
            return self::classNode(strtolower($self));
        }
        return 0;
    }

    /**
     * addReference() with the referencing scope given directly (no Context needs building for it).
     *
     * @param lowercase-string|null $calling_method_id
     * @param lowercase-string|null $calling_function_id
     * @psalm-external-mutation-free
     */
    public function addReferenceFrom(
        int $target_node,
        ?string $calling_method_id,
        ?string $calling_function_id,
        ?string $self,
        ?CodeLocation $location = null,
        int $type = self::EDGE_USE,
        ?string $file_path = null,
    ): void {
        $file_path = $location?->file_path ?? $file_path;

        if ($calling_method_id !== null) {
            $source_node = self::functionLikeNode($calling_method_id);
        } elseif ($calling_function_id !== null) {
            $source_node = self::functionLikeNode($calling_function_id);
        } elseif ($self !== null) {
            $source_node = self::classNode(strtolower($self));
        } elseif ($file_path !== null) {
            $source_node = self::fileNode($file_path);
        } else {
            return;
        }

        $this->addReferenceFromNode($source_node, $target_node, $location, $type, $file_path);
    }

    /** @psalm-external-mutation-free */
    private function addReferenceFromNode(
        int $source_node,
        int $target_node,
        ?CodeLocation $location,
        int $type,
        ?string $file_path,
    ): void {
        $this->addEdge($source_node, $target_node, $type);

        if ($file_path !== null && !isset($this->node_files[$source_node])) {
            $this->node_files[$source_node] = Interner::intern($file_path);
            $this->file_nodes = null;
        }

        if ($location !== null && $this->collect_locations) {
            $location_hash = $location->getHash();
            $this->locations[$target_node][$location_hash] = $location;
            $this->source_locations[$source_node][$target_node][$location_hash] = true;
            $this->location_sources[$target_node][$location_hash][$source_node] = true;
        }
    }

    /**
     * @psalm-external-mutation-free
     */
    public function markAsPublicApi(int $node): void
    {
        $this->addEdge(self::PUBLIC_API, $node, self::EDGE_PUBLIC_API);
    }

    /**
     * Merges another graph of the same process into this one.
     *
     * @psalm-external-mutation-free
     */
    public function addGraph(self $other): void
    {
        foreach ($other->forward_edges as $source_node => $targets) {
            foreach ($targets as $target_node => $type) {
                $this->addEdge($source_node, $target_node, $type);
            }
        }

        foreach ($other->node_files as $node => $file_id) {
            if (!isset($this->node_files[$node])) {
                $this->node_files[$node] = $file_id;
                $this->file_nodes = null;
            }
        }

        foreach ($other->source_locations as $source_node => $targets) {
            foreach ($targets as $target_node => $location_hashes) {
                foreach ($location_hashes as $location_hash => $_) {
                    $location = $other->locations[$target_node][$location_hash] ?? null;

                    if ($location !== null) {
                        $this->addLocation($source_node, $target_node, $location_hash, $location);
                    }
                }
            }
        }

        $this->mutation_info = $other->mutation_info + $this->mutation_info;
        $this->mutation_levels = null;
    }

    /**
     * This graph in a form that can cross a process boundary (node ids are only valid in the process that
     * interned them): the merge of a worker's graph into the main process (see addPortable()).
     *
     * @psalm-external-mutation-free
     */
    public function toPortable(): PortableCodeUseGraph
    {
        $edges = [];

        foreach ($this->forward_edges as $source_node => $targets) {
            $source_name = self::nodeToString($source_node);

            foreach ($targets as $target_node => $type) {
                $edges[$source_name][self::nodeToString($target_node)] = self::EDGE_NAMES[$type] ?? 'use';
            }
        }

        $node_files = [];

        foreach ($this->node_files as $node => $file_id) {
            $node_files[self::nodeToString($node)] = Interner::lookup($file_id);
        }

        $locations = [];

        foreach ($this->source_locations as $source_node => $targets) {
            foreach ($targets as $target_node => $location_hashes) {
                foreach ($location_hashes as $location_hash => $_) {
                    $location = $this->locations[$target_node][$location_hash] ?? null;

                    if ($location !== null) {
                        $locations[] = new PortableCodeUseLocation(
                            self::nodeToString($source_node),
                            self::nodeToString($target_node),
                            $location_hash,
                            $location,
                        );
                    }
                }
            }
        }

        return new PortableCodeUseGraph($edges, $node_files, $this->mutation_info, $locations);
    }

    /**
     * Merges a graph made portable by toPortable() (e.g. a worker's) into this one.
     *
     * @psalm-external-mutation-free
     */
    public function addPortable(PortableCodeUseGraph $other): void
    {
        $this->addEdgeNames($other->edges);
        $this->addNodeFileNames($other->node_files);

        foreach ($other->locations as $portable_location) {
            $this->addLocation(
                self::nodeFromString($portable_location->source_node),
                self::nodeFromString($portable_location->target_node),
                $portable_location->location_hash,
                $portable_location->location,
            );
        }

        $this->mutation_info = $other->mutation_info + $this->mutation_info;
        $this->mutation_levels = null;
    }

    /**
     * @param array<string, array<string, string>> $edges
     * @psalm-external-mutation-free
     */
    private function addEdgeNames(array $edges): void
    {
        foreach ($edges as $source_name => $targets) {
            $source_node = self::nodeFromString($source_name);

            foreach ($targets as $target_name => $type_name) {
                $this->addEdge($source_node, self::nodeFromString($target_name), self::EDGE_TYPES[$type_name] ?? self::EDGE_USE);
            }
        }
    }

    /**
     * @param array<string, string> $node_files
     * @psalm-external-mutation-free
     */
    private function addNodeFileNames(array $node_files): void
    {
        foreach ($node_files as $node_name => $file_path) {
            $node = self::nodeFromString($node_name);

            if (!isset($this->node_files[$node])) {
                $this->node_files[$node] = Interner::intern($file_path);
                $this->file_nodes = null;
            }
        }
    }

    /** @psalm-external-mutation-free */
    private function addLocation(int $source_node, int $target_node, string $location_hash, CodeLocation $location): void
    {
        $this->locations[$target_node][$location_hash] = $location;
        $this->source_locations[$source_node][$target_node][$location_hash] = true;
        $this->location_sources[$target_node][$location_hash][$source_node] = true;
    }

    /**
     * @psalm-external-mutation-free
     */
    public function clear(): void
    {
        $this->forward_edges = [];
        $this->backward_edges = null;
        $this->node_files = [];
        $this->locations = [];
        $this->source_locations = [];
        $this->location_sources = [];
        $this->mutation_info = [];
        $this->mutation_levels = null;
        $this->used = null;
        $this->file_nodes = null;
        $this->class_referencing_nodes = null;
    }

    // Mutation levels

    /**
     * Records the mutations performed by an analysed function-like.
     *
     * @param string $node_id the function-like's node name (functionLikeNodeName())
     * @psalm-external-mutation-free
     */
    public function addMutationInfo(string $node_id, MutationInfo $info): void
    {
        $this->mutation_info[$node_id] = $info;
        $this->mutation_levels = null;
    }

    /**
     * @return array<string, MutationInfo>
     * @psalm-mutation-free
     */
    public function getMutationInfo(): array
    {
        return $this->mutation_info;
    }

    /**
     * The final mutation level of every function-like with mutation info, resolved once
     * the whole codebase has been analysed and cached until the mutation info changes.
     *
     * @return array<string, int> node name => bitmask of {@see Capabilities} constants
     * @psalm-external-mutation-free
     */
    public function getMutationLevels(): array
    {
        return $this->mutation_levels ??= MutationLevelResolver::resolveLevels($this->mutation_info);
    }

    /**
     * Marks the mutation info of a function-like as coming from a previous
     * run, so that its issues are not reported again.
     *
     * @psalm-external-mutation-free
     */
    public function markMutationInfoStale(string $node_id): void
    {
        if (isset($this->mutation_info[$node_id])) {
            $this->mutation_info[$node_id]->fresh = false;
        }
    }

    // Resolution

    /**
     * Computes the set of used nodes: everything reachable from a root node
     * through non-write edges. Iterative, so cycles and deep graphs are fine.
     *
     * Must be called before isUsed(), and again after the graph changes.
     *
     * @param Closure[_](int): bool $is_external whether a node (with outgoing
     *        edges) belongs to code outside of the project, e.g. a vendor class
     *        or a caller made up by a plugin: such code is never reported as
     *        unused, so what it references is used.
     * @psalm-capabilities read-props|write-this-props|write-refs
     */
    public function resolve(Closure $is_external): void
    {
        $used = [self::PUBLIC_API => true];
        $queue = [self::PUBLIC_API];

        foreach ($this->forward_edges as $node => $_) {
            if (!isset($used[$node]) && self::isRoot($node, $is_external)) {
                $used[$node] = true;
                $queue[] = $node;
            }
        }

        $deferred = self::noDeferredOverrides();

        while ($queue) {
            $node = array_pop($queue);

            foreach ($this->forward_edges[$node] ?? [] as $target_node => $type) {
                if ($type === self::EDGE_WRITE || isset($used[$target_node])) {
                    continue;
                }

                if ($type === self::EDGE_OVERRIDE) {
                    $owner_class = self::getOwnerClassId($target_node);

                    // An override edge is normally only followed once the
                    // overriding class is known to be used, since a call to the
                    // parent only reaches the override when that class is
                    // actually instantiated. When the parent is external,
                    // though, external code holding the parent type can invoke
                    // the override on an instance it constructs itself, which
                    // Psalm cannot see — so the override (e.g. a plugin entry
                    // point implementing a vendor interface) must be treated as
                    // reachable regardless of any in-project instantiation.
                    if ($owner_class !== 0 && !$is_external($node)) {
                        $owner_node = self::pack(self::KIND_CLASS, $owner_class, 0);

                        if (!isset($used[$owner_node])) {
                            $deferred[$owner_node][] = $target_node;
                            continue;
                        }
                    }
                }

                $used[$target_node] = true;
                $queue[] = $target_node;

                foreach ($deferred[$target_node] ?? [] as $deferred_node) {
                    if (!isset($used[$deferred_node])) {
                        $used[$deferred_node] = true;
                        $queue[] = $deferred_node;
                    }
                }

                unset($deferred[$target_node]);
            }
        }

        $this->used = $used;
    }

    /**
     * Override edges whose target's class is not used yet, by class node: the empty starting point of resolve(),
     * typed here because a local's type comes from what is assigned to it.
     *
     * @return array<int, list<int>>
     * @psalm-pure
     */
    private static function noDeferredOverrides(): array
    {
        return [];
    }

    /**
     * The backward edges (target => source => edge type), derived from the forward ones the first time a query
     * needs them and kept up to date from then on.
     *
     * @return array<int, array<int, int>>
     * @psalm-external-mutation-free
     */
    private function backwardEdges(): array
    {
        if ($this->backward_edges === null) {
            $backward_edges = [];

            foreach ($this->forward_edges as $source_node => $targets) {
                foreach ($targets as $target_node => $type) {
                    $backward_edges[$target_node][$source_node] = $type;
                }
            }

            $this->backward_edges = $backward_edges;
        }

        return $this->backward_edges;
    }

    /**
     * @psalm-mutation-free
     */
    public function isUsed(int $node): bool
    {
        if ($this->used === null) {
            throw new LogicException('The graph must be resolved before checking usage');
        }

        return isset($this->used[$node]);
    }

    /**
     * Whether a node is referenced by other used code, as opposed to being
     * merely alive as an entry point or through its own members.
     *
     * @psalm-external-mutation-free
     */
    public function isReferenced(int $node): bool
    {
        $used = $this->used;

        if ($used === null) {
            throw new LogicException('The graph must be resolved before checking usage');
        }

        $owner = self::getOwnerClassId($node);

        foreach ($this->backwardEdges()[$node] ?? [] as $source_node => $type) {
            if (($type === self::EDGE_USE || $type === self::EDGE_WRITE)
                && isset($used[$source_node])
                && self::getOwnerClassId($source_node) !== $owner
            ) {
                return true;
            }
        }

        return false;
    }

    // Queries

    /**
     * Returns the nodes referencing $node, optionally only through edges
     * of the given type.
     *
     * @return array<int, true>
     * @psalm-external-mutation-free
     */
    public function getReferencingNodes(int $node, ?int $type = null): array
    {
        $references = [];

        foreach ($this->backwardEdges()[$node] ?? [] as $source_node => $edge_type) {
            if ($type === null || $edge_type === $type) {
                $references[$source_node] = true;
            }
        }

        return $references;
    }

    /**
     * Returns only references made by code that is reachable from an entry point.
     *
     * @return array<int, true>
     * @psalm-external-mutation-free
     */
    public function getUsedReferencingNodes(int $node, ?int $type = null): array
    {
        $used = $this->used;

        if ($used === null) {
            throw new LogicException('The graph must be resolved before checking usage');
        }

        return array_intersect_key($this->getReferencingNodes($node, $type), $used);
    }

    /**
     * @return array<string, CodeLocation>
     * @psalm-mutation-free
     */
    public function getReferenceLocations(int $node): array
    {
        return $this->locations[$node] ?? [];
    }

    /**
     * Target node name => referencing node name => true (see nodeToString())
     *
     * @return array<string, array<string, true>>
     * @psalm-external-mutation-free
     */
    public function getAllReferences(): array
    {
        $result = [];

        foreach ($this->backwardEdges() as $target_node => $sources) {
            $target_name = self::nodeToString($target_node);

            foreach ($sources as $source_node => $_) {
                $result[$target_name][self::nodeToString($source_node)] = true;
            }
        }

        return $result;
    }

    /**
     * Returns, for each referenced class member id (in the statement differ
     * format), the ids of the function-likes referencing it. Used to find the
     * methods to re-analyse when a member changes.
     *
     * @return array<string, array<string, true>>
     * @psalm-external-mutation-free
     */
    public function getFunctionLikeReferencesToMembers(): array
    {
        $result = [];

        foreach ($this->backwardEdges() as $target_node => $sources) {
            $member_id = self::getMemberId($target_node);

            if ($member_id === null) {
                continue;
            }

            foreach ($sources as $source_node => $type) {
                if (($type === self::EDGE_USE || $type === self::EDGE_WRITE)
                    && self::kindOf($source_node) === self::KIND_FUNCTION_LIKE
                ) {
                    $result[$member_id][Interner::lookup(self::firstOf($source_node))] = true;
                }
            }
        }

        return $result;
    }

    /**
     * Returns the nodes referencing the given class or any of its members.
     * The file of a node is given by getNodeFile(); when unknown, it is the
     * file of the node's owner class, if any (see getOwnerClass()).
     *
     * @param lowercase-string $fq_class_name_lc
     * @return array<int, true>
     * @psalm-external-mutation-free
     */
    public function getNodesReferencingClass(string $fq_class_name_lc): array
    {
        if ($this->class_referencing_nodes === null) {
            $class_referencing_nodes = [];

            foreach ($this->backwardEdges() as $target_node => $sources) {
                $owner = self::getOwnerClassId($target_node);

                if ($owner === 0) {
                    continue;
                }

                $owner_name = Interner::lookup($owner);

                foreach ($sources as $source_node => $_) {
                    $class_referencing_nodes[$owner_name][$source_node] = true;
                }
            }

            $this->class_referencing_nodes = $class_referencing_nodes;
        }

        return $this->class_referencing_nodes[$fq_class_name_lc] ?? [];
    }

    /**
     * Returns the file a node was seen in, if known.
     *
     * @psalm-external-mutation-free
     */
    public function getNodeFile(int $node): ?string
    {
        if (self::kindOf($node) === self::KIND_FILE) {
            return Interner::lookup(self::firstOf($node));
        }

        $file_id = $this->node_files[$node] ?? null;

        return $file_id === null ? null : Interner::lookup($file_id);
    }

    // Invalidation (--diff mode)

    /**
     * Removes all the references made by a node, e.g. because the code it
     * represents is about to be re-analysed or was deleted.
     *
     * @psalm-external-mutation-free
     */
    public function removeReferencesFrom(int $node): void
    {
        foreach ($this->source_locations[$node] ?? [] as $target_node => $location_hashes) {
            foreach ($location_hashes as $location_hash => $_) {
                unset($this->location_sources[$target_node][$location_hash][$node]);

                if (!$this->location_sources[$target_node][$location_hash]) {
                    unset(
                        $this->location_sources[$target_node][$location_hash],
                        $this->locations[$target_node][$location_hash],
                    );
                }
            }

            if (!$this->location_sources[$target_node]) {
                unset($this->location_sources[$target_node], $this->locations[$target_node]);
            }
        }

        unset($this->source_locations[$node]);

        if ($this->backward_edges !== null) {
            foreach ($this->forward_edges[$node] ?? [] as $target_node => $_) {
                unset($this->backward_edges[$target_node][$node]);

                if (!$this->backward_edges[$target_node]) {
                    unset($this->backward_edges[$target_node]);
                }
            }
        }

        unset(
            $this->forward_edges[$node],
            $this->node_files[$node],
            $this->mutation_info[self::nodeToString($node)],
        );
        $this->mutation_levels = null;
        $this->used = null;
        $this->file_nodes = null;
        $this->class_referencing_nodes = null;
    }

    /**
     * Removes all the edges of the given types.
     *
     * @param array<int, true> $types
     * @psalm-external-mutation-free
     */
    public function removeEdgesOfTypes(array $types): void
    {
        foreach ($this->forward_edges as $source_node => $targets) {
            foreach ($targets as $target_node => $type) {
                if (!isset($types[$type])) {
                    continue;
                }

                unset($this->forward_edges[$source_node][$target_node]);
            }

            if (!$this->forward_edges[$source_node]) {
                unset($this->forward_edges[$source_node]);
            }
        }

        $this->backward_edges = null;
        $this->used = null;
        $this->class_referencing_nodes = null;
    }

    /**
     * Removes all the references made by the nodes seen in a file, except the
     * nodes in $keep_nodes.
     *
     * @param array<int, true> $keep_nodes
     * @psalm-external-mutation-free
     */
    public function removeReferencesFromFile(string $file_path, array $keep_nodes = []): void
    {
        $this->removeReferencesFrom(self::fileNode($file_path));

        if ($this->file_nodes === null) {
            $file_nodes = [];

            foreach ($this->node_files as $node => $file_id) {
                $file_nodes[$file_id][$node] = true;
            }

            $this->file_nodes = $file_nodes;
        }

        foreach ($this->file_nodes[Interner::find($file_path)] ?? [] as $node => $_) {
            if (!isset($keep_nodes[$node])) {
                $this->removeReferencesFrom($node);
            }
        }
    }

    // Caching

    /**
     * The graph's data for the reference cache. The port's cache lives in memory for the run (see
     * FileReferenceCacheProvider), so it keeps the interned node ids: they are valid for the whole process.
     *
     * @return array{
     *     edges: array<int, array<int, int>>,
     *     node_files: array<int, int>,
     *     mutation_info: array<string, MutationInfo>
     * }
     * @psalm-external-mutation-free
     */
    public function getCacheData(): array
    {
        $edges = [];

        foreach ($this->forward_edges as $source_node => $targets) {
            foreach ($targets as $target_node => $type) {
                if (!isset(self::STRUCTURAL_EDGES[$type])) {
                    $edges[$source_node][$target_node] = $type;
                }
            }
        }

        $mutation_info = [];

        foreach ($this->mutation_info as $node_id => $info) {
            $stale = clone $info;
            $stale->fresh = false;
            $mutation_info[$node_id] = $stale;
        }

        return [
            'edges' => $edges,
            'node_files' => $this->node_files,
            'mutation_info' => $mutation_info,
        ];
    }

    /**
     * Merges cached data produced by getCacheData() into the graph.
     *
     * @param array{
     *     edges: array<int, array<int, int>>,
     *     node_files: array<int, int>,
     *     mutation_info?: array<string, MutationInfo>
     * } $data
     * @psalm-external-mutation-free
     */
    public function loadCacheData(array $data): void
    {
        foreach ($data['edges'] as $source_node => $targets) {
            foreach ($targets as $target_node => $type) {
                $this->addEdge($source_node, $target_node, $type);
            }
        }

        $this->node_files += $data['node_files'];

        $this->mutation_info += $data['mutation_info'] ?? [];
        $this->mutation_levels = null;

        $this->file_nodes = null;
    }
}
