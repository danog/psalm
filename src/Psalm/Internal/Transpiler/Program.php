<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Stmt\ClassMethod;
use PhpParser\Node\Stmt\Function_;
use Psalm\Codebase;
use Psalm\Internal\MethodIdentifier;
use Psalm\Storage\ClassLikeStorage;
use Psalm\Storage\FunctionLikeStorage;
use Psalm\Storage\MethodStorage;
use Psalm\Type\Union;

use function array_keys;
use function count;
use function explode;
use function max;
use function array_key_exists;
use function array_pop;
use function file_get_contents;
use function glob;
use function implode;
use function is_file;
use function rtrim;
use function str_starts_with;
use function strlen;
use function substr;
use function in_array;
use function strtolower;
use function usort;
use function fwrite;

use const STDERR;

/**
 * The whole-program model: classes, hierarchy, functions.
 *
 * @internal
 */
final class Program
{
    /** @var array<lowercase-string, ClassModel> */
    public array $classes = [];

    /** @var array<lowercase-string, FunctionModel> free functions by lowercase fully-qualified name */
    public array $functions = [];

    public TypeMapper $types;

    /** @var array<string, MethodModel> cache keyed by "class::method" */
    private array $method_cache = [];

    /** @var array<lowercase-string, GlobalConstModel> global constants by lowercase name */
    public array $constants = [];

    /**
     * Axis-8 (panic-based errors): the set of exception types (lowercase FQCN) that are "Resultable" — i.e.
     * appear in some `catch` clause, or implement Psalm\Exception\Resultable. A `throw` of such a type (or a
     * subtype) is emitted as `Result::Err` (recoverable, catchable); any other `throw` becomes a panic. Populated
     * by computeResultable().
     * @var array<lowercase-string, true>
     */
    public array $resultable = [];


    public function __construct(
        public readonly Codebase $codebase,
        public readonly Transpiler $transpiler,
    ) {
        $this->types = new TypeMapper($codebase, $this);
        $this->build();
    }

    /** @return list<ClassModel> distinct class models (aliases share a model) */
    public function uniqueClasses(): array
    {
        $seen = [];
        $out = [];
        foreach ($this->classes as $model) {
            $id = spl_object_id($model);
            if (!isset($seen[$id])) {
                $seen[$id] = true;
                $out[] = $model;
            }
        }
        return $out;
    }

    private function build(): void
    {
        // 1. project classes
        foreach ($this->transpiler->classes as $lc => $record) {
            $this->classes[$lc] = new ClassModel($record->storage->name, $record->storage, $record->node, true);
            $this->classes[$lc]->crate = $this->transpiler->crateOfFile($record->file_path);
        }

        fwrite(STDERR, "[program] hierarchy\n");
        // 2. hierarchy (also pulls in external ancestors)
        foreach (array_keys($this->classes) as $lc) {
            $model = $this->classes[$lc];
            $this->linkAncestors($model);
        }

        foreach ($this->uniqueClasses() as $model) {
            foreach ($model->ancestors as $ancestor) {
                // dispatch enums only enumerate classes of their own crate (and upstream ones);
                // instances of subclasses from downstream crates travel through the `Other__` variant
                if ($model->isConcrete() && $model->is_project && $model->crate <= $ancestor->crate) {
                    $ancestor->concrete[] = $model;
                } elseif ($model->isConcrete() && $model->is_project) {
                    // a concrete subclass in a downstream crate: the ancestor's enum can't name it, so the
                    // hierarchy needs the `Other__` escape (see ClassModel::$has_downstream)
                    $ancestor->has_downstream = true;
                }
            }
            if ($model->isConcrete() && $model->is_project) {
                $model->concrete[] = $model;
            }
        }
        if (getenv('FORCE_DOWNSTREAM')) {
            // A/B diagnostic: keep the `Other__` dynamic-dispatch fallback on every hierarchy (the pre-gating
            // behavior), to isolate whether closed-enum dispatch is behind a correctness regression.
            foreach ($this->uniqueClasses() as $model) {
                if ($model->is_project) {
                    $model->has_downstream = true;
                }
            }
        }
        foreach ($this->uniqueClasses() as $model) {
            $model->concrete = $this->dedupe($model->concrete);
            $model->children = $this->dedupe($model->children);
            $model->ancestors = $this->dedupe($model->ancestors);
            usort($model->concrete, static fn(ClassModel $a, ClassModel $b) => $a->fqcn <=> $b->fqcn);
            usort($model->children, static fn(ClassModel $a, ClassModel $b) => $a->fqcn <=> $b->fqcn);
        }

        fwrite(STDERR, "[program] functions\n");
        // 3. free functions
        foreach ($this->transpiler->functions as $record) {
            if ($record->node instanceof Function_) {
                $name = (string) ($record->node->getAttribute('resolvedName') ?? $record->node->name->name);
                $this->functions[strtolower($name)] = new FunctionModel($name, $record);
            }
        }

        fwrite(STDERR, "[program] members\n");
        // 4. members
        foreach ($this->uniqueClasses() as $model) {
            $this->types->current_class = $model->fqcn;
            $this->types->current_crate = $model->crate;
            $this->buildFields($model);
            $this->buildConstants($model);
        }
        $this->types->current_class = null;
        $this->types->current_crate = 0;
        fwrite(STDERR, "[program] methods\n");
        foreach ($this->uniqueClasses() as $model) {
            $this->types->current_crate = $model->crate;
            $this->buildMethods($model);
        }
        $this->unifyDispatchTypes();
        foreach ($this->functions as $fn) {
            $this->types->current_crate = $this->crateOfRecord($fn->record);
            $this->resolveSignature($fn->record->storage, $fn->param_types, $fn->return_type, $fn->record->node);
            $fn->generics = $this->last_generics;
            $this->types->generic_names = [];
            $this->computeBorrowParams($fn);
        }
        $this->types->current_crate = 0;
        $this->propagateLateStaticBinding();
        fwrite(STDERR, "[program] cross-crate inheritance\n");
        foreach ($this->uniqueClasses() as $model) {
            if ($model->is_project && $model->crateRoot() === $model) {
                $this->importUpstreamMethods($model);
            }
        }
        $this->reportInheritanceComponents();
        $this->computeResultable();
        $this->computeThrows();
        $this->computeExternalWrites();
        $this->computeHierarchyImmutable();
        $this->computeBorrowAgreement();
    }

    /**
     * Whole-hierarchy Rc<T>-immutable (axis 4, hot path). A class hierarchy can only convert to Rc<T> ALL-OR-NOTHING:
     * the abstract base's dispatch-enum field accessors must return a uniform type across variants, so every concrete
     * leaf under the same root class must be Rc<T> together. For each such hierarchy where EVERY concrete leaf is
     * immutable-safe (ClassModel::isImmutableSafeMember — mutation-free, write-free own+external, no init-helper
     * writes; withers handled by the LValueTrait make_mut-in-place write-path), mark every leaf hier_immutable.
     * Env-gated (AUTO_IMMUTABLE_HIER=N): only hierarchies with <= N leaves convert (N=0/unset disables), so the
     * mechanism can be validated on small hierarchies first before the big Atomic/Union ones. Diagnostic lists every
     * qualifying hierarchy and its size regardless of the cap.
     */
    private function computeHierarchyImmutable(): void
    {
        // DEPLOYED 2026-09-16: default cap converts all currently-qualifying hierarchies (UnresolvedConstant-18,
        // SourceControlInfo-1; validated by a paired ConstantTest = 131/7/8 identical converted-vs-baseline, and
        // smoke identical). AUTO_IMMUTABLE_HIER can still override the cap (0 disables) for experiments.
        $cap = getenv('AUTO_IMMUTABLE_HIER') !== false ? (int) getenv('AUTO_IMMUTABLE_HIER') : 64;
        // Convert hierarchies with a CONCRETE base and/or concrete-non-leaf members (CodeLocation; the hot Type\Atomic
        // TString/TInt/TArray shape) up to N members — a concrete-non-leaf is emitted Rc<T> for its Own struct while
        // ALSO a variant of its handle enum. DEPLOYED 2026-09-16 (default cap 256): the whole Atomic hierarchy (70
        // members) + CodeLocation convert; build green + smoke identical (55/7,81/26,122/25). HIER_CONCRETE=0 disables.
        $concrete_cap = getenv('HIER_CONCRETE') !== false ? (int) getenv('HIER_CONCRETE') : 256;
        if ($concrete_cap > 0) {
            $this->computeHierarchyImmutableConcrete($concrete_cap);
        }
        $seen_roots = [];
        foreach ($this->uniqueClasses() as $model) {
            if (!$model->is_project || !$model->isLeaf() || $model->parent === null) {
                continue; // standalone leaves are handled by immutable()'s auto-widen; non-leaves are enums
            }
            $root = $model;
            while ($root->parent !== null) {
                $root = $root->parent;
            }
            if (isset($seen_roots[$root->fqcn])) {
                continue;
            }
            $seen_roots[$root->fqcn] = true;
            // Only ABSTRACT / interface roots convert cleanly: a CONCRETE base (e.g. Psalm\CodeLocation, which is both
            // instantiated AND subclassed) is a non-leaf, so immutable()'s isLeaf() guard keeps it Rc<RefCell> while
            // its leaves would become Rc<T> -> MIXED representation -> E0308/E0596 in the base enum accessors. Require
            // every concrete member to be a leaf (no concrete non-leaf anywhere in the hierarchy).
            if ($root->isConcrete()) {
                continue;
            }
            $leaves = [];
            $has_concrete_nonleaf = false;
            foreach ($root->concrete as $c) {
                if ($c->isLeaf()) {
                    $leaves[] = $c;
                } else {
                    $has_concrete_nonleaf = true;
                }
            }
            if ($leaves === [] || $has_concrete_nonleaf) {
                continue;
            }
            $all_ok = true;
            foreach ($leaves as $leaf) {
                if (!$leaf->isImmutableSafeMember()) {
                    $all_ok = false;
                    break;
                }
            }
            if (!$all_ok) {
                continue;
            }
            // Hierarchies whose abstract base declares __construct (constructor inheritance) are handled by the
            // immutable inherited-ctor forwarding in ClassEmitter::emitMethodOnOwn + the self::/parent:: super-copy
            // in CallTrait: the inherited ctor chain runs as super-copies ON THE LEAF against `self` in place
            // (&mut self, make_mut, refcount 1) instead of `self.clone()` -> COW / missing enum __impl.
            fwrite(STDERR, "[hier-immutable-candidate] " . $root->fqcn . " (" . ($root->isInterface() ? "iface" : "abstract")
                . ") leaves=" . count($leaves) . ($cap > 0 && count($leaves) <= $cap ? " CONVERTING" : "") . "\n");
            if ($cap > 0 && count($leaves) <= $cap) {
                foreach ($leaves as $leaf) {
                    $leaf->hier_immutable = true;
                }
            }
        }
    }

