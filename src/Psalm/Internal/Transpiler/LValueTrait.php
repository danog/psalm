<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node;
use Closure;
use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\Scalar;

use function preg_match;
use function count;
use function array_map;
use function implode;
use function in_array;
use function is_string;
use function strtolower;

/**
 * Reads and writes of variables, properties and array elements.
 *
 * @internal
 */
trait LValueTrait
{
    /** Build an assignable place for an lvalue expression. */
    public function place(Expr $e): Place
    {
        if ($e instanceof Expr\Variable && is_string($e->name)) {
            $name = $e->name;
            $t = $this->varType($name);
            if ($this->isSuperglobal($name)) {
                // stored by the runtime; modified through a copy that is written back
                return new Place(
                    $t,
                    fn() => 'php_rt::superglobal(' . Names::rustStringLiteral($name) . ')',
                    fn(string $v) => 'php_rt::superglobal_set(' . Names::rustStringLiteral($name) . ', ' . $v . ');',
                );
            }
            if (!empty($this->refvars[$name])) {
                // reference variable: in-place mutation runs inside the target's `with_mut`
                $rn = Names::var($name);
                return new Place(
                    $t,
                    fn() => $this->readVar($name)->code,
                    fn(string $v) => $this->storeVar($name, $v),
                    fn() => '(*__ref)',
                    fn(string $stmt) => $rn . '.with_mut(|__ref| { ' . $stmt . ' });',
                );
            }
            return new Place(
                $t,
                fn() => $this->readVar($name)->code,
                fn(string $v) => $this->storeVar($name, $v),
                fn() => $this->varPlace($name),
            );
        }
        if (($e instanceof Expr\PropertyFetch || $e instanceof Expr\NullsafePropertyFetch) && $e->name instanceof Identifier) {
            $name = $e->name->name;
            $base = $this->receiver($e->var);
            $bt = $base->type;
            $base_was_option = $bt->kind === RustType::OPTION;
            if ($bt->kind === RustType::OPTION) {
                $base = new Val($base->code . '.unwrap()', $bt->inner());
                $bt = $bt->inner();
            }
            if ($bt->kind === RustType::CLASS_) {
                $cls = $this->program->classOf($bt);
                $field = $cls?->fields[$name] ?? null;
                if ($field !== null) {
                    $rn = $field->acc();
                    $bc = $base->code;
                    // Immutable Rc<T> classes have no RefCell: set_/mut do Rc::make_mut copy-on-write, which needs
                    // the ACTUAL binding (`c`), not a throwaway `c.clone()` — a write through the clone COWs the
                    // copy and is silently lost. When the base is a plain local (`$c` of a wither
                    // `$c = clone $this; $c->prop = x`), write/mut through the raw `let mut` binding so make_mut COWs
                    // `c` in place (exactly PHP clone-then-mutate). Reads still go through the owned `c.clone()`.
                    // Inert for current immutables (all write-free); unblocks withered immutable hierarchies.
                    if ($cls !== null && $cls->immutable()
                        && $e->var instanceof Expr\Variable && is_string($e->var->name) && $e->var->name !== 'this'
                        && $this->isWritableLocal($e->var->name)
                    ) {
                        // The raw binding has the local's STORAGE type, which may be Option<Clause> even when Psalm has
                        // narrowed the read to non-null Clause. set_/mut need `&mut T`, so build a `&mut` into the stored
                        // value: `.get_mut()` for a Late<T> local (e.g. a `$c = clone $this` wither clone-local), or
                        // `.as_mut().unwrap()` for an Option<T> local. Read keeps the already-narrowed base code.
                        $vn = $e->var->name;
                        $bind = Names::var($vn);
                        if (!empty($this->late[$vn])) {
                            $bind .= '.get_mut()';
                        } elseif ($this->varType($vn)->kind === RustType::OPTION) {
                            $bind .= '.as_mut().unwrap()';
                        }
                        // NB: writing a concrete-variant-specific field through an enum/interface-typed local (the
                        // MutableTypeVisitor pattern, `$self:TypeNode` narrowed to TNamedObject) is NOT handled here —
                        // a typed enum dispatch accessor fails because such fields are heterogeneously typed across
                        // variants (E0308). That pattern needs the visitor retyped; the class then stays non-immutable.
                        return new Place(
                            $field->type,
                            fn() => $bc . '.' . $rn . '_get()',
                            // Hoist the value before the `get_mut()`/make_mut receiver borrow: the RHS may read the same
                            // object (`$c->x = f($c->y)` in a wither), which would be an immutable borrow while the
                            // receiver is mutably borrowed (E0502). Evaluating it into __wv first releases that borrow.
                            fn(string $v) => '{ let __wv = ' . $v . '; ' . $bind . '.set_' . $rn . '(__wv); }',
                            fn() => '(*' . $bind . '.' . $rn . '_mut())',
                        );
                    }
                    return new Place(
                        $field->type,
                        fn() => $bc . '.' . $rn . '_get()',
                        fn(string $v) => $bc . '.set_' . $rn . '(' . $v . ');',
                        fn() => '(*' . $bc . '.' . $rn . '_mut())',
                    );
                }
                if ($cls !== null && ($vf = $this->program->variantField($cls, $name)) !== null) {
                    // a field of some concrete variants, reached through the hierarchy handle
                    [$sf, $vt] = $vf;
                    $rn = $sf->acc();
                    $bc = $base->code;
                    return new Place(
                        $vt,
                        fn() => $bc . '.' . $rn . '_get()',
                        fn(string $v) => '{ let __wv = ' . $v . '; let mut __b = ' . $bc . '; __b.set_' . $rn . '(__wv); }',
                    );
                }
                $this->warn('unknown property ' . $name . ' on ' . $bt->toRust(), $e);
            }
            if ($bt->kind === RustType::RT_GENERIC && $bt->name === 'StdClass') {
                $bc = $base->code;
                return new Place(
                    RustType::mixed(),
                    fn() => $bc . '.get_or_null(' . Names::strLit($name) . ')',
                    fn(string $v) => $bc . '.set(' . Names::strLit($name) . ', ' . $v . ');',
                );
            }
            if ($bt->kind === RustType::UNION) {
                $inf = $this->inferredOrMixed($e);
                if ($inf->kind === RustType::MIXED) {
                    // a write target has no inferred type: the place is typed by the members' field types
                    $ftypes = [];
                    foreach ($bt->params as $m) {
                        $mc = $m->kind === RustType::CLASS_ ? $this->program->classOf($m) : null;
                        if ($mc === null) {
                            continue;
                        }
                        $mf = $mc->fields[$name] ?? null;
                        if ($mf !== null) {
                            $ftypes[] = $mf->type;
                        } elseif (($vf = $this->program->variantField($mc, $name)) !== null) {
                            $ftypes[] = $vf[1];
                        }
                    }
                    if ($ftypes !== []) {
                        $inf = $this->program->unionOfRust($ftypes) ?? $inf;
                    }
                }
                $arms_get = [];
                $arms_set = [];
                foreach ($bt->params as $m) {
                    if ($m->kind !== RustType::CLASS_) {
                        continue;
                    }
                    $cls = $this->program->classOf($m);
                    $field = $cls?->fields[$name] ?? null;
                    $ftype = $field?->type;
                    if ($field === null && $cls !== null && ($vf = $this->program->variantField($cls, $name)) !== null) {
                        [$field, $ftype] = $vf;
                    }
                    if ($field === null) {
                        continue;
                    }
                    $arms_get[] = $bt->mangle() . '::' . $m->variantName() . '(__o) => ' . $this->casts->convert('__o.' . $field->acc() . '_get()', $ftype, $inf);
                    $arms_set[] = $bt->mangle() . '::' . $m->variantName() . '(mut __o) => __o.set_' . $field->acc() . '(' . $this->casts->convert('__v', $inf, $ftype) . ')';
                }
                if ($arms_get !== []) {
                    $bc = $base->code;
                    // NB: an immutable Rc<T> union member's set_ is `&mut self` and can't be dispatched on the owned
                    // `$bc` match here (would need &mut of the narrowed value, but the raw binding has the wider storage
                    // union type — matching `&mut binding` with narrowed arms fails E0308). Rare (Atomic TCallable/
                    // TClosure union writes under HIER_CONCRETE); left as-is (RefCell members' &self set_ works).
                    return new Place(
                        $inf,
                        fn() => '(match ' . $bc . ' { ' . implode(', ', $arms_get) . ', _ => unreachable!() })',
                        fn(string $v) => '{ let __v = ' . $v . '; match ' . $bc . ' { ' . implode(', ', $arms_set) . ', _ => unreachable!() } }',
                    );
                }
            }
            if ($bt->kind === RustType::MIXED || $bt->kind === RustType::ANY_OBJECT || $bt->kind === RustType::CLASS_ || $bt->kind === RustType::UNION) {
                $bc = $this->casts->convert($base->code, $bt, RustType::mixed());
                $inf = $this->inferredOrMixed($e);
                return new Place(
                    $inf,
                    fn() => $this->casts->convert('mixed_prop(&' . $bc . ', &' . $this->dynPropName($name) . ').unwrap_or_default()', RustType::mixed(), $inf),
                    fn(string $v) => 'mixed_set_prop(&' . $bc . ', &' . $this->dynPropName($name) . ', ' . $this->casts->convert($v, $inf, RustType::mixed()) . ');',
                );
            }
            $this->warn('property access on ' . $bt->toRust(), $e);
            return $this->deadPlace($this->inferredOrMixed($e));
        }
        if (($e instanceof Expr\PropertyFetch || $e instanceof Expr\NullsafePropertyFetch) && !$e->name instanceof Identifier) {
            // `$obj->$name`: properties are never looked up by name (closed world)
            $this->warn('dynamic property name', $e);
            return $this->deadPlace($this->inferredOrMixed($e));
        }
        if ($e instanceof Expr\StaticPropertyFetch && $e->name instanceof Node\VarLikeIdentifier && $e->class instanceof Name) {
            $fqcn = $this->resolveClassName($e->class);
            $cls = $fqcn !== null ? $this->program->getClass($fqcn) : null;
            $field = $this->findStaticField($cls, $e->name->name);
            if ($field !== null) {
                $path = $field->declaring->path();
                $rn = $field->rustName();
                return new Place(
                    $field->type,
                    fn() => $path . '::st_' . $rn . '()',
                    fn(string $v) => $path . '::st_' . $rn . '_set(' . $v . ');',
                    fn() => '(*__sp)',
                    fn(string $stmt) => $path . '::st_' . $rn . '_with(|__sp| { ' . $stmt . ' });',
                );
            }
            $this->warn('unknown static property', $e);
            return $this->deadPlace($this->inferredOrMixed($e));
        }
        if ($e instanceof Expr\ArrayDimFetch) {
            return $this->dimPlace($e);
        }
        if ($e instanceof Expr\FuncCall || $e instanceof Expr\MethodCall || $e instanceof Expr\StaticCall || $e instanceof Expr\New_
            || $e instanceof Expr\Ternary || $e instanceof Expr\BinaryOp || $e instanceof Expr\Array_
        ) {
            // an rvalue used where a place is expected (`array_pop(f())`): modifications apply to a temporary
            $v = $this->expr($e);
            return new Place($v->type, fn() => $v->code, fn(string $w) => '{ let _ = ' . $w . '; }');
        }
        $this->warn('unsupported lvalue ' . $e->getType(), $e);
        return $this->deadPlace($this->inferredOrMixed($e));
    }

