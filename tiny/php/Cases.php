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

function run_all(): string
{
    return check('node_parts', case_node_parts(), 'PE:PE')
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
        . check('defined_constants', case_defined_constants(), 'size8,eall,eol,pi')
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
        . check('array_to_xml', case_array_to_xml(), "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<report>\n  <item/>\n</report>\n|<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<report>\n  <item>\n    <severity>error</severity>\n    <line_from>4</line_from>\n    <taint_trace/>\n    <refs>\n      <label>a &amp; b</label>\n    </refs>\n    <refs>\n      <label>c</label>\n    </refs>\n  </item>\n</report>\n");
}

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
