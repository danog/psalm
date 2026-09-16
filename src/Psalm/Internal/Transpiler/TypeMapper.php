<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use Psalm\Codebase;
use Psalm\Internal\Type\TypeAlias\ClassTypeAlias;
use Psalm\Type\Atomic\TTypeAlias;
use Psalm\Type;
use Psalm\Type\Atomic;
use Psalm\Type\Atomic\Scalar;
use Psalm\Type\Atomic\TArray;
use Psalm\Type\Atomic\TArrayKey;
use Psalm\Type\Atomic\TBool;
use Psalm\Type\Atomic\TCallable;
use Psalm\Type\Atomic\TCallableObject;
use Psalm\Type\Atomic\TClassConstant;
use Psalm\Type\Atomic\TCallableString;
use Psalm\Type\Atomic\TClassString;
use Psalm\Type\Atomic\TClassStringMap;
use Psalm\Type\Atomic\TValueOf;
use Psalm\Type\Atomic\TTraitString;
use Psalm\Type\Atomic\TClosedResource;
use Psalm\Type\Atomic\TClosure;
use Psalm\Type\Atomic\TConditional;
use Psalm\Type\Atomic\TEnumCase;
use Psalm\Type\Atomic\TFalse;
use Psalm\Type\Atomic\TFloat;
use Psalm\Type\Atomic\TGenericObject;
use Psalm\Type\Atomic\TInt;
use Psalm\Type\Atomic\TIterable;
use Psalm\Type\Atomic\TKeyedArray;
use Psalm\Type\Atomic\TLiteralClassString;
use Psalm\Type\Atomic\TMixed;
use Psalm\Type\Atomic\TNamedObject;
use Psalm\Type\Atomic\TNever;
use Psalm\Type\Atomic\TNull;
use Psalm\Type\Atomic\TNumeric;
use Psalm\Type\Atomic\TObject;
use Psalm\Type\Atomic\TObjectWithProperties;
use Psalm\Type\Atomic\TResource;
use Psalm\Type\Atomic\TScalar;
use Psalm\Type\Atomic\TString;
use Psalm\Type\Atomic\TTemplateParam;
use Psalm\Type\Atomic\TTemplateParamClass;
use Psalm\Type\Atomic\TTrue;
use Psalm\Type\Atomic\TVoid;
use Psalm\Type\Union;

use function array_keys;
use function array_map;
use function array_values;
use function count;
use function is_int;
use function ksort;
use function sort;
use function implode;
use function strtolower;
use function substr;
use function usort;

/**
 * Maps Psalm types to Rust types and records the generated union/shape types that need emitting.
 *
 * @internal
 */
final class TypeMapper
{
    /** @var array<string, RustType> generated union enums by mangled name */
    public array $unions = [];

    /** @var array<string, RustType> generated shape structs by mangled name */
    public array $shapes = [];

    /** @var array<string, RustType> closure signatures encountered */
    public array $closures = [];

    /** @var array<string, int> unsupported atomic types seen (for diagnostics) */
    public array $unsupported = [];

    /** @var array<string, int> reason => count of `Mixed` types generated (drive-to-zero diagnostics) */
    public array $mixed_roots = [];

    /** @var array<string, array<string, int>> reason => context => count (which PHP members yield Mixed) */
    public array $mixed_sites = [];

    /** Class that `static`/`self` types refer to while mapping. */
    public ?string $current_class = null;

    /** @var array<string, true> class-constants currently being resolved (recursion guard). */
    private array $resolving_class_const = [];

    /** Where the type being mapped comes from (a member or a function body), for the map inventories. */
    public ?string $context = null;

    /** @var array<string, array<string, int>> context => Psalm type => count of arrays mapped to `Map<ArrayKey, _>` */
    public array $array_key_sites = [];

    /** @var array<string, array<string, int>> context => Psalm type => count of arrays mapped to `Map<i64, _>` */
    public array $int_key_sites = [];

