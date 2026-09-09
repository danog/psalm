<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use function array_map;
use function count;
use function implode;
use function md5;
use function strlen;
use function substr;

/**
 * Immutable description of a Rust type used by the generated code.
 *
 * @internal
 */
final class RustType
{
    public const INT = 'int';
    public const FLOAT = 'float';
    public const BOOL = 'bool';
    public const STR = 'str';
    public const UNIT = 'unit';
    public const NEVER = 'never';
    public const MIXED = 'mixed';
    public const ARRAY_KEY = 'array_key';
    public const OPTION = 'option';
    public const LIST = 'list';
    public const MAP = 'map';
    public const TUPLE = 'tuple';
    public const SHAPE = 'shape';
    public const UNION = 'union';
    public const CLASS_ = 'class';
    public const CLOSURE = 'closure';
    public const DYN_CALLABLE = 'dyn_callable';
    public const ANY_OBJECT = 'any_object';
    public const RT_GENERIC = 'rt_generic';
    public const RESOURCE = 'resource';

    /**
     * @param list<RustType> $params  type parameters (option/list/map/tuple/union members/closure params)
     * @param array<string, array{RustType, bool}> $fields shape fields => [type, optional]
     */
    private function __construct(
        public readonly string $kind,
        public readonly array $params = [],
        public readonly string $name = '',
        public readonly array $fields = [],
        public readonly ?RustType $ret = null,
    ) {
    }

    private static array $cache = [];

    private static function intern(RustType $t): RustType
    {
        $k = $t->toRust();
        return self::$cache[$k] ??= $t;
    }

    public static function int(): RustType
    {
        return self::intern(new self(self::INT));
    }
    public static function float(): RustType
    {
        return self::intern(new self(self::FLOAT));
    }
    public static function bool(): RustType
    {
        return self::intern(new self(self::BOOL));
    }
    public static function str(): RustType
    {
        return self::intern(new self(self::STR));
    }
    public static function unit(): RustType
    {
        return self::intern(new self(self::UNIT));
    }
    public static function never(): RustType
    {
        return self::intern(new self(self::NEVER));
    }
    public static function mixed(): RustType
    {
        return self::intern(new self(self::MIXED));
    }
    public static function arrayKey(): RustType
    {
        return self::intern(new self(self::ARRAY_KEY));
    }
    public static function resource(): RustType
    {
        return self::intern(new self(self::RESOURCE));
    }
    public static function anyObject(): RustType
    {
        return self::intern(new self(self::ANY_OBJECT));
    }
    public static function dynCallable(): RustType
    {
        return self::intern(new self(self::DYN_CALLABLE));
    }
    public static function option(RustType $inner): RustType
    {
        if ($inner->kind === self::OPTION) {
            return $inner;
        }
        return self::intern(new self(self::OPTION, [$inner]));
    }
    public static function list(RustType $elem): RustType
    {
        return self::intern(new self(self::LIST, [$elem]));
    }
    public static function map(RustType $key, RustType $value): RustType
    {
        return self::intern(new self(self::MAP, [$key, $value]));
    }
    /** @param list<RustType> $elems */
    public static function tuple(array $elems): RustType
    {
        return self::intern(new self(self::TUPLE, $elems));
    }
    /** @param array<string, array{RustType, bool}> $fields */
    public static function shape(array $fields): RustType
    {
        $t = new self(self::SHAPE, [], '', $fields);
        return self::intern($t);
    }
    /** @param list<RustType> $members */
    public static function union(array $members): RustType
    {
        return self::intern(new self(self::UNION, $members));
    }
    /** @param class-string|string $fqcn */
    public static function class(string $fqcn): RustType
    {
        return self::intern(new self(self::CLASS_, [], $fqcn));
    }
    /** @param list<RustType> $params */
    public static function closure(array $params, RustType $ret): RustType
    {
        return self::intern(new self(self::CLOSURE, $params, '', [], $ret));
    }
    /** @param list<RustType> $params */
    public static function rtGeneric(string $name, array $params): RustType
    {
        return self::intern(new self(self::RT_GENERIC, $params, $name));
    }

    public function is(string $kind): bool
    {
        return $this->kind === $kind;
    }

    public function isOption(): bool
    {
        return $this->kind === self::OPTION;
    }

    public function inner(): RustType
    {
        return $this->params[0];
    }

    public function isCopy(): bool
    {
        return match ($this->kind) {
            self::INT, self::FLOAT, self::BOOL, self::UNIT, self::NEVER => true,
            self::OPTION => $this->params[0]->isCopy(),
            default => false,
        };
    }

