<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node;
use PhpParser\Node\Expr;
use PhpParser\Node\Expr\ArrowFunction;
use PhpParser\NodeFinder;
use PhpParser\Node\Expr\Closure;
use PhpParser\Node\Stmt;
use PhpParser\Node\Stmt\ClassMethod;
use PhpParser\Node\Stmt\Function_;
use Psalm\Type\Union;

use function array_key_last;
use function count;
use function implode;
use function in_array;
use function is_string;
use function preg_match;
use function strtolower;

/**
 * Emits the Rust body of one function-like (method, function or closure).
 *
 * @internal
 */
final class BodyEmitter
{
    use ExprTrait;
    use CallTrait;
    use LValueTrait;
    use StmtTrait;

    /**
     * Property name for a dynamic get_prop/set_prop access (object/Mixed receiver, literal name). Warns if the
     * name is not in ClassEmitter::DYN_ACCESS_PROPS — meaning its get_prop/set_prop arm was elided and this
     * access would wrongly resolve to None/false. Returns the Rust string literal for the name.
     */
    private function dynPropName(string $name): string
    {
        \fwrite(\STDERR, "[dyn-used-prop] $name\n");
        if (!isset(ClassEmitter::DYN_ACCESS_PROPS[$name])) {
            \fwrite(\STDERR, "[dyn-prop-MISS] $name — add to ClassEmitter::DYN_ACCESS_PROPS or its get_prop/set_prop arm is elided\n");
        }
        return Names::strLit($name);
    }

    public Writer $w;

    /** The type the surrounding expression expects of the builtin call being emitted (see ExprTrait::exprNatural). */
    public ?RustType $call_expected = null;

    /** @var array<string, RustType> declared Rust type of each local */
    public array $vars = [];

    /** @var array<string, bool> locals stored as Late<T> */
    public array $late = [];

    /** @var array<string, bool> params passed as `&mut T` */
    public array $byref = [];

    /** @var array<string, bool> params received as `&T` (owned/borrowed axis 5: non-escaping read-only) */
    public array $borrow = [];

    /** @var array<string, bool> locals stored as Rc<RefCell<T>> (captured by reference) */
    public array $cells = [];

    /** @var array<string, bool> locals bound by reference (`$x = &...`): stored as PhpRef<T> */
    public array $refvars = [];

    /**
     * Owned/borrowed (axis 5): locals whose value is READ exactly once in the whole body, outside any
     * loop/closure. That single read is necessarily the last use, so it can MOVE out of the local instead
     * of cloning (readVar consults this). Excludes loop/closure-scoped reads (runtime-multi-use / captured).
     * @var array<string, bool>
     */
    public array $single_use = [];

    /**
     * Owned/borrowed (axis 5): locals that appear inside a closure or try block (the latter is emitted as a
     * generated catch_unwind closure). Moving such a local risks move-into-closure, so moveVar refuses them --
     * this blocks BOTH single-use move and return-move. @var array<string, bool>
     */
    public array $move_captured = [];

    /**
     * Active `instanceof` narrowings for the current sub-expression: `$var name => narrowed class type`.
     * Set while emitting the RHS of `$v instanceof X && ...` (and the true branch of `$v instanceof X ? ... : ...`)
     * so a method/property call on `$v` in that sub-expression resolves statically on the subclass instead of
     * falling to the dynamic protocol — Psalm narrows `$v` there but does not record it on the receiver node.
     * @var array<string, RustType>
     */
    public array $narrowings = [];

    /** @var array<string, RustType> parameters the body re-assigns with a wider type: re-declared with the joined type */
    public array $rebound = [];

    /** @var array<string, true> locals already declared by the enclosing code (closure captures) */
    public array $predeclared = [];

    /** @var list<array{break: string, continue: string, try_depth: int, is_block_continue: bool, writeback: string}> */
    private array $loops = [];

    private int $try_depth = 0;

    private int $label_counter = 0;

    private int $tmp_counter = 0;

    public RustType $ret_type;

    /** Axis-8: whether the function being emitted returns Result<T, Throw> (true) or bare T. Set by the caller
     * (ClassEmitter::body / CrateEmitter) from the model's $throws before emitBody(). Controls returnCode() and
     * implicitReturn(). Default true = current (always-Result) behavior. */
    public bool $throws = true;

    public bool $is_generator = false;

    public RustType $gen_key;

    public RustType $gen_val;

    /** Expression that evaluates to the current object handle (`self` in methods, `this` in closures). */
    public string $this_expr = 'self';

    public ?RustType $this_type = null;

    /** class `self`/`parent` refer to (the declaring class of an inherited body emitted for a subclass); defaults to `class` */
    public ?ClassModel $self_class = null;
    /** @var array<string, string> Rust generics of the fn being emitted (PHP template name => Rust name) */
    public array $generics = [];

    /** @var array<string, list<RustType>> static types assigned to locals Psalm types as mixed (first pass) */
    private array $mixed_assigns = [];

    /** @var array<string, RustType> locals retyped from their assignments for the second pass */
    private array $retyped = [];

    /** @var array<string, true> locals written other than by a plain `$x = ...` (never retyped) */
    private array $complex_writes = [];

    /** A plain assignment into a local Psalm types as mixed: its static type is a retyping candidate. */
    public function noteMixedAssign(string $name, RustType $t): void
    {
        $this->mixed_assigns[$name][] = $t;
    }

    public function __construct(
        public readonly Program $program,
        public readonly FunctionRecord $record,
        public readonly ?ClassModel $class,
        public readonly Casts $casts,
        public readonly Builtins $builtins,
        public readonly Diagnostics $diag,
        public readonly ?BodyEmitter $parent = null,
        /** class bound to `static` when emitting a late-static-binding copy of a static method */
        public readonly ?ClassModel $static_class = null,
    ) {
        $this->w = new Writer();
        $this->type_context = $record->file_path . ' ' . ($record->fq_class_name !== null ? $record->fq_class_name . '::' : '') . ($record->method_name ?? $record->storage->cased_name ?? '{closure}') . '() body';
        $this->ret_type = RustType::unit();
        $this->gen_key = RustType::int();
        $this->gen_val = RustType::mixed();
        if ($class !== null) {
            $this->this_type = RustType::class($class->fqcn);
        }
        if ($parent !== null) {
            $this->this_expr = 'this';
            $this->this_type = $parent->this_type;
            $this->generics = $parent->generics;
        }
    }