    /** @var array<string, array<string, int>> context => shape id => count of shapes too wide for a struct (mapped to Map) */
    public array $shape_map_sites = [];

    /** crate whose code is being emitted: types may only name classes of this crate and upstream ones */
    public int $current_crate = 0;

    /** Runtime-provided generic classes. */
    private const RT_GENERICS = [
        'splobjectstorage' => 'SplObjectStorage',
        'arrayobject' => 'ArrayObject',
        'arrayiterator' => 'ArrayIterator',
        'generator' => 'Generator',
        'weakreference' => 'WeakReference',
        'weakmap' => 'WeakMap',
        // php-rt aliases the whole iterator family to one type:
        //   `pub type {PhpIterator,Traversable,IteratorAggregate}<K,V> = Generator<K,V>`.
        // They must therefore map to the SAME Rust name here, or a union that mixes them (e.g.
        // Iterator|Traversable) emits two `CastTo<Generator>` impls for one Rust type (E0119). Unions
        // dedupe members by rendered type, so a shared name collapses them to a single member.
        'iterator' => 'Generator',
        'iteratoraggregate' => 'Generator',
        'traversable' => 'Generator',
    ];

    /**
     * Rust generic parameters in scope (PHP template name => Rust generic name) while mapping the signature
     * and body of a generic function: an unbounded `@template T` maps to a Rust generic `T` instead of Mixed.
     *
     * @var array<string, string>
     */
    public array $generic_names = [];

    public function __construct(
        private readonly Codebase $codebase,
        private readonly Program $program,
    ) {
    }

    /**
     * The unbounded (`mixed`-bounded) fn-level templates of a function, as Rust generic names.
     *
     * @return array<string, string>
     */
    /**
     * Whether a template param has no concrete bound: its `as` is `mixed`, or (as Psalm types a value re-derived
     * from the template inside the body) `T|mixed` / `T as (T as mixed)|mixed` — the same param nested.
     */
    private static function isUnboundedTemplate(TTemplateParam $t, int $depth = 0): bool
    {
        foreach ($t->as->getAtomicTypes() as $a) {
            if ($a instanceof TMixed) {
                continue;
            }
            if ($a instanceof TTemplateParam && $a->param_name === $t->param_name && $depth < 6 && self::isUnboundedTemplate($a, $depth + 1)) {
                continue;
            }
            return false;
        }
        return true;
    }

    public static function genericNamesOf(\Psalm\Storage\FunctionLikeStorage $storage): array
    {
        $out = [];
        foreach ($storage->template_types ?? [] as $name => $defs) {
            foreach ($defs as $bound) {
                if ($bound->isMixed()) {
                    $out[$name] = 'G_' . preg_replace('/[^A-Za-z0-9_]/', '_', $name);
                }
            }
        }
        return $out;
    }

    /** Count a `Mixed` type at its root cause (drive-to-zero diagnostics), returning `Mixed`. */
    /** Count an `AnyObject` (dynamic object handle) at its root cause, returning `AnyObject`. */
    private function anyRoot(string $why): RustType
    {
        $this->mixed_roots['ANYOBJECT ' . $why] = ($this->mixed_roots['ANYOBJECT ' . $why] ?? 0) + 1;
        if ($this->context !== null) {
            $this->mixed_sites['ANYOBJECT ' . $why][$this->context] = ($this->mixed_sites['ANYOBJECT ' . $why][$this->context] ?? 0) + 1;
        }
        return RustType::anyObject();
    }

    private function mixedRoot(string $why): RustType
    {
        $this->mixed_roots[$why] = ($this->mixed_roots[$why] ?? 0) + 1;
        if ($this->context !== null) {
            $this->mixed_sites[$why][$this->context] = ($this->mixed_sites[$why][$this->context] ?? 0) + 1;
        }
        return RustType::mixed();
    }

