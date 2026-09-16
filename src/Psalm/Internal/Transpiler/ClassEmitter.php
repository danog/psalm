<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Stmt\ClassMethod;

use function count;
use function implode;
use function in_array;
use function strtolower;

/**
 * Emits the Rust items for one class: data struct, handle type, accessors, methods, statics, constants.
 *
 * @internal
 */
final class ClassEmitter
{
    public function __construct(
        private readonly Program $program,
        private readonly Casts $casts,
        private readonly Builtins $builtins,
        private readonly Diagnostics $diag,
    ) {
    }

    public function emit(ClassModel $cls, Writer $w): void
    {
        $this->program->types->context = '<class-level> ' . $cls->fqcn;
        if ($cls->isTrait()) {
            return;
        }
        if ($cls->isEnum()) {
            $this->emitEnum($cls, $w);
            return;
        }
        $handle = $cls->handle();
        $own = $cls->ownHandle();
        $obj = $cls->objStruct();
        $leaf = $cls->isLeaf();
        $concrete = $cls->isConcrete();

        // ---- data struct
        if ($concrete) {
            $w->line('pub struct ' . $obj . ' {');
            $w->indent();
            foreach ($cls->fields as $f) {
                $st = $f->storageType();
                if (($ck = $this->cellKind($cls, $f)) !== '') {
                    $st = ($ck === 'Cell' ? 'std::cell::Cell' : 'RefCell') . '<' . $st . '>';
                }
                $w->line('pub ' . $f->rustName() . ': ' . $st . ',');
            }
            $w->close();
            $w->line('#[derive(Clone)]');
            // @psalm-immutable pilot classes drop the RefCell: reads are direct, writes copy-on-write via make_mut
            $w->line('pub struct ' . $own . '(pub ' . ($cls->immutable() ? 'Rc<' . $obj . '>' : 'Rc<RefCell<' . $obj . '>>') . ');');
        }

        // ---- dispatch enum for non-leaf classes and interfaces
        if (!$leaf) {
            $w->line('#[derive(Clone)]');
            $w->open('pub enum ' . $handle . ' {');
            foreach ($cls->concrete as $c) {
                $w->line($c->variant() . '(' . $c->ownPath() . '),');
            }
            if ($cls->has_downstream) {
                // escape hatch for downstream-crate subclasses the enum can't name (dynamic dispatch)
                $w->line('Other__(Mixed),');
            }
            $w->close();
            if ($cls->has_downstream) {
                $this->emitDynTrait($cls, $w);
            }
            $this->emitEnumHandleImpls($cls, $w);
        }

        // ---- accessors and methods on the own handle
        if ($concrete) {
            $w->open('impl ' . $own . ' {');
            foreach ($cls->fields as $f) {
                $this->emitAccessors($f, $w, $cls->immutable(), $this->cellKind($cls, $f));
            }
            $this->emitConstructor($cls, $w);
            foreach ($cls->methods as $m) {
                $this->emitMethodOnOwn($cls, $m, $w);
            }
            $w->line('pub fn new_same_class' . $this->ctorSig($cls, true) . ' -> ' . $cls->path() . ' { ' . $this->wrapOwn($cls, 'Self::new(' . $this->ctorArgs($cls) . ')') . ' }');
            $w->close();
            $this->emitOwnCloneImpls($cls, $w);
        }

        // ---- statics, constants and bodies live on the handle type
        $w->open('impl ' . $handle . ' {');
        foreach ($cls->static_fields as $f) {
            $this->emitStatic($cls, $f, $w);
        }
        foreach ($cls->constants as $c) {
            $this->emitConstant($cls, $c, $w);
        }
        if (!$leaf) {
            foreach ($cls->fields as $f) {
                $this->emitEnumAccessors($cls, $f, $w);
            }
            foreach ($cls->methods as $m) {
                $this->emitMethodOnEnum($cls, $m, $w);
            }
            if ($concrete) {
                $w->line('pub fn new' . $this->ctorSig($cls) . ' -> ' . $handle . ' { ' . $handle . '::' . $cls->variant() . '(' . $own . '::new(' . $this->ctorArgs($cls) . ')) }');
            }
            $w->line('pub fn new_same_class' . $this->ctorSig($cls, true) . ' -> ' . $handle . ' { match self { ' . implode(', ', array_map(fn(ClassModel $c) => $handle . '::' . $c->variant() . '(__h) => ' . $this->casts->convert('__h.new_same_class(' . $this->ctorArgsFor($cls, $c) . ')', RustType::class($c->fqcn), RustType::class($cls->fqcn)), $cls->concrete)) . ($cls->concrete ? ', ' : '') . '_ => unreachable!() } }');
        }
        $w->close();
    }

    private function ancestorIdsLiteral(ClassModel $cls): string
    {
        $ids = [$this->program->classId($cls)];
        foreach ($cls->ancestors as $a) {
            $ids[] = $this->program->classId($a);
        }
        return '&[' . implode(', ', $ids) . ']';
    }

    private function ancestorsLiteral(ClassModel $cls): string
    {
        $names = [strtolower($cls->fqcn)];
        foreach ($cls->ancestors as $a) {
            $names[] = strtolower($a->fqcn);
        }
        return '&[' . implode(', ', array_map(fn($n) => Names::rustStringLiteral($n), $names)) . ']';
    }

    private function wrapOwn(ClassModel $cls, string $code): string
    {
        if ($cls->isLeaf()) {
            return $code;
        }
        return $cls->handle() . '::' . $cls->variant() . '(' . $code . ')';
    }

    // ------------------------------------------------------------------ accessors

    /** 'Cell'/'RefCell'/'' — a field of an immutable class that is mutated through &self needs per-field interior mut. */
    private function cellKind(ClassModel $cls, FieldModel $f): string
    {
        if (!$cls->immutable() || !isset($cls->interiorMutFields()[$f->name])) {
            return '';
        }
        // A Late field (no default -> deferred init) can't be a Cell: Cell<T> requires T: Copy and Late<T> never is.
        // Use RefCell for Late (and for any non-Copy field); Cell only for a plain Copy field.
        return (!$f->isLate() && $f->type->isCopy()) ? 'Cell' : 'RefCell';
    }

