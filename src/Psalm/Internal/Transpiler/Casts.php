<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use function array_keys;
use function count;
use function implode;

/**
 * Generates conversions between Rust representations of PHP types, recording
 * the trait impls that must be emitted for generated types.
 *
 * @internal
 */
final class Casts
{
    /** @var array<string, array{RustType, RustType}> conversions needing a generated CastTo impl */
    public array $casts = [];

    /** @var array<string, array{RustType, RustType}> instanceof checks needing a generated InstanceOf impl */
    public array $instance_checks = [];

    /** @var array<string, string> diagnostics: unsupported conversions */
    public array $warnings = [];

    private int $tmp = 0;

    public function __construct(
        private readonly Program $program,
    ) {
    }

    public function need(RustType $from, RustType $to): void
    {
        if (!self::isLocal($from) && !self::isLocal($to)) {
            // impls between runtime types cannot live in the generated crate (orphan rules)
            $this->warn($from->toRust() . ' => ' . $to->toRust());
            return;
        }
        if (!$this->record_erasures && ($from->kind === RustType::MIXED || $to->kind === RustType::MIXED)) {
            // a dynamic arm built for a closed hierarchy is never emitted: its Mixed conversions are not demanded
            return;
        }
        $key = $from->toRust() . ' => ' . $to->toRust();
        if (getenv('DBG_NEED') && !isset($this->casts[$key]) && ($from->kind === RustType::MIXED || $to->kind === RustType::MIXED)) {
            $frames = [];
            foreach (array_slice(debug_backtrace(DEBUG_BACKTRACE_IGNORE_ARGS, 8), 1, 7) as $f) {
                $frames[] = ($f['class'] ?? '') . '::' . $f['function'] . ':' . ($f['line'] ?? '?');
            }
            fwrite(STDERR, '[need-mixed] ' . $key . ' <= ' . implode(' < ', $frames) . "\n");
        }
        $this->casts[$key] = [$from, $to];
    }

    /** Demand the `FromData` impls a typed view of a data table needs (generated types nested in `$t`). */
    public function needFromData(RustType $t): void
    {
        switch ($t->kind) {
            case RustType::UNION:
            case RustType::SHAPE:
            case RustType::CLASS_:
                $this->need(RustType::rtGeneric('Data', []), $t);
                break;
            case RustType::TUPLE:
            case RustType::OPTION:
            case RustType::LIST:
            case RustType::MAP:
                // runtime containers (tuples included) read their elements through php-rt's generic impls
                foreach ($t->params as $p) {
                    $this->needFromData($p);
                }
                break;
        }
    }

    /** Types defined by the generated crate (eligible for trait impls). */
    public static function isLocal(RustType $t): bool
    {
        return in_array($t->kind, [RustType::CLASS_, RustType::UNION, RustType::SHAPE, RustType::ANY_OBJECT], true);
    }

    public function needInstanceOf(RustType $subject, RustType $target): void
    {
        if (!$this->record_erasures && $subject->kind === RustType::MIXED) {
            return;
        }
        $this->instance_checks[$subject->toRust() . ' => ' . $target->toRust()] = [$subject, $target];
    }

    private function tmp(): string
    {
        return '__c' . (++$this->tmp);
    }

    /**
     * Lookup-style Option mapping: `None` (key absent) stays `None`, `Some(v)` converts the value
     * (a `Mixed` null becomes an absent key unless the target itself is nullable).
     */
    public function optionMap(string $code, RustType $a, RustType $b): string
    {
        if ($a->toRust() === $b->toRust()) {
            return $code;
        }
        if ($a->kind === RustType::MIXED && $b->kind !== RustType::OPTION && $b->kind !== RustType::MIXED) {
            return $code . '.and_then(|__m| __m.to_option()).map(|__v| ' . $this->convert('__v', $a, $b) . ')';
        }
        return $code . '.map(|__v| ' . $this->convert('__v', $a, $b) . ')';
    }

