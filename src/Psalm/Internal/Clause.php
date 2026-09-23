<?php

declare(strict_types=1);

namespace Psalm\Internal;

use Override;
use Psalm\Storage\Assertion;
use Psalm\Storage\ImmutableNonCloneableTrait;
use Psalm\Type\Atomic\TClassConstant;
use Psalm\Type\Atomic\TEnumCase;
use Psalm\Type\Atomic\TLiteralFloat;
use Psalm\Type\Atomic\TLiteralInt;
use Psalm\Type\Atomic\TLiteralString;
use Stringable;

use function array_diff_key;
use function array_keys;
use function assert;
use function count;
use function implode;
use function is_int;
use function ksort;
use function reset;
use function substr;

/**
 * @internal
 * @psalm-immutable
 */
final class Clause implements Stringable
{
    use ImmutableNonCloneableTrait;

    public int $creating_object_id;

    /**
     * An array of strings of the form
     * [
     *     '$a' => ['falsy'],
     *     '$b' => ['!falsy'],
     *     '$c' => ['!null'],
     *     '$d' => ['string', 'int']
     * ]
     *
     * representing the formula
     *
     * !$a || $b || $c !== null || is_string($d) || is_int($d)
     *
     * @var array<string, non-empty-array<string, Assertion>>
     */
    public array $possibilities;

    /**
     * An array of things that are not true
     * [
     *     '$a' => ['!falsy'],
     *     '$b' => ['falsy'],
     *     '$c' => ['null'],
     *     '$d' => ['!string', '!int']
     * ]
     * represents the formula
     *
     * $a && !$b && $c === null && !is_string($d) && !is_int($d)
     *
     * @var array<string, non-empty-list<Assertion>>|null
     */
    public ?array $impossibilities = null;

    public bool $wedge;

    public bool $reconcilable;

    /**
     * The clause's identity (pzoom's `Clause::hash`): an int over interned ids of its variables and assertion
     * keys. Wedges and unreconcilable clauses are identified by the object that created them, as negative
     * numbers, so they never meet an ordinary clause's (non-negative) hash.
     */
    public int $hash;

    /**
     * pzoom's `keys_bloom`: one bit per variable and per assertion key, so `contains()` rejects a clause that
     * is not a subset without looking anything up.
     */
    public int $keys_bloom = 0;

    /** @var array<string, int> interned variable and assertion keys (pzoom interns names as StrIds) */
    private static array $key_ids = [];

    /**
     * @param array<string, non-empty-array<string, Assertion>>  $possibilities
     * @param array<string, bool> $redefined_vars
     */
    public function __construct(
        array $possibilities,
        public int $creating_conditional_id,
        int $creating_object_id,
        bool $wedge = false,
        bool $reconcilable = true,
        public bool $generated = false,
        public array $redefined_vars = [],
    ) {
        // pzoom keeps possibilities in a BTreeMap: sorted by variable
        ksort($possibilities);

        // One pass over interned ids computes both the hash and the bloom (pzoom's compute_hash /
        // compute_keys_bloom). Within a variable the assertions are combined by a sum, so their order does not
        // matter (Psalm's identity never depended on it); across variables the sorted order is hashed. PHP is
        // 64-bit (CliUtils::checkRuntimeRequirements), but an overflowing product becomes a float, so the hash is
        // built in a 32-bit and a 31-bit lane (63 bits: negative numbers stay free for wedges).
        $h1 = 0;
        $h2 = 0;
        $bloom = 0;
        foreach ($possibilities as $var => $assertions) {
            $var_id = self::keyId((string) $var);
            $bloom |= 1 << ($var_id & 63);
            $set1 = 0;
            $set2 = 0;
            foreach ($assertions as $key => $_) {
                $id = self::keyId((string) $key);
                $bloom |= 1 << ($id & 63);
                $set1 = ($set1 + (($id * 0x2545F491) & 0xFFFFFFFF)) & 0xFFFFFFFF;
                $set2 = ($set2 + (($id * 0x1B873593) & 0x7FFFFFFF)) & 0x7FFFFFFF;
            }
            $h1 = ($h1 * 1_000_003 + ((($var_id * 0x2545F491) & 0xFFFFFFFF) ^ $set1)) % 4_294_967_291;
            $h2 = ($h2 * 998_244_353 + ((($var_id * 0x1B873593) & 0x7FFFFFFF) ^ $set2)) % 2_147_483_629;
        }
        $this->keys_bloom = $bloom;
        $this->hash = $wedge || !$reconcilable
            ? -1 - (2 * $creating_object_id + ($wedge ? 1 : 0))
            : ($h1 << 31) | $h2;  // h1 < 2^32, h2 < 2^31: 63 bits, never negative

        $this->possibilities = $possibilities;
        $this->wedge = $wedge;
        $this->reconcilable = $reconcilable;
        $this->creating_object_id = $creating_object_id;
    }