    public function types(): TypeMapper
    {
        $this->program->types->context = $this->type_context;
        $this->program->types->generic_names = $this->generics;
        return $this->program->types;
    }

    /** `File.php:function` naming this body in the map inventories. */
    private string $type_context;

    public function tmp(string $prefix = '__t'): string
    {
        return $prefix . (++$this->tmp_counter);
    }

    public function warn(string $kind, ?Node $node = null): void
    {
        $this->diag->warn($kind, $node, $this->record->file_path);
    }

    // ------------------------------------------------------------------ Psalm type access

    public function psalmType(Node $node): ?Union
    {
        return $this->record->node_data->getType($node);
    }

    public function inferred(Expr $e): ?RustType
    {
        $t = $this->psalmType($e);
        if ($t === null) {
            return null;
        }
        return $this->types()->map($t);
    }

    /** The Rust type an expression is known to have (falls back to mixed). */
    public function inferredOrMixed(Expr $e): RustType
    {
        return $this->inferred($e) ?? RustType::mixed();
    }

    /** Whether Psalm considers the expression's value possibly undefined (missing array key etc.). */
    public function possiblyUndefined(Expr $e): bool
    {
        $t = $this->psalmType($e);
        return $t !== null && $t->possibly_undefined;
    }

    // ------------------------------------------------------------------ locals

    /**
     * Compute the declared Rust types of all locals from Psalm's per-statement snapshots.
     *
     * @param array<string, RustType> $params  param name => type (already declared)
     */
    public function declareLocals(array $params): void
    {
        foreach ($params as $name => $type) {
            $this->vars[$name] = $type;
        }
        // Assignments inside conditions (`while (($x = end($a)) && ...)`) never appear in a statement snapshot
        // with the value's own type (the body only sees the narrowed variable): widen the local by the
        // assigned expression's type so the assignment itself needs no runtime cast.
        $var_types = $this->record->var_types;
        $finder = new NodeFinder();
        $stmts = $this->record->node instanceof ArrowFunction ? [] : ($this->record->node->getStmts() ?? []);
        foreach (self::ownNodes($stmts, Expr\Assign::class) as $assign) {
            if ($assign->var instanceof Expr\Variable && is_string($assign->var->name) && isset($var_types['$' . $assign->var->name])) {
                $assigned = $this->psalmType($assign->expr);
                if ($assigned !== null && !$assigned->hasMixed()) {
                    $var_types['$' . $assign->var->name][] = $assigned;
                }
            }
        }
        // the locals the body mentions (Psalm's snapshots also carry synthetic ids, e.g. `$prop` for
        // `array_filter($this->prop)`): anything else is not a variable of this function
        $mentioned = [];
        // the key/value variables of a foreach over a known-empty iterable are never bound (its body is not emitted)
        $unbound = [];
        foreach (self::ownNodes($stmts, Stmt\Foreach_::class) as $fe) {
            if ($this->inferredOrMixed($fe->expr)->isEmptyIterable()) {
                foreach ([$fe->keyVar, $fe->valueVar] as $fv) {
                    if ($fv instanceof Expr\Variable) {
                        $unbound[spl_object_id($fv)] = true;
                    }
                }
            }
        }
        foreach (self::ownNodes($stmts, Expr\Variable::class) as $v) {
            if (is_string($v->name) && !isset($unbound[spl_object_id($v)])) {
                $mentioned[$v->name] = true;
            }
        }
        foreach (self::ownNodes($stmts, Closure::class) as $closure) {
            foreach ($closure->uses as $use) {
                if (is_string($use->var->name)) {
                    $mentioned[$use->var->name] = true;
                }
            }
        }
        foreach ($var_types as $var_id => $types) {
            $name = substr($var_id, 1);
            if ($name === 'this' || $name === '_' || isset($this->predeclared[$name])) {
                // `$_` is the discard variable: never bound (see the foreach/list emission), never declared
                continue;
            }
            if (!isset($mentioned[$name]) && !isset($params[$name])) {
                continue;
            }
            if (isset($params[$name])) {
                if (!empty($this->byref[$name]) || !empty($this->cells[$name]) || !empty($this->refvars[$name])) {
                    continue;
                }
                $declared = null;
                foreach ($this->record->storage->params as $sp) {
                    if ($sp->name === $name) {
                        $declared = $sp->type;
                    }
                }
                if ($params[$name]->hasGeneric()) {
                    // a generic parameter stays generic: Psalm's narrowings (`is_array($t)`, instanceof) are
                    // applied at the narrowing use through gcast, not by retyping the variable
                    continue;
                }
                $joined = $this->types()->join($declared !== null ? [$declared, ...$types] : $types);
                $rust = $this->types()->map($joined);
                if (($rust->containsMixed() && !$params[$name]->containsMixed())
                    || (str_contains($rust->toRust(), 'AnyObject') && !str_contains($params[$name]->toRust(), 'AnyObject'))
                ) {
                    // the signature's (docblock-inherited) type is more precise than the body's native view
                    continue;
                }
                if ($rust->toRust() !== $params[$name]->toRust()) {
                    $this->rebound[$name] = $params[$name];
                    $this->vars[$name] = $rust;
                    $this->late[$name] = false;
                }
                continue;
            }
            $joined = $this->types()->join($types);
            $rust = $this->types()->map($joined);
            if ($rust->containsMixed()) {
                // a local holding a generic value plus Psalm narrowings of it keeps the generic type
                foreach ($types as $__t) {
                    $__r = $this->types()->map($__t);
                    if ($__r->kind === RustType::GENERIC) {
                        $rust = $__r;
                        break;
                    }
                }
            }
            $this->vars[$name] = $rust;
            $this->late[$name] = !$rust->hasDefault();
            // SSA axis diagnostic: a local whose value takes >=2 types across the function forces a union/Mixed
            // local; splitting it into one name per type in the PHP source removes the union (pzoom "one type per
            // var" idiom). Purely informational — uses the already-computed $rust and Psalm type STRINGS (no extra
            // types()->map() calls, whose need()/registration side effects would perturb emission).
            // Count REAL (non-unit) union members: a `T|false` / `T|bool` union has 1 real member + True/False
            // units and is a legit closed union from the SOURCE type, NOT a "same var, different types" reuse — the
            // true SSA violations are >=2 disjoint real members (Str+List, ClassA+ClassB, ...) or genuine Mixed.
            $real_members = 0;
            if ($rust->kind === RustType::UNION) {
                foreach ($rust->params as $__m) {
                    if (!($__m->kind === RustType::RT_GENERIC && str_starts_with($__m->name, '__unit_'))) {
                        $real_members++;
                    }
                }
            }
            if ($rust->containsMixed()) {
                // Mixed-removal diagnostic: every local whose storage type mentions Mixed, with the Psalm types that
                // produced it (the actionable per-variable list for retyping the PHP source).
                $psalm_names = [];
                foreach ($types as $__t) {
                    $psalm_names[(string) $__t] = true;
                }
                \fwrite(\STDERR, '[mixed-var] $' . $name . ' :: ' . $rust->toRust() . ' <= ' . implode(' , ', array_keys($psalm_names)) . ' @ ' . $this->type_context . "\n");
                if (getenv('DBG_REC') && $name === getenv('DBG_REC')) {
                    foreach ($this->record->stmt_vars as $__st) {
                        $__vars = $this->record->stmt_vars[$__st];
                        if (isset($__vars['$' . $name])) {
                            \fwrite(\STDERR, "    stmt L" . $__st->getStartLine() . ' ' . $__st::class . ' => ' . $__vars['$' . $name]->getId() . "\n");
                        }
                    }
                }
            }
            if (($rust->kind === RustType::MIXED || ($rust->kind === RustType::UNION && $real_members >= 2)) && count($types) >= 2) {
                $psalm_names = [];
                foreach ($types as $__t) {
                    $psalm_names[(string) $__t] = true;
                }
                if (count($psalm_names) >= 2) {
                    \fwrite(\STDERR, '[ssa-candidate] $' . $name . ' :: ' . $rust->toRust() . ' <= ' . implode(' , ', array_keys($psalm_names)) . ' @ ' . $this->type_context . "\n");
                }
            }
        }
    }

