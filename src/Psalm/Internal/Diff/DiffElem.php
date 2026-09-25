<?php

declare(strict_types=1);

namespace Psalm\Internal\Diff;

use PhpParser\Node\Stmt;
use Psalm\Storage\ImmutableNonCloneableTrait;

/**
 * @internal
 * @psalm-immutable
 */
final class DiffElem
{
    use ImmutableNonCloneableTrait;

    public const TYPE_KEEP = 0;
    public const TYPE_REMOVE = 1;
    public const TYPE_ADD = 2;
    public const TYPE_REPLACE = 3;
    public const TYPE_KEEP_SIGNATURE = 4;

    /**
     * @psalm-mutation-free
     */
    public function __construct(
        /** @var int One of the TYPE_* constants */
        public readonly int $type,
        /** Is null for add operations */
        public readonly Stmt|string|null $old,
        /** Is null for remove operations */
        public readonly Stmt|string|null $new,
    ) {
    }
}
