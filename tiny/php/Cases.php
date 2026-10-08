<?php

/**
 * Regression suite for transpiler features, run via the tiny fast loop.
 * Each case computes an actual string and compares to an expected literal;
 * run_all() returns one PASS/FAIL line per case. `bash run.sh` fails on any FAIL.
 *
 * Add a feature -> add a case here so the fast loop catches regressions.
 */

namespace Tiny;

// ---- shared fixtures ----------------------------------------------------

final class A
{
    public function __construct(public int $a = 1) {}
}

final class B
{
    public function __construct(public int $b = 2) {}
}

final class Holder
{
    public A|B|null $x = null;
}

final class Mut
{
    public int $a = 1;
}

final class R
{
    public const RECONCILIATION_OK = 0;
    public const RECONCILIATION_REDUNDANT = 1;
    public const RECONCILIATION_EMPTY = 2;
}

enum Suit: string
{
    case Hearts = 'H';
    case Spades = 'S';
}

// ---- feature: union->Option flatten (nullable-union field assignment) ----

/** @return ($n is A ? A : ($n is B ? B : A|B|null)) */
function resolve(A|B|null $n): A|B|null
{
    return $n;
}

function case_nullable_union(): string
{
    $h = new Holder();
    $h->x = resolve(null);
    $r1 = $h->x === null ? 'null' : 'BAD';
    $h->x = resolve(new A(7));
    $r2 = $h->x instanceof A ? ('A' . $h->x->a) : 'BAD';
    return $r1 . $r2;
}

// ---- feature: wildcard class-constant type (Foo::BAR_*) -> not Mixed ------

/** @param R::RECONCILIATION_* $fail */
function reconcileInner(int &$fail): int
{
    $fail = R::RECONCILIATION_REDUNDANT;
    return 1;
}

function case_wildcard_const(): string
{
    $did = R::RECONCILIATION_OK;
    reconcileInner($did);
    return $did ? 'set' : 'unset';
}

// ---- feature: value-of<Enum> -> backing type -----------------------------

/** @param value-of<Suit> $v */
function takesValueOf(string $v): string
{
    return $v;
}

function case_value_of(): string
{
    return takesValueOf('H') . takesValueOf(Suit::Spades->value);
}

// ---- feature: return-move (axis 5) ---------------------------------------

function makeReturn(): A
{
    $x = new A(5);
    return $x;
}

function condReturn(bool $c): A
{
    $x = new A(3);
    if ($c) {
        return $x;
    }
    $x = new A(4);
    return $x;
}

function case_return_move(): string
{
    return makeReturn()->a . '' . condReturn(true)->a . condReturn(false)->a;
}

// ---- feature: single-use-local move (axis 5) -----------------------------

function consume(A $f): int
{
    return $f->a;
}

function passOnce(): int
{
    $x = new A(7);
    return consume($x);
}

function twice(): int
{
    $x = new A(3);
    return consume($x) + $x->a;
}

/** @param list<int> $items */
function inLoop(array $items): int
{
    $x = new A(2);
    $sum = 0;
    foreach ($items as $i) {
        $sum = $sum + consume($x);
    }
    return $sum;
}

function case_single_use_move(): string
{
    return passOnce() . '' . twice() . inLoop([1, 2, 3]);
}

// ---- feature: move single-use collection into foreach (not clone) ---------

/** @param list<int> $items */
function sumList(array $items): int
{
    $total = 0;
    foreach ($items as $n) {
        $total = $total + $n;
    }
    return $total;
}

function case_foreach_collection_move(): string
{
    $data = [10, 20, 30];
    return (string) sumList($data);
}

// ---- feature: object identity preserved through moves ---------------------

function case_alias_mutation(): string
{
    $y = new Mut();
    $x = $y;
    $x->a = 7;
    return (string) $y->a;
}

// ---- regression: while/do loops must transpile (scanSingleUse cond handling) --

function whileLoop(int $n): int
{
    $sum = 0;
    $i = 0;
    while ($i < $n) {
        $sum = $sum + $i;
        $i = $i + 1;
    }
    return $sum;
}

function doLoop(int $n): int
{
    $sum = 0;
    $i = 0;
    do {
        $sum = $sum + $i;
        $i = $i + 1;
    } while ($i < $n);
    return $sum;
}

function case_loops(): string
{
    return whileLoop(4) . '' . doLoop(3);
}

// ---- regression: assorted control flow (nested loops, break/continue, foreach-key, switch) --

/** @param list<list<int>> $rows */
function nested(array $rows): int
{
    $sum = 0;
    foreach ($rows as $row) {
        foreach ($row as $v) {
            $sum = $sum + $v;
        }
    }
    return $sum;
}

function withBreakContinue(int $n): int
{
    $sum = 0;
    for ($i = 0; $i < $n; $i++) {
        if ($i === 2) {
            continue;
        }
        if ($i === 5) {
            break;
        }
        $sum = $sum + $i;
    }
    return $sum;
}

/** @param array<string, int> $m */
function foreachKey(array $m): int
{
    $sum = 0;
    foreach ($m as $k => $v) {
        $sum = $sum + strlen($k) + $v;
    }
    return $sum;
}

function switchCase(int $n): string
{
    switch ($n) {
        case 1:
            return 'one';
        case 2:
            return 'two';
        default:
            return 'other';
    }
}

function case_control_flow(): string
{
    return nested([[1, 2], [3, 4]]) . '|' . withBreakContinue(10) . '|' . foreachKey(['ab' => 3]) . '|' . switchCase(2);
}

// ---- edge: single-use as a method-call receiver moves safely -------------

function getA(A $f): int
{
    return $f->a;
}

function case_receiver_move(): string
{
    $x = new A(11);
    return (string) getA($x);
}

// ---- feature: &T borrowed param for non-escaping read-only param ---------

/** $a is used only as a property-fetch base (read) -> non-escaping -> borrowed &T. */
function readOnly(A $a): int
{
    return $a->a + $a->a;
}

/** $a escapes (returned) -> must stay owned T. */
function keepIt(A $a): A
{
    return $a;
}

function case_borrow_param(): string
{
    $x = new A(21);
    // $x used twice (not single-use): without borrow each call clones; with &T param -> &x, no clone.
    $r1 = readOnly($x);
    $r2 = readOnly($x);
    // escaping param still works (owned)
    $y = keepIt(new A(5));
    return $r1 . '' . $r2 . $y->a;
}

// ---- feature: &T borrowed param on a PRIVATE method -----------------------

final class Calc
{
    public int $base = 100;

    /** private, non-dispatched: $x used only as a property base -> borrowed &T */
    private function add(A $x): int
    {
        return $this->base + $x->a;
    }

    public function total(A $x): int
    {
        // $x passed twice as an arg (owned here) to the borrow-param private method
        return $this->add($x) + $this->add($x);
    }
}

function case_private_method_borrow(): string
{
    $c = new Calc();
    return (string) $c->total(new A(5));
}

// ---- feature: &T borrowed param on a private static method ----------------

final class StaticCalc
{
    private static function twice(A $x): int
    {
        return $x->a + $x->a;
    }

    public static function run(A $x): int
    {
        return self::twice($x) + self::twice($x);
    }
}

function case_private_static_borrow(): string
{
    return (string) StaticCalc::run(new A(8));
}

// ---- feature: &T borrowed param on a leaf class's own public method -------

final class LeafSvc
{
    public function describe(A $a): int
    {
        return $a->a * 2;
    }
}

function case_leaf_public_borrow(): string
{
    $s = new LeafSvc();
    $x = new A(6);
    return (string) ($s->describe($x) + $s->describe($x));
}

// ---- feature: dispatch-agreement -- borrow only if ALL impls agree --------

// Every implementation is borrow-safe -> the interface method + both impls all take &A.
interface Reader
{
    public function read(A $a): int;
}

final class ReaderX implements Reader
{
    public function read(A $a): int
    {
        return $a->a;
    }
}

final class ReaderY implements Reader
{
    public function read(A $a): int
    {
        return $a->a * 3;
    }
}

function dispatchRead(Reader $r, A $a): int
{
    return $r->read($a);
}

function case_dispatch_agree_borrow(): string
{
    return dispatchRead(new ReaderX(), new A(2)) . '' . dispatchRead(new ReaderY(), new A(2));
}

// One implementation escapes the param (passes it to an owned-param fn) -> the whole group stays OWNED.
interface Handler
{
    public function handle(A $a): int;
}

final class HandlerSafe implements Handler
{
    public function handle(A $a): int
    {
        return $a->a;
    }
}

final class HandlerEscapes implements Handler
{
    public function handle(A $a): int
    {
        return keepIt($a)->a; // $a passed to an owned param -> escapes
    }
}

function dispatchHandle(Handler $h, A $a): int
{
    return $h->handle($a);
}

function case_dispatch_disagree_owned(): string
{
    return dispatchHandle(new HandlerSafe(), new A(3)) . '' . dispatchHandle(new HandlerEscapes(), new A(4));
}

// ---- machinery: union-member narrowing + concrete-base downcast (Reconciler shape) --
// Faithful reduction of Psalm's Type\Atomic hierarchy (Atomic -> concrete-base Arr2 -> NonEmptyArr2; KeyedArr2)
// and the Reconciler's instanceof-narrow-then-downcast. Guards that the transpiler's narrowing/is_instance/
// downcast are sound for concrete-base-with-subclasses (the mechanism behind the TNonEmptyArray->TKeyedArray
// question -- which is a Psalm-analysis optimism, not a transpiler bug: this reproduction does NOT panic).

abstract class Atomic2 {}
class Arr2 extends Atomic2
{
    public int $k = 1;
}
final class NonEmptyArr2 extends Arr2
{
    public int $n = 2;
}
final class KeyedArr2 extends Atomic2
{
    public int $p = 3;
}

/** @return list<Atomic2> */
function atomics(): array
{
    return [new NonEmptyArr2(), new Arr2(), new KeyedArr2()];
}

function case_narrow_downcast(): string
{
    $out = '';
    foreach (atomics() as $a) {
        if ($a instanceof KeyedArr2 || $a instanceof Arr2) {
            if ($a instanceof Arr2) {
                $out .= 'arr' . $a->k;
            } else {
                $out .= 'keyed' . $a->p;
            }
        } else {
            $out .= 'other';
        }
    }
    return $out;
}

// ---- safety: a param whose method is unmodeled (dynamic dispatch) stays OWNED

// DOMDocument::getElementsByTagNameNS is not in the stub -> dynamic call_method path casts the receiver to
// Mixed, which can't apply to a &T. $d must therefore NOT be borrowed. (Compiled, not called at runtime.)
function domDynamic(\DOMDocument $d): int
{
    $list = $d->getElementsByTagNameNS('ns', 'tag');
    return 0;
}

// ---- safety: a borrow-param fn used as a first-class callable still works --

function readOnlyCb(A $a): int
{
    return $a->a + 1;
}

/** @param callable(A): int $f */
function applyIt(callable $f, A $a): int
{
    return $f($a);
}

function case_callable_borrow(): string
{
    $cb = readOnlyCb(...); // first-class callable of a &T-param function
    return (string) applyIt($cb, new A(10));
}

// ---- feature: &self call on a Late-local receiver borrows (no clone) ------

final class Counter
{
    public int $n = 0;
    public function get(): int
    {
        return $this->n;
    }
}

function case_late_receiver_borrow(): string
{
    // $c is reassigned -> Late local; used twice as a &self receiver -> should borrow, not clone.
    $c = new Counter();
    $c = new Counter();
    return $c->get() . '' . $c->get();
}

// ---- edge: a var captured by a closure must NOT be moved -----------------

function case_closure_capture(): string
{
    $x = new A(4);
    $f = function () use ($x): int {
        return $x->a;
    };
    // $x read both in the closure and here: must stay cloned, closure still valid
    return (string) ($x->a + $f());
}

// ---- runner --------------------------------------------------------------

// ---- feature: constructs introduced by the member-name id port (psalm-port 2026-09-24) ----

final class NameTable
{
    /** @var array<string, int> */
    private static array $ids = [];
    /** @var array<int, string> */
    private static array $strings = [];

    public static function intern(string $string): int
    {
        return self::$ids[$string] ?? self::add($string);
    }

    private static function add(string $string): int
    {
        $id = self::hash($string);
        self::$strings[$id] = $string;
        self::$ids[$string] = $id;
        return $id;
    }

    public static function hash(string $string): int
    {
        /** @var array{1: int} $unpacked */
        $unpacked = unpack('q', hash('xxh3', $string, true));
        return $unpacked[1] & PHP_INT_MAX;
    }

    public static function lookup(int $id): string
    {
        return self::$strings[$id] ?? '?';
    }

    /** @return lowercase-string */
    public static function lookupLc(int $id): string
    {
        /** @psalm-suppress LessSpecificReturnStatement */
        return self::lookup($id);
    }
}

final class MemberStore
{
    /** @var array<int, string> */
    public array $members = [];
    private ?string $memo_a = null;
    private ?string $memo_b = null;

    public function memo(bool $b): string
    {
        return $b
            ? ($this->memo_b ??= 'B' . count($this->members))
            : ($this->memo_a ??= 'A' . count($this->members));
    }
}

function case_id_keyed_map(): string
{
    $store = new MemberStore();
    $store->members[NameTable::intern('foo')] = 'Foo';
    $store->members[NameTable::intern('bar')] = 'Bar';
    $hit = isset($store->members[NameTable::intern('foo')]) ? 'y' : 'n';
    $miss = isset($store->members[NameTable::intern('baz')]) ? 'y' : 'n';
    $names = implode(',', array_map(NameTable::lookupLc(...), array_keys($store->members)));
    $kept = array_filter(
        $store->members,
        static fn(int $key): bool => in_array($key, [NameTable::intern('bar')], true),
        ARRAY_FILTER_USE_KEY,
    );
    $out = $hit . $miss . ':' . $names . ':' . implode(',', $kept);
    foreach ($store->members as $id => $member) {
        $name = NameTable::lookup($id);
        $out .= ':' . $name . '=' . $member;
    }
    return $out . ':' . $store->memo(true) . $store->memo(false) . $store->memo(true)
        . ':' . (NameTable::hash('Foo\\Bar') === 5094806515607143651 ? 'h' : 'H')
        . (__rt_str_id('Foo\\Bar') === 5094806515607143651 ? 'r' : 'R');
}

abstract class Kind
{
    abstract public function name(): string;
}

final class KindA extends Kind
{
    public function name(): string
    {
        return 'a';
    }
}

final class KindB extends Kind
{
    public function name(): string
    {
        return 'b';
    }
}

final class KindC extends Kind
{
    public function name(): string
    {
        return 'c';
    }
}

final class KindDispatch
{
    /** @var array<class-string, int> */
    private static array $kinds = [];

    private static function kind(Kind $o): int
    {
        if ($o instanceof KindA) {
            return 1;
        }
        if ($o instanceof KindB) {
            return 2;
        }
        return 0;
    }

    public static function handle(Kind $o): string
    {
        $kind = self::$kinds[$o::class] ??= self::kind($o);
        switch ($kind) {
            case 1:
                assert($o instanceof KindA);
                return 'A' . $o->name();
            case 2:
                assert($o instanceof KindA || $o instanceof KindB);
                return 'B' . $o->name();
        }
        return '-' . $o->name() . $o::class;
    }
}

