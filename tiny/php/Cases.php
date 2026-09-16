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

function run_all(): string
{
    return check('nullable_union', case_nullable_union(), 'nullA7')
        . check('wildcard_const', case_wildcard_const(), 'set')
        . check('value_of', case_value_of(), 'HS')
        . check('return_move', case_return_move(), '534')
        . check('single_use_move', case_single_use_move(), '766')
        . check('foreach_collection_move', case_foreach_collection_move(), '60')
        . check('alias_mutation', case_alias_mutation(), '7')
        . check('receiver_move', case_receiver_move(), '11')
        . check('borrow_param', case_borrow_param(), '42425')
        . check('late_receiver_borrow', case_late_receiver_borrow(), '00')
        . check('closure_capture', case_closure_capture(), '8');
}
