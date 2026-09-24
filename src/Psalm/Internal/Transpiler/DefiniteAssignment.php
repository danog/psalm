<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node;
use PhpParser\Node\Expr;
use PhpParser\Node\Stmt;

/**
 * Which locals are definitely assigned before every read, so they can be declared as plain uninitialized Rust
 * bindings (`let mut x: T;`, checked again by rustc) instead of `Late<T>` cells (an Option checked on every read,
 * a drop on every write).
 *
 * Deliberately conservative, at least as strict as rustc's definite-initialization: a loop body never assigns for
 * the code after the loop, `switch` and `try` contribute nothing after them, a conditional operand (`&&`/`||`/`??`
 * right side, ternary branches) contributes nothing, and any mention of the variable that is not a plain read or a
 * whole-variable write (isset/empty/unset, by-reference use, closure capture by reference, dynamic variables) keeps
 * it a Late cell. A branch that leaves (return/throw/break/continue/exit) constrains nothing.
 *
 * @internal
 */
final class DefiniteAssignment
{
    /** @var array<string, true> the locals under consideration */
    private array $candidates;

    /** @var array<string, true> candidates that failed: read before a definite assignment, or an unsupported use */
    private array $failed = [];

    /**
     * @param list<Stmt> $stmts the function body
     * @param array<string, true> $candidates locals to classify (never parameters, by-ref, cells or globals)
     * @return array<string, true> the candidates that are definitely assigned before every read
     */
    public static function plainLocals(array $stmts, array $candidates): array
    {
        if ($candidates === []) {
            return [];
        }
        $da = new self($candidates);
        $assigned = [];
        $da->stmts($stmts, $assigned);
        return array_diff_key($candidates, $da->failed);
    }

    /** @param array<string, true> $candidates */
    private function __construct(array $candidates)
    {
        $this->candidates = $candidates;
    }

    /**
     * Walks statements in order. `$assigned` is the set of candidates definitely assigned on entry and is updated
     * in place. Returns false when control cannot fall out of the list (its last statement leaves).
     *
     * @param list<Stmt> $stmts
     * @param array<string, true> $assigned
     */
    private function stmts(array $stmts, array &$assigned): bool
    {
        foreach ($stmts as $s) {
            if (!$this->stmt($s, $assigned)) {
                return false;
            }
        }
        return true;
    }