    /**
     * EXPERIMENT: whole-hierarchy Rc<T> including CONCRETE bases and concrete-non-leaf members (the hot Atomic shape:
     * TString/TInt/TArray are concrete AND subclassed). Marks EVERY concrete member of a class hierarchy (root walked
     * via class parents) when all are isImmutableSafeMember. immutable() lets hier_immutable bypass its isLeaf guard,
     * so a concrete-non-leaf gets an Rc<T> Own struct while still being a variant of its handle enum. Gated by
     * HIER_CONCRETE=N (max members) so it can be validated on a small hierarchy (CodeLocation=4) before Atomic.
     */
    private function computeHierarchyImmutableConcrete(int $cap): void
    {
        $seen = [];
        foreach ($this->uniqueClasses() as $model) {
            if (!$model->is_project || !$model->isConcrete() || $model->parent === null) {
                continue;
            }
            $root = $model;
            while ($root->parent !== null) {
                $root = $root->parent;
            }
            if (isset($seen[$root->fqcn])) {
                continue;
            }
            $seen[$root->fqcn] = true;
            $members = $root->concrete; // every concrete class in the hierarchy (leaves + concrete non-leaves)
            if ($members === [] || count($members) > $cap) {
                continue;
            }
            $all_ok = true;
            $has_nonleaf = false;
            foreach ($members as $c) {
                if (!$c->isImmutableSafeMember(true, true)) { // allow memoization + external writes -> per-field Cell
                    $reason = !$c->isConcrete() ? 'not-concrete'
                        : ($c->storage->allowed_mutations > \Psalm\Storage\Mutations::LEVEL_INTERNAL_READ ? 'allowed_mutations=' . $c->storage->allowed_mutations
                        : 'helper-ctor-writes');
                    if (count($members) <= $cap) {
                        fwrite(STDERR, "[hier-concrete-skip] " . $root->fqcn . " member " . $c->fqcn . " fails: " . $reason . "\n");
                    }
                    $all_ok = false;
                    break;
                }
                if (!$c->isLeaf()) {
                    $has_nonleaf = true;
                }
            }
            if (!$all_ok || !$has_nonleaf) {
                continue; // pure-leaf hierarchies are handled by the default path
            }
            fwrite(STDERR, "[hier-immutable-concrete] " . $root->fqcn . " members=" . count($members) . " CONVERTING\n");
            foreach ($members as $c) {
                $c->hier_immutable = true;
            }
        }
    }

    /**
     * Rc<T>-immutable safety (axis 4): mark every class whose property is written post-construction by SOME
     * OTHER method (`$obj->prop = ...`, `$obj->prop op= ...` where `$obj` is not `$this`). Resolves `$obj`'s
     * Psalm-inferred type at the write site and flags each named class it can be. Feeds ClassModel::immutable()
     * auto-widen: an @psalm-immutable class that is externally written cannot be Rc<T> (the write would be lost
     * to Rc::make_mut COW on a shared handle). Own-`$this->` writes are handled separately (postConstructionWrittenFields).
     */
    private function computeExternalWrites(): void
    {
        fwrite(STDERR, "[program] external-write analysis\n");
        $finder = new \PhpParser\NodeFinder();
        // any `$base->prop = ...` / `$base->prop op= ...` (through array dims) where base is not `$this`, and any
        // by-reference alias of such a property (`&$base->prop`), plus writes via clone-then-mutate to a local are
        // covered separately (hasCloneMutateWither). We over-approximate: an aliased/foreach-by-ref property counts.
        $is_ext_write = static function (\PhpParser\Node $n): ?\PhpParser\Node\Expr\PropertyFetch {
            if ($n instanceof \PhpParser\Node\Expr\Assign || $n instanceof \PhpParser\Node\Expr\AssignOp
                || $n instanceof \PhpParser\Node\Expr\AssignRef
            ) {
                $t = $n->var;
            } elseif ($n instanceof \PhpParser\Node\Expr\PreInc || $n instanceof \PhpParser\Node\Expr\PreDec
                || $n instanceof \PhpParser\Node\Expr\PostInc || $n instanceof \PhpParser\Node\Expr\PostDec
            ) {
                $t = $n->var;
            } else {
                return null;
            }
            while ($t instanceof \PhpParser\Node\Expr\ArrayDimFetch) {
                $t = $t->var;
            }
            if ($t instanceof \PhpParser\Node\Expr\PropertyFetch
                && $t->name instanceof \PhpParser\Node\Identifier
                && !($t->var instanceof \PhpParser\Node\Expr\Variable && $t->var->name === 'this')
            ) {
                return $t;
            }
            return null;
        };
        // Wither false-positive relaxation: a write to a `$c = clone $x` clone-local is the class building a modified
        // copy of itself (handled by the make_mut-in-place write-path), NOT external mutation — so it must not flag the
        // class externally_written. Now DEFAULT (was gated behind HIER_CONCRETE): the place() bug it exposed — _mut on
        // an Option-narrowed clone-local — is fixed (varType-based unwrap in LValueTrait::place). Env var can still
        // force-disable for A/B: WITHER_FIX=0.
        $skip_withers = getenv('WITHER_FIX') !== '0';
        $scan = function (?array $stmts, $node_data) use ($finder, $is_ext_write, $skip_withers): void {
            if ($stmts === null || $node_data === null) {
                return;
            }
            // locals assigned `clone ...` in this method are WITHER copies: a write to `$c->prop` where `$c = clone $x`
            // is the class constructing a modified copy of itself (handled by the LValueTrait make_mut-in-place
            // write-path), NOT external mutation of a live instance. Don't let them flag the class externally_written
            // (that false positive was excluding wither-heavy immutable classes like CodeLocation / Type\Atomic\*).
            $clone_locals = [];
            if ($skip_withers) {
                foreach ($finder->find($stmts, static fn(\PhpParser\Node $n): bool =>
                    $n instanceof \PhpParser\Node\Expr\Assign
                    && $n->expr instanceof \PhpParser\Node\Expr\Clone_
                    && $n->var instanceof \PhpParser\Node\Expr\Variable
                    && is_string($n->var->name)) as $a
                ) {
                    /** @var \PhpParser\Node\Expr\Assign $a */
                    /** @var \PhpParser\Node\Expr\Variable $v */
                    $v = $a->var;
                    $clone_locals[$v->name] = true;
                }
            }
            foreach ($finder->find($stmts, static fn(\PhpParser\Node $n): bool => $is_ext_write($n) !== null) as $n) {
                $target = $is_ext_write($n);
                if ($target->var instanceof \PhpParser\Node\Expr\Variable && is_string($target->var->name)
                    && isset($clone_locals[$target->var->name])
                ) {
                    continue; // wither clone-local write, not external mutation
                }
                $base_type = $node_data->getType($target->var);
                if ($base_type === null) {
                    continue;
                }
                foreach ($base_type->getAtomicTypes() as $atomic) {
                    if ($atomic instanceof \Psalm\Type\Atomic\TNamedObject) {
                        $lc = strtolower($atomic->value);
                        if (isset($this->classes[$lc])) {
                            $this->classes[$lc]->externally_written = true;
                            $this->classes[$lc]->ext_written_fields[$target->name->name] = true;
                        }
                    }
                }
            }
        };
        foreach ($this->uniqueClasses() as $model) {
            foreach ($model->methods as $m) {
                if ($m->node === null || $m->record === null) {
                    continue;
                }
                $scan($m->node->stmts ?? [], $m->record->node_data);
            }
        }
        foreach ($this->functions as $fn) {
            if ($fn->record->node !== null) {
                $scan($fn->record->node->stmts ?? [], $fn->record->node_data);
            }
        }
        // diagnostics: total externally-written classes, and how many NEW classes auto-widen would add
        $ext = 0;
        $auto = 0;
        foreach ($this->uniqueClasses() as $model) {
            if ($model->externally_written) {
                $ext++;
            }
            if ($model->is_project && $model->parent === null && !$model->externally_written
                && $model->isLeaf()
                && $model->storage->allowed_mutations <= \Psalm\Storage\Mutations::LEVEL_INTERNAL_READ
            ) {
                $auto++;
                fwrite(STDERR, "[auto-immutable-candidate] " . $model->fqcn . "\n");
            }
        }
        fwrite(STDERR, "[external-writes] " . $ext . " classes externally written; " . $auto . " standalone-leaf @psalm-immutable auto-immutable candidates\n");
    }

    /**
     * Axis-8: populate $this->resultable with every exception type that is caught in a `catch` clause anywhere, or
     * that implements Psalm\Exception\Resultable. These are the "intentional / recoverable" exceptions; a throw of
     * any of them is emitted as Result::Err, while a throw of anything else becomes a panic (php_rt::uncaught).
     * The caught-clause scan makes built-in exceptions (InvalidArgumentException, …) — which cannot implement the
     * marker interface — Resultable too, so no currently-caught throw regresses to a panic.
     */
    private function computeResultable(): void
    {
        $finder = new \PhpParser\NodeFinder();
        $add_catches = function (?array $stmts) use ($finder): void {
            if ($stmts === null) {
                return;
            }
            foreach ($finder->findInstanceOf($stmts, \PhpParser\Node\Stmt\TryCatch::class) as $try) {
                foreach ($try->catches as $catch) {
                    foreach ($catch->types as $type) {
                        $fqcn = (string) ($type->getAttribute('resolvedName') ?? $type->toString());
                        $this->resultable[strtolower(ltrim($fqcn, '\\'))] = true;
                    }
                }
            }
        };
        foreach ($this->uniqueClasses() as $model) {
            foreach ($model->methods as $m) {
                if ($m->declaring === $model && $m->node !== null) {
                    $add_catches($m->node->stmts);
                }
            }
        }
        foreach ($this->functions as $fn) {
            if ($fn->record->node !== null) {
                $add_catches($fn->record->node->stmts);
            }
        }
        // Classes explicitly implementing the marker interface.
        foreach ($this->uniqueClasses() as $model) {
            $lc = strtolower($model->fqcn);
            if (isset($this->resultable[$lc])) {
                continue;
            }
            foreach ($model->ancestors as $anc) {
                if (strtolower($anc->fqcn) === 'psalm\\exception\\resultable') {
                    $this->resultable[$lc] = true;
                    break;
                }
            }
        }
        fwrite(STDERR, "[program] computeResultable: " . count($this->resultable) . " resultable/caught exception types\n");
    }

    /**
     * Whether a `throw` of the given (already-mapped) type must be Result::Err (recoverable) rather than a panic:
     * true if the type — or any of its ancestors — is a Resultable/caught exception. `Throwable`/`Exception` bases
     * are commonly caught, so most declared-exception throws resolve to true; genuine invariant exceptions that are
     * never caught resolve to false and panic.
     */
    public function isResultable(RustType $type): bool
    {
        if ($type->kind !== RustType::CLASS_ && $type->kind !== RustType::ANY_OBJECT && $type->kind !== RustType::MIXED) {
            return false;
        }
        if ($type->kind !== RustType::CLASS_) {
            // A throw whose static type is not a single concrete class (mixed/any-object): be safe and keep it
            // recoverable (Err) so we never turn a catchable throw into a panic by imprecision.
            return true;
        }
        $cls = $this->classOf($type);
        if ($cls === null) {
            return true;
        }
        if (isset($this->resultable[strtolower($cls->fqcn)])) {
            return true;
        }
        foreach ($cls->ancestors as $anc) {
            if (isset($this->resultable[strtolower($anc->fqcn)])) {
                return true;
            }
        }
        return false;
    }

    /**
     * Pure, `?`-free builtins allowed inside a non-throwing method body (axis-8). The is_* type predicates emit
     * `.is_str()`/match expressions with no `?`. Kept conservative: excludes count() (dispatches ->count()? on a
     * Countable), casts (intval/…), and anything whose emission can propagate a Throw. A wrongly-listed one that
     * emits `?` fails to compile (self-validating) — remove it then.
     * @var array<string, true>
     */
    private const PURE_BUILTINS = [
        'is_string' => true, 'is_int' => true, 'is_integer' => true, 'is_float' => true, 'is_double' => true,
        'is_bool' => true, 'is_array' => true, 'is_object' => true, 'is_null' => true, 'is_scalar' => true,
    ];

