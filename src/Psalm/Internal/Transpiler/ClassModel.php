<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Stmt\ClassLike;
use Psalm\Storage\ClassLikeStorage;

use function strtolower;

/**
 * Transpiler view of one class-like.
 *
 * @internal
 */
final class ClassModel
{
    /** @var list<ClassModel> direct subclasses / implementors */
    public array $children = [];

    /** @var list<ClassModel> all concrete (instantiable) descendants, including self when concrete; sorted by name */
    public array $concrete = [];

    /** @var array<string, FieldModel> flattened instance fields (own + inherited), by PHP property name */
    public array $fields = [];

    /** @var array<lowercase-string, MethodModel> methods reachable from this class (own + inherited) by lowercase name */
    public array $methods = [];

    /** @var array<string, FieldModel> static properties declared by this class */
    public array $static_fields = [];

    /** @var array<string, ConstModel> constants declared by this class */
    public array $constants = [];

    public ?ClassModel $parent = null;

    public bool $linked = false;

    /** Index of the crate this class is emitted into (0 = main crate; see Transpiler::crateOfFile). */
    public int $crate = 0;

    /**
     * True when a concrete subclass of this class/interface lives in a DOWNSTREAM crate, so the dispatch
     * enum cannot enumerate it. Only then does the hierarchy need the `Other__` escape variant + the
     * dynamic protocol; when false the enum is CLOSED and dispatches by exhaustive `match` (no `Mixed`,
     * no cast). Always false in a single-crate build (the whole point of collapsing the crate split).
     */
    public bool $has_downstream = false;

    /** @var list<ClassModel> all ancestors (parents, transitively) and implemented interfaces */
    public array $ancestors = [];

    /**
     * True when some OTHER class writes a property of an instance of this class post-construction
     * (`$obj->prop = ...` where $obj is not `$this`). Set by Program::computeExternalWrites(). Such a class
     * is NOT safe as immutable Rc<T>: the external write compiles to `obj.set_prop(v)` = Rc::make_mut COW on a
     * shared handle, silently losing the write. @psalm-immutable does NOT preclude this (e.g. ClassConstantStorage
     * is @psalm-immutable yet ClassLikes.php mutates ->type/->inferred_type during populate). Gates auto-widen.
     */
    public bool $externally_written = false;

    /**
     * Names of properties written post-construction by OTHER code (`$obj->field = ...`, $obj not $this). Set by
     * Program::computeExternalWrites(). When such a class is nonetheless converted to Rc<T> (concrete experiment,
     * allow_ext), these fields become per-field Cell/RefCell (interiorMutFields) so the external write is a correct
     * &self interior mutation on the shared handle (PHP object reference semantics) rather than a lost make_mut COW.
     * @var array<string, true>
     */
    public array $ext_written_fields = [];

    /**
     * True when this leaf is part of a class hierarchy that Program::computeHierarchyImmutable() proved wholly
     * convertible to Rc<T> (every concrete leaf under the same root class is immutable-safe). Set only under the
     * AUTO_IMMUTABLE_HIER env gate. Distinct from the standalone auto-widen (parent===null) path.
     */
    public bool $hier_immutable = false;

    public function __construct(
        public readonly string $fqcn,
        public readonly ClassLikeStorage $storage,
        public readonly ?ClassLike $node,
        public readonly bool $is_project,
    ) {
    }

    public function lc(): string
    {
        return strtolower($this->fqcn);
    }

    public function isInterface(): bool
    {
        return $this->storage->is_interface;
    }

    public function isTrait(): bool
    {
        return $this->storage->is_trait;
    }

    public function isEnum(): bool
    {
        return $this->storage->is_enum;
    }

    public function isAbstract(): bool
    {
        return $this->storage->abstract;
    }

    /** Can `new` be applied to this class? */
    public function isConcrete(): bool
    {
        return !$this->storage->is_interface && !$this->storage->abstract && !$this->storage->is_trait;
    }

    /** Leaf classes get a single newtype handle; everything else gets a dispatch enum. */
    public function isLeaf(): bool
    {
        return $this->isConcrete() && $this->children === [];
    }

    /** Short Rust type name of the handle. */
    public function handle(): string
    {
        return Names::classShort($this->fqcn);
    }

    /** Rust name of the newtype over own instances (same as handle for leaves). */
    public function ownHandle(): string
    {
        return $this->isLeaf() ? $this->handle() : $this->handle() . 'Self';
    }