    public function map(?Union $type): RustType
    {
        if ($type === null) {
            return $this->mixedRoot('null-type (untyped PHP)');
        }

        $atomics = $this->expandAliases(array_values($type->getAtomicTypes()));
        // `T|mixed` (Psalm's sort/ksort/array_* @param-out for a template T): an unbounded template already
        // covers every value, so the union IS the generic parameter (not Mixed)
        if ($this->generic_names !== [] && count($atomics) >= 2) {
            $generic = null;
            $only_generic_and_mixed = true;
            foreach ($atomics as $a) {
                if ($a instanceof TMixed) {
                    continue;
                }
                if ($a instanceof TTemplateParam && isset($this->generic_names[$a->param_name]) && self::isUnboundedTemplate($a)
                    && ($generic === null || $generic === $this->generic_names[$a->param_name])
                ) {
                    $generic = $this->generic_names[$a->param_name];
                    continue;
                }
                $only_generic_and_mixed = false;
                break;
            }
            if ($only_generic_and_mixed && $generic !== null) {
                return RustType::generic($generic);
            }
        }
        $nullable = false;
        $members = [];
        $has_true = false;
        $has_false = false;
        $has_bool = false;

        foreach ($atomics as $atomic) {
            if ($atomic instanceof TNull) {
                $nullable = true;
                continue;
            }
            if ($atomic instanceof TVoid) {
                $nullable = true;
                continue;
            }
            if ($atomic instanceof TNever) {
                continue;
            }
            if ($atomic instanceof TMixed) {
                return $this->mixedRoot('TMixed (declared mixed)');
            }
            if ($atomic instanceof TTrue) {
                $has_true = true;
                continue;
            }
            if ($atomic instanceof TFalse) {
                $has_false = true;
                continue;
            }
            if ($atomic instanceof TBool) {
                $has_bool = true;
                continue;
            }
            $members[] = $this->mapAtomic($atomic);
        }

        if ($has_bool || ($has_true && $has_false)) {
            $members[] = RustType::bool();
        } elseif ($has_true) {
            $members[] = count($members) === 0 && !$nullable ? RustType::bool() : $this->unitVariant('True');
        } elseif ($has_false) {
            // NB: `T|false` stays a union (False unit), NOT Option<T>. Tried mapping it to Option<T> (None==false):
            // it compiled + cleared ~182 SSA candidates + ~130 Mixed, BUT regressed analysis correctness badly
            // (ArgTest 55->28, ArrayAccess 81->47) — `false`-as-`None` diverges from PHP `false` in real flows
            // (coalesce, storing/passing X|false, comparisons beyond `=== false`). Needs a DISTINCT false-option
            // type (not the null Option) with false-aware comparison/coalesce/cast emission. Reverted.
            $members[] = count($members) === 0 && !$nullable ? RustType::bool() : $this->unitVariant('False');
        }

        $result = $this->combine($members);

        if ($nullable && $result->kind !== RustType::MIXED) {
            return RustType::option($result);
        }

        return $result;
    }

    /** Placeholder RustType for `true`/`false` union members (unit variants). */
    private function unitVariant(string $name): RustType
    {
        return RustType::rtGeneric('__unit_' . $name, []);
    }

    /** Register a shape built outside the mapper (typed get_object_vars / (array) casts) for emission. */
    public function registerShape(RustType $shape): RustType
    {
        $this->shapes[$shape->mangle()] = $shape;
        return $shape;
    }

