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
    private function scanReferences(array $stmts): void
    {
        $finder = new \PhpParser\NodeFinder();
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
        foreach ((new \PhpParser\NodeFinder())->findInstanceOf($stmts, Expr\Assign::class) as $assign) {
            $target = $assign->var;
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
    }

    public function readVar(string $name): Val
    {
        if ($name === 'this') {
            return new Val($this->this_expr . '.clone()', $this->this_type ?? RustType::anyObject());
        }
        if (!isset($this->vars[$name])) {
            // variable never seen in snapshots (e.g. only assigned inside a closure use list)
            $this->warn('unknown variable $' . $name);
            $this->vars[$name] = RustType::mixed();
            $this->late[$name] = false;
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
        if (!empty($this->cells[$name])) {
            return $rn . '.borrow_mut().set(' . $code . ');';
        }
        if (!empty($this->refvars[$name])) {
            return $rn . '.set(' . $code . ');';
        }
        if (!empty($this->byref[$name])) {
            return '*' . $rn . ' = ' . $code . ';';
        }
        if (!empty($this->late[$name])) {
            return $rn . '.set(' . $code . ');';
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
        if (!isset($this->vars[$name])) {
            $this->warn('unknown variable $' . $name);
            $this->vars[$name] = RustType::mixed();
            $this->late[$name] = false;
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
