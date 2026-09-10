<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Stmt\ClassMethod;

use function count;
use function implode;
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
                $w->line('pub ' . $f->rustName() . ': ' . $f->storageType() . ',');
            }
            $w->close();
            $w->line('#[derive(Clone)]');
            $w->line('pub struct ' . $own . '(pub Rc<RefCell<' . $obj . '>>);');
        }

        // ---- dispatch enum for non-leaf classes and interfaces
        if (!$leaf) {
            $w->line('#[derive(Clone)]');
            $w->open('pub enum ' . $handle . ' {');
            foreach ($cls->concrete as $c) {
                $w->line($c->variant() . '(' . $c->ownPath() . '),');
            }
            // escape hatch for values that are not instances of this class (docblocks lying)
            $w->line('Other__(Mixed),');
            $w->close();
            $this->emitEnumHandleImpls($cls, $w);
        }

        // ---- accessors and methods on the own handle
        if ($concrete) {
            $w->open('impl ' . $own . ' {');
            foreach ($cls->fields as $f) {
                $this->emitAccessors($f, $w);
            }
            $this->emitConstructor($cls, $w);
            foreach ($cls->methods as $m) {
                $this->emitMethodOnOwn($cls, $m, $w);
            }
            $w->line('pub fn new_same_class' . $this->ctorSig($cls, true) . ' -> Result<' . $cls->path() . ', Throw> { Ok(' . $this->wrapOwn($cls, 'Self::new(' . $this->ctorArgs($cls) . ')?') . ') }');
            $w->close();
            $this->emitPhpObject($cls, $w);
        }

        // ---- statics, constants and bodies live on the handle type
        $w->open('impl ' . $handle . ' {');
        if (!$cls->isInterface() && !$cls->isEnum()) {
            // `$class::method(...)` with a runtime class name; inherited statics are looked up in the parent
            $fallback = $cls->parent !== null && $cls->parent->is_project && !$cls->parent->isInterface() && !$cls->parent->isEnum()
                ? $cls->parent->path() . '::call_static(name, args)'
                : 'Err(DynError::Rt(RtError::error(format!("Call to undefined static method {}::{}()", ' . Names::rustStringLiteral($cls->fqcn) . ', name))))';
            $w->line('pub fn call_static(name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { ' . $this->callMethodArms($cls, true) . '_ => ' . $fallback . ' } }');
        }
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
                $w->line('pub fn new' . $this->ctorSig($cls) . ' -> Result<' . $handle . ', Throw> { Ok(' . $handle . '::' . $cls->variant() . '(' . $own . '::new(' . $this->ctorArgs($cls) . ')?)) }');
            }
            $w->line('pub fn new_same_class' . $this->ctorSig($cls, true) . ' -> Result<' . $handle . ', Throw> { match self { ' . implode(', ', array_map(fn(ClassModel $c) => $handle . '::' . $c->variant() . '(__h) => Ok(' . $this->casts->convert('__h.new_same_class(' . $this->ctorArgsFor($cls, $c) . ')?', RustType::class($c->fqcn), RustType::class($cls->fqcn)) . ')', $cls->concrete)) . ($cls->concrete ? ', ' : '') . '_ => unreachable!() } }');
        }
        $w->close();

        // factories for `new $name(...)` are emitted crate-wide once all bodies have been seen
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

    private function emitAccessors(FieldModel $f, Writer $w): void
    {
        $fld = $f->rustName();
        $rn = $f->acc();
        $t = $f->type->toRust();
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

    private function emitEnumAccessors(ClassModel $cls, FieldModel $f, Writer $w): void
    {
        $rn = $f->acc();
        $t = $f->type->toRust();
        $h = $cls->handle();
        $arms = fn(string $call) => implode(', ', array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(__h) => __h.' . $call, $cls->concrete)) . ($cls->concrete ? ', ' : '') . '_ => unreachable!()';
        $w->line('pub fn ' . $rn . '(&self) -> Ref<\'_, ' . $t . '> { match self { ' . $arms($rn . '()') . ' } }');
        $w->line('pub fn ' . $rn . '_get(&self) -> ' . $t . ' { match self { ' . $arms($rn . '_get()') . ' } }');
        $w->line('pub fn ' . $rn . '_opt(&self) -> Option<' . $t . '> { match self { ' . $arms($rn . '_opt()') . ' } }');
        $w->line('pub fn ' . $rn . '_mut(&self) -> RefMut<\'_, ' . $t . '> { match self { ' . $arms($rn . '_mut()') . ' } }');
        $w->line('pub fn set_' . $rn . '(&self, v: ' . $t . ') { match self { ' . $arms('set_' . $rn . '(v)') . ' } }');
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
        $own = $cls->ownHandle();
        $obj = $cls->objStruct();
        $body = $this->constExprEmitter($cls);
        $inits = [];
        foreach ($cls->fields as $f) {
            $inits[] = $f->rustName() . ': ' . $this->fieldInit($f, $body) . ',';
        }
        // an instance with initialized defaults but without running the constructor
        // (ReflectionClass::newInstanceWithoutConstructor, unserialize)
        $w->open('pub fn new_uninit() -> ' . $own . ' {');
        $w->open($own . '(Rc::new(RefCell::new(' . $obj . ' {');
        foreach ($inits as $line) {
            $w->line($line);
        }
        $w->close('})))');
        $w->close();
        $w->open('pub fn new' . $this->ctorSig($cls) . ' -> Result<' . $own . ', Throw> {');
        $w->open('let this = ' . $own . '(Rc::new(RefCell::new(' . $obj . ' {');
        foreach ($inits as $line) {
            $w->line($line);
        }
        $w->close('})));');
        $ctor = $this->program->findMethod($cls, '__construct');
        if ($ctor !== null && !$ctor->isAbstract()) {
            $w->line('this.' . $ctor->rustName() . '(' . $this->ctorArgs($cls) . ')?;');
        }
        $w->line('Ok(this)');
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

    private function signature(MethodModel $m, bool $with_self): string
    {
        $params = [];
        foreach ($m->storage->params as $i => $p) {
            $t = $m->param_types[$i] ?? RustType::mixed();
            $params[] = 'mut ' . Names::var($p->name) . ': ' . ($p->by_ref ? '&mut ' : '') . $t->toRust();
        }
        $self = $with_self ? '&self' . ($params ? ', ' : '') : '';
        return '(' . $self . implode(', ', $params) . ') -> Result<' . $m->return_type->toRust() . ', Throw>';
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
            if ($cls->isLeaf()) {
                $w->line('pub fn ' . $rn . $sig . ' {');
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
        $up = $this->casts->convert($this->wrapOwn($cls, 'self.clone()'), RustType::class($cls->fqcn), RustType::class($declaring->fqcn));
        $impl = $declaring->isLeaf() ? $rn : $rn . '__impl';
        $w->line('pub fn ' . $rn . $sig . ' { ' . $up . '.' . $impl . '(' . $this->argNames($m) . ') }');
    }

    private function emitMethodOnEnum(ClassModel $cls, MethodModel $m, Writer $w): void
    {
        $rn = $m->rustName();
        $h = $cls->handle();
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
            if ($m->uses_lsb && !$m->isAbstract()) {
                // `static::m()` from an instance: dispatch on the runtime class
                $arms = array_map(function (ClassModel $c) use ($cls, $rn, $m): string {
                    $cm = $this->program->findMethod($c, $m->lc()) ?? $m;
                    // private static methods are never overridden: `static::` still resolves to the declaring class
                    $target = $m->isPrivate() ? $m->declaring : $c;
                    $call = $target->path() . '::' . $rn . '(' . $this->argNames($m) . ')?';
                    return $cls->handle() . '::' . $c->variant() . '(_) => Ok(' . $this->casts->convert($call, $cm->return_type, $m->return_type) . ')';
                }, $cls->concrete);
                $w->line('pub fn ' . $rn . '__static' . $this->signature($m, true) . ' { match self { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => unreachable!() } }');
            }
            return;
        }
        // virtual dispatch
        $arms = [];
        foreach ($cls->concrete as $c) {
            $cm = $this->program->findMethod($c, $m->lc());
            if ($cm === null) {
                $arms[] = $h . '::' . $c->variant() . '(_) => unreachable!("abstract method ' . $rn . '")';
                continue;
            }
            $call = '__h.' . $rn . '(' . $this->convertedArgs($m, $cm) . ')?';
            $arms[] = $h . '::' . $c->variant() . '(__h) => Ok(' . $this->casts->convert($call, $cm->return_type, $m->return_type) . ')';
        }
        $w->line('pub fn ' . $rn . $this->signature($m, true) . ' { match self { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => unreachable!() } }');
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
        if ($m->isStatic()) {
            $b->this_type = null;
        }
        $params = [];
        foreach ($m->storage->params as $i => $p) {
            $params[$p->name] = $m->param_types[$i] ?? RustType::mixed();
            if ($p->by_ref) {
                $b->byref[$p->name] = true;
            }
        }
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
        $init = $f->default !== null ? $body->constExpr($f->default, $f->type) : $this->casts->defaultOf($f->type);
        $w->line('pub fn st_' . $rn . '_cell() -> &\'static std::thread::LocalKey<RefCell<' . $t . '>> { thread_local! { static CELL: RefCell<' . $t . '> = RefCell::new(' . $init . '); } &CELL }');
        $w->line('pub fn st_' . $rn . '() -> ' . $t . ' { Self::st_' . $rn . '_cell().with(|c| c.borrow().clone()) }');
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

    private function emitPhpObject(ClassModel $cls, Writer $w): void
    {
        $own = $cls->ownHandle();
        $w->open('impl php_rt::PhpObject for ' . $own . ' {');
        $w->line('fn class_name(&self) -> &\'static str { ' . Names::rustStringLiteral($cls->fqcn) . ' }');
        $w->line('fn class_ancestors(&self) -> &\'static [&\'static str] { ' . $this->ancestorsLiteral($cls) . ' }');
        $w->line('fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }');
        $w->line('fn as_any(&self) -> &dyn std::any::Any { self }');
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
        $sets = [];
        foreach ($cls->fields as $f) {
            $sets[] = Names::rustStringLiteral($f->name) . ' => { self.set_' . $f->acc() . '(' . $this->casts->convert('value', RustType::mixed(), $f->type) . '); true }';
        }
        $w->line('fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { ' . implode(', ', $sets) . ($sets ? ', ' : '') . '_ => false } }');
        $gets = [];
        foreach ($cls->fields as $f) {
            $get = $f->isLate() ? 'self.' . $f->acc() . '_opt()' : 'Some(self.' . $f->acc() . '_get())';
            $gets[] = Names::rustStringLiteral($f->name) . ' => ' . $get . '.map(|v| ' . $this->casts->convert('v', $f->type, RustType::mixed()) . ')';
        }
        $w->line('fn get_prop(&self, name: &str) -> Option<Mixed> { match name { ' . implode(', ', $gets) . ($gets ? ', ' : '') . '_ => None } }');
        $ts = $this->program->findMethod($cls, '__tostring');
        if ($ts !== null) {
            $w->line('fn php_to_string(&self) -> Option<Str> { self.' . $ts->rustName() . '().ok() }');
        }
        $w->line('fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { ' . $this->callMethodArms($cls) . '_ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", ' . Names::rustStringLiteral($cls->fqcn) . ', name)))) } }');
        $w->close();
        $w->line('impl ' . $own . ' { pub fn to_php_string(&self) -> Result<Str, Throw> { ' . ($ts !== null ? 'self.' . $ts->rustName() . '()' : 'Err(Throw::error(Str::from_static(' . Names::rustStringLiteral('Object of class ' . $cls->fqcn . ' could not be converted to string') . ')))') . ' } }');
        $clone = $this->program->findMethod($cls, '__clone');
        $clone_call = '';
        if ($clone !== null) {
            $recv = $clone->declaring === $cls || !$clone->isPrivate()
                ? 'c'
                : $this->casts->convert('c.clone()', RustType::class($cls->fqcn), RustType::class($clone->declaring->fqcn));
            $clone_call = 'let _ = ' . $recv . '.' . $clone->rustName() . '(); ';
        }
        $w->line('impl php_rt::PhpClone for ' . $own . ' { fn php_clone(&self) -> Self { let c = ' . $own . '(Rc::new(RefCell::new(self.0.borrow().clone()))); ' . $clone_call . 'c } }');
        $w->line('impl Clone for ' . $cls->objStruct() . ' { fn clone(&self) -> Self { ' . $cls->objStruct() . ' { ' . implode(', ', array_map(fn(FieldModel $f) => $f->rustName() . ': self.' . $f->rustName() . '.clone()', $cls->fields)) . ' } } }');
    }

    /** Match arms (lowercase method name => dynamic invocation) for `PhpObject::call_method`. */
    private function callMethodArms(ClassModel $cls, bool $static_only = false): string
    {
        $arms = '';
        foreach ($cls->methods as $m) {
            if ($m->isAbstract() && !$cls->isInterface()) {
                continue;
            }
            if ($static_only && !$m->isStatic()) {
                continue;
            }
            if ($m->isPrivate() && $m->declaring !== $cls) {
                continue;
            }
            if ($m->storage->visibility !== \Psalm\Internal\Analyzer\ClassLikeAnalyzer::VISIBILITY_PUBLIC) {
                // dynamic calls come from outside the class: only public methods are reachable
                continue;
            }
            if ($m->isPrivate() && $m->declaring !== $cls) {
                continue;
            }
            if ($static_only && $m->declaring !== $cls) {
                // inherited statics are dispatched by the declaring class (see the `_` arm)
                continue;
            }
            $params = [];
            $ok = true;
            foreach ($m->storage->params as $i => $p) {
                if ($p->by_ref || $p->is_variadic) {
                    $ok = false;
                    break;
                }
                $pt = $m->param_types[$i] ?? RustType::mixed();
                if (Casts::isLocal($pt)) {
                    $this->casts->needMixedTo($pt);
                }
                if ($pt->kind === RustType::MIXED) {
                    $params[] = 'dyn_arg_req::<Mixed>(&args, ' . $i . ')';
                } elseif ($pt->hasDefault() && $pt->kind !== RustType::CLASS_) {
                    $params[] = 'dyn_arg::<' . $pt->toRust() . '>(&args, ' . $i . ')';
                } else {
                    $params[] = 'dyn_arg_req::<' . $pt->toRust() . '>(&args, ' . $i . ')';
                }
            }
            if (!$ok) {
                continue;
            }
            $call = ($m->isStatic() ? $m->declaring->path() . '::' : 'self.') . $m->rustName() . '(' . implode(', ', $params) . ')';
            $arms .= Names::rustStringLiteral($m->lc()) . ' => { let __r = ' . $call . '.map_err(|e| DynError::Obj(' . $this->casts->convert('e', RustType::class('Throwable'), RustType::mixed()) . '))?; Ok(' . $this->casts->convert('__r', $m->return_type, RustType::mixed()) . ') }, ';
        }
        return $arms;
    }

    private function emitEnumHandleImpls(ClassModel $cls, Writer $w): void
    {
        $h = $cls->handle();
        $arms = fn(string $call) => implode(', ', array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(__h) => __h.' . $call, $cls->concrete)) . ($cls->concrete ? ', ' : '') . $h . '::Other__(__m) => php_rt::other_obj(__m).' . $call . ', _ => unreachable!()';
        $w->open('impl php_rt::PhpObject for ' . $h . ' {');
        $w->line('fn class_name(&self) -> &\'static str { match self { ' . $arms('class_name()') . ' } }');
        $w->line('fn class_ancestors(&self) -> &\'static [&\'static str] { match self { ' . $arms('class_ancestors()') . ' } }');
        $w->line('fn obj_id(&self) -> usize { match self { ' . $arms('obj_id()') . ' } }');
        $w->line('fn as_any(&self) -> &dyn std::any::Any { self }');
        $w->line('fn props(&self) -> Vec<(Str, Mixed)> { match self { ' . $arms('props()') . ' } }');
        $w->line('fn set_prop(&self, name: &str, value: Mixed) -> bool { match self { ' . $arms('set_prop(name, value)') . ' } }');
        $w->line('fn get_prop(&self, name: &str) -> Option<Mixed> { match self { ' . $arms('get_prop(name)') . ' } }');
        $w->line('fn php_to_string(&self) -> Option<Str> { match self { ' . $arms('php_to_string()') . ' } }');
        $w->line('fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match self { ' . $arms('call_method(name, args)') . ' } }');
        $w->line('fn public_props(&self) -> Vec<(Str, Mixed)> { match self { ' . $arms('public_props()') . ' } }');
        $w->close();
        $to_string_arms = implode(', ', array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(__h) => __h.to_php_string()', $cls->concrete)) . ($cls->concrete ? ', ' : '') . $h . '::Other__(__m) => Ok(to_str(__m)), _ => unreachable!()';
        $w->line('impl ' . $h . ' { pub fn to_php_string(&self) -> Result<Str, Throw> { match self { ' . $to_string_arms . ' } } }');
        $w->line('impl php_rt::PhpClone for ' . $h . ' { fn php_clone(&self) -> Self { match self { ' . implode(', ', array_map(fn(ClassModel $c) => $h . '::' . $c->variant() . '(__h) => ' . $h . '::' . $c->variant() . '(__h.php_clone())', $cls->concrete)) . ($cls->concrete ? ', ' : '') . '' . $h . '::Other__(__m) => ' . $h . '::Other__(__m.clone()), _ => unreachable!() } } }');
    }

    private function emitFactory(ClassModel $cls, Writer $w): void
    {
        $h = $cls->handle();
        $ctor = $this->program->findMethod($cls, '__construct');
        $params = [];
        $names = [];
        if ($ctor !== null) {
            foreach ($ctor->storage->params as $i => $p) {
                $params[] = Names::var($p->name) . ': ' . ($ctor->param_types[$i] ?? RustType::mixed())->toRust();
                $names[] = Names::var($p->name);
            }
        }
        $arms = [];
        foreach ($cls->concrete as $c) {
            $cc = $this->program->findMethod($c, '__construct');
            $args = [];
            if ($cc !== null) {
                foreach ($cc->storage->params as $i => $p) {
                    if (isset($names[$i]) && $ctor !== null) {
                        $args[] = $this->casts->convert($names[$i], $ctor->param_types[$i] ?? RustType::mixed(), $cc->param_types[$i] ?? RustType::mixed());
                    } else {
                        $args[] = $this->casts->defaultOf($cc->param_types[$i] ?? RustType::mixed());
                    }
                }
            }
            $arms[] = Names::byteStrLiteral(strtolower($c->fqcn)) . ' => Ok(' . $this->casts->convert($c->path() . '::new(' . implode(', ', $args) . ')?', RustType::class($c->fqcn), RustType::class($cls->fqcn)) . ')';
        }
        $w->line('impl ' . $h . ' { pub fn new_by_name(__name: &Str, ' . implode(', ', $params) . ') -> Result<' . $h . ', Throw> { match __name.to_lowercase().as_bytes().strip_prefix(b"\\\\").unwrap_or(__name.to_lowercase().as_bytes()) { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => Err(Throw::error(cat!(Str::from_static("Class not found: "), __name.clone()))) } } }');
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
            $w->line('pub fn from_value(v: ' . $backing->toRust() . ') -> Result<' . $h . ', Throw> { Self::try_from_value(v.clone()).ok_or_else(|| Throw::value_error(cat!(to_str(&v), Str::from_static(' . Names::rustStringLiteral(' is not a valid backing value for enum ' . $cls->fqcn) . ')))) }');
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
        $w->line('impl ' . $h . ' { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static(' . Names::rustStringLiteral('Object of class ' . $cls->fqcn . ' could not be converted to string') . '))) } }');
        $w->line('impl ' . $h . ' { pub fn new_same_class(&self) -> Result<Self, Throw> { Err(Throw::error(Str::from_static(' . Names::rustStringLiteral('Cannot instantiate enum ' . $cls->fqcn) . '))) } }');
    }
}