    /**
     * @param list<RustType> $members
     */
    public function combine(array $members): RustType
    {
        // dedupe
        $unique = [];
        foreach ($members as $m) {
            $unique[$m->toRust()] = $m;
        }
        $members = array_values($unique);

        if (count($members) === 0) {
            return RustType::unit();
        }
        if (count($members) === 1) {
            $only = $members[0];
            if ($only->kind === RustType::RT_GENERIC && str_starts_with($only->name, '__unit_')) {
                return RustType::bool();
            }
            return $only;
        }

        // int|string => ArrayKey
        $kinds = array_map(static fn(RustType $m) => $m->kind, $members);
        if (count($members) === 2 && in_array(RustType::INT, $kinds, true) && in_array(RustType::STR, $kinds, true)) {
            return RustType::arrayKey();
        }
        foreach ($members as $m) {
            if ($m->kind === RustType::MIXED) {
                return $this->mixedRoot('union-with-mixed');
            }
        }
        // int|ArrayKey, string|ArrayKey => ArrayKey
        if (in_array(RustType::ARRAY_KEY, $kinds, true)) {
            $rest = [];
            foreach ($members as $m) {
                if ($m->kind !== RustType::INT && $m->kind !== RustType::STR && $m->kind !== RustType::ARRAY_KEY) {
                    $rest[] = $m;
                }
            }
            if (count($rest) === 0) {
                return RustType::arrayKey();
            }
            $members = [RustType::arrayKey(), ...$rest];
        }

        // flatten nested unions
        $flat = [];
        foreach ($members as $m) {
            if ($m->kind === RustType::UNION) {
                foreach ($m->params as $p) {
                    $flat[$p->toRust()] = $p;
                }
            } else {
                $flat[$m->toRust()] = $m;
            }
        }
        $members = array_values($flat);
        usort($members, static fn(RustType $a, RustType $b) => $a->mangle() <=> $b->mangle());

        $union = $this->recursive($members) ?? RustType::union($members);
        $this->unions[$union->mangle()] = $union;
        return $union;
    }

    /**
     * `L | list<V>` where `V` is itself `L | list<...>` with the same leaves: one recursive enum.
     *
     * @param list<RustType> $members sorted union members
     */
    private function recursive(array $members): ?RustType
    {
        $lists = [];
        $leaves = [];
        foreach ($members as $m) {
            if ($m->kind === RustType::LIST) {
                $lists[] = $m;
            } else {
                $leaves[] = $m;
            }
        }
        if (count($lists) !== 1 || count($leaves) === 0) {
            return null;
        }
        $elem = $lists[0]->inner();
        $nullable = $elem->kind === RustType::OPTION;
        $ev = $nullable ? $elem->inner() : $elem;
        // only for real nesting: the element union must contain a list itself
        if ($ev->kind !== RustType::UNION || !self::selfSimilar($ev, self::keys($leaves), $nullable, true)) {
            return null;
        }
        return RustType::recursiveUnion($leaves, $nullable);
    }

    /** @param list<RustType> $ts */
    private static function keys(array $ts): string
    {
        $k = array_map(static fn(RustType $t) => $t->toRust(), $ts);
        sort($k);
        return implode('|', $k);
    }

    private static function selfSimilar(RustType $u, string $leaf_keys, bool $nullable, bool $need_list): bool
    {
        if ($u->isRecursive()) {
            $own = [];
            foreach ($u->params as $m) {
                if ($m->kind !== RustType::LIST) {
                    $own[] = $m;
                }
            }
            return self::keys($own) === $leaf_keys;
        }
        $lists = [];
        $own = [];
        foreach ($u->params as $m) {
            if ($m->kind === RustType::LIST) {
                $lists[] = $m;
            } else {
                $own[] = $m;
            }
        }
        if (count($lists) > 1 || self::keys($own) !== $leaf_keys) {
            return false;
        }
        if (count($lists) === 0) {
            return !$need_list;
        }
        $elem = $lists[0]->inner();
        if (($elem->kind === RustType::OPTION) !== $nullable) {
            return false;
        }
        $ev = $nullable ? $elem->inner() : $elem;
        return $ev->kind === RustType::UNION && self::selfSimilar($ev, $leaf_keys, $nullable, false);
    }