    /** Rust name of the data struct. */
    public function objStruct(): string
    {
        return $this->handle() . 'Obj_';
    }

    public function path(): string
    {
        return Names::classPath($this->fqcn);
    }

    public function ownPath(): string
    {
        return 'crate::' . Names::classModule($this->fqcn) . '::' . $this->ownHandle();
    }

    public function objPath(): string
    {
        return 'crate::' . Names::classModule($this->fqcn) . '::' . $this->objStruct();
    }

    /** The topmost ancestor (following `extends`) emitted into the same crate as this class. */
    public function crateRoot(): ClassModel
    {
        $c = $this;
        while ($c->parent !== null && $c->parent->crate === $this->crate && $c->parent->is_project) {
            $c = $c->parent;
        }
        return $c;
    }

    public function isSubclassOf(ClassModel $other): bool
    {
        if ($other === $this) {
            return true;
        }
        foreach ($this->ancestors as $a) {
            if ($a === $other) {
                return true;
            }
        }
        return false;
    }

    /** Variant name used for a concrete class inside dispatch enums. */
    public function variant(): string
    {
        return Names::classMangle($this->fqcn);
    }

    /**
     * Pilot allowlist for the Rc<T> (no RefCell) representation of `@psalm-immutable` classes. Starts with
     * leaf, wither-free pure DTOs (reads become direct field access, no write path) to validate the mechanism
     * before widening to withered/hierarchical immutables (Union, Atomic, ...).
     * @var array<string, true>
     */
    private const IMMUTABLE_PILOT = [
        // Rc<T> (no RefCell) pilot. Write-path handled: magic__construct is emitted `&mut self` (signature()) with
        // a `mut this` in new(); withers write clones (mut locals); set_/_mut accessors use Rc::make_mut. Only
        // leaf, wither-free pure DTOs whose fields are NOT in DYN_ACCESS_PROPS (so get_prop/set_prop emit no
        // &mut-setter calls through &self). Widen carefully; hierarchical/withered immutables (Union, Atomic) need
        // whole-hierarchy conversion for enum-accessor return-type uniformity.
        'Psalm\Internal\Analyzer\ClassLikeNameOptions' => true,
        // set_prop is now empty for immutable classes, so DYN_ACCESS_PROPS fields no longer break (ArrayType
        // has value/count). get_prop reads stay (&self, fine).
        'Psalm\Internal\Type\ArrayType' => true,
        'Psalm\Internal\Type\TypeAlias\InlineTypeAlias' => true,
        'Psalm\Internal\Type\TypeAlias\ClassTypeAlias' => true,
        'Psalm\Internal\Analyzer\Statements\Expression\Call\Method\AtomicCallContext' => true,
        'Psalm\Internal\FileManipulation\CodeMigration' => true,
        // Batch 2: leaf final @psalm-immutable with ZERO interior mutation (no $this->x= outside ctor, no
        // reset/next/etc on $this->). Value DTOs (not map keys).
        'Psalm\Internal\Analyzer\Statements\Expression\Assignment\AssignedProperty' => true,
        'Psalm\Internal\Analyzer\Statements\Expression\Call\HighOrderFunctionArgInfo' => true,
        'Psalm\Internal\Diff\DiffElem' => true,
        'Psalm\Internal\Scope\IfConditionalScope' => true,
        'Psalm\Internal\Type\TypeAlias\LinkableTypeAlias' => true,
        'Psalm\Plugin\EventHandler\Event\AfterAnalysisEvent' => true,
        'Psalm\Plugin\EventHandler\Event\BeforeAddIssueEvent' => true,
        'Psalm\Plugin\EventHandler\Event\AfterCodebasePopulatedEvent' => true,
        // Batch 3: plugin-hook argument DTOs (standalone final @psalm-immutable, no traits, no mutation).
        'Psalm\Plugin\EventHandler\Event\AddRemoveTaintsEvent' => true,
        'Psalm\Plugin\EventHandler\Event\AfterEveryFunctionCallAnalysisEvent' => true,
        'Psalm\Plugin\EventHandler\Event\AfterFileAnalysisEvent' => true,
        'Psalm\Plugin\EventHandler\Event\BeforeFileAnalysisEvent' => true,
        'Psalm\Plugin\EventHandler\Event\FunctionExistenceProviderEvent' => true,
        'Psalm\Plugin\EventHandler\Event\FunctionParamsProviderEvent' => true,
        'Psalm\Plugin\EventHandler\Event\FunctionReturnTypeProviderEvent' => true,
        'Psalm\Plugin\EventHandler\Event\MethodExistenceProviderEvent' => true,
        'Psalm\Plugin\EventHandler\Event\MethodParamsProviderEvent' => true,
        'Psalm\Plugin\EventHandler\Event\MethodReturnTypeProviderEvent' => true,
        'Psalm\Plugin\EventHandler\Event\MethodVisibilityProviderEvent' => true,
        'Psalm\Plugin\EventHandler\Event\PropertyExistenceProviderEvent' => true,
        'Psalm\Plugin\EventHandler\Event\PropertyTypeProviderEvent' => true,
        'Psalm\Plugin\EventHandler\Event\PropertyVisibilityProviderEvent' => true,
        'Psalm\Plugin\EventHandler\Event\StringInterpreterEvent' => true,
        // Batch 4: hot standalone final @psalm-immutable value DTOs with ZERO post-construction $this->
        // writes (only ImmutableNonCloneableTrait + construction-time __unserialize). No withering, no
        // hierarchy, no singleton-cache aliasing (unlike Union) -> Rc::make_mut COW never triggers.
        'Psalm\Internal\MethodIdentifier' => true,
        'Psalm\Internal\DataFlow\Path' => true,
        'Psalm\Internal\Analyzer\DataFlowNodeData' => true,
        'Psalm\Storage\AttributeArg' => true,
        // Batch 5: standalone final @psalm-immutable storage value DTOs, ZERO post-construction writes
        // (ImmutableNonCloneable/Unserialize traits only). Populated at scan, read during analysis.
        'Psalm\Storage\EnumCaseStorage' => true,
        'Psalm\Storage\AttributeStorage' => true,
        // Union: NOT immutable-emit. Attempted Rc<T>, but it regresses list type-inference
        // (testBuildList/testBuildList3/testBuildListOther: merge collapses to the concrete
        // "all-branches-taken" path, losing possibly_undefined + value-union). Root cause:
        // Union is a globally-shared cached singleton (Type::getEmptyArray()) with refcount>>1;
        // value-semantics (Rc::make_mut copy-on-write) diverges from RefCell reference-semantics
        // when a construction-method &mut self touches a shared instance during context merge.
        // Needs a dedicated refactor to remove reference-semantics reliance before re-enabling.
        // 'Psalm\Type\Union' => true,
    ];

