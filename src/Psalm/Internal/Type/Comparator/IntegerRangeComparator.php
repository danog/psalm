<?php

declare(strict_types=1);

namespace Psalm\Internal\Type\Comparator;

use Psalm\Type\Atomic;
use Psalm\Type\Atomic\TInt;
use Psalm\Type\Atomic\TIntRange;
use Psalm\Type\Atomic\TLiteralInt;
use Psalm\Type\Atomic\TNonspecificLiteralInt;
use Psalm\Type\Union;
use UnexpectedValueException;

use function count;

/**
 * @internal
 */
final class IntegerRangeComparator
{
    /**
     * This method is used to check if an integer range can be contained in another
     *
     * @psalm-pure
     */
    public static function isContainedBy(
        TIntRange $input_type_part,
        TIntRange $container_type_part,
    ): bool {
        $is_input_min = $input_type_part->min_bound === null;
        $is_input_max = $input_type_part->max_bound === null;
        $is_container_min = $container_type_part->min_bound === null;
        $is_container_max = $container_type_part->max_bound === null;

        $is_input_min_in_container = (
                $is_container_min ||
                (!$is_input_min && $container_type_part->min_bound <= $input_type_part->min_bound)
            );
        $is_input_max_in_container = (
                $is_container_max ||
                (!$is_input_max && $container_type_part->max_bound >= $input_type_part->max_bound)
            );
        return $is_input_min_in_container && $is_input_max_in_container;
    }

    /**
     * This method is used to check if an integer range can be contained by multiple int types
     * Worst case scenario, the input is `int<-50,max>` and container is `-50|int<-49,50>|positive-int|57`
     */
    public static function isContainedByUnion(
        TIntRange $input_type_part,
        Union $container_type,
    ): bool {
        $container_atomic_types = $container_type->getAtomicTypesByKey();
        $reduced_range = new TIntRange(
            $input_type_part->min_bound,
            $input_type_part->max_bound,
            $input_type_part->from_docblock,
        );

        if (isset($container_atomic_types['int'])) {
            if ($container_atomic_types['int']::class === TInt::class) {
                return true;
            }

            if ($container_atomic_types['int']::class === TNonspecificLiteralInt::class) {
                return true;
            }

            throw new UnexpectedValueException('Should not happen: unknown int key');
        }

        $new_nb_atomics = count($container_atomic_types);
        //loop until we get to a stable situation. Either we can't remove atomics or we have a definite result
        do {
            $nb_atomics = $new_nb_atomics;
            $result_reduction = self::reduceRangeIncrementally($container_atomic_types, $reduced_range);
            $new_nb_atomics = count($container_atomic_types);
        } while ($result_reduction === null && $nb_atomics !== $new_nb_atomics);

        if ($result_reduction === null && $nb_atomics === 0) {
            //the range could not be reduced enough and there is no more atomics, it's not contained
            return false;
        }

        return $result_reduction ?? false;
    }

    /**
     * This method receives an array of atomics from the container and a range.
     * The goal is to use values in atomics in order to reduce the range.
     * Once the range is empty, it means that every value in range was covered by some atomics combination
     *
     * The range is reduced in place: its bounds travel as ints and it is rebuilt when a container comparison
     * needs the atomic (a TIntRange is never written after construction).
     *
     * @param array<string, Atomic> $container_atomic_types
     */
    private static function reduceRangeIncrementally(array &$container_atomic_types, TIntRange &$reduced_range): ?bool
    {
        $min_bound = $reduced_range->min_bound;
        $max_bound = $reduced_range->max_bound;

        foreach ($container_atomic_types as $key => $container_atomic_type) {
            if ($container_atomic_type instanceof TIntRange) {
                $reduced_range = new TIntRange($min_bound, $max_bound);
                if (self::isContainedBy($reduced_range, $container_atomic_type)) {
                    if ($container_atomic_type->max_bound === null && $container_atomic_type->min_bound === null) {
                        //this container range covers any integer
                        return true;
                    }
                    if ($container_atomic_type->max_bound === null) {
                        //this container range is int<X, max>
                        //X-1 becomes the max of our reduced range if it was higher
                        $max_bound = TIntRange::getNewLowestBound(
                            $container_atomic_type->min_bound - 1,
                            $max_bound ?? $container_atomic_type->min_bound - 1,
                        );
                        unset($container_atomic_types[$key]); //we don't need this one anymore
                        continue;
                    }
                    if ($container_atomic_type->min_bound === null) {
                        //this container range is int<min, X>
                        //X+1 becomes the min of our reduced range if it was lower
                        $min_bound = TIntRange::getNewHighestBound(
                            $container_atomic_type->max_bound + 1,
                            $min_bound ?? $container_atomic_type->max_bound + 1,
                        );
                        unset($container_atomic_types[$key]); //we don't need this one anymore
                        continue;
                    }
                    //if the container range has no 'null' bound, it's more complex
                    //in this case, we can only reduce if the container include one bound of our reduced range
                    if ($min_bound !== null
                        && $container_atomic_type->contains($min_bound)
                    ) {
                        //this container range is int<X, Y> and contains the min of our reduced range.
                        //the min from our reduced range becomes Y + 1
                        $min_bound = $container_atomic_type->max_bound + 1;
                        unset($container_atomic_types[$key]); //we don't need this one anymore
                    } elseif ($max_bound !== null
                        && $container_atomic_type->contains($max_bound)) {
                        //this container range is int<X, Y> and contains the max of our reduced range.
                        //the max from our reduced range becomes X - 1
                        $max_bound = $container_atomic_type->min_bound - 1;
                        unset($container_atomic_types[$key]); //we don't need this one anymore
                    }
                    //there is probably a case here where we could unset containers when they're not at all in our range
                } else {
                    //the range in input is wider than container, we return false
                    return false;
                }
            } elseif ($container_atomic_type instanceof TLiteralInt) {
                if (!self::boundsContain($min_bound, $max_bound, $container_atomic_type->value)) {
                    unset($container_atomic_types[$key]); //we don't need this one anymore
                } elseif ($min_bound === $container_atomic_type->value) {
                    $min_bound++;
                    unset($container_atomic_types[$key]); //we don't need this one anymore
                } elseif ($max_bound === $container_atomic_type->value) {
                    $max_bound--;
                    unset($container_atomic_types[$key]); //we don't need this one anymore
                }
            }
        }

        //there is probably a case here if we're left only with TLiteralInt where we could return false if there's less
        //of them than numbers in the reduced range

        //there is also a case where if there's not TLiteralInt anymore and we're left with TIntRange that don't contain
        //bounds from our reduced range where we could return false

        //if our reduced range has its min bound superior to its max bound, it means the container covers it all.
        $reduced_range = new TIntRange($min_bound, $max_bound);

        if ($min_bound !== null && $max_bound !== null && $min_bound > $max_bound) {
            return true;
        }

        //if we didn't return true or false before then the result is inconclusive for this round
        return null;
    }

    /**
     * TIntRange::contains() on bare bounds.
     *
     * @psalm-pure
     */
    private static function boundsContain(?int $min_bound, ?int $max_bound, int $i): bool
    {
        return ($min_bound === null || $min_bound <= $i) && ($max_bound === null || $max_bound >= $i);
    }
}
