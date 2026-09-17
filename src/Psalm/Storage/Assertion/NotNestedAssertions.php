<?php

declare(strict_types=1);

namespace Psalm\Storage\Assertion;

use Override;
use Psalm\Storage\Assertion;

use function array_map;
use function implode;

/**
 * @psalm-immutable
 * @api
 */
final class NotNestedAssertions extends Assertion
{
    /** @param array<string, list<list<Assertion>>> $assertions */
    public function __construct(public readonly array $assertions)
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
        return new NestedAssertions($this->assertions);
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

        return '!@{' . implode(',', $vars) . '}';
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