    /**
     * Type aliases (`@psalm-type` / `@psalm-import-type`) are stored unexpanded in method and property
     * storages: replace them by their definitions.
     *
     * @param list<Atomic> $atomics
     * @return list<Atomic>
     */
    private function expandAliases(array $atomics, int $depth = 0): array
    {
        $out = [];
        foreach ($atomics as $atomic) {
            // `\Foo::Alias` written in a docblock without @psalm-import-type parses as a class CONSTANT type; when
            // Foo declares a @psalm-type of that name it is the alias (php-parser's NodeAttributes::AttributeArray).
            $alias_class = $atomic instanceof TTypeAlias ? $atomic->declaring_fq_classlike_name
                : ($atomic instanceof TClassConstant ? $atomic->fq_classlike_name : null);
            $alias_name = $atomic instanceof TTypeAlias ? $atomic->alias_name
                : ($atomic instanceof TClassConstant ? $atomic->const_name : null);
            if ($alias_class !== null && $alias_name !== null && $depth < 8) {
                $declaring = $alias_class;
                if ($this->codebase->classlikes->doesClassLikeExist(strtolower($declaring))) {
                    $alias = $this->codebase->classlike_storage_provider->get($declaring)->type_aliases[$alias_name] ?? null;
                    if ($alias instanceof ClassTypeAlias) {
                        foreach ($this->expandAliases($alias->replacement_atomic_types, $depth + 1) as $replacement) {
                            $out[] = $replacement;
                        }
                        continue;
                    }
                }
            }
            $out[] = $atomic;
        }
        return $out;
    }

