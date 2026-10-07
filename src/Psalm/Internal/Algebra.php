<?php

declare(strict_types=1);

namespace Psalm\Internal;

use Psalm\Exception\ComplicatedExpressionException;
use Psalm\Storage\Assertion;
use Psalm\Type\Atomic\TArray;
use Psalm\Type\Atomic\TKeyedArray;
use UnexpectedValueException;

use function array_diff_key;
use function array_filter;
use function array_intersect_key;
use function array_key_first;
use function array_keys;
use function array_pop;
use function array_values;
use function assert;
use function count;
use function in_array;
use function mt_rand;
use function reset;

/**
 * @internal
 */
final class Algebra
{
    /**
     * @param array<string, non-empty-list<non-empty-list<Assertion>>>  $all_types
     * @return array<string, non-empty-list<non-empty-list<Assertion>>>
     * @psalm-pure
     */
    public static function negateTypes(array $all_types): array
    {
        $negated_types = [];

        foreach ($all_types as $key => $anded_types) {
            if (count($anded_types) > 1) {
                $new_anded_types = [];

                foreach ($anded_types as $orred_types) {
                    if (count($orred_types) === 1) {
                        $new_anded_types[] = $orred_types[0]->getNegation();
                    } else {
                        continue 2;
                    }
                }

                assert($new_anded_types !== []);

                $negated_types[$key] = [$new_anded_types];
                continue;
            }

            $new_orred_types = [];

            foreach ($anded_types[0] as $orred_type) {
                $new_orred_types[] = [$orred_type->getNegation()];
            }

            $negated_types[$key] = $new_orred_types;
        }

        return $negated_types;
    }

    /**
     * Simplifies a CNF formula the way pzoom's `simplify_cnf` does:
     *
     *  - duplicates (by hash) are dropped;
     *  - unit propagation runs to a fixpoint (at most ten rounds): a unit clause `A` removes `!A` from
     *    every other clause, and a clause differing from another in exactly one negated key loses that
     *    key -- and every clause a round rewrites takes part in the next round, so `$a && (!$a || $b)
     *    && (!$b || $c)` concludes `$c` (Psalm's single pass stopped at `$b`);
     *  - a clause containing another is dropped;
     *  - `(A || X) && (!A || Y) && (X || Y)` loses `(X || Y)`.
     *
     * @param list<Clause>  $clauses
     * @return list<Clause>
     * @psalm-capabilities write-refs
     */
    public static function simplifyCNF(array $clauses): array
    {
        $clause_count = count($clauses);

        //65536 seems to be a significant threshold, when put at 65537, the code https://psalm.dev/r/216f362ea6 goes
        //from seconds in analysis to many minutes
        if ($clause_count > 65_536) {
            return [];
        }

        if ($clause_count > 50) {
            $all_has_unknown = true;

            foreach ($clauses as $clause) {
                $clause_has_unknown = false;
                foreach ($clause->possibilities as $key => $_) {
                    if ($key[0] === '*') {
                        $clause_has_unknown = true;
                        break;
                    }
                }

                if (!$clause_has_unknown) {
                    $all_has_unknown = false;
                    break;
                }
            }

            if ($all_has_unknown) {
                return $clauses;
            }
        }

        // avoid strict duplicates
        $working = [];
        foreach ($clauses as $clause) {
            $working[$clause->hash] = $clause;
        }

        [$removed, $added] = self::unitPropagationRound($working);

        if ($removed !== [] || $added !== []) {
            foreach ($removed as $hash => $_) {
                unset($working[$hash]);
            }
            foreach ($added as $added_clause) {
                $working[$added_clause->hash] = $added_clause;
            }

            // iterate to a fixpoint (bounded)
            for ($round = 0; $round < 9; $round++) {
                [$removed, $added] = self::unitPropagationRound($working);
                if ($removed === [] && $added === []) {
                    break;
                }
                foreach ($removed as $hash => $_) {
                    unset($working[$hash]);
                }
                foreach ($added as $added_clause) {
                    $working[$added_clause->hash] = $added_clause;
                }
            }
        }

        // remove redundant clauses (those containing another)
        $simplified_clauses = [];

        foreach ($working as $clause_a) {
            $is_redundant = false;

            foreach ($working as $clause_b) {
                if ($clause_a === $clause_b
                    || !$clause_b->reconcilable
                    || $clause_b->wedge
                    || $clause_a->wedge
                ) {
                    continue;
                }

                if ($clause_a->contains($clause_b)) {
                    $is_redundant = true;
                    break;
                }
            }

            if (!$is_redundant) {
                $simplified_clauses[$clause_a->hash] = $clause_a;
            }
        }

        $clause_count = count($simplified_clauses);

        // simplify (A || X) && (!A || Y) && (X || Y)
        // to
        // simplify (A || X) && (!A || Y)
        // where X and Y are sets of orred terms
        if ($clause_count > 2 && $clause_count < 256) {
            $clauses = array_values($simplified_clauses);
            for ($i = 0; $i < $clause_count; $i++) {
                $clause_a = $clauses[$i];
                for ($k = $i + 1; $k < $clause_count; $k++) {
                    $clause_b = $clauses[$k];
                    $common_keys = array_keys(
                        array_intersect_key($clause_a->possibilities, $clause_b->possibilities),
                    );
                    if ($common_keys) {
                        $common_negated_keys = [];
                        foreach ($common_keys as $common_key) {
                            if (count($clause_a->possibilities[$common_key]) === 1
                                && count($clause_b->possibilities[$common_key]) === 1
                                && reset($clause_a->possibilities[$common_key])->isNegationOf(
                                    reset($clause_b->possibilities[$common_key]),
                                )
                            ) {
                                $common_negated_keys[] = $common_key;
                            }
                        }

                        if ($common_negated_keys) {
                            $new_possibilities = [];

                            foreach ($clause_a->possibilities as $var_id => $possibilities) {
                                if (in_array($var_id, $common_negated_keys, true)) {
                                    continue;
                                }

                                if (!isset($new_possibilities[$var_id])) {
                                    $new_possibilities[$var_id] = $possibilities;
                                } else {
                                    $new_possibilities[$var_id] += $possibilities;
                                }
                            }

                            foreach ($clause_b->possibilities as $var_id => $possibilities) {
                                if (in_array($var_id, $common_negated_keys, true)) {
                                    continue;
                                }

                                if (!isset($new_possibilities[$var_id])) {
                                    $new_possibilities[$var_id] = $possibilities;
                                } else {
                                    $new_possibilities[$var_id] += $possibilities;
                                }
                            }

                            $conflict_clause = (new Clause(
                                $new_possibilities,
                                $clause_a->creating_conditional_id,
                                $clause_a->creating_conditional_id,
                                false,
                                true,
                                true,
                                [],
                            ));

                            unset($simplified_clauses[$conflict_clause->hash]);
                        }
                    }
                }
            }
        }

        return array_values($simplified_clauses);
    }

