<?php

declare(strict_types=1);

namespace Psalm\Type\Atomic;

use Override;

/**
 * Denotes the `mixed` type, but not empty.
 * Generated for `$x` inside the `if` statement `if ($x) {...}` when `$x` is `mixed` outside.
 *
 * @psalm-immutable
 */
final class TNonEmptyMixed extends TMixed
{
    #[Override]
    public function getId(bool $exact = true, bool $nested = false): string
    {
        return 'non-empty-mixed';
    }
}