    /** Emit code converting `$code` (a value of type `$from`) into a value of type `$to`. */
    public function convert(string $code, RustType $from, RustType $to): string
    {
        if ($from === $to || $from->toRust() === $to->toRust()) {
            return $code;
        }
        if ($code === 'self') {
            // the receiver is borrowed; a converted value is owned
            $code = 'self.clone()';
        }
        $fk = $from->kind;
        $tk = $to->kind;

        // Rust generics: a value flows into/out of a generic parameter unchanged (rustc infers/checks T);
        // a generic narrowed by Psalm to a concrete type is downcast (Any) at the narrowing point.
        if ($to->hasGeneric() && !$from->hasGeneric()) {
            return $code; // a concrete value flowing into a generic slot: rustc infers T
        }
        if ($from->hasGeneric() && !$to->hasGeneric()) {
            if ($tk === RustType::MIXED) {
                $this->noteErasure($from);
                return 'cast::<Mixed>(' . $code . ')';
            }
            return 'php_rt::gcast::<' . $to->toRust() . '>(' . $code . ')';
        }
        // both generic (List<T> -> Map<K, T>, T -> Option<T>): the ordinary structural conversion applies
        if ($fk === RustType::NEVER) {
            return 'never(' . $code . ')';
        }
        if ($fk === RustType::RT_GENERIC && $from->name === 'Num') {
            if ($tk === RustType::FLOAT) {
                return $code . '.to_f64()';
            }
            if ($tk === RustType::INT) {
                return $code . '.to_i64()';
            }
        }
        if ($fk === RustType::RT_GENERIC && $from->name === 'OptValue') {
            // a getopt value into the declared option union (or Mixed)
            if ($tk === RustType::MIXED) {
                return $code . '.to_mixed()';
            }
            if ($tk === RustType::UNION) {
                $this->need($from, $to);
                return 'cast::<' . $to->toRust() . '>(' . $code . ')';
            }
            if ($tk === RustType::STR) {
                return 'php_rt::ToStr::to_php_str(&' . $code . ')';
            }
            if ($tk === RustType::BOOL) {
                return 'truthy(&' . $code . ')';
            }
        }
        if ($fk === RustType::RT_GENERIC && $from->name === 'Scalar') {
            // a constant's value: null becomes an absent Option, the scalar kinds their typed forms
            if ($tk === RustType::OPTION) {
                return '(match ' . $code . ' { php_rt::Scalar::Null => None, __s => Some(' . $this->convert('__s', $from, $to->inner()) . ') })';
            }
            if ($tk === RustType::MIXED) {
                return $code . '.to_mixed()';
            }
            if ($tk === RustType::UNION) {
                $this->need($from, $to);
                return 'cast::<' . $to->toRust() . '>(' . $code . ')';
            }
            if ($tk === RustType::STR) {
                return 'php_rt::ToStr::to_php_str(&' . $code . ')';
            }
            if ($tk === RustType::INT) {
                return 'php_rt::ToInt::to_php_int(&' . $code . ')';
            }
            if ($tk === RustType::FLOAT) {
                return 'php_rt::ToFloat::to_php_float(&' . $code . ')';
            }
            if ($tk === RustType::BOOL) {
                return 'truthy(&' . $code . ')';
            }
        }
        if ($tk === RustType::UNIT) {
            return '{ let _ = ' . $code . '; }';
        }
        if ($tk === RustType::NEVER) {
            return '{ let _ = ' . $code . '; unreachable!() }';
        }
        if ($tk === RustType::MIXED) {
            $this->noteErasure($from);
            if ($fk === RustType::UNIT) {
                // keep the side effects of the unit-typed expression (a call returning void)
                return $code === '()' ? 'Mixed::Null' : '{ let _ = ' . $code . '; Mixed::Null }';
            }
            if ($fk === RustType::TUPLE) {
                $t = $this->tmp();
                $parts = [];
                foreach ($from->params as $i => $p) {
                    $parts[] = '__m.push(' . $this->convert($t . '.' . $i, $p, RustType::mixed()) . ');';
                }
                return '{ let ' . $t . ' = ' . $code . '; let mut __m: Map<ArrayKey, Mixed> = Map::new(); ' . implode(' ', $parts) . ' Mixed::Arr(__m) }';
            }
            if ($fk === RustType::CLOSURE) {
                return 'Mixed::Closure(Rc::new(' . $this->convert($code, $from, RustType::dynCallable()) . '))';
            }
            if ($fk === RustType::DYN_CALLABLE) {
                return 'cast::<Mixed>(' . $code . ')';
            }
            if ($fk === RustType::RESOURCE) {
                return 'cast::<Mixed>(' . $code . ')';
            }
            if ($fk === RustType::OPTION && in_array($from->inner()->kind, [RustType::CLOSURE, RustType::TUPLE, RustType::DYN_CALLABLE], true)) {
                return '(match ' . $code . ' { Some(__o) => ' . $this->convert('__o', $from->inner(), $to) . ', None => Mixed::Null })';
            }
            // A raw closure (`Rc<dyn Fn ...>`) has no `CastTo<Mixed>` impl (orphan rules), so the blanket
            // container→Mixed cast fails for a Map/List of closures: melt the elements to Mixed first
            // (element-wise conversion emits `Mixed::Closure(..)` inline), then cast the plain container.
            if (($fk === RustType::MAP || $fk === RustType::LIST) && $this->containsClosure($from)) {
                $melted = $fk === RustType::MAP ? RustType::map($from->params[0], RustType::mixed()) : RustType::list(RustType::mixed());
                return $this->convert($this->convert($code, $from, $melted), $melted, RustType::mixed());
            }
            $this->needMixedFrom($from);
            return 'cast::<Mixed>(' . $code . ')';
        }
        // optional nullable shape fields are stored as Option<Option<T>>
        if ($from->isNestedOption()) {
            if ($to->isNestedOption()) {
                return $code . '.map(|__v| ' . $this->convert('__v', $from->inner(), $to->inner()) . ')';
            }
            // read as a value: absent and null both read as null
            return $this->convert($code . '.flatten()', $from->inner(), $to);
        }
        if ($to->isNestedOption()) {
            if ($fk === RustType::UNIT) {
                return '{ let _ = ' . $code . '; Some(None) }';
            }
            // storing a value: the key is present (possibly with null)
            return 'Some(' . $this->convert($code, $from, $to->inner()) . ')';
        }
        if ($fk === RustType::OPTION && $tk === RustType::OPTION) {
            $a = $from->inner();
            $b = $to->inner();
            if ($a->toRust() === $b->toRust()) {
                return $code;
            }
            if ($a->kind === RustType::MIXED) {
                return $code . '.and_then(|__m| __m.to_option()).map(|v| ' . $this->convert('v', $a, $b) . ')';
            }
            return $code . '.map(|v| ' . $this->convert('v', $a, $b) . ')';
        }
        if ($fk === RustType::OPTION && $tk === RustType::UNION && ($this->hasUnit($to, 'False') || $this->hasUnit($to, 'True'))) {
            $unit = $this->hasUnit($to, 'False') ? 'False' : 'True';
            return '(match ' . $code . ' { Some(__o) => ' . $this->convert('__o', $from->inner(), $to) . ', None => ' . $to->mangle() . '::' . $unit . ' })';
        }
        if ($fk === RustType::OPTION) {
            $inner = $from->inner();
            if ($tk === RustType::CLASS_) {
                $tc = $this->program->classOf($to);
                if ($tc !== null && !$tc->isLeaf() && $tc->has_downstream) {
                    return '(match ' . $code . ' { Some(__o) => ' . $this->convert('__o', $inner, $to) . ', None => ' . $to->toRust() . '::Other__(Mixed::Null) })';
                }
                if ($tc !== null && !$tc->isLeaf()) {
                    // closed handle: no null-carrying escape variant, so a null narrowed to it is a docblock lie
                    return '(match ' . $code . ' { Some(__o) => ' . $this->convert('__o', $inner, $to) . ', None => panic!(' . Names::rustStringLiteral('null where ' . $tc->fqcn . ' expected') . ') })';
                }
            }
            if ($to->hasDefault() && $tk !== RustType::CLASS_) {
                if ($inner->toRust() === $to->toRust()) {
                    return $code . '.unwrap_or_default()';
                }
                return '(match ' . $code . ' { Some(__o) => ' . $this->convert('__o', $inner, $to) . ', None => ' . $this->defaultOf($to) . ' })';
            }
            return $this->convert($code . '.unwrap()', $inner, $to);
        }
        if ($tk === RustType::OPTION) {
            if ($fk === RustType::UNIT) {
                return '{ let _ = ' . $code . '; None::<' . $to->inner()->toRust() . '> }';
            }
            if ($fk === RustType::MIXED) {
                $inner = $to->inner();
                if (self::isLocal($inner)) {
                    $this->needMixedTo($inner);
                }
                return $code . '.to_option().map(|__m| ' . $this->convert('__m', RustType::mixed(), $inner) . ')';
            }
            if ($fk === RustType::DYN_CALLABLE && $to->inner()->kind === RustType::DYN_CALLABLE) {
                return 'DynCallable::into_option(' . $code . ')';
            }
            if ($fk === RustType::UNION) {
                // A union may itself carry null via an Option-typed member (`Opt_*`, e.g. resolveType's `?Node` typed
                // `U_Opt_Node_or_Name`). `Some(cast(union -> inner))` would force Some and cast a null into the now-
                // closed inner union -> panic. Match instead: flatten the Option member (preserving None), Some() the
                // rest — no Mixed round-trip.
                $mangle = $from->mangle();
                $arms = [];
                foreach ($from->params as $m) {
                    $isUnit = $m->kind === RustType::RT_GENERIC && str_starts_with($m->name, '__unit_');
                    if ($isUnit) {
                        $arms[] = $mangle . '::' . substr($m->name, 7) . ' => None';
                    } elseif ($m->kind === RustType::OPTION) {
                        $arms[] = $mangle . '::' . $m->variantName() . '(__x) => ' . $this->convert('__x', $m, $to);
                    } else {
                        $arms[] = $mangle . '::' . $m->variantName() . '(__x) => Some(' . $this->convert('__x', $m, $to->inner()) . ')';
                    }
                }
                return '(match ' . $code . ' { ' . implode(', ', $arms) . ' })';
            }
            return 'Some(' . $this->convert($code, $from, $to->inner()) . ')';
        }
        if ($fk === RustType::UNIT) {
            if ($tk === RustType::DYN_CALLABLE) {
                // calling it raises the same Error PHP would
                return '{ let _ = ' . $code . '; DynCallable::null_callable() }';
            }
            // null into a non-nullable type: use a default
            if ($to->hasDefault()) {
                return '{ let _ = ' . $code . '; <' . $to->toRust() . '>::default() }';
            }
            $this->warn('null => ' . $to->toRust());
            return '{ let _ = ' . $code . '; unreachable!("null used as ' . $to->toRust() . '") }';
        }
        if ($fk === RustType::MIXED) {
            if ($tk === RustType::TUPLE) {
                $t = $this->tmp();
                $parts = [];
                foreach ($to->params as $i => $p) {
                    $parts[] = $this->convert($t . '.get(' . $i . ').cloned().unwrap_or_default()', RustType::mixed(), $p);
                }
                return '{ let ' . $t . ' = cast::<List<Mixed>>(' . $code . '); (' . implode(', ', $parts) . (count($parts) === 1 ? ',' : '') . ') }';
            }
            if ($tk === RustType::CLOSURE) {
                return $this->convert('to_callable(&' . $code . ')', RustType::dynCallable(), $to);
            }
            if ($tk === RustType::DYN_CALLABLE) {
                return 'to_callable(&' . $code . ')';
            }
            $this->needMixedTo($to);
            return 'cast::<' . $to->toRust() . '>(' . $code . ')';
        }

        // containers
        if ($fk === RustType::LIST && $tk === RustType::LIST) {
            return $code . '.map_elems(|v| ' . $this->convert('v', $from->inner(), $to->inner()) . ')';
        }
        if ($fk === RustType::MAP && $tk === RustType::MAP) {
            [$k1, $v1] = $from->params;
            [$k2, $v2] = $to->params;
            if ($k1->toRust() === $k2->toRust()) {
                return $code . '.map_values(|v| ' . $this->convert('v', $v1, $v2) . ')';
            }
            return $code . '.map_entries(|k, v| (' . $this->convert('k', $k1, $k2) . ', ' . $this->convert('v', $v1, $v2) . '))';
        }
        if ($fk === RustType::LIST && $tk === RustType::MAP) {
            [$k2, $v2] = $to->params;
            $list = $this->convert($code, $from, RustType::list($v2));
            if ($k2->kind === RustType::STR) {
                return $list . '.to_map().map_entries(|k, v| (cast::<Str>(k), v))';
            }
            return 'cast::<' . $to->toRust() . '>(' . $list . ')';
        }
        if ($fk === RustType::MAP && $tk === RustType::LIST) {
            $v1 = $from->params[1];
            $map = $this->convert($code, $from, RustType::map($from->params[0], $to->inner()));
            return 'cast::<' . $to->toRust() . '>(' . $map . ')';
        }
        if ($fk === RustType::TUPLE && $tk === RustType::LIST) {
            $t = $this->tmp();
            $parts = [];
            foreach ($from->params as $i => $p) {
                $parts[] = $this->convert($t . '.' . $i, $p, $to->inner());
            }
            return '{ let ' . $t . ' = ' . $code . '; List::from_vec(vec![' . implode(', ', $parts) . ']) }';
        }
        if ($fk === RustType::TUPLE && $tk === RustType::MAP) {
            $list = $this->convert($code, $from, RustType::list($to->params[1]));
            return $this->convert($list, RustType::list($to->params[1]), $to);
        }
        if ($fk === RustType::LIST && $tk === RustType::TUPLE) {
            $t = $this->tmp();
            $parts = [];
            foreach ($to->params as $i => $p) {
                $parts[] = $this->convert($t . '.idx(' . $i . ').clone()', $from->inner(), $p);
            }
            return '{ let ' . $t . ' = ' . $code . '; (' . implode(', ', $parts) . (count($parts) === 1 ? ',' : '') . ') }';
        }
        if ($fk === RustType::MAP && $tk === RustType::TUPLE) {
            $list = $this->convert($code, $from, RustType::list($from->params[1]));
            return $this->convert($list, RustType::list($from->params[1]), $to);
        }
        if ($fk === RustType::TUPLE && $tk === RustType::TUPLE) {
            $t = $this->tmp();
            $parts = [];
            foreach ($to->params as $i => $p) {
                if (isset($from->params[$i])) {
                    $parts[] = $this->convert($t . '.' . $i, $from->params[$i], $p);
                } else {
                    $parts[] = $this->defaultOf($p);
                }
            }
            return '{ let ' . $t . ' = ' . $code . '; (' . implode(', ', $parts) . (count($parts) === 1 ? ',' : '') . ') }';
        }
        if ($fk === RustType::SHAPE && $tk === RustType::SHAPE) {
            $t = $this->tmp();
            $parts = [];
            foreach ($to->fields as $name => [$ft, $opt]) {
                $rn = Names::field($name);
                if (isset($from->fields[$name])) {
                    [$st, $sopt] = $from->fields[$name];
                    $src = $t . '.' . $rn;
                    $parts[] = $rn . ': ' . ($sopt && $opt
                        ? $this->optionMap($src, $st, $ft)
                        : $this->convert($src, RustType::shapeField($st, $sopt), RustType::shapeField($ft, $opt)));
                } elseif ($opt) {
                    $parts[] = $rn . ': None';
                } else {
                    $parts[] = $rn . ': ' . $this->defaultOf($ft);
                }
            }
            return '{ let ' . $t . ' = ' . $code . '; ' . $to->mangle() . ' { ' . implode(', ', $parts) . ' } }';
        }
        if ($fk === RustType::SHAPE && ($tk === RustType::MAP || $tk === RustType::LIST)) {
            $t = $this->tmp();
            $out = '{ let ' . $t . ' = ' . $code . '; let mut __m: ' . $to->toRust() . ' = Default::default(); ';
            $vt = $tk === RustType::MAP ? $to->params[1] : $to->inner();
            foreach ($from->fields as $name => [$ft, $opt]) {
                $rn = Names::field($name);
                $key = $tk === RustType::MAP ? $this->keyLiteral($name, $to->params[0]) : null;
                $ins = $tk === RustType::MAP ? '__m.insert(' . $key . ', ' : '__m.push(';
                if ($opt) {
                    $out .= 'if let Some(v) = ' . $t . '.' . $rn . ' { ' . $ins . $this->convert('v', $ft, $vt) . '); } ';
                } else {
                    $out .= $ins . $this->convert($t . '.' . $rn, $ft, $vt) . '); ';
                }
            }
            return $out . '__m }';
        }
        if ($fk === RustType::MAP && $tk === RustType::SHAPE) {
            $t = $this->tmp();
            $parts = [];
            $vt = $from->params[1];
            foreach ($to->fields as $name => [$ft, $opt]) {
                $rn = Names::field($name);
                $key = $this->keyLookup($name, $from->params[0]);
                if ($opt) {
                    $parts[] = $rn . ': ' . $this->optionMap($t . '.get(' . $key . ').cloned()', $vt, $ft);
                } else {
                    $parts[] = $rn . ': ' . $this->convert($t . '.idx(' . $key . ').clone()', $vt, $ft);
                }
            }
            return '{ let ' . $t . ' = ' . $code . '; ' . $to->mangle() . ' { ' . implode(', ', $parts) . ' } }';
        }
        if ($fk === RustType::LIST && $tk === RustType::SHAPE) {
            $t = $this->tmp();
            $parts = [];
            $vt = $from->inner();
            foreach ($to->fields as $name => [$ft, $opt]) {
                $rn = Names::field($name);
                $i = (int) $name;
                if ($opt) {
                    $parts[] = $rn . ': ' . $this->optionMap($t . '.get(' . $i . ').cloned()', $vt, $ft);
                } else {
                    $parts[] = $rn . ': ' . $this->convert($t . '.idx(' . $i . ').clone()', $vt, $ft);
                }
            }
            return '{ let ' . $t . ' = ' . $code . '; ' . $to->mangle() . ' { ' . implode(', ', $parts) . ' } }';
        }
        if ($fk === RustType::SHAPE && $tk === RustType::TUPLE) {
            $t = $this->tmp();
            $parts = [];
            foreach ($to->params as $i => $p) {
                $name = (string) $i;
                if (isset($from->fields[$name])) {
                    [$st, $sopt] = $from->fields[$name];
                    $parts[] = $this->convert($t . '.' . Names::field($name), RustType::shapeField($st, $sopt), $p);
                } else {
                    $parts[] = $this->defaultOf($p);
                }
            }
            return '{ let ' . $t . ' = ' . $code . '; (' . implode(', ', $parts) . (count($parts) === 1 ? ',' : '') . ') }';
        }
        if ($fk === RustType::TUPLE && $tk === RustType::SHAPE) {
            $t = $this->tmp();
            $parts = [];
            foreach ($to->fields as $name => [$ft, $opt]) {
                $i = (int) $name;
                $rn = Names::field($name);
                if (isset($from->params[$i])) {
                    $parts[] = $rn . ': ' . $this->convert($t . '.' . $i, $from->params[$i], RustType::shapeField($ft, $opt));
                } elseif ($opt) {
                    $parts[] = $rn . ': None';
                } else {
                    $parts[] = $rn . ': ' . $this->defaultOf($ft);
                }
            }
            return '{ let ' . $t . ' = ' . $code . '; ' . $to->mangle() . ' { ' . implode(', ', $parts) . ' } }';
        }

        // Sym (interned identifier) converts like Str: direct to Str/Mixed (php-rt CastTo impls), else via Str
        if ($fk === RustType::SYM) {
            if ($tk === RustType::STR) {
                return 'cast::<Str>(' . $code . ')';
            }
            if ($tk === RustType::MIXED) {
                return 'cast::<Mixed>(' . $code . ')';
            }
            return $this->convert('cast::<Str>(' . $code . ')', RustType::str(), $to);
        }
        if ($tk === RustType::SYM) {
            if ($fk === RustType::STR) {
                return 'cast::<Sym>(' . $code . ')';
            }
            if ($fk === RustType::MIXED) {
                return 'cast::<Sym>(' . $code . ')';
            }
            return 'cast::<Sym>(' . $this->convert($code, $from, RustType::str()) . ')';
        }
        // scalars
        if ($fk === RustType::INT && $tk === RustType::FLOAT) {
            return '(' . $code . ' as f64)';
        }
        if ($fk === RustType::FLOAT && $tk === RustType::INT) {
            return 'cast::<i64>(' . $code . ')';
        }
        if (($fk === RustType::INT || $fk === RustType::STR || $fk === RustType::BOOL || $fk === RustType::FLOAT || $fk === RustType::ARRAY_KEY)
            && ($tk === RustType::INT || $tk === RustType::STR || $tk === RustType::BOOL || $tk === RustType::FLOAT || $tk === RustType::ARRAY_KEY)
        ) {
            return 'cast::<' . $to->toRust() . '>(' . $code . ')';
        }

        // unions
        if ($tk === RustType::UNION && $fk === RustType::OPTION && !$this->hasUnit($to, 'Null')) {
            // Option<X> into a union without a null member: the value must be present
            return '(match ' . $code . ' { Some(__o) => ' . $this->convert('__o', $from->inner(), $to) . ', None => panic!(' . Names::rustStringLiteral('null where ' . $to->toRust() . ' expected') . ') })';
        }
        if ($tk === RustType::UNION) {
            $member = $this->pickMember($to, $from);
            if ($member !== null) {
                if ($from->kind === RustType::UNION) {
                    $this->need($from, $to);
                    return 'cast::<' . $to->toRust() . '>(' . $code . ')';
                }
                return $to->mangle() . '::' . $member->variantName() . '(' . $this->convert($code, $from, $member) . ')';
            }
            if ($fk === RustType::UNION) {
                $this->need($from, $to);
                return 'cast::<' . $to->toRust() . '>(' . $code . ')';
            }
            if ($fk === RustType::BOOL) {
                // bool into a union with True/False unit variants
                $has_t = $this->hasUnit($to, 'True');
                $has_f = $this->hasUnit($to, 'False');
                if ($has_t && $has_f) {
                    return '(if ' . $code . ' { ' . $to->mangle() . '::True } else { ' . $to->mangle() . '::False })';
                }
                if ($has_f) {
                    return '{ let _ = ' . $code . '; ' . $to->mangle() . '::False }';
                }
                if ($has_t) {
                    return '{ let _ = ' . $code . '; ' . $to->mangle() . '::True }';
                }
            }
            $this->need($from, $to);
            return 'cast::<' . $to->toRust() . '>(' . $code . ')';
        }
        if ($fk === RustType::UNION) {
            if ($tk === RustType::BOOL && ($this->hasUnit($from, 'True') || $this->hasUnit($from, 'False'))) {
                $this->need($from, $to);
                return 'cast::<bool>(' . $code . ')';
            }
            if (($tk === RustType::LIST || $tk === RustType::MAP || $tk === RustType::OPTION) && $this->program->typeCrate($to) > $this->program->typeCrate($from)) {
                // the target is a runtime container naming a class of a later crate: no crate may write the
                // impl (orphan rules), so the narrowing is inlined through the matching container member
                $arms = [];
                foreach ($from->params as $m) {
                    if ($m->kind === RustType::LIST || $m->kind === RustType::MAP || $m->kind === RustType::OPTION || $m->kind === RustType::MIXED) {
                        $arms[] = $from->mangle() . '::' . $m->variantName() . '(__v) => ' . $this->convert('__v', $m, $to);
                    }
                }
                $arms[] = '_ => panic!(' . Names::rustStringLiteral('cannot narrow ' . $from->mangle() . ' into ' . $to->toRust()) . ')';
                return '(match ' . $code . ' { ' . implode(', ', $arms) . ' })';
            }
            $this->need($from, $to);
            return 'cast::<' . $to->toRust() . '>(' . $code . ')';
        }

        // classes
        if ($fk === RustType::CLASS_ && $tk === RustType::CLASS_) {
            $this->need($from, $to);
            return 'cast::<' . $to->toRust() . '>(' . $code . ')';
        }
        if (($fk === RustType::CLASS_ && $tk === RustType::ANY_OBJECT) || ($fk === RustType::ANY_OBJECT && $tk === RustType::CLASS_)) {
            $this->need($from, $to);
            return 'cast::<' . $to->toRust() . '>(' . $code . ')';
        }
        if ($fk === RustType::CLOSURE && $tk === RustType::CLOSURE) {
            return $this->convertClosure($code, $from, $to);
        }
        if ($fk === RustType::STR && ($tk === RustType::CLOSURE || $tk === RustType::DYN_CALLABLE)) {
            // a function name used as a callable: functions are never looked up by name (closed world)
            $this->warn('string used as a callable');
            return $this->convert('to_callable(&Mixed::Str(' . $code . '))', RustType::dynCallable(), $to);
        }
        if ($fk === RustType::CLOSURE && $tk === RustType::DYN_CALLABLE) {
            $t = $this->tmp();
            $n = count($from->params);
            $args = [];
            foreach ($from->params as $i => $p) {
                $args[] = $this->convert('__a[' . $i . '].clone()', RustType::mixed(), $p);
            }
            return '{ let ' . $t . ' = ' . $code . '; DynCallable::new(' . $n . ', move |__a: Vec<Mixed>| -> Mixed { '
                . $this->convert($t . '(' . implode(', ', $args) . ')', $from->ret, RustType::mixed()) . ' }) }';
        }
        if ($fk === RustType::DYN_CALLABLE && $tk === RustType::CLOSURE) {
            $t = $this->tmp();
            $params = [];
            $args = [];
            foreach ($to->params as $i => $p) {
                $params[] = '__p' . $i . ': ' . $p->toRust();
                $args[] = $this->convert('__p' . $i, $p, RustType::mixed());
            }
            return '{ let ' . $t . ' = ' . $code . '; Rc::new(move |' . implode(', ', $params) . '| -> ' . $to->ret->toRust() . ' { '
                . $this->convert($t . '.call(vec![' . implode(', ', $args) . '])', RustType::mixed(), $to->ret) . ' }) as ' . $to->toRust() . ' }';
        }
        if ($fk === RustType::RT_GENERIC && $from->name === 'Generator' && ($tk === RustType::MAP || $tk === RustType::LIST)) {
            [$gk, $gv] = $from->params;
            if ($tk === RustType::LIST) {
                return $this->convert($code . '.into_list()', RustType::list($gv), $to);
            }
            return $this->convert($code . '.into_map()', RustType::map($gk->kind === RustType::INT ? RustType::int() : ($gk->kind === RustType::STR ? RustType::str() : RustType::arrayKey()), $gv), $to);
        }
        if ($tk === RustType::RT_GENERIC && $to->name === 'Generator' && ($fk === RustType::MAP || $fk === RustType::LIST)) {
            [$gk, $gv] = $to->params;
            if ($fk === RustType::LIST) {
                return 'Generator::from_list(' . $this->convert($code, $from, RustType::list($gv)) . ')';
            }
            return 'Generator::from_map(' . $this->convert($code, $from, RustType::map($from->params[0], $gv)) . ')';
        }

        $this->warn($from->toRust() . ' => ' . $to->toRust());
        if (!self::isLocal($from) && !self::isLocal($to)) {
            return '{ let _ = ' . $code . '; unimplemented!(' . Names::rustStringLiteral('cast ' . $from->toRust() . ' => ' . $to->toRust()) . ') }';
        }
        $this->need($from, $to);
        return 'cast::<' . $to->toRust() . '>(' . $code . ')';
    }

