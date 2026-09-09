<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use function array_map;
use function count;
use function implode;
use function in_array;

/**
 * Emits generated types (union enums, shape structs) and the trait impls the
 * generated code relies on (CastTo, InstanceOf, Identical, Truthy, ToStr, ...).
 *
 * @internal
 */
final class CastEmitter
{
    public function __construct(
        private readonly Program $program,
        private readonly Casts $casts,
    ) {
    }

    private function conv(string $code, RustType $from, RustType $to): string
    {
        return $this->casts->convert($code, $from, $to);
    }

    // ------------------------------------------------------------------ generated types

    public function emitUnion(RustType $u, Writer $w): void
    {
        $name = $u->mangle();
        $w->line('#[derive(Clone)]');
        $w->open('pub enum ' . $name . ' {');
        foreach ($u->params as $m) {
            if ($this->isUnit($m)) {
                $w->line($this->unitName($m) . ',');
            } else {
                $w->line($m->variantName() . '(' . $m->toRust() . '),');
            }
        }
        $w->close();
        // Truthy
        $arms = [];
        foreach ($u->params as $m) {
            if ($this->isUnit($m)) {
                $arms[] = $name . '::' . $this->unitName($m) . ' => ' . ($this->unitName($m) === 'True' ? 'true' : 'false');
            } elseif ($m->kind === RustType::CLOSURE) {
                $arms[] = $name . '::' . $m->variantName() . '(_) => true';
            } else {
                $arms[] = $name . '::' . $m->variantName() . '(v) => truthy(v)';
            }
        }
        $w->line('impl php_rt::Truthy for ' . $name . ' { fn truthy(&self) -> bool { match self { ' . implode(', ', $arms) . ' } } }');
        // ToStr
        $arms = [];
        foreach ($u->params as $m) {
            if ($this->isUnit($m)) {
                $arms[] = $name . '::' . $this->unitName($m) . ' => ' . ($this->unitName($m) === 'True' ? 'Str::from_static("1")' : 'Str::empty()');
            } elseif ($this->stringable($m)) {
                $arms[] = $name . '::' . $m->variantName() . '(v) => to_str(v)';
            } elseif ($m->kind === RustType::CLOSURE || $m->kind === RustType::DYN_CALLABLE) {
                $arms[] = $name . '::' . $m->variantName() . '(_) => Str::from_static("Closure")';
            } else {
                $arms[] = $name . '::' . $m->variantName() . '(v) => ' . $this->conv('v.clone()', $m, RustType::mixed()) . '.to_php_str()';
            }
        }
        $w->line('impl php_rt::ToStr for ' . $name . ' { fn to_php_str(&self) -> Str { match self { ' . implode(', ', $arms) . ' } } }');
        $w->line('impl ' . $name . ' { pub fn to_php_string(&self) -> Result<Str, Throw> { Ok(self.to_php_str()) } }');
        // Identical
        $arms = [];
        foreach ($u->params as $m) {
            if ($this->isUnit($m)) {
                $arms[] = '(' . $name . '::' . $this->unitName($m) . ', ' . $name . '::' . $this->unitName($m) . ') => true';
            } elseif ($m->kind === RustType::CLOSURE) {
                $arms[] = '(' . $name . '::' . $m->variantName() . '(a), ' . $name . '::' . $m->variantName() . '(b)) => Rc::ptr_eq(a, b)';
            } else {
                $arms[] = '(' . $name . '::' . $m->variantName() . '(a), ' . $name . '::' . $m->variantName() . '(b)) => identical(a, b)';
            }
        }
        $w->line('impl php_rt::Identical for ' . $name . ' { fn identical(&self, o: &Self) -> bool { match (self, o) { ' . implode(', ', $arms) . ', _ => false } } }');
        // PhpCmp via Mixed
        $w->line('impl php_rt::PhpCmp for ' . $name . ' { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }');
        // Mixed conversion
        $arms = [];
        foreach ($u->params as $m) {
            if ($this->isUnit($m)) {
                $arms[] = $name . '::' . $this->unitName($m) . ' => Mixed::Bool(' . ($this->unitName($m) === 'True' ? 'true' : 'false') . ')';
            } else {
                $arms[] = $name . '::' . $m->variantName() . '(v) => ' . $this->conv('v', $m, RustType::mixed());
            }
        }
        $w->line('impl php_rt::CastTo<Mixed> for ' . $name . ' { fn cast_to(self) -> Mixed { match self { ' . implode(', ', $arms) . ' } } }');
        // from Mixed
        $arms = [];
        foreach ($u->params as $m) {
            $arms[] = $this->mixedToMemberArm($name, $m);
        }
        $w->line('impl php_rt::CastTo<' . $name . '> for Mixed { fn cast_to(self) -> ' . $name . ' { ' . implode(' ', $arms) . ' panic!("cannot narrow Mixed into ' . $name . '") } }');
        // Default for unions containing a defaultable member (first)
        foreach ($u->params as $m) {
            if ($this->isUnit($m)) {
                $w->line('impl Default for ' . $name . ' { fn default() -> Self { ' . $name . '::' . $this->unitName($m) . ' } }');
                break;
            }
            if ($m->hasDefault()) {
                $w->line('impl Default for ' . $name . ' { fn default() -> Self { ' . $name . '::' . $m->variantName() . '(Default::default()) } }');
                break;
            }
        }
        // ToInt/ToFloat/ToArrayKey helpers via Mixed
        $w->line('impl php_rt::ToInt for ' . $name . ' { fn to_php_int(&self) -> i64 { cast::<Mixed>(self.clone()).to_php_int() } }');
        $w->line('impl php_rt::ToFloat for ' . $name . ' { fn to_php_float(&self) -> f64 { cast::<Mixed>(self.clone()).to_php_float() } }');
        $w->line('impl php_rt::ToArrayKey for ' . $name . ' { fn to_php_key(&self) -> ArrayKey { cast::<Mixed>(self.clone()).to_php_key() } }');
        $w->line('impl std::fmt::Debug for ' . $name . ' { fn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result { write!(f, "{:?}", cast::<Mixed>(self.clone())) } }');
        $w->line('impl ' . $name . ' { pub fn unwrap_or_default_marker(self) -> Self { self } }');
        // member accessors / predicates
        foreach ($u->params as $m) {
            if ($this->isUnit($m)) {
                continue;
            }
            $vn = $m->variantName();
            $extra = '';
            if ($m->kind === RustType::CLASS_) {
                // other class members that are subclasses of this one upcast into it
                $mc = $this->program->classOf($m);
                foreach ($u->params as $o) {
                    if ($o !== $m && $o->kind === RustType::CLASS_ && $mc !== null) {
                        $oc = $this->program->classOf($o);
                        if ($oc !== null && $oc->isSubclassOf($mc)) {
                            $extra .= $name . '::' . $o->variantName() . '(v) => ' . $this->conv('v', $o, $m) . ', ';
                        }
                    }
                }
            }
            $w->line('impl php_rt::CastTo<' . $m->toRust() . '> for ' . $name . ' { fn cast_to(self) -> ' . $m->toRust() . ' { match self { ' . $name . '::' . $vn . '(v) => v, ' . $extra . '_ => panic!("union ' . $name . ' is not ' . $vn . '") } } }');
            $w->line('impl php_rt::CastTo<' . $name . '> for ' . $m->toRust() . ' { fn cast_to(self) -> ' . $name . ' { ' . $name . '::' . $vn . '(self) } }');
        }
        if ($this->casts->hasUnit($u, 'True') || $this->casts->hasUnit($u, 'False')) {
            $t = $this->casts->hasUnit($u, 'True') ? $name . '::True => true, ' : '';
            $f = $this->casts->hasUnit($u, 'False') ? $name . '::False => false, ' : '';
            $w->line('impl php_rt::CastTo<bool> for ' . $name . ' { fn cast_to(self) -> bool { match self { ' . $t . $f . 'other => truthy(&other) } } }');
        }
    }

