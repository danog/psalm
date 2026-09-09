<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Expr;
use Psalm\Storage\PropertyStorage;

/**
 * @internal
 */
final class FieldModel
{
    public function __construct(
        public readonly string $name,
        public readonly RustType $type,
        public readonly ClassModel $declaring,
        public readonly PropertyStorage $storage,
        public readonly ?Expr $default,
        public readonly bool $has_default,
        public readonly bool $is_static,
    ) {
    }

    public function rustName(): string
    {
        return Names::field($this->name);
    }

    /** Base name of the generated accessors (`p_x()`, `p_x_get()`, `p_x_mut()`, `set_p_x()`). */
    public function acc(): string
    {
        return 'p_' . Names::field($this->name);
    }

    /**
     * Fields that start uninitialized are stored as Late<T>: typed properties without a default
     * (PHP leaves them uninitialized: absent from get_object_vars, an Error when read), and fields
     * whose Rust type has no Default.
     */
    public function isLate(): bool
    {
        if ($this->has_default) {
            return false;
        }
        return !$this->type->hasDefault() || ($this->storage->signature_type !== null && !$this->is_static);
    }

    public function storageType(): string
    {
        return $this->isLate() ? 'Late<' . $this->type->toRust() . '>' : $this->type->toRust();
    }
}