    /** @var ?array<string, true> cache for interiorMutFields() */
    private ?array $interior_mut_fields = null;

    /** @var ?array<string, true> cache for constructionMethods() */
    private ?array $construction_methods = null;

    /**
     * Lowercase names of methods reachable from `__construct` via `$this->m()` calls (init helpers). Their
     * `$this->field =` writes are CONSTRUCTION-time (on the fresh, uniquely-owned object), not post-construction
     * interior mutation — so they don't force a field into Cell/RefCell, and they are emitted `&mut self` (like
     * the constructor) so those writes go through Rc::make_mut. Distinguishes e.g. Union's private init helper
     * (writes $this->types) from its memoizing getId (writes $this->id post-construction).
     * @return array<string, true>
     */
    public function constructionMethods(): array
    {
        if ($this->construction_methods !== null) {
            return $this->construction_methods;
        }
        $reachable = [];
        if (isset($this->methods['__construct'])) {
            $reachable = ['__construct' => true];
            $queue = ['__construct'];
            $finder = new \PhpParser\NodeFinder();
            while ($queue !== []) {
                $lc = array_pop($queue);
                $m = $this->methods[$lc] ?? null;
                if ($m === null || $m->node === null) {
                    continue;
                }
                foreach ($finder->find($m->node->stmts ?? [], static fn(\PhpParser\Node $n): bool =>
                    $n instanceof \PhpParser\Node\Expr\MethodCall
                    && $n->var instanceof \PhpParser\Node\Expr\Variable && $n->var->name === 'this'
                    && $n->name instanceof \PhpParser\Node\Identifier) as $call
                ) {
                    /** @var \PhpParser\Node\Expr\MethodCall $call */
                    $cl = strtolower($call->name->name);
                    if (!isset($reachable[$cl]) && isset($this->methods[$cl])) {
                        $reachable[$cl] = true;
                        $queue[] = $cl;
                    }
                }
            }
        }
        return $this->construction_methods = $reachable;
    }