    /**
     * Axis-8 (Result-only-where-throws): mark methods that provably cannot throw so their emitted signature
     * is `-> T` instead of `-> Result<T, Throw>` (and their call sites drop the trailing `?`).
     *
     * SOUND FIRST CUT: only PRIVATE, concrete, non-generator methods whose body contains none of a
     * conservative denylist of "may emit `?`" node types. Private is required so there is no interface/override
     * signature-consistency problem — a private method is class-local, its `__Dyn` trait declaration and its impl
     * both read `$m->throws` (so they always agree), and no other class references its signature. The denylist
     * includes every call form (a callee may throw), object construction, throw/try, division/modulo/pow,
     * casts, array-offset access, foreach/match, yield, clone, include, exit, and string
     * concat/interpolation (`__toString` may throw). Anything else keeps `throws = true` (the safe default),
     * so a missed throwing node can only ever leave a method as Result (never a `?`-in-non-Result compile error
     * for a wrongly-flipped one — and if the denylist is still incomplete, the build fails loudly, never silently).
     */
    private function computeThrows(): void
    {
        // Panic-based error model: NO method returns Result — every method returns bare `T`; a PHP `throw` unwinds
        // via php_rt::do_throw and a `try` catch_unwinds it. So mark every method non-throwing. (The name-based
        // fixed-point below is retained but unreachable; kept for history / a possible future Result-where-throws mode.)
        foreach ($this->uniqueClasses() as $model) {
            foreach ($model->methods as $m) {
                $m->throws = false;
            }
        }
        fwrite(STDERR, "[program] computeThrows: panic-mode, all methods non-Result\n");
        return;
        // @phpstan-ignore-next-line (dead code below, retained intentionally)
        $finder = new \PhpParser\NodeFinder();
        // "Base" throwing constructs: emit `?` with no resolvable callee. New_ stays here (a constructor call
        // `X::new(..)?` always propagates — __construct is never flipped). Calls are handled separately below via
        // the call graph.
        $base_deny = [
            \PhpParser\Node\Expr\New_::class,
            // A throw of a Resultable/caught type emits `return Err(..)` (needs Result); a throw of any other type
            // emits php_rt::uncaught(..) (a panic, `!`). computeThrows stays conservative here (any throw -> not
            // flipped) — the panic-vs-Err decision is per-site in throwCode(); Result-elimination for methods whose
            // throws are all non-Resultable is a later refinement keyed on the caught-exceptions set.
            \PhpParser\Node\Expr\Throw_::class,
            \PhpParser\Node\Stmt\Throw_::class,
            \PhpParser\Node\Stmt\TryCatch::class,
            \PhpParser\Node\Expr\Yield_::class,
            \PhpParser\Node\Expr\YieldFrom::class,
            // Only (string) casts can emit `?` (`$obj.to_php_string()?` when the operand is an object). (int)/(float)/
            // (bool)/(array) casts are infallible. Deny only String_ (conservative: also covers scalar (string) which
            // is actually infallible, but the operand type isn't known in this AST pre-pass).
            \PhpParser\Node\Expr\Cast\String_::class,
            // Div/Mod/Pow no longer force Result: php-rt div/imod/intdiv panic on division-by-zero (DivisionByZeroError
            // is never caught) rather than returning Result; pow never was fallible.
            \PhpParser\Node\Expr\BinaryOp\Concat::class,
            \PhpParser\Node\Expr\AssignOp\Concat::class,
            \PhpParser\Node\Expr\ArrayDimFetch::class,
            // Array_ literal IIFE is now emitted non-Result when its chunk has no `?` (ExprTrait::chunked/hasTryOp),
            // and any fallible element call is captured as a name-dep, so Array_ no longer needs a blanket deny.
            \PhpParser\Node\Scalar\InterpolatedString::class,
            \PhpParser\Node\Scalar\Encapsed::class,
            \PhpParser\Node\Stmt\Foreach_::class,
            \PhpParser\Node\Expr\Match_::class,
            \PhpParser\Node\Expr\Clone_::class,
            \PhpParser\Node\Expr\Include_::class,
            \PhpParser\Node\Expr\Exit_::class,
            \PhpParser\Node\Expr\ShellExec::class,
        ];
        // Candidates and the set of lowercase method NAMES each one calls (name-based deps: PHP methods are
        // case-insensitive, and name-consistency forces all methods of a given lc name to share `throws`, so a
        // call's `?` — gated on the resolved callee — always matches the name's throws). A candidate is
        // non-Result iff none of the names it calls is Result.
        /** @var array<int, MethodModel> */
        $candidates = [];
        /** @var array<int, array<string, true>> */
        $name_dep_map = [];
        /** @var array<int, string> */
        $cand_name = [];
        foreach ($this->uniqueClasses() as $model) {
            foreach ($model->methods as $m) {
                // Eligibility: instance + concrete + declared-here + leaf + NO downstream subclass + non-generator
                // + non-enum + not dyn-dispatchable + non-magic. leaf (ClassEmitter line 607) means calls emit
                // rustName() directly (no __impl variant); no-downstream rules out cross-crate override.
                if ($m->declaring !== $model || $m->isStatic() || $m->isAbstract()
                    || !$model->isLeaf() || $model->has_downstream || $model->isEnum()
                    || $m->node === null || $m->node->stmts === null || $m->storage->has_yield
                    || isset(ClassEmitter::DYN_DISPATCH_METHODS[$m->lc()])
                    || str_starts_with($m->lc(), '__')
                ) {
                    continue;
                }
                // Non-private methods must be NON-POLYMORPHIC: not declared in any ancestor/interface (else the
                // shared __Dyn trait / interface signature would mismatch this flipped impl).
                if (!$m->isPrivate()) {
                    $polymorphic = false;
                    foreach ($model->ancestors as $anc) {
                        if ($this->findMethod($anc, $m->lc()) !== null) {
                            $polymorphic = true;
                            break;
                        }
                    }
                    if ($polymorphic) {
                        continue;
                    }
                }
                foreach ($base_deny as $node_class) {
                    if ($finder->findFirstInstanceOf($m->node->stmts, $node_class) !== null) {
                        continue 2;
                    }
                }
                // Collect the lc NAME of every method call. A dynamic name ($o->$v()) is unresolvable. FuncCalls
                // must be infallible builtins (PURE is_* predicates, or SIMPLE-table entries whose return code is
                // not 'S'=Result); any other FuncCall may emit `?` -> unresolvable.
                $name_deps = [];
                $unresolvable = false;
                foreach ($finder->findInstanceOf($m->node->stmts, \PhpParser\Node\Expr\FuncCall::class) as $call) {
                    if (!$call->name instanceof \PhpParser\Node\Name) {
                        $unresolvable = true;
                        break;
                    }
                    $fn_lc = strtolower($call->name->toString());
                    if (!isset(self::PURE_BUILTINS[$fn_lc]) && !Builtins::isInfallibleBuiltin($fn_lc)) {
                        $unresolvable = true;
                        break;
                    }
                }
                if (!$unresolvable) {
                    foreach ([
                        \PhpParser\Node\Expr\MethodCall::class,
                        \PhpParser\Node\Expr\NullsafeMethodCall::class,
                        \PhpParser\Node\Expr\StaticCall::class,
                    ] as $call_class) {
                        foreach ($finder->findInstanceOf($m->node->stmts, $call_class) as $call) {
                            if (!$call->name instanceof \PhpParser\Node\Identifier) {
                                $unresolvable = true;
                                break 2;
                            }
                            $name_deps[strtolower($call->name->name)] = true;
                        }
                    }
                }
                if ($unresolvable) {
                    continue;
                }
                $id = spl_object_id($m);
                $candidates[$id] = $m;
                $name_dep_map[$id] = $name_deps;
                $cand_name[$id] = $m->lc();
            }
        }
        // All lc method names declared by any project method. A call to a name NOT here targets an external/
        // dynamically-dispatched method (emitted as mixed_call/call_method, which always `?`s) -> Result-forcing.
        $all_names = [];
        foreach ($this->uniqueClasses() as $model) {
            foreach ($model->methods as $mm) {
                $all_names[$mm->lc()] = true;
            }
        }
        // Start every candidate non-Result; seed throwing_names with the lc name of every NON-candidate method
        // (they keep the default throws=true). Then a monotonic fixed point: a candidate becomes Result if its own
        // name is throwing (name-consistency), or it calls a name that is throwing / external / dynamically-
        // dispatched (a name in DYN_DISPATCH_METHODS may emit as mixed_call `?` on a mixed receiver). Converges
        // (only false->true).
        foreach ($candidates as $m) {
            $m->throws = false;
        }
        $throwing_names = [];
        foreach ($this->uniqueClasses() as $model) {
            foreach ($model->methods as $mm) {
                if ($mm->throws) {
                    $throwing_names[$mm->lc()] = true;
                }
            }
        }
        do {
            $changed = false;
            foreach ($candidates as $id => $m) {
                if ($m->throws) {
                    continue;
                }
                $result = isset($throwing_names[$cand_name[$id]]);
                if (!$result) {
                    foreach ($name_dep_map[$id] as $d => $_) {
                        if (!isset($all_names[$d]) || isset($throwing_names[$d])
                            || isset(ClassEmitter::DYN_DISPATCH_METHODS[$d])
                        ) {
                            $result = true;
                            break;
                        }
                    }
                }
                if ($result) {
                    $m->throws = true;
                    $throwing_names[$m->lc()] = true;
                    $changed = true;
                }
            }
        } while ($changed);
        $flipped = 0;
        foreach ($candidates as $m) {
            if (!$m->throws) {
                $flipped++;
            }
        }
        fwrite(STDERR, "[program] computeThrows: {$flipped} non-throwing methods (name-based fixed-point)\n");
    }

    /**
     * DIAGNOSTIC for the Path-B partition: connected components of the inheritance graph (extends +
     * implements, project classes only). No dispatch enum may span a crate boundary, so each component
     * must live in one crate; this prints component sizes and which current crates they span (>1 = an
     * OPEN hierarchy under today's split) to judge whether a component-based partition stays RAM-feasible.
     */
    private function reportInheritanceComponents(): void
    {
        $parent = [];
        $find = static function (string $x) use (&$parent, &$find): string {
            while (($parent[$x] ?? $x) !== $x) {
                $parent[$x] = $parent[$parent[$x]] ?? $parent[$x];
                $x = $parent[$x];
            }
            return $x;
        };
        foreach ($this->uniqueClasses() as $m) {
            if (!$m->is_project) {
                continue;
            }
            $parent[$m->fqcn] ??= $m->fqcn;
            foreach ($m->ancestors as $a) {
                if (!$a->is_project) {
                    continue; // external ancestors are not generated enums, so impose no closure
                }
                $parent[$a->fqcn] ??= $a->fqcn;
                $ra = $find($m->fqcn);
                $rb = $find($a->fqcn);
                if ($ra !== $rb) {
                    $parent[$ra] = $rb;
                }
            }
        }
        $comp = [];
        foreach ($this->uniqueClasses() as $m) {
            if (!$m->is_project || !isset($parent[$m->fqcn])) {
                continue;
            }
            $r = $find($m->fqcn);
            $comp[$r]['classes'] = ($comp[$r]['classes'] ?? 0) + 1;
            $comp[$r]['methods'] = ($comp[$r]['methods'] ?? 0) + count($m->methods);
            $comp[$r]['crates'][$m->crate] = true;
        }
        uasort($comp, static fn(array $a, array $b): int => $b['methods'] <=> $a['methods']);
        $open = array_filter($comp, static fn(array $c): bool => count($c['crates']) > 1);
        fwrite(STDERR, sprintf("[components] total=%d, open(span>1 crate)=%d\n", count($comp), count($open)));
        $i = 0;
        foreach ($comp as $root => $c) {
            if (count($c['crates']) > 1 || $i < 15) {
                fwrite(STDERR, sprintf(
                    "[components] %-55s classes=%4d methods=%5d crates={%s}%s\n",
                    $root,
                    $c['classes'],
                    $c['methods'],
                    implode(',', array_keys($c['crates'])),
                    count($c['crates']) > 1 ? '  <-OPEN' : '',
                ));
            }
            $i++;
        }
    }