    /** Whether a local can be used as a `&mut T` place (reference variables can't). */
    public function hasMutPlace(string $name): bool
    {
        return empty($this->refvars[$name]);
    }

    /**
     * Whether `$name` is a plain `let mut c: T` local (or rebound param, same shape) — NOT wrapped in a PhpCell,
     * PhpRef, global accessor, by-ref, or Late<T>. Only such a binding can be the receiver of an immutable class's
     * `Rc::make_mut` set_/mut accessor directly (`c.set_x(v)`); wrapped locals need their own access path. Used by
     * the immutable-Rc<T> in-place write in LValueTrait::place().
     */
    public function isPlainLocal(string $name): bool
    {
        return empty($this->cells[$name]) && empty($this->refvars[$name]) && empty($this->globals[$name])
            && empty($this->byref[$name]) && empty($this->late[$name]);
    }

    /**
     * Like isPlainLocal but ALSO allows a Late<T> local (accessed via `.get_mut()` for a `&mut T`). A local that is
     * not a PhpCell/PhpRef/global/by-ref binding — i.e. a plain `let mut c: T` or `let mut c: Late<T>`. Used by the
     * immutable-Rc<T> in-place write-path, which handles the Late case (wither clone-locals) via get_mut().
     */
    public function isWritableLocal(string $name): bool
    {
        return empty($this->cells[$name]) && empty($this->refvars[$name]) && empty($this->globals[$name])
            && empty($this->byref[$name]);
    }

    /**
     * Find locals that need reference semantics: those bound by `$x = &...` (reference variables),
     * and those aliased by such a binding or captured with `use (&$x)` (shared cells).
     *
     * @param list<Stmt> $stmts
     */
    /** @var array<string, true> locals declared with `global $x` (reference variables backed by the runtime's globals) */
    public array $globals = [];

    public function isSuperglobal(string $name): bool
    {
        return isset(self::SUPERGLOBALS[$name]);
    }

    private function scanReferences(array $stmts): void
    {
        $finder = new \PhpParser\NodeFinder();
        foreach ($finder->findInstanceOf($stmts, Stmt\Global_::class) as $global) {
            foreach ($global->vars as $var) {
                if ($var instanceof Expr\Variable && is_string($var->name)) {
                    $this->refvars[$var->name] = true;
                    $this->globals[$var->name] = true;
                }
            }
        }
        foreach ($finder->findInstanceOf($stmts, Expr\AssignRef::class) as $assign) {
            if ($assign->var instanceof Expr\Variable && is_string($assign->var->name)) {
                $this->refvars[$assign->var->name] = true;
                if ($assign->expr instanceof Expr\Variable && is_string($assign->expr->name) && $assign->expr->name !== 'this') {
                    $this->cells[$assign->expr->name] = true;
                }
            }
        }
        foreach ($finder->findInstanceOf($stmts, Closure::class) as $closure) {
            foreach ($closure->uses as $use) {
                if ($use->byRef && is_string($use->var->name)) {
                    $this->cells[$use->var->name] = true;
                }
            }
        }
        foreach ($this->refvars as $name => $_) {
            unset($this->cells[$name]);
        }
    }