    private function convertClosure(string $code, RustType $from, RustType $to): string
    {
        $t = $this->tmp();
        $params = [];
        $args = [];
        foreach ($to->params as $i => $p) {
            $params[] = '__p' . $i . ': ' . $p->toRust();
            if (isset($from->params[$i])) {
                $args[] = $this->convert('__p' . $i, $p, $from->params[$i]);
            }
        }
        // extra params the source expects but the target does not provide
        for ($i = count($to->params); $i < count($from->params); $i++) {
            $args[] = $this->defaultOf($from->params[$i]);
        }
        $call = $t . '(' . implode(', ', $args) . ')';
        // a mixed return flowing into a scalar-returning callable is coerced, as PHP does for callbacks
        $body = match (true) {
            $from->ret->kind === RustType::MIXED && $to->ret->kind === RustType::STR => 'to_str(&' . $call . ')',
            $from->ret->kind === RustType::MIXED && $to->ret->kind === RustType::INT => 'to_num(&' . $call . ').to_i64()',
            $from->ret->kind === RustType::MIXED && $to->ret->kind === RustType::FLOAT => 'to_num(&' . $call . ').to_f64()',
            $from->ret->kind === RustType::MIXED && $to->ret->kind === RustType::BOOL => 'truthy(&' . $call . ')',
            default => $this->convert($call, $from->ret, $to->ret),
        };
        return '{ let ' . $t . ' = ' . $code . '; Rc::new(move |' . implode(', ', $params) . '| -> ' . $to->ret->toRust() . ' { ' . $body . ' }) as ' . $to->toRust() . ' }';
    }

