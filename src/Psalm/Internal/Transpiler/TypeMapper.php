<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use Psalm\Codebase;
use Psalm\Type;
use Psalm\Type\Atomic;
use Psalm\Type\Atomic\Scalar;
use Psalm\Type\Atomic\TArray;
use Psalm\Type\Atomic\TArrayKey;
use Psalm\Type\Atomic\TBool;
use Psalm\Type\Atomic\TCallable;
use Psalm\Type\Atomic\TCallableObject;
use Psalm\Type\Atomic\TClassConstant;
use Psalm\Type\Atomic\TClassString;
use Psalm\Type\Atomic\TClassStringMap;
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
use function strtolower;
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

    /** Class that `static`/`self` types refer to while mapping. */
    public ?string $current_class = null;

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
        'iterator' => 'PhpIterator',
        'iteratoraggregate' => 'IteratorAggregate',
        'traversable' => 'Traversable',
    ];

    public function __construct(
        private readonly Codebase $codebase,
        private readonly Program $program,
    ) {
    }

    public function map(?Union $type): RustType
    {
        if ($type === null) {
            return RustType::mixed();
        }

        $atomics = array_values($type->getAtomicTypes());
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
                return RustType::mixed();
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
                return RustType::mixed();
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

        $union = RustType::union($members);
        $this->unions[$union->mangle()] = $union;
        return $union;
    }

    public function mapAtomic(Atomic $atomic): RustType
    {
        if ($atomic instanceof TInt) {
            return RustType::int();
        }
        if ($atomic instanceof TFloat) {
            return RustType::float();
        }
        if ($atomic instanceof TBool) {
            return RustType::bool();
        }
        if ($atomic instanceof TString || $atomic instanceof TClassString || $atomic instanceof TLiteralClassString
            || $atomic instanceof TTemplateParamClass
        ) {
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
            return RustType::mixed();
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
            return RustType::anyObject();
        }
        if ($atomic instanceof TObject) {
            return RustType::anyObject();
        }
        if ($atomic instanceof TConditional) {
            return $this->combine([$this->map($atomic->if_type), $this->map($atomic->else_type)]);
        }
        if ($atomic instanceof TClassConstant) {
            return RustType::mixed();
        }
        if ($atomic instanceof TClassStringMap) {
            return RustType::map(RustType::str(), $this->map($atomic->value_param));
        }

        $this->unsupported[$atomic::class] = ($this->unsupported[$atomic::class] ?? 0) + 1;
        return RustType::mixed();
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
                $out[] = isset($params[0]) ? $this->map($params[0]) : RustType::mixed();
                $out[] = isset($params[1]) ? $this->map($params[1]) : RustType::mixed();
                foreach ($out as $i => $p) {
                    if ($p->kind === RustType::UNIT || $p->kind === RustType::NEVER) {
                        $out[$i] = RustType::mixed();
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
                $out[] = isset($params[1]) ? $this->map($params[1]) : RustType::mixed();
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
            return RustType::anyObject();
        }
        $fqcn = $this->program->canonicalClassName($atomic->value);
        $model = $this->program->getClass($fqcn);
        if ($model === null || !$model->is_project) {
            // no generated code for this class (a vendor dependency that is not transpiled): dynamic object
            return RustType::anyObject();
        }
        while ($model !== null && $model->crate > $this->current_crate) {
            // a class of a downstream crate (a test subclass seen by inference): the upstream crate can only
            // name its nearest ancestor defined there
            $model = $model->parent;
        }
        if ($model === null || !$model->is_project) {
            return RustType::anyObject();
        }
        return RustType::class($model->fqcn);
    }

    private function mapArray(Union $key, Union $value): RustType
    {
        return RustType::map($this->mapKey($key), $this->mapValue($value));
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
            return RustType::mixed();
        }
        if ($v->kind === RustType::UNIT && $value->isNever()) {
            return RustType::mixed();
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
            return RustType::map($key, $this->combine($value_types));
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
        $ret = $t->return_type ? $this->map($t->return_type) : RustType::mixed();
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