    private function deadPlace(RustType $t): Place
    {
        return new Place($t, fn() => $this->deadCode('bad lvalue', $t), fn(string $v) => '{ let _ = ' . $v . '; }');
    }

    public function findStaticField(?ClassModel $cls, string $name): ?FieldModel
    {
        while ($cls !== null) {
            if (isset($cls->static_fields[$name])) {
                return $cls->static_fields[$name];
            }
            $cls = $cls->parent;
        }
        return null;
    }

    /**
     * Evaluates `$codes` into temporaries (unless trivially side-effect free) before `$f` builds a statement
     * from them, so that a mutable borrow taken by the statement can't overlap their evaluation.
     *
     * @param list<string> $codes
     * @param Closure(string...): string $f
     */
    private function hoisted(array $codes, Closure $f): string
    {
        $pre = '';
        $names = [];
        foreach ($codes as $c) {
            if (preg_match('/^(?:[A-Za-z_][A-Za-z0-9_]*(?:\.clone\(\))?|-?[0-9]+(?:i64|usize|\.0)?|true|false|None|Str::from_static\("[^"\\]*"\)|ArrayKey::from_str\("[^"\\]*"\)|ArrayKey::Int\(-?[0-9]+\))$/', $c)) {
                $names[] = $c;
                continue;
            }
            $t = $this->tmp('__h');
            $pre .= 'let ' . $t . ' = ' . $c . '; ';
            $names[] = $t;
        }
        $stmt = $f(...$names);
        return $pre === '' ? $stmt : '{ ' . $pre . $stmt . ' }';
    }

