<?php

declare(strict_types=1);

namespace Psalm\Internal\DataFlow;

use Psalm\Storage\ImmutableNonCloneableTrait;

/**
 * @psalm-immutable
 * @internal
 */
final class Path
{
    use ImmutableNonCloneableTrait;

    public const TYPE_CLOSURE_USE = (1 << 0);
    public const TYPE_CLOSURE_RETURN = (1 << 1);
    public const TYPE_CLOSURE_THIS = (1 << 2);

    /**
     * @param int-mask-of<self::TYPE_*> $type
     */
    public function __construct(
        public readonly int $type,
        public readonly int $length,
        public readonly int $added_taints = 0,
        public readonly int $removed_taints = 0,
    ) {
    }
}
