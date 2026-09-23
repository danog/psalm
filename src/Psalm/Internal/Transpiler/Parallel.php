<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use Closure;
use ReflectionClass;
use ReflectionProperty;
use RuntimeException;

use function array_is_list;
use function count;
use function is_array;
use function is_int;
use function is_object;
use function serialize;
use function spl_object_id;
use function unserialize;


/**
 * Parallelism for the emission phases, on Psalm's own worker pool (the same init/tasks/shutdown protocol
 * as the parallel analysis: `Fork\Pool`, forked workers, results serialized back at shutdown).
 *
 * The parent has built the whole program model; each worker is a fork of it (copy-on-write, so the model is
 * shared for free), emits the items it is handed, and at shutdown sends back the state its emission
 * accumulated. The parent merges the workers' state and carries on as if it had emitted everything itself.
 *
 * What a worker accumulates is found empirically (TRANSPILE_STATE_PROBE=1 lists every property that changes
 * during a phase); `$accumulators` names them as [object, property] pairs. Merging is generic:
 *  - a list gets the elements the worker appended,
 *  - a map gets the keys the worker added; an int value that existed before the fork gets the worker's
 *    delta (counters), an array value merges recursively,
 *  - an int property takes the maximum (name counters).
 *
 * Objects travel by value, except the ones every process already has: anything registered in `$shared`
 * (the program model: classes, methods, fields, functions) is sent as a reference and resolved to the
 * parent's own object. RustTypes are re-interned on arrival, so identity comparisons keep working.
 *
 * @internal
 */
final class Parallel
{
    /** @var array<int, object> spl_object_id => object, for objects that exist in every process */
    private array $shared = [];

    /** @param iterable<object> $shared */
    public function __construct(iterable $shared)
    {
        foreach ($shared as $o) {
            $this->shared[spl_object_id($o)] = $o;
        }
    }

    public static function jobs(): int
    {
        $n = (int) (getenv('TRANSPILE_JOBS') ?: 0);
        return $n > 1 && function_exists('pcntl_fork') ? $n : 1;
    }

    /** The job the forked workers run (they inherit it from the parent at fork time). */
    public static ?Parallel $active = null;

    /** @var list<mixed> */
    private array $items = [];
    private ?Closure $work = null;
    /** @var list<array{object, string}> */
    private array $accumulators = [];
    /** @var list<mixed> the accumulators' values at fork time */
    private array $before = [];
    private ?Closure $collect_extra = null;

    /**
     * Run `$work` for every item on Psalm's worker pool (vimeo/psalm's Fork\Pool: forked amphp workers, one
     * task per item handed to whichever worker is idle), then merge what each worker accumulated.
     *
     * @template T
     * @param list<T> $items
     * @param Closure(T): void $work
     * @param list<object> $roots objects whose array/int properties the work may add to: every property a
     *   worker changed is sent back (only the changed entries) and merged into the parent's
     * @param Closure(): array<array-key, mixed> $collect_extra worker-side extra result (e.g. module texts)
     * @param Closure(array<array-key, mixed>): void $merge_extra parent-side merge of one worker's extra result
     */
    public function run(
        array $items,
        int $jobs,
        Closure $work,
        array $roots,
        Closure $collect_extra,
        Closure $merge_extra,
    ): void {
        if ($jobs <= 1 || count($items) < 2) {
            foreach ($items as $item) {
                $work($item);
            }
            return;
        }
        // the pre-fork state: workers send what they changed relative to it, the parent merges that in
        $accumulators = [];
        foreach ($roots as $obj) {
            foreach ((new ReflectionClass($obj))->getProperties() as $p) {
                if ($p->isStatic() || !$p->isInitialized($obj)) {
                    continue;
                }
                $v = $p->getValue($obj);
                if (is_array($v) || is_int($v)) {
                    $accumulators[] = [$obj, $p->getName()];
                }
            }
        }
        $before = [];
        foreach ($accumulators as $i => [$obj, $prop]) {
            $before[$i] = self::read($obj, $prop);
        }
        $this->before = $before;
        $this->items = $items;
        $this->work = $work;
        $this->accumulators = $accumulators;
        $this->collect_extra = $collect_extra;
        self::$active = $this;
        $progress = \Psalm\Internal\Analyzer\ProjectAnalyzer::getInstance()->progress;
        $pool = new \Psalm\Internal\Fork\Pool($jobs, 600.0, $progress);
        try {
            // upstream's protocol (Codebase\Analyzer::doAnalysis): start every worker with an init task first
            // -- a worker forked lazily in the middle of run() never delivers its results
            \Amp\Future\await($pool->runAll(new ParallelInitTask()));
            $pool->run(array_map('strval', array_keys($items)), ParallelItemTask::class);
            $results = \Amp\Future\await($pool->runAll(new ParallelShutdownTask()));
        } finally {
            $pool->shutdown();
            self::$active = null;
        }
        foreach ($results as $payload) {
            [$acc, $extra] = unserialize($payload);
            foreach ($acc as $i => $changed) {
                [$obj, $prop] = $accumulators[$i];
                self::write($obj, $prop, self::merge(self::read($obj, $prop), $before[$i], $this->decode($changed)));
            }
            $merge_extra($this->decode($extra));
        }
    }