    /**
     * `static` is forwarded through `self::m()`, `static::m()` and `parent::m()` calls: a static method
     * calling one that uses late static binding needs per-class copies too.
     */
    private function propagateLateStaticBinding(): void
    {
        $finder = new \PhpParser\NodeFinder();
        $calls = [];
        foreach ($this->uniqueClasses() as $model) {
            foreach ($model->methods as $m) {
                if (!$m->isStatic() || $m->node === null || $m->node->stmts === null || $m->declaring !== $model || $m->uses_lsb) {
                    continue;
                }
                $targets = [];
                foreach ($finder->findInstanceOf($m->node->stmts, \PhpParser\Node\Expr\StaticCall::class) as $call) {
                    if (!$call->class instanceof \PhpParser\Node\Name || !$call->name instanceof \PhpParser\Node\Identifier) {
                        continue;
                    }
                    $kind = strtolower($call->class->toString());
                    $target = $kind === 'parent' ? $model->parent : (($kind === 'self' || $kind === 'static') ? $model : null);
                    if ($target === null) {
                        continue;
                    }
                    $tm = $this->findMethod($target, strtolower($call->name->name));
                    if ($tm !== null && $tm->isStatic()) {
                        $targets[] = $tm;
                    }
                }
                if ($targets !== []) {
                    $calls[] = [$m, $targets];
                }
            }
        }
        do {
            $changed = false;
            foreach ($calls as [$m, $targets]) {
                if ($m->uses_lsb) {
                    continue;
                }
                foreach ($targets as $tm) {
                    if ($tm->uses_lsb) {
                        $m->uses_lsb = true;
                        $changed = true;
                        break;
                    }
                }
            }
        } while ($changed);
    }

    /**
     * A class extending a class of another (upstream) crate re-declares the inherited instance methods:
     * their bodies are emitted again with `$this` bound to this class, since the upstream dispatch enum
     * has no variant for it.
     */
    private function importUpstreamMethods(ClassModel $root): void
    {
        $imports = [];
        foreach ($root->methods as $lc => $m) {
            if ($m->isStatic() || $m->isAbstract() || $m->declaring->crate >= $root->crate || !$m->declaring->is_project) {
                continue;
            }
            $imports[$lc] = $m->importedInto($root);
        }
        // private methods of upstream ancestors are called by the imported bodies but are not inherited
        for ($anc = $root->parent; $anc !== null && $anc->is_project && $anc->crate < $root->crate; $anc = $anc->parent) {
            foreach ($anc->methods as $lc => $m) {
                if (isset($root->methods[$lc]) || isset($imports[$lc]) || $m->declaring !== $anc || !$m->isPrivate() || $m->isStatic() || $m->isAbstract()) {
                    continue;
                }
                $imports[$lc] = $m->importedInto($root);
                $root->methods[$lc] = $imports[$lc];
            }
        }
        if ($imports === []) {
            return;
        }
        $apply = function (ClassModel $c) use (&$apply, $imports, $root): void {
            foreach ($imports as $lc => $imp) {
                if (isset($c->methods[$lc]) && $c->methods[$lc] === $imp->import_of) {
                    $c->methods[$lc] = $imp;
                }
            }
            foreach ($c->children as $child) {
                if ($child->crate === $root->crate && $child->parent === $c) {
                    $apply($child);
                }
            }
        };
        $apply($root);
    }

    /** @var array<string, array{ClassModel, MethodModel}> bodies of upstream methods needed for `parent::m()` calls, by copy name */
    public array $super_copies = [];

    /**
     * Name of the copy of upstream method `$m` emitted on `$root`'s handle type (requested by a `parent::m()`
     * call whose target lives in another crate).
     */
    public function requestSuperCopy(ClassModel $root, MethodModel $m): string
    {
        $origin = $m->origin();
        $name = $m->rustName() . '__super_' . Names::classMangle($origin->declaring->fqcn);
        $this->super_copies[$root->lc() . '::' . $name] = [$root, $origin];
        return $name;
    }

    /**
     * @var array<string, array{ClassModel, FieldModel, RustType}> field accessors to emit on a dispatch enum (interface / class
     *   base) for a CONCRETE-VARIANT-SPECIFIC field written/read through it — keyed by enumLc::fieldName. Needed for
     *   the MutableTypeVisitor pattern: `$self = $node; assert($self instanceof TNamedObject); $self->extra_types = v`
     *   where $self's storage type is the TypeNode interface but the field lives on the concrete variant.
     */
    public array $enum_field_accessors = [];

    /** Request set_/get_/mut dispatch for `$field` on the dispatch enum `$enum` (a variant-specific field). */
    public function requestEnumFieldAccessor(ClassModel $enum, FieldModel $field, RustType $type): void
    {
        $this->enum_field_accessors[$enum->lc() . '::' . $field->name] = [$enum, $field, $type];
    }

    /**
     * A field that concrete variants of the hierarchy `$cls` carry (not declared on `$cls` itself): a sample
     * FieldModel (for the accessor name) and the union of the variants' field types, with the dispatch accessor
     * requested; null when no variant has it or the types cannot be joined.
     *
     * @return array{FieldModel, RustType}|null
     */
    public function variantField(ClassModel $cls, string $name): ?array
    {
        if ($cls->isLeaf()) {
            return null;
        }
        $sample = null;
        $types = [];
        foreach ($cls->concrete as $c) {
            $cf = $c->fields[$name] ?? null;
            if ($cf === null) {
                continue;
            }
            $sample ??= $cf;
            $types[] = $cf->type;
        }
        if ($sample === null) {
            return null;
        }
        $t = $this->unionOfRust($types);
        if ($t === null) {
            return null;
        }
        $this->requestEnumFieldAccessor($cls, $sample, $t);
        return [$sample, $t];
    }

    /** @var array<string, true> recursive unions being measured by typeCrate() */
    private array $type_crate_visiting = [];

    /** Index of the crate a generated type (union, shape, ...) is emitted into: the highest crate of any class it mentions. */
    public function typeCrate(RustType $t): int
    {
        $crate = 0;
        if ($t->kind === RustType::CLASS_) {
            $c = $this->classOf($t);
            return $c !== null ? $c->crate : 0;
        }
        if ($t->isRecursive()) {
            // the enum mentions itself through its list member
            if (isset($this->type_crate_visiting[$t->mangle()])) {
                return 0;
            }
            $this->type_crate_visiting[$t->mangle()] = true;
            try {
                foreach ($t->params as $p) {
                    $crate = max($crate, $this->typeCrate($p));
                }
            } finally {
                unset($this->type_crate_visiting[$t->mangle()]);
            }
            return $crate;
        }
        foreach ($t->params as $p) {
            $crate = max($crate, $this->typeCrate($p));
        }
        foreach ($t->fields as [$ft, $_]) {
            $crate = max($crate, $this->typeCrate($ft));
        }
        if ($t->ret !== null) {
            $crate = max($crate, $this->typeCrate($t->ret));
        }
        return $crate;
    }

    /** @var array<string, FileModel|null> includable files by root-relative path (null: known not to be compilable) */
    public array $files = [];

    /** Root-relative form of an absolute path (paths outside the root stay absolute). */
    public function relativePath(string $abs): string
    {
        $root = rtrim($this->transpiler->root_dir, '/') . '/';
        return str_starts_with($abs, $root) ? substr($abs, strlen($root)) : $abs;
    }

    /** Resolve `.`/`..` segments of a path. */
    public static function normalizePath(string $path): string
    {
        $absolute = str_starts_with($path, '/');
        $parts = [];
        foreach (explode('/', $path) as $seg) {
            if ($seg === '' || $seg === '.') {
                continue;
            }
            if ($seg === '..') {
                array_pop($parts);
                continue;
            }
            $parts[] = $seg;
        }
        return ($absolute ? '/' : '') . implode('/', $parts);
    }

    /** Compile the data files listed on the command line and register every project source file as a unit. */
    public function collectFiles(): void
    {
        $root = rtrim($this->transpiler->root_dir, '/');
        foreach ($this->transpiler->data_globs as $glob) {
            foreach (glob(str_starts_with($glob, '/') ? $glob : $root . '/' . $glob) ?: [] as $path) {
                $this->fileForPath($path);
            }
        }
        foreach ($this->transpiler->classes as $record) {
            $this->fileForPath($record->file_path);
        }
        foreach ($this->transpiler->functions as $record) {
            $this->fileForPath($record->file_path);
        }
    }

    /**
     * The includable file at an absolute path: a data file (compiled `return` value) or a unit of
     * declarations; null when the file does not exist or has top-level code that cannot be compiled.
     */
    public function fileForPath(string $abs_path): ?FileModel
    {
        $abs_path = self::normalizePath($abs_path);
        $rel = $this->relativePath($abs_path);
        if (array_key_exists($rel, $this->files)) {
            return $this->files[$rel];
        }
        $this->files[$rel] = null;
        if (!is_file($abs_path)) {
            return null;
        }
        if ($this->transpiler->isDataFile($abs_path)) {
            // a dictionary: compiled mechanically from the value the file returns (see DataEmitter::emitFile)
            $model = new FileModel($rel, $abs_path, $this->transpiler->crateOfFile($abs_path), null, true);
            $this->files[$rel] = $model;
            return $model;
        }
        $stmts = $this->transpiler->file_stmts[$abs_path] ?? null;
        if ($stmts === null) {
            $code = file_get_contents($abs_path);
            if ($code === false) {
                return null;
            }
            try {
                $parsed = (new \PhpParser\ParserFactory())->createForNewestSupportedVersion()->parse($code) ?? [];
            } catch (\PhpParser\Error $e) {
                return null;
            }
            // resolve `use` imports so class constants in data files name their class fully
            $traverser = new \PhpParser\NodeTraverser();
            $traverser->addVisitor(new \PhpParser\NodeVisitor\NameResolver(null, ['preserveOriginalNames' => true, 'replaceNodes' => false]));
            $parsed = $traverser->traverse($parsed);
            $stmts = self::leftoverStatements($parsed);
        }
        $data = null;
        foreach ($stmts as $stmt) {
            if ($stmt instanceof \PhpParser\Node\Stmt\Return_) {
                if ($data !== null || $stmt->expr === null) {
                    return null;
                }
                $data = $stmt->expr;
            } elseif ($stmt instanceof \PhpParser\Node\Stmt\Nop || $stmt instanceof \PhpParser\Node\Stmt\Declare_
                || $stmt instanceof \PhpParser\Node\Stmt\Use_ || $stmt instanceof \PhpParser\Node\Stmt\GroupUse
                || $stmt instanceof \PhpParser\Node\Stmt\InlineHTML
            ) {
                continue;
            } else {
                // top-level code: not compiled
                return null;
            }
        }
        $model = new FileModel($rel, $abs_path, $this->transpiler->crateOfFile($abs_path), $data);
        $this->files[$rel] = $model;
        return $model;
    }