function case_kind_dispatch(): string
{
    return KindDispatch::handle(new KindA()) . KindDispatch::handle(new KindB()) . KindDispatch::handle(new KindA())
        . KindDispatch::handle(new KindC());
}

final class KeyIds
{
    /** @var array<string, int> */
    private static array $ids = [];

    public static function keyId(string $key): int
    {
        return self::$ids[$key] ??= count(self::$ids);
    }
}

/** @return list<string> */
function names_a(): array
{
    $out = [];
    $out[] = 'a';
    return $out;
}

/** @return list<string> */
function names_of(bool $with_b): array
{
    $a = names_a();
    $b = $with_b ? names_a() : null;
    if ($b !== null) {
        $b[0] = 'b';
    }
    if ($a === []) {
        return $b ?? [];
    }
    if ($b === null) {
        return $a;
    }
    return array_merge($a, $b);
}

function case_coalesce_assign_var_key(): string
{
    $k = 'x';
    $first = KeyIds::keyId($k);
    $again = KeyIds::keyId($k);
    $other = KeyIds::keyId('y');
    return $first . $again . $other . ':' . implode(',', names_of(true)) . ':' . implode(',', names_of(false));
}

abstract class SubNode
{
    abstract public function tag(): string;
}

final class LeafNode extends SubNode
{
    public function tag(): string
    {
        return 'L';
    }
}

final class OtherThing
{
}

final class SubNodeHolder
{
    /** @var object|list<object|null>|string|int|null */
    public mixed $child = null;

    /** @return SubNode|list<SubNode|null>|string|int|null */
    public function typedChild(): mixed
    {
        return $this->child;
    }
}

function describe_child(SubNodeHolder $holder): string
{
    $typed = $holder->typedChild();
    if ($typed instanceof SubNode) {
        return $typed->tag() . ',';
    }
    if (is_array($typed)) {
        return count($typed) . ($typed[0] instanceof SubNode ? $typed[0]->tag() : '?') . ',';
    }
    if ($typed === null) {
        return 'n,';
    }
    return (string) $typed . ',';
}

function case_object_union_narrowing(): string
{
    $holder = new SubNodeHolder();
    $holder->child = new LeafNode();
    $out = describe_child($holder);
    $holder->child = [new LeafNode(), null];
    $out .= describe_child($holder);
    $holder->child = 'str';
    $out .= describe_child($holder);
    $holder->child = 7;
    $out .= describe_child($holder);
    $holder->child = null;
    return $out . describe_child($holder);
}

function case_const_table(): string
{
    $consts = ConstTable::get();
    $inf = $consts['INF'];
    $nan = $consts['NAN'];
    $out = (is_float($inf) && is_infinite($inf) ? 'inf' : '?') . ',';
    $out .= (is_float($nan) && is_nan($nan) ? 'nan' : '?') . ',';
    $out .= (array_key_exists('NULL_ONE', $consts) && $consts['NULL_ONE'] === null ? 'null' : '?') . ',';
    $out .= (is_int($consts['E_ALL']) ? (string) $consts['E_ALL'] : '?') . ',';
    $out .= ($consts['PHP_EOL'] === "\n" ? 'eol' : '?') . ',';
    $out .= (isset($consts['NOPE']) ? '?' : 'absent') . ',';
    return $out . count($consts);
}

function check(string $name, string $actual, string $expected): string
{
    return ($actual === $expected ? 'PASS ' : 'FAIL ') . $name
        . ' got=' . $actual . ' want=' . $expected . "\n";
}

// ---- feature: empty array literals typed from context (no Mixed) ----

/** @return list<string> */
function empty_accum(int $n): array
{
    $out = [];
    for ($i = 0; $i < $n; $i++) {
        $out[] = "s$i";
    }
    return $out;
}

/** @return array{code: string, ignored: list<string>, assertions: array<string, string>} */
function empty_shape(): array
{
    return ['code' => 'c', 'ignored' => [], 'assertions' => []];
}

abstract class ProvBase
{
    /** @return iterable<string, array{code: string, ignored?: list<string>}> */
    abstract public function provider(): iterable;
}

final class Prov extends ProvBase
{
    public function provider(): iterable
    {
        yield 'a' => ['code' => 'x', 'ignored' => []];
        yield 'b' => ['code' => 'y'];
    }
}

function case_empty_arrays(): string
{
    $r = implode(',', empty_accum(2));
    $s = empty_shape();
    $p = new Prov();
    $n = 0;
    foreach ($p->provider() as $row) {
        $n += strlen($row['code']) + count($row['ignored'] ?? []);
    }
    return $r . '|' . count($s['ignored']) . '|' . $n;
}

// ---- feature: typed throw path (no Mixed in throw/catch/rethrow/match) ----

final class MyEx extends \Exception
{
}

function thrower(int $n): int
{
    if ($n > 2) {
        throw new MyEx("big $n");
    }
    return $n * 2;
}

function case_try_catch(): string
{
    $out = [];
    foreach ([1, 3] as $n) {
        try {
            $out[] = (string) thrower($n);
        } catch (MyEx $e) {
            $out[] = 'caught:' . $e->getMessage();
        } finally {
            $out[] = 'f';
        }
    }
    try {
        try {
            throw new \RuntimeException('inner');
        } catch (MyEx $e) {
            $out[] = 'wrong';
        }
    } catch (\Exception $e) {
        $out[] = get_class($e) === 'RuntimeException' ? 'outer' : 'bad';
    }
    $out[] = match (1) { 2 => 'x', default => 'd' };
    try {
        $v = match (7) { 1 => 'a' };
        $out[] = $v;
    } catch (\UnhandledMatchError $e) {
        $out[] = 'unhandled';
    }
    return implode(',', $out);
}

// ---- feature: Rust generics for unbounded @template (no Mixed) ----

final class GAssert
{
    /**
     * @template T
     * @param T $expected
     * @param T $actual
     */
    public static function same($expected, $actual, string $label): string
    {
        if ($expected !== $actual) {
            return $label . ':ne';
        }
        return $label . ':eq';
    }

    /**
     * @template T
     * @param T $value
     * @return T
     */
    public static function identity($value)
    {
        return $value;
    }

    /**
     * @template T
     * @param list<T> $items
     * @param T $needle
     */
    public static function has(array $items, $needle): bool
    {
        foreach ($items as $item) {
            if ($item === $needle) {
                return true;
            }
        }
        return false;
    }
}

/**
 * @template T
 * @param T $v
 * @return string
 */
function g_describe($v): string
{
    return is_string($v) ? 'str' : (is_int($v) ? 'int' : 'other');
}

function case_generics(): string
{
    $out = [];
    $out[] = GAssert::same(1, 1, 'i');
    $out[] = GAssert::same('a', 'b', 's');
    $out[] = GAssert::same([1, 2], [1, 2], 'l');
    $out[] = (string) GAssert::identity(41 + 1);
    $out[] = GAssert::identity('x') . 'y';
    $out[] = GAssert::has(['p', 'q'], 'q') ? 'has' : 'no';
    $out[] = GAssert::has([1, 2], 3) ? 'has' : 'no';
    $a = new A(5);
    $out[] = (string) GAssert::identity($a)->a;
    $out[] = g_describe('s') . g_describe(3) . g_describe(1.5);
    return implode(',', $out);
}

// ---- feature: typed PHPUnit (generic Assert + TestCase hooks, no Mixed) ----

final class PuTest extends \PHPUnit\Framework\TestCase
{
    public int $setups = 0;

    public function setUp(): void
    {
        $this->setups++;
    }

    public function testThrows(): void
    {
        $this->expectException(\RuntimeException::class);
        $this->expectExceptionMessage('boom');
        throw new \RuntimeException('kaboom');
    }
}

function case_phpunit(): string
{
    $out = [];
    $t = new PuTest('testThrows');
    $t->runSetUp();
    $out[] = (string) $t->setups;
    \PHPUnit\Framework\Assert::assertSame(3, 1 + 2);
    \PHPUnit\Framework\Assert::assertSame('ab', 'a' . 'b');
    \PHPUnit\Framework\Assert::assertEquals(['x' => 1], ['x' => 1]);
    \PHPUnit\Framework\Assert::assertCount(2, ['p', 'q']);
    \PHPUnit\Framework\Assert::assertContains('q', ['p', 'q']);
    \PHPUnit\Framework\Assert::assertNotNull($t);
    \PHPUnit\Framework\Assert::assertEmpty([]);
    \PHPUnit\Framework\Assert::assertStringContainsString('ell', 'hello');
    \PHPUnit\Framework\Assert::assertEqualsCanonicalizing(['b', 'a'], ['a', 'b']);
    \PHPUnit\Framework\Assert::assertThat('hello', \PHPUnit\Framework\Assert::stringContains('ll'));
    try {
        \PHPUnit\Framework\Assert::assertSame(1, 2, 'custom');
        $out[] = 'no';
    } catch (\PHPUnit\Framework\AssertionFailedError $e) {
        $out[] = str_contains($e->getMessage(), 'custom') && str_contains($e->getMessage(), 'identical') ? 'failed' : $e->getMessage();
    }
    try {
        \PHPUnit\Framework\Assert::markTestSkipped('later');
    } catch (\PHPUnit\Framework\SkippedTestError $e) {
        $out[] = 'skip:' . $e->getMessage();
    }
    try {
        $t->testThrows();
    } catch (\Throwable $e) {
        $out[] = $t->expectsException() ? 'expects' : 'no';
        try {
            $t->verifyExpectedException($e);
            $out[] = 'verified';
        } catch (\PHPUnit\Framework\ExpectationFailedException $f) {
            $out[] = 'mismatch';
        }
        $t->expectExceptionMessage('other');
        try {
            $t->verifyExpectedException($e);
            $out[] = 'bad';
        } catch (\PHPUnit\Framework\ExpectationFailedException $f) {
            $out[] = 'mismatch';
        }
    }
    $t->setDataName('ds');
    $out[] = $t->getName();
    return implode(',', $out);
}

// ---- feature: typed object comparison (== on objects compares properties, no Mixed) ----

final class Pt
{
    public function __construct(public int $x, public string $tag, public ?Pt $next = null) {}
}

function case_object_eq(): string
{
    $a = new Pt(1, 'a', new Pt(2, 'b'));
    $b = new Pt(1, 'a', new Pt(2, 'b'));
    $c = new Pt(1, 'a', new Pt(3, 'b'));
    $out = [];
    $out[] = $a == $b ? 'eq' : 'ne';
    $out[] = $a == $c ? 'eq' : 'ne';
    $out[] = $a != $c ? 'ne' : 'eq';
    $out[] = $a === $b ? 'same' : 'notsame';
    $out[] = $a == $a ? 'eq' : 'ne';
    $x = new A(7);
    $out[] = $x == new A(7) ? 'eq' : 'ne';
    $out[] = $x == new A(8) ? 'eq' : 'ne';
    return implode(',', $out);
}

function case_declared_preg(): string
{
    $out = '';
    $n = 0;
    foreach (get_declared_classes() as $predefined_class) {
        $predefined_class = (string) preg_replace('/^\\\\/', '', $predefined_class, 1);
        if ($predefined_class === 'stdClass') {
            $out .= strtolower($predefined_class);
        }
        $n++;
    }
    return $out . ($n > 0 ? '+' : '-');
}

final class Invokable
{
    public function __invoke(string $socket, int $n): string
    {
        return $socket . $n;
    }
}

function case_invoke_object(): string
{
    $plugin = new Invokable();
    $r = $plugin('sock', 3);
    return $r;
}

abstract class TNode {}
final class TName extends TNode { public function __construct(public string $n) {} }
final class TNullable extends TNode {}
final class TIdent extends TNode { public function __construct(public string $n) {} }

/** @return null|TNode|TName|TNullable */
function pick_node(TNode $n): TNode|null
{
    return $n;
}

/** @param null|TNode|TName|TNullable $t */
function show_node(TNode|null $t): string
{
    if ($t === null) {
        return 'null';
    }
    if ($t instanceof TName) {
        return 'name:' . $t->n;
    }
    if ($t instanceof TNullable) {
        return 'nullable';
    }
    return 'node';
}

function case_union_base_member(): string
{
    $nodes = [new TName('a'), new TIdent('b'), new TNullable()];
    $out = '';
    foreach ($nodes as $n) {
        $out .= show_node(pick_node($n)) . '|';
    }
    return $out;
}

final class ItemBox {
    /** @var list<TName|null> Items (null for skipped elements) */
    public array $items;
    /** @param list<TName|null> $items */
    public function __construct(array $items = []) {
        $this->items = $items;
    }
}

function case_nullable_list_prop(): string
{
    $b = new ItemBox([new TName('x'), null]);
    $n = 0;
    foreach ($b->items as $it) {
        if ($it !== null) {
            $n++;
        }
    }
    return count($b->items) . ':' . $n;
}

function key_to_int(string|int $k): int|false
{
    if (is_int($k)) {
        return $k;
    }
    return false;
}

function case_key_narrow(): string
{
    $out = '';
    foreach ([3, 'x', 7] as $k) {
        $r = key_to_int($k);
        $out .= $r === false ? 'F' : (string) $r;
    }
    return $out;
}

function case_int_or_false_arith(): string
{
    $s = 'ab*/cd';
    $end = strpos($s, '*/');
    if ($end === false) {
        return 'none';
    }
    $end += 2;
    return (string) $end;
}

/** @return class-string */
function some_class(): string
{
    return \ItemBox::class;
}

function case_sym_str_funcs(): string
{
    $c = some_class();
    $a = str_replace('Item', 'X', $c);
    $b = (string) preg_replace('/^Item/', 'Y', $c, 1);
    return $a . '|' . $b;
}

class MutCtx {
    /** @var array<string, bool> */
    public array $ids = [];
}

function fill_ctx(MutCtx $c): void
{
    $c->ids['a'] = true;
}

function case_prop_empty_narrow(): string
{
    $c = new MutCtx();
    $c->ids = [];
    fill_ctx($c);
    $got = $c->ids;
    $n = count($got);
    $c->ids = $got;
    return (string) $n . ':' . count($c->ids);
}

abstract class SAtomic { public function __toString(): string { return 'atom'; } }
final class SNamed extends SAtomic { public function __construct(public string $value) {} public function __toString(): string { return 'named:' . $this->value; } }
final class STmplClass extends SAtomic { public function __toString(): string { return 'tmpl'; } }

final class SAssertion {
    /** @param SNamed|STmplClass $type */
    public function __construct(public readonly SAtomic $type) {}
    public function __toString(): string { return 'isa-' . $this->type; }
}

function case_union_tostring(): string
{
    $a = new SAssertion(new SNamed('X'));
    $b = new SAssertion(new STmplClass());
    return (string) $a . '|' . (string) $b;
}

final class SIdent { public function __construct(public string $v) {} }
final class ClassStmt { public function __construct(public ?SIdent $name) {} }
final class EnumStmt { public function __construct(public ?SIdent $name) {} }