    /** Worker side: one item. */
    public function runItem(int $index): void
    {
        ($this->work)($this->items[$index]);
    }

    /** Worker side: everything this worker accumulated, encoded for the parent. */
    public function workerResult(): string
    {
        $acc = [];
        foreach ($this->accumulators as $i => [$obj, $prop]) {
            $d = self::delta($this->before[$i], self::read($obj, $prop));
            if ($d !== null) {
                try {
                    $acc[$i] = $this->encode($d);
                } catch (RuntimeException $e) {
                    throw new RuntimeException($e->getMessage() . ' (in ' . $obj::class . '::$' . $prop . ')', 0, $e);
                }
            }
        }
        return serialize([$acc, $this->encode(($this->collect_extra)())]);
    }

    private static function read(object $obj, string $prop): mixed
    {
        return (new ReflectionProperty($obj, $prop))->getValue($obj);
    }

    private static function write(object $obj, string $prop, mixed $v): void
    {
        (new ReflectionProperty($obj, $prop))->setValue($obj, $v);
    }

    /**
     * What `$after` changed relative to `$before`, or null for nothing: a changed int, a list's appended tail
     * (as the full list: merge() takes the elements past `$before`), a map's new or changed entries (nested
     * maps by their own delta).
     */
    private static function delta(mixed $before, mixed $after): mixed
    {
        if ($before === $after) {
            return null;
        }
        if (!is_array($before) || !is_array($after)) {
            return $after;
        }
        if (array_is_list($before) && array_is_list($after)) {
            return count($after) > count($before) ? $after : null;
        }
        $out = [];
        foreach ($after as $k => $v) {
            if (!array_key_exists($k, $before)) {
                $out[$k] = $v;
            } elseif ($before[$k] !== $v) {
                $out[$k] = is_array($v) && is_array($before[$k]) ? (self::delta($before[$k], $v) ?? []) : $v;
            }
        }
        return $out === [] ? null : $out;
    }

    /** `$cur` (the parent, possibly merged with earlier workers) plus what `$worker` changed relative to `$before`. */
    private static function merge(mixed $cur, mixed $before, mixed $worker): mixed
    {
        if (is_int($cur) && is_int($worker)) {
            return max($cur, $worker);
        }
        if (!is_array($cur) || !is_array($worker)) {
            return $cur;
        }
        $before = is_array($before) ? $before : [];
        if (array_is_list($worker) && array_is_list($before) && ($cur === [] || array_is_list($cur))) {
            // appended elements
            for ($i = count($before), $n = count($worker); $i < $n; $i++) {
                $cur[] = $worker[$i];
            }
            return $cur;
        }
        foreach ($worker as $k => $v) {
            if (!array_key_exists($k, $cur)) {
                $cur[$k] = $v;
            } elseif (is_int($v) && is_int($cur[$k])) {
                $cur[$k] += $v - (is_int($before[$k] ?? null) ? $before[$k] : 0);
            } elseif (is_array($v) && is_array($cur[$k])) {
                $cur[$k] = self::merge($cur[$k], $before[$k] ?? [], $v);
            }
        }
        return $cur;
    }

    // ------------------------------------------------------------------ transport

    /** @var array<int, TransportObject> spl_object_id => encoded, within one encode() call (shared subtrees) */
    private array $seen = [];

    private function encode(mixed $v): mixed
    {
        $this->seen = [];
        return $this->enc($v);
    }