    /** Whether the Rust type implements Default (so locals/fields can be initialized without a value). */
    public function hasDefault(): bool
    {
        return match ($this->kind) {
            self::INT, self::FLOAT, self::BOOL, self::UNIT, self::STR, self::MIXED, self::ARRAY_KEY,
            self::OPTION, self::LIST, self::MAP => true,
            self::TUPLE => (static function (array $ps): bool {
                foreach ($ps as $p) {
                    if (!$p->hasDefault()) {
                        return false;
                    }
                }
                return true;
            })($this->params),
            self::SHAPE => (static function (array $fs): bool {
                foreach ($fs as [$t, $opt]) {
                    if (!$opt && !$t->hasDefault()) {
                        return false;
                    }
                }
                return true;
            })($this->fields),
            default => false,
        };
    }

    /** Rust source for the type. */
    public function toRust(): string
    {
        return match ($this->kind) {
            self::INT => 'i64',
            self::FLOAT => 'f64',
            self::BOOL => 'bool',
            self::STR => 'Str',
            self::UNIT => '()',
            self::NEVER => 'Never',
            self::MIXED => 'Mixed',
            self::ARRAY_KEY => 'ArrayKey',
            self::RESOURCE => 'Resource',
            self::ANY_OBJECT => 'AnyObject',
            self::DYN_CALLABLE => 'DynCallable',
            self::OPTION => 'Option<' . $this->params[0]->toRust() . '>',
            self::LIST => 'List<' . $this->params[0]->toRust() . '>',
            self::MAP => 'Map<' . $this->params[0]->toRust() . ', ' . $this->params[1]->toRust() . '>',
            self::TUPLE => '(' . implode(', ', array_map(static fn(RustType $p) => $p->toRust(), $this->params))
                . (count($this->params) === 1 ? ',' : '') . ')',
            self::SHAPE, self::UNION => $this->mangle(),
            self::CLASS_ => Names::classPath($this->name),
            self::CLOSURE => 'Rc<dyn Fn(' . implode(', ', array_map(static fn(RustType $p) => $p->toRust(), $this->params))
                . ') -> Result<' . $this->ret->toRust() . ', Throw>>',
            self::RT_GENERIC => $this->name . '<' . implode(', ', array_map(static fn(RustType $p) => $p->toRust(), $this->params)) . '>',
        };
    }

    /** A valid Rust identifier fragment uniquely describing the type. */
    public function mangle(): string
    {
        return match ($this->kind) {
            self::INT => 'Int',
            self::FLOAT => 'Float',
            self::BOOL => 'Bool',
            self::STR => 'Str',
            self::UNIT => 'Null',
            self::NEVER => 'Never',
            self::MIXED => 'Mixed',
            self::ARRAY_KEY => 'ArrayKey',
            self::RESOURCE => 'Resource',
            self::ANY_OBJECT => 'AnyObject',
            self::DYN_CALLABLE => 'DynCallable',
            self::OPTION => 'Opt_' . $this->params[0]->mangle(),
            self::LIST => 'List_' . $this->params[0]->mangle(),
            self::MAP => 'Map_' . $this->params[0]->mangle() . '_' . $this->params[1]->mangle(),
            self::TUPLE => 'Tup' . count($this->params) . '_' . implode('_', array_map(static fn(RustType $p) => $p->mangle(), $this->params)),
            self::SHAPE => self::shorten('Shape_' . implode('_', array_map(
                static fn(string|int $k, array $f) => Names::typeIdent((string) $k) . ($f[1] ? 'q' : '') . '_' . $f[0]->mangle(),
                array_keys($this->fields),
                $this->fields,
            ))),
            self::UNION => self::shorten('U_' . implode('_or_', array_map(static fn(RustType $p) => $p->mangle(), $this->params))),
            self::CLASS_ => Names::classMangle($this->name),
            self::CLOSURE => self::shorten('Fn' . count($this->params) . '_' . implode('_', array_map(static fn(RustType $p) => $p->mangle(), $this->params)) . '_to_' . $this->ret->mangle()),
            self::RT_GENERIC => $this->name . '_' . implode('_', array_map(static fn(RustType $p) => $p->mangle(), $this->params)),
        };
    }

    private static function shorten(string $s): string
    {
        if (strlen($s) <= 90) {
            return $s;
        }
        return substr($s, 0, 70) . '_' . substr(md5($s), 0, 10);
    }

    /** Variant name of this type when used as a union member. */
    public function variantName(): string
    {
        return match ($this->kind) {
            self::CLASS_ => Names::classMangle($this->name),
            default => $this->mangle(),
        };
    }

    public function __toString(): string
    {
        return $this->toRust();
    }
}