    /**
     * One unit-propagation round over a set of distinct clauses (pzoom's `unit_propagation_round`): the
     * hashes of the clauses to drop and the rewritten clauses to add. The caller applies both and runs
     * another round until nothing changes.
     *
     * @param array<int, Clause> $clauses by hash
     * @return array{array<int, true>, list<Clause>}
     * @psalm-pure
     */
    private static function unitPropagationRound(array $clauses): array
    {
        $removed = [];
        $added = [];

        foreach ($clauses as $clause_a) {
            if (!$clause_a->reconcilable || $clause_a->wedge) {
                continue;
            }

            $is_clause_a_simple = count($clause_a->possibilities) === 1
                && count($clause_a->possibilities[array_key_first($clause_a->possibilities)]) === 1;

            if (!$is_clause_a_simple) {
                $clause_a_keys = array_keys($clause_a->possibilities);
                $clause_a_count = count($clause_a_keys);

                foreach ($clauses as $clause_b) {
                    if ($clause_a === $clause_b
                        || !$clause_b->reconcilable
                        || $clause_b->wedge
                        || count($clause_b->possibilities) !== $clause_a_count
                    ) {
                        continue;
                    }

                    // the same variables (both maps are sorted by variable)
                    if ($clause_a_keys !== array_keys($clause_b->possibilities)) {
                        continue;
                    }

                    $opposing_keys = [];

                    foreach ($clause_a->possibilities as $key => $a_possibilities) {
                        $b_possibilities = $clause_b->possibilities[$key];

                        // the same assertions
                        if (count($a_possibilities) === count($b_possibilities)
                            && array_diff_key($a_possibilities, $b_possibilities) === []
                        ) {
                            continue;
                        }

                        if (count($a_possibilities) === 1
                            && count($b_possibilities) === 1
                            && reset($a_possibilities)->isNegationOf(reset($b_possibilities))
                        ) {
                            $opposing_keys[] = $key;
                            continue;
                        }

                        continue 2;
                    }

                    if (count($opposing_keys) === 1) {
                        $removed[$clause_a->hash] = true;

                        $rewritten = $clause_a->removePossibilities($opposing_keys[0]);

                        if ($rewritten === null) {
                            continue 2;
                        }

                        $added[] = $rewritten;
                    }
                }

                continue;
            }

            // a unit clause: its negation leaves every other clause
            foreach ($clause_a->possibilities as $clause_var => $var_possibilities) {
                $only_type = $var_possibilities[array_key_first($var_possibilities)];
                $negated_hash = $only_type->getNegation()->getHash();

                foreach ($clauses as $clause_b) {
                    if ($clause_a === $clause_b || !$clause_b->reconcilable || $clause_b->wedge) {
                        continue;
                    }

                    $matching_possibilities = $clause_b->possibilities[$clause_var] ?? null;

                    if ($matching_possibilities !== null && isset($matching_possibilities[$negated_hash])) {
                        unset($matching_possibilities[$negated_hash]);

                        $removed[$clause_b->hash] = true;

                        if ($matching_possibilities === []) {
                            $updated_clause = $clause_b->removePossibilities($clause_var);

                            if ($updated_clause !== null) {
                                $added[] = $updated_clause;
                            }
                        } else {
                            $added[] = $clause_b->addPossibilities($clause_var, $matching_possibilities);
                        }
                    }
                }
            }
        }

        return [$removed, $added];
    }

