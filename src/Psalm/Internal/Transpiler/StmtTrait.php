<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node;
use PhpParser\Node\Expr;
use PhpParser\Node\Name;
use PhpParser\Node\Stmt;

use function count;
use function implode;
use function is_string;

/**
 * Statement emission.
 *
 * @internal
 */
trait StmtTrait
{
    public function stmt(Stmt $s): void
    {
        $w = $this->w;
        if ($s instanceof Stmt\Expression) {
            $this->exprStmt($s->expr);
            return;
        }
        if ($s instanceof Stmt\Return_) {
            $this->returnStmt($s);
            return;
        }
        if ($s instanceof Stmt\If_) {
            $this->ifStmt($s);
            return;
        }
        if ($s instanceof Stmt\Echo_) {
            foreach ($s->exprs as $e) {
                $w->line('echo(' . $this->exprTo($e, RustType::str()) . '.as_bytes());');
            }
            return;
        }
        if ($s instanceof Stmt\Foreach_) {
            $this->foreachStmt($s);
            return;
        }
        if ($s instanceof Stmt\While_) {
            $label = $this->newLabel('l');
            $w->open($label . ': loop {');
            $w->line('if !' . $this->truthy($s->cond) . ' { break; }');
            $this->pushLoop($label, $label, false);
            $this->block($s->stmts);
            $this->popLoop();
            $w->close();
            return;
        }
        if ($s instanceof Stmt\Do_) {
            $label = $this->newLabel('l');
            $cont = $this->newLabel('c');
            $w->open($label . ': loop {');
            $w->open($cont . ': {');
            $this->pushLoop($label, $cont, true);
            $this->block($s->stmts);
            $this->popLoop();
            $w->close();
            $w->line('if !' . $this->truthy($s->cond) . ' { break; }');
            $w->close();
            return;
        }
        if ($s instanceof Stmt\For_) {
            $this->forStmt($s);
            return;
        }
        if ($s instanceof Stmt\Switch_) {
            $this->switchStmt($s);
            return;
        }
        if ($s instanceof Stmt\Break_) {
            $n = $s->num instanceof Node\Scalar\Int_ ? $s->num->value : 1;
            $w->line($this->jump(true, $n) . ';');
            return;
        }
        if ($s instanceof Stmt\Continue_) {
            $n = $s->num instanceof Node\Scalar\Int_ ? $s->num->value : 1;
            $w->line($this->jump(false, $n) . ';');
            return;
        }
        if ($s instanceof Stmt\TryCatch) {
            $this->tryStmt($s);
            return;
        }
        if ($s instanceof Stmt\Throw_) {
            $w->line($this->throwCode($s->expr) . ';');
            return;
        }
        if ($s instanceof Stmt\Unset_) {
            foreach ($s->vars as $v) {
                $this->unsetStmt($v);
            }
            return;
        }
        if ($s instanceof Stmt\Nop || $s instanceof Stmt\Declare_ || $s instanceof Stmt\Use_ || $s instanceof Stmt\InlineHTML
            || $s instanceof Stmt\GroupUse
        ) {
            return;
        }
        if ($s instanceof Stmt\Block) {
            $this->block($s->stmts);
            return;
        }
        if ($s instanceof Stmt\Static_) {
            foreach ($s->vars as $var) {
                $name = is_string($var->var->name) ? $var->var->name : '';
                $this->warn('static variable $' . $name, $s);
                if ($var->default !== null) {
                    $w->line($this->assignTo($var->var, $this->expr($var->default)));
                }
            }
            return;
        }
        if ($s instanceof Stmt\Global_) {
            return; // declared as runtime-backed reference variables (see BodyEmitter::scanReferences)
        }
        if ($s instanceof Stmt\Function_ || $s instanceof Stmt\Class_ || $s instanceof Stmt\Interface_ || $s instanceof Stmt\Trait_) {
            // hoisted declarations are emitted elsewhere
            return;
        }
        if ($s instanceof Stmt\Const_) {
            return;
        }
        $this->warn('unsupported statement ' . $s->getType(), $s);
        $w->line('unreachable!("unsupported statement ' . $s->getType() . '");');
    }

    /** @param list<Stmt> $stmts */
    public function block(array $stmts): void
    {
        foreach ($stmts as $s) {
            $this->stmt($s);
        }
    }

