<?php

declare(strict_types=1);

namespace Psalm\Storage\Assertion;

use Override;
use Psalm\Storage\Assertion;

/**
 * @psalm-immutable
 * @api
 */
final class DoesNotHaveExactCount extends Assertion
{
    /** @param positive-int $count */
    public function __construct(public readonly int $count)
    {
    }

    /**
     * @psalm-pure
     */
    #[Override]
    public function isNegation(): bool
    {
        return true;
    }

    #[Override]
    public function getNegation(): Assertion
    {
        return new HasExactCount($this->count);
    }

    public function __toString(): string
    {
        return '!has-exact-count-' . $this->count;
    }

    #[Override]
    public function isNegationOf(Assertion $assertion): bool
    {
        return $assertion instanceof HasExactCount && $assertion->count === $this->count;
    }
}