    /**
     * Owned/borrowed (axis 5): find locals read exactly once (outside loops/closures) so that read can move.
     * @param list<Stmt> $stmts
     */
    private function scanSingleUse(array $stmts): void
    {
        $this->single_use = [];
        $finder = new NodeFinder();
        // Variable nodes that are WRITE targets (a fresh `$v = ...`), not value reads. `$v[..] =`/`$v->p =`
        // read the base, so they are deliberately NOT collected here (counted as reads -> inflate count -> safe).
        $writes = [];
        foreach ($finder->findInstanceOf($stmts, Expr\Assign::class) as $a) {
            foreach ($this->assignTargetVars($a->var) as $v) {
                $writes[spl_object_id($v)] = true;
            }
        }
        foreach ($finder->findInstanceOf($stmts, Expr\AssignRef::class) as $a) {
            if ($a->var instanceof Expr\Variable) {
                $writes[spl_object_id($a->var)] = true;
            }
        }
        foreach ($finder->findInstanceOf($stmts, Stmt\Foreach_::class) as $f) {
            if ($f->keyVar instanceof Expr\Variable) {
                $writes[spl_object_id($f->keyVar)] = true;
            }
            foreach ($this->assignTargetVars($f->valueVar) as $v) {
                $writes[spl_object_id($v)] = true;
            }
        }
        // CAPTURED vars appear inside a real closure or a try block (emitted as a generated catch_unwind
        // closure): moving them anywhere risks move-into-closure, so they are unsafe for BOTH single-use move
        // and return-move (moveVar consults $this->move_captured). REPEATED vars appear in a loop's repeated
        // part (body/cond/update) -> unsafe for single-use only (a return diverges, so return-move is fine).
        // A loop's once-evaluated parts (a foreach collection, a `for` init) constrain neither.
        $this->move_captured = [];
        $captured = [];
        foreach ($finder->findInstanceOf($stmts, Closure::class) as $node) {
            $captured[] = $node;
        }
        foreach ($finder->findInstanceOf($stmts, ArrowFunction::class) as $node) {
            $captured[] = $node;
        }
        foreach ($finder->findInstanceOf($stmts, Stmt\TryCatch::class) as $node) {
            $captured[] = $node;
        }
        foreach ($captured as $node) {
            foreach ($finder->findInstanceOf([$node], Expr\Variable::class) as $v) {
                if (is_string($v->name)) {
                    $this->move_captured[$v->name] = true;
                }
            }
        }
        $unsafe = $this->move_captured; // single-use is also blocked by everything that blocks return-move
        $repeated = [];
        foreach ($finder->findInstanceOf($stmts, Stmt\Foreach_::class) as $node) {
            $repeated = array_merge($repeated, $node->stmts, [$node->valueVar]);
            if ($node->keyVar !== null) {
                $repeated[] = $node->keyVar;
            }
        }
        foreach ($finder->findInstanceOf($stmts, Stmt\While_::class) as $node) {
            $repeated = array_merge($repeated, $node->stmts, [$node->cond]); // While_->cond is a single Expr
        }
        foreach ($finder->findInstanceOf($stmts, Stmt\Do_::class) as $node) {
            $repeated = array_merge($repeated, $node->stmts, [$node->cond]); // Do_->cond is a single Expr
        }
        foreach ($finder->findInstanceOf($stmts, Stmt\For_::class) as $node) {
            // init runs once; cond, loop (update) and body repeat
            $repeated = array_merge($repeated, $node->stmts, $node->cond, $node->loop);
        }
        foreach ($repeated as $node) {
            if ($node === null) {
                continue;
            }
            foreach ($finder->findInstanceOf([$node], Expr\Variable::class) as $v) {
                if (is_string($v->name)) {
                    $unsafe[$v->name] = true;
                }
            }
        }
        $counts = [];
        foreach ($finder->findInstanceOf($stmts, Expr\Variable::class) as $v) {
            if (!is_string($v->name) || isset($writes[spl_object_id($v)])) {
                continue;
            }
            $counts[$v->name] = ($counts[$v->name] ?? 0) + 1;
        }
        foreach ($counts as $name => $n) {
            if ($n === 1 && !isset($unsafe[$name])) {
                $this->single_use[$name] = true;
            }
        }
    }

    /**
     * Variable nodes assigned as fresh values by an assignment target (a bare `$v` or the `$v`s of a
     * list()/[] destructuring). ArrayDim/Property/StaticProp targets read their base, so return none.
     * @return list<Expr\Variable>
     */
    private function assignTargetVars(Expr $var): array
    {
        if ($var instanceof Expr\Variable) {
            return [$var];
        }
        if ($var instanceof Expr\List_ || $var instanceof Expr\Array_) {
            $out = [];
            foreach ($var->items as $item) {
                if ($item !== null) {
                    foreach ($this->assignTargetVars($item->value) as $v) {
                        $out[] = $v;
                    }
                }
            }
            return $out;
        }
        return [];
    }

    /** Emit `let` declarations for locals that are not parameters. */
    public function emitLocalDecls(array $params): void
    {
        foreach ($this->vars as $name => $type) {
            $rn = Names::var($name);
            if (isset($params[$name])) {
                if (isset($this->rebound[$name])) {
                    $this->w->line('let mut ' . $rn . ': ' . $type->toRust() . ' = ' . $this->casts->convert($rn, $this->rebound[$name], $type) . ';');
                    continue;
                }
                // parameters captured/bound by reference are re-wrapped
                if (!empty($this->cells[$name])) {
                    $this->w->line('let ' . $rn . ': PhpCell<' . $type->toRust() . '> = cell_of(' . $rn . ');');
                } elseif (!empty($this->refvars[$name])) {
                    $this->w->line('let mut ' . $rn . ': PhpRef<' . $type->toRust() . '> = PhpRef::of(' . $rn . ');');
                }
                continue;
            }
            if (isset($this->predeclared[$name])) {
                continue;
            }
            if (!empty($this->cells[$name])) {
                $this->w->line('let ' . $rn . ': PhpCell<' . $type->toRust() . '> = new_cell();');
                continue;
            }
            if (!empty($this->globals[$name])) {
                $lit = Names::rustStringLiteral($name);
                // the CLI globals have runtime accessors of their own types
                [$src, $src_t] = match ($name) {
                    'argv' => ['php_rt::argv()', RustType::list(RustType::str())],
                    'argc' => ['php_rt::argc()', RustType::int()],
                    default => ['php_rt::global_get(' . $lit . ')', RustType::mixed()],
                };
                $this->w->line('let mut ' . $rn . ': PhpRef<' . $type->toRust() . '> = PhpRef::new(move || ' . $this->casts->convert($src, $src_t, $type) . ', move |__v: ' . $type->toRust() . '| php_rt::global_set(' . $lit . ', ' . $this->casts->convert('__v', $type, RustType::mixed()) . '));');
                continue;
            }
            if (!empty($this->refvars[$name])) {
                $this->w->line('let mut ' . $rn . ': PhpRef<' . $type->toRust() . '> = PhpRef::detached();');
                continue;
            }
            if (!empty($this->late[$name])) {
                $this->w->line('let mut ' . $rn . ': Late<' . $type->toRust() . '> = Late::uninit();');
            } else {
                $this->w->line('let mut ' . $rn . ': ' . $type->toRust() . ' = Default::default();');
            }
        }
    }