    private function exprStmt(Expr $e): void
    {
        $w = $this->w;
        if ($e instanceof Expr\Assign) {
            $w->line($this->assignStmt($e));
            return;
        }
        if ($e instanceof Expr\AssignOp) {
            $code = $this->assignOpStmt($e);
            if ($code !== '') {
                $w->line($code);
            }
            return;
        }
        if ($e instanceof Expr\AssignRef) {
            $w->line($this->assignRefStmt($e));
            return;
        }
        if ($e instanceof Expr\PreInc || $e instanceof Expr\PostInc || $e instanceof Expr\PreDec || $e instanceof Expr\PostDec) {
            $v = $this->expr($e);
            $w->line('let _ = ' . $v->code . ';');
            return;
        }
        if ($e instanceof Expr\Throw_) {
            $w->line($this->throwCode($e->expr) . ';');
            return;
        }
        if ($e instanceof Expr\Exit_) {
            $v = $this->expr($e);
            $w->line($v->code . ';');
            return;
        }
        if ($e instanceof Expr\Yield_ || $e instanceof Expr\YieldFrom) {
            $v = $this->expr($e);
            $w->line('let _ = ' . $v->code . ';');
            return;
        }
        $v = $this->expr($e);
        if ($v->type->kind === RustType::UNIT) {
            $w->line($v->code . ';');
        } elseif ($v->type->kind === RustType::NEVER) {
            $w->line($v->code . ';');
        } else {
            // a discarded value: annotated with its type for readability, except a Mixed one (no Mixed text
            // for a value nothing reads)
            if ($v->type->hasGeneric()) {
                // the callee's generic result: typed by Psalm's view of the call (else discarded as unit)
                $inf = $this->inferred($e);
                $ann = $inf !== null && !$inf->containsMixed() && !$inf->hasGeneric() ? $inf : RustType::unit();
                $w->line('let _: ' . $ann->toRust() . ' = ' . $v->code . ';');
            } else {
                $w->line($v->type->containsMixed() ? 'let _ = ' . $v->code . ';' : 'let _: ' . $v->type->toRust() . ' = ' . $v->code . ';');
            }
        }
    }

    private function returnStmt(Stmt\Return_ $s): void
    {
        $w = $this->w;
        if ($this->is_generator) {
            if ($s->expr !== null) {
                $w->line('let _ = ' . $this->expr($s->expr)->code . ';');
            }
            $w->line($this->returnCode('Generator::from_pairs(std::mem::take(&mut __gen))') . ';');
            return;
        }
        if ($s->expr === null) {
            $w->line($this->returnCode($this->ret_type->kind === RustType::UNIT ? '()' : $this->casts->convert('()', RustType::unit(), $this->ret_type)) . ';');
            return;
        }
        if ($this->ret_type->kind === RustType::UNIT) {
            $v = $this->expr($s->expr);
            $w->line('let _ = ' . $v->code . ';');
            $w->line($this->returnCode('()') . ';');
            return;
        }
        if ($this->ret_type->kind === RustType::NEVER) {
            $v = $this->expr($s->expr);
            $w->line('let _ = ' . $v->code . ';');
            $w->line('unreachable!();');
            return;
        }
        // Owned/borrowed (axis 5): `return $x;` is the last use of $x on this (diverging) path, so move out of
        // the local instead of cloning it — Rust permits the move even when other, non-returning paths use $x.
        if ($s->expr instanceof Expr\Variable && is_string($s->expr->name)
            && ($moved = $this->moveVar($s->expr->name)) !== null
        ) {
            $w->line($this->returnCode($this->casts->convert($moved->code, $moved->type, $this->ret_type)) . ';');
            return;
        }
        $w->line($this->returnCode($this->exprTo($s->expr, $this->ret_type)) . ';');
    }

    private function ifStmt(Stmt\If_ $s): void
    {
        $w = $this->w;
        $w->open('if ' . $this->truthy($s->cond) . ' {');
        $this->block($s->stmts);
        foreach ($s->elseifs as $elseif) {
            $w->dedent();
            $w->line('} else if ' . $this->truthy($elseif->cond) . ' {');
            $w->indent();
            $this->block($elseif->stmts);
        }
        if ($s->else !== null) {
            $w->dedent();
            $w->line('} else {');
            $w->indent();
            $this->block($s->else->stmts);
        }
        $w->close();
    }

    private function forStmt(Stmt\For_ $s): void
    {
        $w = $this->w;
        $w->open('{');
        foreach ($s->init as $e) {
            $this->exprStmt($e);
        }
        $label = $this->newLabel('l');
        $cont = $this->newLabel('c');
        $w->open($label . ': loop {');
        if ($s->cond !== []) {
            $conds = [];
            foreach ($s->cond as $i => $c) {
                if ($i < count($s->cond) - 1) {
                    $this->exprStmt($c);
                } else {
                    $conds[] = $this->truthy($c);
                }
            }
            $w->line('if !(' . implode(' && ', $conds) . ') { break; }');
        }
        $w->open($cont . ': {');
        $this->pushLoop($label, $cont, true);
        $this->block($s->stmts);
        $this->popLoop();
        $w->close();
        foreach ($s->loop as $e) {
            $this->exprStmt($e);
        }
        $w->close();
        $w->close();
    }