    private function mixedToMemberArm(string $name, RustType $m): string
    {
        if ($this->isUnit($m)) {
            $b = $this->unitName($m) === 'True' ? 'true' : 'false';
            return 'if let Mixed::Bool(' . $b . ') = self { return ' . $name . '::' . $this->unitName($m) . '; }';
        }
        $vn = $m->variantName();
        return match ($m->kind) {
            RustType::INT => 'if let Mixed::Int(v) = self { return ' . $name . '::' . $vn . '(v); }',
            RustType::FLOAT => 'if let Mixed::Float(v) = self { return ' . $name . '::' . $vn . '(v); }',
            RustType::BOOL => 'if let Mixed::Bool(v) = self { return ' . $name . '::' . $vn . '(v); }',
            RustType::STR => 'if let Mixed::Str(v) = self { return ' . $name . '::' . $vn . '(v); }',
            RustType::ARRAY_KEY => 'if let Mixed::Int(v) = self { return ' . $name . '::' . $vn . '(ArrayKey::Int(v)); } if let Mixed::Str(v) = &self { return ' . $name . '::' . $vn . '(ArrayKey::from_str_val(v.clone())); }',
            RustType::UNIT => 'if let Mixed::Null = self { return ' . $name . '::' . $vn . '(()); }',
            RustType::LIST, RustType::MAP, RustType::TUPLE, RustType::SHAPE => 'if let Mixed::Arr(_) = &self { return ' . $name . '::' . $vn . '(cast::<' . $m->toRust() . '>(self)); }',
            RustType::CLASS_, RustType::ANY_OBJECT => 'if let Mixed::Obj(o) = &self { if let Some(v) = try_downcast::<' . $m->toRust() . '>(o) { return ' . $name . '::' . $vn . '(v); } }',
            RustType::CLOSURE, RustType::DYN_CALLABLE => 'if let Mixed::Closure(_) = &self { return ' . $name . '::' . $vn . '(' . $this->conv('self', RustType::mixed(), $m) . '); }',
            RustType::MIXED => 'return ' . $name . '::' . $vn . '(self);',
            default => 'if let Some(v) = try_cast_mixed::<' . $m->toRust() . '>(&self) { return ' . $name . '::' . $vn . '(v); }',
        };
    }