    /** @param array<string, true> $assigned */
    private function stmt(Stmt $s, array &$assigned): bool
    {
        if ($s instanceof Stmt\Expression) {
            $this->expr($s->expr, $assigned);
            return true;
        }
        if ($s instanceof Stmt\Return_) {
            if ($s->expr !== null) {
                $this->expr($s->expr, $assigned);
            }
            return false;
        }
        if ($s instanceof Stmt\Echo_ || $s instanceof Stmt\InlineHTML) {
            foreach ($s instanceof Stmt\Echo_ ? $s->exprs : [] as $e) {
                $this->expr($e, $assigned);
            }
            return true;
        }
        if ($s instanceof Stmt\If_) {
            $this->expr($s->cond, $assigned);
            $branches = [];
            $a = $assigned;
            if ($this->stmts($s->stmts, $a)) {
                $branches[] = $a;
            }
            foreach ($s->elseifs as $ei) {
                // an elseif condition runs only when the earlier ones were false: its assignments are conditional
                $c = $assigned;
                $this->expr($ei->cond, $c);
                $a = $c;
                if ($this->stmts($ei->stmts, $a)) {
                    $branches[] = $a;
                }
            }
            if ($s->else !== null) {
                $a = $assigned;
                if ($this->stmts($s->else->stmts, $a)) {
                    $branches[] = $a;
                }
            } else {
                $branches[] = $assigned;
            }
            if ($branches === []) {
                return false; // every branch leaves
            }
            $assigned = self::intersect($branches);
            return true;
        }
        if ($s instanceof Stmt\While_ || $s instanceof Stmt\Do_) {
            if ($s instanceof Stmt\While_) {
                $this->expr($s->cond, $assigned);
            }
            $body = $assigned;
            $this->stmts($s->stmts, $body);
            if ($s instanceof Stmt\Do_) {
                $this->expr($s->cond, $body);
            }
            // the body may not run (while) or is not modelled (do): nothing it assigns counts afterwards
            return true;
        }
        if ($s instanceof Stmt\For_) {
            foreach ($s->init as $e) {
                $this->expr($e, $assigned);
            }
            $body = $assigned;
            foreach ($s->cond as $e) {
                $this->expr($e, $body);
            }
            $this->stmts($s->stmts, $body);
            foreach ($s->loop as $e) {
                $this->expr($e, $body);
            }
            return true;
        }
        if ($s instanceof Stmt\Foreach_) {
            $this->expr($s->expr, $assigned);
            $body = $assigned;
            if ($s->keyVar !== null) {
                $this->assignTarget($s->keyVar, $body);
            }
            if ($s->byRef) {
                $this->mention($s->valueVar);
            } else {
                $this->assignTarget($s->valueVar, $body);
            }
            $this->stmts($s->stmts, $body);
            return true;
        }
        if ($s instanceof Stmt\Switch_) {
            $this->expr($s->cond, $assigned);
            foreach ($s->cases as $case) {
                $body = $assigned;
                if ($case->cond !== null) {
                    $this->expr($case->cond, $body);
                }
                $this->stmts($case->stmts, $body);
            }
            return true; // fall-through and a missing default make the cases' assignments conditional
        }
        if ($s instanceof Stmt\TryCatch) {
            $body = $assigned;
            $this->stmts($s->stmts, $body);
            foreach ($s->catches as $catch) {
                $body = $assigned;
                if ($catch->var !== null) {
                    $this->assignTarget($catch->var, $body);
                }
                $this->stmts($catch->stmts, $body);
            }
            if ($s->finally !== null) {
                $body = $assigned;
                $this->stmts($s->finally->stmts, $body);
            }
            return true;
        }
        if ($s instanceof Stmt\Throw_) {
            $this->expr($s->expr, $assigned);
            return false;
        }
        if ($s instanceof Stmt\Break_ || $s instanceof Stmt\Continue_ || $s instanceof Stmt\Goto_) {
            return false;
        }
        if ($s instanceof Stmt\Unset_) {
            foreach ($s->vars as $v) {
                $this->mention($v);
            }
            return true;
        }
        if ($s instanceof Stmt\Global_ || $s instanceof Stmt\Static_) {
            foreach ($s instanceof Stmt\Global_ ? $s->vars : array_map(static fn($v) => $v->var, $s->vars) as $v) {
                $this->mention($v);
            }
            return true;
        }
        if ($s instanceof Stmt\Block) {
            return $this->stmts($s->stmts, $assigned);
        }
        if ($s instanceof Stmt\Nop || $s instanceof Stmt\Label || $s instanceof Stmt\Declare_ || $s instanceof Stmt\Use_
            || $s instanceof Stmt\Const_ || $s instanceof Stmt\ClassLike || $s instanceof Stmt\Function_
        ) {
            return true;
        }
        // anything else: every variable mentioned inside is treated as an unsupported use
        $this->mentionAll($s);
        return true;
    }

