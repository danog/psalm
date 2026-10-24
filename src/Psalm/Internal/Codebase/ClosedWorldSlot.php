<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use Psalm\Type;
use Psalm\Type\Union;

/**
 * The contributions to one slot during one pass: the union of the non-`mixed` ones, whether a `mixed` one was
 * seen, and whether every contribution came from direct recursion.
 *
 * @internal
 */
final class ClosedWorldSlot
{
    public ?Union $type = null;
    public ?Union $non_recursive = null;
    public bool $mixed = false;
    public bool $mixed_non_recursive = false;
    public bool $void_paths = false;

    public function add(Union $type, bool $recursive): void
    {
        if (ClosedWorld::hasMixed($type)) {
            $this->mixed = true;
            if (!$recursive) {
                $this->mixed_non_recursive = true;
            }
            return;
        }
        $type = ClosedWorld::generalize($type);
        $this->type = $this->type === null ? $type : Type::combineUnionTypes($this->type, $type);
        if (!$recursive) {
            $this->non_recursive = $this->non_recursive === null ? $type : Type::combineUnionTypes($this->non_recursive, $type);
        }
    }

    public function merge(ClosedWorldSlot $other): void
    {
        if ($other->type !== null) {
            $this->type = $this->type === null ? $other->type : Type::combineUnionTypes($this->type, $other->type);
        }
        if ($other->non_recursive !== null) {
            $this->non_recursive = $this->non_recursive === null
                ? $other->non_recursive
                : Type::combineUnionTypes($this->non_recursive, $other->non_recursive);
        }
        $this->mixed = $this->mixed || $other->mixed;
        $this->mixed_non_recursive = $this->mixed_non_recursive || $other->mixed_non_recursive;
        $this->void_paths = $this->void_paths || $other->void_paths;
    }

    public function isEmpty(): bool
    {
        return $this->type === null && !$this->mixed;
    }

    public function isOnlyNull(): bool
    {
        return !$this->mixed && $this->type !== null && $this->type->isNull();
    }

    /**
     * The slot value: on the first (optimistic) guess the non-`mixed`, non-recursive contributions; afterwards
     * all of them. Null when nothing was contributed (the declared type stays).
     */
    /**
     * The slot value. `first`: the optimistic first guess (non-`mixed`, non-recursive contributions);
     * `optimistic`: the non-`mixed` contributions (a `mixed` one is assumed to stem from a slot not inferred
     * yet); `sound`: all of them. Null when nothing (but `mixed`, optimistically) was contributed: the declared
     * type stays.
     */
    public function value(string $mode, ?Union $declared): ?Union
    {
        if ($mode === 'first' && $this->non_recursive !== null) {
            return $this->non_recursive;
        }
        if ($mode !== 'sound') {
            return $this->type;
        }
        if ($this->mixed) {
            return $declared ?? Type::getMixed();
        }
        return $this->type;
    }
}