    private function isUnit(RustType $m): bool
    {
        return $m->kind === RustType::RT_GENERIC && str_starts_with($m->name, '__unit_');
    }

    private function unitName(RustType $m): string
    {
        return substr($m->name, 7);
    }

    private function stringable(RustType $m): bool
    {
        return in_array($m->kind, [RustType::INT, RustType::FLOAT, RustType::BOOL, RustType::STR, RustType::ARRAY_KEY, RustType::MIXED, RustType::UNIT, RustType::OPTION], true);
    }

    public function emitShape(RustType $s, Writer $w): void
    {
        $name = $s->mangle();
        $all_default = $s->hasDefault();
        $w->line('#[derive(Clone' . ($all_default ? ', Default' : '') . ')]');
        $w->open('pub struct ' . $name . ' {');
        foreach ($s->fields as $k => [$t, $opt]) {
            $w->line('pub ' . Names::field($k) . ': ' . ($opt ? RustType::option($t)->toRust() : $t->toRust()) . ',');
        }
        $w->close();
        $w->line('impl php_rt::Truthy for ' . $name . ' { fn truthy(&self) -> bool { ' . (count($s->fields) ? 'true' : 'false') . ' } }');
        $w->line('impl php_rt::ToStr for ' . $name . ' { fn to_php_str(&self) -> Str { Str::from_static("Array") } }');
        $parts = [];
        foreach ($s->fields as $k => [$t, $opt]) {
            $parts[] = 'identical(&self.' . Names::field($k) . ', &o.' . Names::field($k) . ')';
        }
        $w->line('impl php_rt::Identical for ' . $name . ' { fn identical(&self, o: &Self) -> bool { ' . ($parts ? implode(' && ', $parts) : 'true') . ' } }');
        $w->line('impl php_rt::PhpCmp for ' . $name . ' { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }');
        $w->line('impl php_rt::Len for ' . $name . ' { fn php_count(&self) -> i64 { let mut n = 0i64; ' . implode(' ', array_map(fn($k, $f) => $f[1] ? 'if self.' . Names::field($k) . '.is_some() { n += 1; }' : 'n += 1;', array_keys($s->fields), $s->fields)) . ' n } }');
        $ins = [];
        foreach ($s->fields as $k => [$t, $opt]) {
            $key = 'ArrayKey::from(' . Names::strLit($k) . ')';
            if ($opt) {
                $ins[] = 'if let Some(v) = self.' . Names::field($k) . ' { m.insert(' . $key . ', ' . $this->conv('v', $t, RustType::mixed()) . '); }';
            } else {
                $ins[] = 'm.insert(' . $key . ', ' . $this->conv('self.' . Names::field($k), $t, RustType::mixed()) . ');';
            }
        }
        $w->line('impl php_rt::CastTo<Mixed> for ' . $name . ' { fn cast_to(self) -> Mixed { let mut m: Map<ArrayKey, Mixed> = Map::new(); ' . implode(' ', $ins) . ' Mixed::Arr(m) } }');
        $outs = [];
        foreach ($s->fields as $k => [$t, $opt]) {
            $key = '&ArrayKey::from(' . Names::strLit($k) . ')';
            if ($opt) {
                $outs[] = Names::field($k) . ': ' . $this->conv('m.get(' . $key . ').cloned()', RustType::option(RustType::mixed()), RustType::option($t));
            } else {
                $outs[] = Names::field($k) . ': ' . $this->conv('m.get(' . $key . ').cloned().unwrap_or_default()', RustType::mixed(), $t);
            }
        }
        $w->line('impl php_rt::CastTo<' . $name . '> for Mixed { fn cast_to(self) -> ' . $name . ' { let m = cast::<Map<ArrayKey, Mixed>>(self); ' . $name . ' { ' . implode(', ', $outs) . ' } } }');
        $w->line('impl std::fmt::Debug for ' . $name . ' { fn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result { write!(f, "{:?}", cast::<Mixed>(self.clone())) } }');
    }