    /**
     * Statements of a parsed file that are not declarations (namespaces are flattened).
     *
     * @param list<\PhpParser\Node\Stmt> $stmts
     * @return list<\PhpParser\Node\Stmt>
     */
    private static function leftoverStatements(array $stmts): array
    {
        $out = [];
        foreach ($stmts as $stmt) {
            if ($stmt instanceof \PhpParser\Node\Stmt\Namespace_) {
                foreach (self::leftoverStatements($stmt->stmts ?? []) as $inner) {
                    $out[] = $inner;
                }
                continue;
            }
            if ($stmt instanceof \PhpParser\Node\Stmt\ClassLike || $stmt instanceof \PhpParser\Node\Stmt\Function_) {
                continue;
            }
            $out[] = $stmt;
        }
        return $out;
    }

    /** Index of the crate a function or constant is emitted into. */
    public function crateOfRecord(FunctionRecord $record): int
    {
        return $this->transpiler->crateOfFile($record->file_path);
    }

    /** @param list<ClassModel> $models */
    private function dedupe(array $models): array
    {
        $seen = [];
        $out = [];
        foreach ($models as $m) {
            $id = spl_object_id($m);
            if (!isset($seen[$id])) {
                $seen[$id] = true;
                $out[] = $m;
            }
        }
        return $out;
    }

    private function linkAncestors(ClassModel $model): void
    {
        if ($model->linked) {
            return;
        }
        $model->linked = true;
        $storage = $model->storage;
        $seen = [];
        if ($storage->parent_class !== null) {
            $parent = $this->getClass($storage->parent_class);
            if ($parent !== null) {
                $model->parent = $parent;
                $parent->children[] = $model;
                $this->linkAncestors($parent);
                $seen[$parent->lc()] = $parent;
                foreach ($parent->ancestors as $a) {
                    $seen[$a->lc()] = $a;
                }
            }
        }
        $interfaces = $storage->is_interface ? $storage->parent_interfaces : $storage->class_implements;
        foreach ($interfaces as $lc_iface => $_) {
            $iface = $this->getClass($lc_iface);
            if ($iface === null || $iface === $model || isset($seen[$iface->lc()])) {
                continue;
            }
            $this->linkAncestors($iface);
            $seen[$iface->lc()] = $iface;
            foreach ($iface->ancestors as $a) {
                $seen[$a->lc()] = $a;
            }
        }
        $model->ancestors = array_values($seen);
        // direct interface parents count as children links for dispatch enums
        $direct = $storage->is_interface ? $storage->direct_interface_parents : $storage->direct_class_interfaces;
        foreach ($direct as $lc_iface => $_) {
            $iface = $this->getClass($lc_iface);
            if ($iface !== null && $iface !== $model) {
                $iface->children[] = $model;
            }
        }
    }

    /** Get or lazily create a class model (external classes are created on demand). */
    public function getClass(string $fqcn): ?ClassModel
    {
        $lc = strtolower($this->codebase->classlikes->getUnAliasedName(ltrim($fqcn, '\\')));
        if (isset($this->classes[$lc])) {
            return $this->classes[$lc];
        }
        if (!$this->codebase->classlike_storage_provider->has($lc)) {
            return null;
        }
        $storage = $this->codebase->classlike_storage_provider->get($lc);
        $alias_of = $this->aliasTarget($storage);
        if ($alias_of !== null) {
            $target = $this->getClass($alias_of);
            if ($target !== null) {
                $this->classes[$lc] = $target;
                return $target;
            }
        }
        $model = new ClassModel($storage->name, $storage, null, false);
        $this->classes[$lc] = $model;
        $this->linkAncestors($model);
        return $model;
    }

    /**
     * Classes declared in project files that were never analyzed (e.g. `if (false) { class X extends Y {} }`
     * compatibility shims) are treated as aliases of their parent.
     */
    private function aliasTarget(ClassLikeStorage $storage): ?string
    {
        if ($storage->parent_class === null || $storage->location === null) {
            return null;
        }
        if (!$this->transpiler->config->isInProjectDirs($storage->location->file_path)) {
            return null;
        }
        if (isset($this->transpiler->classes[strtolower($storage->name)])) {
            return null;
        }
        foreach ($storage->methods as $m) {
            if ($m->defining_fqcln === null || strtolower($m->defining_fqcln) === strtolower($storage->name)) {
                return null;
            }
        }
        if ($storage->properties !== []) {
            return null;
        }
        return $storage->parent_class;
    }

    public function canonicalClassName(string $name): string
    {
        $name = $this->codebase->classlikes->getUnAliasedName(ltrim($name, '\\'));
        $lc = strtolower($name);
        if (isset($this->classes[$lc])) {
            return $this->classes[$lc]->fqcn;
        }
        $model = $this->getClass($name);
        if ($model !== null) {
            return $model->fqcn;
        }
        return ltrim($name, '\\');
    }

    private function buildFields(ClassModel $model): void
    {
        // inherited first (parent chain), preserving declaration order
        if ($model->parent !== null) {
            $this->buildFields($model->parent);
            foreach ($model->parent->fields as $name => $field) {
                $model->fields[$name] = $field;
            }
        }
        // types are mapped in the class' own crate context (this may run for a parent from a subclass' build)
        $this->types->current_class = $model->fqcn;
        $this->types->current_crate = $model->crate;
        $storage = $model->storage;
        $properties = $storage->properties;
        // properties brought in by used traits are not copied into the class' own property storage
        foreach ($storage->declaring_property_ids as $name => $declaring_id) {
            if (isset($properties[$name]) || isset($model->fields[$name])) {
                continue;
            }
            $trait = $this->getClass($declaring_id);
            if ($trait !== null && $trait->isTrait() && isset($trait->storage->properties[$name])) {
                $properties[$name] = $trait->storage->properties[$name];
            }
        }
        foreach ($properties as $name => $prop_storage) {
            $declaring_id = $storage->declaring_property_ids[$name] ?? $model->fqcn;
            $declaring = $this->getClass($declaring_id) ?? $model;
            if ($declaring !== $model && !$storage->is_trait) {
                // trait property: treat the using class as declaring for emission purposes
                $declaring_storage = $declaring->storage;
                if (!$declaring_storage->is_trait) {
                    continue;
                }
            }
            $default = null;
            $has_default = $prop_storage->has_default;
            $node_class = $declaring->is_project ? $declaring : $model;
            if ($node_class->node !== null) {
                $declared = (bool) $prop_storage->is_promoted;
                foreach ($node_class->node->getProperties() as $pnode) {
                    foreach ($pnode->props as $item) {
                        if ($item->name->name === $name) {
                            $declared = true;
                            $default = $item->default;
                            $has_default = $has_default || $item->default !== null;
                        }
                    }
                }
                if (!$declared && $declaring === $model && !$storage->is_trait) {
                    // declared only by another definition of the class (Psalm's own stub of a builtin class
                    // that a runtime stub replaces): not part of the generated struct
                    continue;
                }
            }
            if (isset($model->fields[$name]) && !$prop_storage->is_static) {
                // redeclared inherited property: keep the root declaration's type
                $root = $model->fields[$name];
                $model->fields[$name] = new FieldModel(
                    $name,
                    $root->type,
                    $root->declaring,
                    $root->storage,
                    $default ?? $root->default,
                    $has_default || $root->has_default,
                    false,
                );
                continue;
            }
            $this->types->context = $model->fqcn . '::$' . $name;
            if (($probe = getenv('TRANSPILE_PROP_PROBE')) !== false && $probe !== ''
                && stripos($model->fqcn . '::$' . $name, $probe) !== false
            ) {
                fwrite(STDERR, '[prop-probe] ' . $model->fqcn . '::$' . $name . ' = '
                    . ($prop_storage->type === null ? 'null' : $prop_storage->type->getId())
                    . ' @ ' . ($prop_storage->location?->file_path ?? 'none') . "\n");
            }
            $field = new FieldModel(
                $name,
                $this->types->map($prop_storage->type),
                $model,
                $prop_storage,
                $default,
                $has_default,
                (bool) $prop_storage->is_static,
            );
            if ($prop_storage->is_static) {
                $model->static_fields[$name] = $field;
            } else {
                $model->fields[$name] = $field;
            }
        }
        $this->widenFieldsByAssignments($model);
    }

    /**
     * A property whose docblock omits `null` but which the class itself assigns a nullable value
     * (`/** @var Stmt[] *\/ public $stmts;` set from a `?array` parameter) becomes nullable, so that
     * `null` survives instead of turning into a default value.
     */
    private function widenFieldsByAssignments(ClassModel $model): void
    {
        if ($model->node === null) {
            return;
        }
        $finder = new \PhpParser\NodeFinder();
        foreach ($model->node->getMethods() as $mnode) {
            $record = $this->transpiler->getFunctionRecord($mnode, $model->fqcn);
            if ($record === null || $mnode->stmts === null) {
                continue;
            }
            foreach ($finder->findInstanceOf($mnode->stmts, \PhpParser\Node\Expr\Assign::class) as $assign) {
                $target = $assign->var;
                if (!$target instanceof \PhpParser\Node\Expr\PropertyFetch
                    || !$target->var instanceof \PhpParser\Node\Expr\Variable
                    || $target->var->name !== 'this'
                    || !$target->name instanceof \PhpParser\Node\Identifier
                ) {
                    continue;
                }
                $name = $target->name->name;
                $field = $model->fields[$name] ?? null;
                if ($field === null || $field->declaring !== $model || $field->type->kind === RustType::OPTION || $field->type->kind === RustType::MIXED) {
                    continue;
                }
                $assigned = $record->node_data->getType($assign->expr);
                if ($assigned === null || !$assigned->isNullable()) {
                    continue;
                }
                $declared = $field->storage->type;
                if ($declared !== null && !$assigned->isNull()) {
                    // a narrowing assignment (a wide union stored into a specifically typed property, as a
                    // generic setter does) converts at the assignment and fails on null there, like PHP's own
                    // property type check would; only a value that differs from the property type by null
                    // alone widens the field
                    $builder = $assigned->getBuilder();
                    $builder->removeType('null');
                    $without_null = $builder->freeze();
                    if (!\Psalm\Internal\Type\Comparator\UnionTypeComparator::isContainedBy($this->codebase, $without_null, $declared)) {
                        continue;
                    }
                }
                $model->fields[$name] = new FieldModel(
                    $field->name,
                    RustType::option($field->type),
                    $field->declaring,
                    $field->storage,
                    $field->default,
                    $field->has_default,
                    $field->is_static,
                );
            }
        }
    }

    private function buildConstants(ClassModel $model): void
    {
        $exprs = [];
        if ($model->node !== null) {
            foreach ($model->node->getConstants() as $cnode) {
                foreach ($cnode->consts as $c) {
                    $exprs[$c->name->name] = $c->value;
                }
            }
        }
        foreach ($model->storage->constants as $name => $cstorage) {
            if (!isset($exprs[$name]) && $model->is_project) {
                // inherited from an interface/parent; skip, the declaring class emits it
                continue;
            }
            $type = $cstorage->type ?? $cstorage->inferred_type;
            $this->types->context = $model->fqcn . '::' . $name;
            $model->constants[$name] = new ConstModel(
                $name,
                $this->types->map($type),
                $model,
                $cstorage,
                $exprs[$name] ?? null,
            );
        }
    }

    private function buildMethods(ClassModel $model): void
    {
        foreach ($model->storage->declaring_method_ids as $lc_name => $declaring_id) {
            $method = $this->methodFor($model, $lc_name, $declaring_id);
            if ($method !== null) {
                $model->methods[$lc_name] = $method;
            }
        }
        // abstract classes: methods of implemented interfaces are dispatched too
        if (!$model->isConcrete() && !$model->isInterface()) {
            foreach ($model->ancestors as $iface) {
                if (!$iface->isInterface()) {
                    continue;
                }
                foreach ($iface->storage->methods as $lc_name => $_) {
                    if (isset($model->methods[$lc_name])) {
                        continue;
                    }
                    $method = $this->methodFor($model, $lc_name, new MethodIdentifier($iface->fqcn, $lc_name));
                    if ($method !== null) {
                        $model->methods[$lc_name] = $method;
                    }
                }
            }
        }
    }