    private function dimPlace(Expr\ArrayDimFetch $e): Place
    {
        $parent = $this->place($e->var);
        $pt = $parent->type;
        if ($pt->kind === RustType::OPTION) {
            // writing through a nullable container autovivifies it
            $inner = $pt->inner();
            $p = $parent;
            $parent = new Place(
                $inner,
                fn() => $p->read() . '.unwrap_or_default()',
                fn(string $v) => $p->write('Some(' . $v . ')'),
                $p->hasMut() ? fn() => '(*' . $p->mut() . ($inner->hasDefault() ? '.get_or_insert_with(Default::default))' : '.as_mut().expect("null container"))') : null,
                fn(string $s) => $p->wrap($s),
            );
            $pt = $inner;
        }
        if ($pt->kind === RustType::UNION) {
            // a union with a single array member (the parser's semantic values, `T|list<T>` ...): the
            // element is written through that member instead of a Mixed round trip
            $member = null;
            foreach ($pt->params as $m) {
                if ($m->kind === RustType::LIST || $m->kind === RustType::MAP) {
                    $member = $member === null ? $m : false;
                }
            }
            if ($member instanceof RustType) {
                $u = $pt;
                $p = $parent;
                $vn = $member->variantName();
                $un = $u->toRust();
                $parent = new Place(
                    $member,
                    fn() => '(match ' . $p->read() . ' { ' . $un . '::' . $vn . '(__a) => __a, _ => Default::default() })',
                    fn(string $v) => $p->write($un . '::' . $vn . '(' . $v . ')'),
                    $p->hasMut() ? fn() => '(*' . $p->mut() . '.' . $vn . '_or_insert())' : null,
                    fn(string $s) => $p->wrap($s),
                );
                $pt = $member;
            }
        }
        $dim = $e->dim;

        $has_mut = $parent->hasMut();
        $wrap = fn(string $s) => $parent->wrap($s);
        if ($pt->kind === RustType::LIST) {
            $vt = $pt->inner();
            if ($dim === null) {
                return new Place(
                    $vt,
                    fn() => $vt->hasDefault() ? $this->casts->defaultOf($vt) : $this->deadCode('read of $a[]', $vt), // `$a[]['k'] = v`: a fresh element
                    fn(string $v) => $this->hoisted([$v], fn(string $v) => $parent->modify(fn(string $p) => $p . '.push(' . $v . ');')),
                );
            }
            $key = fn() => $this->exprTo($dim, RustType::int());
            return new Place(
                $vt,
                fn() => $parent->read() . '.idx(' . $key() . ').clone()',
                fn(string $v) => $this->hoisted([$key(), $v], fn(string $k, string $v) => $parent->modify(fn(string $p) => $p . '.set(' . $k . ', ' . $v . ');')),
                $has_mut ? fn() => '(*' . $parent->mut() . '.idx_mut(' . $key() . '))' : null,
                $wrap,
            );
        }
        if ($pt->kind === RustType::MAP) {
            [$kt, $vt] = $pt->params;
            if ($dim === null) {
                return new Place(
                    $vt,
                    fn() => $vt->hasDefault() ? $this->casts->defaultOf($vt) : $this->deadCode('read of $a[]', $vt), // `$a[]['k'] = v`: a fresh element
                    fn(string $v) => $this->hoisted([$v], fn(string $v) => $parent->modify(fn(string $p) => $p . '.push(' . $v . ');')),
                );
            }
            $key = fn() => $this->keyExpr($dim, $kt);
            $mut = !$has_mut ? null : ($vt->hasDefault()
                ? fn() => '(*' . $parent->mut() . '.entry_or_default(' . $key() . '))'
                : fn() => '(*' . $parent->mut() . '.idx_mut(&' . $key() . '))');
            return new Place(
                $vt,
                fn() => $parent->read() . '.idx(&' . $key() . ').clone()',
                fn(string $v) => $this->hoisted([$key(), $v], fn(string $k, string $v) => $parent->modify(fn(string $p) => $p . '.insert(' . $k . ', ' . $v . ');')),
                $mut,
                $wrap,
            );
        }
        if ($pt->kind === RustType::TUPLE && $dim !== null) {
            $k = $this->literalKey($dim);
            if ($k !== null && isset($pt->params[(int) $k])) {
                $i = (int) $k;
                $vt = $pt->params[$i];
                return new Place(
                    $vt,
                    fn() => $parent->read() . '.' . $i,
                    fn(string $v) => $this->hoisted([$v], fn(string $v) => $parent->modify(fn(string $p) => $p . '.' . $i . ' = ' . $v . ';')),
                    $has_mut ? fn() => $parent->mut() . '.' . $i : null,
                    $wrap,
                );
            }
        }
        if ($pt->kind === RustType::SHAPE && $dim !== null) {
            $k = $this->literalKey($dim);
            if ($k !== null && isset($pt->fields[$k])) {
                [$vt, $opt] = $pt->fields[$k];
                $rn = Names::field($k);
                if ($opt) {
                    $st = RustType::shapeField($vt, true);
                    return new Place(
                        $vt,
                        fn() => $this->casts->convert($parent->read() . '.' . $rn, $st, $vt),
                        fn(string $v) => $this->hoisted([$this->casts->convert($v, $vt, $st)], fn(string $v) => $parent->modify(fn(string $p) => $p . '.' . $rn . ' = ' . $v . ';')),
                        $has_mut ? fn() => '(*' . $parent->mut() . '.' . $rn . '.get_or_insert_with(Default::default))' : null,
                        $wrap,
                    );
                }
                return new Place(
                    $vt,
                    fn() => $parent->read() . '.' . $rn,
                    fn(string $v) => $this->hoisted([$v], fn(string $v) => $parent->modify(fn(string $p) => $p . '.' . $rn . ' = ' . $v . ';')),
                    $has_mut ? fn() => $parent->mut() . '.' . $rn : null,
                    $wrap,
                );
            }
            if ($k !== null) {
                $this->warn('write to unknown shape key ' . $k, $e);
            }
        }
        if ($pt->kind === RustType::STR && $dim !== null) {
            $key = fn() => $this->exprTo($dim, RustType::int());
            return new Place(
                RustType::str(),
                fn() => 'str_index(&' . $parent->read() . ', ' . $key() . ')',
                fn(string $v) => $this->hoisted([$key(), $v], fn(string $k, string $v) => $parent->modify(fn(string $p) => 'str_set_index(&mut ' . $p . ', ' . $k . ', &' . $v . ');')),
            );
        }
        if ($pt->kind === RustType::CLASS_) {
            // ArrayAccess
            $cls = $this->program->classOf($pt);
            $get = $cls !== null ? $this->program->findMethod($cls, 'offsetget') : null;
            $set = $cls !== null ? $this->program->findMethod($cls, 'offsetset') : null;
            if ($get !== null && $set !== null) {
                $kt = $set->param_types[0] ?? RustType::mixed();
                $vt = $set->param_types[1] ?? RustType::mixed();
                $key = fn() => $dim === null ? 'None' : $this->exprTo($dim, $kt);
                return new Place(
                    $get->return_type,
                    fn() => $parent->read() . '.' . $get->rustName() . '(' . $this->exprTo($dim, $get->param_types[0] ?? RustType::mixed()) . ')',
                    fn(string $v) => $parent->read() . '.' . $set->rustName() . '(' . $key() . ', ' . $this->casts->convert($v, $get->return_type, $vt) . ');',
                );
            }
        }
        if ($pt->kind === RustType::MIXED) {
            $key = fn() => $dim === null ? 'None' : 'Some(' . $this->keyExpr($dim, RustType::arrayKey()) . ')';
            return new Place(
                RustType::mixed(),
                fn() => $dim === null ? 'Mixed::Null' : 'mixed_get(&' . $parent->read() . ', &' . $this->keyExpr($dim, RustType::arrayKey()) . ').unwrap_or_default()',
                fn(string $v) => $this->hoisted([$key(), $v], fn(string $k, string $v) => $parent->modify(fn(string $p) => 'mixed_set(&mut ' . $p . ', ' . $k . ', ' . $v . ');')),
                $has_mut ? fn() => '(*mixed_entry(&mut ' . $parent->mut() . ', ' . $key() . '))' : null,
                $wrap,
            );
        }
        if ($pt->kind === RustType::UNION || $pt->kind === RustType::ANY_OBJECT || $pt->kind === RustType::CLASS_ || $pt->kind === RustType::SHAPE || $pt->kind === RustType::TUPLE) {
            // a union of array forms, a shape written with a runtime key, or an ArrayAccess object typed
            // loosely: modified as a Mixed value
            $key = fn() => $dim === null ? 'None' : 'Some(' . $this->keyExpr($dim, RustType::arrayKey()) . ')';
            $to_mixed = fn(string $p) => $this->casts->convert($p, $pt, RustType::mixed());
            return new Place(
                RustType::mixed(),
                fn() => $dim === null ? 'Mixed::Null' : 'mixed_get(&' . $to_mixed($parent->read()) . ', &' . $this->keyExpr($dim, RustType::arrayKey()) . ').unwrap_or_default()',
                fn(string $v) => $this->hoisted([$key(), $v], fn(string $k, string $v) => $parent->modify(fn(string $p) => '{ let mut __mm = ' . $to_mixed($p . '.clone()') . '; mixed_set(&mut __mm, ' . $k . ', ' . $v . '); ' . $p . ' = ' . $this->casts->convert('__mm', RustType::mixed(), $pt) . '; }')),
            );
        }
        if ($pt->kind === RustType::RT_GENERIC && in_array($pt->name, ['ArrayObject', 'ArrayIterator', 'SplObjectStorage', 'WeakMap'], true) && $dim !== null) {
            // runtime containers with interior mutability: the parent is read (a shared handle), not modified
            [$kt, $vt] = $pt->params;
            $setter = $pt->name === 'SplObjectStorage' || $pt->name === 'WeakMap' ? 'attach' : 'set';
            $key = fn() => $this->exprTo($dim, $kt);
            return new Place(
                $vt,
                fn() => $parent->read() . '.idx(&' . $key() . ')',
                fn(string $v) => $this->hoisted([$key(), $v], fn(string $k, string $v) => $parent->read() . '.' . $setter . '(' . $k . ', ' . $v . ');'),
            );
        }
        $this->warn('array write on ' . $pt->toRust(), $e);
        return $this->deadPlace($this->inferredOrMixed($e));
    }