    private function foreachStmt(Stmt\Foreach_ $s): void
    {
        $w = $this->w;
        // the subject of foreach is a receiver position: a Traversable object needs the class Psalm
        // narrowed it to (the one that declares getIterator/current)
        $subject = $this->receiver($s->expr);
        $st = $subject->type;
        if ($st->kind === RustType::OPTION) {
            $subject = new Val($subject->code . '.unwrap_or_default()', $st->inner());
            $st = $st->inner();
        }
        $w_pre = null; // a statement binding the iterated container before the loop
        // a container whose elements are never is always empty: the body never runs
        if ($st->isEmptyIterable()) {
            $w->line('let _ = ' . $subject->code . ';');
            return;
        }
        $label = $this->newLabel('l');
        $kv = $this->tmp('__kv');
        $key_var = $s->keyVar;
        $val_var = $s->valueVar;
        if ($s->byRef) {
            $this->foreachByRef($s, $subject, $label, $kv);
            return;
        }

        $key_t = RustType::int();
        $val_t = RustType::mixed();
        $iter = null;
        $by_ref = false; // the loop yields `&K`/`&V` into the container (no copy of the entries)
        switch ($st->kind) {
            case RustType::LIST:
            case RustType::MAP:
                // Iterate the container IN PLACE: `into_iter()` on a shared Rc container copies every entry into a
                // fresh Vec first. A borrowable subject (a local the body never writes, a field of an Rc<T> class)
                // is iterated directly; anything else is snapshotted into a local handle (one Rc clone, which is
                // also what makes the body's writes to the original copy-on-write, as PHP's foreach-over-a-copy).
                $by_ref = true;
                $src = $this->borrowableLoopSubject($s, $subject);
                if ($src === null) {
                    $src = $this->tmp('__s');
                    $w_pre = 'let ' . $src . ' = ' . $subject->code . ';';
                }
                if ($st->kind === RustType::LIST) {
                    $val_t = $st->inner();
                    $iter = $src . '.iter().enumerate().map(|(__i, __v)| (__i as i64, __v))';
                } else {
                    [$key_t, $val_t] = $st->params;
                    $iter = $src . '.iter()';
                }
                break;
            case RustType::TUPLE:
                $lt = RustType::list($this->types()->combine($st->params));
                $val_t = $lt->inner();
                $iter = $this->casts->convert($subject->code, $st, $lt) . '.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v))';
                break;
            case RustType::SHAPE:
                $mt = RustType::map(RustType::arrayKey(), $this->shapeValueType($st));
                [$key_t, $val_t] = $mt->params;
                $iter = $this->casts->convert($subject->code, $st, $mt) . '.into_iter()';
                break;
            case RustType::MIXED:
                $key_t = RustType::arrayKey();
                $iter = 'mixed_iter(' . $subject->code . ').unwrap_or_else(|__e| __throw_rt(__e))';
                break;
            case RustType::RT_GENERIC:
                if (in_array($st->name, ['Generator', 'ArrayObject', 'ArrayIterator', 'SplObjectStorage', 'PhpIterator', 'IteratorAggregate', 'Traversable', 'WeakMap'], true)) {
                    [$key_t, $val_t] = [$st->params[0] ?? RustType::mixed(), $st->params[1] ?? RustType::mixed()];
                    if ($st->name === 'SplObjectStorage') {
                        // iterating SplObjectStorage yields objects as values
                        $val_t = $st->params[0];
                        $key_t = RustType::int();
                        $iter = $subject->code . '.iter_objects()';
                    } else {
                        $iter = $subject->code . '.into_pairs()';
                    }
                }
                break;
            case RustType::CLASS_:
                $cls = $this->program->classOf($st);
                if ($cls !== null) {
                    [$iter, $key_t, $val_t] = $this->objectIterator($cls, $subject->code, $s);
                }
                break;
            case RustType::UNION:
                $mt = RustType::map(RustType::arrayKey(), RustType::mixed());
                foreach ($st->params as $m) {
                    if ($m->kind === RustType::MAP || $m->kind === RustType::LIST) {
                        $mt = $m->kind === RustType::LIST ? RustType::map(RustType::int(), $m->inner()) : $m;
                    }
                }
                [$key_t, $val_t] = $mt->params;
                $iter = $this->casts->convert($subject->code, $st, $mt) . '.into_iter()';
                break;
        }
        if ($iter === null) {
            $this->warn('foreach over ' . $st->toRust(), $s);
            $w->line('let _ = ' . $subject->code . ';');
            $iter = 'std::iter::empty::<(i64, Mixed)>()';
            $key_t = RustType::int();
            $val_t = RustType::mixed();
        }
        if ($w_pre !== null) {
            $w->line($w_pre);
        }
        $w->open($label . ': for ' . $kv . ' in ' . $iter . ' {');
        if ($key_var !== null) {
            $key_val = $by_ref && $st->kind === RustType::MAP
                ? new Val($kv . '.0.clone()', $key_t, '(*' . $kv . '.0)')
                : new Val($kv . '.0', $key_t);
            $w->line($this->assignTo($key_var, $key_val));
        }
        if (!($val_var instanceof Expr\Variable && $val_var->name === '_')) {
            // `$_` is the conventional discard: no binding (it never takes a type)
            $val_val = $by_ref ? new Val($kv . '.1.clone()', $val_t, '(*' . $kv . '.1)') : new Val($kv . '.1', $val_t);
            $w->line($this->assignTo($val_var, $val_val));
        }
        $this->pushLoop($label, $label, false);
        $this->block($s->stmts);
        $this->popLoop();
        $w->close();
    }