    /**
     * Look for clauses with only one possible value
     *
     * doesn't infer the "unset" correctly
     *
     * @param  list<Clause>  $clauses
     * @param  array<string, bool> $cond_referenced_var_ids
     * @param  array<string, array<int, list<Assertion>>> $active_truths
     * @return array<string, list<list<Assertion>>>
     * @psalm-capabilities read-props|write-this-props|write-refs
     */
    public static function getTruthsFromFormula(
        array $clauses,
        ?int $creating_conditional_id = null,
        array &$cond_referenced_var_ids = [],
        array &$active_truths = [],
    ): array {
        $truths = [];
        $active_truths = [];

        if ($clauses === []) {
            return [];
        }

        foreach ($clauses as $clause) {
            if (!$clause->reconcilable || count($clause->possibilities) !== 1) {
                continue;
            }

            foreach ($clause->possibilities as $var => $possible_types) {
                if ($var[0] === '*') {
                    continue;
                }

                // if there's only one possible type, return it
                if (count($possible_types) === 1) {
                    $possible_type = array_pop($possible_types);

                    if (isset($truths[$var]) && !isset($clause->redefined_vars[$var])) {
                        $truths[$var][] = [$possible_type];
                    } else {
                        $truths[$var] = [[$possible_type]];
                    }

                    if ($creating_conditional_id && $creating_conditional_id === $clause->creating_conditional_id) {
                        if (!isset($active_truths[$var])) {
                            $active_truths[$var] = [];
                        }

                        $active_truths[$var][count($truths[$var]) - 1] = [$possible_type];
                    }
                } else {
                    // if there's only one active clause, return all the non-negation clause members ORed together
                    // already distinct: a clause keys its assertions by hash
                    $things_that_can_be_said = $possible_types;

                    if ($clause->generated && count($possible_types) > 1) {
                        unset($cond_referenced_var_ids[$var]);
                    }

                    $truths[$var] = [array_values($things_that_can_be_said)];

                    if ($creating_conditional_id && $creating_conditional_id === $clause->creating_conditional_id) {
                        $active_truths[$var] = [array_values($things_that_can_be_said)];
                    }
                }
            }
        }

        foreach ($truths as $var => $anded_types) {
            $has_list_or_array = false;
            foreach ($anded_types as $orred_types) {
                foreach ($orred_types as $assertion) {
                    if ($assertion->isNegation()) {
                        continue;
                    }

                    $assertion_type = $assertion->getAtomicType();

                    if ($assertion_type instanceof TArray
                        || $assertion_type instanceof TKeyedArray) {
                        $has_list_or_array = true;
                        // list/array are collapsed, therefore there can only be 1 and we can abort
                        // otherwise we would have to remove them all individually
                        // e.g. array<string, string> cannot be array<int, float>
                        break 2;
                    }
                }
            }

            if ($has_list_or_array === false) {
                continue;
            }

            foreach ($anded_types as $key => $orred_types) {
                foreach ($orred_types as $index => $assertion) {
                    // we only need to check negations
                    // due to type collapsing, any negations for arrays are irrelevant
                    if (!$assertion->isNegation()) {
                        continue;
                    }

                    $assertion_type = $assertion->getAtomicType();

                    if ($assertion_type instanceof TArray
                        || $assertion_type instanceof TKeyedArray) {
                        unset($truths[$var][$key][$index]);
                    }
                }

                /**
                 * doesn't infer the "unset" correctly
                 *
                 * @psalm-suppress DocblockTypeContradiction
                 */
                if ($truths[$var][$key] === []) {
                    unset($truths[$var][$key]);
                } else {
                    /**
                     * doesn't infer the "unset" correctly
                     *
                     * @psalm-suppress RedundantFunctionCallGivenDocblockType
                     */
                    $truths[$var][$key] = array_values($truths[$var][$key]);
                }
            }
        }

        /** @psalm-suppress LessSpecificReturnStatement */
        return $truths;
    }