    private function methodFor(ClassModel $model, string $lc_name, MethodIdentifier $declaring_id): ?MethodModel
    {
        $declaring = $this->getClass($declaring_id->fq_class_name);
        if ($declaring === null) {
            return null;
        }
        $body_owner = $declaring;
        if ($declaring->isTrait()) {
            // trait methods are emitted into the class that uses the trait; its subclasses inherit them
            $body_owner = $model;
            $trait_lc = strtolower($declaring->fqcn);
            for ($c = $model; $c !== null; $c = $c->parent) {
                if (isset($c->storage->used_traits[$trait_lc])) {
                    $body_owner = $c;
                    break;
                }
            }
        }
        $key = $body_owner->lc() . '::' . $lc_name;
        if (isset($this->method_cache[$key])) {
            return $this->method_cache[$key];
        }
        $storage = $declaring->storage->methods[$lc_name] ?? null;
        if ($storage === null) {
            return null;
        }
        $node = null;
        if ($declaring->node !== null) {
            foreach ($declaring->node->getMethods() as $mnode) {
                if (strtolower($mnode->name->name) === $lc_name) {
                    $node = $mnode;
                }
            }
        }
        if ($node === null && $declaring->is_project && !$declaring->isInterface()) {
            // leftover from Psalm's own stub of an overridden builtin class
            return null;
        }
        $record = $node !== null ? $this->transpiler->getFunctionRecord($node, $body_owner->fqcn) : null;
        $method = new MethodModel($storage->cased_name ?? $lc_name, $body_owner, $storage, $node, $record);
        if ($storage->is_static && $node !== null && $node->stmts !== null) {
            $method->uses_lsb = (new \PhpParser\NodeFinder())->findFirst($node->stmts, static function (\PhpParser\Node $n): bool {
                $class = null;
                if ($n instanceof \PhpParser\Node\Expr\New_ || $n instanceof \PhpParser\Node\Expr\StaticCall
                    || $n instanceof \PhpParser\Node\Expr\ClassConstFetch || $n instanceof \PhpParser\Node\Expr\StaticPropertyFetch
                    || $n instanceof \PhpParser\Node\Expr\Instanceof_
                ) {
                    $class = $n->class;
                }
                return $class instanceof \PhpParser\Node\Name && strtolower($class->toString()) === 'static';
            }) !== null;
        }
        $this->method_cache[$key] = $method;
        $saved = $this->types->current_class;
        $saved_crate = $this->types->current_crate;
        $this->types->current_class = $body_owner->fqcn;
        $this->types->current_crate = $body_owner->crate;
        $this->resolveSignature($storage, $method->param_types, $method->return_type, $method->node);
        $method->generics = $this->last_generics;
        $this->types->generic_names = [];
        $this->types->current_class = $saved;
        $this->types->current_crate = $saved_crate;
        // by-reference parameters must keep the type of the root declaration so dispatch signatures agree
        foreach ($storage->params as $i => $p) {
            if (!$p->by_ref) {
                continue;
            }
            foreach ($model->storage->overridden_method_ids[$lc_name] ?? $declaring->storage->overridden_method_ids[$lc_name] ?? [] as $overridden) {
                $parent_cls = $this->getClass($overridden->fq_class_name);
                $pm = $parent_cls !== null ? $this->findMethod($parent_cls, $lc_name) : null;
                if ($pm !== null && isset($pm->param_types[$i])) {
                    $method->param_types[$i] = $pm->param_types[$i];
                    break;
                }
            }
        }
        $this->computeMethodBorrowParams($method);
        $this->method_cache[$key] = $method;
        return $method;
    }

    /**
     * A shape-typed parameter whose body reads keys the docblock doesn't declare (`$subNodes['type']`
     * next to `array{flags?: int, ...}`) gets those keys as optional `Mixed` fields, so that call sites
     * passing them keep the values instead of dropping them as unknown.
     */
    private function extendShapeWithBodyReads(RustType $t, string $param_name, \PhpParser\Node\FunctionLike $node): RustType
    {
        $shape = $t->kind === RustType::OPTION ? $t->inner() : $t;
        if ($shape->kind !== RustType::SHAPE) {
            return $t;
        }
        $stmts = $node->getStmts();
        if ($stmts === null) {
            return $t;
        }
        $extra = [];
        foreach ((new \PhpParser\NodeFinder())->findInstanceOf($stmts, \PhpParser\Node\Expr\ArrayDimFetch::class) as $fetch) {
            if (!$fetch->var instanceof \PhpParser\Node\Expr\Variable || $fetch->var->name !== $param_name) {
                continue;
            }
            $key = null;
            if ($fetch->dim instanceof \PhpParser\Node\Scalar\String_) {
                $key = $fetch->dim->value;
            } elseif ($fetch->dim instanceof \PhpParser\Node\Scalar\Int_) {
                $key = (string) $fetch->dim->value;
            }
            if ($key === null || isset($shape->fields[$key]) || isset($extra[$key])) {
                continue;
            }
            $extra[$key] = [RustType::mixed(), true];
        }
        if ($extra === []) {
            return $t;
        }
        $shape = RustType::shape($shape->fields + $extra);
        $this->types->shapes[$shape->mangle()] = $shape;
        return $t->kind === RustType::OPTION ? RustType::option($shape) : $shape;
    }

    /**
     * A shape-typed return whose body returns array literals with keys the docblock doesn't list
     * (`@return array{nodeType: string, ...}` next to `return [..., 'endLine' => ...]`) gets those keys
     * as optional `Mixed` fields so the values survive.
     */
    private function extendShapeWithReturnedKeys(RustType $t, \PhpParser\Node\FunctionLike $node): RustType
    {
        $stmts = $node->getStmts();
        if ($stmts === null) {
            return $t;
        }
        $literals = [];
        foreach ((new \PhpParser\NodeFinder())->findInstanceOf($stmts, \PhpParser\Node\Stmt\Return_::class) as $ret) {
            if ($ret->expr instanceof \PhpParser\Node\Expr\Array_) {
                $literals[] = $ret->expr;
            }
        }
        return $this->extendShapeWithLiterals($t, $literals);
    }

    /**
     * Extend the shape(s) in `$t` with the keys of the array literals `$literals` (data-provider style
     * `array<string, array{...}>` returns are extended one level down, from the nested literals).
     *
     * @param list<\PhpParser\Node\Expr\Array_> $literals
     */
    private function extendShapeWithLiterals(RustType $t, array $literals): RustType
    {
        if ($literals === []) {
            return $t;
        }
        if ($t->kind === RustType::OPTION) {
            $inner = $this->extendShapeWithLiterals($t->inner(), $literals);
            return $inner === $t->inner() ? $t : RustType::option($inner);
        }
        if ($t->kind === RustType::LIST || $t->kind === RustType::MAP) {
            $vt = $t->kind === RustType::LIST ? $t->inner() : $t->params[1];
            $nested = [];
            foreach ($literals as $lit) {
                foreach ($lit->items as $item) {
                    if ($item->value instanceof \PhpParser\Node\Expr\Array_) {
                        $nested[] = $item->value;
                    }
                }
            }
            $new = $this->extendShapeWithLiterals($vt, $nested);
            if ($new === $vt) {
                return $t;
            }
            return $t->kind === RustType::LIST ? RustType::list($new) : RustType::map($t->params[0], $new);
        }
        if ($t->kind !== RustType::SHAPE) {
            return $t;
        }
        $extra = [];
        foreach ($literals as $lit) {
            foreach ($lit->items as $item) {
                $key = null;
                if ($item->key instanceof \PhpParser\Node\Scalar\String_) {
                    $key = $item->key->value;
                } elseif ($item->key instanceof \PhpParser\Node\Scalar\Int_) {
                    $key = (string) $item->key->value;
                }
                if ($key === null || isset($t->fields[$key]) || isset($extra[$key])) {
                    continue;
                }
                $extra[$key] = [RustType::mixed(), true];
            }
        }
        if ($extra === []) {
            return $t;
        }
        $shape = RustType::shape($t->fields + $extra);
        $this->types->shapes[$shape->mangle()] = $shape;
        return $shape;
    }

    /**
     * @param list<RustType> $param_types
     */
    /**
     * Owned/borrowed (axis 5): mark free-function params that are non-escaping read-only, so they can be
     * emitted as `&T` and callers borrow instead of cloning. Conservative: a param qualifies only if its
     * type is a heap object/union (CLASS_/UNION -- where an Rc clone actually costs) and EVERY occurrence
     * of it in the body is a borrowing read (method-call receiver, property-fetch base, or instanceof
     * operand). Any other use (return, store, pass-by-value, reassignment, closure capture, by-ref) makes
     * it escape -> keep it owned. rustc is the backstop: an unsound borrow would fail to compile.
     */
    private function computeBorrowParams(FunctionModel $fn): void
    {
        $node = $fn->record->node;
        if (!$node instanceof \PhpParser\Node\Stmt\Function_) {
            return;
        }
        $fn->borrow_params = $this->borrowSafeParams($node->stmts, $node->params, $fn->param_types);
    }

    /**
     * Owned/borrowed (axis 5): mark a PRIVATE method's non-escaping read-only params as `&T`. Restricted to
     * private methods because they have a single implementation and are never dispatched, so the borrowed
     * signature can't diverge from another variant/override the way a public/protected method's could.
     */
    /**
     * Owned/borrowed (axis 5): per-body escape analysis for one method. Records which params THIS body could
     * receive as `&T` into $local_borrow; the final $borrow_params is decided later by computeBorrowAgreement,
     * which intersects local_borrow across each dispatch group so overriders keep a uniform signature.
     * Excludes constructors and immutable construction helpers (reached via new/super-copy with owned args)
     * and static methods (dispatched through the `__static` variant machinery).
     */
    private function computeMethodBorrowParams(MethodModel $method): void
    {
        if ($method->node === null || $method->isAbstract()) {
            return;
        }
        // Non-private static methods are reachable through the `__static` / late-static-binding dispatch
        // variants (uniform signature required across the hierarchy); only private statics are single-impl.
        if ($method->isStatic() && !$method->isPrivate()) {
            return;
        }
        $lc = $method->lc();
        if ($lc === '__construct' || $lc === 'magic__construct'
            || isset($method->declaring->constructionMethods()[$lc])
        ) {
            return;
        }
        $method->local_borrow = $this->borrowSafeParams($method->node->stmts ?? [], $method->node->params, $method->param_types);
        if ($method->isStatic()) {
            // private static: single implementation, not in the instance-method agreement pass -> finalise now.
            $method->borrow_params = $method->local_borrow;
        }
    }

