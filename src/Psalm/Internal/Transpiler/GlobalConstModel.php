<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Expr;

/**
 * A namespace-level `const` or `define()`.
 *
 * @internal
 */
final class GlobalConstModel
{
    public function __construct(
        public readonly string $name,
        public readonly RustType $type,
        public readonly Expr $expr,
        public readonly FunctionRecord $record,
    ) {
    }
}
