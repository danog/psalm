<?php

declare(strict_types=1);

namespace Psalm\Storage\Assertion;

use Override;
use Psalm\Storage\Assertion;
use Psalm\Storage\UnserializeMemoryUsageSuppressionTrait;

use function array_map;
use function implode;

/**
 * @psalm-immutable
 * @api
 */
final class NestedAssertions extends Assertion
{
    use UnserializeMemoryUsageSuppressionTrait;
    /** @param array<string, list<list<Assertion>>> $assertions */
    public function __construct(public readonly array $assertions)
    {
    }

    #[Override]
    public function getNegation(): Assertion
    {
        return new NotNestedAssertions($this->assertions);
    }

    public function __toString(): string
    {
        $vars = [];
        foreach ($this->assertions as $var_id => $clauses) {
            $ors = [];
            foreach ($clauses as $clause) {
                $ors[] = '[' . implode('|', array_map(static fn(Assertion $assertion): string => (string) $assertion, $clause)) . ']';
            }
            $vars[] = $var_id . ':' . implode('&', $ors);
        }

        return '@{' . implode(',', $vars) . '}';
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
