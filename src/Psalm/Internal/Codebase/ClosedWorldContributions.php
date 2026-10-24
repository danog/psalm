<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

/**
 * What the analysis of one file (a root file: the file Psalm analyzed, trait bodies included) contributed to the
 * closed-world slots. Kept per file so that only re-analyzed files replace theirs.
 *
 * @internal
 */
final class ClosedWorldContributions
{
    /** @var array<int, array<int, ClosedWorldSlot>> arguments: storage id => param offset => slot */
    public array $args = [];

    /** @var array<int, array{int, ?int}> call arities: storage id => [min, max] (max null: unpacked) */
    public array $arities = [];

    /** @var array<string, array<int, ClosedWorldSlot>> calls on receivers of unknown type: method lc => offset */
    public array $unknown_method_args = [];

    /** @var array<int, array<int, ClosedWorldSlot>> what bodies leave in by-reference parameters */
    public array $outs = [];

    /** Whether some callable value reaching a dynamic call is not resolved by its type. */
    public bool $unknown_callables = false;

    /** @var array<string, ClosedWorldSlot> writes of global and static variables, by slot key */
    public array $global_writes = [];

    /** @var array<int, ClosedWorldSlot> body return contributions: storage id => slot */
    public array $returns = [];

    /** @var array<int, true> storages whose body always exits */
    public array $always_exits = [];

    /** @var array<int, true> storages whose body was analyzed */
    public array $analyzed = [];

    /** @var array<string, ClosedWorldSlot> property writes: "declaring class lc::name" => slot */
    public array $props = [];

    /** @var array<string, ClosedWorldSlot> writes to properties of receivers of unknown type: name => slot */
    public array $unknown_prop_writes = [];

    /** @var array<string, array<string, ClosedWorldSlot>> writes of undeclared properties: class lc => name */
    public array $dynamic_props = [];

    /** @var array<string, ClosedWorldSlot> `$o->$name = v`: class lc (or `*`) => slot */
    public array $any_prop_writes = [];

    /** @var array<int, ClosedWorldSlot> arguments of unresolved dynamic calls, by offset */
    public array $dynamic_pool = [];

    public ?ClosedWorldSlot $dynamic_pool_spread = null;

    /**
     * `new static(...)` / `new $class(...)`: class lc (`*`: unknown) => offset => slot; the constructor of the
     * class or of any subclass receives them.
     *
     * @var array<string, array<int, ClosedWorldSlot>>
     */
    public array $polymorphic_new = [];

    /** @var array<string, array<int, bool>> file => branch start => whether it was ever reachable */
    public array $branches = [];

    /** @param array<string, self> $parts */
    public static function merge(array $parts): self
    {
        $all = new self();
        foreach ($parts as $c) {
            self::mergeSlots2($all->args, $c->args);
            foreach ($c->arities as $id => [$min, $max]) {
                $old = $all->arities[$id] ?? null;
                $all->arities[$id] = $old === null
                    ? [$min, $max]
                    : [\min($old[0], $min), $old[1] === null || $max === null ? null : \max($old[1], $max)];
            }
            self::mergeSlots2($all->unknown_method_args, $c->unknown_method_args);
            self::mergeSlots2($all->outs, $c->outs);
            $all->unknown_callables = $all->unknown_callables || $c->unknown_callables;
            self::mergeSlots($all->global_writes, $c->global_writes);
            self::mergeSlots($all->returns, $c->returns);
            $all->always_exits += $c->always_exits;
            $all->analyzed += $c->analyzed;
            self::mergeSlots($all->props, $c->props);
            self::mergeSlots($all->unknown_prop_writes, $c->unknown_prop_writes);
            self::mergeSlots2($all->dynamic_props, $c->dynamic_props);
            self::mergeSlots($all->any_prop_writes, $c->any_prop_writes);
            self::mergeSlots($all->dynamic_pool, $c->dynamic_pool);
            if ($c->dynamic_pool_spread !== null) {
                ($all->dynamic_pool_spread ??= new ClosedWorldSlot())->merge($c->dynamic_pool_spread);
            }
            self::mergeSlots2($all->polymorphic_new, $c->polymorphic_new);
            foreach ($c->branches as $file => $branches) {
                foreach ($branches as $pos => $reachable) {
                    $all->branches[$file][$pos] = ($all->branches[$file][$pos] ?? false) || $reachable;
                }
            }
        }
        return $all;
    }

    /**
     * @param array<array-key, ClosedWorldSlot> $into
     * @param array<array-key, ClosedWorldSlot> $from
     */
    private static function mergeSlots(array &$into, array $from): void
    {
        foreach ($from as $k => $slot) {
            ($into[$k] ??= new ClosedWorldSlot())->merge($slot);
        }
    }

    /**
     * @param array<array-key, array<array-key, ClosedWorldSlot>> $into
     * @param array<array-key, array<array-key, ClosedWorldSlot>> $from
     */
    private static function mergeSlots2(array &$into, array $from): void
    {
        foreach ($from as $k => $slots) {
            $into[$k] ??= [];
            self::mergeSlots($into[$k], $slots);
        }
    }
}