    /**
     * Fields of an immutable (Rc<T>) class that are written to `$this->field` OUTSIDE the constructor (in a
     * &self method — e.g. lazy memoization `$this->id = ...`, or a `$this->checked = true` flag). These need
     * per-field interior mutability (Cell<T>/RefCell<T>) since Rc<T> has none; other fields stay plain.
     * Scans all reachable methods (own + inherited + trait, flattened) so trait-based mutation is caught.
     * @return array<string, true>
     */
    public function interiorMutFields(): array
    {
        if ($this->interior_mut_fields !== null) {
            return $this->interior_mut_fields;
        }
        if (!$this->immutable()) {
            return $this->interior_mut_fields = [];
        }
        // Fields needing per-field interior mutability: own post-construction $this writes (memoization) PLUS fields
        // written EXTERNALLY by other code (ext_written_fields) — both become Cell/RefCell so the write is a correct
        // &self interior mutation on the shared Rc<T> (no lost make_mut COW; PHP reference semantics for external writes).
        $set = $this->postConstructionWrittenFields();
        foreach ($this->ext_written_fields as $fld => $_) {
            if (isset($this->fields[$fld])) {
                $set[$fld] = true;
            }
        }
        if ($this->hier_immutable) {
            // A field mutated (memoized or externally) anywhere in the hierarchy must be Cell/RefCell in EVERY member,
            // or the shared base-enum accessor can't dispatch uniformly. Union across the whole hierarchy (fields on
            // this class only). Covers inherited-method memoization (CodeLocation::calculateRealLocation) and external
            // writes to a base-typed handle (Type\Atomic\* params/return_type).
            $root = $this;
            while ($root->parent !== null) {
                $root = $root->parent;
            }
            // Union over the WHOLE hierarchy: root + all concrete leaves + every abstract intermediate base (walk each
            // leaf's parent chain). A field written through an ABSTRACT-base-typed handle (e.g. `$type->checked` where
            // $type: Type\Atomic) is recorded on that abstract base, which is not in ->concrete — include it here or the
            // field stays plain and the write (dispatched on a clone) is lost / needs &mut.
            $members = [$root->fqcn => $root];
            foreach ($root->concrete as $c) {
                $members[$c->fqcn] = $c;
                for ($a = $c->parent; $a !== null; $a = $a->parent) {
                    $members[$a->fqcn] = $a;
                }
            }
            foreach ($members as $m) {
                foreach ($m->postConstructionWrittenFields() as $fld => $_) {
                    if (isset($this->fields[$fld])) {
                        $set[$fld] = true;
                    }
                }
                foreach ($m->ext_written_fields as $fld => $_) {
                    if (isset($this->fields[$fld])) {
                        $set[$fld] = true;
                    }
                }
            }
        }
        return $this->interior_mut_fields = $set;
    }

    /** @var ?array<string, true> cache for postConstructionWrittenFields() */
    private ?array $post_ctor_written = null;

    /**
     * Fields written via `$this->field =` (or `$this->field[...] =`) OUTSIDE the construction methods. Computed
     * WITHOUT consulting immutable() (which depends on this), so it is safe to use in the immutable() decision.
     * @return array<string, true>
     */
    private function postConstructionWrittenFields(): array
    {
        if ($this->post_ctor_written !== null) {
            return $this->post_ctor_written;
        }
        $set = [];
        $ctor_methods = $this->constructionMethods();
        $finder = new \PhpParser\NodeFinder();
        foreach ($this->methods as $m) {
            // construction-time writes (ctor + init helpers reachable from it) are not interior mutation
            if ($m->node === null || isset($ctor_methods[$m->lc()])) {
                continue;
            }
            foreach ($finder->find($m->node->stmts ?? [], static fn(\PhpParser\Node $n): bool =>
                ($n instanceof \PhpParser\Node\Expr\Assign || $n instanceof \PhpParser\Node\Expr\AssignOp)
                && self::thisPropName($n->var) !== null) as $assign
            ) {
                /** @var \PhpParser\Node\Expr\Assign|\PhpParser\Node\Expr\AssignOp $assign */
                $fname = self::thisPropName($assign->var);
                if ($fname !== null && isset($this->fields[$fname])) {
                    $set[$fname] = true;
                }
            }
        }
        return $this->post_ctor_written = $set;
    }