    private function enc(mixed $v): mixed
    {
        if (is_array($v)) {
            foreach ($v as $k => $x) {
                if (is_array($x) || is_object($x)) {
                    $v[$k] = $this->enc($x);
                }
            }
            return $v;
        }
        if (!is_object($v)) {
            return $v;
        }
        $id = spl_object_id($v);
        if (isset($this->shared[$id]) && $this->shared[$id] === $v) {
            return new TransportRef($id);
        }
        if (isset($this->seen[$id])) {
            return $this->seen[$id];
        }
        if ($v instanceof \Closure || $v instanceof \WeakMap || $v instanceof \Generator
            || !str_starts_with($v::class, 'Psalm\\Internal\\Transpiler\\')
        ) {
            // analysis objects (AST nodes, storages, types) are the parent's too, but reachable only through the
            // shared model: one arriving here means the model grew an object the registry does not know
            throw new RuntimeException('cannot send a ' . $v::class . ' back from a transpile worker');
        }
        $t = new TransportObject($v::class);
        $this->seen[$id] = $t;
        for ($r = new ReflectionClass($v); $r !== false; $r = $r->getParentClass()) {
            foreach ($r->getProperties() as $p) {
                if ($p->isStatic() || $p->getDeclaringClass()->getName() !== $r->getName() || !$p->isInitialized($v)) {
                    continue;
                }
                $t->props[] = [$r->getName(), $p->getName(), $this->enc($p->getValue($v))];
            }
        }
        return $t;
    }

    /** @var array<int, object> TransportObject id => decoded */
    private array $decoded = [];
    /** @var array<int, true> objects whose properties are still being decoded */
    private array $in_progress = [];
    private bool $cycle = false;

    private function decode(mixed $v): mixed
    {
        $this->decoded = [];
        return $this->dec($v);
    }

    private function dec(mixed $v): mixed
    {
        if (is_array($v)) {
            foreach ($v as $k => $x) {
                if (is_array($x) || is_object($x)) {
                    $v[$k] = $this->dec($x);
                }
            }
            return $v;
        }
        if ($v instanceof TransportRef) {
            return $this->shared[$v->id] ?? throw new RuntimeException('unknown shared object in a worker result');
        }
        if (!$v instanceof TransportObject) {
            return $v;
        }
        $key = spl_object_id($v);
        if (isset($this->decoded[$key])) {
            if (isset($this->in_progress[$key])) {
                $this->cycle = true; // a reference back into an object still being built
            }
            return $this->decoded[$key];
        }
        $o = (new ReflectionClass($v->class))->newInstanceWithoutConstructor();
        $this->decoded[$key] = $o;
        $this->in_progress[$key] = true;
        $outer_cycle = $this->cycle;
        $this->cycle = false;
        foreach ($v->props as [$class, $name, $value]) {
            (new ReflectionProperty($class, $name))->setValue($o, $this->dec($value));
        }
        unset($this->in_progress[$key]);
        // a RustType joins this process's interned instances, unless it is part of a cycle (a recursive type
        // references itself): that one stays as built, its key is still computable once the cycle is closed
        if ($o instanceof RustType && !$this->cycle) {
            $o = RustType::reintern($o);
            $this->decoded[$key] = $o;
        }
        $this->cycle = $outer_cycle || $this->cycle;
        return $o;
    }
}

/** @internal a reference to an object every process has (see Parallel::$shared) */
final class TransportRef
{
    public function __construct(public int $id)
    {
    }
}

/** @internal an object sent by value */
final class TransportObject
{
    /** @var list<array{class-string, string, mixed}> declaring class, property, encoded value */
    public array $props = [];

    public function __construct(public string $class)
    {
    }
}

/** @internal emit one item on a worker */
final class ParallelItemTask implements \Amp\Parallel\Worker\Task
{
    public function __construct(private readonly string $index)
    {
    }

    public function run(\Amp\Sync\Channel $channel, \Amp\Cancellation $cancellation): int
    {
        Parallel::$active?->runItem((int) $this->index);
        return 0;
    }
}

/** @internal collect a worker's accumulated state */
final class ParallelShutdownTask implements \Amp\Parallel\Worker\Task
{
    public function run(\Amp\Sync\Channel $channel, \Amp\Cancellation $cancellation): string
    {
        return Parallel::$active?->workerResult() ?? throw new RuntimeException('no active transpile job in this worker');
    }
}

/** @internal starts a worker (every worker is forked before any item is handed out) */
final class ParallelInitTask implements \Amp\Parallel\Worker\Task
{
    public function run(\Amp\Sync\Channel $channel, \Amp\Cancellation $cancellation): int
    {
        return 0;
    }
}