    // ------------------------------------------------------------------ class conversions

    public function emitClassImpls(ClassModel $cls, Writer $w): void
    {
        if ($cls->isTrait() || $cls->isEnum()) {
            return;
        }
        $h = $cls->handle();
        $ht = RustType::class($cls->fqcn);
        // Truthy / ToStr / Identical / PhpCmp on the handle type
        $w->line('impl php_rt::Truthy for ' . $h . ' { fn truthy(&self) -> bool { true } }');
        $w->line('impl php_rt::ToStr for ' . $h . ' { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static(' . Names::rustStringLiteral($cls->fqcn) . ')) } }');
        $w->line('impl php_rt::Identical for ' . $h . ' { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }');
        $w->line('impl php_rt::PhpCmp for ' . $h . ' { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }');
        $w->line('impl std::fmt::Debug for ' . $h . ' { fn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }');
        // to Mixed: store the concrete own handle
        if ($cls->isLeaf()) {
            $w->line('impl php_rt::CastTo<Mixed> for ' . $h . ' { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }');
        } else {
            $arms = array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(v) => Mixed::Obj(Rc::new(v))', $cls->concrete);
            $w->line('impl php_rt::CastTo<Mixed> for ' . $h . ' { fn cast_to(self) -> Mixed { match self { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => unreachable!() } } }');
            if ($cls->isConcrete()) {
                $own = $cls->ownHandle();
                $w->line('impl php_rt::CastTo<Mixed> for ' . $own . ' { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }');
                $w->line('impl php_rt::Truthy for ' . $own . ' { fn truthy(&self) -> bool { true } }');
                $w->line('impl php_rt::Identical for ' . $own . ' { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }');
            }
        }
        // from Mixed: downcast to any concrete descendant
        $arms = [];
        $some_arms = [];
        foreach ($cls->concrete as $c) {
            $arms[] = 'if let Some(v) = o.as_any().downcast_ref::<' . $c->ownPath() . '>() { return ' . $this->wrapConcrete($cls, $c, 'v.clone()') . '; }';
            $some_arms[] = 'if let Some(v) = o.as_any().downcast_ref::<' . $c->ownPath() . '>() { return Some(' . $this->wrapConcrete($cls, $c, 'v.clone()') . '); }';
        }
        $w->line('impl php_rt::CastTo<' . $h . '> for Mixed { fn cast_to(self) -> ' . $h . ' { if let Mixed::Obj(o) = &self { ' . implode(' ', $arms) . ' } panic!(' . Names::rustStringLiteral('Mixed value is not a ' . $cls->fqcn) . ') } }');
        $w->line('impl php_rt::TryDowncast for ' . $h . ' { fn try_downcast(o: &AnyObj) -> Option<Self> { ' . implode(' ', $some_arms) . ' None } }');
        // AnyObject
        $w->line('impl php_rt::CastTo<AnyObject> for ' . $h . ' { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }');
        $w->line('impl php_rt::CastTo<' . $h . '> for AnyObject { fn cast_to(self) -> ' . $h . ' { cast::<' . $h . '>(cast::<Mixed>(self)) } }');
        $w->line('impl php_rt::InstanceOf<' . $h . '> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name(' . Names::rustStringLiteral(strtolower($cls->fqcn)) . ') } }');
        $w->line('impl php_rt::InstanceOf<' . $h . '> for Mixed { fn is_instance(&self) -> bool { self.instance_of(' . Names::rustStringLiteral(strtolower($cls->fqcn)) . ') } }');
        $w->line('impl php_rt::InstanceOf<' . $h . '> for ' . $h . ' { fn is_instance(&self) -> bool { true } }');
    }