    /**
     * Owned/borrowed (axis 5): finalise every instance method's $borrow_params. A method that participates in
     * dispatch (an interface/abstract method plus all its concrete implementations, or a base method plus its
     * overriders) MUST keep a uniform signature, so a param is borrowed only if EVERY implementation in the
     * group could borrow it locally. We union all methods that share a dispatch point, then per group borrow
     * param i iff all body-carrying members have local_borrow[i]; the agreed set is applied to the whole group
     * (a bodyless interface/abstract method receives the group's decision). Ungrouped methods (private,
     * leaf-own non-overriding) form singleton groups and simply keep their local analysis.
     */
    private function computeBorrowAgreement(): void
    {
        $parent = [];
        $models = [];
        $find = function (int $x) use (&$parent, &$find): int {
            while ($parent[$x] !== $x) {
                $parent[$x] = $parent[$parent[$x]] ?? $parent[$x];
                $x = $parent[$x];
            }
            return $x;
        };
        $add = function (MethodModel $m) use (&$parent, &$models): int {
            $id = spl_object_id($m);
            if (!isset($parent[$id])) {
                $parent[$id] = $id;
                $models[$id] = $m;
            }
            return $id;
        };
        $union = function (MethodModel $a, MethodModel $b) use ($add, $find, &$parent): void {
            $ra = $find($add($a));
            $rb = $find($add($b));
            if ($ra !== $rb) {
                $parent[$ra] = $rb;
            }
        };
        foreach ($this->uniqueClasses() as $cls) {
            foreach ($cls->methods as $m) {
                if (!$m->isStatic()) {
                    $add($m);
                }
            }
            // A class dispatched on (interface/abstract root, or a base with subclasses) routes each instance
            // method NAME to its concrete implementations -- those must share one signature.
            if ($cls->concrete === [] && $cls->children === []) {
                continue;
            }
            foreach ($cls->methods as $m) {
                if ($m->isStatic() || $m->isPrivate() || $m->lc() === '__construct' || $m->lc() === 'magic__construct') {
                    continue;
                }
                foreach ($cls->concrete as $c) {
                    $cm = $this->findMethod($c, $m->lc());
                    if ($cm !== null && !$cm->isStatic()) {
                        $union($m, $cm);
                    }
                }
            }
        }
        $groups = [];
        foreach ($models as $id => $m) {
            $groups[$find($id)][] = $m;
        }
        foreach ($groups as $group) {
            $agreed = null;
            $has_body = false;
            foreach ($group as $m) {
                if ($m->node === null || $m->isAbstract()) {
                    continue; // interface/abstract declaration: receives the decision, doesn't constrain it
                }
                $has_body = true;
                $agreed = $agreed === null ? $m->local_borrow : array_intersect_key($agreed, $m->local_borrow);
                if ($agreed === []) {
                    break;
                }
            }
            if (!$has_body || $agreed === null || $agreed === []) {
                continue;
            }
            // A param may be borrowed only if its type is IDENTICAL across every member: dispatch forwards the
            // arg by reference with no conversion, so a member that narrows/widens param i (its enum arm would
            // cast, which can't apply to a &T) must veto the borrow of i.
            foreach (array_keys($agreed) as $i) {
                $ty = null;
                foreach ($group as $m) {
                    $mt = $m->param_types[$i] ?? null;
                    if ($mt === null) {
                        unset($agreed[$i]); // arity mismatch across the group
                        break;
                    }
                    $r = $mt->toRust();
                    if ($ty === null) {
                        $ty = $r;
                    } elseif ($ty !== $r) {
                        unset($agreed[$i]);
                        break;
                    }
                }
            }
            if ($agreed === []) {
                continue;
            }
            foreach ($group as $m) {
                $m->borrow_params = $agreed;
            }
        }
    }

    /**
     * The escape-analysis core shared by free functions and private methods: a param qualifies as borrow-safe
     * (`&T`) when its type is a heap object/union (CLASS_/UNION) and EVERY occurrence of it in the body is a
     * borrowing read -- a method-call receiver or property-fetch base. Any other use (return, store, compare,
     * pass-by-value, reassignment, closure capture, by-ref, instanceof-narrowing) makes it escape -> owned.
     * @param list<\PhpParser\Node\Stmt> $stmts
     * @param list<\PhpParser\Node\Param> $params
     * @param list<RustType> $param_types
     * @return array<int, true>
     */
    private function borrowSafeParams(array $stmts, array $params, array $param_types): array
    {
        $borrow = [];
        $finder = new \PhpParser\NodeFinder();
        // Variable nodes that are a borrowing read (safe): the receiver/base/operand of a read expression.
        $safe = [];
        $recv_method = []; // receiver Variable node-id => lowercase method name (for static-resolvability check)
        foreach ($finder->findInstanceOf($stmts, \PhpParser\Node\Expr\MethodCall::class) as $m) {
            // A first-class callable `$x->m(...)` CAPTURES the receiver into a closure that may be stored/
            // escape ('static), so its receiver is NOT a plain borrow.
            if ($m->var instanceof \PhpParser\Node\Expr\Variable && !$m->isFirstClassCallable()) {
                $safe[spl_object_id($m->var)] = true;
                if ($m->name instanceof \PhpParser\Node\Identifier) {
                    $recv_method[spl_object_id($m->var)] = strtolower($m->name->name);
                }
            }
        }
        foreach ($finder->findInstanceOf($stmts, \PhpParser\Node\Expr\NullsafeMethodCall::class) as $m) {
            if ($m->var instanceof \PhpParser\Node\Expr\Variable) {
                $safe[spl_object_id($m->var)] = true;
            }
        }
        foreach ($finder->findInstanceOf($stmts, \PhpParser\Node\Expr\PropertyFetch::class) as $p) {
            if ($p->var instanceof \PhpParser\Node\Expr\Variable) {
                $safe[spl_object_id($p->var)] = true;
            }
        }
        foreach ($finder->findInstanceOf($stmts, \PhpParser\Node\Expr\NullsafePropertyFetch::class) as $p) {
            if ($p->var instanceof \PhpParser\Node\Expr\Variable) {
                $safe[spl_object_id($p->var)] = true;
            }
        }
        // NB: `instanceof` is deliberately NOT a safe use: the transpiler inserts a narrowing cast
        // (cast::<Sub>($p)) after it, which needs an owned value -> would not typecheck on a `&T`.
        // A property WRITE ($p->x = ...) reads the base too, but mutating the borrow is not what we allow;
        // exclude params whose base is written.
        $written_base = [];
        foreach ($finder->findInstanceOf($stmts, \PhpParser\Node\Expr\Assign::class) as $a) {
            $tgt = $a->var;
            if (($tgt instanceof \PhpParser\Node\Expr\PropertyFetch || $tgt instanceof \PhpParser\Node\Expr\ArrayDimFetch)
                && $tgt->var instanceof \PhpParser\Node\Expr\Variable && is_string($tgt->var->name)
            ) {
                $written_base[$tgt->var->name] = true;
                unset($safe[spl_object_id($tgt->var)]);
            }
        }
        foreach ($params as $i => $p) {
            if ($p->byRef || $p->variadic || !$p->var instanceof \PhpParser\Node\Expr\Variable || !is_string($p->var->name)) {
                continue;
            }
            $t = $param_types[$i] ?? null;
            if ($t === null || ($t->kind !== RustType::CLASS_ && $t->kind !== RustType::UNION)) {
                continue; // only heap objects/unions are worth borrowing
            }
            $name = $p->var->name;
            if (isset($written_base[$name])) {
                continue;
            }
            // Classes the param type can be at runtime, for the method-resolvability check below.
            $recv_classes = $t->kind === RustType::CLASS_
                ? [$this->classOf($t)]
                : array_map(fn(RustType $m) => $m->kind === RustType::CLASS_ ? $this->classOf($m) : null, $t->params);
            $all_safe = true;
            foreach ($finder->findInstanceOf($stmts, \PhpParser\Node\Expr\Variable::class) as $v) {
                if (!is_string($v->name) || $v->name !== $name) {
                    continue;
                }
                $oid = spl_object_id($v);
                if (!isset($safe[$oid])) {
                    $all_safe = false;
                    break;
                }
                // A method-call receiver escapes into Mixed when the method is NOT statically resolvable on the
                // param's type (unmodeled builtin like DOMDocument::getElementsByTagNameNS -> the dynamic
                // call_method path casts the receiver to Mixed, which cannot apply to a &T). Require resolution
                // on every possible runtime class.
                if (isset($recv_method[$oid])) {
                    $lcm = $recv_method[$oid];
                    foreach ($recv_classes as $rc) {
                        if ($rc === null || $this->findMethod($rc, $lcm) === null) {
                            $all_safe = false;
                            break 2;
                        }
                    }
                }
            }
            if ($all_safe) {
                $borrow[$i] = true;
            }
        }
        return $borrow;
    }

    /**
     * Whether an overridden method's docblock type is at least as precise as the override's own native type
     * (an interface's `@return mixed` must not replace an implementation's native `?DOMNode`).
     */
    private function refines(Union $inherited, ?Union $native): bool
    {
        if ($native === null || $native->isMixed()) {
            return true;
        }
        try {
            return \Psalm\Internal\Type\Comparator\UnionTypeComparator::isContainedBy($this->codebase, $inherited, $native);
        } catch (\Throwable) {
            return false;
        }
    }

    /** Whether a declared type carries no docblock information beyond the native signature type. */
    private static function isNativeOnly(?Union $type, ?Union $signature_type): bool
    {
        if ($type === null) {
            return true;
        }
        if ($signature_type === null) {
            return false; // docblock-only type
        }
        return $type->getId() === $signature_type->getId();
    }

    /** The generics of the signature resolved last (a side result of resolveSignature). @var array<string, string> */
    private array $last_generics = [];

