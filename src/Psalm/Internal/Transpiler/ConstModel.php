<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Expr;
use Psalm\Storage\ClassConstantStorage;

/**
 * @internal
 */
final class ConstModel
{
    public function __construct(
        public readonly string $name,
        public readonly RustType $type,
        public readonly ClassModel $declaring,
        public readonly ClassConstantStorage $storage,
        public readonly ?Expr $expr,
    ) {
    }

    public function rustName(): string
    {
        return Names::constant($this->name);
    }
}