    public function hasUnit(RustType $union, string $name): bool
    {
        foreach ($union->params as $m) {
            if ($m->kind === RustType::RT_GENERIC && $m->name === '__unit_' . $name) {
                return true;
            }
        }
        return false;
    }

    /** Find the union member a value of type `$from` should be stored in. */
    public function pickMember(RustType $union, RustType $from): ?RustType
    {
        foreach ($union->params as $m) {
            if ($m->toRust() === $from->toRust()) {
                return $m;
            }
        }
        // class into an ancestor member
        if ($from->kind === RustType::CLASS_) {
            $fc = $this->program->classOf($from);
            foreach ($union->params as $m) {
                if ($m->kind === RustType::CLASS_ && $fc !== null) {
                    $mc = $this->program->classOf($m);
                    if ($mc !== null && $fc->isSubclassOf($mc)) {
                        return $m;
                    }
                }
            }
            foreach ($union->params as $m) {
                if ($m->kind === RustType::ANY_OBJECT) {
                    return $m;
                }
            }
        }
        if ($from->kind === RustType::INT || $from->kind === RustType::STR) {
            foreach ($union->params as $m) {
                if ($m->kind === RustType::ARRAY_KEY) {
                    return $m;
                }
            }
        }
        if ($from->kind === RustType::INT) {
            foreach ($union->params as $m) {
                if ($m->kind === RustType::FLOAT) {
                    return $m;
                }
            }
        }
        // structural container compatibility: pick the unique member of the same kind
        $same = [];
        foreach ($union->params as $m) {
            if ($m->kind === $from->kind && in_array($m->kind, [RustType::LIST, RustType::MAP, RustType::TUPLE, RustType::SHAPE, RustType::CLOSURE, RustType::OPTION], true)) {
                $same[] = $m;
            }
        }
        if (count($same) === 1) {
            return $same[0];
        }
        if ($from->kind === RustType::TUPLE || $from->kind === RustType::SHAPE) {
            foreach ($union->params as $m) {
                if ($m->kind === RustType::LIST || $m->kind === RustType::MAP) {
                    return $m;
                }
            }
        }
        if ($from->kind === RustType::LIST) {
            foreach ($union->params as $m) {
                if ($m->kind === RustType::MAP) {
                    return $m;
                }
            }
        }
        return null;
    }

