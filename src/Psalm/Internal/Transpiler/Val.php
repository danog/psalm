<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

/**
 * A piece of Rust expression code together with the Rust type it evaluates to.
 *
 * `$code` is always an OWNED value of `$type` (a plain local reads as `x.clone()`). When the value also lives
 * somewhere borrowable, `$place` names that place: a Rust place expression of type `$type`, so that `&$place`
 * is a `&T` and `$place.m()` auto-refs — `x`, `(*self)`, `(*x.get())` for a Late local, `(*b.p_f())` for a
 * field of an `Rc<T>` (immutable) class. Consumers that only need to look at the value (a `&self` method
 * receiver, a borrowed builtin argument, a comparison operand, the subject of a foreach) use the place and
 * skip the clone; the borrow checker verifies every such use, so a place is never wrong, only absent.
 *
 * A field of a RefCell class is reachable only through a borrow guard (`Ref<'_, T>` / `PropRef`), which must
 * not outlive the expression that reads it (a guard held across a call that writes the same object panics).
 * Such a value carries no `$place` but a `$guard`: `let` statements binding the guard(s), and `$gplace`, the
 * place inside them. `applyOwned()` scopes the guard in a block around an owned-result operation
 * (`{ let __g = b.p_f(); (*__g).count() }`), which is the only way a guarded value is ever borrowed.
 *
 * @internal
 */
final class Val
{
    /**
     * @param ?string $place  borrowable place of type `$type`, or null when the value is a temporary
     * @param ?string $guard  `let` statements binding a borrow guard, when the value is only reachable through one
     * @param ?string $gplace the place inside `$guard` (requires `$guard`)
     * @param bool    $temp   the place runs through a temporary of the enclosing expression: usable as an
     *                        argument, a receiver or a loop subject, but not bound by a `let` (dropped at its `;`)
     */
    public function __construct(
        public readonly string $code,
        public readonly RustType $type,
        public readonly ?string $place = null,
        public readonly ?string $guard = null,
        public readonly ?string $gplace = null,
        public readonly bool $temp = false,
    ) {
    }

    /** The same type with other code: a temporary (no place). */
    public function with(string $code): Val
    {
        return new Val($code, $this->type);
    }

    /** A `&T` for a borrowing consumer that is verified by the borrow checker (never a guard). */
    public function borrow(): string
    {
        if ($this->place !== null) {
            return '&' . $this->place;
        }
        return Names::refOf($this->code);
    }

    /** The receiver of a `&self`-style method call (auto-ref): the place when there is one, else the owned value. */
    public function recv(): string
    {
        return $this->place ?? $this->code;
    }

    /**
     * `<value><tail>` where `$tail` (`.p_f_get()`, `.count()`, `.get(&k).cloned()`) yields an OWNED result: the
     * place is used directly; a guarded value scopes its guard in a block so nothing borrowed escapes.
     */
    public function applyOwned(string $tail): string
    {
        if ($this->place !== null) {
            return $this->place . $tail;
        }
        if ($this->guard !== null) {
            return '{ ' . $this->guard . ' ' . $this->gplace . $tail . ' }';
        }
        return $this->code . $tail;
    }

    /** Whether the value can be borrowed without a guard (a plain `&place`). */
    public function borrowable(): bool
    {
        return $this->place !== null;
    }
}
