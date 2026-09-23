<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

/**
 * What the owned/borrowed analysis of one body knows about its surroundings (see Program::borrowSafeParams).
 *
 * @internal
 */
final class BorrowContext
{
    /**
     * @param array<string, RustType> $param_types the body's parameters by name
     */
    public function __construct(
        public readonly ?ClassModel $cls,
        public readonly ?FunctionRecord $record,
        public readonly array $param_types,
    ) {
    }
}