    /** Emit an expression used as a receiver (no clone for `$this`). */
    public function receiver(Expr $e): Val
    {
        if ($e instanceof Expr\Variable && $e->name === 'this' && $this->this_type !== null) {
            // Honor Psalm's narrowing of `$this` to a subclass (`if ($this instanceof Sub) { $this->subMethod(); }`):
            // downcast so a method/property that lives only on the subclass resolves statically (match dispatch)
            // instead of falling back to the dynamic protocol. The narrowing guarantees the runtime class, so the
            // downcast is safe (and panics loudly if a docblock lied, per the no-dynamic-protocol policy).
            $inf = $this->inferred($e);
            if ($inf !== null && $inf->kind === RustType::CLASS_ && $this->this_type->kind === RustType::CLASS_
                && $inf->toRust() !== $this->this_type->toRust()
            ) {
                $sub = $this->program->classOf($inf);
                $cur = $this->program->classOf($this->this_type);
                if ($sub !== null && $cur !== null && $sub->isSubclassOf($cur)) {
                    return new Val($this->casts->convert($this->this_expr . '.clone()', $this->this_type, $inf), $inf);
                }
            }
            return new Val($this->this_expr, $this->this_type);
        }
        return $this->expr($e);
    }

    /** Read a local variable as an owned value of its declared type. */
    /**
     * A value read with its declared (storage) type rather than the type Psalm narrowed it to at this
     * point: used by runtime type checks (`is_string($x)`, `$x instanceof C`), where a value that
     * contradicts the narrowing (an `Other__` union member) must not be unwrapped before the test.
     */
    public function rawValue(Expr $e): Val
    {
        if (($e instanceof Expr\Variable && is_string($e->name) && $e->name !== 'this')
            || ($e instanceof Expr\PropertyFetch && $e->name instanceof \PhpParser\Node\Identifier)
            || ($e instanceof Expr\StaticPropertyFetch && $e->name instanceof \PhpParser\Node\VarLikeIdentifier)
        ) {
            $p = $this->place($e);
            return new Val($p->read(), $p->type);
        }
        return $this->expr($e);
    }

    /** A value of type `$t` the transpiler cannot produce (unsupported construct, external code): panics if reached. */
    public function dead(string $msg, RustType $t): Val
    {
        return new Val($this->deadCode($msg, $t), $t);
    }

    public function deadCode(string $msg, RustType $t): string
    {
        if ($t->kind === RustType::NEVER) {
            return 'unreachable!(' . Names::rustStringLiteral($msg) . ')';
        }
        return 'dead::<' . $t->toRust() . '>(' . Names::rustStringLiteral($msg) . ')';
    }

    /**
     * Locals assigned in the body that Psalm's statement snapshots never mention (assignments inside
     * conditions of nested expressions): declared with the assigned expression's inferred type.
     *
     * @param list<Stmt> $stmts
     */
    /**
     * Instances of `$class` in the function's own body: nested closures, arrow functions and anonymous classes
     * are not descended into (their assignments declare their own locals).
     *
     * @template T of \PhpParser\Node
     * @param list<\PhpParser\Node> $stmts
     * @param class-string<T> $class
     * @return list<T>
     */
    private static function ownNodes(array $stmts, string $class): array
    {
        $found = [];
        $traverser = new \PhpParser\NodeTraverser();
        $traverser->addVisitor(new class ($class, $found) extends \PhpParser\NodeVisitorAbstract {
            /** @param list<\PhpParser\Node> $found */
            public function __construct(private string $class, public array &$found)
            {
            }

            public function enterNode(\PhpParser\Node $node)
            {
                if ($node instanceof Closure || $node instanceof ArrowFunction || $node instanceof Stmt\Class_) {
                    return \PhpParser\NodeVisitor::DONT_TRAVERSE_CHILDREN;
                }
                if ($node instanceof $this->class) {
                    $this->found[] = $node;
                }
                return null;
            }
        });
        $traverser->traverse($stmts);
        return $found;
    }

    private function declareAssignedVars(array $stmts, array $params): void
    {
        $finder = new \PhpParser\NodeFinder();
        foreach (self::ownNodes($stmts, Expr\Assign::class) as $assign) {
            $target = $assign->var;
            if ($target instanceof Expr\List_ || $target instanceof Expr\Array_) {
                foreach ($finder->findInstanceOf([$target], Expr\Variable::class) as $v) {
                    if (!is_string($v->name) || $v->name === 'this' || isset($this->vars[$v->name]) || isset($params[$v->name]) || isset($this->predeclared[$v->name])) {
                        continue;
                    }
                    $pt = $this->psalmType($v);
                    $t = $pt !== null ? $this->types()->map($pt) : RustType::mixed();
                    $this->vars[$v->name] = $t;
                    $this->late[$v->name] = !$t->hasDefault();
                }
                continue;
            }
            if (!$target instanceof Expr\Variable || !is_string($target->name) || $target->name === 'this') {
                continue;
            }
            $name = $target->name;
            if (isset($this->vars[$name]) || isset($params[$name]) || isset($this->predeclared[$name])) {
                continue;
            }
            $pt = $this->psalmType($assign->expr);
            $t = $pt !== null ? $this->types()->map($pt) : RustType::mixed();
            $this->vars[$name] = $t;
            $this->late[$name] = !$t->hasDefault();
        }
        // catch variables (possibly unused, which Psalm's snapshots omit)
        foreach (self::ownNodes($stmts, Stmt\Catch_::class) as $catch) {
            if ($catch->var === null || !is_string($catch->var->name) || isset($this->vars[$catch->var->name]) || isset($params[$catch->var->name])) {
                continue;
            }
            $pt = $this->psalmType($catch->var);
            $t = $pt !== null ? $this->types()->map($pt) : RustType::class('Throwable');
            $this->vars[$catch->var->name] = $t;
            $this->late[$catch->var->name] = !$t->hasDefault();
        }
        // out-parameters of calls (`preg_match($re, $s, $matches)`) are assigned by the callee
        $calls = [
            ...self::ownNodes($stmts, Expr\FuncCall::class),
            ...self::ownNodes($stmts, Expr\MethodCall::class),
            ...self::ownNodes($stmts, Expr\StaticCall::class),
            ...self::ownNodes($stmts, Expr\New_::class),
        ];
        foreach ($calls as $call) {
            foreach ($call->args as $arg) {
                if (!$arg instanceof \PhpParser\Node\Arg || !$arg->value instanceof Expr\Variable || !is_string($arg->value->name)) {
                    continue;
                }
                $name = $arg->value->name;
                if ($name === 'this' || isset($this->vars[$name]) || isset($params[$name]) || isset($this->predeclared[$name]) || isset(self::SUPERGLOBALS[$name])) {
                    continue;
                }
                $pt = $this->psalmType($arg->value);
                $t = $pt !== null ? $this->types()->map($pt) : RustType::mixed();
                $this->vars[$name] = $t;
                $this->late[$name] = !$t->hasDefault();
            }
        }
    }