    /**
     * Walks an expression in evaluation order: reads are checked against `$assigned`, whole-variable writes are
     * added to it. Conditional operands run in a copy of the set.
     *
     * @param array<string, true> $assigned
     */
    private function expr(?Node $e, array &$assigned): void
    {
        if ($e === null) {
            return;
        }
        if ($e instanceof Expr\Variable) {
            $this->read($e, $assigned);
            return;
        }
        if ($e instanceof Expr\Assign) {
            $this->expr($e->expr, $assigned);
            $this->assignTarget($e->var, $assigned);
            return;
        }
        if ($e instanceof Expr\AssignOp) {
            // `$x .= y` / `$x ??= y` read the target first; a `??=` right side is conditional
            $this->readTarget($e->var, $assigned);
            if ($e instanceof Expr\AssignOp\Coalesce) {
                $c = $assigned;
                $this->expr($e->expr, $c);
            } else {
                $this->expr($e->expr, $assigned);
            }
            $this->assignTarget($e->var, $assigned);
            return;
        }
        if ($e instanceof Expr\AssignRef) {
            $this->mention($e->var);
            $this->mention($e->expr);
            return;
        }
        if ($e instanceof Expr\PreInc || $e instanceof Expr\PreDec || $e instanceof Expr\PostInc || $e instanceof Expr\PostDec) {
            $this->readTarget($e->var, $assigned);
            $this->assignTarget($e->var, $assigned);
            return;
        }
        if ($e instanceof Expr\BinaryOp\BooleanAnd || $e instanceof Expr\BinaryOp\BooleanOr
            || $e instanceof Expr\BinaryOp\LogicalAnd || $e instanceof Expr\BinaryOp\LogicalOr
            || $e instanceof Expr\BinaryOp\LogicalXor || $e instanceof Expr\BinaryOp\Coalesce
        ) {
            $this->expr($e->left, $assigned);
            $c = $assigned;
            $this->expr($e->right, $c);
            return;
        }
        if ($e instanceof Expr\Ternary) {
            $this->expr($e->cond, $assigned);
            $a = $assigned;
            $this->expr($e->if, $a);
            $b = $assigned;
            $this->expr($e->else, $b);
            if ($e->if !== null) {
                $assigned = self::intersect([$a, $b]);
            }
            return;
        }
        if ($e instanceof Expr\Match_) {
            $this->expr($e->cond, $assigned);
            foreach ($e->arms as $arm) {
                $c = $assigned;
                foreach ($arm->conds ?? [] as $cond) {
                    $this->expr($cond, $c);
                }
                $this->expr($arm->body, $c);
            }
            return;
        }
        if ($e instanceof Expr\Closure) {
            foreach ($e->uses as $use) {
                if ($use->byRef) {
                    $this->mention($use->var);
                } else {
                    $this->read($use->var, $assigned);
                }
            }
            return; // the body is another function
        }
        if ($e instanceof Expr\ArrowFunction) {
            // captures by value at creation: every outer variable it mentions is read now
            $params = [];
            foreach ($e->params as $p) {
                if ($p->var instanceof Expr\Variable && is_string($p->var->name)) {
                    $params[$p->var->name] = true;
                }
            }
            foreach ((new \PhpParser\NodeFinder())->findInstanceOf([$e->expr], Expr\Variable::class) as $v) {
                if (is_string($v->name) && !isset($params[$v->name])) {
                    $this->read($v, $assigned);
                }
            }
            return;
        }
        if ($e instanceof Expr\Isset_ || $e instanceof Expr\Empty_) {
            foreach ($e instanceof Expr\Isset_ ? $e->vars : [$e->expr] as $v) {
                $this->mention($v);
            }
            return;
        }
        if ($e instanceof Expr\FuncCall && $e->name instanceof Node\Name
            && in_array(strtolower($e->name->toString()), ['compact', 'extract', 'get_defined_vars', 'func_get_args'], true)
        ) {
            foreach ($this->candidates as $name => $_) {
                $this->failed[$name] = true;
            }
            return;
        }
        if ($e instanceof Expr\FuncCall || $e instanceof Expr\MethodCall || $e instanceof Expr\StaticCall
            || $e instanceof Expr\New_ || $e instanceof Expr\NullsafeMethodCall
        ) {
            foreach ($e->getSubNodeNames() as $sub) {
                if ($sub === 'args') {
                    continue;
                }
                $this->child($e->$sub, $assigned);
            }
            foreach ($e->args as $arg) {
                if ($arg instanceof Node\Arg) {
                    if ($arg->byRef) {
                        $this->mention($arg->value);
                    } else {
                        $this->expr($arg->value, $assigned);
                    }
                }
            }
            return;
        }
        if ($e instanceof Expr\List_ || $e instanceof Expr\Array_) {
            // a read of an array literal; as an assignment target it is handled by assignTarget
            foreach ($e->items as $item) {
                if ($item !== null) {
                    $this->expr($item->key, $assigned);
                    if ($item->byRef) {
                        $this->mention($item->value);
                    } else {
                        $this->expr($item->value, $assigned);
                    }
                }
            }
            return;
        }
        foreach ($e->getSubNodeNames() as $sub) {
            $this->child($e->$sub, $assigned);
        }
    }

