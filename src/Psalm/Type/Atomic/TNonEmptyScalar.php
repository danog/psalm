<?php

declare(strict_types=1);

namespace Psalm\Type\Atomic;

use Override;

/**
 * Denotes a `scalar` type that is also non-empty.
 *
 * @psalm-immutable
 * @api
 */
final class TNonEmptyScalar extends TScalar
{
    /**
     * @psalm-pure
     */
    #[Override]
    protected function computeId(bool $exact = true, bool $nested = false): string
    {
        return 'non-empty-scalar';
    }
}