    private function emitAccessors(FieldModel $f, Writer $w, bool $immut = false, string $cell = ''): void
    {
        $fld = $f->rustName();
        $rn = $f->acc();
        $t = $f->type->toRust();
        if ($cell === 'Cell') {
            // Copy field with per-field interior mutability: get/set through &self, no borrow guard.
            $w->line('pub fn ' . $rn . '(&self) -> ' . $t . ' { self.0.' . $fld . '.get() }');
            $w->line('pub fn ' . $rn . '_get(&self) -> ' . $t . ' { self.0.' . $fld . '.get() }');
            $w->line('pub fn ' . $rn . '_opt(&self) -> Option<' . $t . '> { Some(self.0.' . $fld . '.get()) }');
            $w->line('pub fn set_' . $rn . '(&self, v: ' . $t . ') { self.0.' . $fld . '.set(v); }');
            return;
        }
        if ($cell === 'RefCell') {
            if ($f->isLate()) {
                // memoized deferred-init field: RefCell<Late<T>>. Borrow through &self, then through Late.
                $w->line('pub fn ' . $rn . '(&self) -> Ref<\'_, ' . $t . '> { Ref::map(self.0.' . $fld . '.borrow(), |__l| __l.get()) }');
                $w->line('pub fn ' . $rn . '_get(&self) -> ' . $t . ' { self.0.' . $fld . '.borrow().get().clone() }');
                $w->line('pub fn ' . $rn . '_opt(&self) -> Option<' . $t . '> { self.0.' . $fld . '.borrow().as_option().cloned() }');
                $w->line('pub fn ' . $rn . '_mut(&self) -> RefMut<\'_, ' . $t . '> { RefMut::map(self.0.' . $fld . '.borrow_mut(), |__l| __l.' . ($f->type->hasDefault() ? 'get_or_default_mut()' : 'get_mut()') . ') }');
                $w->line('pub fn set_' . $rn . '(&self, v: ' . $t . ') { self.0.' . $fld . '.borrow_mut().set(v); }');
                return;
            }
            // non-Copy field with per-field interior mutability (e.g. memoized Option<Str>): borrow through &self.
            $w->line('pub fn ' . $rn . '(&self) -> Ref<\'_, ' . $t . '> { self.0.' . $fld . '.borrow() }');
            $w->line('pub fn ' . $rn . '_get(&self) -> ' . $t . ' { self.0.' . $fld . '.borrow().clone() }');
            $w->line('pub fn ' . $rn . '_opt(&self) -> Option<' . $t . '> { Some(self.0.' . $fld . '.borrow().clone()) }');
            $w->line('pub fn ' . $rn . '_mut(&self) -> RefMut<\'_, ' . $t . '> { self.0.' . $fld . '.borrow_mut() }');
            $w->line('pub fn set_' . $rn . '(&self, v: ' . $t . ') { *self.0.' . $fld . '.borrow_mut() = v; }');
            return;
        }
        if ($immut) {
            // Rc<T> (no RefCell): reads are direct borrows; writes (construction / wither clones only) copy-on-write
            if ($f->isLate()) {
                $w->line('pub fn ' . $rn . '(&self) -> &' . $t . ' { self.0.' . $fld . '.get() }');
                $w->line('pub fn ' . $rn . '_get(&self) -> ' . $t . ' { self.0.' . $fld . '.get().clone() }');
                $w->line('pub fn ' . $rn . '_opt(&self) -> Option<' . $t . '> { self.0.' . $fld . '.as_option().cloned() }');
                $w->line('pub fn ' . $rn . '_mut(&mut self) -> &mut ' . $t . ' { Rc::make_mut(&mut self.0).' . $fld . ($f->type->hasDefault() ? '.get_or_default_mut()' : '.get_mut()') . ' }');
                $w->line('pub fn set_' . $rn . '(&mut self, v: ' . $t . ') { Rc::make_mut(&mut self.0).' . $fld . '.set(v); }');
            } else {
                $w->line('pub fn ' . $rn . '(&self) -> &' . $t . ' { &self.0.' . $fld . ' }');
                $w->line('pub fn ' . $rn . '_get(&self) -> ' . $t . ' { self.0.' . $fld . '.clone() }');
                $w->line('pub fn ' . $rn . '_opt(&self) -> Option<' . $t . '> { Some(self.0.' . $fld . '.clone()) }');
                $w->line('pub fn ' . $rn . '_mut(&mut self) -> &mut ' . $t . ' { &mut Rc::make_mut(&mut self.0).' . $fld . ' }');
                $w->line('pub fn set_' . $rn . '(&mut self, v: ' . $t . ') { Rc::make_mut(&mut self.0).' . $fld . ' = v; }');
            }
            return;
        }
        if ($f->isLate()) {
            $w->line('pub fn ' . $rn . '(&self) -> Ref<\'_, ' . $t . '> { Ref::map(self.0.borrow(), |o| o.' . $fld . '.get()) }');
            $w->line('pub fn ' . $rn . '_get(&self) -> ' . $t . ' { self.0.borrow().' . $fld . '.get().clone() }');
            $w->line('pub fn ' . $rn . '_opt(&self) -> Option<' . $t . '> { self.0.borrow().' . $fld . '.as_option().cloned() }');
            $w->line('pub fn ' . $rn . '_mut(&self) -> RefMut<\'_, ' . $t . '> { RefMut::map(self.0.borrow_mut(), |o| o.' . $fld . ($f->type->hasDefault() ? '.get_or_default_mut()' : '.get_mut()') . ') }');
            $w->line('pub fn set_' . $rn . '(&self, v: ' . $t . ') { self.0.borrow_mut().' . $fld . '.set(v); }');
        } else {
            $w->line('pub fn ' . $rn . '(&self) -> Ref<\'_, ' . $t . '> { Ref::map(self.0.borrow(), |o| &o.' . $fld . ') }');
            $w->line('pub fn ' . $rn . '_get(&self) -> ' . $t . ' { self.0.borrow().' . $fld . '.clone() }');
            $w->line('pub fn ' . $rn . '_opt(&self) -> Option<' . $t . '> { Some(self.0.borrow().' . $fld . '.clone()) }');
            $w->line('pub fn ' . $rn . '_mut(&self) -> RefMut<\'_, ' . $t . '> { RefMut::map(self.0.borrow_mut(), |o| &mut o.' . $fld . ') }');
            $w->line('pub fn set_' . $rn . '(&self, v: ' . $t . ') { self.0.borrow_mut().' . $fld . ' = v; }');
        }
    }

    /**
     * Typed dynamic-dispatch trait for a non-leaf hierarchy. Once wired, the escape variant will hold an
     * `Rc<dyn {H}__Dyn>` instead of `Mixed`, so cross-crate / docblock-lie instances dispatch with STATIC
     * types through a vtable — no name-matched, `Mixed`-boxed `PhpObject` protocol. `PhpObject` supertrait
     * keeps it interoperable with the (un-migrated) `Mixed` world during the migration. Emitted additively
     * for now: a declaration only, so the tree stays green until the `Other__` variant is switched over.
     */
    private function emitDynTrait(ClassModel $cls, Writer $w): void
    {
        $seen = [];
        $w->open('pub trait ' . $cls->handle() . '__Dyn: php_rt::PhpObject {');
        foreach ($cls->methods as $m) {
            if ($m->isStatic()) {
                continue;
            }
            $rn = $m->rustName();
            if (isset($seen[$rn])) {
                continue;
            }
            $seen[$rn] = true;
            // Trait method declarations have no body, so parameter patterns (incl. `mut`) are not allowed.
            $params = [];
            foreach ($m->storage->params as $i => $p) {
                $t = $m->param_types[$i] ?? RustType::mixed();
                $params[] = Names::var($p->name) . ': ' . ($p->by_ref ? '&mut ' : '') . $t->toRust();
            }
            $self = '&self' . ($params ? ', ' : '');
            $w->line('fn ' . $rn . '(' . $self . implode(', ', $params) . ') -> ' . $this->retType($m) . ';');
        }
        $w->close();
    }

    private function emitEnumAccessors(ClassModel $cls, FieldModel $f, Writer $w): void
    {
        $this->program->types->context = '<enum accessors> ' . $cls->fqcn . '::$' . $f->name;
        // the dynamic (escape-variant) arms below are built unconditionally but only emitted for open
        // hierarchies: their conversions must not count as erasures otherwise
        $this->casts->record_erasures = $cls->has_downstream;
        try {
            $this->emitEnumAccessorsInner($cls, $f, $w);
        } finally {
            $this->casts->record_erasures = true;
        }
    }