function name_or_node(ClassStmt|EnumStmt $c): string
{
    $n = $c->name ?? $c;
    if ($n instanceof SIdent) {
        return 'i:' . $n->v;
    }
    return 'node';
}

function case_union_prop_coalesce(): string
{
    return name_or_node(new ClassStmt(new SIdent('A')))
        . '|' . name_or_node(new ClassStmt(null))
        . '|' . name_or_node(new EnumStmt(new SIdent('E')));
}

abstract class RecvBase { public function kind(): string { return 'base'; } }
final class RecvA extends RecvBase { public function kind(): string { return 'a'; } }
final class RecvB extends RecvBase { public function kind(): string { return 'b'; } }
final class RecvC extends RecvBase {}

function classify_node(RecvBase $n): string
{
    if ($n instanceof RecvA || $n instanceof RecvB) {
        return 'x' . $n->kind();
    }
    return 'other:' . $n->kind();
}

function case_union_receiver_base_method(): string
{
    return classify_node(new RecvA()) . '|' . classify_node(new RecvB()) . '|' . classify_node(new RecvC());
}

final class VerRe
{
    public const PHP_VERSION_REGEX = '^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:\\..*)?$';
    public const SUPPORTED = '^(5\\.[456]|7\\.[01234]|8\\.[012345])(\\..*)?$';
}

function case_version_regex(): string
{
    $a = preg_match('/' . VerRe::PHP_VERSION_REGEX . '/', '7.4') ? 'a' : '-';
    $b = preg_match('/' . VerRe::SUPPORTED . '/', '7.4') ? 'b' : '-';
    $c = preg_match('/' . VerRe::PHP_VERSION_REGEX . '/', 'x.y') ? 'c' : '-';
    return $a . $b . $c;
}

function case_prop_empty_merge(): string
{
    $c = new MutCtx();
    $first = $c->ids;
    $c->ids = [];
    fill_ctx($c);
    $more = $c->ids;
    $c->ids = array_merge($more, $first);
    $n = count($c->ids);
    $c->ids = array_replace($c->ids, $more);
    return $n . ':' . count($c->ids);
}

abstract class ArgBase { public function tag(): string { return 'base'; } }
final class ArgLit extends ArgBase { public function __construct(public int $v) {} public function tag(): string { return 'lit'; } }
final class ArgOther extends ArgBase { public function tag(): string { return 'other'; } }

function takes_arg_base(ArgBase $b): string
{
    return $b->tag();
}

/** @return list<ArgBase> */
function mk_arg_bases(): array
{
    return [new ArgLit(1), new ArgOther()];
}

function case_arg_no_downcast(): string
{
    $out = '';
    foreach (mk_arg_bases() as $x) {
        if ($x instanceof ArgLit) {
            $out .= takes_arg_base($x);
        } else {
            $out .= takes_arg_base($x);
        }
    }
    return $out;
}

abstract class HStmt {}
final class HBlock extends HStmt { /** @param list<HStmt> $stmts */ public function __construct(public array $stmts) {} }
final class HEcho extends HStmt { public function __construct(public string $text) {} }

function count_stmts(HStmt $s): int
{
    $n = 1;
    if (isset($s->stmts)) {
        foreach ($s->stmts as $inner) {
            $n += count_stmts($inner);
        }
    }
    return $n;
}

function case_variant_isset(): string
{
    $tree = new HBlock([new HEcho('a'), new HBlock([new HEcho('b')])]);
    return (string) count_stmts($tree) . ':' . (string) count_stmts(new HEcho('c'));
}

function case_prop_empty_unset(): string
{
    $c = new MutCtx();
    $c->ids = [];
    fill_ctx($c);
    $ids = $c->ids;
    $copy = $ids;
    unset($copy['a']);
    return count($ids) . ':' . count($copy);
}

/** A class whose truthiness the engine decides, like SimpleXMLElement (see CastEmitter __rt_truthy). */
final class MaybeEmptyBag
{
    public function __construct(private bool $full) {}

    public function __rt_truthy(): bool
    {
        return $this->full;
    }
}

function case_rt_truthy(): string
{
    $full = new MaybeEmptyBag(true);
    $empty = new MaybeEmptyBag(false);
    return ($full ? 'F' : '-') . ($empty ? 'E' : '-');
}

abstract class VNode {}
final class VVar extends VNode { /** @param string|VNode $name */ public function __construct(public $name) {} }
final class VIdent extends VNode { public function __construct(public string $name) {} }
final class VNop extends VNode {}

/** @param list<VNode> $nodes */
function read_names(array $nodes): string
{
    $out = '';
    foreach ($nodes as $n) {
        if ($n instanceof VNop) {
            $out .= '-';
            continue;
        }
        $name = $n->name;
        $out .= is_string($name) ? $name : '?';
    }
    return $out;
}

function case_variant_union_field(): string
{
    return read_names([new VVar('a'), new VIdent('b'), new VNop(), new VVar(new VIdent('c'))]);
}

final class ArgHolder
{
    public function __construct(public ArgBase $node) {}
}

function case_prop_no_downcast(): string
{
    $out = '';
    foreach ([new ArgLit(2), new ArgOther()] as $n) {
        $h = new ArgHolder($n);
        if ($h->node instanceof ArgLit) {
            $out .= takes_arg_base($h->node);
        } else {
            $out .= takes_arg_base($h->node);
        }
    }
    return $out;
}

/** @return list<ArgBase> */
function mk_narrow_bases(): array
{
    return [new ArgLit(5), new ArgOther()];
}

function case_narrow_only_receiver(): string
{
    $out = '';
    foreach (mk_narrow_bases() as $n) {
        if ($n instanceof ArgLit) {
            // a member only the subclass has: the receiver still downcasts
            $out .= (string) $n->v;
            // a plain value use keeps the stored class
            $out .= takes_arg_base($n);
        } else {
            $out .= takes_arg_base($n);
        }
    }
    return $out;
}

interface PlugBase {}
interface PlugEntry extends PlugBase { public function __invoke(string $arg): string; }
final class PlugOne implements PlugEntry { public function __invoke(string $arg): string { return 'one:' . $arg; } }
final class PlugPlain implements PlugBase {}

function run_plugin(PlugBase $p): string
{
    if (!$p instanceof PlugEntry) {
        return 'skip';
    }
    return $p('x');
}

function case_callee_narrowing(): string
{
    return run_plugin(new PlugOne()) . '|' . run_plugin(new PlugPlain());
}

