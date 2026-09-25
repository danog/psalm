<?php

declare(strict_types=1);

namespace Psalm\Storage\Assertion;

use Override;
use Psalm\Storage\Assertion;

/**
 * @psalm-immutable
 * @api
 */
final class DoesNotHaveAtLeastCount extends Assertion
{
    /** @param positive-int $count */
    public function __construct(public readonly int $count)
    {
    }

    #[Override]
    public function getNegation(): Assertion
    {
        return new HasAtLeastCount($this->count);
    }

    /**
     * @psalm-pure
     */
    #[Override]
    public function isNegation(): bool
    {
        return true;
    }

    public function __toString(): string
    {
        return '!has-at-least-' . $this->count;
    }

    #[Override]
    public function isNegationOf(Assertion $assertion): bool
    {
        return $assertion instanceof HasAtLeastCount && $this->count === $assertion->count;
    }
}