    private function resolveSignature(FunctionLikeStorage $storage, array &$param_types, RustType &$return_type, ?\PhpParser\Node $node = null): void
    {
        // Rust generics: an unbounded fn-level @template becomes a generic parameter for single-implementation
        // callees (free functions, static and private methods); dispatched methods keep the Mixed mapping.
        $generic_ok = !($storage instanceof MethodStorage) || $storage->is_static
            || $storage->visibility === \Psalm\Internal\Analyzer\ClassLikeAnalyzer::VISIBILITY_PRIVATE
            || $storage->final
            || ($storage->defining_fqcln !== null && $this->codebase->classlike_storage_provider->has($storage->defining_fqcln)
                && $this->codebase->classlike_storage_provider->get($storage->defining_fqcln)->final);
        $this->last_generics = $generic_ok ? TypeMapper::genericNamesOf($storage) : [];
        if ($generic_ok && $storage instanceof MethodStorage && $storage->defining_fqcln !== null
            && $this->codebase->classlike_storage_provider->has($storage->defining_fqcln)
        ) {
            // the class-level templates of a final class (no Rust struct generics exist for them) become generics
            // of each method whose signature mentions them: `Future<T>::await(): T` returns a G_T the call site
            // narrows to Psalm's resolved type
            $class_storage = $this->codebase->classlike_storage_provider->get($storage->defining_fqcln);
            if ($class_storage->final && !$class_storage->abstract) {
                foreach ($class_storage->template_types ?? [] as $name => $defs) {
                    foreach ($defs as $bound) {
                        if ($bound->isMixed() && !isset($this->last_generics[$name])) {
                            $this->last_generics[$name] = 'G_' . preg_replace('/[^A-Za-z0-9_]/', '_', $name);
                        }
                    }
                }
            }
        }
        $this->types->generic_names = $this->last_generics;
        $param_types = [];
        $fn = ($storage instanceof \Psalm\Storage\MethodStorage && $storage->defining_fqcln !== null ? $storage->defining_fqcln . '::' : '') . ($storage->cased_name ?? '{closure}');
        // Overriding methods without their own docblock inherit the overridden method's docblock types (the
        // types Psalm itself uses at call sites), e.g. a test's `provider(): iterable` under a trait's
        // `@return iterable<string, array{...}>` — otherwise the native `iterable`/`array` maps to Mixed.
        $inherited = [];
        if ($storage instanceof \Psalm\Storage\MethodStorage && $storage->defining_fqcln !== null && $storage->cased_name !== null) {
            $lc_own = strtolower($storage->cased_name);
            try {
                $cls_storage = $this->codebase->classlike_storage_provider->get($storage->defining_fqcln);
                foreach ($cls_storage->overridden_method_ids[$lc_own] ?? [] as $mid) {
                    try {
                        $inherited[] = $this->codebase->methods->getStorage($mid);
                    } catch (\Throwable) {
                    }
                }
            } catch (\Throwable) {
            }
        }
        foreach ($storage->params as $param) {
            $this->types->context = $fn . '() param $' . $param->name;
            $ptype = $param->type;
            if (self::isNativeOnly($ptype, $param->signature_type)) {
                foreach ($inherited as $inh) {
                    foreach ($inh->params as $ip) {
                        if ($ip->name === $param->name && $ip->type !== null && !self::isNativeOnly($ip->type, $ip->signature_type)
                            && $this->refines($ip->type, $ptype)
                        ) {
                            $ptype = $ip->type;
                            break 2;
                        }
                    }
                }
            }
            if ($param->by_ref && $param->out_type !== null && $ptype !== null) {
                // the caller sees whatever `@param-out` says the call leaves behind, so the reference
                // has to hold that too: a narrower declared type would reject the assignment
                $ptype = \Psalm\Type::combineUnionTypes($ptype, $param->out_type, $this->codebase);
            }
            $t = $this->types->map($ptype);
            if ($param->is_variadic) {
                $t = RustType::list($t);
            }
            if ($node instanceof \PhpParser\Node\FunctionLike) {
                $t = $this->extendShapeWithBodyReads($t, $param->name, $node);
            }
            $param_types[] = $t;
        }
        $this->types->context = $fn . '() return';
        // magic methods with no return value are void, not `mixed` (PHP language rule); their absent
        // declared return would otherwise map to Mixed
        $lc_name = strtolower($storage->cased_name ?? '');
        $ret_union = $storage->return_type;
        if (self::isNativeOnly($ret_union, $storage->signature_return_type)) {
            foreach ($inherited as $inh) {
                if ($inh->return_type !== null && !self::isNativeOnly($inh->return_type, $inh->signature_return_type)
                    && !($ret_union !== null && ($ret_union->isVoid() || $ret_union->isNever()))
                    && $this->refines($inh->return_type, $ret_union)
                ) {
                    $ret_union = $inh->return_type;
                    break;
                }
            }
        }
        if ($ret_union === null
            && in_array($lc_name, ['__construct', '__destruct', '__clone', '__wakeup', '__unset', '__set'], true)
        ) {
            $return_type = RustType::unit();
        } else {
            $return_type = $this->types->map($ret_union);
        }
        if ($node instanceof \PhpParser\Node\FunctionLike) {
            $return_type = $this->extendShapeWithReturnedKeys($return_type, $node);
        }
        if ($ret_union !== null && $ret_union->isVoid()) {
            $return_type = RustType::unit();
        }
        if ($ret_union !== null && $ret_union->isNever()) {
            $return_type = RustType::never();
        }
        if ($storage->has_yield) {
            $ret = $ret_union;
            $key = RustType::mixed();
            $val = RustType::mixed();
            if ($ret !== null) {
                foreach ($ret->getAtomicTypes() as $atomic) {
                    if ($atomic instanceof \Psalm\Type\Atomic\TGenericObject || $atomic instanceof \Psalm\Type\Atomic\TIterable) {
                        $key = $this->types->map($atomic->type_params[0] ?? null);
                        $val = $this->types->map($atomic->type_params[1] ?? null);
                    } elseif ($atomic instanceof \Psalm\Type\Atomic\TArray) {
                        $key = $this->types->map($atomic->type_params[0]);
                        $val = $this->types->map($atomic->type_params[1]);
                    }
                }
            }
            $return_type = RustType::rtGeneric('Generator', [$key, $val]);
        }
        // a generic no parameter or the return mentions cannot be inferred at call sites: dropped
        if ($this->last_generics !== []) {
            $texts = array_map(static fn(RustType $t) => $t->toRust(), $param_types);
            $texts[] = $return_type->toRust();
            foreach ($this->last_generics as $name => $g) {
                $used = false;
                foreach ($texts as $text) {
                    if (preg_match('/\b' . preg_quote($g, '/') . '\b/', $text)) {
                        $used = true;
                        break;
                    }
                }
                if (!$used) {
                    unset($this->last_generics[$name]);
                }
            }
            $this->types->generic_names = $this->last_generics;
        }
    }

    /**
     * Dispatch unions: a hierarchy method declared with `mixed` (or a class template) in a position where every
     * concrete implementation declares a real type gets the union of those types on the hierarchy handle
     * (`Iterator::current(): mixed` over `SplFileInfo`/`string` implementors dispatches as `U_SplFileInfo_or_Str`;
     * the caller narrows it with the type Psalm gives the call). Closed hierarchies only: an open one shares
     * its signature with downstream crates.
     */
    private function unifyDispatchTypes(): void
    {
        $n = 0;
        foreach ($this->uniqueClasses() as $cls) {
            if (getenv('DBG_DISPATCH')) {
                fwrite(STDERR, "[dbg-dispatch] {$cls->fqcn} leaf=" . var_export($cls->isLeaf(), true) . " down=" . var_export($cls->has_downstream, true) . " concrete=" . count($cls->concrete) . " methods=" . implode(',', array_keys($cls->methods)) . "\n");
            }
            if ($cls->isLeaf() || $cls->isTrait() || $cls->isEnum() || $cls->has_downstream || $cls->concrete === []) {
                continue;
            }
            foreach ($cls->methods as $lc => $m) {
                if ($m->isStatic() || $m->isPrivate()) {
                    continue;
                }
                $impls = [];
                foreach ($cls->concrete as $c) {
                    $cm = $this->findMethod($c, $lc);
                    if ($cm === null || $cm->isAbstract()) {
                        continue 2;
                    }
                    if ($cm !== $m) {
                        $impls[] = $cm;
                    }
                }
                if (getenv('DBG_DISPATCH')) {
                    fwrite(STDERR, "[dbg-dispatch]   {$cls->fqcn}::$lc ret=" . $m->return_type->toRust() . " impls=" . count($impls) . ' [' . implode(' | ', array_map(static fn(MethodModel $cm) => $cm->declaring->fqcn . ':' . $cm->return_type->toRust(), $impls)) . "]\n");
                }
                if ($impls === []) {
                    continue;
                }
                if ($m->return_type->containsMixed()) {
                    $u = $this->unionOfRust(array_map(static fn(MethodModel $cm) => $cm->return_type, $impls));
                    if ($u !== null) {
                        $m->return_type = $u;
                        $n++;
                    }
                }
                foreach ($m->param_types as $i => $pt) {
                    if (!$pt->containsMixed()) {
                        continue;
                    }
                    $types = [];
                    foreach ($impls as $cm) {
                        if (!isset($cm->param_types[$i])) {
                            continue 2;
                        }
                        $types[] = $cm->param_types[$i];
                    }
                    $u = $this->unionOfRust($types);
                    if ($u !== null) {
                        $m->param_types[$i] = $u;
                        $n++;
                    }
                }
            }
        }
        fwrite(STDERR, "[program] $n dispatch signatures typed as unions of their implementations\n");
    }

    /**
     * The union of the implementations' types, or null when one of them is itself dynamic/generic/void.
     *
     * @param list<RustType> $types
     */
    public function unionOfRust(array $types): ?RustType
    {
        $members = [];
        $nullable = false;
        if ($types !== [] && count(array_filter($types, static fn(RustType $t) => $t->kind === RustType::NEVER)) === count($types)) {
            // a local only ever assigned from an empty container's elements: unreachable, typed as such
            return RustType::never();
        }
        foreach ($types as $t) {
            if ($t->kind === RustType::OPTION) {
                $nullable = true;
                $t = $t->inner();
            }
            if ($t->containsMixed() || $t->hasGeneric() || in_array($t->kind, [RustType::UNIT, RustType::NEVER, RustType::RT_GENERIC, RustType::CLOSURE, RustType::DYN_CALLABLE], true)) {
                return null;
            }
            $members[$t->toRust()] = $t;
        }
        if ($members === []) {
            return null;
        }
        $u = $this->types->combine(array_values($members));
        if ($u->containsMixed()) {
            return null;
        }
        return $nullable ? RustType::option($u) : $u;
    }

    /** Some body calls `constant()` (dynamic constant lookup by name). */
    public bool $uses_constant_fn = false;

    /** True while a body is emitted: method resolutions on hierarchy handles are dispatch demands. */
    public bool $record_dispatch = false;

    /** @var array<string, true> `hierarchyLc::methodLc` dispatch methods some code calls through the handle */
    public array $dispatch_demands = [];

    /** @var array<string, array{ClassModel, MethodModel}> dispatch methods deferred to the post-pass */
    public array $pending_dispatch = [];

    /** @var array<string, array<string, RustType>> data file (rel path) => demanded typed views (key => type; 'mixed' for the Mixed view) */
    public array $data_demands = [];

    /** @var array<string, ?RustType> data file (rel path) => the type of its contents (see DataEmitter::inferType) */
    private array $data_types = [];

    /** The type of a data file's contents, null when they are not a uniform table. */
    public function dataType(FileModel $file): ?RustType
    {
        if (!array_key_exists($file->rel_path, $this->data_types)) {
            $emitter = new DataEmitter($this->codebase);
            try {
                $this->types->context = 'data file ' . $file->rel_path;
                $this->data_types[$file->rel_path] = $emitter->inferType($emitter->valueOf($file->abs_path), $this);
            } catch (\Throwable $e) {
                $this->data_types[$file->rel_path] = null;
            }
        }
        return $this->data_types[$file->rel_path];
    }

    public function noteDispatch(ClassModel $class, string $lc_name): void
    {
        if (!$class->isLeaf()) {
            $this->dispatch_demands[$class->lc() . '::' . $lc_name] = true;
        }
    }

    /** Resolve the method reachable as `$lc_name` on `$class`. */
    public function findMethod(ClassModel $class, string $lc_name): ?MethodModel
    {
        if ($this->record_dispatch) {
            $this->noteDispatch($class, $lc_name);
        }
        if (isset($class->methods[$lc_name])) {
            return $class->methods[$lc_name];
        }
        if (isset($class->storage->declaring_method_ids[$lc_name])) {
            $m = $this->methodFor($class, $lc_name, $class->storage->declaring_method_ids[$lc_name]);
            if ($m !== null) {
                $class->methods[$lc_name] = $m;
            }
            return $m;
        }
        // interfaces: look at parent interfaces
        foreach ($class->ancestors as $ancestor) {
            if (isset($ancestor->methods[$lc_name])) {
                return $ancestor->methods[$lc_name];
            }
        }
        return null;
    }

    public function getConstant(string $name): ?GlobalConstModel
    {
        return $this->constants[strtolower(ltrim($name, '\\'))] ?? null;
    }

    public function getFunction(string $fq_name): ?FunctionModel
    {
        return $this->functions[strtolower(ltrim($fq_name, '\\'))] ?? null;
    }

    /** The class model for a Rust class type. */
    /** @var array<string, int> program-wide class numbers (PhpObject::class_id), assigned on first use */
    private array $class_ids = [];

    /** Program-wide number of a class: casts and instanceof compare these instead of TypeIds or names. */
    public function classId(ClassModel $c): int
    {
        $lc = strtolower($c->fqcn);
        if (!isset($this->class_ids[$lc])) {
            $this->class_ids[$lc] = count($this->class_ids) + 1;
        }
        return $this->class_ids[$lc];
    }

    public function classOf(RustType $t): ?ClassModel
    {
        if ($t->kind !== RustType::CLASS_) {
            return null;
        }
        return $this->getClass($t->name);
    }
}
