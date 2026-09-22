<?php

declare(strict_types=1);

namespace Psalm\Storage\Assertion;

use Override;
use Psalm\Storage\Assertion;
use Psalm\Type\Union;
use Psalm\Storage\UnserializeMemoryUsageSuppressionTrait;

/**
 * @psalm-immutable
 * @api
 */
final class InArray extends Assertion
{
    use UnserializeMemoryUsageSuppressionTrait;
    public function __construct(public readonly Union $type)
    {
    }

    #[Override]
    public function getNegation(): Assertion
    {
        return new NotInArray($this->type);
    }

    public function __toString(): string
    {
        return 'in-array-' . $this->type->getId();
    }

    #[Override]
    public function isNegationOf(Assertion $assertion): bool
    {
        return $assertion instanceof NotInArray && $this->type->getId() === $assertion->type->getId();
    }
}