    // ------------------------------------------------------------------ reads

    private function dimFetch(Expr\ArrayDimFetch $e): Val
    {
        $__v = $this->dimFetchInner($e);
        return $__v;
    }

    private function dimFetchInner(Expr\ArrayDimFetch $e): Val
    {
        if ($e->dim === null) {
            $this->warn('read of $a[]', $e);
            return $this->dead('read of $a[]', RustType::never());
        }
        $base = $this->expr($e->var);
        $bt = $base->type;
        if ($bt->kind === RustType::OPTION) {
            $ov = $this->optionalValue($e);
            if ($ov !== null) {
                return $this->narrowOptional($ov, $e);
            }
            $base = new Val($base->code . '.unwrap()', $bt->inner());
            $bt = $bt->inner();
        }
        $dim = $e->dim;
        $pu = $this->possiblyUndefined($e);
        if ($bt->kind === RustType::LIST) {
            $vt = $bt->inner();
            $idx = $this->exprTo($dim, RustType::int());
            if ($pu) {
                return $this->narrowOptional(new Val($base->code . '.get(' . $idx . ').cloned()', RustType::option($vt)), $e);
            }
            return $this->narrow(new Val($base->code . '.idx(' . $idx . ').clone()', $vt), $e);
        }
        if ($bt->kind === RustType::MAP) {
            [$kt, $vt] = $bt->params;
            $key = $this->keyExpr($dim, $kt);
            if ($pu) {
                return $this->narrowOptional(new Val($base->code . '.get(&' . $key . ').cloned()', RustType::option($vt)), $e);
            }
            return $this->narrow(new Val($base->code . '.idx(&' . $key . ').clone()', $vt), $e);
        }
        if ($bt->kind === RustType::TUPLE) {
            $k = $this->literalKey($dim);
            if ($k !== null && isset($bt->params[(int) $k])) {
                return $this->narrow(new Val($base->code . '.' . (int) $k, $bt->params[(int) $k]), $e);
            }
            $lt = RustType::list($this->types()->combine($bt->params));
            $conv = $this->casts->convert($base->code, $bt, $lt);
            return $this->narrow(new Val($conv . '.idx(' . $this->exprTo($dim, RustType::int()) . ').clone()', $lt->inner()), $e);
        }
        if ($bt->kind === RustType::SHAPE) {
            $k = $this->literalKey($dim);
            if ($k !== null && isset($bt->fields[$k])) {
                [$ft, $opt] = $bt->fields[$k];
                $rn = Names::field($k);
                if ($opt) {
                    return $this->narrowOptional(new Val($base->code . '.' . $rn, RustType::shapeField($ft, true)), $e);
                }
                return $this->narrow(new Val($base->code . '.' . $rn, $ft), $e);
            }
            if ($k !== null) {
                $this->warn('read of unknown shape key ' . $k, $e);
                return $this->dead('unknown key', $this->inferredOrMixed($e));
            }
            $mt = RustType::map(RustType::arrayKey(), $this->shapeValueType($bt));
            $conv = $this->casts->convert($base->code, $bt, $mt);
            return $this->narrow(new Val($conv . '.idx(&' . $this->keyExpr($dim, RustType::arrayKey()) . ').clone()', $mt->params[1]), $e);
        }
        if ($bt->kind === RustType::STR) {
            return new Val('str_index(&' . $base->code . ', ' . $this->exprTo($dim, RustType::int()) . ')', RustType::str());
        }
        if ($bt->kind === RustType::MIXED) {
            $k = $this->expr($dim);
            $code = 'mixed_get(&' . $base->code . ', &' . $this->keyFrom($k, RustType::arrayKey()) . ')';
            return $this->narrowOptional(new Val($code, RustType::option(RustType::mixed())), $e);
        }
        if ($bt->kind === RustType::CLASS_) {
            $cls = $this->program->classOf($bt);
            $get = $cls !== null ? $this->program->findMethod($cls, 'offsetget') : null;
            if ($get !== null) {
                return $this->narrow(new Val($base->code . '.' . $get->rustName() . '(' . $this->exprTo($dim, $get->param_types[0] ?? RustType::mixed()) . ')', $get->return_type), $e);
            }
        }
        if ($bt->kind === RustType::RT_GENERIC && in_array($bt->name, ['ArrayObject', 'ArrayIterator', 'SplObjectStorage', 'WeakMap'], true)) {
            $k = $this->exprTo($dim, $bt->params[0]);
            if ($pu) {
                return $this->narrowOptional(new Val($base->code . '.get(&' . $k . ')', RustType::option($bt->params[1])), $e);
            }
            return $this->narrow(new Val($base->code . '.idx(&' . $k . ')', $bt->params[1]), $e);
        }
        if ($bt->kind === RustType::UNION) {
            $ui = $this->unionIndex($base->code, $bt, $this->keyExpr($dim, RustType::arrayKey()));
            if ($ui !== null) {
                return $this->narrowOptional(new Val($ui[0], RustType::option($ui[1])), $e);
            }
            // array access on a union without array members (ArrayAccess objects): resolved dynamically
            $code = 'mixed_get(&' . $this->casts->convert($base->code, $bt, RustType::mixed()) . ', &' . $this->keyExpr($dim, RustType::arrayKey()) . ')';
            return $this->narrowOptional(new Val($code, RustType::option(RustType::mixed())), $e);
        }
        $this->warn('array read on ' . $bt->toRust(), $e);
        return $this->dead('array read on ' . $bt->toRust() . '', $this->inferredOrMixed($e));
    }