    /** Wrap a concrete own handle value into the handle type of `$cls`. */
    private function wrapConcrete(ClassModel $cls, ClassModel $c, string $code): string
    {
        if ($cls->isLeaf()) {
            return $code;
        }
        return $cls->path() . '::' . $c->variant() . '(' . $code . ')';
    }

    /** Emit all cast/instanceof impls between generated types recorded during body emission. */
    /** @var array<string, bool> */
    private array $done = [];

    public function emitRecordedCasts(Writer $w): void
    {
        $done = &$this->done;
        // iterate until no new needs are added (emitting casts may require nested ones)
        while (true) {
            $pending = [];
            foreach ($this->casts->casts as $k => $pair) {
                if (!isset($done[$k])) {
                    $pending[$k] = $pair;
                }
            }
            foreach ($this->casts->instance_checks as $k => $pair) {
                if (!isset($done['is:' . $k])) {
                    $pending['is:' . $k] = $pair;
                }
            }
            if ($pending === []) {
                break;
            }
            foreach ($pending as $k => [$from, $to]) {
                $done[$k] = true;
                if (str_starts_with($k, 'is:')) {
                    $this->emitInstanceOf($from, $to, $w);
                } else {
                    $this->emitCast($from, $to, $w);
                }
            }
        }
    }