    /**
     * Code iterating a PHP object (Iterator / IteratorAggregate) as owned (key, value) pairs.
     *
     * @return array{string, RustType, RustType}|null iterator expression, key type, value type
     */
    public function objectPairs(ClassModel $cls, string $code): ?array
    {
        $get = $this->program->findMethod($cls, 'getiterator');
        if ($get !== null) {
            $inner = $get->return_type;
            $sub = $code . '.' . $get->rustName() . '()';
            if ($inner->kind === RustType::RT_GENERIC) {
                return [$sub . '.into_pairs().into_iter()', $inner->params[0] ?? RustType::mixed(), $inner->params[1] ?? RustType::mixed()];
            }
            if ($inner->kind === RustType::CLASS_) {
                $ic = $this->program->classOf($inner);
                if ($ic !== null && $ic !== $cls) {
                    return $this->objectPairs($ic, '(' . $sub . ')');
                }
            }
            if ($inner->kind === RustType::LIST) {
                return [$sub . '.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v))', RustType::int(), $inner->inner()];
            }
            if ($inner->kind === RustType::MAP) {
                return [$sub . '.into_iter()', $inner->params[0], $inner->params[1]];
            }
        }
        $current = $this->program->findMethod($cls, 'current');
        $key = $this->program->findMethod($cls, 'key');
        $rewind = $this->program->findMethod($cls, 'rewind');
        $valid = $this->program->findMethod($cls, 'valid');
        $next = $this->program->findMethod($cls, 'next');
        if ($current !== null && $key !== null && $rewind !== null && $valid !== null && $next !== null) {
            $it = '{ let __it = ' . $code . '; iterate_php_iterator(|| { __it.' . $rewind->rustName() . '(); }, || __it.' . $valid->rustName() . '(), || { __it.' . $next->rustName() . '(); }, || __it.' . $key->rustName() . '(), || __it.' . $current->rustName() . '()) }';
            return [$it, $key->return_type, $current->return_type];
        }
        return null;
    }