    /**
     * `$u[$k]` on a union with array members: a match over the list/map members (`None` for the others),
     * as `[code, element type]`; null when the union has no array member or their element types cannot join.
     *
     * @return array{string, RustType}|null
     */
    public function unionIndex(string $base, RustType $u, string $key): ?array
    {
        $elems = [];
        foreach ($u->params as $m) {
            if ($m->kind === RustType::LIST) {
                $elems[] = $m->inner();
            } elseif ($m->kind === RustType::MAP) {
                $elems[] = $m->params[1];
            }
        }
        if ($elems === []) {
            return null;
        }
        $et = $this->program->unionOfRust($elems);
        if ($et === null) {
            return null;
        }
        $arms = [];
        foreach ($u->params as $m) {
            if ($m->kind === RustType::LIST) {
                $arms[] = $u->mangle() . '::' . $m->variantName() . '(__l) => __l.get(php_rt::ToInt::to_php_int(&__k)).cloned().map(|__v| ' . $this->casts->convert('__v', $m->inner(), $et) . ')';
            } elseif ($m->kind === RustType::MAP) {
                $arms[] = $u->mangle() . '::' . $m->variantName() . '(__m) => __m.get(&' . $this->casts->convert('__k.clone()', RustType::arrayKey(), $m->params[0]) . ').cloned().map(|__v| ' . $this->casts->convert('__v', $m->params[1], $et) . ')';
            }
        }
        return ['{ let __k = ' . $key . '; match ' . $base . ' { ' . implode(', ', $arms) . ', _ => None } }', $et];
    }

    /** Narrow an Option-typed read to what Psalm inferred (unwrapping when Psalm says it is defined). */
    private function narrowOptional(Val $v, Expr $e): Val
    {
        $inf = $this->inferred($e);
        if ($inf === null || ($inf->kind === RustType::MIXED && $v->type->inner()->kind !== RustType::MIXED)) {
            // Psalm lost the type (a deep alias read): the statically typed read stands
            return $v;
        }
        if ($this->possiblyUndefined($e)) {
            if ($inf->kind === RustType::OPTION || $inf->kind === RustType::MIXED) {
                return new Val($this->casts->convert($v->code, $v->type, $inf), $inf);
            }
            return new Val($this->casts->convert($v->code, $v->type, RustType::option($inf)), RustType::option($inf));
        }
        return new Val($this->casts->convert($v->code, $v->type, $inf), $inf);
    }

    private function propertyFetch(Expr\PropertyFetch|Expr\NullsafePropertyFetch $e, bool $nullsafe): Val
    {
        if (!$e->name instanceof Identifier) {
            $base = $this->receiver($e->var);
            $name = $this->exprTo($e->name, RustType::str());
            $code = 'mixed_prop(&' . $this->casts->convert($base->code, $base->type, RustType::mixed()) . ', &' . $name . ')';
            return $this->narrowOptional(new Val($code, RustType::option(RustType::mixed())), $e);
        }
        $name = $e->name->name;
        // the receiver of `?->` is read with its declared type: Psalm narrows it to non-null for the fetch itself
        $base = $nullsafe ? $this->rawValue($e->var) : $this->receiver($e->var);
        $bt = $base->type;
        if ($bt->kind === RustType::OPTION) {
            if ($nullsafe) {
                $ov = $this->optionalValue($e);
                if ($ov !== null) {
                    return $this->narrowOptional($ov, $e);
                }
                // `$x?->prop` on a nullable class: null short-circuits
                $inner = $bt->inner();
                $cls = $inner->kind === RustType::CLASS_ ? $this->program->classOf($inner) : null;
                $field = $cls?->fields[$name] ?? null;
                if ($field !== null) {
                    $ft = $field->type;
                    if ($ft->kind === RustType::OPTION) {
                        return $this->narrow(new Val('(match ' . $base->code . ' { Some(__b) => __b.' . $field->acc() . '_get(), None => None })', $ft), $e);
                    }
                    return $this->narrow(new Val('(match ' . $base->code . ' { Some(__b) => Some(__b.' . $field->acc() . '_get()), None => None })', RustType::option($ft)), $e);
                }
            }
            $base = new Val($base->code . '.unwrap()', $bt->inner());
            $bt = $bt->inner();
        }
        if ($bt->kind === RustType::CLASS_) {
            $cls = $this->program->classOf($bt);
            $field = $cls?->fields[$name] ?? null;
            if ($field !== null) {
                return $this->narrow(new Val($base->code . '.' . $field->acc() . '_get()', $field->type), $e);
            }
            if ($cls !== null && ($vf = $this->program->variantField($cls, $name)) !== null) {
                return $this->narrow(new Val($base->code . '.' . $vf[0]->acc() . '_get()', $vf[1]), $e);
            }
            $enum_iface = in_array(strtolower($cls->fqcn ?? ''), ['unitenum', 'backedenum'], true);
            if ($cls !== null && ($name === 'name' || $name === 'value') && ($cls->isEnum() || $enum_iface || ($cls->concrete !== [] && !array_filter($cls->concrete, static fn(ClassModel $c) => !$c->isEnum())))) {
                $backing = $cls->isEnum() ? (string) $cls->storage->enum_type : (string) ($cls->concrete[0]->storage->enum_type ?? ($enum_iface && strtolower($cls->fqcn) === 'backedenum' ? 'string' : ''));
                if ($name === 'name' || $backing !== '') {
                    $t = $name === 'name' ? RustType::str() : ($backing === 'int' ? RustType::int() : RustType::str());
                    return new Val($base->code . '.' . $name . '()', $t);
                }
            }
            $getter = $cls !== null ? $this->program->findMethod($cls, '__get') : null;
            if ($getter !== null && $getter->node !== null) {
                // magic property: `__get($name)`
                return $this->narrow(new Val($base->code . '.' . $getter->rustName() . '(' . Names::strLit($name) . ')', $getter->return_type), $e);
            }
            // interface-typed receiver or undeclared property: dynamic lookup
            $code = 'mixed_prop(&' . $this->casts->convert($base->code, $bt, RustType::mixed()) . ', &' . $this->dynPropName($name) . ')';
            return $this->narrowOptional(new Val($code, RustType::option(RustType::mixed())), $e);
        }
        if ($bt->kind === RustType::UNION) {
            // property common to several classes
            $arms = [];
            $res = $this->inferredOrMixed($e);
            if ($res->kind === RustType::MIXED) {
                // Psalm cannot type the property of a union member: the members' field types do
                $ftypes = [];
                foreach ($bt->params as $m) {
                    $mc = $m->kind === RustType::CLASS_ ? $this->program->classOf($m) : null;
                    if ($mc === null) {
                        continue;
                    }
                    $mf = $mc->fields[$name] ?? null;
                    if ($mf !== null) {
                        $ftypes[] = $mf->type;
                    } elseif (($vf = $this->program->variantField($mc, $name)) !== null) {
                        $ftypes[] = $vf[1];
                    }
                }
                if ($ftypes !== []) {
                    $res = $this->program->unionOfRust($ftypes) ?? $res;
                }
            }
            foreach ($bt->params as $m) {
                if ($m->kind !== RustType::CLASS_) {
                    continue;
                }
                $cls = $this->program->classOf($m);
                $field = $cls?->fields[$name] ?? null;
                $ftype = $field?->type;
                if ($field === null && $cls !== null && ($vf = $this->program->variantField($cls, $name)) !== null) {
                    [$field, $ftype] = $vf;
                }
                if ($field === null) {
                    continue;
                }
                $arms[] = $bt->mangle() . '::' . $m->variantName() . '(__o) => ' . $this->casts->convert('__o.' . $field->acc() . '_get()', $ftype, $res);
            }
            if ($arms !== []) {
                return new Val('(match ' . $base->code . ' { ' . implode(', ', $arms) . ', _ => unreachable!() })', $res);
            }
        }
        if ($bt->kind === RustType::RT_GENERIC && $bt->name === 'StdClass') {
            return $this->narrowOptional(new Val($base->code . '.get(' . Names::strLit($name) . ')', RustType::option(RustType::mixed())), $e);
        }
        if ($bt->kind === RustType::MIXED || $bt->kind === RustType::ANY_OBJECT || $bt->kind === RustType::UNION) {
            $code = 'mixed_prop(&' . $this->casts->convert($base->code, $bt, RustType::mixed()) . ', &' . $this->dynPropName($name) . ')';
            return $this->narrowOptional(new Val($code, RustType::option(RustType::mixed())), $e);
        }
        if ($bt->kind === RustType::RT_GENERIC) {
            return $this->narrow(new Val($base->code . '.prop_' . Names::field($name) . '()', $this->inferredOrMixed($e)), $e);
        }
        $this->warn('property read on ' . $bt->toRust(), $e);
        return $this->dead('property read on ' . $bt->toRust() . '', $this->inferredOrMixed($e));
    }