    private function emitCast(RustType $from, RustType $to, Writer $w): void
    {
        $fk = $from->kind;
        $tk = $to->kind;
        // member <-> union and Mixed <-> union impls are emitted together with the union enum
        if ($fk === RustType::UNION && ($tk === RustType::MIXED || $this->isExactMember($from, $to))) {
            return;
        }
        if ($tk === RustType::UNION && ($fk === RustType::MIXED || $this->isExactMember($to, $from))) {
            return;
        }
        // unions
        if ($fk === RustType::UNION && $tk === RustType::UNION) {
            $arms = [];
            foreach ($from->params as $m) {
                if ($this->isUnit($m)) {
                    $target = $this->casts->hasUnit($to, $this->unitName($m)) ? $to->mangle() . '::' . $this->unitName($m) : ($this->hasBool($to) ? $to->mangle() . '::Bool(' . ($this->unitName($m) === 'True' ? 'true' : 'false') . ')' : 'panic!("no variant")');
                    $arms[] = $from->mangle() . '::' . $this->unitName($m) . ' => ' . $target;
                    continue;
                }
                $member = $this->casts->pickMember($to, $m);
                if ($member !== null) {
                    $arms[] = $from->mangle() . '::' . $m->variantName() . '(v) => ' . $this->conv('v', $m, $to);
                } elseif ($m->kind === RustType::BOOL && ($this->casts->hasUnit($to, 'True') || $this->casts->hasUnit($to, 'False'))) {
                    $arms[] = $from->mangle() . '::' . $m->variantName() . '(v) => ' . $this->conv('v', RustType::bool(), $to);
                } else {
                    $arms[] = $from->mangle() . '::' . $m->variantName() . '(v) => ' . $this->conv($this->conv('v', $m, RustType::mixed()), RustType::mixed(), $to);
                }
            }
            $w->line('impl php_rt::CastTo<' . $to->toRust() . '> for ' . $from->toRust() . ' { fn cast_to(self) -> ' . $to->toRust() . ' { match self { ' . implode(', ', $arms) . ' } } }');
            return;
        }
        if ($fk === RustType::UNION) {
            // narrowing to a non-member type: try each member that can convert
            $arms = [];
            foreach ($from->params as $m) {
                if ($this->isUnit($m)) {
                    if ($tk === RustType::BOOL) {
                        $arms[] = $from->mangle() . '::' . $this->unitName($m) . ' => ' . ($this->unitName($m) === 'True' ? 'true' : 'false');
                    } else {
                        $arms[] = $from->mangle() . '::' . $this->unitName($m) . ' => ' . $this->conv('Mixed::Bool(' . ($this->unitName($m) === 'True' ? 'true' : 'false') . ')', RustType::mixed(), $to);
                    }
                    continue;
                }
                if ($m->toRust() === $to->toRust()) {
                    $arms[] = $from->mangle() . '::' . $m->variantName() . '(v) => v';
                } elseif ($this->convertible($m, $to)) {
                    $arms[] = $from->mangle() . '::' . $m->variantName() . '(v) => ' . $this->conv('v', $m, $to);
                } else {
                    $arms[] = $from->mangle() . '::' . $m->variantName() . '(_) => panic!("cannot narrow ' . $from->mangle() . '::' . $m->variantName() . ' into ' . $to->toRust() . '")';
                }
            }
            $w->line('impl php_rt::CastTo<' . $to->toRust() . '> for ' . $from->toRust() . ' { fn cast_to(self) -> ' . $to->toRust() . ' { match self { ' . implode(', ', $arms) . ' } } }');
            return;
        }
        if ($tk === RustType::UNION) {
            $member = $this->casts->pickMember($to, $from);
            if ($member !== null) {
                $w->line('impl php_rt::CastTo<' . $to->toRust() . '> for ' . $from->toRust() . ' { fn cast_to(self) -> ' . $to->toRust() . ' { ' . $to->mangle() . '::' . $member->variantName() . '(' . $this->conv('self', $from, $member) . ') } }');
                return;
            }
            if ($fk === RustType::CLASS_) {
                // a class whose descendants map to several members (e.g. Atomic into TInt|TString)
                $cls = $this->program->classOf($from);
                $arms = [];
                if ($cls !== null && !$cls->isLeaf()) {
                    foreach ($cls->concrete as $c) {
                        $target = null;
                        foreach ($to->params as $m) {
                            if ($m->kind === RustType::CLASS_) {
                                $mc = $this->program->classOf($m);
                                if ($mc !== null && $c->isSubclassOf($mc)) {
                                    $target = $m;
                                    break;
                                }
                            }
                        }
                        if ($target !== null) {
                            $arms[] = $from->toRust() . '::' . $c->variant() . '(v) => ' . $to->mangle() . '::' . $target->variantName() . '(' . $this->conv('v', RustType::class($c->fqcn), $target) . ')';
                        } else {
                            $arms[] = $from->toRust() . '::' . $c->variant() . '(_) => panic!(' . Names::rustStringLiteral('cannot narrow ' . $c->fqcn . ' into ' . $to->mangle()) . ')';
                        }
                    }
                    $w->line('impl php_rt::CastTo<' . $to->toRust() . '> for ' . $from->toRust() . ' { fn cast_to(self) -> ' . $to->toRust() . ' { match self { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => unreachable!() } } }');
                    return;
                }
            }
            $w->line('impl php_rt::CastTo<' . $to->toRust() . '> for ' . $from->toRust() . ' { fn cast_to(self) -> ' . $to->toRust() . ' { ' . $this->conv($this->conv('self', $from, RustType::mixed()), RustType::mixed(), $to) . ' } }');
            return;
        }
        if ($fk === RustType::CLASS_ && $tk === RustType::CLASS_) {
            $this->emitClassCast($from, $to, $w);
            return;
        }
        if ($fk === RustType::CLASS_ && $tk === RustType::RT_GENERIC && in_array($to->name, ['PhpIterator', 'Traversable', 'IteratorAggregate', 'Generator'], true)) {
            $cls = $this->program->classOf($from);
            $pairs = $cls !== null ? $this->casts->objectPairs($cls, 'self') : null;
            if ($pairs !== null) {
                [$it, $kt, $vt] = $pairs;
                $tk_ = $to->params[0] ?? RustType::mixed();
                $tv_ = $to->params[1] ?? RustType::mixed();
                $w->line('impl php_rt::CastTo<' . $to->toRust() . '> for ' . $from->toRust() . ' { fn cast_to(self) -> ' . $to->toRust() . ' { let __run = || -> Result<Vec<(' . $tk_->toRust() . ', ' . $tv_->toRust() . ')>, Throw> { Ok(' . $it . '.map(|(__k, __v)| (' . $this->conv('__k', $kt, $tk_) . ', ' . $this->conv('__v', $vt, $tv_) . ')).collect()) }; Generator::from_pairs(__run().unwrap_or_else(|e| panic!("uncaught {}", e))) } }');
                return;
            }
        }
        if ($fk === RustType::RT_GENERIC && $from->name === 'Num' && $tk === RustType::UNION) {
            $w->line('impl php_rt::CastTo<' . $to->toRust() . '> for Num { fn cast_to(self) -> ' . $to->toRust() . ' { match self { Num::Int(i) => ' . $this->conv('i', RustType::int(), $to) . ', Num::Float(f) => ' . $this->conv('f', RustType::float(), $to) . ' } } }');
            return;
        }
        if (!Casts::isLocal($from) && !Casts::isLocal($to)) {
            return;
        }
        if (($fk === RustType::TUPLE || $fk === RustType::CLOSURE) && $tk === RustType::MIXED) {
            if ($fk === RustType::TUPLE) {
                $parts = [];
                foreach ($from->params as $i => $p) {
                    $parts[] = 'm.push(' . $this->conv('self.' . $i, $p, RustType::mixed()) . ');';
                }
                $w->line('impl php_rt::CastTo<Mixed> for ' . $from->toRust() . ' { fn cast_to(self) -> Mixed { let mut m: Map<ArrayKey, Mixed> = Map::new(); ' . implode(' ', $parts) . ' Mixed::Arr(m) } }');
            } else {
                $w->line('impl php_rt::CastTo<Mixed> for ' . $from->toRust() . ' { fn cast_to(self) -> Mixed { Mixed::Closure(Rc::new(' . $this->conv('self', $from, RustType::dynCallable()) . ')) } }');
            }
            return;
        }
        if ($fk === RustType::MIXED && ($tk === RustType::TUPLE || $tk === RustType::CLOSURE)) {
            if ($tk === RustType::TUPLE) {
                $parts = [];
                foreach ($to->params as $i => $p) {
                    $parts[] = $this->conv('l.get(' . $i . ').cloned().unwrap_or_default()', RustType::mixed(), $p);
                }
                $w->line('impl php_rt::CastTo<' . $to->toRust() . '> for Mixed { fn cast_to(self) -> ' . $to->toRust() . ' { let l = cast::<List<Mixed>>(self); (' . implode(', ', $parts) . (count($parts) === 1 ? ',' : '') . ') } }');
            } else {
                $w->line('impl php_rt::CastTo<' . $to->toRust() . '> for Mixed { fn cast_to(self) -> ' . $to->toRust() . ' { ' . $this->conv('to_callable(&self)', RustType::dynCallable(), $to) . ' } }');
            }
            return;
        }
        // already handled elsewhere (class <-> Mixed/AnyObject, shapes) or genuinely unsupported
        if (($fk === RustType::CLASS_ && ($tk === RustType::MIXED || $tk === RustType::ANY_OBJECT)) || ($tk === RustType::CLASS_ && ($fk === RustType::MIXED || $fk === RustType::ANY_OBJECT))
            || ($fk === RustType::SHAPE && $tk === RustType::MIXED) || ($fk === RustType::MIXED && $tk === RustType::SHAPE)
            || ($fk === RustType::UNION && $tk === RustType::MIXED) || ($fk === RustType::MIXED && $tk === RustType::UNION)
            || ($fk === RustType::ANY_OBJECT && $tk === RustType::MIXED) || ($fk === RustType::MIXED && $tk === RustType::ANY_OBJECT)
            || ($fk === RustType::DYN_CALLABLE && $tk === RustType::MIXED) || ($fk === RustType::MIXED && $tk === RustType::DYN_CALLABLE)
        ) {
            return;
        }
        $w->line('impl php_rt::CastTo<' . $to->toRust() . '> for ' . $from->toRust() . ' { fn cast_to(self) -> ' . $to->toRust() . ' { unimplemented!("cast ' . $from->toRust() . ' => ' . $to->toRust() . '") } }');
    }

