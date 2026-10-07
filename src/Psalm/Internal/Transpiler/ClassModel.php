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
    /** @var ?array<string, true> memo of constructionOnlyFields() */
    private ?array $construction_only = null;
    /** @var ?bool memo of ownWritesFieldsDynamically() */
    private ?bool $dynamic_field_writes = null;
    /** @var ?array<string, true> memo of ownNonConstructionWrites() */
    private ?array $own_non_ctor_writes = null;

    /**
     * Lowercase names of methods reachable from `__construct` via `$this->m()` calls (init helpers). Their
     * `$this->field =` writes are CONSTRUCTION-time (on the fresh, uniquely-owned object), not post-construction
     * interior mutation — so they don't force a field into Cell/RefCell, and they are emitted `&mut self` (like
     * the constructor) so those writes go through Rc::make_mut. Distinguishes e.g. Union's private init helper
     * (writes $this->types) from its memoizing getId (writes $this->id post-construction).
     * @return array<string, true>
     */
    /**
     * Forget the memos that depend on the method set (construction methods and the write scans built on them):
     * they may have been asked for while the methods were still being attached (Program::buildMethods asks
     * constructionMethods() of a method's declaring class per method), and an answer memoized before the
     * constructor was attached counts every constructor write as post-construction.
     */
    public function invalidateMethodMemos(): void
    {
        $this->construction_methods = null;
        $this->construction_only = null;
        $this->dynamic_field_writes = null;
        $this->own_non_ctor_writes = null;
        $this->post_ctor_written = null;
        $this->no_helper_ctor_writes = null;
        $this->interior_mut_fields = null;
    }

    public function constructionMethods(): array
    {
        if ($this->construction_methods !== null) {
            return $this->construction_methods;
        }
        $reachable = [];
        // __unserialize / __wakeup run on the freshly created object of an unserialization (nothing else holds
        // it yet), so their writes are construction too
        foreach (['__unserialize', '__wakeup'] as $hook) {
            if (isset($this->methods[$hook])) {
                $reachable[$hook] = true;
            }
        }
        if (isset($this->methods['__construct'])) {
            $reachable['__construct'] = true;
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
    /**
     * Whether `$field` is ever assigned after construction: by this class's own methods, externally by other
     * code, or by any subclass (which shares the accessor through the hierarchy). A field that is never
     * written after construction can be read through a plain borrow: no guard, no clone.
     */
    public function writtenAfterConstruction(string $field): bool
    {
        if (isset($this->postConstructionWrittenFields()[$field]) || isset($this->ext_written_fields[$field])) {
            return true;
        }
        foreach ($this->children as $child) {
            if ($child->writtenAfterConstruction($field)) {
                return true;
            }
        }
        return false;
    }

    /**
     * Fields of a RefCell-layout class that nothing writes after construction (only `__construct`,
     * `__unserialize` and `__wakeup` may assign them; no by-reference use, no external write, no write in any
     * class of the hierarchy, no dynamic-name property write). They are stored outside the RefCell as
     * `php_rt::late::Init<T>` so a read through a shared handle is a plain `&T`. CO_FIELDS=0 disables.
     *
     * @return array<string, true>
     */
    public function constructionOnlyFields(): array
    {
        if ($this->construction_only !== null) {
            return $this->construction_only;
        }
        $v = getenv('CO_FIELDS');
        if ($v === '0' || !$this->isConcrete() || $this->immutable() || $this->valueType()) {
            return $this->construction_only = [];
        }
        $root = $this;
        while ($root->parent !== null) {
            $root = $root->parent;
        }
        $set = [];
        $diag = getenv('CO_DIAG');
        foreach ($this->fields as $f) {
            if ($f->is_static) {
                continue;
            }
            if (!$root->subtreeWritesField($f->name)) {
                $set[$f->name] = true;
            } elseif ($diag !== false && $diag !== '') {
                // rejected by the strict rule: say why (`plain-write` = an assignment outside construction too)
                fwrite(STDERR, "[co-diag] " . $this->fqcn . "::$" . $f->name . " " . $root->whySubtreeWrites($f->name)
                    . ($this->writtenAfterConstruction($f->name) ? ' (written after construction)' : '') . "\n");
            }
        }
        return $this->construction_only = $set;
    }

    /** The first reason the strict construction-only rule rejects `$field` in this subtree (diagnostics). */
    private function whySubtreeWrites(string $field): string
    {
        if ($this->ownWritesFieldsDynamically()) {
            return 'dynamic-name write in ' . $this->fqcn;
        }
        if (isset($this->ext_written_fields[$field])) {
            return 'external write recorded on ' . $this->fqcn;
        }
        $writes = $this->ownNonConstructionWrites();
        if (isset($writes[$field])) {
            return ($this->own_write_kinds[$field] ?? 'write') . ' in ' . $this->fqcn;
        }
        foreach ($this->children as $child) {
            $why = $child->whySubtreeWrites($field);
            if ($why !== '') {
                return $why;
            }
        }
        return '';
    }

    /** @var array<string, string> field => kind of the first non-construction write found (diagnostics) */
    private array $own_write_kinds = [];

    /** Whether this class or any class below it writes `$field` after construction (or dynamically). */
    private function subtreeWritesField(string $field): bool
    {
        if (isset($this->ownNonConstructionWrites()[$field]) || isset($this->ext_written_fields[$field])
            || $this->ownWritesFieldsDynamically()
        ) {
            return true;
        }
        foreach ($this->children as $child) {
            if ($child->subtreeWritesField($field)) {
                return true;
            }
        }
        return false;
    }

    /** `$this->{$expr} = ...` anywhere in the class: every field may be written. */
    private function ownWritesFieldsDynamically(): bool
    {
        if ($this->dynamic_field_writes !== null) {
            return $this->dynamic_field_writes;
        }
        $finder = new \PhpParser\NodeFinder();
        foreach ($this->methods as $m) {
            if ($m->node === null) {
                continue;
            }
            $hit = $finder->findFirst($m->node->stmts ?? [], static function (\PhpParser\Node $n): bool {
                if (!($n instanceof \PhpParser\Node\Expr\Assign || $n instanceof \PhpParser\Node\Expr\AssignOp
                    || $n instanceof \PhpParser\Node\Expr\AssignRef)) {
                    return false;
                }
                $t = $n->var;
                while ($t instanceof \PhpParser\Node\Expr\ArrayDimFetch) {
                    $t = $t->var;
                }
                return $t instanceof \PhpParser\Node\Expr\PropertyFetch
                    && $t->var instanceof \PhpParser\Node\Expr\Variable && $t->var->name === 'this'
                    && !$t->name instanceof \PhpParser\Node\Identifier;
            });
            if ($hit !== null) {
                return $this->dynamic_field_writes = true;
            }
        }
        return $this->dynamic_field_writes = false;
    }

    /**
     * Fields this class's methods (own, inherited and trait methods, flattened) write outside `__construct`,
     * `__unserialize` and `__wakeup`: assignments, compound assignments, reference assignments (either side),
     * increments, unset, by-reference foreach, list destructuring targets and by-reference call arguments.
     *
     * @return array<string, true>
     */
    private function ownNonConstructionWrites(): array
    {
        if ($this->own_non_ctor_writes !== null) {
            return $this->own_non_ctor_writes;
        }
        $set = [];
        $finder = new \PhpParser\NodeFinder();
        $program = self::$program;
        foreach ($this->methods as $lc => $m) {
            // __destruct runs when nothing else holds the object: its writes cannot race a read
            if ($m->node === null || $lc === '__construct' || $lc === '__unserialize' || $lc === '__wakeup' || $lc === '__destruct') {
                continue;
            }
            foreach ($finder->find($m->node->stmts ?? [], static fn(\PhpParser\Node $n): bool => true) as $n) {
                $targets = [];
                $kind = $n->getType() . ' in ' . $lc . '()';
                if ($n instanceof \PhpParser\Node\Expr\Assign || $n instanceof \PhpParser\Node\Expr\AssignOp) {
                    $targets[] = $n->var;
                } elseif ($n instanceof \PhpParser\Node\Expr\AssignRef) {
                    $targets[] = $n->var;
                    $targets[] = $n->expr;
                } elseif ($n instanceof \PhpParser\Node\Expr\PreInc || $n instanceof \PhpParser\Node\Expr\PreDec
                    || $n instanceof \PhpParser\Node\Expr\PostInc || $n instanceof \PhpParser\Node\Expr\PostDec
                ) {
                    $targets[] = $n->var;
                } elseif ($n instanceof \PhpParser\Node\Stmt\Unset_) {
                    foreach ($n->vars as $v) {
                        $targets[] = $v;
                    }
                } elseif ($n instanceof \PhpParser\Node\Stmt\Foreach_) {
                    if ($n->byRef) {
                        $targets[] = $n->expr;
                    }
                } elseif ($n instanceof \PhpParser\Node\Expr\FuncCall || $n instanceof \PhpParser\Node\Expr\MethodCall
                    || $n instanceof \PhpParser\Node\Expr\StaticCall || $n instanceof \PhpParser\Node\Expr\New_
                    || $n instanceof \PhpParser\Node\Expr\NullsafeMethodCall
                ) {
                    if ($n->isFirstClassCallable()) {
                        continue;
                    }
                    foreach ($n->getArgs() as $i => $arg) {
                        if (self::thisPropName($arg->value) !== null
                            && ($program === null || $program->argMayBeByRef($n, $i, $arg, $this, $m))
                        ) {
                            $targets[] = $arg->value;
                        }
                    }
                }
                foreach ($targets as $t) {
                    if ($t instanceof \PhpParser\Node\Expr\List_ || $t instanceof \PhpParser\Node\Expr\Array_) {
                        foreach ($t->items as $item) {
                            if ($item !== null && ($fname = self::thisPropName($item->value)) !== null) {
                                $set[$fname] = true;
                                $this->own_write_kinds[$fname] ??= $kind;
                            }
                        }
                        continue;
                    }
                    if (($fname = self::thisPropName($t)) !== null) {
                        $set[$fname] = true;
                        $this->own_write_kinds[$fname] ??= $kind;
                    }
                }
            }
        }
        return $this->own_non_ctor_writes = $set;
    }

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

    /**
     * `Method::$field` for every post-construction `$this->field` write that is not a cache write: one inside a
     * method that may mutate (Psalm capabilities beyond MUTATION_FREE). A write inside a
     * mutation-free method is a suppressed memo write, which a value type may lose.
     *
     * @return list<string>
     */
    private function nonCachePostConstructionWrites(): array
    {
        $out = [];
        $ctor_methods = $this->constructionMethods();
        $finder = new \PhpParser\NodeFinder();
        foreach ($this->methods as $m) {
            if ($m->node === null || isset($ctor_methods[$m->lc()])) {
                continue;
            }
            if (($m->storage->capabilities & ~\Psalm\Storage\Capabilities::MUTATION_FREE) === 0) {
                continue;
            }
            foreach ($finder->find($m->node->stmts ?? [], static fn(\PhpParser\Node $n): bool =>
                ($n instanceof \PhpParser\Node\Expr\Assign || $n instanceof \PhpParser\Node\Expr\AssignOp)
                && self::thisPropName($n->var) !== null) as $assign
            ) {
                /** @var \PhpParser\Node\Expr\Assign|\PhpParser\Node\Expr\AssignOp $assign */
                $fname = self::thisPropName($assign->var);
                if ($fname !== null && isset($this->fields[$fname])) {
                    $out[] = $m->name . '::$' . $fname;
                }
            }
        }
        return array_values(array_unique($out));
    }

    /**
     * Whether any class of this hierarchy (root, intermediates, concrete members) writes a field of `$this` after
     * construction inside a mutation-free method: a suppressed cache write, the only write a mutation-free callee
     * can ever perform on an object of the family. Without any, a guard on a member's cell can stay across a
     * mutation-free call (nothing that call runs can borrow the cell mutably).
     */
    public function familyHasCacheWrites(): bool
    {
        if ($this->family_cache_writes !== null) {
            return $this->family_cache_writes;
        }
        $root = $this;
        while ($root->parent !== null) {
            $root = $root->parent;
        }
        $members = [$root->fqcn => $root, $this->fqcn => $this];
        foreach ($root->concrete as $c) {
            $members[$c->fqcn] = $c;
            for ($a = $c->parent; $a !== null; $a = $a->parent) {
                $members[$a->fqcn] = $a;
            }
        }
        $finder = new \PhpParser\NodeFinder();
        foreach ($members as $m) {
            $ctor_methods = $m->constructionMethods();
            foreach ($m->methods as $meth) {
                if ($meth->node === null || isset($ctor_methods[$meth->lc()])
                    || ($meth->storage->capabilities & ~\Psalm\Storage\Capabilities::MUTATION_FREE) !== 0
                ) {
                    continue;
                }
                if ($finder->findFirst($meth->node->stmts ?? [], static fn(\PhpParser\Node $n): bool =>
                    ($n instanceof \PhpParser\Node\Expr\Assign || $n instanceof \PhpParser\Node\Expr\AssignOp)
                    && self::thisPropName($n->var) !== null) !== null
                ) {
                    return $this->family_cache_writes = true;
                }
            }
        }
        return $this->family_cache_writes = false;
    }

    /** @var ?bool memo of familyHasCacheWrites() */
    private ?bool $family_cache_writes = null;

    /** @var array<string, bool> memo of methodWritesThis() by method name */
    private array $method_writes_this = [];

    /**
     * Whether the method's body writes a field of `$this` (an assignment, compound assignment, increment,
     * decrement or unset through `$this->f`, or an element/property of one). For a value type such a method
     * (a memo write, `__clone` resetting a memo) must run on the value itself: the own-handle wrapper that
     * forwards to the hierarchy `__impl` on a temporary copy would lose the write.
     */
    public function methodWritesThis(MethodModel $m): bool
    {
        $key = $m->lc();
        if (isset($this->method_writes_this[$key])) {
            return $this->method_writes_this[$key];
        }
        if ($m->node === null) {
            return $this->method_writes_this[$key] = false;
        }
        $finder = new \PhpParser\NodeFinder();
        $found = $finder->findFirst($m->node->stmts ?? [], static function (\PhpParser\Node $n): bool {
            if ($n instanceof \PhpParser\Node\Expr\Assign || $n instanceof \PhpParser\Node\Expr\AssignOp
                || $n instanceof \PhpParser\Node\Expr\AssignRef
                || $n instanceof \PhpParser\Node\Expr\PreInc || $n instanceof \PhpParser\Node\Expr\PreDec
                || $n instanceof \PhpParser\Node\Expr\PostInc || $n instanceof \PhpParser\Node\Expr\PostDec
            ) {
                $t = $n->var;
            } elseif ($n instanceof \PhpParser\Node\Stmt\Unset_) {
                foreach ($n->vars as $v) {
                    if (self::thisPropName($v) !== null) {
                        return true;
                    }
                }
                return false;
            } else {
                return false;
            }
            return self::thisPropName($t) !== null;
        }) !== null;
        return $this->method_writes_this[$key] = $found;
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
     * `@psalm-immutable` (capabilities within MUTATION_FREE) leaf classes on the pilot allowlist; reads are direct
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
    /**
     * Whether every concrete member of this hierarchy reads $name through a plain `&T`: construction-only in a
     * RefCell member, a plain (non-cell, non-Late) field in an immutable member. The enum accessor is then
     * `PropRef::Plain` in every arm, so a read through the enum is a place for the rest of its expression.
     */
    public function plainAcrossHierarchy(string $name): bool
    {
        if ($this->concrete === []) {
            return false;
        }
        foreach ($this->concrete as $c) {
            $f = $c->fields[$name] ?? null;
            if ($f === null || $f->isLate()) {
                return false;
            }
            if ($c->immutable()) {
                if ($c->cellKind($f) !== '') {
                    return false;
                }
            } elseif (!isset($c->constructionOnlyFields()[$name])) {
                return false;
            }
        }
        return true;
    }

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
            && ($this->storage->capabilities & ~\Psalm\Storage\Capabilities::MUTATION_FREE) === 0
            && ($allow_ext || !$this->externally_written)
            && ($allow_memo || $this->postConstructionWrittenFields() === [])
            && $this->noHelperConstructionWrites();
    }

    /**
     * How a field of this class is stored when the class is an `Rc<T>` (immutable) wrapper: '' for a plain field
     * (read as `&T`), 'Cell' for a Copy field written after construction, 'RefCell' for any other such field.
     * A RefCell class (the default) stores every field behind the object's own RefCell.
     */
    public function cellKind(FieldModel $f): string
    {
        if (!$this->immutable() || !isset($this->interiorMutFields()[$f->name])) {
            return '';
        }
        // A Late field (no default -> deferred init) can't be a Cell: Cell<T> requires T: Copy and Late<T> never is.
        // Use RefCell for Late (and for any non-Copy field); Cell only for a plain Copy field.
        return (!$f->isLate() && $f->type->isCopy()) ? 'Cell' : 'RefCell';
    }

    /**
     * An immutable class stored BY VALUE (pzoom's `TAtomic` inside `Vec<TAtomic>`): the handle struct owns its
     * fields inline instead of an `Rc`, so a hierarchy enum carries the payload inline, a list of them is one
     * contiguous allocation, and reads never chase a pointer. Copies are copies (PHP identity becomes structural
     * equality, as in pzoom). Gated by VALUE_TYPES=1 until measured.
     */
    public function valueType(): bool
    {
        if ($this->value_type !== null) {
            return $this->value_type;
        }
        $v = getenv('VALUE_TYPES');
        if ($v === false || $v === '' || $v === '0' || !$this->immutable() || self::$program === null) {
            return $this->value_type = false;
        }
        // `@rust-handle` on the class: kept a shared (Rc) object by request of the source (Psalm\Type\Atomic\SourceSpan:
        // an optional field of every atomic, cheaper as an 8-byte handle than as an inline value)
        $doc = $this->node?->getDocComment()?->getText() ?? '';
        if (str_contains($doc, '@rust-handle')) {
            if (getenv('IMMUTABLE_DIAG') !== false && getenv('IMMUTABLE_DIAG') !== '') {
                fwrite(STDERR, "[value-excluded] " . $this->fqcn . " stays Rc: @rust-handle\n");
            }
            return $this->value_type = false;
        }
        // VALUE_TYPES_EXCLUDE: comma-separated class names kept on the Rc path. A value that is copied into many
        // containers (Union: every scope map) costs more in copies than the handle it replaces (c118: +2.5% wall,
        // +15% peak memory); pzoom shares its unions through Rc in scopes and inlines only the atomics.
        foreach (explode(',', (string) getenv('VALUE_TYPES_EXCLUDE')) as $ex) {
            if ($ex !== '' && strcasecmp(trim($ex), $this->fqcn) === 0) {
                if (getenv('IMMUTABLE_DIAG') !== false && getenv('IMMUTABLE_DIAG') !== '') {
                    fwrite(STDERR, "[value-excluded] " . $this->fqcn . " stays Rc: VALUE_TYPES_EXCLUDE\n");
                }
                return $this->value_type = false;
            }
        }
        // VALUE_TYPES_INCLUDE: when set, ONLY the listed classes (matched on the class or any of its ancestors, so a
        // whole hierarchy follows its root) become values; everything else stays on the Rc path (targeted
        // experiments: `VALUE_TYPES=1 VALUE_TYPES_INCLUDE=PhpToken` for the parser's tokens)
        $inc = (string) getenv('VALUE_TYPES_INCLUDE');
        if ($inc !== '') {
            $listed = false;
            for ($c = $this; $c !== null && !$listed; $c = $c->parent) {
                foreach (explode(',', $inc) as $in) {
                    if ($in !== '' && strcasecmp(trim($in), $c->fqcn) === 0) {
                        $listed = true;
                        break;
                    }
                }
            }
            if (!$listed) {
                return $this->value_type = false;
            }
        }
        if (self::$program->identityObserved($this)) {
            if (getenv('IMMUTABLE_DIAG') !== false && getenv('IMMUTABLE_DIAG') !== '') {
                fwrite(STDERR, "[value-identity] " . $this->fqcn . " stays Rc: its identity is observed\n");
            }
            return $this->value_type = false;
        }
        // a value is copied on every read: a write through a copy is lost. An Rc<T> with Cell fields shares such
        // writes (`$type->from_docblock = false` on an atomic taken out of a union), a value cannot, so only
        // recomputable cache writes (suppressed writes inside mutation-free methods) are tolerated: no field may
        // be written from outside the class (through any base- or interface-typed handle either) and every own
        // post-construction write must sit in a mutation-free method.
        $ext = [];
        for ($m = $this; $m !== null; $m = $m->parent) {
            foreach ($m->ext_written_fields as $fld => $_) {
                $ext[$fld] = true;
            }
        }
        foreach ($this->storage->class_implements as $lc => $_) {
            foreach ((self::$program->classes[$lc] ?? null)?->ext_written_fields ?? [] as $fld => $_) {
                $ext[$fld] = true;
            }
        }
        if ($ext !== []) {
            if (getenv('IMMUTABLE_DIAG') !== false && getenv('IMMUTABLE_DIAG') !== '') {
                fwrite(STDERR, "[value-extwrite] " . $this->fqcn . " stays Rc: written from outside: " . implode(',', array_keys($ext)) . "\n");
            }
            return $this->value_type = false;
        }
        if (($w = $this->nonCachePostConstructionWrites()) !== []) {
            if (getenv('IMMUTABLE_DIAG') !== false && getenv('IMMUTABLE_DIAG') !== '') {
                fwrite(STDERR, "[value-writes] " . $this->fqcn . " stays Rc: written after construction outside mutation-free methods: " . implode(',', $w) . "\n");
            }
            return $this->value_type = false;
        }
        // while this class is being decided, a field reaching it inline means an infinite value: not a value type
        $this->value_type = false;
        $family = [];
        $root = $this;
        while ($root->parent !== null) {
            $root = $root->parent;
        }
        $family[$root->lc()] = true;
        foreach ($root->concrete as $c) {
            for ($m = $c; $m !== null; $m = $m->parent) {
                $family[$m->lc()] = true;
            }
        }
        $visiting = [$this->lc() => true];
        foreach ($this->fields as $f) {
            if (self::inlineReaches($f->type, $family, $visiting)) {
                return $this->value_type = false;
            }
        }
        // pzoom keeps TAtomic at 40 bytes and TUnion at 64 (big variants boxed): a value copied on every
        // by-value pass must stay small, or the copies cost more than the Rc bump they replace and deep
        // recursion overflows the stack (c113). Classes above the cap stay Rc handles.
        $cap = (int) (getenv('VALUE_TYPES_CAP') ?: '96');
        $box_cap = self::boxCap();
        $size = 0;
        $sizing = [$this->lc() => true];
        $boxed = [];
        foreach ($this->fields as $f) {
            if ($f->is_static) {
                continue;
            }
            $fs = self::inlineSize($f->type, $sizing);
            // pzoom boxes the unions inside its atomics (`Box<TUnion>`): a large field of a value is a Box,
            // so the value itself stays small enough to copy (the field's accessors deref transparently)
            if ($fs > $box_cap && !$f->isLate() && !isset($this->interiorMutFields()[$f->name])) {
                $boxed[$f->name] = true;
                $fs = 8;
            }
            $size += $fs;
        }
        $this->boxed_fields = $boxed;
        if ($size > $cap) {
            if (getenv('IMMUTABLE_DIAG') !== false && getenv('IMMUTABLE_DIAG') !== '') {
                fwrite(STDERR, "[value-size] " . $this->fqcn . " stays Rc: ~" . $size . " bytes > cap " . $cap . "\n");
            }
            return $this->value_type = false;
        }
        $this->inline_size = $size;
        if (getenv('IMMUTABLE_DIAG') !== false && getenv('IMMUTABLE_DIAG') !== '') {
            fwrite(STDERR, "[value-type] " . $this->fqcn . " ~" . $size . " bytes" . ($boxed !== [] ? " boxed=" . implode(',', array_keys($boxed)) : '') . "\n");
        }
        return $this->value_type = true;
    }

    /** @var ?bool memo of valueType() */
    private ?bool $value_type = null;
    /** @var ?int estimated inline byte size once valueType() is true */
    private ?int $inline_size = null;
    /** @var array<string, true> fields of a value type stored as `Box<T>` (see valueType()) */
    private array $boxed_fields = [];

    /**
     * VALUE_BOX_CAP: a field of a value type above this many bytes lives behind a copy-on-write Rc (a cheap
     * clone; pzoom's Box<TUnion> inside its atomics).
     */
    public static function boxCap(): int
    {
        return (int) (getenv('VALUE_BOX_CAP') ?: '40');
    }

    /**
     * VALUE_VARIANT_BOX_CAP: a value's payload in its hierarchy enum above this many bytes is a Box (the enum
     * stays the size of its inline members; an atomic of ~80 bytes stays inline by default).
     */
    public static function variantBoxCap(): int
    {
        return (int) (getenv('VALUE_VARIANT_BOX_CAP') ?: '96');
    }

    /** @return array<string, true> the fields stored as `Box<T>` (only a value type has any) */
    public function boxedFields(): array
    {
        $this->valueType();
        return $this->boxed_fields;
    }

    /** Whether this value's payload in its hierarchy enum is a `Box<Own>` (its inline size is above the box cap). */
    public function boxedVariant(): bool
    {
        return $this->valueType() && ($this->inline_size ?? 0) > self::variantBoxCap();
    }

    /** The estimated inline size of a value of this type, or 8 for a handle. */
    public function payloadSize(): int
    {
        return $this->valueType() && !$this->boxedVariant() ? ($this->inline_size ?? 8) : 8;
    }

    /**
     * Estimated inline byte size of a value of type $t: handles 8, Str/List 16, Map 8, options +8, a value
     * class its fields, a hierarchy enum 8 + its largest inline member. Rough, for the value-type cap only.
     *
     * @param array<string, true> $sizing classes being sized (a cycle counts as a handle)
     */
    public static function inlineSize(RustType $t, array &$sizing): int
    {
        switch ($t->kind) {
            case RustType::BOOL:
                return 1;
            case RustType::UNIT:
            case RustType::NEVER:
                return 0;
            case RustType::INT:
            case RustType::FLOAT:
            case RustType::SYM:
            case RustType::MAP:
            case RustType::ANY_OBJECT:
            case RustType::RESOURCE:
            case RustType::GENERIC:
                return 8;
            case RustType::STR:
            case RustType::LIST:
            case RustType::CLOSURE:
            case RustType::DYN_CALLABLE:
            case RustType::RT_GENERIC:
                return 16;
            case RustType::ARRAY_KEY:
            case RustType::MIXED:
                return 24;
            case RustType::OPTION:
                return 8 + self::inlineSize($t->inner(), $sizing);
            case RustType::TUPLE:
                $n = 0;
                foreach ($t->params as $p) {
                    $n += self::inlineSize($p, $sizing);
                }
                return $n;
            case RustType::SHAPE:
                $n = 0;
                foreach ($t->fields as $p) {
                    $n += self::inlineSize($p, $sizing);
                }
                return $n;
            case RustType::UNION:
                $max = 0;
                foreach ($t->params as $p) {
                    $max = max($max, self::inlineSize($p, $sizing));
                }
                return 8 + $max;
            case RustType::CLASS_:
                $c = self::$program?->classOf($t);
                if ($c === null || isset($sizing[$c->lc()])) {
                    return 8;
                }
                if ($c->isLeaf()) {
                    return $c->valueType() ? ($c->inline_size ?? 8) : 8;
                }
                // a hierarchy handle: an enum over its concrete members (a large value payload is boxed)
                $max = 0;
                foreach ($c->concrete as $m) {
                    $max = max($max, $m->payloadSize());
                }
                return 8 + $max;
        }
        return 8;
    }

    /** The program, for class lookups from field types (set once by Program). */
    public static ?Program $program = null;

    /**
     * Whether a value of type $t stored inline (no Rc/List/Map between) can contain a member of $family: an
     * option, tuple, union or shape stores its members inline; a value class stores its fields inline.
     *
     * @param array<string, true> $family
     * @param array<string, true> $visiting
     */
    private static function inlineReaches(RustType $t, array $family, array &$visiting): bool
    {
        switch ($t->kind) {
            case RustType::OPTION:
            case RustType::TUPLE:
            case RustType::UNION:
                foreach ($t->params as $p) {
                    if (self::inlineReaches($p, $family, $visiting)) {
                        return true;
                    }
                }
                return false;
            case RustType::SHAPE:
                foreach ($t->fields as $p) {
                    if (self::inlineReaches($p, $family, $visiting)) {
                        return true;
                    }
                }
                return false;
            case RustType::CLASS_:
                $c = self::$program?->classOf($t);
                if ($c === null) {
                    return false;
                }
                if (isset($family[$c->lc()])) {
                    return true;
                }
                if (isset($visiting[$c->lc()])) {
                    return true; // a cycle through another class being decided: conservative
                }
                if (!$c->immutable()) {
                    return false; // an Rc<RefCell> handle: an indirection
                }
                $visiting[$c->lc()] = true;
                foreach ($c->fields as $f) {
                    if (self::inlineReaches($f->type, $family, $visiting)) {
                        return true;
                    }
                }
                return false;
            default:
                return false;
        }
    }

    public function immutable(): bool
    {
        // @psalm-immutable == LEVEL_INTERNAL_READ (no internal writes post-construction, safe for Rc<T>);
        // LEVEL_INTERNAL_READ_WRITE (@psalm-external-mutation-free) allows memoization writes and needs RefCell.
        if (($this->storage->capabilities & ~\Psalm\Storage\Capabilities::MUTATION_FREE) !== 0) {
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
        // The same, tolerating memo writes (nullable lazily computed values, bool flags): as an Rc<T> those fields
        // become Cell/RefCell (interiorMutFields), so the write stays visible to every holder of the handle, while
        // every other field is a plain `&T` read (Psalm\Internal\Clause: 8 M clone reads of $possibilities per run
        // with the RefCell layout). NO_IMMUTABLE_MEMO=1 keeps the strict rule.
        $nim = getenv('NO_IMMUTABLE_MEMO');
        if ($this->parent === null
            && !$this->externally_written
            && ($nim === false || $nim === '' || $nim === '0')
            && $this->memoOnlyPostConstructionWrites()
            && $this->noHelperConstructionWrites()
        ) {
            return true;
        }
        if (($diag = getenv('IMMUTABLE_DIAG')) !== false && $diag !== '' && str_contains(strtolower($this->fqcn), strtolower($diag))) {
            fwrite(STDERR, "[immutable-diag] " . $this->fqcn
                . " parent=" . ($this->parent === null ? 'none' : $this->parent->fqcn)
                . " ext_written=" . var_export($this->externally_written, true)
                . " ext_fields=" . implode(',', array_keys($this->ext_written_fields))
                . " post_ctor=" . implode(',', array_keys($this->postConstructionWrittenFields()))
                . " helper_ok=" . var_export($this->noHelperConstructionWrites(), true)
                . " memo_only=" . var_export($this->memoOnlyPostConstructionWrites(), true)
                . " leaf=" . var_export($this->isLeaf(), true)
                . "\n");
        }
        // Memoizing standalone leaves (Type\Union: id/exact_id/checked): the memo fields become per-field
        // Cell/RefCell through interiorMutFields(), every other field is a plain `&T` read.
        if ($this->parent === null
            && !$this->externally_written
            && $this->noHelperConstructionWrites()
            && $this->memoOnlyPostConstructionWrites()
        ) {
            return true;
        }
        return false;
    }

    /**
     * Every post-construction `$this->field =` write of this class is a memo: the field is nullable (a lazily
     * computed value) or a bool flag, never a field the object's identity or contents live in.
     */
    private function memoOnlyPostConstructionWrites(): bool
    {
        $written = $this->postConstructionWrittenFields();
        if ($written === []) {
            return false;
        }
        foreach ($written as $name => $_) {
            $f = $this->fields[$name] ?? null;
            if ($f === null) {
                return false;
            }
            if (!$f->type->isCopy() && !$f->type->isOption()) {
                return false;
            }
        }
        return true;
    }
}
