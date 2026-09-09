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

    /** @var list<ClassModel> all ancestors (parents, transitively) and implemented interfaces */
    public array $ancestors = [];

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
        return $this->isConcrete() && $this->children === [] && !$this->isEnum();
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
        return $this->handle() . 'Obj';
    }

    public function path(): string
    {
        return Names::classPath($this->fqcn);
    }

    public function ownPath(): string
    {
        return 'crate::' . Names::modulePath($this->fqcn) . '::' . $this->ownHandle();
    }

    public function objPath(): string
    {
        return 'crate::' . Names::modulePath($this->fqcn) . '::' . $this->objStruct();
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
}