    /** @var ?bool cache for noHelperConstructionWrites() */
    private ?bool $no_helper_ctor_writes = null;

    /**
     * Whether the ONLY construction-method that writes `$this->field` is `__construct` itself (no init-helper
     * writes `$this`). An init-helper that writes `$this` is emitted `&mut self` (like the ctor) but, when it is
     * inherited from an abstract base and reached through the dispatch enum on a SHARED handle, that `&mut self`
     * cannot borrow (E0596) — the exact failure that made the UnresolvedConstant family unsafe as Rc<T>. So a
     * class is only Rc<T>-safe if all its construction-time `$this` writes live in `__construct`.
     */
    private function noHelperConstructionWrites(): bool
    {
        if ($this->no_helper_ctor_writes !== null) {
            return $this->no_helper_ctor_writes;
        }
        $ctor_methods = $this->constructionMethods();
        $finder = new \PhpParser\NodeFinder();
        foreach ($this->methods as $m) {
            // only NON-__construct construction-methods (init helpers reachable from the ctor)
            if ($m->node === null || $m->lc() === '__construct' || !isset($ctor_methods[$m->lc()])) {
                continue;
            }
            foreach ($finder->find($m->node->stmts ?? [], static fn(\PhpParser\Node $n): bool =>
                ($n instanceof \PhpParser\Node\Expr\Assign || $n instanceof \PhpParser\Node\Expr\AssignOp)
                && self::thisPropName($n->var) !== null) as $_
            ) {
                return $this->no_helper_ctor_writes = false;
            }
        }
        return $this->no_helper_ctor_writes = true;
    }

    /** @var ?bool cache for hasCloneMutateWither() */
    private ?bool $has_clone_mutate_wither = null;

    /**
     * Whether any method clones an object into a local and then writes a property of that local — the
     * `$c = clone $this; $c->prop = x; return $c;` wither pattern. Such a write compiles as `$c.prop = x`
     * on a local typed `Rc<T>` (clone yields `Rc<T>`, not an owned `T`), which cannot borrow mutably
     * (E0596) — the exact source of the 812 borrow errors when leaf @psalm-immutable widening was tried
     * 2026-09-15. `Rc::make_mut` is emitted only for `$this->` writes, not for clone-locals, so a class
     * with this pattern is NOT Rc<T>-safe under the current model (needs owned-clone escape analysis).
     * Sound over-approximation: any property write to a variable that was assigned a `clone` anywhere in
     * the same method disqualifies the class. Pure value markers (the Assertion family) have no such
     * pattern, so this only tightens qualification — it never drops a validated-green class.
     */
    private function hasCloneMutateWither(): bool
    {
        if ($this->has_clone_mutate_wither !== null) {
            return $this->has_clone_mutate_wither;
        }
        $finder = new \PhpParser\NodeFinder();
        foreach ($this->methods as $m) {
            if ($m->node === null) {
                continue;
            }
            $stmts = $m->node->stmts ?? [];
            // locals assigned a `clone ...` expression in this method
            $clone_locals = [];
            foreach ($finder->find($stmts, static fn(\PhpParser\Node $n): bool =>
                $n instanceof \PhpParser\Node\Expr\Assign
                && $n->expr instanceof \PhpParser\Node\Expr\Clone_
                && $n->var instanceof \PhpParser\Node\Expr\Variable
                && is_string($n->var->name)) as $assign
            ) {
                /** @var \PhpParser\Node\Expr\Assign $assign */
                /** @var \PhpParser\Node\Expr\Variable $v */
                $v = $assign->var;
                $clone_locals[$v->name] = true;
            }
            if ($clone_locals === []) {
                continue;
            }
            // a property write to any clone-local => wither
            foreach ($finder->find($stmts, static fn(\PhpParser\Node $n): bool =>
                ($n instanceof \PhpParser\Node\Expr\Assign || $n instanceof \PhpParser\Node\Expr\AssignOp)
                && self::localPropName($n->var) !== null) as $assign
            ) {
                /** @var \PhpParser\Node\Expr\Assign|\PhpParser\Node\Expr\AssignOp $assign */
                if (isset($clone_locals[self::localPropName($assign->var)])) {
                    return $this->has_clone_mutate_wither = true;
                }
            }
        }
        return $this->has_clone_mutate_wither = false;
    }