    /**
     * @param non-empty-list<Clause>  $clauses
     * @return list<Clause>
     * @psalm-pure
     */
    public static function groupImpossibilities(array $clauses): array
    {
        $complexity = 1;

        $seed_clauses = [];

        $clause = array_pop($clauses);

        if (!$clause->wedge) {
            if ($clause->impossibilities === null) {
                throw new UnexpectedValueException('$clause->impossibilities should not be null');
            }

            foreach ($clause->impossibilities as $var => $impossible_types) {
                foreach ($impossible_types as $impossible_type) {
                    $seed_clause = new Clause(
                        [$var => [$impossible_type->getHash() => $impossible_type]],
                        $clause->creating_conditional_id,
                        $clause->creating_object_id,
                    );

                    $seed_clauses[] = $seed_clause;

                    ++$complexity;
                }
            }
        }

        if (!$clauses || !$seed_clauses) {
            return $seed_clauses;
        }

        $complexity_upper_bound = count($seed_clauses);

        foreach ($clauses as $clause) {
            $i = 0;
            foreach ($clause->possibilities as $p) {
                $i += count($p);
            }

            $complexity_upper_bound *= $i;

            if ($complexity_upper_bound > 20_000) {
                throw new ComplicatedExpressionException();
            }
        }

        while ($clauses) {
            $clause = array_pop($clauses);

            $new_clauses = [];

            foreach ($seed_clauses as $grouped_clause) {
                if ($clause->impossibilities === null) {
                    throw new UnexpectedValueException('$clause->impossibilities should not be null');
                }

                foreach ($clause->impossibilities as $var => $impossible_types) {
                    foreach ($impossible_types as $impossible_type) {
                        $new_clause_possibilities = $grouped_clause->possibilities;

                        if (isset($new_clause_possibilities[$var])) {
                            $impossible_type_hash = $impossible_type->getHash();
                            $new_clause_possibilities[$var][$impossible_type_hash] = $impossible_type;

                            foreach ($new_clause_possibilities[$var] as $ak => $av) {
                                foreach ($new_clause_possibilities[$var] as $bk => $bv) {
                                    if ($ak == $bk) {
                                        break;
                                    }

                                    if ($ak !== $impossible_type_hash && $bk !== $impossible_type_hash) {
                                        continue;
                                    }

                                    if ($av->isNegationOf($bv)) {
                                        break 3;
                                    }
                                }
                            }
                        } else {
                            $new_clause_possibilities[$var] = [$impossible_type->getHash() => $impossible_type];
                        }

                        $new_clause = new Clause(
                            $new_clause_possibilities,
                            $grouped_clause->creating_conditional_id,
                            $clause->creating_object_id,
                            false,
                            true,
                            true,
                            [],
                        );

                        $new_clauses[] = $new_clause;

                        ++$complexity;

                        if ($complexity > 20_000) {
                            throw new ComplicatedExpressionException();
                        }
                    }
                }
            }

            $seed_clauses = $new_clauses;
        }

        return $seed_clauses;
    }