    private const SUPERGLOBALS = ['_SERVER' => true, '_ENV' => true, '_GET' => true, '_POST' => true, '_COOKIE' => true, '_FILES' => true, '_REQUEST' => true, 'GLOBALS' => true];

    public function readVar(string $name): Val
    {
        if ($name === 'this') {
            return new Val($this->this_expr . '.clone()', $this->this_type ?? RustType::anyObject());
        }
        if (isset(self::SUPERGLOBALS[$name])) {
            return new Val('php_rt::superglobal(' . Names::rustStringLiteral($name) . ')', RustType::map(RustType::str(), RustType::mixed()));
        }
        if (!isset($this->vars[$name])) {
            // never assigned anywhere: PHP reads an undefined variable as null
            $this->warn('unknown variable $' . $name);
            return new Val('Mixed::Null', RustType::mixed());
        }
        // Owned/borrowed (axis 5): a local read exactly once (outside loops/closures) can move, not clone.
        if (!empty($this->single_use[$name]) && ($moved = $this->moveVar($name)) !== null) {
            return $moved;
        }
        $t = $this->vars[$name];
        $rn = Names::var($name);
        if (!empty($this->cells[$name])) {
            return new Val($rn . '.borrow().get().clone()', $t);
        }
        if (!empty($this->refvars[$name])) {
            return new Val($rn . '.get()', $t);
        }
        if (!empty($this->byref[$name])) {
            return new Val('(*' . $rn . ').clone()', $t);
        }
        if (!empty($this->borrow[$name])) {
            // `&T` param (borrow-safe): every use is a read (method receiver / property fetch), which works on
            // the borrow directly -- no Rc clone. `$rn` is already `&T`; Rust auto-refs for `.m()`/`.prop_get()`.
            return new Val($rn, $t);
        }
        if (!empty($this->late[$name])) {
            return new Val($rn . '.get().clone()', $t);
        }
        if ($t->isCopy()) {
            return new Val($rn, $t);
        }
        return new Val($rn . '.clone()', $t);
    }

    /**
     * Owned/borrowed (axis 5): read a local by MOVING out of it instead of cloning, saving an Rc refcount
     * bump. Only valid where the read is the last use on this control-flow path — the caller must guarantee
     * that (e.g. a bare `return $x;`, which diverges, so Rust permits the move even if other paths use $x).
     * Returns null when the local can't be safely moved (reference/cell/byref/global/captured/Copy/superglobal
     * /unknown), leaving the caller to fall back to the cloning read.
     */
    public function moveVar(string $name): ?Val
    {
        if ($name === 'this' || isset(self::SUPERGLOBALS[$name]) || !isset($this->vars[$name])) {
            return null;
        }
        if (!empty($this->cells[$name]) || !empty($this->refvars[$name]) || !empty($this->byref[$name])
            || !empty($this->globals[$name]) || !empty($this->move_captured[$name])
        ) {
            return null;
        }
        $t = $this->vars[$name];
        if ($t->isCopy()) {
            return null; // a plain read is already a cheap copy; nothing to save
        }
        $rn = Names::var($name);
        if (!empty($this->late[$name])) {
            return new Val($rn . '.take()', $t);
        }
        return new Val($rn, $t);
    }

    /** Statement storing a value (already of the declared type) into a local. */
    public function storeVar(string $name, string $code): string
    {
        $rn = Names::var($name);
        $simple = preg_match('/^[A-Za-z_][A-Za-z0-9_]*(?:\.clone\(\))?$/', $code) === 1;
        if (!empty($this->cells[$name])) {
            return $simple ? $rn . '.borrow_mut().set(' . $code . ');' : '{ let __sv = ' . $code . '; ' . $rn . '.borrow_mut().set(__sv); }';
        }
        if (!empty($this->refvars[$name])) {
            return $simple ? $rn . '.set(' . $code . ');' : '{ let __sv = ' . $code . '; ' . $rn . '.set(__sv); }';
        }
        if (!empty($this->byref[$name])) {
            return '*' . $rn . ' = ' . $code . ';';
        }
        if (!empty($this->late[$name])) {
            return $simple ? $rn . '.set(' . $code . ');' : '{ let __sv = ' . $code . '; ' . $rn . '.set(__sv); }';
        }
        return $rn . ' = ' . $code . ';';
    }

    /** Place expression giving `&mut T` access to a local. */
    public function varPlace(string $name): string
    {
        $rn = Names::var($name);
        if (!empty($this->cells[$name])) {
            return '(*' . $rn . '.borrow_mut().get_mut())';
        }
        if (!empty($this->byref[$name])) {
            return '(*' . $rn . ')';
        }
        if (!empty($this->late[$name])) {
            return '(*' . $rn . '.get_mut())';
        }
        return $rn;
    }

    public function varType(string $name): RustType
    {
        if ($name === 'this') {
            return $this->this_type ?? RustType::anyObject();
        }
        if (isset(self::SUPERGLOBALS[$name])) {
            return RustType::map(RustType::str(), RustType::mixed());
        }
        if (!isset($this->vars[$name]) && !empty($this->globals[$name])) {
            $this->vars[$name] = RustType::mixed();
        }
        if (!isset($this->vars[$name])) {
            // never assigned anywhere: reads yield null (see readVar)
            $this->warn('unknown variable $' . $name);
            return RustType::mixed();
        }
        return $this->vars[$name];
    }

    // ------------------------------------------------------------------ control-flow bookkeeping

    public function newLabel(string $prefix): string
    {
        return "'" . $prefix . (++$this->label_counter);
    }

    public function pushLoop(string $break_label, string $continue_label, bool $is_block_continue, string $writeback = ''): void
    {
        $this->loops[] = [
            'break' => $break_label,
            'continue' => $continue_label,
            'try_depth' => $this->try_depth,
            'is_block_continue' => $is_block_continue,
            'writeback' => $writeback,
        ];
    }