    public function mapAtomic(Atomic $atomic): RustType
    {
        if ($atomic instanceof TTypeAlias || $atomic instanceof TClassConstant) {
            $expanded = $this->expandAliases([$atomic]);
            if (count($expanded) !== 1 || $expanded[0] !== $atomic) {
                return $this->map(new Union($expanded));
            }
        }
        if ($atomic instanceof TInt) {
            return RustType::int();
        }
        if ($atomic instanceof TFloat) {
            return RustType::float();
        }
        if ($atomic instanceof TBool) {
            return RustType::bool();
        }
        // StrId axis (pzoom models names as interned StrId): class-strings -> interned `Sym`. ENABLED 2026-09-15
        // under the user's "panics OK instead of dynamic-protocol/perf-loss" license: the prior wall was that
        // `class-string|object` unions become `Sym|AnyObject` and `CastTo<Str>` panics on the AnyObject arm when
        // analyzer code uses the object side as a string. That panic is now acceptable (type-mismatch panic beats
        // keeping names as un-interned Str). TClassString covers TTemplateParamClass; TLiteralClassString extends
        // TLiteralString so it needs its own arm. Plain TString stays Str.
        if ($atomic instanceof TClassString || $atomic instanceof TLiteralClassString
            || $atomic instanceof TTraitString || $atomic instanceof TCallableString
        ) {
            // All name-typed strings intern as Sym: class-string (+ literal/unknown/template variants),
            // trait-string, callable-string (function/method names). TUnknownClassString extends TClassString
            // so it is already covered. Data strings stay Str. NOTE: lowercase-string is NOT a clean name type
            // (Psalm types lowercased FILE PATHS as lowercase-string in config, e.g. ProjectFileFilter) — routing
            // it to Sym broke loadFromArray transpilation and aborted all tests; kept as Str.
            return RustType::sym();
        }
        if ($atomic instanceof TString) {
            return RustType::str();
        }
        if ($atomic instanceof TArrayKey) {
            return RustType::arrayKey();
        }
        if ($atomic instanceof TNull || $atomic instanceof TVoid) {
            return RustType::unit();
        }
        if ($atomic instanceof TNever) {
            return RustType::never();
        }
        if ($atomic instanceof TMixed) {
            return $this->mixedRoot('TMixed (declared mixed)');
        }
        if ($atomic instanceof TScalar) {
            return $this->combine([RustType::bool(), RustType::float(), RustType::int(), RustType::str()]);
        }
        if ($atomic instanceof TNumeric) {
            return $this->combine([RustType::float(), RustType::int(), RustType::str()]);
        }
        if ($atomic instanceof TResource || $atomic instanceof TClosedResource) {
            return RustType::resource();
        }
        if ($atomic instanceof TKeyedArray) {
            return $this->mapKeyedArray($atomic);
        }
        if ($atomic instanceof TArray) {
            return $this->mapArray($atomic->type_params[0], $atomic->type_params[1]);
        }
        if ($atomic instanceof TIterable) {
            return $this->mapArray($atomic->type_params[0], $atomic->type_params[1]);
        }
        if ($atomic instanceof TClosure || $atomic instanceof TCallable) {
            return $this->mapCallable($atomic);
        }
        if ($atomic instanceof TCallableObject) {
            return RustType::dynCallable();
        }
        if ($atomic instanceof TTemplateParam) {
            if (isset($this->generic_names[$atomic->param_name]) && self::isUnboundedTemplate($atomic)) {
                return RustType::generic($this->generic_names[$atomic->param_name]);
            }
            return $this->map($atomic->as);
        }
        if ($atomic instanceof TEnumCase) {
            return RustType::class($atomic->value);
        }
        if ($atomic instanceof TGenericObject) {
            $lc = strtolower($atomic->value);
            if (isset(self::RT_GENERICS[$lc])) {
                return RustType::rtGeneric(
                    self::RT_GENERICS[$lc],
                    $this->rtGenericParams(self::RT_GENERICS[$lc], $atomic->type_params),
                );
            }
            return $this->mapNamedObject($atomic);
        }
        if ($atomic instanceof TNamedObject) {
            return $this->mapNamedObject($atomic);
        }
        if ($atomic instanceof TObjectWithProperties) {
            return $this->anyRoot('TObjectWithProperties');
        }
        if ($atomic instanceof TObject) {
            return $this->anyRoot('TObject (declared object)');
        }
        if ($atomic instanceof TConditional) {
            return $this->combine([$this->map($atomic->if_type), $this->map($atomic->else_type)]);
        }
        if ($atomic instanceof TClassConstant) {
            // Resolve `Foo::BAR` to the constant's declared/inferred type rather than falling back to Mixed.
            // Resolve self/static/$this/parent against the class currently being mapped so those don't fall to Mixed.
            $const_class = $atomic->fq_classlike_name;
            $const_class_lc = strtolower($const_class);
            if (($const_class_lc === 'self' || $const_class_lc === 'static' || $const_class_lc === '$this')
                && $this->current_class !== null
            ) {
                $const_class = $this->current_class;
            } elseif ($const_class_lc === 'parent' && $this->current_class !== null) {
                $cur = $this->program->getClass($this->program->canonicalClassName($this->current_class));
                if ($cur !== null && $cur->parent !== null) {
                    $const_class = $cur->parent->fqcn;
                }
            }
            $key = strtolower($const_class) . '::' . $atomic->const_name;
            // Wildcards (`Foo::BAR_*`) resolve too: getClassConstantType expands the pattern (via
            // StorageByPatternResolver) to the union of all matching constants' types -> avoids Mixed for
            // e.g. Reconciler::RECONCILIATION_* (a set of int literals).
            if (!isset($this->resolving_class_const[$key])) {
                $this->resolving_class_const[$key] = true;
                try {
                    $resolved = $this->codebase->classlikes->getClassConstantType(
                        $const_class,
                        $atomic->const_name,
                        \ReflectionProperty::IS_PRIVATE,
                    );
                } catch (\Throwable) {
                    $resolved = null;
                } finally {
                    unset($this->resolving_class_const[$key]);
                }
                if ($resolved !== null) {
                    return $this->map($resolved);
                }
            }
            return $this->mixedRoot('TClassConstant');
        }
        if ($atomic instanceof TClassStringMap) {
            return RustType::map(RustType::str(), $this->map($atomic->value_param));
        }
        if ($atomic instanceof TValueOf) {
            // `value-of<Enum|array>` -> the underlying value type (e.g. a backed enum's backing int/string),
            // rather than falling back to Mixed.
            try {
                $resolved = TValueOf::getValueType($atomic->type, $this->codebase);
            } catch (\Throwable) {
                $resolved = null;
            }
            if ($resolved !== null) {
                return $this->map($resolved);
            }
            return $this->mixedRoot('TValueOf');
        }

        $this->unsupported[$atomic::class] = ($this->unsupported[$atomic::class] ?? 0) + 1;
        return $this->mixedRoot('unsupported-atomic');
    }

