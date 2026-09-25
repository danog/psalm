<?php

declare(strict_types=1);

namespace Psalm\Storage;

use Override;
use Psalm\Type\Atomic;
use Stringable;

use function count;
use function get_object_vars;
use function str_contains;

/**
 * @psalm-immutable
 * @api
 */
abstract class Assertion implements Stringable
{
    use ImmutableNonCloneableTrait;
    use UnserializeMemoryUsageSuppressionTrait;

    /** @psalm-mutation-free */
    abstract public function getNegation(): Assertion;

    /** @psalm-mutation-free */
    abstract public function isNegationOf(self $assertion): bool;

    /** @psalm-mutation-free */
    #[Override]
    abstract public function __toString(): string;

    /**
     * Memoized getHash(). Not serialized: the ids are process-local (see __serialize).
     */
    private ?int $hash_memo = null;

    /** @var array<string, int> */
    private static array $hash_ids = [];

    /**
     * The assertion's identity inside a clause (pzoom's `Assertion::to_hash`): an interned id of its string
     * form, so equal assertions share it and clauses key their assertions by int. As with Atomic's memos, an
     * assertion mentioning a type variable (`` `_0 ``) is re-read each time, since its string grows with the
     * variable's bounds.
     *
     * @psalm-mutation-free
     * @psalm-suppress ImpureStaticProperty the table only grows; an id never changes meaning
     */
    final public function getHash(): int
    {
        if ($this->hash_memo !== null) {
            return $this->hash_memo;
        }
        $string = $this->__toString();
        $hash = self::$hash_ids[$string] ??= count(self::$hash_ids);
        if (!str_contains($string, '`')) {
            /** @psalm-suppress ImpurePropertyAssignment, InaccessibleProperty Cache */
            $this->hash_memo = $hash;
        }
        return $hash;
    }

    /**
     * @return array<string, mixed>
     * @psalm-mutation-free
     */
    public function __serialize(): array
    {
        $vars = get_object_vars($this);
        unset($vars['hash_memo']);
        return $vars;
    }

    /**
     * @psalm-pure
     */
    public function isNegation(): bool
    {
        return false;
    }

    /**
     * @psalm-pure
     */
    public function hasEquality(): bool
    {
        return false;
    }

    /**
     * @psalm-pure
     */
    public function getAtomicType(): ?Atomic
    {
        return null;
    }

    /**
     * @return static
     * @psalm-mutation-free
     */
    public function setAtomicType(Atomic $type): self
    {
        return $this;
    }
}
