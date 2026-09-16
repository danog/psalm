<?php

namespace Tiny;

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

/**
 * Mirrors SimpleNameResolver::resolveType: a conditional-return function whose
 * declared return is a nullable union, assigned into a nullable-union field.
 * The null path must stay None, not be forced into the (now closed) bare union.
 *
 * @return ($n is A ? A : ($n is B ? B : A|B|null))
 */
function resolve(A|B|null $n): A|B|null
{
    return $n;
}

function run(): string
{
    $h = new Holder();
    $h->x = resolve(null);
    $r1 = $h->x === null ? 'null-ok' : 'BAD';
    $h->x = resolve(new A(7));
    $r2 = $h->x instanceof A ? ('A:' . $h->x->a) : 'BAD';
    return $r1 . '|' . $r2;
}