    /**
     * @param list<Union> $params
     * @return list<RustType>
     */
    private function rtGenericParams(string $name, array $params): array
    {
        $out = [];
        switch ($name) {
            case 'SplObjectStorage':
            case 'ArrayObject':
            case 'ArrayIterator':
            case 'PhpIterator':
            case 'IteratorAggregate':
            case 'Traversable':
            case 'Generator':
                $out[] = isset($params[0]) ? $this->map($params[0]) : $this->mixedRoot('generic-param-absent');
                $out[] = isset($params[1]) ? $this->map($params[1]) : $this->mixedRoot('generic-param-absent');
                foreach ($out as $i => $p) {
                    if ($p->kind === RustType::UNIT || $p->kind === RustType::NEVER) {
                        $out[$i] = $this->mixedRoot('generic-param-empty');
                    }
                }
                if ($name === 'SplObjectStorage' && ($out[0]->kind === RustType::MIXED)) {
                    $out[0] = RustType::anyObject();
                }
                break;
            case 'WeakReference':
                $out[] = isset($params[0]) ? $this->map($params[0]) : RustType::anyObject();
                break;
            case 'WeakMap':
                $out[] = isset($params[0]) ? $this->map($params[0]) : RustType::anyObject();
                $out[] = isset($params[1]) ? $this->map($params[1]) : $this->mixedRoot('generic-param-absent');
                break;
        }
        return $out;
    }

    private function mapNamedObject(TNamedObject $atomic): RustType
    {
        $lc = strtolower($atomic->value);
        if (isset(self::RT_GENERICS[$lc])) {
            return RustType::rtGeneric(self::RT_GENERICS[$lc], $this->rtGenericParams(self::RT_GENERICS[$lc], []));
        }
        if ($lc === 'closure' || $lc === 'callable') {
            return RustType::dynCallable();
        }
        if ($lc === 'stdclass') {
            return RustType::rtGeneric('StdClass', []);
        }
        if ($lc === 'static' || $lc === 'self' || $lc === '$this') {
            if ($this->current_class !== null) {
                return RustType::class($this->current_class);
            }
            return $this->anyRoot('static/self outside a class');
        }
        $fqcn = $this->program->canonicalClassName($atomic->value);
        $model = $this->program->getClass($fqcn);
        if ($model === null || !$model->is_project) {
            // no generated code for this class (a vendor dependency that is not transpiled): dynamic object
            return $this->anyRoot('external class ' . $fqcn);
        }
        while ($model !== null && $model->crate > $this->current_crate) {
            // a class of a downstream crate (a test subclass seen by inference): the upstream crate can only
            // name its nearest ancestor defined there
            $model = $model->parent;
        }
        if ($model === null || !$model->is_project) {
            return $this->anyRoot('downstream-crate class ' . $fqcn);
        }
        return RustType::class($model->fqcn);
    }

    private function mapArray(Union $key, Union $value): RustType
    {
        $map = RustType::map($this->mapKey($key), $this->mapValue($value));
        $this->recordMap($map, 'array<' . $key->getId() . ', ' . $value->getId() . '>');
        return $map;
    }

    /**
     * Inventory of the hash maps the program uses: PHP arrays should be lists, shapes or string-keyed maps
     * (int-keyed maps rarely, array-key-keyed ones never).
     */
    private function recordMap(RustType $map, string $psalm): void
    {
        if ($this->context === null) {
            return;
        }
        if ($map->params[0]->kind === RustType::ARRAY_KEY) {
            $this->array_key_sites[$this->context][$psalm] = ($this->array_key_sites[$this->context][$psalm] ?? 0) + 1;
        } elseif ($map->params[0]->kind === RustType::INT) {
            $this->int_key_sites[$this->context][$psalm] = ($this->int_key_sites[$this->context][$psalm] ?? 0) + 1;
        }
    }

