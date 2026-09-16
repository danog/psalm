<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

/**
 * @internal
 */
final class FunctionModel
{
    /** @var list<RustType> */
    public array $param_types = [];

    /**
     * Owned/borrowed (axis 5): parameter indices that are non-escaping read-only -> emitted as `&T` so
     * callers borrow instead of cloning. See Program::computeBorrowParams.
     * @var array<int, true>
     */
    public array $borrow_params = [];

    public RustType $return_type;

    /** Panic-based model: no function returns Result (default false). See MethodModel::$throws. */
    public bool $throws = false;

    public function __construct(
        public readonly string $fq_name,
        public readonly FunctionRecord $record,
    ) {
        $this->return_type = RustType::unit();
    }

    public function rustName(): string
    {
        return Names::function($this->fq_name);
    }

    public function path(): string
    {
        return Names::functionPath($this->fq_name);
    }
}