    private function staticPropertyFetch(Expr\StaticPropertyFetch $e): Val
    {
        if (!$e->class instanceof Name || !$e->name instanceof Node\VarLikeIdentifier) {
            $this->warn('dynamic static property', $e);
            return $this->dead('dynamic static property', $this->inferredOrMixed($e));
        }
        $fqcn = $this->resolveClassName($e->class);
        $cls = $fqcn !== null ? $this->program->getClass($fqcn) : null;
        $field = $this->findStaticField($cls, $e->name->name);
        if ($field === null) {
            $this->warn('unknown static property ' . $e->name->name, $e);
            return $this->dead('unknown static property', $this->inferredOrMixed($e));
        }
        return $this->narrow(new Val($field->declaring->path() . '::st_' . $field->rustName() . '()', $field->type), $e);
    }

    private function classConstFetch(Expr\ClassConstFetch $e): Val
    {
        if (!$e->name instanceof Identifier) {
            $this->warn('dynamic class constant', $e);
            return $this->dead('dynamic class constant', $this->inferredOrMixed($e));
        }
        $name = $e->name->name;
        if ($e->class instanceof Name) {
            $fqcn = $this->resolveClassName($e->class);
            if ($name === 'class') {
                if (strtolower($e->class->toString()) === 'static' && $this->this_type !== null && $this->static_class === null) {
                    return new Val('Str::from_str(' . $this->this_expr . '.class_name())', RustType::str());
                }
                return new Val(Names::strLit((string) $fqcn), RustType::str());
            }
            $cls = $fqcn !== null ? $this->program->getClass($fqcn) : null;
            if ($cls === null) {
                $this->warn('constant on unknown class ' . $fqcn, $e);
                return $this->dead('unknown class constant', $this->inferredOrMixed($e));
            }
            if ($cls->isEnum() && isset($cls->storage->enum_cases[$name])) {
                return new Val($cls->path() . '::' . Names::typeIdent($name), RustType::class($cls->fqcn));
            }
            $const = $this->findConstant($cls, $name);
            if ($const === null && strtolower($e->class->toString()) === 'static' && $this->this_type !== null && $this->class !== null && !$this->class->isLeaf()) {
                // late static binding: the constant is declared in subclasses
                $arms = [];
                $res = null;
                foreach ($this->class->concrete as $c) {
                    $cc = $this->findConstant($c, $name);
                    if ($cc === null) {
                        $arms[] = $this->class->path() . '::' . $c->variant() . '(_) => unreachable!("undefined constant ' . $name . '")';
                        continue;
                    }
                    $res ??= $cc->type;
                    $arms[] = $this->class->path() . '::' . $c->variant() . '(_) => ' . $this->casts->convert($cc->declaring->path() . '::' . $cc->rustName() . '()', $cc->type, $res);
                }
                if ($res !== null) {
                    return $this->narrow(new Val('(match ' . $this->this_expr . ' { ' . implode(', ', $arms) . ', _ => unreachable!() })', $res), $e);
                }
            }
            if ($const === null) {
                // literal known to Psalm?
                $pt = $this->psalmType($e);
                if ($pt !== null && $pt->isSingleIntLiteral()) {
                    return new Val($pt->getSingleIntLiteral()->value . 'i64', RustType::int());
                }
                if ($pt !== null && $pt->isSingleStringLiteral()) {
                    return new Val(Names::strLit($pt->getSingleStringLiteral()->value), RustType::str());
                }
                $this->warn('unknown class constant ' . $fqcn . '::' . $name, $e);
                return $this->dead('unknown class constant ' . $name . '', $this->inferredOrMixed($e));
            }
            return $this->narrow(new Val($const->declaring->path() . '::' . $const->rustName() . '()', $const->type), $e);
        }
        // $obj::class / $obj::CONST
        $base = $this->expr($e->class);
        if ($name === 'class') {
            return new Val($this->casts->classNameOf($base->code, $base->type) ?? 'class_name_of(&' . $this->casts->convert($base->code, $base->type, RustType::mixed()) . ')', RustType::str());
        }
        $res = $this->inferredOrMixed($e);
        $bt = $base->type->kind === RustType::OPTION ? $base->type->inner() : $base->type;
        if ($bt->kind === RustType::CLASS_) {
            // `$object::CONST`: constants are not overridable per instance beyond the static class
            $cls = $this->program->classOf($bt);
            $const = $cls !== null ? $this->findConstant($cls, $name) : null;
            if ($const !== null) {
                return $this->narrow(new Val('{ let _ = ' . $base->code . '; ' . $const->declaring->path() . '::' . $const->rustName() . '() }', $const->type), $e);
            }
        }
        // `$class::CONST` with a class name: classes are never looked up by name (closed world)
        $this->warn('class constant on a class name', $e);
        return $this->dead('class constant on a class name', $res);
    }

    public function findConstant(ClassModel $cls, string $name): ?ConstModel
    {
        if (isset($cls->constants[$name])) {
            return $cls->constants[$name];
        }
        foreach ($cls->ancestors as $a) {
            if (isset($a->constants[$name])) {
                return $a->constants[$name];
            }
        }
        // inherited constants of external classes
        if (isset($cls->storage->constants[$name]) && !$cls->is_project) {
            return null;
        }
        return null;
    }

    // ------------------------------------------------------------------ assignments

    /** Statement code for `$target = $value` where `$value` is already converted to the target's type. */
    public function assignTo(Expr $target, Val $value): string
    {
        if ($target instanceof Expr\List_ || $target instanceof Expr\Array_) {
            return $this->destructure($target, $value);
        }
        $place = $this->place($target);
        return $place->write($this->casts->convert($value->code, $value->type, $place->type));
    }