    /** Write-back statements (foreach by reference) for loops crossed by a `break n`/`continue n`. */
    private function writebacks(int $n): string
    {
        $out = '';
        $total = count($this->loops);
        for ($i = $total - 1; $i >= $total - $n && $i >= 0; $i--) {
            if ($this->loops[$i]['writeback'] !== '' && $this->loops[$i]['try_depth'] >= $this->try_depth) {
                $out .= $this->loops[$i]['writeback'] . ' ';
            }
        }
        return $out;
    }

    public function popLoop(): void
    {
        array_pop($this->loops);
    }

    /** Code for `break $n` / `continue $n`. */
    public function jump(bool $is_break, int $n): string
    {
        $idx = count($this->loops) - $n;
        if ($idx < 0 || !isset($this->loops[$idx])) {
            $this->warn('break/continue outside loop');
            return 'unreachable!()';
        }
        $loop = $this->loops[$idx];
        $wb = $this->writebacks($n);
        if ($loop['try_depth'] < $this->try_depth) {
            return '{ ' . $wb . 'return Flow::' . ($is_break ? 'Break' : 'Continue') . '(' . $n . ') }';
        }
        if ($is_break) {
            return '{ ' . $wb . 'break ' . $loop['break'] . ' }';
        }
        if ($loop['is_block_continue']) {
            return '{ ' . $wb . 'break ' . $loop['continue'] . ' }';
        }
        return '{ ' . $wb . 'continue ' . $loop['continue'] . ' }';
    }

    /** Code for `return <value>`. */
    public function returnCode(string $value_code): string
    {
        if ($this->try_depth > 0) {
            // Inside a try/catch body closure (which returns Flow<T>): a PHP `return` becomes Flow::Return so the
            // enclosing dispatch performs the actual method return. No Result wrapper (panic-based error model).
            return 'return Flow::Return(' . $value_code . ')';
        }
        return 'return ' . $value_code;
    }

    public function enterTry(): void
    {
        $this->try_depth++;
    }

    public function leaveTry(): void
    {
        $this->try_depth--;
    }

    public function inTry(): bool
    {
        return $this->try_depth > 0;
    }

    /** Whether a `break`/`continue` targeting loop `$n` must escape a try closure. */
    public function jumpEscapesTry(int $n): bool
    {
        $idx = count($this->loops) - $n;
        return isset($this->loops[$idx]) && $this->loops[$idx]['try_depth'] < $this->try_depth;
    }

    public function loopDepth(): int
    {
        return count($this->loops);
    }

    /** Loops whose bodies are (partly) inside the current try closure need Flow handling after the try. */
    public function loopsInsideTry(): array
    {
        $out = [];
        foreach ($this->loops as $i => $loop) {
            if ($loop['try_depth'] >= $this->try_depth) {
                $out[] = count($this->loops) - $i; // n for break n
            }
        }
        return $out;
    }

    /** The loop record for `break n` (n from 1). */
    public function loopAt(int $n): ?array
    {
        $idx = count($this->loops) - $n;
        return $this->loops[$idx] ?? null;
    }

    // ------------------------------------------------------------------ function emission

    /**
     * Emit the whole body (declarations + statements) of the function.
     *
     * @param array<string, RustType> $params
     */
    public function emitBody(array $params, ?array $stmts, RustType $ret_type): string
    {
        $saved = $this->program->record_dispatch;
        $this->program->record_dispatch = true;
        try {
            return $this->emitBodyInner($params, $stmts, $ret_type);
        } finally {
            $this->program->record_dispatch = $saved;
        }
    }

    /**
     * Locals Psalm types as `mixed` but whose every (plain) assignment has a precise static type are retyped to
     * the join of those types and the body is emitted again: Psalm loses the type where the transpiler's
     * declarations keep it (a property of a union member, a deep alias read).
     */
    private function emitBodyInner(array $params, ?array $stmts, RustType $ret_type): string
    {
        $this->scanWriteKinds($stmts ?? []);
        // a discarded pass must not leave its recorded conversions/erasures behind
        $snap_erasures = $this->casts->erasures;
        $snap_casts = $this->casts->casts;
        $snap_checks = $this->casts->instance_checks;
        // retyping iterates: a local typed by a retyped one (`foreach ($table as [$a, $b])` after `$table`)
        // becomes a candidate only in the pass where its source is typed
        for ($pass = 0; ; $pass++) {
            $out = $this->emitBodyPass($params, $stmts, $ret_type);
            $retype = [];
            if (getenv('DBG_RETYPE') && $this->mixed_assigns !== []) {
                fwrite(STDERR, '[retype] ' . $this->type_context . ' candidates: ' . implode(', ', array_map(fn($n, $ts) => '$' . $n . '=' . implode('|', array_map(fn($t) => $t->toRust(), $ts)) . (isset($this->complex_writes[$n]) ? ' (complex)' : ''), array_keys($this->mixed_assigns), $this->mixed_assigns)) . "\n");
            }
            foreach ($this->mixed_assigns as $name => $types) {
                if (isset($this->retyped[$name]) || isset($this->complex_writes[$name]) || isset($params[$name])
                    || !empty($this->byref[$name]) || !empty($this->cells[$name]) || !empty($this->refvars[$name]) || !empty($this->globals[$name])
                ) {
                    continue;
                }
                // `$x = null` assignments make the local nullable
                $nullable = false;
                $values = [];
                foreach ($types as $t) {
                    if ($t->kind === RustType::UNIT) {
                        $nullable = true;
                    } else {
                        $values[] = $t;
                    }
                }
                $u = $values === [] ? null : $this->program->unionOfRust($values);
                if ($u !== null && !$u->containsMixed() && !$u->hasGeneric()) {
                    $retype[$name] = $nullable && $u->kind !== RustType::OPTION ? RustType::option($u) : $u;
                }
            }
            if (getenv('DBG_RETYPE') && $retype !== []) {
                fwrite(STDERR, '[retype] ' . $this->type_context . ' RETYPED: ' . implode(', ', array_map(fn($n, $t) => '$' . $n . '=' . $t->toRust(), array_keys($retype), $retype)) . "\n");
            }
            if ($retype === [] || $pass >= 4) {
                return $out;
            }
            $this->retyped = $retype + $this->retyped;
            $this->mixed_assigns = [];
            $this->w = new Writer();
            $this->casts->erasures = $snap_erasures;
            $this->casts->casts = $snap_casts;
            $this->casts->instance_checks = $snap_checks;
        }
    }

