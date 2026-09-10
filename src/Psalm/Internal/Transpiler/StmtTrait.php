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
            $w->line('return Err(' . $this->throwValue($s->expr) . ');');
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
            $w->line('return Err(' . $this->throwValue($e->expr) . ');');
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
            $w->line('let _: ' . $v->type->toRust() . ' = ' . $v->code . ';');
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
        $subject = $this->expr($s->expr);
        $st = $subject->type;
        if ($st->kind === RustType::OPTION) {
            $subject = new Val($subject->code . '.unwrap_or_default()', $st->inner());
            $st = $st->inner();
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
        switch ($st->kind) {
            case RustType::LIST:
                $val_t = $st->inner();
                $iter = $subject->code . '.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v))';
                break;
            case RustType::MAP:
                [$key_t, $val_t] = $st->params;
                $iter = $subject->code . '.into_iter()';
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
                $iter = 'mixed_iter(' . $subject->code . ')?';
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
        $w->open($label . ': for ' . $kv . ' in ' . $iter . ' {');
        if ($key_var !== null) {
            $w->line($this->assignTo($key_var, new Val($kv . '.0', $key_t)));
        }
        $w->line($this->assignTo($val_var, new Val($kv . '.1', $val_t)));
        $this->pushLoop($label, $label, false);
        $this->block($s->stmts);
        $this->popLoop();
        $w->close();
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
        if ($place->type->kind === RustType::LIST || $place->type->kind === RustType::MAP) {
            $st = $place->type;
        }
        $is_list = $st->kind === RustType::LIST;
        $kt = $is_list ? RustType::int() : $st->params[0];
        $vt = $is_list ? $st->inner() : $st->params[1];
        $keys = $this->tmp('__keys');
        if ($is_list) {
            $w->line('let ' . $keys . ': Vec<i64> = (0..' . $subject->code . '.len() as i64).collect();');
        } else {
            $w->line('let ' . $keys . ': Vec<' . $kt->toRust() . '> = ' . $subject->code . '.keys().cloned().collect();');
        }
        // the value variable is a plain local; each iteration copies in and writes back
        $val_place = $this->place($s->valueVar);
        $elem_read = $is_list ? $place->read() . '.idx(' . $kv . ').clone()' : $place->read() . '.idx(&' . $kv . ').clone()';
        $store_back = $place->modify(fn(string $p) => $is_list
            ? $p . '.set(' . $kv . ', ' . $this->casts->convert($val_place->read(), $val_place->type, $vt) . ');'
            : $p . '.insert(' . $kv . '.clone(), ' . $this->casts->convert($val_place->read(), $val_place->type, $vt) . ');');
        $w->open($label . ': for ' . $kv . ' in ' . $keys . ' {');
        if ($s->keyVar !== null) {
            $w->line($this->assignTo($s->keyVar, new Val($kv . '.clone()', $kt)));
        }
        $w->line($val_place->write($this->casts->convert($elem_read, $vt, $val_place->type)));
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
            $l = $this->casts->convert($tmp . '.clone()', $subject->type, $ct);
            $r = $this->casts->convert($cv->code, $cv->type, $ct);
            $conds[] = [$i, 'loose_eq(&' . $l . ', &' . $r . ')'];
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

    private function tryStmt(Stmt\TryCatch $s): void
    {
        $w = $this->w;
        $flow_t = 'Flow<' . $this->ret_type->toRust() . '>';
        $r = $this->tmp('__r');
        $w->line('let ' . $r . ': Result<' . $flow_t . ', Throw> = (|| -> Result<' . $flow_t . ', Throw> {');
        $w->indent();
        $this->enterTry();
        $this->block($s->stmts);
        $this->leaveTry();
        $w->line('#[allow(unreachable_code)] Ok(Flow::Normal)');
        $w->dedent();
        $w->line('})();');
        if ($s->catches !== []) {
            $w->line('let ' . $r . ' = match ' . $r . ' {');
            $w->indent();
            $w->line('Err(__e) => (|| -> Result<' . $flow_t . ', Throw> {');
            $w->indent();
            $this->enterTry();
            $first = true;
            foreach ($s->catches as $catch) {
                $checks = [];
                $target = null;
                foreach ($catch->types as $type) {
                    $fqcn = $this->resolveClassName($type);
                    if ($fqcn === null) {
                        continue;
                    }
                    $tt = RustType::class($fqcn);
                    $this->casts->needInstanceOf(RustType::class('Throwable'), $tt);
                    $checks[] = 'is_instance::<' . $tt->toRust() . '>(&__e)';
                    $target = $target === null ? $tt : $this->types()->combine([$target, $tt]);
                }
                if ($checks === []) {
                    continue;
                }
                $w->open(($first ? 'if ' : 'else if ') . implode(' || ', $checks) . ' {');
                $first = false;
                if ($catch->var !== null && is_string($catch->var->name)) {
                    $vt = $this->varType($catch->var->name);
                    $w->line($this->storeVar($catch->var->name, $this->casts->convert('__e.clone()', RustType::class('Throwable'), $vt)));
                }
                $this->block($catch->stmts);
                $w->line('#[allow(unreachable_code)] Ok(Flow::Normal)');
                $w->close();
            }
            $w->line(($first ? '' : 'else ') . '{ Err(__e) }');
            $this->leaveTry();
            $w->dedent();
            $w->line('})(),');
            $w->line('__ok => __ok,');
            $w->dedent();
            $w->line('};');
        }
        if ($s->finally !== null) {
            $this->block($s->finally->stmts);
        }
        // dispatch non-local control flow
        $w->open('match ' . $r . ' {');
        $w->line('Ok(Flow::Normal) => {}');
        $w->line('Ok(Flow::Return(__v)) => ' . $this->returnCode('__v') . ',');
        $loops = $this->loopDepth();
        if ($loops > 0) {
            for ($n = 1; $n <= $loops; $n++) {
                $w->line('Ok(Flow::Break(' . $n . ')) => ' . $this->jump(true, $n) . ',');
                $w->line('Ok(Flow::Continue(' . $n . ')) => ' . $this->jump(false, $n) . ',');
            }
            $w->line('Ok(Flow::Break(_)) | Ok(Flow::Continue(_)) => unreachable!(),');
        } else {
            $w->line('Ok(Flow::Break(_)) | Ok(Flow::Continue(_)) => unreachable!(),');
        }
        $w->line('Err(__e) => return Err(__e),');
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
                $w->line($parent->wrap($parent->mut() . '.remove(&' . $this->keyExpr($e->dim, $pt->params[0]) . ');'));
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
                    $w->line($parent->read() . '.' . $m->rustName() . '(' . $this->exprTo($e->dim, $m->param_types[0] ?? RustType::mixed()) . ')?;');
                    return;
                }
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