    private function isExactMember(RustType $union, RustType $t): bool
    {
        foreach ($union->params as $m) {
            if (!$this->isUnit($m) && $m->toRust() === $t->toRust()) {
                return true;
            }
        }
        return false;
    }

    private function hasBool(RustType $u): bool
    {
        foreach ($u->params as $m) {
            if ($m->kind === RustType::BOOL) {
                return true;
            }
        }
        return false;
    }

    private function convertible(RustType $from, RustType $to): bool
    {
        if ($from->toRust() === $to->toRust()) {
            return true;
        }
        if ($to->kind === RustType::MIXED) {
            return true;
        }
        if ($from->kind === RustType::CLASS_ && $to->kind === RustType::CLASS_) {
            $fc = $this->program->classOf($from);
            $tc = $this->program->classOf($to);
            return $fc !== null && $tc !== null && ($fc->isSubclassOf($tc) || $tc->isSubclassOf($fc));
        }
        $scalars = [RustType::INT, RustType::FLOAT, RustType::STR, RustType::BOOL, RustType::ARRAY_KEY];
        if (in_array($from->kind, $scalars, true) && in_array($to->kind, $scalars, true)) {
            return true;
        }
        $containers = [RustType::LIST, RustType::MAP, RustType::TUPLE, RustType::SHAPE];
        if (in_array($from->kind, $containers, true) && in_array($to->kind, $containers, true)) {
            return true;
        }
        if ($from->kind === RustType::OPTION || $to->kind === RustType::OPTION) {
            return true;
        }
        if ($to->kind === RustType::UNION) {
            return $this->casts->pickMember($to, $from) !== null;
        }
        if ($from->kind === RustType::CLASS_ && $to->kind === RustType::ANY_OBJECT) {
            return true;
        }
        return false;
    }