    /**
     * The place to iterate a foreach subject in without copying it, or null when it must be snapshotted: a plain
     * local the body never writes (nor captures into a closure, nor passes to a call that might take it by
     * reference), or a borrowable non-local place (a field of an Rc<T> class) outside a `&mut self` method.
     */
    private function borrowableLoopSubject(Stmt\Foreach_ $s, Val $subject): ?string
    {
        if ($subject->place === null) {
            return null;
        }
        $e = $s->expr;
        if ($e instanceof Expr\Variable && is_string($e->name)) {
            $name = $e->name;
            if ($name === 'this' || !empty($this->move_captured[$name]) || !empty($this->cells[$name])
                || !empty($this->refvars[$name]) || !empty($this->globals[$name])
            ) {
                return null;
            }
            return $this->varWrittenIn($name, $s->stmts, $s->keyVar, $s->valueVar) ? null : $subject->place;
        }
        // a field place: the borrow of `self` it holds across the loop conflicts with a `&mut self` receiver
        if ($this->class !== null && $this->record->method_name !== null
            && $this->class->isImmutableCtorMethod(strtolower($this->record->method_name))
        ) {
            return null;
        }
        return $subject->place;
    }

    /**
     * Whether `$name` may be written by the statements: assigned (directly, destructured, as a loop variable, by
     * `unset`, through an element/property write rooted at it), referenced, captured by a closure, or passed
     * directly to a call (which may take it by reference).
     *
     * @param list<Stmt> $stmts
     */
    private function varWrittenIn(string $name, array $stmts, ?Expr ...$extra): bool
    {
        $finder = new \PhpParser\NodeFinder();
        $nodes = array_merge($stmts, array_values(array_filter($extra)));
        $root = static function (Expr $t): ?Expr {
            while ($t instanceof Expr\ArrayDimFetch || $t instanceof Expr\PropertyFetch || $t instanceof Expr\NullsafePropertyFetch) {
                $t = $t->var;
            }
            return $t;
        };
        $names = static function (Expr $t) use (&$names, $root): array {
            if ($t instanceof Expr\List_ || $t instanceof Expr\Array_) {
                $out = [];
                foreach ($t->items as $item) {
                    if ($item !== null) {
                        $out = array_merge($out, $names($item->value));
                    }
                }
                return $out;
            }
            $r = $root($t);
            return $r instanceof Expr\Variable && is_string($r->name) ? [$r->name] : [];
        };
        $writes = [];
        foreach ($finder->find($nodes, static fn(\PhpParser\Node $n) => $n instanceof Expr\Assign || $n instanceof Expr\AssignOp
            || $n instanceof Expr\AssignRef || $n instanceof Expr\PreInc || $n instanceof Expr\PreDec
            || $n instanceof Expr\PostInc || $n instanceof Expr\PostDec) as $n
        ) {
            $writes = array_merge($writes, $names($n->var));
            if ($n instanceof Expr\AssignRef) {
                $writes = array_merge($writes, $names($n->expr));
            }
        }
        foreach ($finder->findInstanceOf($nodes, Stmt\Unset_::class) as $n) {
            foreach ($n->vars as $v) {
                $writes = array_merge($writes, $names($v));
            }
        }
        foreach ($finder->findInstanceOf($nodes, Stmt\Foreach_::class) as $n) {
            $writes = array_merge($writes, $names($n->valueVar), $n->keyVar !== null ? $names($n->keyVar) : []);
            if ($n->byRef) {
                $writes = array_merge($writes, $names($n->expr));
            }
        }
        foreach ($finder->findInstanceOf($nodes, \PhpParser\Node\Arg::class) as $a) {
            if ($a->value instanceof Expr\Variable && is_string($a->value->name)) {
                $writes[] = $a->value->name;
            }
        }
        foreach ($finder->find($nodes, static fn(\PhpParser\Node $n) => $n instanceof Expr\Closure || $n instanceof Expr\ArrowFunction) as $c) {
            foreach ($finder->findInstanceOf([$c], Expr\Variable::class) as $v) {
                if (is_string($v->name)) {
                    $writes[] = $v->name;
                }
            }
        }
        foreach ($finder->findInstanceOf($nodes, Stmt\Global_::class) as $g) {
            foreach ($g->vars as $v) {
                $writes = array_merge($writes, $names($v));
            }
        }
        foreach ($finder->findInstanceOf($nodes, Stmt\Static_::class) as $st) {
            foreach ($st->vars as $v) {
                $writes = array_merge($writes, $names($v->var));
            }
        }
        // extra targets passed by the caller (the loop's own key/value variables)
        foreach (array_filter($extra) as $t) {
            $writes = array_merge($writes, $names($t));
        }
        return in_array($name, $writes, true);
    }