function case_xml_config(): string
{
    $xml = new \SimpleXMLElement('<?xml version="1.0"?>
                <psalm>
                    <forbiddenFunctions>
                        <function name="print" />
                        <function name="var_export" />
                    </forbiddenFunctions>
                </psalm>');
    $out = isset($xml->forbiddenFunctions) ? 'set' : 'unset';
    $out .= isset($xml->forbiddenFunctions->function) ? '+fn' : '-fn';
    $names = '';
    foreach ($xml->forbiddenFunctions->function as $fn) {
        $names .= (string) $fn['name'] . ',';
    }
    return $out . ':' . $names;
}

function case_dom_config(): string
{
    $dom = new \DOMDocument();
    $dom->loadXML('<?xml version="1.0"?>
                <psalm>
                    <forbiddenFunctions>
                        <function name="eval" />
                        <function name="print" />
                    </forbiddenFunctions>
                </psalm>', LIBXML_NONET);
    $dom->xinclude(LIBXML_NOWARNING | LIBXML_NONET);
    $xml = \simplexml_import_dom($dom);
    if ($xml === null) {
        return 'null';
    }
    $out = isset($xml->forbiddenFunctions) ? 'set' : 'unset';
    $out .= isset($xml->forbiddenFunctions->function) ? '+fn' : '-fn';
    $names = '';
    foreach ($xml->forbiddenFunctions->function as $fn) {
        $names .= (string) $fn['name'] . ',';
    }
    return $out . ':' . $names;
}

function case_semver_constraints(): string
{
    $parser = new \Composer\Semver\VersionParser();
    $constraint = $parser->parseConstraints('^7.2.1|7.3,<8');
    $out = '';
    foreach (['5.4', '7.1', '7.2', '7.3', '8.0'] as $candidate) {
        $hit = $constraint->matches(new \Composer\Semver\Constraint\Constraint('<=', $candidate . '.0.0-dev'))
            || $constraint->matches(new \Composer\Semver\Constraint\Constraint('<=', $candidate . '.999'));
        $out .= $hit ? '1' : '0';
    }
    return $out;
}

// ---- feature: late static binding for class constants -------------------

abstract class Issue
{
    public const SHORTCODE = 0;
    public const LEVEL = -1;

    public function code(): int
    {
        return static::SHORTCODE;
    }

    public function level(): int
    {
        return static::LEVEL;
    }
}

final class UndefinedThing extends Issue
{
    public const SHORTCODE = 24;
}

final class MixedThing extends Issue
{
    public const SHORTCODE = 138;
    public const LEVEL = 1;
}

function case_static_const(): string
{
    $issues = [new UndefinedThing(), new MixedThing()];
    $out = '';
    foreach ($issues as $issue) {
        $out .= $issue->code() . ':' . $issue->level() . '|';
    }
    return $out;
}

// ---- feature: array_splice keeps the string keys it extracts -------------

function case_splice_keys(): string
{
    $dependencies = [];
    foreach (['foo\\bar', 'baz'] as $name) {
        $dependencies[$name] = true;
    }
    $taken = array_splice($dependencies, 0, 1);
    return (string) key($taken) . ':' . implode(',', array_keys($dependencies));
}

// ---- feature: indexing a union of a tuple and a list by a constant -------

final class TypeParams
{
    /** @var array{A, B}|array<never, never> */
    public array $params;

    /** @param array{A, B}|array<never, never> $params */
    public function __construct(array $params = [])
    {
        $this->params = isset($params[0], $params[1]) ? $params : [new A(9), new B(9)];
    }
}

function case_union_tuple_index(): string
{
    $pair = new TypeParams([new A(3), new B(4)]);
    $dflt = new TypeParams();

    // a variable offset, as `$input_type->type_params[$offset]` uses
    $set = 0;
    foreach ([0, 1, 2] as $offset) {
        if (isset($pair->params[$offset])) {
            $set++;
        }
    }

    return $pair->params[0]->a . ':' . $pair->params[1]->b . ':' . $dflt->params[0]->a . ':' . $set;
}

// ---- feature: a node's text written through nodeValue is serialized ------

function case_dom_node_value(): string
{
    $doc = new \DOMDocument('1.0', 'UTF-8');
    $doc->formatOutput = true;
    $root = $doc->createElement('report');
    $doc->appendChild($root);
    $item = $doc->createElement('failure');
    $item->setAttribute('type', 'X');
    $item->nodeValue = "line1\nline2";
    $root->appendChild($item);

    return trim($doc->saveXML()) . '|' . (string) $item->nodeValue;
}

abstract class Atom
{
}

final class ArrAtom extends Atom
{
    /** @var array{A, B} */
    public array $tp;

    public function __construct(A $a, B $b)
    {
        $this->tp = [$a, $b];
    }
}

final class ObjAtom extends Atom
{
    /** @var list<A> */
    public array $tp;

    /** @param list<A> $tp */
    public function __construct(array $tp)
    {
        $this->tp = $tp;
    }
}

/** The shape of `$input_type->type_params[$offset]`: the guard reads through the base enum. */
function atom_param(Atom $atom, int $offset): string
{
    if ($atom instanceof ArrAtom && isset($atom->tp[$offset])) {
        return 'arr';
    }
    if ($atom instanceof ObjAtom && isset($atom->tp[$offset])) {
        return 'obj';
    }
    return 'none';
}

function case_union_tuple_isset(): string
{
    return atom_param(new ArrAtom(new A(1), new B(2)), 1)
        . ':' . atom_param(new ObjAtom([new A(1)]), 0)
        . ':' . atom_param(new ObjAtom([]), 0)
        . ':' . atom_param(new ArrAtom(new A(1), new B(2)), 5);
}

// ---- feature: a numeric union converted, not narrowed, by an int operation ----

/** @param int|float $a */
function mod_of(int|float $a, int|float $b): int
{
    return $a % $b;
}

function case_union_modulo(): string
{
    return mod_of(25, 2) . ':' . mod_of(25.4, 2) . ':' . mod_of(25, 2.5) . ':' . mod_of(25.5, 2.5);
}

// ---- feature: a by-reference foreach follows removals, as PHP does ------

/** @return array<string, int> */
function ref_map(): array
{
    $m = [];
    foreach (['a', 'b', 'c'] as $i => $name) {
        $m[$name] = $i + 1;
    }

    return $m;
}

function case_foreach_ref_unset(): string
{
    $m = ref_map();

    foreach ($m as $k => &$v) {
        if ($k === 'b') {
            unset($m[$k]);
        }
        $v = $v * 10;
    }
    unset($v);

    // by key, not by iteration order: what this pins is the write-through, not the ordering
    return ($m['a'] ?? 0) . ':' . (isset($m['b']) ? 'b' : '-') . ':' . ($m['c'] ?? 0);
}

// ---- feature: two classes declaring the same union property type keep both members ----

function case_node_parts(): string
{
    $part = new \Tiny\Node\InterpolatedStringPart('a');
    $var = new \Tiny\Node\Expr\Variable('v');

    $shell = new \Tiny\Node\Expr\ShellExec([$part, $var]);
    $interp = new \Tiny\Node\Scalar\InterpolatedString([$part, $var]);

    $out = '';
    foreach ($shell->parts as $p) {
        $out .= $p instanceof \Tiny\Node\InterpolatedStringPart ? 'P' : 'E';
    }
    $out .= ':';
    foreach ($interp->parts as $p) {
        $out .= $p instanceof \Tiny\Node\InterpolatedStringPart ? 'P' : 'E';
    }

    return $out;
}

final class Graph
{
    public const ROOT = 'root';
    /** @var array<string, array<string, string>> */
    private array $edges = ['root' => ['a' => 'use', 'b' => 'use'], 'a' => ['c' => 'use'], 'b' => ['c' => 'write'], 'c' => ['d' => 'use']];

    /** @return array<string, true> */
    public function resolve(): array
    {
        $used = [self::ROOT => true];
        $queue = [self::ROOT];
        while ($queue) {
            $node = array_pop($queue);
            foreach ($this->edges[$node] ?? [] as $target => $type) {
                if ($type === 'write' || isset($used[$target])) {
                    continue;
                }
                $used[$target] = true;
                $queue[] = $target;
            }
        }
        return $used;
    }
}

function case_queue_pop(): string
{
    return implode(',', array_keys((new Graph())->resolve()));
}

final class UseGraph
{
    public const PUBLIC_API = 'public-api';
    public const EDGE_WRITE = 'write';
    public const EDGE_OVERRIDE = 'override';
    /** @var array<string, array<string, string>> */
    private array $forward_edges = [];
    /** @var array<string, true>|null */
    private ?array $used = null;

    public function addEdge(string $a, string $b, string $type): void
    {
        $this->forward_edges[$a][$b] = $type;
    }

    public static function classNode(string $fq_class_name_lc): string
    {
        return 'class:' . $fq_class_name_lc;
    }

    public static function getOwnerClass(string $node_id): ?string
    {
        $pos = strpos($node_id, '::');
        return $pos === false ? null : strtolower(substr($node_id, 0, $pos));
    }

    /** @param Closure(string): bool $is_external */
    private static function isRoot(string $node_id, Closure $is_external): bool
    {
        return str_starts_with($node_id, 'root:') || $is_external($node_id);
    }

    /** @param Closure(string): bool $is_external */
    public function resolve(Closure $is_external): void
    {
        $used = [self::PUBLIC_API => true];
        $queue = [self::PUBLIC_API];

        foreach ($this->forward_edges as $node_id => $_) {
            if (!isset($used[$node_id]) && self::isRoot($node_id, $is_external)) {
                $used[$node_id] = true;
                $queue[] = $node_id;
            }
        }

        $deferred = [];

        while ($queue) {
            $node_id = array_pop($queue);

            foreach ($this->forward_edges[$node_id] ?? [] as $target_node => $type) {
                if ($type === self::EDGE_WRITE || isset($used[$target_node])) {
                    continue;
                }

                if ($type === self::EDGE_OVERRIDE) {
                    $owner_class = self::getOwnerClass($target_node);

                    if ($owner_class !== null && !$is_external($node_id)) {
                        $owner_node = self::classNode($owner_class);

                        if (!isset($used[$owner_node])) {
                            $deferred[$owner_node][] = $target_node;
                            continue;
                        }
                    }
                }

                $used[$target_node] = true;
                $queue[] = $target_node;

                foreach ($deferred[$target_node] ?? [] as $deferred_node) {
                    if (!isset($used[$deferred_node])) {
                        $used[$deferred_node] = true;
                        $queue[] = $deferred_node;
                    }
                }

                unset($deferred[$target_node]);
            }
        }

        $this->used = $used;
    }

    /** @return list<string> */
    public function usedNodes(): array
    {
        return array_keys($this->used ?? []);
    }
}

function case_use_graph(): string
{
    $g = new UseGraph();
    $g->addEdge('public-api', 'a::m', 'use');
    $g->addEdge('a::m', 'b::n', 'override');
    $g->addEdge('a::m', 'c::p', 'write');
    $g->addEdge('root:x', 'class:b', 'use');
    $g->addEdge('class:b', 'd::q', 'use');
    $g->resolve(static fn(string $n): bool => $n === 'ext::e');
    return implode(',', $g->usedNodes());
}

/**
 * Psalm types an inline `$templates[$name]['psalm'] = ...` next to `$templates[$name][$source] = ...` as
 * `array{psalm: ..., none?: ...}` (psalm required for every entry), so all writes go through a declared type.
 *
 * @param array<string, array<string, array{string, ?string, ?string, bool}>> $templates
 * @param array{string, ?string, ?string, bool} $entry
 */
function template_add(array &$templates, string $name, string $source, array $entry): void
{
    $templates[$name][$source] = $entry;
}

/**
 * Mirrors FunctionLikeDocblockParser's template collection after master's purity templates: an optional block
 * writes `[name, 'of', bound, false]` entries, then each `@template` line writes `[name, modifier|null,
 * type|null, false]` into the same nested map, and the preferred source entry is picked per template.
 *
 * @param list<string> $purity
 * @param array<int, string> $lines
 * @param array<int, bool> $psalm_offsets
 */
function template_entries(array $purity, array $lines, array $psalm_offsets): string
{
    $templates = [];

    if ($purity !== []) {
        foreach ($purity as $name) {
            template_add($templates, $name, 'psalm', [$name, 'of', 'impure', false]);
        }
    }

    foreach ($lines as $offset => $line) {
        $parts = preg_split('/[\s]+/', $line);
        if ($parts === false) {
            return 'split-failed';
        }
        $template_name = array_shift($parts);
        if (!$template_name) {
            return 'empty';
        }
        $source = isset($psalm_offsets[$offset]) ? 'psalm' : 'none';
        if (count($parts) > 1 && in_array(strtolower($parts[0]), ['as', 'super', 'of'], true)) {
            $modifier = strtolower(array_shift($parts));
            template_add($templates, $template_name, $source, [$template_name, $modifier, implode(' ', $parts), false]);
        } else {
            template_add($templates, $template_name, $source, [$template_name, null, null, false]);
        }
    }

    $out = [];
    foreach ($templates as $entries) {
        foreach (['psalm', 'phpstan', 'none'] as $source) {
            if (isset($entries[$source])) {
                $out[] = $entries[$source];
                break;
            }
        }
    }

    $s = '';
    foreach ($out as $t) {
        $s .= $t[0] . ':' . ($t[1] ?? '-') . ':' . ($t[2] ?? '-') . ';';
    }
    return $s;
}

function case_template_entries(): string
{
    return template_entries([], [0 => 'TIn as object'], [])
        . '|' . template_entries(['P'], [0 => 'T', 1 => 'U of array<int>'], [1 => true]);
}

/**
 * A list narrowed to a one-element shape by count() is read as the list it is (no tuple rebuilt and turned
 * back into a list).
 *
 * @param list<string> $xs
 * @return list<string>
 */
function count1_identity(array $xs): array
{
    if (count($xs) === 1) {
        return $xs;
    }
    return array_reverse($xs);
}

/** @param list<int> $xs */
function count1_first(array $xs): string
{
    if (count($xs) === 1) {
        return 'one:' . $xs[0] . ':' . (string) current($xs);
    }
    return 'many:' . count($xs);
}

function case_count1_identity(): string
{
    return implode(',', count1_identity(['a'])) . '|' . implode(',', count1_identity(['a', 'b'])) . '|'
        . count1_first([7]) . '|' . count1_first([1, 2]);
}

/** A root whose variants store `$name` with different types (as php-parser's Expr does). */
abstract class P1Node
{
}

/** A non-leaf sub-hierarchy (like Variable with Psalm's VirtualVariable) storing `$name` alike. */
class P1Var extends P1Node
{
    public function __construct(public string|P1Node $name)
    {
    }
}

final class P1VirtualVar extends P1Var
{
}

final class P1Call extends P1Node
{
    public function __construct(public int $name)
    {
    }
}

function p1_read(P1Node $n): string
{
    if ($n instanceof P1Var && is_string($n->name)) {
        return 'v:' . $n->name;
    }
    if ($n instanceof P1Call) {
        return 'c:' . $n->name;
    }
    return '?';
}

function case_sub_enum_read(): string
{
    return p1_read(new P1Var('x')) . ',' . p1_read(new P1VirtualVar('y')) . ',' . p1_read(new P1Call(3)) . ','
        . p1_read(new P1Var(new P1Call(1)));
}

/** A non-leaf concrete class (like php-parser's FuncCall with Psalm's VirtualFuncCall below it). */
class P6Call
{
    /** @param list<int> $args */
    public function __construct(public array $args)
    {
    }

    /** @return list<int> */
    public function getArgs(): array
    {
        return $this->args;
    }

    public function kind(): string
    {
        return 'call';
    }
}

final class P6VirtualCall extends P6Call
{
    public function kind(): string
    {
        return 'virtual';
    }
}

function case_own_getter(): string
{
    $a = new P6Call([1, 2]);
    $b = new P6VirtualCall([3]);
    $c = $a instanceof P6VirtualCall ? $a : $b;
    return implode(',', $a->getArgs()) . '|' . $a->kind() . '|' . implode(',', $b->getArgs()) . '|' . $b->kind()
        . '|' . $c->kind();
}

/** A static map read under `??`/isset (borrowed, not cloned out of its cell), with a key that writes it. */
final class StaticMapRead
{
    /** @var array<string, string> */
    private static array $m = [];

    public static function fill(string $k): string
    {
        self::$m[$k . '!'] = 'filled';
        return $k . '!';
    }

    public static function get(string $k): string
    {
        return self::$m[$k] ?? 'none';
    }

    public static function has(string $k): bool
    {
        return isset(self::$m[$k]);
    }

    public static function reentrant(string $k): string
    {
        return self::$m[self::fill($k)] ?? 'none';
    }
}

function case_static_map_read(): string
{
    $a = StaticMapRead::get('x');
    $b = StaticMapRead::has('x') ? 'y' : 'n';
    $c = StaticMapRead::reentrant('x');
    $d = StaticMapRead::get('x!');
    $e = StaticMapRead::has('x!') ? 'y' : 'n';
    return $a . ',' . $b . ',' . $c . ',' . $d . ',' . $e;
}

final class MapSortHolder
{
    /** @var array<int, string> */
    public array $names = [];
}

/** @param array<int, string|int> $m */
function map_sort_dump(array $m): string
{
    $out = '';
    foreach ($m as $k => $v) {
        $out .= $k . '=' . $v . ' ';
    }
    return $out;
}

/** sort()/rsort()/usort() on int-keyed maps (holes, non-sequential keys) sort in place and renumber; stable. */
function case_map_sort(): string
{
    /** @var array<int, string> $m */
    $m = [5 => 'c', 2 => 'a', 9 => 'b'];
    unset($m[2]);
    $m[] = 'a';
    sort($m);
    $out = map_sort_dump($m);
    /** @var array<int, int> $r */
    $r = [3 => 1, 1 => 3, 7 => 2];
    rsort($r);
    $out .= '|' . map_sort_dump($r);
    /** @var array<int, string> $u */
    $u = [4 => 'bb', 8 => 'a', 6 => 'cc', 2 => 'd'];
    usort($u, static fn(string $x, string $y): int => strlen($x) <=> strlen($y));
    $out .= '|' . map_sort_dump($u);
    $h = new MapSortHolder();
    $h->names[10] = 'z';
    $h->names[3] = 'y';
    sort($h->names);
    $out .= '|' . map_sort_dump($h->names);
    $m[] = 'n';
    return $out . '|' . map_sort_dump($m);
}

final class ListIdHolder
{
    /** @param list<string> $items */
    public function __construct(public array $items)
    {
    }

    /** @param array<int, string> $items */
    public function same(array $items): bool
    {
        return $items === $this->items;
    }

    /** @param array<array-key, string> $items */
    public function sameKeyed(array $items): bool
    {
        return $this->items === $items;
    }
}

/** `===` between a list and an int-keyed map: keys 0..n in order and identical values, no conversion. */
function case_list_map_identical(): string
{
    $h = new ListIdHolder(['a', 'b']);
    /** @var array<int, string> $holes */
    $holes = [0 => 'a', 2 => 'b'];
    /** @var array<int, string> $swapped */
    $swapped = [1 => 'b', 0 => 'a'];
    /** @var array<array-key, string> $named */
    $named = ['x' => 'a', 'y' => 'b'];
    /** @var array<array-key, string> $num */
    $num = [0 => 'a', 1 => 'b'];
    return ($h->same(['a', 'b']) ? 'y' : 'n') . ($h->same(['a', 'c']) ? 'y' : 'n') . ($h->same($holes) ? 'y' : 'n')
        . ($h->same($swapped) ? 'y' : 'n') . ($h->same(['a']) ? 'y' : 'n') . ($h->sameKeyed($named) ? 'y' : 'n')
        . ($h->sameKeyed($num) ? 'y' : 'n') . ($h->sameKeyed([]) ? 'y' : 'n');
}

function run_all(): string
{
    return check('list_map_identical', case_list_map_identical(), 'ynnnnnyn')
        . check('map_sort', case_map_sort(), '0=a 1=b 2=c |0=3 1=2 2=1 |0=a 1=d 2=bb 3=cc |0=y 1=z |0=a 1=b 2=c 3=n ')
        . check('option_instanceof', case_option_instanceof(), 'sc3s-')
        . check('static_map_read', case_static_map_read(), 'none,n,filled,filled,y')
        . check('own_getter', case_own_getter(), '1,2|call|3|virtual|virtual')
        . check('sub_enum_read', case_sub_enum_read(), 'v:x,v:y,c:3,?')
        . check('count1_identity', case_count1_identity(), 'a|b,a|one:7:7|many:2')
        . check('template_entries', case_template_entries(), 'TIn:as:object;|P:of:impure;T:-:-;U:of:array<int>;')
        . check('cond_return', case_cond_return(), 'fallback:n7:n7')
        . check('use_graph', case_use_graph(), 'public-api,root:x,class:b,d::q,a::m,b::n')
        . check('queue_pop', case_queue_pop(), 'root,a,b,c,d')
        . check('const_table', case_const_table(), 'inf,nan,null,30719,eol,absent,10')
        . check('object_union_narrowing', case_object_union_narrowing(), 'L,2L,str,7,n,')
        . check('coalesce_assign_var_key', case_coalesce_assign_var_key(), '001:a,b:a')
        . check('id_keyed_map', case_id_keyed_map(), 'yn:foo,bar:Bar:foo=Foo:bar=Bar:B2A2B2:hr')
        . check('kind_dispatch', case_kind_dispatch(), 'AaBbAa-cTiny\\KindC')
        . check('node_parts', case_node_parts(), 'PE:PE')
        . check('generic_empty_return', case_generic_empty_return(), '0:2')
        . check('guard_key_writes', case_guard_key_writes(), 'ab4')
        . check('shape_dyn_key', case_shape_dyn_key(), 'int,-,zero,string,')
        . check('foreach_ref_unset', case_foreach_ref_unset(), '10:-:30')
        . check('union_modulo', case_union_modulo(), '1:1:1:1')
        . check('union_tuple_isset', case_union_tuple_isset(), 'arr:obj:none:none')
        . check('dom_node_value', case_dom_node_value(), "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<report>\n  <failure type=\"X\">line1\nline2</failure>\n</report>|line1\nline2")
        . check('union_tuple_index', case_union_tuple_index(), '3:4:9:2')
        . check('static_const', case_static_const(), '24:-1|138:1|')
        . check('splice_keys', case_splice_keys(), 'foo\\bar:baz')
        . check('dom_config', case_dom_config(), 'set+fn:eval,print,')
        . check('semver_constraints', case_semver_constraints(), '00111')
        . check('xml_config', case_xml_config(), 'set+fn:print,var_export,')
        . check('callee_narrowing', case_callee_narrowing(), 'one:x|skip')
        . check('narrow_only_receiver', case_narrow_only_receiver(), '5litother')
        . check('prop_no_downcast', case_prop_no_downcast(), 'litother')
        . check('variant_union_field', case_variant_union_field(), 'ab-?')
        . check('rt_truthy', case_rt_truthy(), 'F-')
        . check('prop_empty_unset', case_prop_empty_unset(), '1:0')
        . check('variant_isset', case_variant_isset(), '4:1')
        . check('arg_no_downcast', case_arg_no_downcast(), 'litother')
        . check('prop_empty_merge', case_prop_empty_merge(), '1:1')
        . check('version_regex', case_version_regex(), 'ab-')
        . check('union_receiver_base_method', case_union_receiver_base_method(), 'xa|xb|other:base')
        . check('union_prop_coalesce', case_union_prop_coalesce(), 'i:A|node|i:E')
        . check('union_tostring', case_union_tostring(), 'isa-named:X|isa-tmpl')
        . check('prop_empty_narrow', case_prop_empty_narrow(), '1:1')
        . check('sym_str_funcs', case_sym_str_funcs(), 'XBox|YBox')
        . check('int_or_false_arith', case_int_or_false_arith(), '4')
        . check('key_narrow', case_key_narrow(), '3F7')
        . check('nullable_list_prop', case_nullable_list_prop(), '2:1')
        . check('union_base_member', case_union_base_member(), 'name:a|node|nullable|')
        . check('invoke_object', case_invoke_object(), 'sock3')
        . check('declared_preg', case_declared_preg(), 'stdclass+')
        . check('nullable_union', case_nullable_union(), 'nullA7')
        . check('wildcard_const', case_wildcard_const(), 'set')
        . check('value_of', case_value_of(), 'HS')
        . check('return_move', case_return_move(), '534')
        . check('single_use_move', case_single_use_move(), '766')
        . check('foreach_collection_move', case_foreach_collection_move(), '60')
        . check('alias_mutation', case_alias_mutation(), '7')
        . check('narrow_downcast', case_narrow_downcast(), 'arr1arr1keyed3')
        . check('loops', case_loops(), '63')
        . check('control_flow', case_control_flow(), '10|8|5|two')
        . check('receiver_move', case_receiver_move(), '11')
        . check('borrow_param', case_borrow_param(), '42425')
        . check('private_method_borrow', case_private_method_borrow(), '210')
        . check('private_static_borrow', case_private_static_borrow(), '32')
        . check('leaf_public_borrow', case_leaf_public_borrow(), '24')
        . check('dispatch_agree_borrow', case_dispatch_agree_borrow(), '26')
        . check('dispatch_disagree_owned', case_dispatch_disagree_owned(), '34')
        . check('callable_borrow', case_callable_borrow(), '11')
        . check('late_receiver_borrow', case_late_receiver_borrow(), '00')
        . check('closure_capture', case_closure_capture(), '8')
        . check('empty_arrays', case_empty_arrays(), 's0,s1|0|2')
        . check('try_catch', case_try_catch(), '2,f,caught:big 3,f,outer,d,unhandled')
        . check('generics', case_generics(), 'i:eq,s:ne,l:eq,42,xy,has,no,5,strintother')
        . check('phpunit', case_phpunit(), '1,failed,skip:later,expects,verified,mismatch,testThrows with data set "ds"')
        . check('object_eq', case_object_eq(), 'eq,ne,ne,notsame,eq,eq,ne')
        . check('defined_constants', case_defined_constants(), 'max9223372036854775807,eall,eol,pi')
        . check('defined_fold', case_defined_fold(), 'rt=1,cls=1,nocls=0,unknown=0,dyn=1')
        . check('memo_immutable', case_memo_immutable(), 'v7:v7:1|v7:1|n:1')
        . check('memo_trait_immutable', case_memo_trait_immutable(), 'k3:k3:1')
        . check('variant_field_read', case_variant_field_read(), '7|x|-')
        . check('nested_receiver_narrowing', case_nested_receiver_narrowing(), '7:x')
        . check('value_hierarchy', case_value_hierarchy(), 'i1,s:a,l2,s:q!|i:1|9|k=i1:s:a:l2:s:q!')
        . check('value_containers', case_value_containers(), 'in:1,0|eq:1,0|u:i1,text|o:s:z,none|ref:i4')
        . check('identity_cycle', case_identity_cycle(), 'cycle@3:a')
        . check('value_factory', case_value_factory(), 'TF3:T')
        . check('value_named', case_value_named(), 'A&B|A&B&C|self->Foo|k:A&B|static:1')
        . check('value_nonleaf', case_value_nonleaf(), 'G<int>|Foo<int>|Foo&B<int>|1|self->Foo')
        . check('borrow_local', case_borrow_local(), '6:3:a,b,c:x')
        . check('borrow_return', case_borrow_return(), '2:9:3:2:1:1:v:null')
        . check('ptr_reads', case_ptr_reads(), '4:5:A:a:4:9:4:v:x')
        . check('data_file', case_data_file(), 'a1x-,b2y3.5|a,b')
        . check('elseif_assign', case_elseif_assign(), 'none,a5,skip|1')
        . check('json_encode', case_json_encode(), '{"a":1,"b":[1,2,3],"c":null,"d":true,"e":1.5,"f":"x\\"y"}|[{"x":1,"label":null},{"x":2,"label":"p"}]|{"a":1,"b":"two","c":[3,4]}|{"3":"a","5":"b"}|[]|' . "{\n    \"k\": [\n        1,\n        \"z\"\n    ]\n}")
        . check('dom_iteration', case_dom_iteration(), 'Issue,x')
        . check('json_decode', case_json_decode(), '1|1+2|k,k2|-|noe|x|badnull')
        . check('simplexml_iteration', case_simplexml_iteration(), 'a,b,x')
        . check('nullable_prop_receiver', case_nullable_prop_receiver(), 'p9,q9,0:1')
        . check('generic_callable', case_generic_callable(), '42|a,b|run,done,run,done')
        . check('typed_builtins', case_typed_builtins(), '1:ab@2|x|no|0|1,-1|user:0')
        . check('deferred_graph', case_deferred_graph(['root', 'root', 'n1']), 'n0,n1,n2')
        . check('typed_builtins2', case_typed_builtins2(), "c1,r6,'a\\'b',42,nf,yes,null,1.5,pos")
        . check('elseif_narrowing', case_elseif_narrowing(), 'p,n,-')
        . check('filter_table', case_filter_table(), '257:1,2,9|min=1,d=0;259:1,9|d=0;516:3|d=0')
        . check('element_retype', case_element_retype(), 'A=1x,B=2y|ab')
        . check('narrow_reassign', case_narrow_reassign(), 'p,ri,ri,-,rr')
        . check('closure_param', case_closure_param(), 'a:1|none|x:1')
        . check('generic_binding', case_generic_binding(), 'ok')
        . check('destructure_null', case_destructure_null(), 'a=1,b=x|a=,b=|a=2,b=y')
        . check('assert_if_false_key', case_assert_if_false_key(), 's:Foo|i:15')
        . check('substr_count_window', case_substr_count_window(), '5|1|2|3|0|2')
        . check('static_false_compare', case_static_false_compare(), 'called:1|caught')
        . check('coalesce_nullable', case_coalesce_nullable(), 'none|a')
        . check('static_via_object', case_static_via_object(), 'no|yes|7')
        . check('byref_override_arg', case_byref_override_arg(), 'a:stop/no|b:go/yes')
        . check('php_shifts', case_php_shifts(), '0|1|0|-1|2|0|4611686018427387904')
        . check('builtin_arity', case_builtin_arity(), 'ok')
        . check('sprintf_percent', case_sprintf_percent(), '%|%|%|%|%a|%b|a%c')
        . check('dynamic_new_dead', case_dynamic_new_dead(), 'named')
        . check('method_exists', case_method_exists(), 'yn')
        . check('hook_param_narrowing', case_hook_param_narrowing(), 'rich+|plain|plain')
        . check('class_string_or_object', case_class_string_or_object(), 'name:Tiny\\HookA|obj:A|name:Other')
        . check('dir_const', case_dir_const(), 'php/fixtures')
        . check('array_map_void', case_array_map_void(), 'A,B,C')
        . check('provided_builtins', case_provided_builtins(), 'sort:1|in_array:1|usort:1|file_exists:1|go:0|nonesuch_xyz:0')
        . check('htmlspecialchars_flags', case_htmlspecialchars_flags(), 'a&quot;b&#039;c&lt;&amp;&gt;|a&quot;b&apos;c&lt;&amp;&gt;|a&quot;b&apos;c&lt;&amp;&gt;|a&quot;b\'c&lt;&amp;&gt;|a"b\'c&lt;&amp;&gt;')
        . check('round_modes', case_round_modes(), '3,2,2,3,3,2,2,3,4,3,4,3,4,3,3,4,-3,-2,-2,-3,-2,-3,-2,-3,1.5,1.4,1.4,1.5,1.5,1.4,1.4,1.5,1.6,1.5,1.6,1.5,1.6,1.5,1.5,1.6,2,2,2,2,3,2,2,3,-2,-2,-2,-2,-2,-3,-2,-3,1,1,1,1,1,1,1,1,-2,-1,-2,-1,-1,-2,-1,-2')
        . check('array_to_xml', case_array_to_xml(), "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<report>\n  <item/>\n</report>\n|<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<report>\n  <item>\n    <severity>error</severity>\n    <line_from>4</line_from>\n    <taint_trace/>\n    <refs>\n      <label>a &amp; b</label>\n    </refs>\n    <refs>\n      <label>c</label>\n    </refs>\n  </item>\n</report>\n");
}

// ---- feature: a constant table read from a dictionary in its declared type (psalm-port's ConstantMap) ----

final class ConstTable
{
    /** @var array<string, scalar|null>|null */
    private static ?array $map = null;

    /** @return array<string, scalar|null> */
    public static function get(): array
    {
        if (self::$map === null) {
            self::$map = require __DIR__ . '/data/consts.php';
        }
        return self::$map;
    }
}

final class FoldConsts
{
    public const HERE = 1;
}

/** @psalm-pure */
function fold_name(): string
{
    return 'PHP_EOL';
}

/**
 * @psalm-immutable
 */
final class MemoBox
{
    private ?string $id = null;

    private bool $checked = false;

    public function __construct(public int $v, public readonly ?MemoBox $next = null)
    {
    }

    public function id(): string
    {
        if ($this->id === null) {
            /** @psalm-suppress ImpurePropertyAssignment memo */
            $this->id = 'v' . $this->v;
        }
        return $this->id;
    }

    /** @psalm-external-mutation-free */
    public function markChecked(): void
    {
        /** @psalm-suppress ImpurePropertyAssignment memo */
        $this->checked = true;
    }

    public function isChecked(): bool
    {
        return $this->checked;
    }

    public function withV(int $v): self
    {
        // a clone copies the memos too (PHP semantics)
        $c = clone $this;
        /** @psalm-suppress ImpurePropertyAssignment wither */
        $c->v = $v;
        return $c;
    }
}

interface TraitBoxNode
{
    public function key(): string;
}

trait MemoBoxTrait
{
    private ?string $key = null;

    private bool $checked = false;

    public function __construct(public int $k)
    {
        $this->checked = false;
        $this->key = null;
    }

    public function key(): string
    {
        if ($this->key === null) {
            /** @psalm-suppress ImpurePropertyAssignment memo */
            $this->key = 'k' . $this->k;
        }
        return $this->key;
    }

    /** @psalm-external-mutation-free */
    public function markChecked(): void
    {
        /** @psalm-suppress ImpurePropertyAssignment memo */
        $this->checked = true;
    }

    public function isChecked(): bool
    {
        return $this->checked;
    }
}

/**
 * @psalm-immutable
 */
final class TraitMemoBox implements TraitBoxNode
{
    use MemoBoxTrait;

    /** @param array<string, mixed> $properties */
    public function __unserialize(array $properties): void
    {
        foreach ($properties as $name => $value) {
            $this->$name = $value;
        }
    }
}

function case_memo_trait_immutable(): string
{
    $a = new TraitMemoBox(3);
    $first = $a->key();
    $a->markChecked();
    return $first . ':' . $a->key() . ':' . (int) $a->isChecked();
}

abstract class VfBase
{
}

class VfMid extends VfBase
{
    public function __construct(public int $name)
    {
    }
}

final class VfMidChild extends VfMid
{
}

final class VfOther extends VfBase
{
    public function __construct(public string $name)
    {
    }
}

final class VfNone extends VfBase
{
}

/** @return list<VfBase> */
function vf_values(): array
{
    return [new VfMidChild(7), new VfOther('x'), new VfNone()];
}

final class VfHolder
{
    /** @param list<VfBase> $items */
    public function __construct(public array $items)
    {
    }

    /** @return list<VfBase> */
    public function items(): array
    {
        return $this->items;
    }
}

abstract class VfBox
{
}

final class VfBoxA extends VfBox
{
    public function __construct(public VfHolder $holder)
    {
    }

    public function holder(): VfHolder
    {
        return $this->holder;
    }
}

final class VfBoxB extends VfBox
{
}

/** @return list<VfBox> */
function vf_boxes(): array
{
    return [new VfBoxA(new VfHolder(vf_values())), new VfBoxB()];
}

function case_nested_receiver_narrowing(): string
{
    $out = [];
    foreach (vf_boxes() as $box) {
        if ($box instanceof VfBoxA) {
            // the property fetch's receiver is a method call on a narrowed receiver: only the outer
            // receiver may stay un-narrowed, the inner one must still resolve holder() on VfBoxA
            $first = $box->holder()->items()[0];
            if ($first instanceof VfMid) {
                $out[] = (string) $first->name;
            }
            $second = $box->holder->items[1];
            if ($second instanceof VfOther) {
                $out[] = $second->name;
            }
        }
    }
    return implode(':', $out);
}

function case_variant_field_read(): string
{
    $out = [];
    foreach (vf_values() as $v) {
        if ($v instanceof VfMid) {
            $out[] = (string) $v->name;
        } elseif ($v instanceof VfOther) {
            $out[] = $v->name;
        } else {
            $out[] = '-';
        }
    }
    return implode('|', $out);
}

/**
 * @psalm-immutable
 */
abstract class VAtom
{
    private ?string $key_memo = null;

    abstract protected function computeKey(): string;

    public function getKey(): string
    {
        if ($this->key_memo === null) {
            /** @psalm-suppress ImpurePropertyAssignment memo */
            $this->key_memo = $this->computeKey();
        }
        return $this->key_memo;
    }
}

/**
 * @psalm-immutable
 */
final class VInt extends VAtom
{
    public function __construct(public int $v)
    {
    }

    protected function computeKey(): string
    {
        return 'i' . $this->v;
    }

    public function withV(int $v): self
    {
        $c = clone $this;
        /** @psalm-suppress ImpurePropertyAssignment wither */
        $c->v = $v;
        return $c;
    }
}

/**
 * @psalm-immutable
 */
class VStr extends VAtom
{
    public function __construct(public string $s)
    {
    }

    protected function computeKey(): string
    {
        return 's:' . $this->s;
    }
}

/**
 * A concrete non-leaf member's subclass: the hierarchy takes the same immutability path as Type\Atomic.
 *
 * @psalm-immutable
 */
final class VLitStr extends VStr
{
    protected function computeKey(): string
    {
        return 's:' . $this->s . '!';
    }
}

/**
 * @psalm-immutable
 */
final class VList extends VAtom
{
    /** @param list<VAtom> $items */
    public function __construct(public array $items)
    {
    }

    protected function computeKey(): string
    {
        return 'l' . count($this->items);
    }
}

/** @return list<VAtom> */
function v_atoms(): array
{
    return [new VInt(1), new VStr('a'), new VList([new VInt(2), new VStr('b')]), new VLitStr('q')];
}

final class VHolder
{
    public function __construct(
        public VAtom|string $either,
        public ?VAtom $maybe = null,
    ) {
    }
}

function v_bump(VAtom &$a): void
{
    if ($a instanceof VInt) {
        $a = $a->withV($a->v + 3);
    }
}

function case_value_containers(): string
{
    $list = v_atoms();
    $first = $list[0];
    // strict membership of an element taken from the list (true), and of an atom no element equals (false)
    $in = (int) in_array($first, $list, true) . ',' . (int) in_array(new VInt(5), $list, true);
    // loose equality compares fields
    $eq = (int) (new VInt(1) == new VInt(1)) . ',' . (int) (new VInt(1) == new VInt(2));
    $h1 = new VHolder(new VInt(1), new VStr('z'));
    $h2 = new VHolder('text');
    $u = ($h1->either instanceof VAtom ? $h1->either->getKey() : 'str') . ',' . (is_string($h2->either) ? $h2->either : 'atom');
    $o = ($h1->maybe !== null ? $h1->maybe->getKey() : 'none') . ',' . ($h2->maybe !== null ? $h2->maybe->getKey() : 'none');
    $r = new VInt(1);
    v_bump($r);
    return 'in:' . $in . '|eq:' . $eq . '|u:' . $u . '|o:' . $o . '|ref:' . $r->getKey();
}

function case_value_hierarchy(): string
{
    $keys = [];
    $ints = [];
    $bumped = '';
    foreach (v_atoms() as $a) {
        $keys[] = $a->getKey();
        if ($a instanceof VInt) {
            $ints[] = 'i:' . $a->v;
            // a clone copies the key memo too (PHP): read the new field, not the key
            $bumped = (string) $a->withV($a->v + 8)->v;
        } elseif ($a instanceof VList) {
            foreach ($a->items as $inner) {
                if ($inner instanceof VStr && $inner->s === 'zzz') {
                    $ints[] = 'never';
                }
            }
        }
    }
    $same = v_atoms();
    $byKey = [];
    foreach ($same as $a) {
        $byKey[$a->getKey()] = $a;
    }
    $collected = implode(':', array_keys($byKey));
    return implode(',', $keys) . '|' . implode(',', $ints) . '|' . $bumped . '|k=' . $collected;
}

function case_memo_immutable(): string
{
    $a = new MemoBox(7);
    $first = $a->id();
    $a->markChecked();
    $b = $a->withV(8);
    $wrapped = new MemoBox(1, $a);
    return $first . ':' . $a->id() . ':' . (int) $a->isChecked()
        . '|' . $b->id() . ':' . (int) $b->isChecked()
        . '|n:' . (int) ($wrapped->next !== null && $wrapped->next->isChecked());
}

function case_defined_fold(): string
{
    $dyn = fold_name();
    return 'rt=' . (int) \defined('PSALM_COMPILED')
        . ',cls=' . (int) \defined('Tiny\\FoldConsts::HERE')
        . ',nocls=' . (int) \defined('Tiny\\FoldConsts::NOPE')
        . ',unknown=' . (int) \defined('SURELY_NOT_A_CONSTANT')
        . ',dyn=' . (int) \defined($dyn);
}

function case_defined_constants(): string
{
    $constants = ConstTable::get();
    $out = [];
    $out[] = isset($constants['PHP_INT_MAX']) ? 'max' . (string) $constants['PHP_INT_MAX'] : 'nomax';
    $out[] = isset($constants['E_ALL']) && is_int($constants['E_ALL']) ? 'eall' : 'noeall';
    $out[] = ($constants['PHP_EOL'] ?? '') === "\n" ? 'eol' : 'noeol';
    $out[] = array_key_exists('M_PI', $constants) && is_float($constants['M_PI']) ? 'pi' : 'nopi';
    return implode(',', $out);
}

// ---- feature: typed data files (require of a dictionary in the declared type, no Mixed) ----

function case_data_file(): string
{
    $table = require __DIR__ . '/data/table.php';
    $out = [];
    foreach ($table as $name => [$n, $s, $f]) {
        $out[] = $name . $n . $s . ($f === null ? '-' : (string) $f);
    }
    $names = array_keys($table);
    return implode(',', $out) . '|' . implode(',', $names);
}

// ---- probe: elseif-assigned locals and property array_filter (no Mixed locals expected) ----

final class TypeSource
{
    /** @var array<string, A> */
    private array $types = [];

    /** @var array<string, bool> */
    private array $flags = ['x' => true, 'y' => false];

    public function __construct()
    {
        $this->types['b'] = new A(5);
    }

    public function getType(string $key): ?A
    {
        return $this->types[$key] ?? null;
    }

    public function forgetFlags(): void
    {
        $this->flags = array_filter($this->flags);
    }

    public function flagCount(): int
    {
        return count($this->flags);
    }
}

function case_elseif_assign(): string
{
    $src = new TypeSource();
    $out = [];
    foreach (['a', 'b', 'c'] as $key) {
        if ($key === 'c') {
            $out[] = 'skip';
        } elseif ($stmt_type = $src->getType($key)) {
            $out[] = 'a' . $stmt_type->a;
        } else {
            $out[] = 'none';
        }
    }
    $src->forgetFlags();
    return implode(',', $out) . '|' . $src->flagCount();
}

// ---- feature: typed json_encode (unions, shapes, lists, maps, objects; no Mixed) ----

final class JsonPoint implements \JsonSerializable
{
    public function __construct(public int $x, public ?string $label = null) {}

    /** @return array{x: int, label: ?string} */
    public function jsonSerialize(): array
    {
        return ['x' => $this->x, 'label' => $this->label];
    }
}

final class JsonPlain
{
    public int $a = 1;
    public string $b = 'two';
    private int $hidden = 3;
    /** @var list<int> */
    public array $c = [3, 4];
}

function case_json_encode(): string
{
    $out = [];
    $out[] = json_encode(['a' => 1, 'b' => [1, 2, 3], 'c' => null, 'd' => true, 'e' => 1.5, 'f' => 'x"y']);
    $out[] = json_encode([new JsonPoint(1), new JsonPoint(2, 'p')]);
    $out[] = json_encode(new JsonPlain());
    $mixed_keys = [3 => 'a', 5 => 'b'];
    $out[] = json_encode($mixed_keys);
    $out[] = json_encode([]);
    $out[] = json_encode(['k' => [1, 'z']], JSON_PRETTY_PRINT);
    return implode('|', $out);
}

// ---- probe: DOM iteration types (childNodes, getElementsByTagName) ----

function case_dom_iteration(): string
{
    $doc = new \DOMDocument();
    $doc->loadXML('<files><file src="a.php"><Issue><code>x</code></Issue></file></files>');
    $out = [];
    foreach ($doc->getElementsByTagName('file') as $file) {
        foreach ($file->childNodes as $issue) {
            if (!$issue instanceof \DOMElement) {
                continue;
            }
            $out[] = $issue->tagName;
            foreach ($issue->getElementsByTagName('code') as $code) {
                $out[] = $code->textContent;
            }
        }
    }
    return implode(',', $out);
}

// ---- feature: typed json_decode (a declared return type reads the document; no Mixed) ----

/** @return array{a: int, b: list<int>, c: array<string, string>, d: ?string, e?: bool, f: float|string}|null */
function decode_doc(string $json): ?array
{
    return json_decode($json, true);
}

function case_json_decode(): string
{
    $doc = decode_doc('{"a":1,"b":[1,2],"c":{"k":"v","k2":"w"},"d":null,"f":"x"}');
    if ($doc === null) {
        return 'null';
    }
    $out = [(string) $doc['a'], implode('+', $doc['b']), implode(',', array_keys($doc['c'])), $doc['d'] ?? '-', isset($doc['e']) ? 'e' : 'noe', is_string($doc['f']) ? $doc['f'] : 'num'];
    $bad = decode_doc('nope');
    $out[] = $bad === null ? 'badnull' : 'bad';
    return implode('|', $out);
}

// ---- probe: SimpleXML magic property iteration types (config.xml style) ----

function case_simplexml_iteration(): string
{
    $xml = new \SimpleXMLElement('<psalm cacheDirectory="x"><enableExtensions><extension name="a"/><extension name="b"/></enableExtensions></psalm>');
    $out = [];
    foreach ($xml->enableExtensions->extension as $extension) {
        $out[] = (string) $extension['name'];
    }
    if (isset($xml['cacheDirectory'])) {
        $out[] = (string) $xml['cacheDirectory'];
    }
    return implode(',', $out);
}

// ---- probe: narrowed nullable property used as a call receiver (no Mixed) ----

final class WithParent
{
    public ?A $parent = null;
}

function case_nullable_prop_receiver(): string
{
    $w = new WithParent();
    $w->parent = new A(9);
    $out = [];
    if ($w->parent) {
        $out[] = 'p' . $w->parent->a;
    }
    if ($w->parent !== null) {
        $out[] = 'q' . $w->parent->a;
    }
    exec('true', $lines, $status);
    $out[] = count($lines) . ':' . $status;
    return implode(',', $out);
}

// ---- feature: fn-level generics on final-class methods (callable(): T) ----

final class Collector
{
    /** @var list<string> */
    public array $seen = [];

    /**
     * @template T
     * @param callable(): T $f
     * @return T
     */
    public function runAndCollect(callable $f)
    {
        $this->seen[] = 'run';
        $ret = $f();
        $this->seen[] = 'done';
        return $ret;
    }
}

function case_generic_callable(): string
{
    $c = new Collector();
    $n = $c->runAndCollect(static fn(): int => 41 + 1);
    $list = $c->runAndCollect(/** @return list<string> */ static fn(): array => ['a', 'b']);
    return $n . '|' . implode(',', $list) . '|' . implode(',', $c->seen);
}


/** @return list<never> */
function never_list(): array
{
    return [];
}

function case_typed_builtins(): string
{
    $out = '';
    if (preg_match('/(a)(b)/', 'xxab', $m, PREG_OFFSET_CAPTURE) === 1) {
        $out .= '1:' . $m[0][0] . '@' . $m[0][1];
    }
    if (preg_match('/(x)(y)?/', 'x', $m2, PREG_UNMATCHED_AS_NULL) === 1) {
        $out .= '|' . $m2[1] . ($m2[2] === null ? '' : 'Y');
    }
    $empty = never_list();
    $out .= '|' . (in_array('a', $empty, true) ? 'yes' : 'no');
    $count = 0;
    foreach ($empty as $_v) {
        $count++;
    }
    $out .= '|' . $count;
    if (preg_match('/(a)(z)?/', 'a', $m3, PREG_OFFSET_CAPTURE | PREG_UNMATCHED_AS_NULL) === 1) {
        $out .= '|' . ($m3[1][0] === null ? 'N' : '1') . ',' . $m3[2][1];
    }
    $fns = get_defined_functions();
    $out .= '|user:' . count(array_filter($fns['user'], static fn(string $n): bool => $n === 'no_such_fn'));
    return $out;
}

/** @param list<string> $items */
function case_deferred_graph(array $items): string
{
    $deferred = [];
    $used = [];
    $queue = ['root'];
    foreach ($items as $i => $item) {
        $deferred[$item][] = 'n' . $i;
    }
    while ($queue) {
        $node = array_pop($queue);
        if (isset($deferred[$node])) {
            foreach ($deferred[$node] as $d) {
                if (!isset($used[$d])) {
                    $used[$d] = true;
                    $queue[] = $d;
                }
            }
            unset($deferred[$node]);
        }
    }
    return implode(',', array_keys($used));
}

abstract class Shape2
{
}

final class Circle2 extends Shape2
{
    public function __construct(public float $r)
    {
    }
}

final class Rect2 extends Shape2
{
    public function __construct(public float $w, public float $h)
    {
    }

    public function area(): float
    {
        return $this->w * $this->h;
    }
}

/** @return list<Shape2> */
function shapes2(): array
{
    return [new Circle2(1.0), new Rect2(2.0, 3.0)];
}

function case_typed_builtins2(): string
{
    $out = [];
    foreach (shapes2() as $s) {
        if ($s instanceof Circle2) {
            $out[] = 'c' . $s->r;
        } elseif ($s instanceof Rect2) {
            $out[] = 'r' . $s->area();
        }
    }
    $out[] = var_export('a\'b', true);
    $out[] = (string) (filter_var('42', FILTER_VALIDATE_INT) ?: 0);
    $out[] = filter_var('x', FILTER_VALIDATE_INT) === false ? 'nf' : 'f';
    $out[] = filter_var('yes', FILTER_VALIDATE_BOOLEAN, ['flags' => FILTER_NULL_ON_FAILURE]) === true ? 'yes' : 'no';
    $out[] = filter_var('maybe', FILTER_VALIDATE_BOOLEAN, ['flags' => FILTER_NULL_ON_FAILURE]) === null ? 'null' : 'nn';
    $f = filter_var('1.5', FILTER_VALIDATE_FLOAT);
    $out[] = $f === false ? 'ff' : (string) $f;
    $v = strpos('abc', 'b');
    $out[] = $v != false ? 'pos' : 'nopos';
    return implode(',', $out);
}

class NBase
{
}

class NMid extends NBase
{
}

final class NLit extends NMid
{
    public function __construct(public int $v)
    {
    }
}

final class NRange extends NMid
{
    public function __construct(private int $min)
    {
    }

    public function isPositive(): bool
    {
        return $this->min > 0;
    }
}

function narrow_pair(NBase $x, NBase $y): string
{
    if ($x instanceof NMid && $y instanceof NMid) {
        $positive = false;
        if ($x instanceof NLit) {
            $positive = $x->v > 0;
        } elseif ($x instanceof NRange) {
            $positive = $x->isPositive();
        }
        return $positive ? 'p' : 'n';
    }
    return '-';
}

function case_elseif_narrowing(): string
{
    return narrow_pair(new NLit(3), new NMid()) . ',' . narrow_pair(new NRange(-1), new NLit(1)) . ',' . narrow_pair(new NBase(), new NMid());
}

/** @return array<int, array{flags: list<int>, options: array<string, int>}> */
function filter_table(): array
{
    $general = [9];
    $validate = [
        257 => [
            'flags' => [1, 2],
            'options' => ['min' => 1],
        ],
        259 => [
            'flags' => [1],
            'options' => [],
        ],
    ];
    foreach ($validate as $filter_int => $filter_data) {
        $validate[$filter_int]['flags'] = array_merge($filter_data['flags'], $general);
        $defaults = ['d' => 0];
        $validate[$filter_int]['options'] = array_merge($filter_data['options'], $defaults);
    }
    $other = [
        516 => [
            'flags' => [3],
            'options' => ['d' => 0],
        ],
    ];
    return $validate + $other;
}

function case_filter_table(): string
{
    $out = [];
    foreach (filter_table() as $id => $entry) {
        $opts = [];
        foreach ($entry['options'] as $k => $v) {
            $opts[] = $k . '=' . $v;
        }
        $out[] = $id . ':' . implode(',', $entry['flags']) . '|' . implode(',', $opts);
    }
    return implode(';', $out);
}

function pair_str(string $a, string $b): string
{
    return $a . $b;
}

function case_element_retype(): string
{
    $table = require __DIR__ . '/data/table.php';
    $copy = [];
    foreach ($table as $name => $row) {
        $copy[strtoupper($name)] = $row;
    }
    $out = [];
    foreach ($copy as $name => [$n, $s]) {
        $out[] = $name . '=' . $n . $s;
    }
    $parts = explode('-', 'a-b');
    return implode(',', $out) . '|' . pair_str(...$parts);
}

final class NStrLit extends NBase
{
    public function __construct(public string $value)
    {
    }
}

function narrow_reassign(NBase $left, NBase $right): string
{
    if ($left instanceof NStrLit) {
        if (is_numeric($left->value)) {
            $left = new NLit((int) $left->value);
        }
    }
    if ($left instanceof NRange && $right instanceof NRange) {
        return 'rr';
    }
    if (($left instanceof NRange && $right instanceof NMid) ||
        ($left instanceof NMid && $right instanceof NRange)
    ) {
        return 'ri';
    }
    if ($left instanceof NMid && $right instanceof NMid) {
        $positive = false;
        if ($left instanceof NLit) {
            $positive = $left->v > 0;
        } elseif ($left instanceof NRange) {
            $positive = $left->isPositive();
        }
        return $positive ? 'p' : 'n';
    }
    return '-';
}

function case_narrow_reassign(): string
{
    return narrow_reassign(new NStrLit('3'), new NMid()) . ',' . narrow_reassign(new NRange(2), new NLit(1)) . ','
        . narrow_reassign(new NRange(-1), new NMid()) . ',' . narrow_reassign(new NStrLit('x'), new NMid()) . ','
        . narrow_reassign(new NRange(1), new NRange(2));
}

function case_array_to_xml(): string
{
    $empty = \Spatie\ArrayToXml\ArrayToXml::convert(['item' => []], 'report', true, 'UTF-8', '1.0', ['preserveWhiteSpace' => false, 'formatOutput' => true]);
    $items = [
        ['severity' => 'error', 'line_from' => 4, 'taint_trace' => '', 'refs' => [['label' => 'a & b'], ['label' => 'c']]],
    ];
    $full = \Spatie\ArrayToXml\ArrayToXml::convert(['item' => $items], 'report', true, 'UTF-8', '1.0', ['preserveWhiteSpace' => false, 'formatOutput' => true]);
    return $empty . '|' . $full;
}

/**
 * @param null|Closure(string): array{id: int|null, count: int} $handler
 */
function handle_message(string $message, ?Closure $handler): string
{
    if ($handler === null) {
        return 'none';
    }
    $reply = $handler($message);
    return $message . ':' . $reply['count'];
}

function case_closure_param(): string
{
    $pool = new MsgPool();
    $pool->run(['x'], static fn(string $d): int => strlen($d), null, /** @return array{id: int|null, count: int} */ static fn(string $m): array => ['id' => null, 'count' => strlen($m)]);
    return handle_message('a', /** @return array{id: int|null, count: int} */ static fn(string $m): array => ['id' => null, 'count' => strlen($m)]) . '|' . handle_message('b', null) . '|' . $pool->last;
}

final class MsgPool
{
    public string $last = '';

    /**
     * @template TResult
     * @param list<string> $items
     * An array of task data items to be divided up among the
     * workers. The size of this is the number of forked processes.
     * @phpcsSuppress SlevomatCodingStandard.TypeHints.ParameterTypeHint
     * @param Closure(string): TResult $task_factory Builds the task executed on each task data item.
     *                                                                It must return an array (to be gathered).
     *
     * @param Closure(TResult $data):void $task_done_closure A closure to execute when a task is done
     * @param null|Closure(string): array{id: int|null, count: int} $message_handler Handles a message sent by a worker over its task
     *        channel and returns the reply. Used to answer requests a task makes mid-execution (e.g.
     *        registering a custom taint in the parent process).
     */
    public function run(array $items, Closure $task_factory, ?Closure $task_done_closure = null, ?Closure $message_handler = null): void
    {
        foreach ($items as $item) {
            $result = $task_factory($item);
            if ($task_done_closure !== null) {
                $task_done_closure($result);
            }
            if ($message_handler !== null) {
                $this->last = $item . ':' . $message_handler($item)['count'];
            }
        }
    }
}

final class GBind
{
    /**
     * @template T
     * @param T $expected
     * @param T $actual
     */
    public static function same($expected, $actual): bool
    {
        return $expected === $actual;
    }
}

/** @return array<string, array<string, int>> */
function nested_counts(): array
{
    return ['a' => ['x' => 1]];
}

function maybe_name(int $n): ?string
{
    return $n > 0 ? 'n' : null;
}

function case_generic_binding(): string
{
    $ok = GBind::same(['a' => ['x' => 1]], nested_counts())
        && !GBind::same([], nested_counts())
        && GBind::same('n', maybe_name(1))
        && GBind::same([['T', 'of', 'string', false]], [['T', 'of', 'string', false]]);
    return $ok ? 'ok' : 'bad';
}

// ---- feature: a list destructure of null clears its targets ----

final class DNPair
{
    public function __construct(public readonly int $n, public readonly string $s)
    {
    }
}

final class DNTable
{
    /** @var array<string, DNPair> */
    private array $rows;

    public function __construct()
    {
        $this->rows = ['one' => new DNPair(1, 'x'), 'two' => new DNPair(2, 'y')];
    }

    /** @return array{0: int, 1: string}|null */
    public function find(string $key): ?array
    {
        $row = $this->rows[$key] ?? null;
        if ($row === null) {
            return null;
        }
        return [$row->n, $row->s];
    }
}

function case_destructure_null(): string
{
    $table = new DNTable();
    $out = [];
    foreach (['one', 'missing', 'two'] as $key) {
        $n = null;
        $s = null;
        [$n, $s] = $table->find($key);
        $out[] = 'a=' . ($n === null ? '' : (string) $n) . ',b=' . ($s === null ? '' : $s);
    }
    return implode('|', $out);
}

// ---- feature: assert-if-false does not collapse a ternary's other arm ----

final class AIFLiteral
{
    public function __construct(public readonly string $value)
    {
    }
}

/**
 * @psalm-assert-if-false !numeric $literal_array_key
 * @psalm-pure
 */
function aif_key_int(string|int $literal_array_key): false|int
{
    if (is_int($literal_array_key)) {
        return $literal_array_key;
    }
    if (!is_numeric($literal_array_key)) {
        return false;
    }
    return (int) $literal_array_key;
}

function case_assert_if_false_key(): string
{
    $out = [];
    foreach (['Foo', '15'] as $raw) {
        $literal = new AIFLiteral($raw);
        $key_value = null;
        $string_to_int = aif_key_int($literal->value);
        $key_value = $string_to_int === false ? $literal->value : $string_to_int;
        $out[] = is_string($key_value) ? 's:' . $key_value : 'i:' . $key_value;
    }
    return implode('|', $out);
}

// ---- feature: substr_count counts only inside its offset/length window ----

function case_substr_count_window(): string
{
    $doc = "/**\n * @psalm-import-type abcd\n * @var int $p\n * @psalm-consistent-constructor\n */\n";
    $out = [];
    $out[] = (string) substr_count($doc, "\n");
    $out[] = (string) substr_count($doc, "\n", 0, 10);
    $out[] = (string) substr_count($doc, "\n", 0, 35);
    $out[] = (string) substr_count($doc, "\n", 0, 52);
    $out[] = (string) substr_count($doc, "\n", 4, 20);
    $out[] = (string) substr_count($doc, "\n", -12);
    return implode('|', $out);
}

// ---- feature: a comparison the types settle still evaluates its operand ----

final class SFCounter
{
    public static int $calls = 0;
}

final class SFThing
{
    public function touch(string $key): SFThing
    {
        SFCounter::$calls++;
        if ($key === 'bad') {
            throw new \RuntimeException('bad key');
        }
        return $this;
    }
}

function case_static_false_compare(): string
{
    $out = [];
    $thing = new SFThing();
    if ((new SFThing())->touch('ok') === false) {
        $out[] = 'false';
    }
    $out[] = 'called:' . SFCounter::$calls;
    try {
        $thing->touch('bad');
        $out[] = 'no-throw';
    } catch (\RuntimeException $e) {
        $out[] = 'caught';
    }
    return implode('|', $out);
}

// ---- feature: `??=` keeps a nullable right-hand side nullable ----

final class CNArm
{
    public function __construct(public readonly string $name)
    {
    }
}

/** @param list<CNArm> $arms */
function cn_pick(array $arms): string
{
    $last = null;
    $last ??= array_shift($arms);
    return $last === null ? 'none' : $last->name;
}

function case_coalesce_nullable(): string
{
    return cn_pick([]) . '|' . cn_pick([new CNArm('a'), new CNArm('b')]);
}

// ---- feature: a static read through an object names that object's class ----

final class SVOHook
{
    public static bool $called = false;

    public static int $count = 7;

    public static function fire(): void
    {
        self::$called = true;
    }
}

function case_static_via_object(): string
{
    $hook = new SVOHook();
    $out = $hook::$called ? 'yes' : 'no';
    SVOHook::fire();
    return $out . '|' . ($hook::$called ? 'yes' : 'no') . '|' . $hook::$count;
}

// ---- feature: an override's extra by-reference parameter reaches the caller ----

interface BRVisitor
{
    public function visit(string $name): string;
}

final class BRPlain implements BRVisitor
{
    public function visit(string $name): string
    {
        return $name . ':go';
    }
}

final class BRStopper implements BRVisitor
{
    public function visit(string $name, bool &$descend = true): string
    {
        $descend = false;
        return $name . ':stop';
    }
}

/** @param list<array{string, BRVisitor}> $rows */
function br_run(array $rows): string
{
    $out = [];
    foreach ($rows as [$name, $visitor]) {
        $descend = true;
        $label = $visitor->visit($name, $descend);
        $out[] = $label . '/' . ($descend ? 'yes' : 'no');
    }
    return implode('|', $out);
}

function case_byref_override_arg(): string
{
    return br_run([['a', new BRStopper()], ['b', new BRPlain()]]);
}

// ---- feature: shifting past the word width empties the value, as PHP does ----

function case_php_shifts(): string
{
    $one = 1;
    $sixtyfour = 64;
    $neg = -8;
    $out = [];
    $out[] = (string) ($one << $sixtyfour);
    $out[] = (string) ($one >> 0);
    $out[] = (string) ($one >> $sixtyfour);
    $out[] = (string) ($neg >> $sixtyfour);
    $out[] = (string) ($one << 1);
    $out[] = (string) ($one << 65);
    $out[] = (string) ($one << 62);
    return implode('|', $out);
}

// ---- feature: every builtin mapping takes the arguments PHP passes it ----

function case_builtin_arity(): string
{
    // a mapping that declares fewer parameters than the call passes only fails when the generated
    // crate is compiled, so every widened mapping is called here with all of them
    $dir = sys_get_temp_dir();
    $file = $dir . '/tiny_builtin_arity.txt';
    $ok = touch($file, 1000000000)
        && count(scandir($dir, 1)) > 0
        && htmlspecialchars("a'b\"c", ENT_QUOTES) === 'a&#039;b&quot;c'
        && round(2.5, 0, PHP_ROUND_HALF_DOWN) === 2.0
        && str_starts_with(uniqid('p', true), 'p')
        && class_exists('Rt\\Missing', false) === false
        && count(get_loaded_extensions(false)) > 0;
    clearstatcache(true, $file);
    @unlink($file);
    return $ok ? 'ok' : 'bad';
}

// ---- feature: a `%` conversion writes a literal percent, whatever precedes it ----

function case_sprintf_percent(): string
{
    return implode('|', [
        sprintf('%%'),
        sprintf('%1$%', 7),
        sprintf('%5%', 7),
        sprintf('%-5%', 7),
        sprintf('%1$%a', 7),
        // a `%` conversion still takes an argument slot, so the next one moves on
        sprintf('%5%%s', 'a', 'b'),
        sprintf('%s%5%%s', 'a', 'b', 'c'),
    ]);
}

// ---- feature: every rounding mode PHP has ----

function case_round_modes(): string
{
    $cases = [[2.5, 0], [3.5, 0], [-2.5, 0], [1.45, 1], [1.55, 1], [2.4, 0], [-2.4, 0], [1.0, 0], [-1.5, 0]];
    $out = [];
    foreach ($cases as [$v, $p]) {
        foreach ([1, 2, 3, 4, 5, 6, 7, 8] as $m) {
            $out[] = rtrim(rtrim(number_format(round($v, $p, $m), 2, '.', ''), '0'), '.');
        }
    }
    return implode(',', $out);
}

// ---- feature: the quote flags and the doctype a report asks for ----

function case_htmlspecialchars_flags(): string
{
    $out = [];
    foreach ([ENT_QUOTES, ENT_QUOTES | ENT_XML1, ENT_QUOTES | ENT_HTML5, ENT_COMPAT, ENT_NOQUOTES] as $f) {
        $out[] = htmlspecialchars("a\"b'c<&>", $f);
    }
    return implode('|', $out);
}

// ---- feature: function_exists knows every builtin the program provides ----

function case_provided_builtins(): string
{
    $out = [];
    foreach (['sort', 'in_array', 'usort', 'file_exists', 'go', 'nonesuch_xyz'] as $name) {
        $out[] = $name . ':' . (function_exists($name) ? '1' : '0');
    }
    return implode('|', $out);
}

// ---- feature: array_map over a callback that returns nothing ----

/**
 * @return array<int, string>
 */
function amv_args(): array
{
    return [0 => 'a', 1 => 'b', 2 => 'c'];
}

function case_array_map_void(): string
{
    $seen = [];
    array_map(
        static function (string $arg) use (&$seen): void {
            $seen[] = strtoupper($arg);
        },
        amv_args(),
    );
    return implode(',', $seen);
}

// ---- feature: an interpreted-only branch may instantiate a class by name ----

final class NamedPlugin
{
    public function tag(): string
    {
        return 'named';
    }
}

/**
 * @param class-string<NamedPlugin> $name
 */
function make_by_name(string $name): NamedPlugin
{
    if (!\defined('PSALM_COMPILED')) {
        return new $name();
    }
    return new NamedPlugin();
}

function case_dynamic_new_dead(): string
{
    return make_by_name(NamedPlugin::class)->tag();
}

// ---- feature: __DIR__ inside a class constant ----

final class DirConst
{
    public const FIXTURES = __DIR__ . '/fixtures';
}

function case_dir_const(): string
{
    return basename(dirname(DirConst::FIXTURES)) . '/' . basename(DirConst::FIXTURES);
}

// ---- feature: a class-string passed where an object or a class name is accepted ----

interface Hookish
{
    public function tag(): string;
}

final class HookA implements Hookish
{
    public function tag(): string
    {
        return 'A';
    }
}

/**
 * @param Hookish|class-string<Hookish> $handler
 */
function hook_tag(Hookish|string $handler): string
{
    if (is_string($handler)) {
        return 'name:' . $handler;
    }
    return 'obj:' . $handler->tag();
}

function case_class_string_or_object(): string
{
    return hook_tag(HookA::class) . '|' . hook_tag(new HookA()) . '|' . hook_tag('Other');
}

// ---- feature: a class-name-or-object parameter narrowed and reassigned in the body ----

interface HookBase
{
    public function tag(): string;
}

interface HookExtra extends HookBase
{
    public function extra(): string;
}

final class HookPlain implements HookBase
{
    public function tag(): string
    {
        return 'plain';
    }
}

final class HookRich implements HookExtra
{
    public function tag(): string
    {
        return 'rich';
    }

    public function extra(): string
    {
        return '+';
    }
}

/**
 * @param class-string<HookBase> $name
 */
function hook_by_name(string $name): HookBase
{
    return $name === HookRich::class ? new HookRich() : new HookPlain();
}

/**
 * @param HookBase|class-string<HookBase> $handler
 */
function register_hook(HookBase|string $handler): string
{
    if (is_string($handler)) {
        $handler = hook_by_name($handler);
    }
    $out = $handler->tag();
    if ($handler instanceof HookExtra) {
        $out .= $handler->extra();
    }
    return $out;
}

function case_hook_param_narrowing(): string
{
    return register_hook(HookRich::class) . '|' . register_hook(new HookPlain()) . '|' . register_hook(HookPlain::class);
}

// ---- feature: method_exists() on a value of known class ----

final class HasMethods
{
    public function known(): int
    {
        return 1;
    }
}

function case_method_exists(): string
{
    $o = new HasMethods();
    return (method_exists($o, 'known') ? 'y' : 'n') . (method_exists($o, 'nope') ? 'y' : 'n');
}

final class ShapeConsts
{
    public const SPECIAL = ['int' => 'int', 'string' => 'string', '0' => 'zero'];
}

/** A constant shape read with a variable key: a key match, not a map rebuilt per read. */
function case_shape_dyn_key(): string
{
    $out = '';
    foreach (['int', 'nope', '0', 'string'] as $k) {
        $out .= isset(ShapeConsts::SPECIAL[$k]) ? ShapeConsts::SPECIAL[$k] : '-';
        $out .= ',';
    }
    return $out;
}

final class TokenCursor
{
    /** @var list<string> */
    public array $tokens = ['a', 'b', 'c'];
    public int $pos = -1;

    public function next(): string
    {
        return $this->tokens[++$this->pos];
    }

    public function count(): int
    {
        return count($this->tokens) + (isset($this->tokens[$this->pos]) ? 1 : 0);
    }
}

/** An element read whose key writes the same object: no borrow of the object may span the key. */
function case_guard_key_writes(): string
{
    $c = new TokenCursor();
    return $c->next() . $c->next() . $c->count();
}

/**
 * @template T
 * @param list<T> $items
 * @return array<int, T>
 */
function generic_or_empty(array $items, bool $empty): array
{
    if ($empty) {
        return [];
    }
    return $items;
}

/** An empty literal returned where the return type is a container of a generic. */
function case_generic_empty_return(): string
{
    return count(generic_or_empty(['a', 'b'], true)) . ':' . count(generic_or_empty(['a', 'b'], false));
}

/**
 * Identity observed through spl_object_id: must stay a handle under VALUE_TYPES (a copy would get a fresh id and
 * the visited-set cycle check below would never fire: ConstantTypeResolver looped until OOM this way).
 *
 * @psalm-immutable
 */
final class IdNode
{
    public function __construct(public readonly string $name, public readonly string $next)
    {
    }
}

/**
 * @param array<string, IdNode> $nodes
 * @param array<int, true> $visited
 */
function id_walk(array $nodes, IdNode $n, array $visited, int $depth): string
{
    $id = spl_object_id($n);
    if (isset($visited[$id])) {
        return 'cycle@' . $depth . ':' . $n->name;
    }
    if ($depth > 20) {
        return 'deep';
    }
    return id_walk($nodes, $nodes[$n->next], $visited + [$id => true], $depth + 1);
}

function case_identity_cycle(): string
{
    $nodes = ['a' => new IdNode('a', 'b'), 'b' => new IdNode('b', 'c'), 'c' => new IdNode('c', 'a')];
    return id_walk($nodes, $nodes['a'], [], 0);
}

/**
 * An immutable hierarchy whose base fills a plain field through a base-typed local after a private factory
 * (Atomic::create) and withers it through a clone-local: both writes go through `&mut` on the local's binding.
 *
 * @psalm-immutable
 */
abstract class VBase
{
    public bool $flag = false;

    public static function make(int $v, bool $flag): VBase
    {
        $r = self::inner($v);
        /** @psalm-suppress ImpurePropertyAssignment, InaccessibleProperty construction */
        $r->flag = $flag;
        return $r;
    }

    private static function inner(int $v): VBase
    {
        return new VLeaf($v);
    }

    /** @return static */
    public function withFlag(bool $f): static
    {
        if ($f === $this->flag) {
            return $this;
        }
        $c = clone $this;
        /** @psalm-suppress ImpurePropertyAssignment, InaccessibleProperty wither */
        $c->flag = $f;
        return $c;
    }
}

/**
 * @psalm-immutable
 */
final class VLeaf extends VBase
{
    public function __construct(public int $v)
    {
    }
}

function case_value_factory(): string
{
    $a = VBase::make(3, true);
    $b = $a->withFlag(false);
    $same = $a->withFlag(true);
    return ($a->flag ? 'T' : 'F') . ($b->flag ? 'T' : 'F') . ($b instanceof VLeaf ? $b->v : 0) . ':' . ($same === $a ? 'T' : 'F');
}

/**
 * A TNamedObject-like value: a name, an intersection map of atoms, a memoized key, withers, and an expander
 * that narrows an enum-typed parameter and reassigns it with wither results.
 *
 * @psalm-immutable
 */
class NAtom
{
    private ?string $key_memo = null;

    protected function computeKey(): string
    {
        return 'atom';
    }

    public function getKey(): string
    {
        if ($this->key_memo === null) {
            /** @psalm-suppress ImpurePropertyAssignment memo */
            $this->key_memo = $this->computeKey();
        }
        return $this->key_memo;
    }

    /**
     * @psalm-mutation-free
     */
    protected function __clone()
    {
        /** @psalm-suppress ImpurePropertyAssignment memo */
        $this->key_memo = null;
    }
}

/**
 * @psalm-immutable
 */
class NNamed extends NAtom
{
    /** @param array<string, NAtom> $extra_types */
    public function __construct(public string $value, public array $extra_types = [], public bool $is_static = false)
    {
    }

    protected function computeKey(): string
    {
        $k = $this->value;
        foreach ($this->extra_types as $t) {
            $k .= '&' . $t->getKey();
        }
        return $k;
    }

    /** @param array<string, NAtom> $extra */
    public function setIntersectionTypes(array $extra): self
    {
        if ($extra === $this->extra_types) {
            return $this;
        }
        $c = clone $this;
        /** @psalm-suppress ImpurePropertyAssignment wither */
        $c->extra_types = $extra;
        return $c;
    }

    public function setValue(string $v): self
    {
        if ($v === $this->value) {
            return $this;
        }
        $c = clone $this;
        /** @psalm-suppress ImpurePropertyAssignment wither */
        $c->value = $v;
        return $c;
    }

    public function setIsStatic(bool $s): self
    {
        if ($s === $this->is_static) {
            return $this;
        }
        $c = clone $this;
        /** @psalm-suppress ImpurePropertyAssignment wither */
        $c->is_static = $s;
        return $c;
    }
}

/**
 * @psalm-immutable
 */
final class NInt extends NAtom
{
    protected function computeKey(): string
    {
        return 'int';
    }
}

function n_expand(NAtom $t, ?string $self_class, bool $make_static): NAtom
{
    if ($t instanceof NNamed) {
        if ($t->value === 'self' && $self_class !== null) {
            $t = $t->setValue($self_class);
        }
        if ($t->extra_types !== []) {
            $extra = [];
            foreach ($t->extra_types as $e) {
                $e = n_expand($e, $self_class, false);
                $extra[$e->getKey()] = $e;
            }
            $t = $t->setIntersectionTypes($extra);
        }
        if ($make_static) {
            $t = $t->setIsStatic(true);
        }
    }
    return $t;
}

function case_value_named(): string
{
    $b = new NNamed('B');
    $a = new NNamed('A', ['B' => $b]);
    $k1 = $a->getKey();
    $c = $a->setIntersectionTypes($a->extra_types + ['C' => new NNamed('C')]);
    $k2 = $c->getKey();
    $s = n_expand(new NNamed('self'), 'Foo', false);
    $k3 = 'self->' . $s->getKey();
    /** @var array<string, NAtom> $map */
    $map = [$a->getKey() => $a, 'int' => new NInt()];
    $found = '';
    foreach ($map as $key => $atom) {
        if ($atom instanceof NNamed && $atom->extra_types !== []) {
            $found = 'k:' . $key;
        }
    }
    $st = n_expand($a, null, true);
    $k5 = 'static:' . ($st instanceof NNamed && $st->is_static ? '1' : '0');
    return $k1 . '|' . $k2 . '|' . $k3 . '|' . $found . '|' . $k5;
}

/**
 * A concrete non-leaf value (NNamed) with a value subclass adding a field (NGeneric, like TGenericObject): the
 * inherited withers run on the subclass instance, narrowing from the base, and self-resolution on both.
 *
 * @psalm-immutable
 */
final class NGeneric extends NNamed
{
    /**
     * @param list<NAtom> $type_params
     * @param array<string, NAtom> $extra_types
     */
    public function __construct(string $value, public array $type_params, array $extra_types = [], bool $is_static = false)
    {
        parent::__construct($value, $extra_types, $is_static);
    }

    protected function computeKey(): string
    {
        $k = parent::computeKey() . '<';
        foreach ($this->type_params as $i => $p) {
            $k .= ($i > 0 ? ',' : '') . $p->getKey();
        }
        return $k . '>';
    }
}

function case_value_nonleaf(): string
{
    try {
        return case_value_nonleaf_inner();
    } catch (\Throwable $e) {
        return 'EXC:' . $e::class . ':' . $e->getMessage();
    }
}

function case_value_nonleaf_inner(): string
{
    $g = new NGeneric('G', [new NInt()]);
    $k1 = $g->getKey();
    $f = n_expand(new NGeneric('self', [new NInt()]), 'Foo', false);
    $k2 = $f->getKey();
    $h = $f instanceof NNamed ? $f->setIntersectionTypes(['B' => new NNamed('B')]) : $f;
    $k3 = $h->getKey();
    $st = n_expand($h, null, true);
    $k4 = $st instanceof NNamed && $st->is_static ? '1' : '0';
    $plain = n_expand(new NNamed('self'), 'Foo', false);
    return $k1 . '|' . $k2 . '|' . $k3 . '|' . $k4 . '|self->' . $plain->getKey();
}

/**
 * A borrowed local: `$items = $this->items` in an immutable leaf class reads the list as a `&List` for the whole
 * method (no clone), used as a loop subject, a count argument and a method receiver.
 *
 * @psalm-immutable
 */
final class BList
{
    /**
     * @param list<int> $items
     * @param array<string, string> $names
     */
    public function __construct(public readonly array $items, public readonly array $names, public readonly ?BList $inner = null)
    {
    }

    /** @return list<int> */
    public function getItems(): array
    {
        return $this->items;
    }

    /** @return array<string, string> */
    public function getNames(): array
    {
        return $this->names;
    }

    public function getInner(): ?BList
    {
        return $this->inner;
    }

    public function sum(): int
    {
        $items = $this->items;
        $n = 0;
        foreach ($items as $i) {
            $n += $i;
        }
        return $n;
    }

    public function describe(): string
    {
        $names = $this->names;
        $items = $this->items;
        $inner = $this->inner;
        $keys = implode(',', array_keys($names));
        $extra = $inner !== null ? $inner->describe() : 'x';
        return count($items) . ':' . $keys . ':' . $extra;
    }
}

function case_borrow_local(): string
{
    $l = new BList([1, 2, 3], ['a' => 'A', 'b' => 'B', 'c' => 'C'], null);
    return $l->sum() . ':' . $l->describe();
}

/**
 * Borrowed getter returns: `getItems()` returns `&List` (a place into the receiver) and is used as a count
 * argument, a foreach subject, an owned local (clone), an array-key lookup and a nullable chain.
 */
/**
 * PHP's internal array pointer is not modeled: reset()/current()/key() read the first element, end() the last, on
 * locals, on fields of immutable and mutable objects, and on temporaries; none of them is a write of the argument.
 */
final class PtrBag
{
    /** @param list<int> $items */
    public function __construct(public array $items)
    {
    }
}

function case_ptr_reads(): string
{
    $l = new BList([4, 5], ['a' => 'A', 'k' => 'v'], null);
    $bag = new PtrBag([4, 9]);
    $arr = [4, 5];
    $first = reset($arr);
    $last = end($arr);
    $names = $l->names;
    $cur = current($names);
    $key = key($names);
    $a = reset($bag->items);
    $b = end($bag->items);
    $c = reset($l->items);
    $d = current($l->names);
    $e = reset($l->getNames());
    $empty = [];
    $none = reset($empty) === false ? 'x' : 'y';
    return $first . ':' . $last . ':' . $cur . ':' . $key . ':' . $a . ':' . $b . ':' . $c . ':' . ($d === 'A' ? 'v' : 'w') . ':' . ($e === 'A' ? $none : 'z');
}

function case_borrow_return(): string
{
    $l = new BList([4, 5], ['k' => 'v'], new BList([9], [], null));
    $n = count($l->getItems());
    $s = 0;
    foreach ($l->getItems() as $i) {
        $s += $i;
    }
    $owned = $l->getItems();
    $owned[] = 6;
    $names = $l->getNames();
    $inner = $l->getInner();
    $in = $inner !== null ? count($inner->getItems()) : -1;
    $k = $l->getNames()['k'];
    return $n . ':' . $s . ':' . count($owned) . ':' . count($l->getItems()) . ':' . count($names) . ':' . $in . ':' . $k
        . ':' . ($l->getInner()?->getInner() === null ? 'null' : 'obj');
}

final class CondRet
{
    /**
     * @psalm-pure
     * @return ($id is null ? null : string)
     */
    public static function lookupOrNull(?int $id): ?string
    {
        return $id === null ? null : 'n' . $id;
    }
}

/** A conditional return type (`($x is null ? null : string)`) is a nullable string: `??` falls back on null. */
function case_cond_return(): string
{
    $a = null;
    $b = 7;
    $x = CondRet::lookupOrNull($a) ?? 'fallback';
    $y = CondRet::lookupOrNull($b) ?? 'fallback';
    $z = CondRet::lookupOrNull($b);
    return $x . ':' . $y . ':' . ($z ?? 'none');
}

abstract class OptShape
{
}

final class OptCircle extends OptShape
{
    public function __construct(public int $r)
    {
    }
}

final class OptSquare extends OptShape
{
}

/** `instanceof` on a nullable object local tests it in place (no clone), across a loop popping from a list. */
function case_option_instanceof(): string
{
    /** @var list<OptShape> $shapes */
    $shapes = [new OptSquare(), new OptCircle(3), new OptSquare()];
    $out = '';
    while ($shape = array_pop($shapes)) {
        if ($shape instanceof OptCircle) {
            $out .= 'c' . $shape->r;
        } elseif ($shape instanceof OptSquare) {
            $out .= 's';
        }
    }
    $none = array_pop($shapes);
    return $out . ($none instanceof OptCircle ? 'C' : '-');
}