    public function defaultOf(RustType $t): string
    {
        if ($t->hasDefault()) {
            return '<' . $t->toRust() . '>::default()';
        }
        $this->warn('default of ' . $t->toRust());
        return 'unreachable!("no default for ' . $t->toRust() . '")';
    }

    private function keyLiteral(string|int $name, RustType $key_type): string
    {
        $name = (string) $name;
        return match ($key_type->kind) {
            RustType::INT => (string) (int) $name,
            RustType::STR => Names::strLit($name),
            default => 'ArrayKey::from(' . Names::strLit($name) . ')',
        };
    }

    private function keyLookup(string|int $name, RustType $key_type): string
    {
        $name = (string) $name;
        return match ($key_type->kind) {
            RustType::INT => '&' . (int) $name,
            RustType::STR => '&' . Names::strLit($name),
            default => '&ArrayKey::from(' . Names::strLit($name) . ')',
        };
    }

    /**
     * Whether the type is or contains a raw closure (`Rc<dyn Fn ...>`), which has no `CastTo<Mixed>` impl.
     * `$seen` guards against recursive unions (a union whose list member points back to itself).
     */
    private function containsClosure(RustType $t, array &$seen = []): bool
    {
        $k = $t->toRust();
        if (isset($seen[$k])) {
            return false;
        }
        $seen[$k] = true;
        switch ($t->kind) {
            case RustType::CLOSURE:
                return true;
            case RustType::LIST:
            case RustType::OPTION:
                return $this->containsClosure($t->inner(), $seen);
            case RustType::MAP:
                return $this->containsClosure($t->params[1], $seen);
            case RustType::TUPLE:
            case RustType::UNION:
                foreach ($t->params as $p) {
                    if ($this->containsClosure($p, $seen)) {
                        return true;
                    }
                }
                return false;
            case RustType::SHAPE:
                foreach ($t->fields as [$ft]) {
                    if ($this->containsClosure($ft, $seen)) {
                        return true;
                    }
                }
                return false;
            default:
                return false;
        }
    }