    /** `foreach ($mixed as $k => &$v)`: keys from the array view, values written back through `mixed_set`. */
    private function foreachByRefMixed(Stmt\Foreach_ $s, Place $place, string $label, string $kv): void
    {
        $w = $this->w;
        $keys = $this->tmp('__keys');
        $w->line('let ' . $keys . ': Vec<ArrayKey> = cast::<Map<ArrayKey, Mixed>>(' . $place->read() . ').keys().cloned().collect();');
        $val_place = $this->place($s->valueVar);
        $elem_read = 'mixed_get(&' . $place->read() . ', &' . $kv . ').unwrap_or_default()';
        $store_back = $place->modify(fn(string $p) => 'mixed_set(&mut ' . $p . ', Some(' . $kv . '.clone()), ' . $this->casts->convert($val_place->read(), $val_place->type, RustType::mixed()) . ');');
        $w->open($label . ': for ' . $kv . ' in ' . $keys . ' {');
        if ($s->keyVar !== null) {
            $w->line($this->assignTo($s->keyVar, new Val($kv . '.clone()', RustType::arrayKey())));
        }
        $w->line($val_place->write($this->casts->convert($elem_read, RustType::mixed(), $val_place->type)));
        $this->pushLoop($label, $label, false, $store_back);
        $this->block($s->stmts);
        $this->popLoop();
        $w->line($store_back);
        $w->close();
    }

