<?php

declare(strict_types=1);

namespace Psalm\Type\Atomic;

/**
 * Where a docblock type was written: byte offsets inside the docblock and the text as typed (an import alias),
 * kept only by the types the type parser builds. One optional handle on Atomic instead of three fields, so the
 * atomics that never carry a position (the vast majority) stay small.
 *
 * @rust-handle a shared object even when immutable classes become values: as a handle the optional span costs an
 *              atomic 8 bytes, whereas an inline value would widen every atomic
 * @psalm-immutable
 */
final class SourceSpan
{
    public function __construct(
        public readonly ?int $offset_start,
        public readonly ?int $offset_end,
        public readonly ?string $text,
    ) {
    }
}