    private function emitClassCast(RustType $from, RustType $to, Writer $w): void
    {
        $fc = $this->program->classOf($from);
        $tc = $this->program->classOf($to);
        $fh = $from->toRust();
        $th = $to->toRust();
        if ($fc === null || $tc === null) {
            $w->line('impl php_rt::CastTo<' . $th . '> for ' . $fh . ' { fn cast_to(self) -> ' . $th . ' { unimplemented!("cast between unknown classes") } }');
            return;
        }
        $arms = [];
        if ($fc->isLeaf()) {
            // upcast of a leaf into an ancestor enum, or (impossible) sideways cast
            if ($fc->isSubclassOf($tc)) {
                $body = $tc->isLeaf() ? 'self' : $th . '::' . $fc->variant() . '(self)';
            } else {
                $body = 'panic!(' . Names::rustStringLiteral('cannot cast ' . $fc->fqcn . ' to ' . $tc->fqcn) . ')';
            }
            $w->line('impl php_rt::CastTo<' . $th . '> for ' . $fh . ' { fn cast_to(self) -> ' . $th . ' { ' . $body . ' } }');
            return;
        }
        foreach ($fc->concrete as $c) {
            if ($c->isSubclassOf($tc)) {
                $arms[] = $fh . '::' . $c->variant() . '(v) => ' . $this->wrapConcrete($tc, $c, 'v');
            } else {
                $arms[] = $fh . '::' . $c->variant() . '(_) => panic!(' . Names::rustStringLiteral('cannot cast ' . $c->fqcn . ' to ' . $tc->fqcn) . ')';
            }
        }
        $w->line('impl php_rt::CastTo<' . $th . '> for ' . $fh . ' { fn cast_to(self) -> ' . $th . ' { match self { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => unreachable!() } } }');
    }

    private function emitInstanceOf(RustType $subject, RustType $target, Writer $w): void
    {
        $th = $target->toRust();
        $tc = $this->program->classOf($target);
        if ($subject->kind === RustType::CLASS_) {
            $sc = $this->program->classOf($subject);
            if ($sc === null || $tc === null) {
                return;
            }
            if ($sc === $tc) {
                return; // emitted with the class impls
            }
            if ($sc->isLeaf()) {
                $w->line('impl php_rt::InstanceOf<' . $th . '> for ' . $subject->toRust() . ' { fn is_instance(&self) -> bool { ' . ($sc->isSubclassOf($tc) ? 'true' : 'false') . ' } }');
                return;
            }
            $yes = [];
            foreach ($sc->concrete as $c) {
                if ($c->isSubclassOf($tc)) {
                    $yes[] = $subject->toRust() . '::' . $c->variant() . '(_)';
                }
            }
            $body = $yes === [] ? 'false' : 'matches!(self, ' . implode(' | ', $yes) . ')';
            $w->line('impl php_rt::InstanceOf<' . $th . '> for ' . $subject->toRust() . ' { fn is_instance(&self) -> bool { ' . $body . ' } }');
            return;
        }
        if ($subject->kind === RustType::UNION) {
            $arms = [];
            foreach ($subject->params as $m) {
                if ($this->isUnit($m)) {
                    $arms[] = $subject->mangle() . '::' . $this->unitName($m) . ' => false';
                } elseif ($m->kind === RustType::CLASS_ || $m->kind === RustType::ANY_OBJECT || $m->kind === RustType::MIXED) {
                    $this->casts->needInstanceOf($m, $target);
                    $arms[] = $subject->mangle() . '::' . $m->variantName() . '(v) => is_instance::<' . $th . '>(v)';
                } else {
                    $arms[] = $subject->mangle() . '::' . $m->variantName() . '(_) => false';
                }
            }
            $w->line('impl php_rt::InstanceOf<' . $th . '> for ' . $subject->toRust() . ' { fn is_instance(&self) -> bool { match self { ' . implode(', ', $arms) . ' } } }');
            return;
        }
        // Mixed / AnyObject handled by class impls
    }
}
