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

function run_all(): string
{
    return check('nullable_union', case_nullable_union(), 'nullA7')
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
        . check('closure_capture', case_closure_capture(), '8');
}
    check('empty_arrays', case_empty_arrays(), 's0,s1|0|2');
    check('try_catch', case_try_catch(), '2,f,caught:big 3,f,outer,d,unhandled');
    check('generics', case_generics(), 'i:eq,s:ne,l:eq,42,xy,has,no,5,strintother');
    check('phpunit', case_phpunit(), '1,failed,skip:later,expects,verified,mismatch,testThrows with data set "ds"');
    check('object_eq', case_object_eq(), 'eq,ne,ne,notsame,eq,eq,ne');

// ---- feature: typed constant table (get_defined_constants without Mixed) ----

function case_defined_constants(): string
{
    $constants = get_defined_constants();
    $out = [];
    $out[] = isset($constants['PHP_INT_SIZE']) ? 'size' . (string) $constants['PHP_INT_SIZE'] : 'nosize';
    $out[] = isset($constants['E_ALL']) && is_int($constants['E_ALL']) ? 'eall' : 'noeall';
    $out[] = ($constants['PHP_EOL'] ?? '') === "\n" ? 'eol' : 'noeol';
    $out[] = array_key_exists('M_PI', $constants) && is_float($constants['M_PI']) ? 'pi' : 'nopi';
    return implode(',', $out);
}
check('defined_constants', case_defined_constants(), 'size8,eall,eol,pi');

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
check('data_file', case_data_file(), 'a1x-,b2y3.5|a,b');

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
check('elseif_assign', case_elseif_assign(), 'none,a5,skip|1');