    private function needMixedFrom(RustType $from): void
    {
        switch ($from->kind) {
            case RustType::CLASS_:
            case RustType::UNION:
            case RustType::SHAPE:
            case RustType::ANY_OBJECT:
            case RustType::CLOSURE:
            case RustType::TUPLE:
                $this->need($from, RustType::mixed());
                break;
            case RustType::OPTION:
            case RustType::LIST:
                $this->needMixedFrom($from->inner());
                break;
            case RustType::MAP:
                $this->needMixedFrom($from->params[1]);
                break;
        }
    }

    /**
     * Every type a body erases to Mixed, with the first member that does it: the objects reachable
     * through these conversions are the only ones the dynamic object protocol (props/get_prop/call_method)
     * can ever be applied to. Keyed by Rust type; value = [type, context].
     *
     * @var array<string, array{RustType, ?string}>
     */
    public array $erasures = [];

    /** False while emitting code that is generated but unreachable (no erasure is recorded). */
    public bool $record_erasures = true;

    public function noteErasure(RustType $from): void
    {
        if (!$this->record_erasures) {
            return;
        }
        $k = $from->toRust();
        if (!isset($this->erasures[$k])) {
            $this->erasures[$k] = [$from, $this->program->types->context];
        }
    }

    /**
     * Classes whose objects can reach the runtime's dynamic protocol: the closure of the recorded erasures
     * over union members, shape fields, containers, class fields (props() erases every field) and
     * subclasses. An `object` or generic value erased to Mixed may hold any class ("all").
     *
     * @return array{all: ?string, classes: array<string, true>, keepers: list<string>}
     */
    /** `get_class($v)` / `$v::class` without erasing the object; null when the type has no static class name. */
    public function classNameOf(string $code, RustType $t): ?string
    {
        if ($t->kind === RustType::OPTION) {
            $inner = $this->classNameOf($code . '.clone().expect("null where object expected")', $t->inner());
            return $inner;
        }
        if ($t->kind === RustType::CLASS_ || $t->kind === RustType::ANY_OBJECT) {
            return 'Str::from_static(php_rt::PhpObject::class_name(&' . $code . '))';
        }
        if ($t->kind === RustType::UNION && self::unionHasObject($t)) {
            return 'Str::from_static(' . $code . '.class_name())';
        }
        if ($t->kind === RustType::GENERIC) {
            // a generic value narrowed to an object by Psalm: its class name through the PhpKind bound
            return 'Str::from_static(php_rt::PhpKind::php_class_name(&' . $code . ').unwrap_or_else(|| panic!("get_class(): not an object")))';
        }
        return null;
    }

