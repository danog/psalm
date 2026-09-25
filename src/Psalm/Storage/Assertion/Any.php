<?php

declare(strict_types=1);

namespace Psalm\Storage\Assertion;

use Override;
use Psalm\Storage\Assertion;

/**
 * @psalm-immutable
 * @api
 */
final class Any extends Assertion
{
    #[Override]
    public function getNegation(): Assertion
    {
        return $this;
    }

    /**
     * @psalm-pure
     */
    public function __toString(): string
    {
        return 'mixed';
    }

    /**
     * @psalm-pure
     */
    #[Override]
    public function isNegationOf(Assertion $assertion): bool
    {
        return false;
    }
}
