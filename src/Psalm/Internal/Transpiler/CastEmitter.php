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
        // escape hatch for values that don't match any declared member (docblocks lying, invalid-input tests)
        $w->line('Other__(Mixed),');
        $w->close();
        // in-place access to an array member (`$u[] = v` on a union-typed place): a value that is not that
        // array becomes an empty one, as PHP autovivifies null
        foreach ($u->params as $m) {
            if ($m->kind === RustType::LIST || $m->kind === RustType::MAP) {
                $v = $m->variantName();
                $w->line('impl ' . $name . ' { pub fn ' . $v . '_or_insert(&mut self) -> &mut ' . $m->toRust() . ' { if !matches!(self, ' . $name . '::' . $v . '(_)) { *self = ' . $name . '::' . $v . '(Default::default()); } match self { ' . $name . '::' . $v . '(__a) => __a, _ => unreachable!() } } }');
            }
        }
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
        $arms[] = $name . '::Other__(v) => truthy(v)';
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
        $arms[] = $name . '::Other__(v) => v.to_php_str()';
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
        $arms[] = '(' . $name . '::Other__(a), ' . $name . '::Other__(b)) => identical(a, b)';
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
        $arms[] = $name . '::Other__(v) => v';
        $w->line('impl php_rt::CastTo<Mixed> for ' . $name . ' { fn cast_to(self) -> Mixed { match self { ' . implode(', ', $arms) . ' } } }');
        // from Mixed
        $arms = [];
        foreach ($u->params as $m) {
            $arms[] = $this->mixedToMemberArm($name, $m);
        }
        $w->line('impl php_rt::CastTo<' . $name . '> for Mixed { fn cast_to(self) -> ' . $name . ' { ' . implode(' ', $arms) . ' ' . $name . '::Other__(self) } }');
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
            if ($m->kind === RustType::DYN_CALLABLE) {
                // closure members of the union are callables too
                foreach ($u->params as $o) {
                    if ($o->kind === RustType::CLOSURE) {
                        $extra .= $name . '::' . $o->variantName() . '(v) => ' . $this->conv('v', $o, $m) . ', ';
                    }
                }
            }
            $mc = $m->kind === RustType::CLASS_ ? $this->program->classOf($m) : null;
            if ($mc !== null) {
                foreach ($u->params as $o) {
                    if ($o === $m || $o->kind !== RustType::CLASS_) {
                        continue;
                    }
                    $oc = $this->program->classOf($o);
                    if ($oc === null || $oc->isSubclassOf($mc)) {
                        continue;
                    }
                    if (!$mc->isLeaf()) {
                        // unrelated class members are carried through the target's escape variant
                        $extra .= $name . '::' . $o->variantName() . '(v) => ' . $this->conv($this->conv('v', $o, RustType::mixed()), RustType::mixed(), $m) . ', ';
                    } elseif (!$oc->isLeaf()) {
                        // a non-leaf member may hold the target in its own escape variant
                        $extra .= $name . '::' . $o->variantName() . '(v) => ' . $this->conv('v', $o, $m) . ', ';
                    }
                }
            }
            if ($m->kind === RustType::CLOSURE) {
                // other closure members (a closure literal's inferred signature joined with a declared one):
                // adapted by wrapping
                foreach ($u->params as $o) {
                    if ($o !== $m && $o->kind === RustType::CLOSURE && count($o->params) === count($m->params)) {
                        $extra .= $name . '::' . $o->variantName() . '(v) => ' . $this->conv('v', $o, $m) . ', ';
                    }
                }
            }
            if ($m->kind === RustType::STR) {
                // `(string) $x` on the other members: scalars and objects with __toString
                foreach ($u->params as $o) {
                    if ($o === $m || $this->isUnit($o)) {
                        continue;
                    }
                    $oc = $o->kind === RustType::CLASS_ ? $this->program->classOf($o) : null;
                    if (in_array($o->kind, [RustType::INT, RustType::FLOAT, RustType::BOOL, RustType::ARRAY_KEY], true)
                        || ($oc !== null && $this->program->findMethod($oc, '__tostring') !== null)
                    ) {
                        $extra .= $name . '::' . $o->variantName() . '(v) => ' . $this->conv('v', $o, $m) . ', ';
                    }
                }
            }
            $extra .= $name . '::Other__(v) => ' . $this->conv('v', RustType::mixed(), $m) . ', ';
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
            // resources travel through Mixed as their integer id: never a panic to catch
            RustType::RESOURCE => 'if let Mixed::Int(__id) = self { if let Some(v) = php_rt::containers::resource_by_id(__id as usize) { return ' . $name . '::' . $vn . '(v); } }',
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
            $w->line('pub ' . Names::field($k) . ': ' . RustType::shapeField($t, $opt)->toRust() . ',');
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
                $this->casts->needMixedTo($t);
                $outs[] = Names::field($k) . ': ' . $this->casts->optionMap('m.get(' . $key . ').cloned()', RustType::mixed(), $t);
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
        if ($cls->isTrait()) {
            return;
        }
        if ($cls->isEnum()) {
            $this->emitLeafMarker($cls, $w);
            return;
        }
        $h = $cls->handle();
        $ht = RustType::class($cls->fqcn);
        // Truthy / ToStr / Identical / PhpCmp on the handle type
        if ($cls->isLeaf()) {
            $w->line('impl php_rt::Truthy for ' . $h . ' { fn truthy(&self) -> bool { true } }');
            $w->line('impl php_rt::Identical for ' . $h . ' { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }');
        } else {
            $w->line('impl php_rt::Truthy for ' . $h . ' { fn truthy(&self) -> bool { match self { ' . $h . '::Other__(__m) => truthy(__m), _ => true } } }');
            $w->line('impl php_rt::Identical for ' . $h . ' { fn identical(&self, o: &Self) -> bool { match (self, o) { (' . $h . '::Other__(a), ' . $h . '::Other__(b)) => identical(a, b), (' . $h . '::Other__(_), _) | (_, ' . $h . '::Other__(_)) => false, _ => self.obj_id() == o.obj_id() } } }');
        }
        $w->line('impl php_rt::ToStr for ' . $h . ' { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static(' . Names::rustStringLiteral($cls->fqcn) . ')) } }');
        $num = $this->program->findMethod($cls, '__tostring') !== null ? 'to_num(&Mixed::Str(self.to_php_str()))' : '{ let _ = self; Num::Int(1) }';
        $w->line('impl php_rt::ToInt for ' . $h . ' { fn to_php_int(&self) -> i64 { ' . $num . '.to_i64() } }');
        $w->line('impl php_rt::ToFloat for ' . $h . ' { fn to_php_float(&self) -> f64 { ' . $num . '.to_f64() } }');
        $w->line('impl php_rt::PhpCmp for ' . $h . ' { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }');
        $w->line('impl std::fmt::Debug for ' . $h . ' { fn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }');
        // to Mixed: store the concrete own handle
        if ($cls->isLeaf()) {
            $w->line('impl php_rt::CastTo<Mixed> for ' . $h . ' { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }');
        } else {
            $arms = array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(v) => Mixed::Obj(Rc::new(v))', $cls->concrete);
            $arms[] = $h . '::Other__(m) => m';
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
        // one match on the program-wide class number (a linear chain of TypeId downcasts over every
        // concrete class dominated the profile of dynamic dispatch)
        foreach ($cls->concrete as $c) {
            $get = 'o.as_any().downcast_ref::<' . $c->ownPath() . '>().unwrap().clone()';
            $arms[] = $this->program->classId($c) . ' => return ' . $this->wrapConcrete($cls, $c, $get) . ',';
            $some_arms[] = $this->program->classId($c) . ' => return Some(' . $this->wrapConcrete($cls, $c, $get) . '),';
        }
        $fallback = $cls->isLeaf() ? 'panic!(' . Names::rustStringLiteral('Mixed value is not a ' . $cls->fqcn) . ')' : $h . '::Other__(self)';
        $w->line('impl php_rt::CastTo<' . $h . '> for Mixed { fn cast_to(self) -> ' . $h . ' { if let Mixed::Obj(o) = &self { match o.class_id() { ' . implode(' ', $arms) . ' _ => {} } } ' . $fallback . ' } }');
        $w->line('impl php_rt::TryDowncast for ' . $h . ' { fn try_downcast(o: &AnyObj) -> Option<Self> { match o.class_id() { ' . implode(' ', $some_arms) . ' _ => {} } None } }');
        // AnyObject
        $w->line('impl php_rt::CastTo<AnyObject> for ' . $h . ' { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }');
        $w->line('impl php_rt::CastTo<' . $h . '> for AnyObject { fn cast_to(self) -> ' . $h . ' { cast::<' . $h . '>(cast::<Mixed>(self)) } }');
        $w->line('impl php_rt::InstanceOf<' . $h . '> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_id(' . $this->program->classId($cls) . ') } }');
        $w->line('impl php_rt::InstanceOf<' . $h . '> for Mixed { fn is_instance(&self) -> bool { self.instance_of_id(' . $this->program->classId($cls) . ') } }');
        if ($cls->isLeaf()) {
            $w->line('impl php_rt::InstanceOf<' . $h . '> for ' . $h . ' { fn is_instance(&self) -> bool { true } }');
        } else {
            $w->line('impl php_rt::InstanceOf<' . $h . '> for ' . $h . ' { fn is_instance(&self) -> bool { match self { ' . $h . '::Other__(__m) => __m.instance_of_id(' . $this->program->classId($cls) . '), _ => true } } }');
            // narrowing to any leaf class (of this crate or a downstream one) in a single generic impl
            // instead of one impl with an arm per descendant for each target class
            $w->line('impl<T: crate::Leaf__ + Clone + \'static> php_rt::CastTo<T> for ' . $h . ' where Mixed: php_rt::CastTo<T> { fn cast_to(self) -> T { if let Some(v) = self.inner_any().downcast_ref::<T>() { return v.clone(); } cast::<T>(cast::<Mixed>(self)) } }');
        }
        $this->emitLeafMarker($cls, $w);
    }

    /** Leaf classes implement the marker trait of their crate and of every upstream crate. */
    public function emitLeafMarker(ClassModel $cls, Writer $w): void
    {
        if (!$cls->isLeaf()) {
            return;
        }
        $h = $cls->handle();
        $w->line('impl crate::Leaf__ for ' . $h . ' {}');
        for ($i = 0; $i < $cls->crate; $i++) {
            $w->line('impl ::' . $this->program->transpiler->crateName($i) . '::Leaf__ for ' . $h . ' {}');
        }
    }

    /** Wrap a concrete own handle value into the handle type of `$cls`. */
    private function wrapConcrete(ClassModel $cls, ClassModel $c, string $code): string
    {
        if ($cls->isLeaf()) {
            return $code;
        }
        if (!in_array($c, $cls->concrete, true)) {
            // a subclass from a downstream crate: the enum has no variant for it
            return $cls->path() . '::Other__(Mixed::Obj(Rc::new(' . $code . ')))';
        }
        return $cls->path() . '::' . $c->variant() . '(' . $code . ')';
    }

    /** Emit all cast/instanceof impls between generated types recorded during body emission. */
    /** @var array<string, bool> */
    private array $done = [];

    /** @param callable(RustType, RustType): ?Writer $select writer for the crate an impl belongs to (null: cannot be written) */
    public function emitRecordedCasts(callable $select): void
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
                $w = $select($from, $to);
                if ($w === null) {
                    continue;
                }
                if (str_starts_with($k, 'is:')) {
                    $this->emitInstanceOf($from, $to, $w);
                } else {
                    $this->emitCast($from, $to, $w);
                }
            }
        }
    }

    /** A class type whose code is not generated (a vendor dependency): no impls can be written for it. */
    private function isExternal(RustType $t): bool
    {
        if ($t->kind === RustType::OPTION) {
            return $this->isExternal($t->inner());
        }
        if ($t->kind !== RustType::CLASS_) {
            return false;
        }
        $c = $this->program->classOf($t);
        return $c !== null && !$c->is_project;
    }

    private function emitCast(RustType $from, RustType $to, Writer $w): void
    {
        if ($this->isExternal($from) || $this->isExternal($to)) {
            return;
        }
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
            $arms[] = $from->mangle() . '::Other__(v) => ' . $this->conv('v', RustType::mixed(), $to);
            $w->line('impl php_rt::CastTo<' . $to->toRust() . '> for ' . $from->toRust() . ' { fn cast_to(self) -> ' . $to->toRust() . ' { match self { ' . implode(', ', $arms) . ' } } }');
            return;
        }
        if ($fk === RustType::UNION && $tk === RustType::BOOL && ($this->casts->hasUnit($from, 'True') || $this->casts->hasUnit($from, 'False'))) {
            return; // emitted with the union enum
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
            $arms[] = $from->mangle() . '::Other__(v) => ' . $this->conv('v', RustType::mixed(), $to);
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
                    // through Mixed: one downcast attempt per union member instead of an arm per descendant
                    // (keeps the generated code small for classes with hundreds of subclasses)
                    $w->line('impl php_rt::CastTo<' . $to->toRust() . '> for ' . $from->toRust() . ' { fn cast_to(self) -> ' . $to->toRust() . ' { ' . $this->conv($this->conv('self', $from, RustType::mixed()), RustType::mixed(), $to) . ' } }');
                    return;
                }
                if (false) {
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
                            $tc = $this->program->classOf($target);
                            $arms[] = $from->toRust() . '::' . $c->variant() . '(v) => ' . $to->mangle() . '::' . $target->variantName() . '(' . ($tc !== null ? $this->wrapConcrete($tc, $c, 'v') : 'v') . ')';
                        } else {
                            $arms[] = $from->toRust() . '::' . $c->variant() . '(v) => ' . $to->mangle() . '::Other__(' . $this->conv('v', RustType::class($c->fqcn), RustType::mixed()) . ')';
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
        if ($fk === RustType::CLASS_ && $tk === RustType::STR) {
            // (string) $object goes through __toString
            $w->line('impl php_rt::CastTo<Str> for ' . $from->toRust() . ' { fn cast_to(self) -> Str { self.to_php_string().unwrap_or_else(|__e| panic!("{}", __e)) } }');
            return;
        }
        if ($fk === RustType::CLASS_ && ($tk === RustType::INT || $tk === RustType::FLOAT || $tk === RustType::BOOL)) {
            $fc = $this->program->classOf($from);
            $has_ts = $fc !== null && $this->program->findMethod($fc, '__tostring') !== null;
            $via = $has_ts ? 'to_num(&Mixed::Str(self.to_php_string().unwrap_or_default()))' : '{ let _ = self; Num::Int(1) }';
            $body = match ($tk) {
                RustType::INT => $via . '.to_i64()',
                RustType::FLOAT => $via . '.to_f64()',
                default => '{ let _ = self; true }',
            };
            $w->line('impl php_rt::CastTo<' . $to->toRust() . '> for ' . $from->toRust() . ' { fn cast_to(self) -> ' . $to->toRust() . ' { ' . $body . ' } }');
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
        if ($from->kind === RustType::ANY_OBJECT && $to->kind === RustType::CLASS_) {
            // any object narrows to a class through Mixed (a class of a later crate travels as AnyObject::Other)
            return true;
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
                $body = $this->wrapConcrete($tc, $fc, 'self');
            } elseif (!$tc->isLeaf()) {
                $body = $th . '::Other__(Mixed::Obj(Rc::new(self)))';
            } else {
                $body = 'panic!(' . Names::rustStringLiteral('cannot cast ' . $fc->fqcn . ' to ' . $tc->fqcn) . ')';
            }
            $w->line('impl php_rt::CastTo<' . $th . '> for ' . $fh . ' { fn cast_to(self) -> ' . $th . ' { ' . $body . ' } }');
            return;
        }
        if ($tc->isLeaf() && $tc->crate >= $fc->crate) {
            // covered by the generic `CastTo<T: Leaf__>` impl of the source enum
            $this->casts->needMixedTo($to);
            return;
        }
        // one arm per concrete class of the source, however large the enum: a conversion through Mixed
        // (an Rc allocation plus a linear downcast chain over every concrete class of the target) on every
        // node upcast/downcast dominated the profile of the parser traversal
        foreach ($fc->concrete as $c) {
            if ($c->isSubclassOf($tc)) {
                $arms[] = $fh . '::' . $c->variant() . '(v) => ' . $this->wrapConcrete($tc, $c, 'v');
            } elseif ($tc->isLeaf()) {
                $arms[] = $fh . '::' . $c->variant() . '(_) => panic!(' . Names::rustStringLiteral('cannot cast ' . $c->fqcn . ' to ' . $tc->fqcn) . ')';
            } else {
                // not an instance of the target: carried through its escape variant
                $arms[] = $fh . '::' . $c->variant() . '(v) => ' . $th . '::Other__(Mixed::Obj(Rc::new(v)))';
            }
        }
        $arms[] = $fh . '::Other__(m) => ' . $this->conv('m', RustType::mixed(), $to);
        $w->line('impl php_rt::CastTo<' . $th . '> for ' . $fh . ' { fn cast_to(self) -> ' . $th . ' { match self { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => unreachable!() } } }');
    }

    private function emitInstanceOf(RustType $subject, RustType $target, Writer $w): void
    {
        if ($this->isExternal($subject) || $this->isExternal($target)) {
            return;
        }
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
            $other = $subject->toRust() . '::Other__(__m) => __m.instance_of_id(' . $this->program->classId($tc) . ')';
            $body = 'match self { ' . ($yes === [] ? '' : implode(' | ', $yes) . ' => true, ') . $other . ', _ => false }';
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
            $this->casts->needInstanceOf(RustType::mixed(), $target);
            $arms[] = $subject->mangle() . '::Other__(v) => is_instance::<' . $th . '>(v)';
            $w->line('impl php_rt::InstanceOf<' . $th . '> for ' . $subject->toRust() . ' { fn is_instance(&self) -> bool { match self { ' . implode(', ', $arms) . ' } } }');
            return;
        }
        // Mixed / AnyObject handled by class impls
    }
}