    /** The variable name if `$e` is `$name->prop` or `$name->prop[...]` (any depth) for a non-$this local, else null. */
    private static function localPropName(\PhpParser\Node\Expr $e): ?string
    {
        while ($e instanceof \PhpParser\Node\Expr\ArrayDimFetch) {
            $e = $e->var;
        }
        if ($e instanceof \PhpParser\Node\Expr\PropertyFetch
            && $e->var instanceof \PhpParser\Node\Expr\Variable && is_string($e->var->name)
            && $e->var->name !== 'this'
        ) {
            return $e->var->name;
        }
        return null;
    }

    /** The property name if `$e` is `$this->name` or `$this->name[...]` (any depth), else null. */
    private static function thisPropName(\PhpParser\Node\Expr $e): ?string
    {
        while ($e instanceof \PhpParser\Node\Expr\ArrayDimFetch) {
            $e = $e->var;
        }
        if ($e instanceof \PhpParser\Node\Expr\PropertyFetch
            && $e->var instanceof \PhpParser\Node\Expr\Variable && $e->var->name === 'this'
            && $e->name instanceof \PhpParser\Node\Identifier
        ) {
            return $e->name->name;
        }
        return null;
    }

    /**
     * Whether this class is emitted as `Rc<T>` (immutable, no RefCell) rather than `Rc<RefCell<T>>`. Only for
     * `@psalm-immutable` (allowed_mutations = LEVEL_NONE) leaf classes on the pilot allowlist; reads are direct
     * and any writes go through `Rc::make_mut` on a uniquely-owned value (construction / wither clones).
     */
    /**
     * Per-class immutable-safety for a hierarchy member: concrete, @psalm-immutable (mutation-free), no own
     * post-construction $this writes, no init-helper writes, and not externally written. Withers are allowed
     * (handled by the LValueTrait::place make_mut-in-place write-path). Used by Program::computeHierarchyImmutable
     * to decide if a WHOLE hierarchy can convert together (enum-accessor uniformity needs all leaves converted).
     */
    /**
     * Whether EVERY concrete class dispatched by this class/interface's handle enum is immutable() (Rc<T>). When
     * true the base enum's field accessors and construction-method dispatch must use the immutable forms
     * (PropRef::Owned getters, no set_/mut, no magic__construct dispatch) — see ClassEmitter::emitEnumAccessors /
     * the enum method dispatch. False for an empty hierarchy or any mutable/mixed member.
     */
    public function allConcreteImmutable(): bool
    {
        if ($this->concrete === []) {
            return false;
        }
        foreach ($this->concrete as $c) {
            if (!$c->immutable()) {
                return false;
            }
        }
        return true;
    }

    /**
     * Whether, when emitted ON THIS class, method `$lc` is an immutable-Rc<T> construction method that writes $this
     * in place (the ctor or an init-helper reachable from it). Such a method needs `&mut self` and, when inherited,
     * must be run as a super-copy on this (leaf) class against `self` directly rather than forwarded to the base
     * enum's `__impl` on a clone (which would COW and lose the writes, and the enum omits __impl for immutable
     * hierarchies anyway). Used by ClassEmitter (forwarding stub + super-copy signature) and CallTrait (self::/parent::).
     */
    public function isImmutableCtorMethod(string $lc): bool
    {
        return $this->immutable() && ($lc === '__construct' || isset($this->constructionMethods()[$lc]));
    }

    /**
     * @param bool $allow_memo when true, permit post-construction `$this->field =` writes (memoization / lazy caches):
     *   those fields become per-field Cell/RefCell inside the Rc<T> struct via interiorMutFields(), so the write is
     *   correct on the shared handle (no make_mut COW). Default false = the deployed conservative behavior.
     */
    /**
     * @param bool $allow_memo permit post-construction `$this->field =` writes (they become per-field Cell/RefCell).
     * @param bool $allow_ext  permit EXTERNAL writes (`$obj->field =` by other code): the written fields become
     *   per-field Cell/RefCell (interiorMutFields includes ext_written_fields), so the external write is a correct
     *   &self interior mutation on the shared Rc<T>. Needed for the Atomic hierarchy (params/return_type set post-hoc).
     */
    public function isImmutableSafeMember(bool $allow_memo = false, bool $allow_ext = false): bool
    {
        return $this->isConcrete()
            && $this->storage->allowed_mutations <= \Psalm\Storage\Mutations::LEVEL_INTERNAL_READ
            && ($allow_ext || !$this->externally_written)
            && ($allow_memo || $this->postConstructionWrittenFields() === [])
            && $this->noHelperConstructionWrites();
    }

