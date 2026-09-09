<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

/**
 * A piece of Rust expression code together with the Rust type it evaluates to.
 *
 * @internal
 */
final class Val
{
    public function __construct(
        public readonly string $code,
        public readonly RustType $type,
    ) {
    }

    public function with(string $code): Val
    {
        return new Val($code, $this->type);
    }
}