    /** `spl_object_id($v)` without erasing the object; null when the type is not statically an object. */
    public function objIdOf(string $code, RustType $t): ?string
    {
        if ($t->kind === RustType::OPTION) {
            return $this->objIdOf($code . '.clone().expect("null where object expected")', $t->inner());
        }
        if ($t->kind === RustType::CLASS_ || $t->kind === RustType::ANY_OBJECT) {
            return '(php_rt::PhpObject::obj_id(&' . $code . ') as i64)';
        }
        if ($t->kind === RustType::UNION && self::unionHasObject($t)) {
            return '(' . $code . '.obj_id() as i64)';
        }
        return null;
    }

    /** `$v instanceof $name` (`$name` a `Str` expression): typed on objects/unions/generics, through Mixed otherwise. */
    public function instanceOfName(string $code, RustType $t, string $name): string
    {
        $inner = $t->kind === RustType::OPTION ? $t->inner() : $t;
        if ($inner->kind === RustType::CLASS_ || $inner->kind === RustType::ANY_OBJECT || $inner->kind === RustType::GENERIC
            || ($inner->kind === RustType::UNION && self::unionHasObject($inner))
        ) {
            return 'php_rt::InstanceOfName::php_instance_of(&' . $code . ', ' . $name . '.as_bytes())';
        }
        return 'instance_of_name(&' . $this->convert($code, $t, RustType::mixed()) . ', &' . $name . ')';
    }

    /** Whether a value of type `$from` converts losslessly into `$to` (equal, Option widening, or a union member). */
    public function fits(RustType $from, RustType $to): bool
    {
        if ($from->toRust() === $to->toRust()) {
            return true;
        }
        if ($to->kind === RustType::OPTION) {
            return $this->fits($from->kind === RustType::OPTION ? $from->inner() : $from, $to->inner());
        }
        if ($from->kind === RustType::OPTION) {
            return false;
        }
        if ($to->kind === RustType::UNION) {
            return $this->pickMember($to, $from) !== null;
        }
        return false;
    }

    public static function unionHasObject(RustType $u): bool
    {
        foreach ($u->params as $m) {
            if ($m->kind === RustType::CLASS_ || $m->kind === RustType::ANY_OBJECT) {
                return true;
            }
        }
        return false;
    }

    /** @var array<string, array{all: ?string, classes: array<string, true>}> */
    private array $reach_memo = [];

    /**
     * The classes reachable from a value of type $t once it is erased to Mixed (through union members, shape
     * fields, containers, class fields and subclasses); 'all' names the member that can hold any object.
     *
     * @return array{all: ?string, classes: array<string, true>}
     */
    private function reachOf(RustType $t): array
    {
        $k = $t->toRust();
        if (isset($this->reach_memo[$k])) {
            return $this->reach_memo[$k];
        }
        $this->reach_memo[$k] = ['all' => null, 'classes' => []]; // cycle guard
        $r = ['all' => null, 'classes' => []];
        $merge = function (RustType $sub) use (&$r): void {
            $sr = $this->reachOf($sub);
            $r['all'] ??= $sr['all'];
            $r['classes'] += $sr['classes'];
        };
        switch ($t->kind) {
            case RustType::CLASS_:
                $c = $this->program->classOf($t);
                if ($c !== null) {
                    foreach (array_merge([$c], $c->concrete) as $cc) {
                        $r['classes'][$cc->fqcn] = true;
                    }
                    foreach (array_merge([$c], $c->concrete) as $cc) {
                        foreach ($cc->fields as $f) {
                            $merge($f->type);
                        }
                    }
                }
                break;
            case RustType::UNION:
            case RustType::OPTION:
            case RustType::LIST:
            case RustType::TUPLE:
            case RustType::MAP:
            case RustType::CLOSURE:
                foreach ($t->params as $pt) {
                    $merge($pt);
                }
                if ($t->kind === RustType::CLOSURE && $t->ret !== null) {
                    $merge($t->ret);
                }
                break;
            case RustType::SHAPE:
                foreach ($t->fields as [$ft]) {
                    $merge($ft);
                }
                break;
            case RustType::ANY_OBJECT:
            case RustType::GENERIC:
            case RustType::DYN_CALLABLE:
                $r['all'] = $k;
                break;
        }
        return $this->reach_memo[$k] = $r;
    }

    public function dynReachable(): array
    {
        $classes = [];
        $all = null;
        $keepers = [];
        foreach ($this->erasures as $k => [$t, $ctx]) {
            $r = $this->reachOf($t);
            if ($r['all'] === null && $r['classes'] === []) {
                continue;
            }
            $all ??= $r['all'] === null ? null : $r['all'] . ' erased as ' . $k;
            $classes += $r['classes'];
            $desc = $k;
            if ($t->kind === RustType::SHAPE) {
                $desc .= '{' . implode(', ', array_map(fn($fk, $f) => $fk . ': ' . $f[0]->toRust(), array_keys($t->fields), $t->fields)) . '}';
            }
            $keepers[] = ($r['all'] !== null ? 'ALL(' . $r['all'] . ') ' : count($r['classes']) . ' classes ') . $desc . ' @ ' . ($ctx ?? '?');
        }
        return ['all' => $all, 'classes' => $classes, 'keepers' => $keepers];
    }

    public function needMixedTo(RustType $to): void
    {
        switch ($to->kind) {
            case RustType::CLASS_:
            case RustType::UNION:
            case RustType::SHAPE:
            case RustType::ANY_OBJECT:
                $this->need(RustType::mixed(), $to);
                break;
            case RustType::OPTION:
            case RustType::LIST:
                $this->needMixedTo($to->inner());
                break;
            case RustType::MAP:
                $this->needMixedTo($to->params[1]);
                break;
        }
    }

    private function warn(string $msg): void
    {
        $this->warnings[$msg] = $msg;
    }
}