    /**
     * `$x = &<target>`: rebinds the reference variable `$x` onto the target, or falls back to a copy
     * when the target can't be referenced.
     */
    public function assignRefStmt(Expr\AssignRef $e): string
    {
        $var = $e->var;
        if ($var instanceof Expr\Variable && is_string($var->name) && !empty($this->refvars[$var->name])) {
            $rn = Names::var($var->name);
            $rt = $this->varType($var->name);
            $src = $e->expr;
            if ($src instanceof Expr\Variable && is_string($src->name) && $src->name !== 'this') {
                $st = $this->varType($src->name);
                $sn = Names::var($src->name);
                if (!empty($this->refvars[$src->name]) && $st->toRust() === $rt->toRust()) {
                    return $rn . ' = ' . $sn . '.clone();';
                }
                if (!empty($this->cells[$src->name])) {
                    if ($st->toRust() === $rt->toRust()) {
                        return $rn . ' = PhpRef::from_cell(' . $sn . '.clone());';
                    }
                    return $rn . ' = { let __c1 = ' . $sn . '.clone(); let __c2 = ' . $sn . '.clone(); PhpRef::new(move || '
                        . $this->casts->convert('__c1.borrow().get().clone()', $st, $rt) . ', move |__v| __c2.borrow_mut().set('
                        . $this->casts->convert('__v', $rt, $st) . ')) };';
                }
            }
            if ($src instanceof Expr\PropertyFetch && $src->name instanceof Identifier) {
                $base = $this->receiver($src->var);
                $bt = $base->type->kind === RustType::OPTION ? $base->type->inner() : $base->type;
                $cls = $bt->kind === RustType::CLASS_ ? $this->program->classOf($bt) : null;
                $field = $cls?->fields[$src->name->name] ?? null;
                if ($field !== null) {
                    $acc = $field->acc();
                    $bc = $base->type->kind === RustType::OPTION ? $base->code . '.unwrap()' : $base->code;
                    return $rn . ' = { let __o = ' . $bc . '; let __o2 = __o.clone(); PhpRef::new(move || '
                        . $this->casts->convert('__o.' . $acc . '_get()', $field->type, $rt) . ', move |__v| __o2.set_' . $acc . '('
                        . $this->casts->convert('__v', $rt, $field->type) . ')) };';
                }
            }
            if ($src instanceof Expr\PropertyFetch && !$src->name instanceof Identifier) {
                $this->warn('reference to a dynamically named property', $e);
                return $rn . ' = PhpRef::of(' . $this->deadCode('dynamic property name', $rt) . ');';
            }
            $this->warn('reference to an unsupported target', $e);
            return $rn . ' = PhpRef::of(' . $this->exprTo($src, $rt) . ');';
        }
        if ($var instanceof Expr\ArrayDimFetch && $e->expr instanceof Expr\ArrayDimFetch && $var->dim !== null && $e->expr->dim !== null) {
            // `$a[$x] = &$a[$y]`: both keys share one slot of the same array (see php_rt::Map::alias)
            $printer = new \PhpParser\PrettyPrinter\Standard();
            if ($printer->prettyPrintExpr($var->var) === $printer->prettyPrintExpr($e->expr->var)) {
                $base = $this->place($var->var);
                $bt = $base->type->kind === RustType::OPTION ? $base->type->inner() : $base->type;
                if ($bt->kind === RustType::MAP) {
                    $kt = $bt->params[0];
                    $k1 = $this->keyExpr($var->dim, $kt);
                    $k2 = $this->keyExpr($e->expr->dim, $kt);
                    return $this->hoisted([$k1, $k2], fn(string $a, string $b) => $base->modify(fn(string $p) => $p . '.alias(' . $a . ', ' . $b . ');'));
                }
            }
        }
        $this->warn('assign by reference', $e);
        return $this->assignStmt(new Expr\Assign($e->var, $e->expr, $e->getAttributes()));
    }

    public function assignStmt(Expr\Assign $e): string
    {
        $target = $e->var;
        if ($target instanceof Expr\List_ || $target instanceof Expr\Array_) {
            return $this->destructure($target, $this->expr($e->expr));
        }
        $place = $this->place($target);
        $value = $this->exprTo($e->expr, $place->type);
        return $place->write($value);
    }

    private function assignExpr(Expr\Assign $e): Val
    {
        $target = $e->var;
        if ($target instanceof Expr\List_ || $target instanceof Expr\Array_) {
            $v = $this->expr($e->expr);
            $tmp = $this->tmp();
            return new Val('{ let ' . $tmp . ' = ' . $v->code . '; ' . $this->destructure($target, new Val($tmp . '.clone()', $v->type)) . ' ' . $tmp . ' }', $v->type);
        }
        $place = $this->place($target);
        $rhs = $this->expr($e->expr);
        if ($rhs->type->kind === RustType::OPTION && $place->type->kind !== RustType::OPTION && $place->type->kind !== RustType::MIXED
            && $rhs->type->inner()->toRust() === $place->type->toRust()
        ) {
            // `if ($x = maybe())`: the target (never null by Psalm's typing) is only set when there is a
            // value; the expression keeps the optional value for the enclosing condition
            $tmp = $this->tmp();
            return new Val('{ match ' . $rhs->code . ' { Some(' . $tmp . ') => { ' . $place->write($tmp . '.clone()') . ' Some(' . $tmp . ') } None => None } }', $rhs->type);
        }
        $value = $this->casts->convert($rhs->code, $rhs->type, $place->type);
        $tmp = $this->tmp();
        return new Val('{ let ' . $tmp . ' = ' . $value . '; ' . $place->write($tmp . '.clone()') . ' ' . $tmp . ' }', $place->type);
    }

    /** `[$a, $b] = $value` */
    private function destructure(Expr\List_|Expr\Array_ $target, Val $value): string
    {
        $vt = $value->type;
        $tmp = $this->tmp('__d');
        $optional = false;
        if ($vt->kind === RustType::OPTION) {
            $optional = true;
            $vt = $vt->inner();
            $code = 'if let Some(' . $tmp . ') = ' . $value->code . ' { ';
        } else {
            $code = 'let ' . $tmp . ' = ' . $value->code . '; ';
        }
        $i = 0;
        foreach ($target->items as $item) {
            if ($item === null) {
                $i++;
                continue;
            }
            $key = $item->key !== null ? $this->literalKey($item->key) : (string) $i;
            $i++;
            $elem = null;
            if ($vt->kind === RustType::TUPLE) {
                $idx = (int) $key;
                if (isset($vt->params[$idx])) {
                    $elem = new Val($tmp . '.' . $idx . '.clone()', $vt->params[$idx]);
                }
            } elseif ($vt->kind === RustType::SHAPE && $key !== null && isset($vt->fields[$key])) {
                [$ft, $opt] = $vt->fields[$key];
                $elem = new Val($opt ? $this->casts->convert($tmp . '.' . Names::field($key) . '.clone()', RustType::shapeField($ft, true), $ft) : $tmp . '.' . Names::field($key) . '.clone()', $ft);
            } elseif ($vt->kind === RustType::LIST) {
                $elem = new Val($tmp . '.idx(' . (int) $key . ').clone()', $vt->inner());
            } elseif ($vt->kind === RustType::MAP) {
                [$kt, $vtt] = $vt->params;
                $kcode = $item->key !== null ? $this->keyExpr($item->key, $kt) : $this->keyFrom(new Val($key . 'i64', RustType::int()), $kt);
                $elem = new Val($tmp . '.idx(&' . $kcode . ').clone()', $vtt);
            } elseif ($vt->kind === RustType::UNION && ($ui = $this->unionIndex($tmp . '.clone()', $vt, $item->key !== null ? $this->keyExpr($item->key, RustType::arrayKey()) : 'ArrayKey::Int(' . (int) $key . ')')) !== null) {
                // a union holding an array (the parser's semantic values): the array member is read, typed
                [$ucode, $et] = $ui;
                $elem = new Val($et->hasDefault() ? '(match ' . $ucode . ' { Some(__v) => __v, None => Default::default() })' : $ucode . '.expect("destructuring: missing element")', $et);
            } elseif ($vt->kind === RustType::MIXED || $vt->kind === RustType::UNION) {
                // a union that may hold an array (e.g. the parser's semantic values): read through Mixed
                $kcode = $item->key !== null ? $this->keyExpr($item->key, RustType::arrayKey()) : 'ArrayKey::Int(' . (int) $key . ')';
                $base = $vt->kind === RustType::MIXED ? $tmp : $this->casts->convert($tmp . '.clone()', $vt, RustType::mixed());
                $elem = new Val('mixed_get(&' . $base . ', &' . $kcode . ').unwrap_or_default()', RustType::mixed());
            }
            if ($elem === null) {
                $this->warn('destructuring of ' . $vt->toRust(), $target);
                continue;
            }
            $code .= $this->assignTo($item->value, $elem) . ' ';
        }
        return $optional ? $code . '}' : $code;
    }