    /** `foreach ($arr as $k => &$v)`: iterate keys and write the value variable back into the container. */
    private function foreachByRef(Stmt\Foreach_ $s, Val $subject, string $label, string $kv): void
    {
        $w = $this->w;
        $st = $subject->type;
        if (!($s->expr instanceof Expr\Variable || $s->expr instanceof Expr\PropertyFetch || $s->expr instanceof Expr\StaticPropertyFetch || $s->expr instanceof Expr\ArrayDimFetch)
            || ($st->kind !== RustType::LIST && $st->kind !== RustType::MAP)
        ) {
            $this->warn('foreach by reference over ' . $st->toRust(), $s);
            $w->line('let _ = ' . $subject->code . ';');
            return;
        }
        $place = $this->place($s->expr);
        if ($place->type->kind === RustType::OPTION) {
            // a nullable container Psalm knows to be set here
            $p = $place;
            $place = new Place(
                $p->type->inner(),
                fn() => $p->read() . '.unwrap_or_default()',
                fn(string $v) => $p->write('Some(' . $v . ')'),
                $p->hasMut() ? fn() => '(*' . $p->mut() . ($p->type->inner()->hasDefault() ? '.get_or_insert_with(Default::default))' : '.as_mut().expect("null container"))') : null,
                fn(string $st) => $p->wrap($st),
            );
        }
        if ($place->type->kind === RustType::MIXED) {
            $this->foreachByRefMixed($s, $place, $label, $kv);
            return;
        }
        // the container's declared type decides element types (the subject may be narrowed)
        $keys_src = $subject->code;
        if ($place->type->kind === RustType::LIST || $place->type->kind === RustType::MAP) {
            $st = $place->type;
            $keys_src = $place->read();
        }
        $is_list = $st->kind === RustType::LIST;
        $kt = $is_list ? RustType::int() : $st->params[0];
        $vt = $is_list ? $st->inner() : $st->params[1];
        $keys = $this->tmp('__keys');
        if ($is_list) {
            $w->line('let ' . $keys . ': Vec<i64> = (0..' . $keys_src . '.len() as i64).collect();');
        } else {
            $w->line('let ' . $keys . ': Vec<' . $kt->toRust() . '> = ' . $keys_src . '.keys().cloned().collect();');
        }
        // the value variable is a plain local; each iteration copies in and writes back
        $val_place = $this->place($s->valueVar);
        $elem = $this->tmp('__el');
        $elem_get = $is_list ? $place->read() . '.get(' . $kv . ').cloned()' : $place->read() . '.get(&' . $kv . ').cloned()';
        // a reference to an element the body has removed writes nowhere, as PHP's does
        $store_back = $place->modify(fn(string $p) => $is_list
            ? $p . '.replace(' . $kv . ', ' . $this->casts->convert($val_place->read(), $val_place->type, $vt) . ');'
            : $p . '.replace(' . $kv . '.clone(), ' . $this->casts->convert($val_place->read(), $val_place->type, $vt) . ');');
        $w->open($label . ': for ' . $kv . ' in ' . $keys . ' {');
        // an element removed by an earlier iteration is no longer iterated, as in PHP
        $w->line('let ' . $elem . ' = match ' . $elem_get . ' { Some(__v) => __v, None => continue ' . $label . ' };');
        if ($s->keyVar !== null) {
            $w->line($this->assignTo($s->keyVar, new Val($kv . '.clone()', $kt)));
        }
        $w->line($val_place->write($this->casts->convert($elem, $vt, $val_place->type)));
        $this->pushLoop($label, $label, false, $store_back);
        $this->block($s->stmts);
        $this->popLoop();
        $w->line($store_back);
        $w->close();
    }

    /**
     * Iterator expression for foreach over an object (Iterator / IteratorAggregate / plain props).
     *
     * @return array{string, RustType, RustType}
     */
    private function objectIterator(ClassModel $cls, string $code, Stmt\Foreach_ $s): array
    {
        $pairs = $this->casts->objectPairs($cls, $code);
        if ($pairs !== null) {
            return $pairs;
        }
        // plain object: iterate its properties (only the public ones when seen from outside the class)
        $inside = $this->class !== null && $this->class->isSubclassOf($cls);
        return ['{ let __o = ' . $code . '; php_rt::PhpObject::' . ($inside ? 'props' : 'public_props') . '(&__o).into_iter().map(|(k, v)| (ArrayKey::from_str_val(k), v)) }', RustType::arrayKey(), RustType::mixed()];
    }

    private function switchStmt(Stmt\Switch_ $s): void
    {
        $w = $this->w;
        $label = $this->newLabel('sw');
        $subject = $this->expr($s->cond);
        $tmp = $this->tmp('__sw');
        $w->open($label . ': {');
        $w->line('let ' . $tmp . ' = ' . $subject->code . ';');
        $conds = [];
        $default_idx = null;
        foreach ($s->cases as $i => $case) {
            if ($case->cond === null) {
                $default_idx = $i;
                continue;
            }
            $cv = $this->expr($case->cond);
            $ct = $this->commonType($subject->type, $cv->type, true);
            $l = $this->casts->convertVal(new Val($tmp . '.clone()', $subject->type, $tmp), $ct);
            $r = $this->casts->convertVal($cv, $ct);
            $conds[] = [$i, 'loose_eq(' . $l->borrow() . ', ' . $r->borrow() . ')'];
        }
        $idx = $this->tmp('__idx');
        $code = 'let ' . $idx . ': usize = ';
        if ($conds === []) {
            $code .= ($default_idx ?? count($s->cases)) . ';';
        } else {
            $parts = [];
            foreach ($conds as [$i, $c]) {
                $parts[] = 'if ' . $c . ' { ' . $i . ' }';
            }
            $code .= implode(' else ', $parts) . ' else { ' . ($default_idx ?? count($s->cases)) . ' };';
        }
        $w->line($code);
        $this->pushLoop($label, $this->loopAt(1)['continue'] ?? $label, $this->loopAt(1)['is_block_continue'] ?? false);
        // `continue` inside switch behaves like `break` in PHP only for `continue 1` targeting the switch;
        // PHP actually treats `continue` in a switch as `continue` of the enclosing loop, so map it.
        $outer = $this->loopAt(2);
        $this->popLoop();
        $this->pushLoop($label, $outer['continue'] ?? $label, $outer['is_block_continue'] ?? false);
        foreach ($s->cases as $i => $case) {
            $w->open('if ' . $idx . ' <= ' . $i . ' {');
            $this->block($case->stmts);
            $w->close();
        }
        $this->popLoop();
        $w->close();
    }

