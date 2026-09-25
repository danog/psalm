<?php

declare(strict_types=1);

namespace Psalm\Storage\Assertion;

use Override;
use Psalm\Storage\Assertion;

/**
 * @psalm-immutable
 * @api
 */
final class IsLessThanOrEqualTo extends Assertion
{
    public function __construct(public readonly int $value)
    {
    }

    #[Override]
    public function getNegation(): Assertion
    {
        return new IsGreaterThan($this->value);
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
        return '!>' . $this->value;
    }

    #[Override]
    public function isNegationOf(Assertion $assertion): bool
    {
        return $assertion instanceof IsGreaterThan && $this->value === $assertion->value;
    }

    /**
     * @psalm-pure
     */
    public function doesFilterNullOrFalse(): bool
    {
        return false;
    }
}
