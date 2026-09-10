<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node;
use PhpParser\Node\Expr;
use PhpParser\Node\Expr\ArrowFunction;
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

    public Writer $w;

    /** @var array<string, RustType> declared Rust type of each local */
    public array $vars = [];

    /** @var array<string, bool> locals stored as Late<T> */
    public array $late = [];

    /** @var array<string, bool> params passed as `&mut T` */
    public array $byref = [];

    /** @var array<string, bool> locals stored as Rc<RefCell<T>> (captured by reference) */
    public array $cells = [];

    /** @var array<string, bool> locals bound by reference (`$x = &...`): stored as PhpRef<T> */
    public array $refvars = [];

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

    public bool $is_generator = false;

    public RustType $gen_key;

    public RustType $gen_val;

    /** Expression that evaluates to the current object handle (`self` in methods, `this` in closures). */
    public string $this_expr = 'self';

    public ?RustType $this_type = null;

    /** class `self`/`parent` refer to (the declaring class of an inherited body emitted for a subclass); defaults to `class` */
    public ?ClassModel $self_class = null;

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
        $this->ret_type = RustType::unit();
        $this->gen_key = RustType::int();
        $this->gen_val = RustType::mixed();
        if ($class !== null) {
            $this->this_type = RustType::class($class->fqcn);
        }
        if ($parent !== null) {
            $this->this_expr = 'this';
            $this->this_type = $parent->this_type;
        }
    }

    public function types(): TypeMapper
    {
        return $this->program->types;
    }

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
        foreach ($this->record->var_types as $var_id => $types) {
            $name = substr($var_id, 1);
            if ($name === 'this' || isset($this->predeclared[$name])) {
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
                $joined = $this->types()->join($declared !== null ? [$declared, ...$types] : $types);
                $rust = $this->types()->map($joined);
                if ($rust->toRust() !== $params[$name]->toRust()) {
                    $this->rebound[$name] = $params[$name];
                    $this->vars[$name] = $rust;
                    $this->late[$name] = false;
                }
                continue;
            }
            $joined = $this->types()->join($types);
            $rust = $this->types()->map($joined);
            $this->vars[$name] = $rust;
            $this->late[$name] = !$rust->hasDefault();
        }
    }

    /** Whether a local can be used as a `&mut T` place (reference variables can't). */
    public function hasMutPlace(string $name): bool
    {
        return empty($this->refvars[$name]);
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
                $this->w->line('let mut ' . $rn . ': PhpRef<' . $type->toRust() . '> = PhpRef::new(move || ' . $this->casts->convert('php_rt::global_get(' . $lit . ')', RustType::mixed(), $type) . ', move |__v: ' . $type->toRust() . '| php_rt::global_set(' . $lit . ', ' . $this->casts->convert('__v', $type, RustType::mixed()) . '));');
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
    private function declareAssignedVars(array $stmts, array $params): void
    {
        $finder = new \PhpParser\NodeFinder();
        foreach ($finder->findInstanceOf($stmts, Expr\Assign::class) as $assign) {
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
        foreach ($finder->findInstanceOf($stmts, Stmt\Catch_::class) as $catch) {
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
            ...$finder->findInstanceOf($stmts, Expr\FuncCall::class),
            ...$finder->findInstanceOf($stmts, Expr\MethodCall::class),
            ...$finder->findInstanceOf($stmts, Expr\StaticCall::class),
            ...$finder->findInstanceOf($stmts, Expr\New_::class),
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
        if (!empty($this->late[$name])) {
            return new Val($rn . '.get().clone()', $t);
        }
        if ($t->isCopy()) {
            return new Val($rn, $t);
        }
        return new Val($rn . '.clone()', $t);
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
            return '{ ' . $wb . 'return Ok(Flow::' . ($is_break ? 'Break' : 'Continue') . '(' . $n . ')) }';
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
            return 'return Ok(Flow::Return(' . $value_code . '))';
        }
        return 'return Ok(' . $value_code . ')';
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
        $this->declareLocals($params);
        $this->declareAssignedVars($stmts ?? [], $params);
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
            $this->w->line('#[allow(unreachable_code)] Ok(Generator::from_pairs(__gen))');
        } elseif (!$ends_with_return) {
            $this->w->line('#[allow(unreachable_code)] ' . $this->implicitReturn());
        }
        return $this->w->get();
    }

    private function implicitReturn(): string
    {
        $t = $this->ret_type;
        if ($t->kind === RustType::UNIT) {
            return 'Ok(())';
        }
        if ($t->kind === RustType::OPTION) {
            return 'Ok(None)';
        }
        if ($t->kind === RustType::MIXED) {
            return 'Ok(Mixed::Null)';
        }
        if ($t->kind === RustType::NEVER) {
            return 'unreachable!()';
        }
        return 'unreachable!("missing return")';
    }
}