    private function assignOpExpr(Expr\AssignOp $e): Val
    {
        $stmt = $this->assignOpStmt($e);
        $place = $this->place($e->var);
        return new Val('{ ' . $stmt . ' ' . $place->read() . ' }', $place->type);
    }

    public function assignOpStmt(Expr\AssignOp $e): string
    {
        $place = $this->place($e->var);
        $t = $place->type;
        if ($e instanceof Expr\AssignOp\Coalesce) {
            $ov = $this->optionalValue($e->var);
            $inner = $t->kind === RustType::OPTION ? $t->inner() : $t;
            $rhs = $this->exprTo($e->expr, $inner);
            $store = $t->kind === RustType::OPTION ? 'Some(' . $rhs . ')' : $rhs;
            if ($ov !== null) {
                return 'if ' . $ov->code . '.is_none() { ' . $place->write($store) . ' }';
            }
            if ($t->kind === RustType::MIXED) {
                return 'if ' . $place->read() . '.is_null() { ' . $place->write($rhs) . ' }';
            }
            return '';
        }
        if ($e instanceof Expr\AssignOp\Concat) {
            $rhs = $this->exprTo($e->expr, RustType::str());
            if ($t->kind === RustType::STR && $place->hasMut()) {
                // the right side may read the same object: evaluate it before borrowing the place
                $tmp = $this->tmp();
                return '{ let ' . $tmp . ' = ' . $rhs . '; ' . $place->wrap('append(&mut ' . $place->mut() . ', ' . $tmp . ');') . ' }';
            }
            if ($t->kind === RustType::OPTION && $t->inner()->kind === RustType::STR) {
                $tmp = $this->tmp();
                return '{ let ' . $tmp . ' = concat(' . $place->read() . '.unwrap_or_default(), ' . $rhs . '); ' . $place->write('Some(' . $tmp . ')') . ' }';
            }
            $tmp = $this->tmp();
            return '{ let ' . $tmp . ' = concat(' . $this->casts->convert($place->read(), $t, RustType::str()) . ', ' . $rhs . '); ' . $place->write($this->casts->convert($tmp, RustType::str(), $t)) . ' }';
        }
        $rhs_t = $this->inferredOrMixed($e->expr);
        $res_t = $this->inferredOrMixed($e);
        $int_ok = fn(RustType $x) => $x->kind === RustType::INT || $x->kind === RustType::BOOL;
        $tmp = $this->tmp();
        if ($e instanceof Expr\AssignOp\Plus || $e instanceof Expr\AssignOp\Minus || $e instanceof Expr\AssignOp\Mul) {
            $sig = $e instanceof Expr\AssignOp\Plus ? '+' : ($e instanceof Expr\AssignOp\Minus ? '-' : '*');
            if ($sig === '+' && ($t->kind === RustType::MAP || $t->kind === RustType::LIST)) {
                $mt = $t->kind === RustType::MAP ? $t : RustType::map(RustType::int(), $t->inner());
                $rhs = $this->exprTo($e->expr, $mt);
                return '{ let ' . $tmp . ' = array_union(&' . $this->casts->convert($place->read(), $t, $mt) . ', &' . $rhs . '); ' . $place->write($this->casts->convert($tmp, $mt, $t)) . ' }';
            }
            if ($int_ok($t) && $int_ok($rhs_t) && $res_t->kind !== RustType::FLOAT) {
                $fn = match ($sig) {
                    '+' => 'wrapping_add',
                    '-' => 'wrapping_sub',
                    default => 'wrapping_mul',
                };
                $rhs = $this->exprTo($e->expr, RustType::int());
                return '{ let ' . $tmp . ' = ' . $rhs . '; ' . $place->write('(' . $place->read() . ').' . $fn . '(' . $tmp . ')') . ' }';
            }
            if ($t->kind === RustType::FLOAT || $t->kind === RustType::INT) {
                $rhs = $this->exprTo($e->expr, RustType::float());
                return '{ let ' . $tmp . ' = ' . $rhs . '; ' . $place->write($this->casts->convert('(' . $this->casts->convert($place->read(), $t, RustType::float()) . ' ' . $sig . ' ' . $tmp . ')', RustType::float(), $t)) . ' }';
            }
            $fn = match ($sig) {
                '+' => 'num_add',
                '-' => 'num_sub',
                default => 'num_mul',
            };
            $rhs = $this->numOperand($e->expr);
            $cur = 'to_num(&' . $this->casts->convert($place->read(), $t, RustType::mixed()) . ')';
            return '{ let ' . $tmp . ' = ' . $fn . '(' . $cur . ', ' . $rhs . '); ' . $place->write($this->casts->convert($tmp . '.to_mixed()', RustType::mixed(), $t)) . ' }';
        }
        if ($e instanceof Expr\AssignOp\Div) {
            $rhs = $this->exprTo($e->expr, RustType::float());
            return '{ let ' . $tmp . ' = div_f(' . $this->casts->convert($place->read(), $t, RustType::float()) . ', ' . $rhs . '); ' . $place->write($this->casts->convert($tmp, RustType::float(), $t)) . ' }';
        }
        if ($e instanceof Expr\AssignOp\Mod) {
            $rhs = $this->exprTo($e->expr, RustType::int());
            return '{ let ' . $tmp . ' = imod(' . $this->casts->convert($place->read(), $t, RustType::int()) . ', ' . $rhs . '); ' . $place->write($this->casts->convert($tmp, RustType::int(), $t)) . ' }';
        }
        if ($e instanceof Expr\AssignOp\Pow) {
            $rhs = $this->exprTo($e->expr, RustType::float());
            return '{ let ' . $tmp . ' = pow_f(' . $this->casts->convert($place->read(), $t, RustType::float()) . ', ' . $rhs . '); ' . $place->write($this->casts->convert($tmp, RustType::float(), $t)) . ' }';
        }
        $op = match (true) {
            $e instanceof Expr\AssignOp\BitwiseAnd => '&',
            $e instanceof Expr\AssignOp\BitwiseOr => '|',
            $e instanceof Expr\AssignOp\BitwiseXor => '^',
            $e instanceof Expr\AssignOp\ShiftLeft => '<<',
            $e instanceof Expr\AssignOp\ShiftRight => '>>',
            default => null,
        };
        if ($op !== null) {
            $rhs = $this->exprTo($e->expr, RustType::int());
            $cur = $this->casts->convert($place->read(), $t, RustType::int());
            $expr = $op === '<<' || $op === '>>'
                ? '(' . $cur . ').wrapping_' . ($op === '<<' ? 'shl' : 'shr') . '((' . $tmp . ') as u32)'
                : '(' . $cur . ' ' . $op . ' ' . $tmp . ')';
            return '{ let ' . $tmp . ' = ' . $rhs . '; ' . $place->write($this->casts->convert($expr, RustType::int(), $t)) . ' }';
        }
        $this->warn('unsupported assign op', $e);
        return '';
    }
}