    /**
     * @psalm-external-mutation-free
     * @psalm-suppress ImpureStaticProperty the table only grows; an id never changes meaning
     */
    private static function keyId(string $key): int
    {
        return self::$key_ids[$key] ??= count(self::$key_ids);
    }

    /**
     * @psalm-mutation-free
     */
    public function contains(Clause $other_clause): bool
    {
        if (count($other_clause->possibilities) > count($this->possibilities)) {
            return false;
        }

        // a subset's variables and assertion keys all appear here, so its bloom bits must too
        if (($other_clause->keys_bloom & ~$this->keys_bloom) !== 0) {
            return false;
        }

        foreach ($other_clause->possibilities as $var => $_) {
            if (!isset($this->possibilities[$var])) {
                return false;
            }
        }

        foreach ($other_clause->possibilities as $var => $possible_types) {
            // keyed by each assertion's string form (pzoom compares the assertion hashes)
            if (array_diff_key($possible_types, $this->possibilities[$var]) !== []) {
                return false;
            }
        }

        return true;
    }

    /**
     * @psalm-mutation-free
     */
    #[Override]
    public function __toString(): string
    {
        $clause_strings = [];

        foreach ($this->possibilities as $var_id => $values) {
            if ($var_id[0] === '*') {
                $var_id = '<expr>';
            }

            $var_id_clauses = [];
            foreach ($values as $value) {
                $value = (string) $value;
                if ($value === 'falsy') {
                    $var_id_clauses[] = '!'.$var_id;
                    continue;
                }

                if ($value === '!falsy') {
                    $var_id_clauses[] = $var_id;
                    continue;
                }

                $negate = false;

                if ($value[0] === '!') {
                    $negate = true;
                    $value  = substr($value, 1);
                }

                if ($value[0] === '=') {
                    $value = substr($value, 1);
                }

                if ($negate) {
                    $var_id_clauses[] = $var_id.' is not '.$value;
                    continue;
                }

                $var_id_clauses[] = $var_id.' is '.$value;
            }

            if (count($var_id_clauses) > 1) {
                $clause_strings[] = '('.implode(') || (', $var_id_clauses).')';
            } else {
                assert(!empty($var_id_clauses));
                $clause_strings[] = reset($var_id_clauses);
            }
        }

        if (count($clause_strings) > 1) {
            return '(' . implode(') || (', $clause_strings) . ')';
        }

        assert(!empty($clause_strings));

        return reset($clause_strings);
    }

    public function removePossibilities(string $var_id): ?self
    {
        $possibilities = $this->possibilities;
        unset($possibilities[$var_id]);

        if (!$possibilities) {
            return null;
        }

        return new self(
            $possibilities,
            $this->creating_conditional_id,
            $this->creating_object_id,
            $this->wedge,
            $this->reconcilable,
            $this->generated,
            $this->redefined_vars,
        );
    }

    /**
     * @param non-empty-array<string, Assertion> $clause_var_possibilities
     */
    public function addPossibilities(string $var_id, array $clause_var_possibilities): self
    {
        $possibilities = $this->possibilities;
        $possibilities[$var_id] = $clause_var_possibilities;

        return new self(
            $possibilities,
            $this->creating_conditional_id,
            $this->creating_object_id,
            $this->wedge,
            $this->reconcilable,
            $this->generated,
            $this->redefined_vars,
        );
    }

    public function calculateNegation(): self
    {
        if ($this->impossibilities !== null) {
            return $this;
        }

        $impossibilities = [];

        foreach ($this->possibilities as $var_id => $possibility) {
            $impossibility = [];

            foreach ($possibility as $type) {
                if (!$type->hasEquality()
                    || (($inner_type = $type->getAtomicType())
                        && ($inner_type instanceof TLiteralInt
                            || $inner_type instanceof TLiteralFloat
                            || $inner_type instanceof TLiteralString
                            || $inner_type instanceof TClassConstant
                            || $inner_type instanceof TEnumCase))
                ) {
                    $impossibility[] = $type->getNegation();
                }
            }

            if ($impossibility) {
                $impossibilities[$var_id] = $impossibility;
            }
        }

        $clause = clone $this;

        $clause->impossibilities = $impossibilities;

        return $clause;
    }
}
