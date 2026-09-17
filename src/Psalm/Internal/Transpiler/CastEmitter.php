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
        // Unions are CLOSED: no Other__(Mixed) escape. A value that matches no declared member is a docblock/type
        // lie and PANICS at construction (CastTo<Union> for Mixed below), per the no-Mixed policy.
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
        $w->line('impl php_rt::Truthy for ' . $name . ' { fn truthy(&self) -> bool { match self { ' . implode(', ', $arms) . ' } } }');
        $kind_arms = [];
        $inst_arms = [];
        foreach ($u->params as $m) {
            if ($this->isUnit($m)) {
                $kind_arms[] = $this->memberPat($name, $m, 'v') . ' => ' . ($this->unitName($m) === 'Null' ? 'php_rt::Kind::Null' : 'php_rt::Kind::Bool');
                $inst_arms[] = $this->memberPat($name, $m, 'v') . ' => false';
            } else {
                $kind_arms[] = $this->memberPat($name, $m, 'v') . ' => php_rt::PhpKind::php_kind(v)';
                $inst_arms[] = $this->memberPat($name, $m, 'v') . ' => php_rt::InstanceOfName::php_instance_of(v, __n)';
            }
        }
        $w->line('impl php_rt::PhpKind for ' . $name . ' { fn php_kind(&self) -> php_rt::Kind { match self { ' . implode(', ', $kind_arms) . ' } } }');
        $w->line('impl php_rt::InstanceOfName for ' . $name . ' { fn php_instance_of(&self, __n: &[u8]) -> bool { match self { ' . implode(', ', $inst_arms) . ' } } }');
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
        $w->line('impl ' . $name . ' { pub fn to_php_string(&self) -> Str { self.to_php_str() } }');
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
        $obj_members = array_filter($u->params, static fn(RustType $m) => $m->kind === RustType::CLASS_ || $m->kind === RustType::ANY_OBJECT);
        $fallback = count($obj_members) >= 2
            ? 'match (self.obj_identity(), o.obj_identity()) { (Some(__a), Some(__b)) => __a == __b, _ => false }'
            : 'false';
        $w->line('impl php_rt::Identical for ' . $name . ' { fn identical(&self, o: &Self) -> bool { match (self, o) { ' . implode(', ', $arms) . ', _ => ' . $fallback . ' } } }');
        // PhpCmp: pairwise over the members, PHP's loose comparison rules per kind (no Mixed round trip)
        $w->line('impl php_rt::PhpCmp for ' . $name . ' { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { use std::cmp::Ordering::*; match (self, o) { ' . $this->unionCmpArms($u, $name) . ' } } }');
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
        $w->line('impl php_rt::CastTo<' . $name . '> for Mixed { fn cast_to(self) -> ' . $name . ' { ' . implode(' ', $arms) . ' ' . 'panic!("Mixed value {:?} not in union ' . $name . '", self) } }');
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
        // ToInt/ToFloat/ToArrayKey/Debug: member-wise
        $w->line('impl php_rt::ToInt for ' . $name . ' { fn to_php_int(&self) -> i64 { match self { ' . $this->unionMemberArms($u, $name, 'int') . ' } } }');
        $w->line('impl php_rt::ToFloat for ' . $name . ' { fn to_php_float(&self) -> f64 { match self { ' . $this->unionMemberArms($u, $name, 'float') . ' } } }');
        $w->line('impl php_rt::ToArrayKey for ' . $name . ' { fn to_php_key(&self) -> ArrayKey { match self { ' . $this->unionMemberArms($u, $name, 'key') . ' } } }');
        $w->line('impl std::fmt::Debug for ' . $name . ' { fn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result { match self { ' . $this->unionMemberArms($u, $name, 'debug') . ' } } }');
        $w->line('impl ' . $name . ' { pub fn unwrap_or_default_marker(self) -> Self { self } }');
        if (Casts::unionHasObject($u)) {
            // get_class() / spl_object_id() on a union holding objects
            $cn = [];
            $oi = [];
            foreach ($u->params as $m) {
                $obj = $m->kind === RustType::CLASS_ || $m->kind === RustType::ANY_OBJECT;
                $cn[] = $this->memberPat($name, $m, 'v') . ' => ' . ($obj ? 'php_rt::PhpObject::class_name(v)' : 'panic!("get_class(): Argument #1 ($object) must be of type object")');
                $oi[] = $this->memberPat($name, $m, 'v') . ' => ' . ($obj ? 'php_rt::PhpObject::obj_id(v)' : 'panic!("spl_object_id(): Argument #1 ($object) must be of type object")');
            }
            $ts = [];
            foreach ($u->params as $m) {
                $obj = $m->kind === RustType::CLASS_ || $m->kind === RustType::ANY_OBJECT;
                $ts[] = $this->memberPat($name, $m, 'v') . ' => ' . ($obj ? 'php_rt::PhpObject::php_to_string(v)' : ($m->kind === RustType::STR ? 'Some(v.clone())' : 'None'));
            }
            $ident = [];
            foreach ($u->params as $m) {
                $obj = $m->kind === RustType::CLASS_ || $m->kind === RustType::ANY_OBJECT;
                $ident[] = $this->memberPat($name, $m, 'v') . ' => ' . ($obj ? 'Some(php_rt::PhpObject::obj_id(v))' : 'None');
            }
            $w->line('impl ' . $name . ' { pub fn class_name(&self) -> &\'static str { match self { ' . implode(', ', $cn) . ' } } pub fn obj_id(&self) -> usize { match self { ' . implode(', ', $oi) . ' } } pub fn php_to_string(&self) -> Option<Str> { match self { ' . implode(', ', $ts) . ' } } pub fn obj_identity(&self) -> Option<usize> { match self { ' . implode(', ', $ident) . ' } } }');
        }
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
            // (union is closed — no Other__ arm; the `_ => panic!` fallback below covers a wrong-member extract)
            // A Bool member's `CastTo<bool> for union` (member-extract-or-panic) collides with the truthy-based
            // `CastTo<bool>` emitted below for unit-unions (E0119). The truthy version is the correct PHP bool
            // coercion (and matches the Bool member anyway: truthy(Bool(v)) == v), so skip this one in that case.
            if (!($m->kind === RustType::BOOL && ($this->casts->hasUnit($u, 'True') || $this->casts->hasUnit($u, 'False')))) {
                $w->line('impl php_rt::CastTo<' . $m->toRust() . '> for ' . $name . ' { fn cast_to(self) -> ' . $m->toRust() . ' { match self { ' . $name . '::' . $vn . '(v) => v, ' . $extra . '_ => panic!("union ' . $name . ' is not ' . $vn . '") } } }');
            }
            $w->line('impl php_rt::CastTo<' . $name . '> for ' . $m->toRust() . ' { fn cast_to(self) -> ' . $name . ' { ' . $name . '::' . $vn . '(self) } }');
        }
        if ($this->casts->hasUnit($u, 'True') || $this->casts->hasUnit($u, 'False')) {
            $t = $this->casts->hasUnit($u, 'True') ? $name . '::True => true, ' : '';
            $f = $this->casts->hasUnit($u, 'False') ? $name . '::False => false, ' : '';
            $w->line('impl php_rt::CastTo<bool> for ' . $name . ' { fn cast_to(self) -> bool { match self { ' . $t . $f . 'other => truthy(&other) } } }');
        }
    }

    /** PHP comparison kind of a union member: null, bool, int, float, str, key, arr, obj, closure, other. */
    private function cmpKind(RustType $m): string
    {
        if ($this->isUnit($m)) {
            return $this->unitName($m) === 'Null' ? 'null' : 'bool';
        }
        return match ($m->kind) {
            RustType::BOOL => 'bool',
            RustType::INT => 'int',
            RustType::FLOAT => 'float',
            RustType::STR, RustType::SYM => 'str',
            RustType::ARRAY_KEY => 'key',
            RustType::LIST, RustType::MAP, RustType::SHAPE, RustType::TUPLE => 'arr',
            RustType::CLASS_, RustType::ANY_OBJECT => 'obj',
            RustType::CLOSURE, RustType::DYN_CALLABLE => 'closure',
            default => 'other',
        };
    }

    /** Pattern binding a member of the union to `$var` (unit members bind nothing). */
    private function memberPat(string $name, RustType $m, string $var): string
    {
        return $this->isUnit($m) ? $name . '::' . $this->unitName($m) : $name . '::' . $m->variantName() . '(' . $var . ')';
    }

    private function truthyOf(RustType $m, string $var): string
    {
        if ($this->isUnit($m)) {
            return $this->unitName($m) === 'True' ? 'true' : 'false';
        }
        return match ($this->cmpKind($m)) {
            'obj', 'closure' => 'true',
            'bool' => '(*' . $var . ')',
            default => 'php_rt::Truthy::truthy(' . $var . ')',
        };
    }

    private function strOf(RustType $m, string $var): string
    {
        return $m->kind === RustType::STR ? $var . '.clone()' : 'php_rt::ToStr::to_php_str(' . $var . ')';
    }

    private function numOf(RustType $m, string $var): string
    {
        return $m->kind === RustType::INT ? 'php_rt::Num::Int(*' . $var . ')' : 'php_rt::Num::Float(*' . $var . ')';
    }

    /** One arm of the pairwise union comparison: the rules of php-rt's `PhpCmp for Mixed`, per member kind. */
    private function unionCmpArm(string $name, RustType $a, RustType $b): string
    {
        $pat = '(' . $this->memberPat($name, $a, 'a') . ', ' . $this->memberPat($name, $b, 'b') . ') => ';
        $ka = $this->cmpKind($a);
        $kb = $this->cmpKind($b);
        if ($a === $b) {
            if ($a->kind === RustType::CLOSURE) {
                return $pat . 'if Rc::ptr_eq(a, b) { Equal } else { Greater }';
            }
            if ($ka === 'closure') {
                return $pat . 'if identical(a, b) { Equal } else { Greater }';
            }
            if ($ka === 'null' || $ka === 'bool' && $this->isUnit($a)) {
                return $pat . 'Equal';
            }
            if ($this->cmpableType($a, false)) {
                return $pat . 'a.php_cmp(b)';
            }
            return $pat . 'if identical(a, b) { Equal } else { Greater }';
        }
        $num = fn(string $k) => $k === 'int' || $k === 'float';
        if ($ka === 'null' && $kb === 'null') {
            return $pat . 'Equal';
        }
        if ($ka === 'null' && $kb === 'str') {
            return $pat . 'if ' . $this->strOf($b, 'b') . '.as_bytes().is_empty() { Equal } else { Less }';
        }
        if ($ka === 'str' && $kb === 'null') {
            return $pat . 'if ' . $this->strOf($a, 'a') . '.as_bytes().is_empty() { Equal } else { Greater }';
        }
        if ($ka === 'null') {
            return $pat . 'false.cmp(&' . $this->truthyOf($b, 'b') . ')';
        }
        if ($kb === 'null') {
            return $pat . $this->truthyOf($a, 'a') . '.cmp(&false)';
        }
        if ($ka === 'bool' || $kb === 'bool') {
            return $pat . $this->truthyOf($a, 'a') . '.cmp(&' . $this->truthyOf($b, 'b') . ')';
        }
        if ($num($ka) && $num($kb)) {
            $fa = $ka === 'int' ? '(*a as f64)' : '(*a)';
            $fb = $kb === 'int' ? '(*b as f64)' : '(*b)';
            return $pat . $fa . '.partial_cmp(&' . $fb . ').unwrap_or(Equal)';
        }
        if ($num($ka) && $kb === 'str') {
            return $pat . 'php_rt::cmp_num_str(' . $this->numOf($a, 'a') . ', ' . $this->strOf($b, 'b') . '.as_bytes())';
        }
        if ($ka === 'str' && $num($kb)) {
            return $pat . 'php_rt::cmp_num_str(' . $this->numOf($b, 'b') . ', ' . $this->strOf($a, 'a') . '.as_bytes()).reverse()';
        }
        if ($ka === 'str' && $kb === 'str') {
            return $pat . 'php_rt::cmp_str(' . $this->strOf($a, 'a') . '.as_bytes(), ' . $this->strOf($b, 'b') . '.as_bytes())';
        }
        $keyable = fn(string $k) => in_array($k, ['int', 'float', 'str', 'bool'], true);
        if ($ka === 'key' && $keyable($kb)) {
            return $pat . 'a.php_cmp(&php_rt::ToArrayKey::to_php_key(b))';
        }
        if ($kb === 'key' && $keyable($ka)) {
            return $pat . 'php_rt::ToArrayKey::to_php_key(a).php_cmp(b)';
        }
        if ($ka === 'arr') {
            return $pat . 'Greater';
        }
        if ($kb === 'arr') {
            return $pat . 'Less';
        }
        if ($ka === 'obj' && $kb === 'obj') {
            return $pat . 'if php_rt::PhpObject::obj_id(a) == php_rt::PhpObject::obj_id(b) { Equal } else { Greater }';
        }
        if ($ka === 'obj' && $kb === 'str') {
            return $pat . 'match php_rt::PhpObject::php_to_string(a) { Some(s) => php_rt::cmp_str(s.as_bytes(), ' . $this->strOf($b, 'b') . '.as_bytes()), None => Greater }';
        }
        if ($ka === 'str' && $kb === 'obj') {
            return $pat . 'match php_rt::PhpObject::php_to_string(b) { Some(s) => php_rt::cmp_str(' . $this->strOf($a, 'a') . '.as_bytes(), s.as_bytes()), None => Less }';
        }
        if ($ka === 'obj' || $ka === 'closure') {
            return $pat . 'Greater';
        }
        if ($kb === 'obj' || $kb === 'closure') {
            return $pat . 'Less';
        }
        return $pat . 'Greater';
    }

    /**
     * `$code` (a value of the class type `$m`, a hierarchy member of a source union) wrapped into the target
     * union `$to` through the target members that are subclasses of `$m`: a typed downcast chain, or null.
     */
    private function downcastArm(string $code, RustType $m, RustType $to): ?string
    {
        if ($m->kind !== RustType::CLASS_) {
            return null;
        }
        $mc = $this->program->classOf($m);
        if ($mc === null) {
            return null;
        }
        $subs = [];
        foreach ($to->params as $t) {
            if ($t->kind === RustType::CLASS_ && ($tc = $this->program->classOf($t)) !== null && $tc->isSubclassOf($mc)) {
                $subs[] = $t;
            }
        }
        if ($subs === []) {
            return null;
        }
        if (count($subs) === 1) {
            return $to->mangle() . '::' . $subs[0]->variantName() . '(' . $this->conv($code, $m, $subs[0]) . ')';
        }
        $chain = [];
        foreach ($subs as $t) {
            $this->casts->needInstanceOf($m, $t);
            $chain[] = 'if is_instance::<' . $t->toRust() . '>(&' . $code . ') { ' . $to->mangle() . '::' . $t->variantName() . '(' . $this->conv($code, $m, $t) . ') }';
        }
        return implode(' else ', $chain) . ' else { panic!(' . Names::rustStringLiteral('cannot narrow ' . $m->toRust() . ' into ' . $to->toRust()) . ') }';
    }

    private function unionCmpArms(RustType $u, string $name): string
    {
        $arms = [];
        foreach ($u->params as $a) {
            foreach ($u->params as $b) {
                $arms[] = $this->unionCmpArm($name, $a, $b);
            }
        }
        return implode(', ', $arms);
    }

    /** Member-wise arms of a union's ToInt ('int'), ToFloat ('float'), ToArrayKey ('key') or Debug ('debug'). */
    private function unionMemberArms(RustType $u, string $name, string $what): string
    {
        $arms = [];
        foreach ($u->params as $m) {
            $pat = $this->memberPat($name, $m, 'v') . ' => ';
            $k = $this->cmpKind($m);
            if ($this->isUnit($m)) {
                $lit = ['Null' => ['0', '0.0', 'ArrayKey::Str(Str::from_static(""))', 'write!(f, "null")'], 'True' => ['1', '1.0', 'ArrayKey::Int(1)', 'write!(f, "true")'], 'False' => ['0', '0.0', 'ArrayKey::Int(0)', 'write!(f, "false")']][$this->unitName($m)];
                $arms[] = $pat . $lit[['int' => 0, 'float' => 1, 'key' => 2, 'debug' => 3][$what]];
                continue;
            }
            $scalar = in_array($k, ['bool', 'int', 'float', 'str', 'key'], true);
            $sv = $m->kind === RustType::SYM ? '&php_rt::ToStr::to_php_str(v)' : 'v';
            $arms[] = $pat . match ($what) {
                'int' => $scalar ? 'php_rt::ToInt::to_php_int(' . $sv . ')' : ($k === 'arr' ? '(' . $this->truthyOf($m, 'v') . ') as i64' : '1'),
                'float' => $scalar ? 'php_rt::ToFloat::to_php_float(' . $sv . ')' : ($k === 'arr' ? '(' . $this->truthyOf($m, 'v') . ') as i64 as f64' : '1.0'),
                'key' => $scalar ? 'php_rt::ToArrayKey::to_php_key(' . $sv . ')' : 'panic!(' . Names::rustStringLiteral('Illegal offset type (' . $m->toRust() . ')') . ')',
                'debug' => $k === 'closure' ? 'write!(f, "Closure")' : 'write!(f, "{:?}", v)',
            };
        }
        return implode(', ', $arms);
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
        $w->line('impl php_rt::PhpKind for ' . $name . ' { fn php_kind(&self) -> php_rt::Kind { php_rt::Kind::Arr } }');
        $w->line('impl php_rt::InstanceOfName for ' . $name . ' { fn php_instance_of(&self, _n: &[u8]) -> bool { false } }');
        $w->line('impl php_rt::ToStr for ' . $name . ' { fn to_php_str(&self) -> Str { Str::from_static("Array") } }');
        $parts = [];
        foreach ($s->fields as $k => [$t, $opt]) {
            $parts[] = 'identical(&self.' . Names::field($k) . ', &o.' . Names::field($k) . ')';
        }
        $w->line('impl php_rt::Identical for ' . $name . ' { fn identical(&self, o: &Self) -> bool { ' . ($parts ? implode(' && ', $parts) : 'true') . ' } }');
        // PhpCmp: field by field (PHP compares arrays element-wise) when every field type is comparable
        $cmp_parts = [];
        foreach ($s->fields as $k => [$t, $opt]) {
            $ft = RustType::shapeField($t, $opt);
            if (!$this->cmpableType($ft)) {
                fwrite(STDERR, '  [cmp-skip] shape ' . $name . '.' . $k . ' :: ' . $ft->toRust() . "\n");
                continue;
            }
            $inner = $ft->kind === RustType::OPTION ? $ft->inner() : $ft;
            if (in_array($inner->kind, [RustType::CLOSURE, RustType::DYN_CALLABLE, RustType::RT_GENERIC], true)) {
                $cmp_parts[] = '{ if !identical(&self.' . Names::field($k) . ', &o.' . Names::field($k) . ') { return std::cmp::Ordering::Greater; } }';
            } else {
                $cmp_parts[] = '{ let __c = self.' . Names::field($k) . '.php_cmp(&o.' . Names::field($k) . '); if __c != std::cmp::Ordering::Equal { return __c; } }';
            }
        }
        $w->line('impl php_rt::PhpCmp for ' . $name . ' { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { ' . implode(' ', $cmp_parts) . ' std::cmp::Ordering::Equal } }');
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
        // Debug: field by field (closures and runtime containers print their kind)
        $dbg = '';
        foreach ($s->fields as $k => [$t, $opt]) {
            $ft = RustType::shapeField($t, $opt);
            $inner = $ft->kind === RustType::OPTION ? $ft->inner() : $ft;
            $val = in_array($inner->kind, [RustType::CLOSURE, RustType::DYN_CALLABLE, RustType::RT_GENERIC, RustType::TUPLE], true)
                ? '&' . Names::rustStringLiteral($inner->kind === RustType::TUPLE ? 'array' : 'Closure')
                : '&self.' . Names::field($k);
            $dbg .= '.field(' . Names::rustStringLiteral((string) $k) . ', ' . $val . ')';
        }
        $w->line('impl std::fmt::Debug for ' . $name . ' { fn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result { f.debug_struct("array")' . $dbg . '.finish() } }');
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
        // A CLOSED hierarchy (no downstream-crate subclass) has no `Other__` variant: it dispatches by
        // exhaustive match over its concrete variants, exactly like a leaf, so it takes the leaf path here.
        $closed = !$cls->has_downstream;
        // Truthy / ToStr / Identical / PhpCmp on the handle type
        if ($cls->isLeaf() || $closed) {
            $w->line('impl php_rt::PhpKind for ' . $h . ' { fn php_kind(&self) -> php_rt::Kind { php_rt::Kind::Obj } }');
            $w->line('impl php_rt::InstanceOfName for ' . $h . ' { fn php_instance_of(&self, __n: &[u8]) -> bool { php_rt::PhpObject::class_ancestors(self).iter().any(|a| a.as_bytes().eq_ignore_ascii_case(__n)) } }');
            $w->line('impl php_rt::Truthy for ' . $h . ' { fn truthy(&self) -> bool { true } }');
            $w->line('impl php_rt::Identical for ' . $h . ' { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }');
        } else {
            $w->line('impl php_rt::PhpKind for ' . $h . ' { fn php_kind(&self) -> php_rt::Kind { match self { ' . $h . '::Other__(__m) => php_rt::PhpKind::php_kind(__m), _ => php_rt::Kind::Obj } } }');
            $w->line('impl php_rt::InstanceOfName for ' . $h . ' { fn php_instance_of(&self, __n: &[u8]) -> bool { match self { ' . $h . '::Other__(__m) => php_rt::InstanceOfName::php_instance_of(__m, __n), _ => php_rt::PhpObject::class_ancestors(self).iter().any(|a| a.as_bytes().eq_ignore_ascii_case(__n)) } } }');
            $w->line('impl php_rt::Truthy for ' . $h . ' { fn truthy(&self) -> bool { match self { ' . $h . '::Other__(__m) => truthy(__m), _ => true } } }');
            $w->line('impl php_rt::Identical for ' . $h . ' { fn identical(&self, o: &Self) -> bool { match (self, o) { (' . $h . '::Other__(a), ' . $h . '::Other__(b)) => identical(a, b), (' . $h . '::Other__(_), _) | (_, ' . $h . '::Other__(_)) => false, _ => self.obj_id() == o.obj_id() } } }');
        }
        $w->line('impl php_rt::ToStr for ' . $h . ' { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static(' . Names::rustStringLiteral($cls->fqcn) . ')) } }');
        $num = $this->program->findMethod($cls, '__tostring') !== null ? 'to_num(&Mixed::Str(self.to_php_str()))' : '{ let _ = self; Num::Int(1) }';
        $w->line('impl php_rt::ToInt for ' . $h . ' { fn to_php_int(&self) -> i64 { ' . $num . '.to_i64() } }');
        $w->line('impl php_rt::ToFloat for ' . $h . ' { fn to_php_float(&self) -> f64 { ' . $num . '.to_f64() } }');
        $this->emitHandleCmp($cls, $w);
        $w->line('impl std::fmt::Debug for ' . $h . ' { fn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }');
        // to Mixed: store the concrete own handle
        if ($cls->isLeaf()) {
            $w->line('impl php_rt::CastTo<Mixed> for ' . $h . ' { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }');
        } else {
            $arms = array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(v) => Mixed::Obj(Rc::new(v))', $cls->concrete);
            if (!$closed) {
                $arms[] = $h . '::Other__(m) => m';
            }
            $w->line('impl php_rt::CastTo<Mixed> for ' . $h . ' { fn cast_to(self) -> Mixed { match self { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => unreachable!() } } }');
            if ($cls->isConcrete()) {
                $own = $cls->ownHandle();
                $w->line('impl php_rt::CastTo<Mixed> for ' . $own . ' { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }');
                $w->line('impl php_rt::Truthy for ' . $own . ' { fn truthy(&self) -> bool { true } }');
                $w->line('impl php_rt::PhpKind for ' . $own . ' { fn php_kind(&self) -> php_rt::Kind { php_rt::Kind::Obj } }');
                $w->line('impl php_rt::InstanceOfName for ' . $own . ' { fn php_instance_of(&self, __n: &[u8]) -> bool { php_rt::PhpObject::class_ancestors(self).iter().any(|a| a.as_bytes().eq_ignore_ascii_case(__n)) } }');
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
        $fallback = ($cls->isLeaf() || $closed) ? 'panic!(' . Names::rustStringLiteral('Mixed value is not a ' . $cls->fqcn) . ')' : $h . '::Other__(self)';
        $w->line('impl php_rt::CastTo<' . $h . '> for Mixed { fn cast_to(self) -> ' . $h . ' { if let Mixed::Obj(o) = &self { match o.class_id() { ' . implode(' ', $arms) . ' _ => {} } } ' . $fallback . ' } }');
        // a subclass declared in a downstream crate (Psalm's Virtual* nodes extend php-parser nodes)
        // is still an instance of this class: it lands in the escape variant instead of failing. A closed
        // hierarchy has no downstream subclass, so the concrete match above is exhaustive (else: not this type).
        $none = ($cls->isLeaf() || $closed) ? 'None' : 'if o.instance_of_id(' . $this->program->classId($cls) . ') { Some(' . $h . '::Other__(Mixed::Obj(o.clone()))) } else { None }';
        $w->line('impl php_rt::TryDowncast for ' . $h . ' { fn try_downcast(o: &AnyObj) -> Option<Self> { match o.class_id() { ' . implode(' ', $some_arms) . ' _ => {} } ' . $none . ' } }');
        // AnyObject
        // to AnyObject: the concrete own handle is erased directly (no Mixed round trip)
        if ($cls->isLeaf() && !$cls->isEnum()) {
            $w->line('impl php_rt::CastTo<AnyObject> for ' . $h . ' { fn cast_to(self) -> AnyObject { AnyObject(Rc::new(self)) } }');
        } elseif ($closed && $cls->concrete !== [] && !array_filter($cls->concrete, static fn(ClassModel $c) => $c->isEnum())) {
            $arms = array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(v) => AnyObject(Rc::new(v))', $cls->concrete);
            $w->line('impl php_rt::CastTo<AnyObject> for ' . $h . ' { fn cast_to(self) -> AnyObject { match self { ' . implode(', ', $arms) . ', _ => unreachable!() } } }');
        } else {
            $w->line('impl php_rt::CastTo<AnyObject> for ' . $h . ' { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }');
        }
        // from AnyObject: the hierarchy's class-id match (its TryDowncast)
        $w->line('impl php_rt::CastTo<' . $h . '> for AnyObject { fn cast_to(self) -> ' . $h . ' { php_rt::try_downcast::<' . $h . '>(&self.0).unwrap_or_else(|| panic!(' . Names::rustStringLiteral('object is not a ' . $cls->fqcn) . ')) } }');
        $w->line('impl php_rt::InstanceOf<' . $h . '> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_id(' . $this->program->classId($cls) . ') } }');
        $w->line('impl php_rt::InstanceOf<' . $h . '> for Mixed { fn is_instance(&self) -> bool { self.instance_of_id(' . $this->program->classId($cls) . ') } }');
        if ($cls->isLeaf() || $closed) {
            $w->line('impl php_rt::InstanceOf<' . $h . '> for ' . $h . ' { fn is_instance(&self) -> bool { true } }');
        } else {
            $w->line('impl php_rt::InstanceOf<' . $h . '> for ' . $h . ' { fn is_instance(&self) -> bool { match self { ' . $h . '::Other__(__m) => __m.instance_of_id(' . $this->program->classId($cls) . '), _ => true } } }');
        }
        if (!$cls->isLeaf()) {
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

    /** A non-leaf handle that still carries the `Other__` escape variant (i.e. has a downstream subclass). */
    private function openHandle(?ClassModel $c): bool
    {
        return $c !== null && !$c->isLeaf() && $c->has_downstream;
    }

    /** Wrap a concrete own handle value into the handle type of `$cls`. */
    private function wrapConcrete(ClassModel $cls, ClassModel $c, string $code): string
    {
        if ($cls->isLeaf()) {
            return $code;
        }
        if (!in_array($c, $cls->concrete, true)) {
            // a subclass from a downstream crate: the enum has no variant for it. A closed hierarchy has
            // no such subclass, so this cannot happen there (guarded to a panic rather than a dead Other__).
            $this->casts->noteErasure(RustType::class($c->fqcn));
            return $this->openHandle($cls)
                ? $cls->path() . '::Other__(Mixed::Obj(Rc::new(' . $code . ')))'
                : 'panic!(' . Names::rustStringLiteral('unexpected downstream subclass in closed hierarchy ' . $cls->fqcn) . ')';
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
                } elseif (($down = $this->downcastArm('v', $m, $to)) !== null) {
                    $arms[] = $from->mangle() . '::' . $m->variantName() . '(v) => ' . $down;
                } else {
                    $arms[] = $from->mangle() . '::' . $m->variantName() . '(v) => ' . $this->conv($this->conv('v', $m, RustType::mixed()), RustType::mixed(), $to);
                }
            }
            $w->line('impl php_rt::CastTo<' . $to->toRust() . '> for ' . $from->toRust() . ' { fn cast_to(self) -> ' . $to->toRust() . ' { match self { ' . implode(', ', $arms) . ' } } }');
            return;
        }
        if ($fk === RustType::UNION && $tk === RustType::BOOL && ($this->casts->hasUnit($from, 'True') || $this->casts->hasUnit($from, 'False'))) {
            return; // emitted with the union enum
        }
        if ($fk === RustType::UNION) {
            // If `to` is exactly a member of the union, emitUnion() already emitted this
            // `CastTo<to> for union` impl; re-emitting it here is a duplicate (E0119).
            foreach ($from->params as $um) {
                if (!$this->isUnit($um) && $um->toRust() === $to->toRust()) {
                    return;
                }
            }
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
                } elseif ($tk === RustType::CLASS_ && $m->kind === RustType::CLASS_ && ($mc = $this->program->classOf($m)) !== null
                    && ($tc = $this->program->classOf($to)) !== null && $tc->isSubclassOf($mc)
                ) {
                    // a hierarchy member narrowed to one of its subclasses (typed downcast)
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
                // If `from` is exactly this member, emitUnion() already emitted the
                // `CastTo<union> for from` impl; skip to avoid a duplicate (E0119).
                if ($member->toRust() === $from->toRust()) {
                    return;
                }
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
                            $arms[] = $from->toRust() . '::' . $c->variant() . '(_) => panic!("' . $c->fqcn . ' is not a member of union ' . $to->mangle() . '")';
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
            $w->line('impl php_rt::CastTo<Str> for ' . $from->toRust() . ' { fn cast_to(self) -> Str { self.to_php_string() } }');
            return;
        }
        if ($fk === RustType::CLASS_ && ($tk === RustType::INT || $tk === RustType::FLOAT || $tk === RustType::BOOL)) {
            $fc = $this->program->classOf($from);
            $has_ts = $fc !== null && $this->program->findMethod($fc, '__tostring') !== null;
            $via = $has_ts ? 'to_num(&Mixed::Str(self.to_php_string()))' : '{ let _ = self; Num::Int(1) }';
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
        // SYM (interned name) converts like a string scalar: Sym<->Str/ArrayKey/Int/... via php-rt CastTo impls.
        // Without SYM here, a `Sym|AnyObject` union cast to Str would panic on the Sym arm too (it should convert).
        $scalars = [RustType::INT, RustType::FLOAT, RustType::STR, RustType::BOOL, RustType::ARRAY_KEY, RustType::SYM];
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
            } elseif ($this->openHandle($tc)) {
                $this->casts->noteErasure($from);
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
            } elseif ($this->openHandle($tc)) {
                // not an instance of the target: carried through its escape variant
                $this->casts->noteErasure(RustType::class($c->fqcn));
                $arms[] = $fh . '::' . $c->variant() . '(v) => ' . $th . '::Other__(Mixed::Obj(Rc::new(v)))';
            } else {
                $arms[] = $fh . '::' . $c->variant() . '(_) => panic!(' . Names::rustStringLiteral('cannot cast ' . $c->fqcn . ' to ' . $tc->fqcn) . ')';
            }
        }
        if ($this->openHandle($fc)) {
            $arms[] = $fh . '::Other__(m) => ' . $this->conv('m', RustType::mixed(), $to);
        }
        $w->line('impl php_rt::CastTo<' . $th . '> for ' . $fh . ' { fn cast_to(self) -> ' . $th . ' { match self { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => unreachable!() } } }');
    }

    /**
     * PHP `==`/`<=>` on objects: same class and every property equal (compared in declaration order), typed
     * field by field; different classes are uncomparable (Greater, as php-rt orders them). Classes with a field
     * whose type has no PhpCmp (closures, runtime containers) keep the Mixed-protocol comparison.
     */
    private function emitHandleCmp(ClassModel $cls, Writer $w): void
    {
        $h = $cls->handle();
        if ($cls->isLeaf()) {
            $w->line('impl php_rt::PhpCmp for ' . $h . ' { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { ' . $this->fieldCmpBody($cls) . ' } }');
            return;
        }
        $arms = [];
        foreach ($cls->concrete as $c) {
            $arms[] = '(' . $h . '::' . $c->variant() . '(a), ' . $h . '::' . $c->variant() . '(b)) => a.php_cmp(b)';
        }
        if ($this->openHandle($cls)) {
            // downstream subclasses travel erased (their erasure is recorded where they are wrapped)
            $arms[] = '(' . $h . '::Other__(a), ' . $h . '::Other__(b)) => a.php_cmp(b)';
        }
        $w->line('impl php_rt::PhpCmp for ' . $h . ' { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { match (self, o) { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => std::cmp::Ordering::Greater } } }');
        if ($cls->isConcrete() && $cls->ownHandle() !== $h) {
            // a concrete non-leaf: its own newtype (an enum variant payload) compares field by field too
            $w->line('impl php_rt::PhpCmp for ' . $cls->ownHandle() . ' { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { ' . $this->fieldCmpBody($cls) . ' } }');
        }
    }

    /** Field-by-field comparison of two instances of the same concrete class. */
    private function fieldCmpBody(ClassModel $cls): string
    {
        $parts = ['if self.obj_id() == o.obj_id() { return std::cmp::Ordering::Equal; }'];
        foreach ($cls->fields as $f) {
            $get = $f->isLate() ? $f->acc() . '_opt()' : $f->acc() . '_get()';
            $inner = $f->type->kind === RustType::OPTION ? $f->type->inner() : $f->type;
            if (in_array($inner->kind, [RustType::CLOSURE, RustType::DYN_CALLABLE, RustType::RT_GENERIC], true)) {
                // closures and runtime containers compare by identity (PHP compares closures/iterators by handle)
                $parts[] = '{ if !identical(&self.' . $get . ', &o.' . $get . ') { return std::cmp::Ordering::Greater; } }';
                continue;
            }
            if (!$this->cmpableType($f->type)) {
                // a container of closures: no element comparison exists; the field does not take part
                fwrite(STDERR, '  [cmp-skip] ' . $cls->fqcn . '::$' . $f->name . ' :: ' . $f->type->toRust() . "\n");
                continue;
            }
            $parts[] = '{ let __c = self.' . $get . '.php_cmp(&o.' . $get . '); if __c != std::cmp::Ordering::Equal { return __c; } }';
        }
        $parts[] = 'std::cmp::Ordering::Equal';
        return implode(' ', $parts);
    }

    /** @var array<string, bool> */
    private array $cmpable_classes = [];

    /** Whether every field of every concrete member of the hierarchy has a PhpCmp type. */
    private function cmpableClass(ClassModel $cls): bool
    {
        if (isset($this->cmpable_classes[$cls->fqcn])) {
            return $this->cmpable_classes[$cls->fqcn];
        }
        $this->cmpable_classes[$cls->fqcn] = true; // provisional (cycles through class-typed fields)
        $members = $cls->isLeaf() ? [$cls] : $cls->concrete;
        $ok = true;
        foreach ($members as $m) {
            foreach ($m->fields as $f) {
                if (!$this->cmpableType($f->type)) {
                    $ok = false;
                    break 2;
                }
            }
        }
        return $this->cmpable_classes[$cls->fqcn] = $ok;
    }

    /**
     * Whether values of the type implement php_rt::PhpCmp (and Truthy, for Option members). Closures, callables
     * and runtime containers have no PhpCmp: at the top level (or under Option) a field of such a type is compared
     * by identity instead ($allow_identity), but inside a list/map there is no such fallback.
     */
    private function cmpableType(RustType $t, bool $allow_identity = true): bool
    {
        switch ($t->kind) {
            case RustType::INT:
            case RustType::FLOAT:
            case RustType::BOOL:
            case RustType::STR:
            case RustType::SYM:
            case RustType::UNIT:
            case RustType::ARRAY_KEY:
            case RustType::MIXED:
            case RustType::RESOURCE:
            case RustType::SHAPE:
            case RustType::UNION:
            case RustType::ANY_OBJECT:
                return true;
            case RustType::OPTION:
                // Option<T>: PhpCmp needs T: PhpCmp + Truthy; identity-compared kinds go through Option<T: Identical>
                $inner = $t->inner();
                return in_array($inner->kind, [RustType::INT, RustType::FLOAT, RustType::BOOL, RustType::STR, RustType::SYM, RustType::LIST, RustType::MAP, RustType::CLASS_, RustType::UNION, RustType::SHAPE, RustType::ARRAY_KEY, RustType::MIXED, RustType::CLOSURE, RustType::DYN_CALLABLE, RustType::RT_GENERIC, RustType::GENERIC], true)
                    && $this->cmpableType($inner, $allow_identity);
            case RustType::LIST:
                return $this->cmpableType($t->inner(), false);
            case RustType::MAP:
                return $this->cmpableType($t->params[1], false);
            case RustType::CLASS_:
                $c = $this->program->classOf($t);
                return $c !== null && $c->is_project && !$c->isEnum() && !$c->isTrait();
            case RustType::GENERIC:
                return true; // PhpValue bound
            case RustType::CLOSURE:
            case RustType::DYN_CALLABLE:
            case RustType::RT_GENERIC:
                return $allow_identity; // compared by identity (see fieldCmpBody) when a field/Option field
            case RustType::TUPLE:
                foreach ($t->params as $pt) {
                    if (!$this->cmpableType($pt, false)) {
                        return false;
                    }
                }
                return true;
            default:
                return false;
        }
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
            $other = $this->openHandle($sc)
                ? $subject->toRust() . '::Other__(__m) => __m.instance_of_id(' . $this->program->classId($tc) . '), '
                : '';
            $body = 'match self { ' . ($yes === [] ? '' : implode(' | ', $yes) . ' => true, ') . $other . '_ => false }';
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
            $w->line('impl php_rt::InstanceOf<' . $th . '> for ' . $subject->toRust() . ' { fn is_instance(&self) -> bool { match self { ' . implode(', ', $arms) . ' } } }');
            return;
        }
        // Mixed / AnyObject handled by class impls
    }
}