    /**
     * @param list<Clause>  $left_clauses
     * @param list<Clause>  $right_clauses
     * @return list<Clause>
     * @psalm-pure
     */
    public static function combineOredClauses(
        array $left_clauses,
        array $right_clauses,
        int $conditional_object_id,
    ): array {
        if (count($left_clauses) > 60_000 || count($right_clauses) > 60_000) {
            return [];
        }

        $clauses = [];

        $all_wedges = true;
        $has_wedge = false;

        foreach ($left_clauses as $left_clause) {
            foreach ($right_clauses as $right_clause) {
                $all_wedges = $all_wedges && ($left_clause->wedge && $right_clause->wedge);
                $has_wedge = $has_wedge || ($left_clause->wedge && $right_clause->wedge);
            }
        }

        if ($all_wedges) {
            return [new Clause([], $conditional_object_id, $conditional_object_id, true)];
        }

        foreach ($left_clauses as $left_clause) {
            foreach ($right_clauses as $right_clause) {
                if ($left_clause->wedge && $right_clause->wedge) {
                    // handled below
                    continue;
                }

                /** @var  array<string, non-empty-array<int, Assertion>> */
                $possibilities = [];

                $can_reconcile = true;

                if ($left_clause->wedge ||
                    $right_clause->wedge ||
                    !$left_clause->reconcilable ||
                    !$right_clause->reconcilable
                ) {
                    $can_reconcile = false;
                }

                foreach ($left_clause->possibilities as $var => $possible_types) {
                    if (isset($right_clause->redefined_vars[$var])) {
                        continue;
                    }

                    if (isset($possibilities[$var])) {
                        $possibilities[$var] += $possible_types;
                    } else {
                        $possibilities[$var] = $possible_types;
                    }
                }

                foreach ($right_clause->possibilities as $var => $possible_types) {
                    if (isset($possibilities[$var])) {
                        $possibilities[$var] += $possible_types;
                    } else {
                        $possibilities[$var] = $possible_types;
                    }
                }

                foreach ($possibilities as $var_possibilities) {
                    if (count($var_possibilities) === 2) {
                        $vals = array_values($var_possibilities);
                        /** @psalm-suppress PossiblyUndefinedIntArrayOffset */
                        if ($vals[0]->isNegationOf($vals[1])) {
                            continue 2;
                        }
                    }
                }

                $creating_conditional_id =
                    $right_clause->creating_conditional_id === $left_clause->creating_conditional_id
                    ? $right_clause->creating_conditional_id
                    : $conditional_object_id;

                $clauses[] = new Clause(
                    $possibilities,
                    $creating_conditional_id,
                    $creating_conditional_id,
                    false,
                    $can_reconcile,
                    $right_clause->generated
                        || $left_clause->generated
                        || count($left_clauses) > 1
                        || count($right_clauses) > 1,
                    [],
                );
            }
        }

        if ($has_wedge) {
            $clauses[] = new Clause([], $conditional_object_id, $conditional_object_id, true);
        }

        return $clauses;
    }

    /**
     * Negates a set of clauses
     * negateClauses([$a || $b]) => !$a && !$b
     * negateClauses([$a, $b]) => !$a || !$b
     * negateClauses([$a, $b || $c]) =>
     *   (!$a || !$b) &&
     *   (!$a || !$c)
     * negateClauses([$a, $b || $c, $d || $e || $f]) =>
     *   (!$a || !$b || !$d) &&
     *   (!$a || !$b || !$e) &&
     *   (!$a || !$b || !$f) &&
     *   (!$a || !$c || !$d) &&
     *   (!$a || !$c || !$e) &&
     *   (!$a || !$c || !$f)
     *
     * @param list<Clause>  $clauses
     * @return non-empty-list<Clause>
     */
    public static function negateFormula(array $clauses): array
    {
        $clauses = array_filter(
            $clauses,
            static fn(Clause $clause): bool => $clause->reconcilable,
        );

        if (!$clauses) {
            $cond_id = mt_rand(0, 100_000_000);
            return [new Clause([], $cond_id, $cond_id, true)];
        }

        $clauses_with_impossibilities = [];

        foreach ($clauses as $clause) {
            $clauses_with_impossibilities[] = $clause->calculateNegation();
        }

        unset($clauses);

        $impossible_clauses = self::groupImpossibilities($clauses_with_impossibilities);

        if (!$impossible_clauses) {
            $cond_id = mt_rand(0, 100_000_000);
            return [new Clause([], $cond_id, $cond_id, true)];
        }

        $negated = self::simplifyCNF($impossible_clauses);

        if (!$negated) {
            $cond_id = mt_rand(0, 100_000_000);
            return [new Clause([], $cond_id, $cond_id, true)];
        }

        return $negated;
    }
}