    /**
     * Axis-8 / panic-based errors: a `throw` of a Resultable (or anywhere-caught) exception emits `return Err(..)`
     * so it propagates to its catch (the intentional, recoverable error path). A `throw` of any other exception —
     * an invariant violation never meant to be recovered from — emits `php_rt::uncaught(..)`, a panic that does not
     * need Result. Both branches are `!`-typed, so this fits statement and expression positions alike. See
     * Program::isResultable / computeResultable.
     */
    private function throwCode(Expr $e): string
    {
        // Panic-based errors: every `throw` unwinds via php_rt::do_throw carrying the exception as the program's
        // typed `Throw` (the Throwable handle enum); a `try` boundary catch_unwinds and matches it. Returns `!`.
        $v = $this->expr($e);
        return 'php_rt::do_throw(' . $this->casts->convert($v->code, $v->type, RustType::class('Throwable')) . ')';
    }

    private function tryStmt(Stmt\TryCatch $s): void
    {
        // Panic-based try/catch: the try body runs inside catch_unwind and returns Flow<T> (return/break/continue
        // inside a try become Flow, since the body is a closure). A PHP `throw` unwinds as a panic; the boundary
        // recovers the exception via php_rt::take_thrown (a genuine Rust panic resumes unwinding). Unmatched catches
        // re-throw. AssertUnwindSafe: the body mutates captured scope locals; their pre-throw values persist (PHP
        // keeps side effects up to the throw).
        $w = $this->w;
        $flow_t = 'Flow<' . $this->ret_type->toRust() . '>';
        $mixed = RustType::mixed();
        $throwable = RustType::class('Throwable');
        $r = $this->tmp('__r');
        $w->line('let ' . $r . ': ' . $flow_t . ' = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> ' . $flow_t . ' {');
        $w->indent();
        $this->enterTry();
        $this->block($s->stmts);
        $this->leaveTry();
        $w->line('#[allow(unreachable_code)] Flow::Normal');
        $w->dedent();
        $w->line('})) {');
        $w->indent();
        $w->line('Ok(__flow) => __flow,');
        $w->line('Err(__payload) => {');
        $w->indent();
        $w->line('let __e: ' . $throwable->toRust() . ' = php_rt::take_thrown::<' . $throwable->toRust() . '>(__payload);');
        $rethrow = 'php_rt::do_throw(__e)';
        if ($s->catches !== []) {
            $w->line('(|| -> ' . $flow_t . ' {');
            $w->indent();
            $this->enterTry();
            $first = true;
            foreach ($s->catches as $catch) {
                $checks = [];
                foreach ($catch->types as $type) {
                    $fqcn = $this->resolveClassName($type);
                    if ($fqcn === null) {
                        continue;
                    }
                    $tt = RustType::class($fqcn);
                    $this->casts->needInstanceOf($throwable, $tt);
                    $checks[] = 'is_instance::<' . $tt->toRust() . '>(&__e)';
                }
                if ($checks === []) {
                    continue;
                }
                $w->open(($first ? 'if ' : 'else if ') . implode(' || ', $checks) . ' {');
                $first = false;
                if ($catch->var !== null && is_string($catch->var->name)) {
                    $vt = $this->varType($catch->var->name);
                    if ($vt->kind === RustType::MIXED || ($vt->kind === RustType::OPTION && $vt->inner()->kind === RustType::MIXED)) {
                        // a catch variable Psalm lost: its static type is the union of the caught classes
                        $caught = [];
                        foreach ($catch->types as $ctn) {
                            $caught[] = $this->types()->map(new \Psalm\Type\Union([new \Psalm\Type\Atomic\TNamedObject($ctn->toString())]));
                        }
                        $this->noteMixedAssign($catch->var->name, $this->program->unionOfRust($caught) ?? $throwable);
                    }
                    $w->line($this->storeVar($catch->var->name, $this->casts->convert('__e.clone()', $throwable, $vt)));
                }
                $this->block($catch->stmts);
                $w->line('#[allow(unreachable_code)] Flow::Normal');
                $w->close();
            }
            $this->leaveTry();
            $w->line(($first ? '' : 'else ') . '{ ' . $rethrow . ' }');
            $w->dedent();
            $w->line('})()');
        } else {
            // try/finally with no catch: run finally (below) then re-throw.
            $w->line($rethrow);
        }
        $w->dedent();
        $w->line('}');
        $w->dedent();
        $w->line('};');
        if ($s->finally !== null) {
            $this->block($s->finally->stmts);
        }
        // dispatch non-local control flow
        $w->open('match ' . $r . ' {');
        $w->line('Flow::Normal => {}');
        $w->line('Flow::Return(__v) => ' . $this->returnCode('__v') . ',');
        $loops = $this->loopDepth();
        if ($loops > 0) {
            for ($n = 1; $n <= $loops; $n++) {
                $w->line('Flow::Break(' . $n . ') => ' . $this->jump(true, $n) . ',');
                $w->line('Flow::Continue(' . $n . ') => ' . $this->jump(false, $n) . ',');
            }
        }
        $w->line('#[allow(unreachable_patterns)] Flow::Break(_) | Flow::Continue(_) => unreachable!(),');
        $w->close();
    }

