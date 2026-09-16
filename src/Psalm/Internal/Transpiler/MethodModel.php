<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Stmt\ClassMethod;
use Psalm\Storage\MethodStorage;

/**
 * @internal
 */
final class MethodModel
{
    /** @var list<RustType> */
    public array $param_types = [];

    /**
     * Owned/borrowed (axis 5): parameter indices received as `&T` (non-escaping read-only). Finalised by
     * Program::computeBorrowAgreement -- for a dispatched method it is the agreement across the whole
     * override/interface group so every impl keeps a uniform signature.
     * @var array<int, true>
     */
    public array $borrow_params = [];

    /**
     * Owned/borrowed (axis 5): per-body escape-analysis result (which params THIS body could borrow),
     * before dispatch-group agreement. @var array<int, true>
     */
    public array $local_borrow = [];

    public RustType $return_type;

    /** static method whose body refers to `static` (needs a copy per calling class) */
    public bool $uses_lsb = false;

    /**
     * Axis-8 (Result-only-where-throws): whether this method's emitted signature returns
     * `Result<T, Throw>` (true) or bare `T` (false). Default true = current behavior. Set false only by
     * Program::computeThrows() for provably non-throwing methods. When false, the signature, the return
     * emission (returnCode/implicitReturn), AND every static call site (which drops the trailing `?`) must
     * all agree — they all read this flag. See Program::computeThrows() for the (conservative) predicate.
     */
    public bool $throws = false;

    /**
     * For a method inherited from a class in another crate: the inherited method. Its body is emitted
     * again for `$declaring` (the topmost class of this crate) with `$this` bound to that class.
     */
    public ?MethodModel $import_of = null;

    public function __construct(
        public readonly string $name,
        public readonly ClassModel $declaring,
        public readonly MethodStorage $storage,
        public readonly ?ClassMethod $node,
        public readonly ?FunctionRecord $record,
    ) {
        $this->return_type = RustType::unit();
    }

    /** A copy of this method as if declared by `$declaring` (a subclass in another crate). */
    public function importedInto(ClassModel $declaring): MethodModel
    {
        $m = new MethodModel($this->name, $declaring, $this->storage, $this->node, $this->record);
        $m->param_types = $this->param_types;
        $m->return_type = $this->return_type;
        $m->uses_lsb = $this->uses_lsb;
        $m->borrow_params = $this->borrow_params;
        $m->import_of = $this;
        return $m;
    }

    /** The method as originally declared (following imports). */
    public function origin(): MethodModel
    {
        $m = $this;
        while ($m->import_of !== null) {
            $m = $m->import_of;
        }
        return $m;
    }

    public function lc(): string
    {
        return strtolower($this->name);
    }

    public function rustName(): string
    {
        return Names::method($this->name);
    }

    public function isPrivate(): bool
    {
        return $this->storage->visibility === \Psalm\Internal\Analyzer\ClassLikeAnalyzer::VISIBILITY_PRIVATE;
    }

    public function isStatic(): bool
    {
        return $this->storage->is_static;
    }

    public function isAbstract(): bool
    {
        return $this->storage->abstract || $this->node === null || $this->node->stmts === null;
    }
}
