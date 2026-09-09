<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use Closure;

/**
 * An assignable location (variable, property, array element, static property).
 *
 * @internal
 */
final class Place
{
    /**
     * @param Closure(): string $reader  owned-value read code
     * @param Closure(string): string $writer  statement code storing a value of `$type`
     * @param (Closure(): string)|null $mut  expression usable as a `&mut T` place, if available
     * @param (Closure(string): string)|null $wrapper  wraps a statement that uses the mutable place
     */
    public function __construct(
        public readonly RustType $type,
        private readonly Closure $reader,
        private readonly Closure $writer,
        private readonly ?Closure $mut = null,
        private readonly ?Closure $wrapper = null,
    ) {
    }

    public function read(): string
    {
        return ($this->reader)();
    }

    public function write(string $value_code): string
    {
        return $this->wrap(($this->writer)($value_code));
    }

    public function hasMut(): bool
    {
        return $this->mut !== null;
    }

    public function mut(): string
    {
        return ($this->mut)();
    }

    /**
     * Statement applying `$f` (which receives a `&mut T` place expression) to this location,
     * falling back to read-modify-write when no mutable place is available.
     *
     * @param Closure(string): string $f
     */
    private static int $counter = 0;

    public function modify(Closure $f): string
    {
        if ($this->mut !== null) {
            return $this->wrap($f(($this->mut)()));
        }
        $p = '__p' . (++self::$counter);
        return '{ let mut ' . $p . ' = ' . $this->read() . '; ' . $f($p) . ' ' . $this->write($p) . ' }';
    }

    /**
     * Expression applying `$f` (which receives a `&mut T` place expression and returns an expression)
     * to this location and yielding that expression's value.
     *
     * @param Closure(string): string $f
     */
    public function modifyValue(Closure $f): string
    {
        if ($this->mut !== null) {
            return $this->wrap($f(($this->mut)()));
        }
        $p = '__p' . (++self::$counter);
        return '{ let mut ' . $p . ' = ' . $this->read() . '; let __r = ' . $f($p) . '; ' . $this->write($p) . ' __r }';
    }

    /** Wrap a statement that uses `mut()` (needed for thread-local statics). */
    public function wrap(string $stmt): string
    {
        return $this->wrapper !== null ? ($this->wrapper)($stmt) : $stmt;
    }
}