    private function unsetStmt(Expr $e): void
    {
        $w = $this->w;
        if ($e instanceof Expr\Variable && is_string($e->name)) {
            $t = $this->varType($e->name);
            if ($t->kind === RustType::OPTION) {
                $w->line($this->storeVar($e->name, 'None'));
            } elseif ($t->hasDefault()) {
                $w->line($this->storeVar($e->name, 'Default::default()'));
            }
            return;
        }
        if ($e instanceof Expr\ArrayDimFetch && $e->dim !== null) {
            $parent = $this->place($e->var);
            $pt = $parent->type;
            if ($pt->kind === RustType::OPTION) {
                $pt = $pt->inner();
                $p = $parent;
                $parent = new Place($pt, fn() => $p->read() . '.unwrap_or_default()', fn(string $v) => $p->write('Some(' . $v . ')'), $p->hasMut() ? fn() => '(*' . $p->mut() . '.get_or_insert_with(Default::default))' : null, fn(string $s) => $p->wrap($s));
            }
            if ($pt->kind === RustType::MAP) {
                $w->line($parent->wrap($parent->mut() . '.remove(' . Names::refOf($this->keyExpr($e->dim, $pt->params[0])) . ');'));
                return;
            }
            if ($pt->kind === RustType::LIST) {
                // unsetting from a list: Psalm turns it into an array; keep list semantics for the last element
                $w->line($parent->wrap('list_unset(&mut ' . $parent->mut() . ', ' . $this->exprTo($e->dim, RustType::int()) . ');'));
                return;
            }
            if ($pt->kind === RustType::SHAPE) {
                $k = $this->literalKey($e->dim);
                if ($k !== null && isset($pt->fields[$k]) && $pt->fields[$k][1]) {
                    $w->line($parent->wrap($parent->mut() . '.' . Names::field($k) . ' = None;'));
                    return;
                }
            }
            if ($pt->kind === RustType::MIXED) {
                $w->line($parent->wrap('mixed_unset(&mut ' . $parent->mut() . ', &' . $this->keyExpr($e->dim, RustType::arrayKey()) . ');'));
                return;
            }
            if ($pt->kind === RustType::CLASS_) {
                $cls = $this->program->classOf($pt);
                $m = $cls !== null ? $this->program->findMethod($cls, 'offsetunset') : null;
                if ($m !== null) {
                    $w->line($parent->read() . '.' . $m->rustName() . '(' . $this->exprTo($e->dim, $m->param_types[0] ?? RustType::mixed()) . ');');
                    return;
                }
            }
            if ($pt->kind === RustType::RT_GENERIC && in_array($pt->name, ['ArrayObject', 'ArrayIterator', 'SplObjectStorage', 'WeakMap'], true)) {
                $remover = $pt->name === 'SplObjectStorage' || $pt->name === 'WeakMap' ? 'detach' : 'remove';
                $w->line($parent->read() . '.' . $remover . '(' . $this->exprToVal($e->dim, $pt->params[0])->borrow() . ');');
                return;
            }
            $this->warn('unset on ' . $pt->toRust(), $e);
            return;
        }
        if ($e instanceof Expr\PropertyFetch) {
            $place = $this->place($e);
            if ($place->type->kind === RustType::OPTION) {
                $w->line($place->write('None'));
            } elseif ($place->type->hasDefault()) {
                $w->line($place->write('Default::default()'));
            }
            return;
        }
        $this->warn('unsupported unset', $e);
    }
}
