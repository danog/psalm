<?php

declare(strict_types=1);

namespace Psalm\Storage\Assertion;

use Override;
use Psalm\Storage\Assertion;
use UnexpectedValueException;

/**
 * @psalm-immutable
 * @api
 */
final class HasArrayKey extends Assertion
{
    public function __construct(public readonly string $key)
    {
    }

    /**
     * @psalm-pure
     */
    #[Override]
    public function getNegation(): Assertion
    {
        throw new UnexpectedValueException('This should never be called');
    }

    public function __toString(): string
    {
        return 'has-array-key-' . $this->key;
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