    private function mapKey(Union $key): RustType
    {
        $k = $this->map($key);
        return match ($k->kind) {
            RustType::INT => RustType::int(),
            RustType::STR => RustType::str(),
            default => RustType::arrayKey(),
        };
    }

    private function mapValue(Union $value): RustType
    {
        $v = $this->map($value);
        if ($v->kind === RustType::NEVER) {
            return $this->mixedRoot('array-value-never (empty array)');
        }
        if ($v->kind === RustType::UNIT && $value->isNever()) {
            return $this->mixedRoot('array-value-never (empty array)');
        }
        return $v;
    }

    private function mapKeyedArray(TKeyedArray $t): RustType
    {
        $props = $t->properties;
        if ($t->is_list) {
            $sequential = true;
            $i = 0;
            $has_optional = false;
            foreach ($props as $k => $v) {
                if ($k !== $i) {
                    $sequential = false;
                }
                if ($v->possibly_undefined) {
                    $has_optional = true;
                }
                $i++;
            }
            if ($sequential && !$has_optional && $t->fallback_params === null && count($props) > 0 && count($props) <= 12) {
                return RustType::tuple(array_map(fn(Union $u) => $this->map($u), array_values($props)));
            }
            $values = array_map(fn(Union $u) => $this->map($u), array_values($props));
            if ($t->fallback_params !== null) {
                $values[] = $this->mapValue($t->fallback_params[1]);
            }
            return RustType::list($this->combine($values));
        }

        // a keyed array with many keys is a map, not a record (a dictionary literal such as the call maps,
        // whose union of thousands of entries would otherwise yield a shape with every parameter name as an
        // optional key): one generic type, computed by Psalm from the combined keys and values
        if (count($props) > 32) {
            $generic = $t->getGenericArrayType();
            $map = RustType::map($this->mapKey($generic->type_params[0]), $this->mapValue($generic->type_params[1]));
            if ($this->context !== null) {
                $id = 'shape with ' . count($props) . ' keys: ' . substr($t->getId(), 0, 160);
                $this->shape_map_sites[$this->context][$id] = ($this->shape_map_sites[$this->context][$id] ?? 0) + 1;
            }
            return $map;
        }

        if ($t->fallback_params !== null) {
            $key_types = [];
            $value_types = [];
            foreach ($props as $k => $v) {
                $key_types[] = is_int($k) ? RustType::int() : RustType::str();
                $value_types[] = $this->map($v);
            }
            $key_types[] = $this->mapKey($t->fallback_params[0]);
            $value_types[] = $this->mapValue($t->fallback_params[1]);
            $key = $this->combine($key_types);
            if ($key->kind !== RustType::INT && $key->kind !== RustType::STR) {
                $key = RustType::arrayKey();
            }
            $map = RustType::map($key, $this->combine($value_types));
            $this->recordMap($map, $t->getId());
            return $map;
        }

        $fields = [];
        foreach ($props as $k => $v) {
            $fields[(string) $k] = [$this->map($v), $v->possibly_undefined];
        }
        $shape = RustType::shape($fields);
        $this->shapes[$shape->mangle()] = $shape;
        return $shape;
    }

    private function mapCallable(TClosure|TCallable $t): RustType
    {
        if ($t->params === null) {
            return RustType::dynCallable();
        }
        $params = [];
        foreach ($t->params as $p) {
            $params[] = $this->map($p->type);
        }
        $ret = $t->return_type ? $this->map($t->return_type) : $this->mixedRoot('callable-no-return');
        $c = RustType::closure($params, $ret);
        $this->closures[$c->mangle()] = $c;
        return $c;
    }

    /**
     * Join many Psalm types into one (the declared Rust type of a variable).
     *
     * @param non-empty-list<Union> $types
     */
    public function join(array $types): Union
    {
        $result = null;
        foreach ($types as $t) {
            $result = $result === null ? $t : Type::combineUnionTypes($result, $t, $this->codebase);
        }
        return $result;
    }
}