    /** Locals written other than by `$x = expr` (compound ops, element/property writes, foreach, list(), references). */
    private function scanWriteKinds(array $stmts): void
    {
        $finder = new NodeFinder();
        $root = function (Expr $e): ?string {
            while ($e instanceof Expr\ArrayDimFetch || $e instanceof Expr\PropertyFetch || $e instanceof Expr\StaticPropertyFetch) {
                if ($e instanceof Expr\StaticPropertyFetch || $e instanceof Expr\PropertyFetch) {
                    // a write through a property does not rebind the local holding the object (a handle)
                    return null;
                }
                $e = $e->var;
            }
            return $e instanceof Expr\Variable && is_string($e->name) ? $e->name : null;
        };
        $mark = function (?string $n): void {
            if ($n !== null) {
                $this->complex_writes[$n] = true;
            }
        };
        foreach ($finder->find($stmts, static fn(Node $n) => $n instanceof Expr\Assign || $n instanceof Expr\AssignOp || $n instanceof Expr\AssignRef
            || $n instanceof Expr\PreInc || $n instanceof Expr\PreDec || $n instanceof Expr\PostInc || $n instanceof Expr\PostDec
            || $n instanceof Node\Stmt\Foreach_ || $n instanceof Node\Stmt\Global_ || $n instanceof Node\Stmt\Static_ || $n instanceof Expr\Closure || $n instanceof Expr\List_ || $n instanceof Expr\Unset_ || $n instanceof Node\Stmt\Unset_) as $n) {
            if ($n instanceof Expr\Assign) {
                if (!($n->var instanceof Expr\Variable)) {
                    $mark($root($n->var));
                }
            } elseif ($n instanceof Expr\AssignOp || $n instanceof Expr\AssignRef) {
                $mark($root($n->var));
                if ($n instanceof Expr\AssignRef) {
                    $mark($root($n->expr));
                }
            } elseif ($n instanceof Expr\PreInc || $n instanceof Expr\PreDec || $n instanceof Expr\PostInc || $n instanceof Expr\PostDec) {
                $mark($root($n->var));
            } elseif ($n instanceof Node\Stmt\Foreach_) {
                // the loop variables are assigned per iteration (a by-reference value is a reference); a
                // destructured value (`as [$a, $b]`) assigns its variables plainly too
                if ($n->byRef || !($n->valueVar instanceof Expr\Variable || $n->valueVar instanceof Expr\List_ || $n->valueVar instanceof Expr\Array_)) {
                    $mark($root($n->valueVar));
                }
                if ($n->keyVar !== null && !($n->keyVar instanceof Expr\Variable)) {
                    $mark($root($n->keyVar));
                }
            } elseif ($n instanceof Node\Stmt\Global_) {
                foreach ($n->vars as $v) {
                    $mark($root($v));
                }
            } elseif ($n instanceof Node\Stmt\Static_) {
                foreach ($n->vars as $v) {
                    $mark($v->var->name === null || !is_string($v->var->name) ? null : $v->var->name);
                }
            } elseif ($n instanceof Expr\Closure) {
                foreach ($n->uses as $u) {
                    if ($u->byRef && is_string($u->var->name)) {
                        $mark($u->var->name);
                    }
                }
            } elseif ($n instanceof Expr\List_) {
                foreach ($n->items as $item) {
                    if ($item !== null && !($item->value instanceof Expr\Variable) && !$item->byRef) {
                        $mark($root($item->value));
                    } elseif ($item !== null && $item->byRef) {
                        $mark($root($item->value));
                    }
                }
            } elseif ($n instanceof Node\Stmt\Unset_) {
                foreach ($n->vars as $v) {
                    $mark($root($v));
                }
            }
        }
    }

    private function emitBodyPass(array $params, ?array $stmts, RustType $ret_type): string
    {
        $this->types()->current_class = $this->class?->fqcn;
        $this->types()->current_crate = $this->class !== null ? $this->class->crate : $this->program->crateOfRecord($this->record);
        $this->ret_type = $ret_type;
        $storage = $this->record->storage;
        $this->is_generator = $storage->has_yield;
        if ($this->is_generator && $ret_type->kind === RustType::RT_GENERIC) {
            $this->gen_key = $ret_type->params[0];
            $this->gen_val = $ret_type->params[1];
        }

        $this->scanReferences($stmts ?? []);
        $this->scanSingleUse($stmts ?? []);
        $this->declareLocals($params);
        $this->declareAssignedVars($stmts ?? [], $params);
        foreach ($this->retyped as $name => $t) {
            $this->vars[$name] = $t;
            $this->late[$name] = !$t->hasDefault();
        }
        $this->emitLocalDecls($params);
        if ($this->is_generator) {
            $this->w->line('let mut __gen: Vec<(' . $this->gen_key->toRust() . ', ' . $this->gen_val->toRust() . ')> = Vec::new();');
        }

        foreach ($stmts ?? [] as $stmt) {
            $this->stmt($stmt);
        }

        $last = $stmts ? $stmts[array_key_last($stmts)] : null;
        $ends_with_return = $last instanceof Stmt\Return_ || $last instanceof Stmt\Throw_
            || ($last instanceof Stmt\Expression && $last->expr instanceof Expr\Throw_)
            || ($last instanceof Stmt\Expression && $last->expr instanceof Expr\Exit_);
        if ($this->is_generator) {
            $this->w->line('#[allow(unreachable_code)] Generator::from_pairs(__gen)');
        } elseif (!$ends_with_return) {
            $this->w->line('#[allow(unreachable_code)] ' . $this->implicitReturn());
        }
        return $this->w->get();
    }

    private function implicitReturn(): string
    {
        $t = $this->ret_type;
        if ($t->kind === RustType::NEVER) {
            return 'unreachable!()';
        }
        $value = match ($t->kind) {
            RustType::UNIT => '()',
            RustType::OPTION => 'None',
            RustType::MIXED => 'Mixed::Null',
            default => null,
        };
        if ($value === null) {
            return 'unreachable!("missing return")';
        }
        // Panic-based model: methods return bare `T` (no Result).
        return $value;
    }
}