    public function immutable(): bool
    {
        // @psalm-immutable == LEVEL_INTERNAL_READ (no internal writes post-construction, safe for Rc<T>);
        // LEVEL_INTERNAL_READ_WRITE (@psalm-external-mutation-free) allows memoization writes and needs RefCell.
        if ($this->storage->allowed_mutations > \Psalm\Storage\Mutations::LEVEL_INTERNAL_READ) {
            return false;
        }
        // Whole-hierarchy conversion (Program::computeHierarchyImmutable) may mark a CONCRETE-NON-LEAF class (a base
        // that is instantiated AND subclassed, e.g. under the HIER_CONCRETE experiment) — allow those to bypass the
        // isLeaf guard below. Default (deployed) marks only leaves, so this changes nothing there.
        if ($this->hier_immutable) {
            return true;
        }
        if (!$this->isLeaf()) {
            return false;
        }
        // Hand-vetted pilot classes only. Broad auto-widening (leaf + mutation-free + no post-construction $this->
        // writes) was tried 2026-09-15 and produced 2287 build errors (812 "cannot borrow behind & reference"):
        // Psalm's allowed_mutations marker does NOT align with the transpiler's emitted mutation sites — many
        // flagged-immutable classes are still written externally (`$obj->prop = x` from other classes) or via
        // _mut/set_ accessors the postConstructionWrittenFields scan (own-methods `$this->` only) cannot see. The
        // Rc<T> axis needs per-class validation (the pilot) or a whole-program external-mutation analysis first.
        if (isset(self::IMMUTABLE_PILOT[$this->fqcn])) {
            return true;
        }
        // Scoped hierarchy conversions: self-contained families of pure @psalm-immutable value objects (created +
        // read, never mutated — @psalm-immutable forbids external mutation too), safe as Rc<T>. Validated keep-green
        // one family at a time. Concrete leaves convert (abstract bases are excluded by isLeaf() above); any
        // interior-mut field still gets a Cell via interiorMutFields()/postConstructionWrittenFields().
        // @psalm-immutable value families made Rc<T>. Guards: no post-construction $this writes (would need a
        // Cell), and no init-helper $this writes (noHelperConstructionWrites) — the latter is what made
        // UnresolvedConstant unsafe (inherited abstract-base helper emitted `&mut self`, dispatched on a shared
        // handle -> E0596). Concrete leaves convert (abstract bases are excluded by isLeaf() above).
        $immutable_families = [
            'Psalm\\Storage\\Assertion\\',   // 41 pure assertion value markers (validated green)
            // 'Psalm\\Type\\Atomic\\' -> 2013 errors (1290 E0308 + 718 E0596): the Atomic hierarchy is withered/
            //   dispatched/mutated pervasively (MutableUnion/TypeVisitor/withers) beyond the noHelperConstructionWrites
            //   guard. Broad hot-path immutable-Rc<T> needs whole-hierarchy owned-handle dispatch + wither handling.
        ];
        foreach ($immutable_families as $prefix) {
            if (str_starts_with($this->fqcn, $prefix)
                && $this->postConstructionWrittenFields() === []
                && $this->noHelperConstructionWrites()
                && !$this->hasCloneMutateWither()
            ) {
                return true;
            }
        }
        // Auto-widen (DEPLOYED 2026-09-16, was env AUTO_IMMUTABLE): admit any STANDALONE (no parent class -> not a
        // variant of a base-class dispatch enum, so no enum-accessor return-type uniformity constraint; interfaces are
        // fine, they carry no fields) leaf @psalm-immutable class that is provably write-free: no own post-construction
        // $this writes, no init-helper writes, AND not externally written (Program::computeExternalWrites, a
        // whole-program external-mutation analysis). Validated: build green + smoke identical to baseline. Hierarchy-
        // bound leaves are excluded by `parent === null` and handled by hier_immutable (computeHierarchyImmutable).
        // hasCloneMutateWither() intentionally NOT checked — the LValueTrait::place make_mut-in-place write-path
        // handles clone-then-mutate withers (none among standalone classes in practice).
        if ($this->parent === null
            && !$this->externally_written
            && $this->postConstructionWrittenFields() === []
            && $this->noHelperConstructionWrites()
        ) {
            return true;
        }
        return false;
    }
}
