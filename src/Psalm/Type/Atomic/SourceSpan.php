<?php

declare(strict_types=1);

namespace Psalm\Type\Atomic;

/**
 * Where a docblock type was written: byte offsets inside the docblock and the text as typed (an import alias),
 * kept only by the types the type parser builds. One optional handle on Atomic instead of three fields, so the
 * atomics that never carry a position (the vast majority) stay small.
 *
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