    private function emitEnumAccessorsInner(ClassModel $cls, FieldModel $f, Writer $w): void
    {
        $rn = $f->acc();
        $t = $f->type->toRust();
        $h = $cls->handle();
        $name = Names::rustStringLiteral($f->name);
        // instances of subclasses from other crates are reached through the escape variant: their
        // properties are read and written dynamically
        $dyn_get = $this->casts->convert('php_rt::dyn_prop(__m, ' . $name . ')', RustType::mixed(), $f->type);
        $arms = fn(string $call, string $other) => implode(', ', array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(__h) => ' . $call, $cls->concrete)) . ($cls->concrete ? ', ' : '') . ($cls->has_downstream ? $h . '::Other__(__m) => ' . $other . ', ' : '') . '_ => unreachable!()';
        if ($cls->allConcreteImmutable()) {
            // Immutable-leaf hierarchy (Rc<T>, no RefCell): leaf getters return `&T` (not a Ref guard), so the
            // enum getter must hand back an OWNED copy (PropRef::Owned) — PropRef::Borrowed holds a Ref<'_,T> only.
            // The mutator accessors (rn_mut/set_rn) are OMITTED: an @psalm-immutable base is never mutated through
            // the handle (verified by Program::computeExternalWrites — else the hierarchy would not have converted),
            // and the leaf set_/mut are `&mut self` which cannot be called on the `&LeafHandle` a `&self` match binds.
            $w->line('pub fn ' . $rn . '(&self) -> PropRef<\'_, ' . $t . '> { match self { ' . $arms('PropRef::Owned(__h.' . $rn . '_get())', 'PropRef::Owned(' . $dyn_get . ')') . ' } }');
            $w->line('pub fn ' . $rn . '_get(&self) -> ' . $t . ' { match self { ' . $arms('__h.' . $rn . '_get()', $dyn_get) . ' } }');
            $w->line('pub fn ' . $rn . '_opt(&self) -> Option<' . $t . '> { match self { ' . $arms('__h.' . $rn . '_opt()', 'php_rt::other_obj(__m).get_prop(' . $name . ').map(|__v| ' . $this->casts->convert('__v', RustType::mixed(), $f->type) . ')') . ' } }');
            // INTERIOR-MUT (Cell/RefCell) fields keep their mutators even on an immutable enum: the leaf set_/mut are
            // `&self` (Cell.set / RefCell.borrow_mut), dispatchable on the `&LeafHandle` a `&self` match binds — no COW.
            // Plain immutable fields' set_/mut are `&mut self` (make_mut) and stay omitted.
            $interior = false;
            $interior_refcell = false;
            foreach ($cls->concrete as $c) {
                if (isset($c->interiorMutFields()[$f->name])) {
                    $interior = true;
                    if ($this->cellKind($c, $f) === 'RefCell') {
                        $interior_refcell = true;
                    }
                }
            }
            if ($interior) {
                $w->line('pub fn set_' . $rn . '(&self, v: ' . $t . ') { match self { ' . $arms('__h.set_' . $rn . '(v)', '{ php_rt::other_obj(__m).set_prop(' . $name . ', ' . $this->casts->convert('v', $f->type, RustType::mixed()) . '); }') . ' } }');
                if ($interior_refcell) {
                    $w->line('pub fn ' . $rn . '_mut(&self) -> PropMut<\'_, ' . $t . '> { match self { ' . $arms('PropMut::Borrowed(__h.' . $rn . '_mut())', '{ let __o = php_rt::other_obj(__m); PropMut::owned(' . $dyn_get . ', Box::new(move |__v: ' . $t . '| { __o.set_prop(' . $name . ', ' . $this->casts->convert('__v', $f->type, RustType::mixed()) . '); })) }') . ' } }');
                }
            } else {
                // Plain immutable field: an `&mut self` enum set_ dispatching to each leaf's `&mut self` make_mut setter
                // (`&mut self` match binds __h as `&mut LeafHandle`). Usable only on a mut/owned enum — construction and
                // wither clone-locals (`$c = clone $this; $c->field = v`), never a shared &self handle (which has no
                // external writes, or the class would not have converted). Lets concrete-base withers write plain fields.
                $w->line('pub fn set_' . $rn . '(&mut self, v: ' . $t . ') { match self { ' . $arms('__h.set_' . $rn . '(v)', '{ php_rt::other_obj(__m).set_prop(' . $name . ', ' . $this->casts->convert('v', $f->type, RustType::mixed()) . '); }') . ' } }');
            }
            return;
        }
        $w->line('pub fn ' . $rn . '(&self) -> PropRef<\'_, ' . $t . '> { match self { ' . $arms('PropRef::Borrowed(__h.' . $rn . '())', 'PropRef::Owned(' . $dyn_get . ')') . ' } }');
        $w->line('pub fn ' . $rn . '_get(&self) -> ' . $t . ' { match self { ' . $arms('__h.' . $rn . '_get()', $dyn_get) . ' } }');
        $w->line('pub fn ' . $rn . '_opt(&self) -> Option<' . $t . '> { match self { ' . $arms('__h.' . $rn . '_opt()', 'php_rt::other_obj(__m).get_prop(' . $name . ').map(|__v| ' . $this->casts->convert('__v', RustType::mixed(), $f->type) . ')') . ' } }');
        $w->line('pub fn ' . $rn . '_mut(&self) -> PropMut<\'_, ' . $t . '> { match self { ' . $arms('PropMut::Borrowed(__h.' . $rn . '_mut())', '{ let __o = php_rt::other_obj(__m); PropMut::owned(' . $dyn_get . ', Box::new(move |__v: ' . $t . '| { __o.set_prop(' . $name . ', ' . $this->casts->convert('__v', $f->type, RustType::mixed()) . '); })) }') . ' } }');
        $w->line('pub fn set_' . $rn . '(&self, v: ' . $t . ') { match self { ' . $arms('__h.set_' . $rn . '(v)', '{ php_rt::other_obj(__m).set_prop(' . $name . ', ' . $this->casts->convert('v', $f->type, RustType::mixed()) . '); }') . ' } }');
    }

    // ------------------------------------------------------------------ constructor

    private function ctorSig(ClassModel $cls, bool $with_self = false): string
    {
        $ctor = $this->program->findMethod($cls, '__construct');
        $params = [];
        if ($ctor !== null) {
            foreach ($ctor->storage->params as $i => $p) {
                $t = $ctor->param_types[$i] ?? RustType::mixed();
                $params[] = 'mut ' . Names::var($p->name) . ': ' . ($p->by_ref ? '&mut ' : '') . $t->toRust();
            }
        }
        return '(' . ($with_self ? '&self' . ($params ? ', ' : '') : '') . implode(', ', $params) . ')';
    }

    /** Constructor arguments of `$base`'s signature adapted to `$target`'s constructor. */
    private function ctorArgsFor(ClassModel $base, ClassModel $target): string
    {
        $bc = $this->program->findMethod($base, '__construct');
        $tc = $this->program->findMethod($target, '__construct');
        if ($tc === null) {
            return '';
        }
        $args = [];
        foreach ($tc->storage->params as $i => $p) {
            $bt = $bc !== null && isset($bc->storage->params[$i]) ? ($bc->param_types[$i] ?? RustType::mixed()) : null;
            $tt = $tc->param_types[$i] ?? RustType::mixed();
            if ($bt !== null) {
                $args[] = $p->by_ref ? Names::var($bc->storage->params[$i]->name) : $this->casts->convert(Names::var($bc->storage->params[$i]->name) . '.clone()', $bt, $tt);
            } else {
                $args[] = $this->casts->defaultOf($tt);
            }
        }
        return implode(', ', $args);
    }

    private function ctorArgs(ClassModel $cls): string
    {
        $ctor = $this->program->findMethod($cls, '__construct');
        $args = [];
        if ($ctor !== null) {
            foreach ($ctor->storage->params as $p) {
                $args[] = Names::var($p->name);
            }
        }
        return implode(', ', $args);
    }

    private function emitConstructor(ClassModel $cls, Writer $w): void
    {
        $this->program->types->context = '<constructor> ' . $cls->fqcn;
        $own = $cls->ownHandle();
        $obj = $cls->objStruct();
        $body = $this->constExprEmitter($cls);
        $inits = [];
        foreach ($cls->fields as $f) {
            $init = $this->fieldInit($f, $body);
            if (($ck = $this->cellKind($cls, $f)) !== '') {
                $init = ($ck === 'Cell' ? 'std::cell::Cell' : 'RefCell') . '::new(' . $init . ')';
            }
            $inits[] = $f->rustName() . ': ' . $init . ',';
        }
        // an instance with initialized defaults but without running the constructor
        // (ReflectionClass::newInstanceWithoutConstructor, unserialize)
        // immutable (Rc<T>) classes drop the RefCell; `this` is mut so the constructor can make_mut its fields
        $immut = $cls->immutable();
        $cell_open = $immut ? 'Rc::new(' : 'Rc::new(RefCell::new(';
        $cell_close_uninit = $immut ? '}))' : '})))';
        $cell_close_new = $immut ? '}));' : '})));';
        $w->open('pub fn new_uninit() -> ' . $own . ' {');
        $w->open($own . '(' . $cell_open . $obj . ' {');
        foreach ($inits as $line) {
            $w->line($line);
        }
        $w->close($cell_close_uninit);
        $w->close();
        $w->open('pub fn new' . $this->ctorSig($cls) . ' -> ' . $own . ' {');
        $w->open('let ' . ($immut ? 'mut ' : '') . 'this = ' . $own . '(' . $cell_open . $obj . ' {');
        foreach ($inits as $line) {
            $w->line($line);
        }
        $w->close($cell_close_new);
        $ctor = $this->program->findMethod($cls, '__construct');
        if ($ctor !== null && !$ctor->isAbstract()) {
            $w->line('this.' . $ctor->rustName() . '(' . $this->ctorArgs($cls) . ');');
        }
        $w->line('this');
        $w->close();
    }

    private function fieldInit(FieldModel $f, BodyEmitter $body): string
    {
        if ($f->default !== null) {
            $code = $body->constExpr($f->default, $f->type);
            return $f->isLate() ? 'Late::new(' . $code . ')' : $code;
        }
        if ($f->isLate()) {
            return 'Late::uninit()';
        }
        return 'Default::default()';
    }

    /** An emitter usable for constant expressions in the class' scope. */
    private function constExprEmitter(ClassModel $cls): BodyEmitter
    {
        $record = $this->dummyRecord($cls);
        return new BodyEmitter($this->program, $record, $cls, $this->casts, $this->builtins, $this->diag, null);
    }

    private function dummyRecord(ClassModel $cls): FunctionRecord
    {
        $node = new ClassMethod('__dummy');
        $storage = new \Psalm\Storage\MethodStorage();
        $file = $cls->node !== null ? ($this->program->transpiler->classes[$cls->lc()]->file_path ?? '') : '';
        return new FunctionRecord($node, $storage, new \Psalm\Internal\Provider\NodeDataProvider(), $file, $cls->fqcn, null);
    }

    // ------------------------------------------------------------------ methods

    private function signature(MethodModel $m, bool $with_self, bool $force_mut_self = false): string
    {
        $params = [];
        foreach ($m->storage->params as $i => $p) {
            $t = $m->param_types[$i] ?? RustType::mixed();
            if ($p->by_ref) {
                $params[] = 'mut ' . Names::var($p->name) . ': &mut ' . $t->toRust();
            } elseif (isset($m->borrow_params[$i])) {
                // owned/borrowed (axis 5): private-method param received as `&T` (non-escaping read-only).
                $params[] = Names::var($p->name) . ': &' . $t->toRust();
            } else {
                $params[] = 'mut ' . Names::var($p->name) . ': ' . $t->toRust();
            }
        }
        // An immutable (Rc<T>) class's construction methods (ctor + init helpers reachable from it) write $this
        // fields via make_mut on the fresh object, so they need &mut self (withers write clones, not $this, and
        // stay &self; genuine post-construction mutation goes through per-field Cell/RefCell instead).
        // $force_mut_self: an inherited construction method emitted ON A LEAF (forwarding stub / super-copy) also
        // writes $this in place and needs &mut self even though $m->declaring (the base) is not a leaf.
        $recv = ($force_mut_self || ($m->declaring->immutable() && isset($m->declaring->constructionMethods()[$m->lc()]))) ? '&mut self' : '&self';
        $self = $with_self ? $recv . ($params ? ', ' : '') : '';
        return self::genericParams($m->generics) . '(' . $self . implode(', ', $params) . ') -> ' . $this->retType($m);
    }

    /** `<G_T: php_rt::PhpValue, ...>` for a generic fn (every PHP value type implements PhpValue). */
    public static function genericParams(array $generics): string
    {
        if ($generics === []) {
            return '';
        }
        return '<' . implode(', ', array_map(static fn(string $g) => $g . ': php_rt::PhpValue', array_values($generics))) . '>';
    }

    /** Axis-8: `Result<T, Throw>` for throwing methods (default), bare `T` for provably non-throwing ones. */
    private function retType(MethodModel $m): string
    {
        return $m->throws ? 'Result<' . $m->return_type->toRust() . ', Throw>' : $m->return_type->toRust();
    }

    private function argNames(MethodModel $m): string
    {
        $names = [];
        foreach ($m->storage->params as $p) {
            $names[] = Names::var($p->name);
        }
        return implode(', ', $names);
    }

    private function emitMethodOnOwn(ClassModel $cls, MethodModel $m, Writer $w): void
    {
        $rn = $m->rustName();
        $sig = $this->signature($m, !$m->isStatic());
        $declaring = $m->declaring;
        if ($m->isStatic()) {
            // static methods live on the handle type; leaves are their own handle
            if (!$cls->isLeaf()) {
                return;
            }
            if ($declaring === $cls) {
                if ($m->isAbstract()) {
                    return;
                }
                $w->line('pub fn ' . $rn . $sig . ' {');
                $w->raw($this->body($cls, $m));
                $w->line('}');
            } elseif (!$m->isAbstract() && $m->uses_lsb) {
                // late static binding: a copy of the body with `static` bound to this class
                $w->line('pub fn ' . $rn . $sig . ' {');
                $w->raw($this->body($declaring, $m, $cls));
                $w->line('}');
            } elseif (!$m->isAbstract()) {
                $w->line('pub fn ' . $rn . $sig . ' { ' . $declaring->path() . '::' . $rn . '(' . $this->argNames($m) . ') }');
            }
            return;
        }
        if ($declaring === $cls) {
            if ($m->isAbstract()) {
                return;
            }
            if ($cls->isLeaf() || $cls->isImmutableCtorMethod($m->lc())) {
                // Leaf: body lives here. Immutable concrete-base construction method: run the body IN PLACE on this
                // own-handle (&mut self, make_mut/Cell, refcount 1 during construction) instead of forwarding to the
                // enum __impl on a clone (which COWs and is omitted for immutable-leaf enums).
                $w->line('pub fn ' . $rn . ($cls->isImmutableCtorMethod($m->lc()) ? $this->signature($m, true, true) : $sig) . ' {');
                $w->raw($this->body($cls, $m));
                $w->line('}');
            } else {
                // body is on the enum as `__impl` (or as the method itself when private); the own handle forwards
                $impl = $m->isPrivate() ? $rn : $rn . '__impl';
                $w->line('pub fn ' . $rn . $sig . ' { ' . $cls->path() . '::' . $cls->variant() . '(self.clone()).' . $impl . '(' . $this->argNames($m) . ') }');
            }
            return;
        }
        // inherited: forward to the declaring ancestor's implementation
        if ($m->isAbstract() || $m->isPrivate()) {
            return;
        }
        // Immutable Rc<T> leaf inheriting a construction method: the generic forward below is
        // `cast::<Base>(self.clone()).rn__impl(args)` — but the clone COWs (writes lost) and the base enum omits
        // __impl for immutable hierarchies. Instead run a super-copy of the inherited body ON THIS leaf against
        // `self` directly (&mut self, make_mut in place, refcount 1 during construction). Nested self::/parent::
        // calls inside that body are likewise routed to leaf super-copies by CallTrait.
        if ($cls->isImmutableCtorMethod($m->lc())) {
            $copy = $this->program->requestSuperCopy($cls, $m);
            $w->line('pub fn ' . $rn . $this->signature($m, true, true) . ' { self.' . $copy . '(' . $this->argNames($m) . ') }');
            return;
        }
        $up = $this->casts->convert($this->wrapOwn($cls, 'self.clone()'), RustType::class($cls->fqcn), RustType::class($declaring->fqcn));
        $impl = $declaring->isLeaf() ? $rn : $rn . '__impl';
        $w->line('pub fn ' . $rn . $sig . ' { ' . $up . '.' . $impl . '(' . $this->argNames($m) . ') }');
    }

    private function emitMethodOnEnum(ClassModel $cls, MethodModel $m, Writer $w): void
    {
        $this->program->types->context = '<dispatch> ' . $cls->fqcn . '::' . $m->name;
        $rn = $m->rustName();
        $h = $cls->handle();
        if ($cls->allConcreteImmutable() && !$m->isStatic() && $m->lc() === '__construct') {
            // Immutable leaves emit magic__construct as `&mut self` (make_mut on the fresh object); it cannot be
            // dispatched through the `&self` handle enum, and constructing through an existing base handle is never
            // valid PHP (you construct a concrete type, then wrap). Skip the enum dispatch arm entirely.
            return;
        }
        if ($m->isPrivate() && !$m->isStatic() && $m->declaring === $cls && !$m->isAbstract()) {
            // private methods are never overridden: no dispatch, the body is the method
            $w->line('pub fn ' . $rn . $this->signature($m, true) . ' {');
            $w->raw($this->body($cls, $m));
            $w->line('}');
            return;
        }
        if ($m->isPrivate() && !$m->isStatic() && $m->declaring !== $cls) {
            return;
        }
        if ($m->isStatic()) {
            if ($m->declaring === $cls) {
                $w->line('pub fn ' . $rn . $this->signature($m, false) . ' {');
                $w->raw($this->body($cls, $m));
                $w->line('}');
            } elseif ($m->uses_lsb && !$m->isAbstract()) {
                $w->line('pub fn ' . $rn . $this->signature($m, false) . ' {');
                $w->raw($this->body($m->declaring, $m, $cls));
                $w->line('}');
            } else {
                $w->line('pub fn ' . $rn . $this->signature($m, false) . ' { ' . $m->declaring->path() . '::' . $rn . '(' . $this->argNames($m) . ') }');
            }
            if (!$m->isPrivate()) {
                // `static::m()` / `$obj->m()` on a static method: dispatched on the runtime class (plugin hook
                // handlers are called through instances, classes are never looked up by name)
                $arms = array_map(function (ClassModel $c) use ($cls, $rn, $m): string {
                    $cm = $this->program->findMethod($c, $m->lc());
                    if ($cm === null || $cm->isAbstract()) {
                        return $cls->handle() . '::' . $c->variant() . '(_) => unreachable!("abstract static method ' . $rn . '")';
                    }
                    $target = $cm->uses_lsb ? $c : $cm->declaring;
                    $call = $target->path() . '::' . $cm->rustName() . '(' . $this->convertedArgs($m, $cm) . ')';
                    return $cls->handle() . '::' . $c->variant() . '(_) => ' . $this->casts->convert($call, $cm->return_type, $m->return_type);
                }, $cls->concrete);
                if ($cls->has_downstream) {
                    $arms[] = $cls->handle() . '::Other__(__m) => ' . $this->dynamicCall($m);
                }
                $w->line('pub fn ' . $rn . '__static' . $this->signature($m, true) . ' { match self { ' . ($arms ? implode(', ', $arms) . ', ' : '') . '_ => unreachable!() } }');
            }
            return;
        }
        // virtual dispatch: an arm per concrete class that overrides the method; the others run the shared
        // body (`__impl`, present on this handle for every non-abstract method) with `self` being this enum
        $shared = $m->isAbstract() ? null : 'self.' . $rn . '__impl(' . $this->argNames($m) . ')';
        $arms = [];
        foreach ($cls->concrete as $c) {
            $cm = $this->program->findMethod($c, $m->lc());
            if ($cm === null) {
                $arms[] = $h . '::' . $c->variant() . '(_) => unreachable!("abstract method ' . $rn . '")';
                continue;
            }
            if ($shared !== null && $cm === $m) {
                continue;
            }
            $call = '__h.' . $rn . '(' . $this->convertedArgs($m, $cm) . ')';
            $arms[] = $h . '::' . $c->variant() . '(__h) => ' . $this->casts->convert($call, $cm->return_type, $m->return_type);
        }
        if ($cls->has_downstream) {
            $arms[] = $h . '::Other__(__m) => ' . $this->dynamicCall($m);
        }
        $arms[] = '_ => ' . ($shared ?? 'unreachable!()');
        $w->line('pub fn ' . $rn . $this->signature($m, true) . ' { match self { ' . implode(', ', $arms) . ' } }');
        if ($m->declaring === $cls && !$m->isAbstract()) {
            $w->line('pub fn ' . $rn . '__impl' . $this->signature($m, true) . ' {');
            $w->raw($this->body($cls, $m));
            $w->line('}');
        } elseif ($m->declaring !== $cls && !$m->isAbstract() && !$m->declaring->isLeaf()) {
            // make `__impl` reachable through this handle too (parent::foo() from grandchildren)
            $up = $this->casts->convert('self.clone()', RustType::class($cls->fqcn), RustType::class($m->declaring->fqcn));
            $w->line('pub fn ' . $rn . '__impl' . $this->signature($m, true) . ' { ' . $up . '.' . $rn . '__impl(' . $this->argNames($m) . ') }');
        }
    }

    /**
     * A call of `$m` on an object held in the escape variant (an instance of a subclass defined in
     * another crate, or a value the docblocks lied about): dispatched by name through `PhpObject::call_method`.
     */
    private function dynamicCall(MethodModel $m): string
    {
        $args = [];
        foreach ($m->storage->params as $i => $p) {
            $pt = $m->param_types[$i] ?? RustType::mixed();
            $name = Names::var($p->name);
            $args[] = $this->casts->convert($p->by_ref ? '(*' . $name . ').clone()' : $name, $pt, RustType::mixed());
        }
        $call = 'php_rt::other_obj(__m).call_method(' . Names::rustStringLiteral($m->lc()) . ', vec![' . implode(', ', $args) . '])';
        $rt = $m->return_type;
        if ($rt->kind === RustType::UNIT) {
            return '{ let _ = ' . $call . '; () }';
        }
        if ($rt->kind === RustType::NEVER) {
            return '{ let _ = ' . $call . '; unreachable!() }';
        }
        return $this->casts->convert($call, RustType::mixed(), $rt);
    }

    /** Arguments passed from a base-class signature to an overriding method (contravariant params). */
    private function convertedArgs(MethodModel $base, MethodModel $override): string
    {
        $args = [];
        foreach ($base->storage->params as $i => $p) {
            if (!isset($override->storage->params[$i])) {
                break;
            }
            $bt = $base->param_types[$i] ?? RustType::mixed();
            $ot = $override->param_types[$i] ?? $bt;
            $name = Names::var($p->name);
            if ($p->by_ref) {
                $args[] = $name;
            } else {
                $args[] = $this->casts->convert($name, $bt, $ot);
            }
        }
        // extra params of the override get defaults
        for ($i = count($base->storage->params); $i < count($override->storage->params); $i++) {
            $default = $this->casts->defaultOf($override->param_types[$i] ?? RustType::mixed());
            $args[] = ($override->storage->params[$i]->by_ref ? '&mut ' : '') . $default;
        }
        return implode(', ', $args);
    }

    private function body(ClassModel $cls, MethodModel $m, ?ClassModel $static_class = null): string
    {
        if ($m->record === null || $m->node === null) {
            $this->diag->warn('method without analysis record', $m->node, $cls->fqcn);
            return "    unreachable!(\"method " . $m->name . " was not analyzed\")\n";
        }
        $b = new BodyEmitter($this->program, $m->record, $cls, $this->casts, $this->builtins, $this->diag, null, $static_class);
        $b->generics = $m->generics;
        if ($m->isStatic()) {
            $b->this_type = null;
        }
        if ($m->origin()->declaring !== $cls) {
            // a body inherited from another crate: `self`/`parent` keep referring to the declaring class
            $b->self_class = $m->origin()->declaring;
        }
        $params = [];
        foreach ($m->storage->params as $i => $p) {
            $params[$p->name] = $m->param_types[$i] ?? RustType::mixed();
            if ($p->by_ref) {
                $b->byref[$p->name] = true;
            } elseif (isset($m->borrow_params[$i])) {
                $b->borrow[$p->name] = true;
            }
        }
        $b->throws = $m->throws;
        try {
            $code = $b->emitBody($params, $m->node->stmts, $m->return_type);
        } catch (\Throwable $e) {
            $this->diag->warn('transpiler error: ' . $e->getMessage() . ' @ ' . basename($e->getFile()) . ':' . $e->getLine(), $m->node, $m->record->file_path);
            return "    unreachable!(\"transpiler error in " . $m->name . "\")\n";
        }
        if (strtolower($m->name) === '__construct') {
            // constructor property promotion: the parameters initialize the properties first
            $pre = '';
            foreach ($m->storage->params as $i => $p) {
                $field = $p->promoted_property ? ($cls->fields[$p->name] ?? null) : null;
                if ($field === null) {
                    continue;
                }
                $pt = $m->param_types[$i] ?? RustType::mixed();
                $val = $p->by_ref ? '(*' . Names::var($p->name) . ').clone()' : Names::var($p->name) . '.clone()';
                $pre .= 'self.set_' . $field->acc() . '(' . $this->casts->convert($val, $pt, $field->type) . ");\n";
            }
            $code = $pre . $code;
        }
        return $this->indent($code);
    }

    /** A copy of upstream method `$m` on `$root`'s handle type, named `$name` (see Program::requestSuperCopy). */
    /**
     * Dispatch accessors (set_/get_) for a CONCRETE-VARIANT-SPECIFIC field `$f` on a base/interface enum `$enum`,
     * requested when code writes/reads that field through an enum-typed local narrowed to a variant that has it
     * (MutableTypeVisitor). Arms cover the concrete variants that declare the field; others are unreachable (the
     * access is guarded by the narrowing that reached this variant). set_ is `&mut self` (works for the concrete
     * variant's &self Cell setter or &mut make_mut setter); get_ is `&self`.
     */
    public function emitEnumFieldAccessor(ClassModel $enum, FieldModel $f, RustType $ft, Writer $w): void
    {
        $this->program->types->context = '<variant field accessor> ' . $enum->fqcn . '::$' . $f->name;
        $h = $enum->handle();
        $t = $ft->toRust();
        $set = [];
        $get = [];
        foreach ($enum->concrete as $c) {
            $cf = $c->fields[$f->name] ?? null;
            if ($cf === null || !$this->casts->fits($cf->type, $ft)) {
                continue;
            }
            $set[] = $h . '::' . $c->variant() . '(__h) => __h.set_' . $cf->acc() . '(' . $this->casts->convert('v', $ft, $cf->type) . ')';
            $get[] = $h . '::' . $c->variant() . '(__h) => ' . $this->casts->convert('__h.' . $cf->acc() . '_get()', $cf->type, $ft);
        }
        if ($set === []) {
            return;
        }
        $rn = $f->acc();
        $w->line('impl ' . $h . ' {');
        $w->line('pub fn set_' . $rn . '(&mut self, v: ' . $t . ') { match self { ' . implode(', ', $set) . ', _ => unreachable!("set_' . $rn . ' on wrong ' . $h . ' variant") } }');
        $w->line('pub fn ' . $rn . '_get(&self) -> ' . $t . ' { match self { ' . implode(', ', $get) . ', _ => unreachable!("' . $rn . '_get on wrong ' . $h . ' variant") } }');
        $w->line('}');
    }

    public function emitSuperCopy(ClassModel $root, MethodModel $m, string $name, Writer $w): void
    {
        // A construction super-copy for an immutable class runs on its OWN handle (the concrete newtype) where the
        // in-place &mut construction happens — for a concrete-non-leaf, ownHandle() ({T}Self) differs from handle()
        // (the dispatch enum). Non-ctor / cross-crate super-copies stay on the enum handle. (Leaves: ownHandle()==handle().)
        $ctor_immut = !$m->isStatic() && $root->isImmutableCtorMethod($m->lc());
        $w->line('impl ' . ($ctor_immut ? $root->ownHandle() : $root->handle()) . ' {');
        if ($m->isStatic()) {
            // `parent::m()` from an overriding static method: the parent's body with `static` bound to `$root`
            $w->line('pub fn ' . $name . $this->signature($m, false) . ' {');
            $w->raw($this->body($m->declaring, $m, $root));
        } else {
            // A construction method copied onto an immutable leaf writes $this in place -> &mut self; make_mut on the
            // fresh object (refcount 1 during construction) mutates it directly (no clone/COW).
            $w->line('pub fn ' . $name . $this->signature($m, true, $root->isImmutableCtorMethod($m->lc())) . ' {');
            $w->raw($this->body($root, $m));
        }
        $w->line('}');
        $w->line('}');
    }

    private function indent(string $code): string
    {
        $out = '';
        foreach (explode("\n", rtrim($code, "\n")) as $line) {
            $out .= '    ' . $line . "\n";
        }
        return $out;
    }

    // ------------------------------------------------------------------ statics and constants

    private function emitStatic(ClassModel $cls, FieldModel $f, Writer $w): void
    {
        $rn = $f->rustName();
        $t = $f->type->toRust();
        $body = $this->constExprEmitter($cls);
        if ($f->default === null && !$f->type->hasDefault()) {
            // `public static Foo $instance;` without initializer: unset until first assigned
            $w->line('pub fn st_' . $rn . '_cell() -> &\'static std::thread::LocalKey<RefCell<Late<' . $t . '>>> { thread_local! { static CELL: RefCell<Late<' . $t . '>> = RefCell::new(Late::uninit()); } &CELL }');
            $w->line('pub fn st_' . $rn . '() -> ' . $t . ' { Self::st_' . $rn . '_cell().with(|c| c.borrow().get().clone()) }');
            $w->line('pub fn st_' . $rn . '_opt() -> Option<' . $t . '> { Self::st_' . $rn . '_cell().with(|c| c.borrow().as_option().cloned()) }');
            $w->line('pub fn st_' . $rn . '_set(v: ' . $t . ') { Self::st_' . $rn . '_cell().with(|c| { c.borrow_mut().set(v); }) }');
            $w->line('pub fn st_' . $rn . '_with<R>(f: impl FnOnce(&mut ' . $t . ') -> R) -> R { Self::st_' . $rn . '_cell().with(|c| f(c.borrow_mut().get_mut())) }');
            return;
        }
        $init = $f->default !== null ? $body->constExpr($f->default, $f->type) : $this->casts->defaultOf($f->type);
        $w->line('pub fn st_' . $rn . '_cell() -> &\'static std::thread::LocalKey<RefCell<' . $t . '>> { thread_local! { static CELL: RefCell<' . $t . '> = RefCell::new(' . $init . '); } &CELL }');
        $w->line('pub fn st_' . $rn . '() -> ' . $t . ' { Self::st_' . $rn . '_cell().with(|c| c.borrow().clone()) }');
        $w->line('pub fn st_' . $rn . '_opt() -> Option<' . $t . '> { Some(Self::st_' . $rn . '()) }');
        $w->line('pub fn st_' . $rn . '_set(v: ' . $t . ') { Self::st_' . $rn . '_cell().with(|c| { *c.borrow_mut() = v; }) }');
        $w->line('pub fn st_' . $rn . '_with<R>(f: impl FnOnce(&mut ' . $t . ') -> R) -> R { Self::st_' . $rn . '_cell().with(|c| f(&mut *c.borrow_mut())) }');
    }

    private function emitConstant(ClassModel $cls, ConstModel $c, Writer $w): void
    {
        $rn = $c->rustName();
        $t = $c->type;
        if ($c->expr === null) {
            $w->line('pub fn ' . $rn . '() -> ' . $t->toRust() . ' { ' . $this->casts->defaultOf($t) . ' }');
            return;
        }
        $body = $this->constExprEmitter($cls);
        $code = $body->constExpr($c->expr, $t);
        if ($t->isCopy() || $t->kind === RustType::STR) {
            $w->line('pub fn ' . $rn . '() -> ' . $t->toRust() . ' { ' . $code . ' }');
        } else {
            $w->line('pub fn ' . $rn . '() -> ' . $t->toRust() . ' { thread_local! { static V: ' . $t->toRust() . ' = ' . $code . '; } V.with(|v| v.clone()) }');
        }
    }

    // ------------------------------------------------------------------ PhpObject and enum impls

    /**
     * The runtime object trait for the class (and, for a hierarchy root, for its enum handle). Emitted after
     * every body, because the dynamic part (props/get_prop/set_prop/call_method: the object viewed through
     * Mixed) is only generated for classes some body actually erases to Mixed ($dynamic); the others keep the
     * runtime's panicking defaults.
     */
    public function emitObjectProtocol(ClassModel $cls, Writer $w, bool $dynamic): void
    {
        if ($cls->isEnum()) {
            return; // PHP enums carry their own (static) object impl
        }
        if ($cls->isConcrete()) {
            $this->emitPhpObject($cls, $w, $dynamic);
        }
        if (!$cls->isLeaf()) {
            $this->emitEnumHandleProtocol($cls, $w, $dynamic);
        }
    }

    private function emitPhpObject(ClassModel $cls, Writer $w, bool $dynamic): void
    {
        $own = $cls->ownHandle();
        $w->open('impl php_rt::PhpObject for ' . $own . ' {');
        $w->line('fn class_name(&self) -> &\'static str { ' . Names::rustStringLiteral($cls->fqcn) . ' }');
        $w->line('fn class_ancestors(&self) -> &\'static [&\'static str] { ' . $this->ancestorsLiteral($cls) . ' }');
        $w->line('fn class_id(&self) -> u32 { ' . $this->program->classId($cls) . ' }');
        $w->line('fn class_ancestor_ids(&self) -> &\'static [u32] { ' . $this->ancestorIdsLiteral($cls) . ' }');
        $w->line('fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }');
        $w->line('fn as_any(&self) -> &dyn std::any::Any { self }');
        $w->line('fn php_clone_dyn(&self) -> AnyObj { Rc::new(self.php_clone()) }');
        $ts = $this->program->findMethod($cls, '__tostring');
        if ($ts !== null) {
            $w->line('fn php_to_string(&self) -> Option<Str> { Some(self.' . $ts->rustName() . '()) }');
        }
        if (!$dynamic) {
            $w->close();
            return;
        }
        $props = [];
        foreach ($cls->fields as $f) {
            $get = $f->isLate() ? 'self.' . $f->acc() . '_opt()' : 'Some(self.' . $f->acc() . '_get())';
            $props[] = 'if let Some(v) = ' . $get . ' { out.push((Str::from_static(' . Names::rustStringLiteral($f->name) . '), ' . $this->casts->convert('v', $f->type, RustType::mixed()) . ')); }';
        }
        $w->line('fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); ' . implode(' ', $props) . ' out }');
        $pub = [];
        foreach ($cls->fields as $f) {
            if ($f->storage->visibility !== \Psalm\Internal\Analyzer\ClassLikeAnalyzer::VISIBILITY_PUBLIC) {
                continue;
            }
            $get = $f->isLate() ? 'self.' . $f->acc() . '_opt()' : 'Some(self.' . $f->acc() . '_get())';
            $pub[] = 'if let Some(v) = ' . $get . ' { out.push((Str::from_static(' . Names::rustStringLiteral($f->name) . '), ' . $this->casts->convert('v', $f->type, RustType::mixed()) . ')); }';
        }
        $w->line('fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); ' . implode(' ', $pub) . ' out }');
        // Only properties ever dynamically accessed (via mixed_get_prop/mixed_set_prop / other_obj.get_prop
        // on an object/Mixed receiver) need an arm; others are dead here (their cast::<Mixed> was a bulk of the
        // residual Mixed). See DYN_ACCESS_PROPS (case-sensitive, PHP property names are not case-folded).
        $sets = [];
        // Immutable (Rc<T>) classes have no interior mutability, and external set_prop on them violates the
        // @psalm-immutable contract anyway — so emit no set arms (returns false); this also avoids an
        // uncompilable &self->&mut set_p_X call for any dyn-accessed field (e.g. Union's `types`).
        if (!$cls->immutable()) {
            foreach ($cls->fields as $f) {
                if (!isset(self::DYN_ACCESS_PROPS[$f->name])) {
                    continue;
                }
                $sets[] = Names::rustStringLiteral($f->name) . ' => { self.set_' . $f->acc() . '(' . $this->casts->convert('value', RustType::mixed(), $f->type) . '); true }';
            }
        }
        $w->line('fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { ' . implode(', ', $sets) . ($sets ? ', ' : '') . '_ => false } }');
        $gets = [];
        foreach ($cls->fields as $f) {
            if (!isset(self::DYN_ACCESS_PROPS[$f->name])) {
                continue;
            }
            $get = $f->isLate() ? 'self.' . $f->acc() . '_opt()' : 'Some(self.' . $f->acc() . '_get())';
            $gets[] = Names::rustStringLiteral($f->name) . ' => ' . $get . '.map(|v| ' . $this->casts->convert('v', $f->type, RustType::mixed()) . ')';
        }
        $w->line('fn get_prop(&self, name: &str) -> Option<Mixed> { match name { ' . implode(', ', $gets) . ($gets ? ', ' : '') . '_ => None } }');
        $w->line('fn call_method(&self, name: &str, args: Vec<Mixed>) -> Mixed { match name { ' . $this->callMethodArms($cls) . '_ => php_rt::do_throw(Throw::error(cat!(Str::from_static(' . Names::rustStringLiteral('Call to undefined method ' . $cls->fqcn . '::') . '), Str::from_str(name), Str::from_static("()")))) } }');
        $w->close();
    }

    /** `to_php_string()` and `clone` support on the own handle (class-time emission). */
    private function emitOwnCloneImpls(ClassModel $cls, Writer $w): void
    {
        $own = $cls->ownHandle();
        $ts = $this->program->findMethod($cls, '__tostring');
        $w->line('impl ' . $own . ' { pub fn to_php_string(&self) -> Str { ' . ($ts !== null ? 'self.' . $ts->rustName() . '()' : 'panic!(' . Names::rustStringLiteral('Uncaught exception: Object of class ' . $cls->fqcn . ' could not be converted to string') . ')') . ' } }');
        $clone = $this->program->findMethod($cls, '__clone');
        $clone_call = '';
        if ($clone !== null) {
            $recv = $clone->declaring === $cls || !$clone->isPrivate()
                ? 'c'
                : $this->casts->convert('c.clone()', RustType::class($cls->fqcn), RustType::class($clone->declaring->fqcn));
            $clone_call = 'let _ = ' . $recv . '.' . $clone->rustName() . '(); ';
        }
        $clone_cell = $cls->immutable() ? $own . '(Rc::new((*self.0).clone()))' : $own . '(Rc::new(RefCell::new(self.0.borrow().clone())))';
        $w->line('impl php_rt::PhpClone for ' . $own . ' { fn php_clone(&self) -> Self { let ' . ($cls->immutable() && $clone_call !== '' ? 'mut ' : '') . 'c = ' . $clone_cell . '; ' . $clone_call . 'c } }');
        $w->line('impl Clone for ' . $cls->objStruct() . ' { fn clone(&self) -> Self { ' . $cls->objStruct() . ' { ' . implode(', ', array_map(fn(FieldModel $f) => $f->rustName() . ': self.' . $f->rustName() . '.clone()', $cls->fields)) . ' } } }');
    }

    /**
     * The ONLY method names ever dynamically dispatched (via `PhpObject::call_method`) across the whole
     * program — i.e. called on a receiver the transpiler types as `object`/AnyObject/Mixed rather than a
     * concrete class. Derived from a census of every `.call_method("<name>")` in the generated crate (there
     * are no dynamic-name `$o->$m()` calls, and `mixed_call_method` is never invoked), so any method NOT in
     * this set is provably never reached through the dynamic protocol and needs no `call_method` arm — which
     * elides its `dyn_arg` argument marshalling (the bulk of the residual `Mixed`). Keyed lowercase.
     * TODO(dephp): replace this census-derived constant with a transpiler pass that collects the names during
     * emission (record the method name whenever a call is emitted through the dynamic protocol).
     * @var array<string, true>
     */
    public const DYN_DISPATCH_METHODS = [
        'createmock' => true, 'await' => true, 'equalto' => true, 'check' => true, 'run' => true, 'method' => true,
        'loginfo' => true, 'disableextension' => true, 'close' => true, 'willreturn' => true, 'runall' => true, 'render' => true,
        'matches' => true, 'getcount' => true, 'getattribute' => true, 'fetch' => true, 'disableextensions' => true,
        'setsubnode' => true, 'setattributes' => true, 'send' => true, 'receive' => true, 'parseconstraints' => true,
        'settypes' => true, 'setheaders' => true, 'setdefaultcommand' => true,
    ];

    /**
     * The ONLY property names ever dynamically accessed (via mixed_get_prop/mixed_set_prop / other_obj.get_prop
     * on an object/Mixed receiver), censused from the generated crate. A property NOT here is never reached
     * through get_prop/set_prop, so its arm (and its cast::<Mixed>) is elided. CASE-SENSITIVE (PHP property
     * names are not case-folded, unlike method names). Self-validated at the property-access emit sites.
     * @var array<string, true>
     */
    public const DYN_ACCESS_PROPS = [
        'type' => true, 'name' => true, 'stmts' => true, 'signature_type' => true, 'out_type' => true, 'extra_types' => true,
        'default_type' => true, 'value' => true, 'types' => true, 'type_params' => true, 'stdout_report_options' => true, 'track_mutations' => true,
        'textContent' => true, 'props' => true, 'nodeValue' => true, 'is_static' => true, 'generated_report_options' => true, 'eventDispatcher' => true,
        'enableJit' => true,
    ];

    /** Match arms (lowercase method name => dynamic invocation) for `PhpObject::call_method`. */
    private function callMethodArms(ClassModel $cls, bool $static_only = false): string
    {
        $arms = '';
        foreach ($cls->methods as $m) {
            // Only methods ever reached through the dynamic protocol need an arm; all others are dead here
            // (their dyn_arg marshalling was the bulk of the residual Mixed). See DYN_DISPATCH_METHODS.
            if (!isset(self::DYN_DISPATCH_METHODS[$m->lc()])) {
                continue;
            }
            if ($m->isAbstract() && !$cls->isInterface()) {
                continue;
            }
            if ($static_only && !$m->isStatic()) {
                continue;
            }
            if ($m->isPrivate() && $m->declaring !== $cls) {
                continue;
            }
            // non-public methods stay reachable: untyped receivers inside the class (`$self->reduce()`
            // in php-parser's semantic actions) are dispatched by name too
            if ($static_only && $m->declaring !== $cls) {
                // inherited statics are dispatched by the declaring class (see the `_` arm)
                continue;
            }
            $params = [];
            $pre = '';
            $ok = true;
            foreach ($m->storage->params as $i => $p) {
                if ($p->is_variadic) {
                    $ok = false;
                    break;
                }
                $pt = $m->param_types[$i] ?? RustType::mixed();
                if (Casts::isLocal($pt)) {
                    $this->casts->needMixedTo($pt);
                }
                $inner = $pt->kind === RustType::OPTION ? $pt->inner() : $pt;
                if ($p->by_ref) {
                    // a by-reference parameter of a dynamically called method (`enterNode(Node $node, bool
                    // &$traverseChildren = true)` in Psalm's visitors): a local cell seeded from the argument or
                    // the parameter's default; the caller does not see writes to it
                    if (!$pt->hasDefault()) {
                        $ok = false;
                        break;
                    }
                    $seed = $pt->kind === RustType::MIXED ? 'dyn_arg_req::<Mixed>(&args, ' . $i . ')' : 'dyn_arg::<' . $pt->toRust() . '>(&args, ' . $i . ')';
                    if ($p->default_type !== null || $pt->kind === RustType::OPTION) {
                        $seed = 'match args.get(' . $i . ') { Some(_) => ' . $seed . ', None => ' . $this->casts->defaultOf($pt) . ' }';
                    }
                    $pre .= 'let mut __ref' . $i . ': ' . $pt->toRust() . ' = ' . $seed . '; ';
                    $params[] = '&mut __ref' . $i;
                    continue;
                }
                if ($pt->kind === RustType::MIXED) {
                    $pc = 'dyn_arg_req::<Mixed>(&args, ' . $i . ')';
                } elseif (in_array($inner->kind, [RustType::CLOSURE, RustType::TUPLE, RustType::DYN_CALLABLE, RustType::RT_GENERIC, RustType::RESOURCE], true)) {
                    $pc = $this->casts->convert('dyn_arg_req::<Mixed>(&args, ' . $i . ')', RustType::mixed(), $pt);
                } elseif ($pt->hasDefault() && $pt->kind !== RustType::CLASS_) {
                    $pc = 'dyn_arg::<' . $pt->toRust() . '>(&args, ' . $i . ')';
                } else {
                    $pc = 'dyn_arg_req::<' . $pt->toRust() . '>(&args, ' . $i . ')';
                }
                // a `&T` (borrow-safe) param: borrow the marshalled owned temporary for the call
                $params[] = isset($m->borrow_params[$i]) ? '&' . $pc : $pc;
            }
            if (!$ok) {
                continue;
            }
            $call = ($m->isStatic() ? $m->declaring->path() . '::' : 'self.') . $m->rustName() . '(' . implode(', ', $params) . ')';
            $arms .= Names::rustStringLiteral($m->lc()) . ' => { ' . $pre . 'let __r = ' . $call . '; ' . $this->casts->convert('__r', $m->return_type, RustType::mixed()) . ' }, ';
        }
        return $arms;
    }

    private function enumArms(ClassModel $cls): callable
    {
        $h = $cls->handle();
        return fn(string $call) => implode(', ', array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(__h) => __h.' . $call, $cls->concrete)) . ($cls->concrete ? ', ' : '') . ($cls->has_downstream ? $h . '::Other__(__m) => php_rt::other_obj(__m).' . $call . ', ' : '') . '_ => unreachable!()';
    }

    private function emitEnumHandleProtocol(ClassModel $cls, Writer $w, bool $dynamic): void
    {
        $h = $cls->handle();
        $arms = $this->enumArms($cls);
        $w->open('impl php_rt::PhpObject for ' . $h . ' {');
        $w->line('fn class_name(&self) -> &\'static str { match self { ' . $arms('class_name()') . ' } }');
        $w->line('fn class_ancestors(&self) -> &\'static [&\'static str] { match self { ' . $arms('class_ancestors()') . ' } }');
        $w->line('fn class_id(&self) -> u32 { match self { ' . $arms('class_id()') . ' } }');
        $w->line('fn class_ancestor_ids(&self) -> &\'static [u32] { match self { ' . $arms('class_ancestor_ids()') . ' } }');
        $w->line('fn obj_id(&self) -> usize { match self { ' . $arms('obj_id()') . ' } }');
        $w->line('fn as_any(&self) -> &dyn std::any::Any { self }');
        $w->line('fn php_clone_dyn(&self) -> AnyObj { match self { ' . $arms('php_clone_dyn()') . ' } }');
        $w->line('fn php_to_string(&self) -> Option<Str> { match self { ' . $arms('php_to_string()') . ' } }');
        if ($dynamic) {
            $w->line('fn props(&self) -> Vec<(Str, Mixed)> { match self { ' . $arms('props()') . ' } }');
            $w->line('fn set_prop(&self, name: &str, value: Mixed) -> bool { match self { ' . $arms('set_prop(name, value)') . ' } }');
            $w->line('fn get_prop(&self, name: &str) -> Option<Mixed> { match self { ' . $arms('get_prop(name)') . ' } }');
            $w->line('fn call_method(&self, name: &str, args: Vec<Mixed>) -> Mixed { match self { ' . $arms('call_method(name, args)') . ' } }');
            $w->line('fn public_props(&self) -> Vec<(Str, Mixed)> { match self { ' . $arms('public_props()') . ' } }');
        }
        $w->close();
    }

    private function emitEnumHandleImpls(ClassModel $cls, Writer $w): void
    {
        $this->program->types->context = '<handle impls> ' . $cls->fqcn;
        $h = $cls->handle();
        $arms = $this->enumArms($cls);
        $enum_iface = in_array(strtolower($cls->fqcn), ['unitenum', 'backedenum'], true);
        if ($enum_iface || ($cls->concrete !== [] && !array_filter($cls->concrete, static fn(ClassModel $c) => !$c->isEnum()))) {
            // an interface implemented only by PHP enums (UnitEnum/BackedEnum): `->name` / `->value` dispatch
            $name_arms = array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(__h) => __h.name()', $cls->concrete);
            $w->line('impl ' . $h . ' { pub fn name(&self) -> Str { match self { ' . implode('', array_map(static fn($a) => $a . ', ', $name_arms)) . '_ => unreachable!() } } }');
            $backing = array_values(array_unique(array_map(static fn(ClassModel $c) => (string) $c->storage->enum_type, $cls->concrete)));
            if ($backing === [] && strtolower($cls->fqcn) === 'backedenum') {
                $backing = ['string'];
            }
            if (count($backing) === 1 && $backing[0] !== '') {
                $vt = $backing[0] === 'int' ? 'i64' : 'Str';
                $value_arms = array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(__h) => __h.value()', $cls->concrete);
                $w->line('impl ' . $h . ' { pub fn value(&self) -> ' . $vt . ' { match self { ' . implode('', array_map(static fn($a) => $a . ', ', $value_arms)) . '_ => unreachable!() } } }');
            }
        }
        $to_string_arms = implode(', ', array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(__h) => __h.to_php_string()', $cls->concrete)) . ($cls->concrete ? ', ' : '') . ($cls->has_downstream ? $h . '::Other__(__m) => to_str(__m), ' : '') . '_ => unreachable!()';
        $w->line('impl ' . $h . ' { pub fn to_php_string(&self) -> Str { match self { ' . $to_string_arms . ' } } }');
        $w->line('impl ' . $h . ' { pub fn inner_any(&self) -> &dyn std::any::Any { match self { ' . implode(', ', array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(__h) => __h', $cls->concrete)) . ($cls->concrete ? ', ' : '') . ($cls->has_downstream ? $h . '::Other__(__m) => __m, ' : '') . '_ => unreachable!() } } }');
        $w->line('impl php_rt::PhpClone for ' . $h . ' { fn php_clone(&self) -> Self { match self { ' . implode(', ', array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(__h) => ' . $h . '::' . $c->variant() . '(__h.php_clone())', $cls->concrete)) . ($cls->concrete ? ', ' : '') . ($cls->has_downstream ? $h . '::Other__(__m) => ' . $h . '::Other__(__m.clone()), ' : '') . '_ => unreachable!() } } }');
    }

    // ------------------------------------------------------------------ enums

    private function emitEnum(ClassModel $cls, Writer $w): void
    {
        $h = $cls->handle();
        $backing = $cls->storage->enum_type === 'int' ? RustType::int() : ($cls->storage->enum_type === 'string' ? RustType::str() : null);
        $cases = [];
        foreach ($cls->storage->enum_cases as $name => $case) {
            $cases[$name] = $case;
        }
        $w->line('#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]');
        $w->open('pub enum ' . $h . ' {');
        foreach ($cases as $name => $_) {
            $w->line(Names::typeIdent($name) . ',');
        }
        $w->close();
        $w->open('impl ' . $h . ' {');
        $w->line('pub fn cases() -> List<' . $h . '> { list![' . implode(', ', array_map(fn($n) => $h . '::' . Names::typeIdent($n), array_keys($cases))) . '] }');
        $w->line('pub fn name(&self) -> Str { match self { ' . implode(', ', array_map(fn($n) => $h . '::' . Names::typeIdent($n) . ' => Str::from_static(' . Names::rustStringLiteral($n) . ')', array_keys($cases))) . ' } }');
        if ($backing !== null) {
            $body = $this->constExprEmitter($cls);
            $vals = [];
            foreach ($cases as $name => $case) {
                $expr = null;
                if ($cls->node !== null) {
                    foreach ($cls->node->stmts as $stmt) {
                        if ($stmt instanceof \PhpParser\Node\Stmt\EnumCase && $stmt->name->name === $name) {
                            $expr = $stmt->expr;
                        }
                    }
                }
                $vals[$name] = $expr !== null ? $body->constExpr($expr, $backing) : $this->casts->defaultOf($backing);
            }
            $w->line('pub fn value(&self) -> ' . $backing->toRust() . ' { match self { ' . implode(', ', array_map(fn($n) => $h . '::' . Names::typeIdent($n) . ' => ' . $vals[$n], array_keys($cases))) . ' } }');
            $w->line('pub fn try_from_value(v: ' . $backing->toRust() . ') -> Option<' . $h . '> { for c in Self::cases().iter() { if identical(&c.value(), &v) { return Some(*c); } } None }');
            $w->line('pub fn from_value(v: ' . $backing->toRust() . ') -> ' . $h . ' { Self::try_from_value(v.clone()).unwrap_or_else(|| panic!("Uncaught exception: {} is not a valid backing value for enum ' . addslashes($cls->fqcn) . '", to_str(&v))) }');
        }
        foreach ($cls->constants as $c) {
            $this->emitConstant($cls, $c, $w);
        }
        foreach ($cls->methods as $m) {
            if ($m->declaring !== $cls || $m->isAbstract()) {
                continue;
            }
            $w->line('pub fn ' . $m->rustName() . $this->signature($m, !$m->isStatic()) . ' {');
            $w->raw($this->body($cls, $m));
            $w->line('}');
        }
        $w->close();
        $w->line('impl_enum_handle!(' . $h . ', ' . Names::rustStringLiteral($cls->fqcn) . ', ' . $this->ancestorsLiteral($cls) . ');');
        $w->line('impl ' . $h . ' { pub fn to_php_string(&self) -> Str { panic!(' . Names::rustStringLiteral('Uncaught exception: Object of class ' . $cls->fqcn . ' could not be converted to string') . ') } }');
        $w->line('impl ' . $h . ' { pub fn new_same_class(&self) -> Self { panic!(' . Names::rustStringLiteral('Cannot instantiate enum ' . $cls->fqcn) . ') } }');
    }
}