    /** @param array<string, true> $assigned */
    private function child(mixed $n, array &$assigned): void
    {
        if ($n instanceof Expr) {
            $this->expr($n, $assigned);
        } elseif ($n instanceof Node) {
            // identifiers, names, args: nothing to track except what they contain
            foreach ($n->getSubNodeNames() as $sub) {
                $this->child($n->$sub, $assigned);
            }
        } elseif (is_array($n)) {
            foreach ($n as $x) {
                $this->child($x, $assigned);
            }
        }
    }

    /** A whole-variable write (or a write into part of it, which needs the variable initialized: a read first). */
    private function assignTarget(Expr $t, array &$assigned): void
    {
        if ($t instanceof Expr\Variable) {
            if (is_string($t->name)) {
                $assigned[$t->name] = true;
            } else {
                $this->expr($t->name, $assigned);
                $this->failAll();
            }
            return;
        }
        if ($t instanceof Expr\List_ || $t instanceof Expr\Array_) {
            foreach ($t->items as $item) {
                if ($item !== null) {
                    $this->expr($item->key, $assigned);
                    if ($item->byRef) {
                        $this->mention($item->value);
                    } else {
                        $this->assignTarget($item->value, $assigned);
                    }
                }
            }
            return;
        }
        // `$x[..] = v`, `$x->f = v`: the container is read (must be initialized) and stays assigned
        $this->readTarget($t, $assigned);
    }

    /** The reads an assignment target performs before its write (`$x[$k]`, `$x->f`, `$x`). */
    private function readTarget(Expr $t, array &$assigned): void
    {
        if ($t instanceof Expr\Variable) {
            $this->read($t, $assigned);
            return;
        }
        if ($t instanceof Expr\ArrayDimFetch) {
            $this->readTarget($t->var, $assigned);
            $this->expr($t->dim, $assigned);
            return;
        }
        if ($t instanceof Expr\PropertyFetch || $t instanceof Expr\NullsafePropertyFetch) {
            $this->expr($t->var, $assigned);
            if (!$t->name instanceof Node\Identifier) {
                $this->expr($t->name, $assigned);
            }
            return;
        }
        $this->expr($t, $assigned);
    }

    /** A plain read: fails the candidate unless definitely assigned here. */
    private function read(Expr\Variable $v, array $assigned): void
    {
        if (!is_string($v->name)) {
            $this->expr($v->name, $assigned);
            $this->failAll();
            return;
        }
        if (isset($this->candidates[$v->name]) && !isset($assigned[$v->name])) {
            $this->failed[$v->name] = true;
        }
    }

    /** Any other use of a variable (by reference, isset/empty/unset, global/static): not a plain local. */
    private function mention(Node $n): void
    {
        foreach ((new \PhpParser\NodeFinder())->findInstanceOf([$n], Expr\Variable::class) as $v) {
            if (is_string($v->name)) {
                if (isset($this->candidates[$v->name])) {
                    $this->failed[$v->name] = true;
                }
            } else {
                $this->failAll();
            }
        }
    }

    private function mentionAll(Node $n): void
    {
        $this->mention($n);
    }

    private function failAll(): void
    {
        foreach ($this->candidates as $name => $_) {
            $this->failed[$name] = true;
        }
    }

    /**
     * @param non-empty-list<array<string, true>> $sets
     * @return array<string, true>
     */
    private static function intersect(array $sets): array
    {
        $out = $sets[0];
        foreach (array_slice($sets, 1) as $s) {
            $out = array_intersect_key($out, $s);
        }
        return $out;
    }
}
