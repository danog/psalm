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
    /** union of the contributions that are not `mixed` themselves (sound phase) */
    public ?Union $type = null;
    /** the same without any nested `mixed` (optimistic phase: unknown items are assumed not to exist) */
    public ?Union $optimistic = null;
    /** the optimistic union of the contributions not from direct recursion (first guess) */
    public ?Union $non_recursive = null;
    public bool $mixed = false;
    public bool $void_paths = false;

    public function add(Union $type, bool $recursive): void
    {
        if (ClosedWorld::hasMixed($type)) {
            $this->mixed = true;
            return;
        }
        $type = ClosedWorld::generalize($type);
        $this->type = self::join($this->type, $type);
        $stripped = ClosedWorld::stripMixed($type);
        $this->optimistic = self::join($this->optimistic, $stripped);
        if (!$recursive) {
            $this->non_recursive = self::join($this->non_recursive, $stripped);
        }
    }

    private static function join(?Union $a, ?Union $b): ?Union
    {
        if ($a === null) {
            return $b;
        }
        if ($b === null) {
            return $a;
        }
        return Type::combineUnionTypes($a, $b);
    }

    public function merge(ClosedWorldSlot $other): void
    {
        $this->type = self::join($this->type, $other->type);
        $this->optimistic = self::join($this->optimistic, $other->optimistic);
        $this->non_recursive = self::join($this->non_recursive, $other->non_recursive);
        $this->mixed = $this->mixed || $other->mixed;
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
     * The slot value. `first`: the optimistic first guess (non-recursive contributions); `optimistic`: the
     * contributions without any `mixed` (a `mixed` is assumed to stem from a slot not inferred yet); `sound`: all
     * of them. Null when nothing (but `mixed`, optimistically) was contributed: the declared type stays.
     */
    public function value(string $mode, ?Union $declared): ?Union
    {
        if ($mode === 'first' && $this->non_recursive !== null) {
            return $this->non_recursive;
        }
        if ($mode !== 'sound') {
            return $this->optimistic;
        }
        if ($this->mixed) {
            return $declared ?? Type::getMixed();
        }
        return $this->type;
    }
}
