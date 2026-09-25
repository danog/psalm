<?php

declare(strict_types=1);

namespace Psalm\Tests\Internal;

use Psalm\Internal\Interner;
use Psalm\Internal\Sym;
use Psalm\Tests\TestCase;

final class InternerTest extends TestCase
{
    public function testIdsAreStableHashesAndRoundTrip(): void
    {
        $id = Interner::intern('Foo\\Bar');
        $this->assertSame($id, Interner::intern('Foo\\Bar'));
        $this->assertSame($id, Interner::hash('Foo\\Bar'));
        $this->assertSame('Foo\\Bar', Interner::lookup($id));
        $this->assertNotSame($id, Interner::intern('foo\\bar'), 'ids are case-sensitive');
        $this->assertGreaterThanOrEqual(0, $id);
    }

    public function testPreloadedConstantsMatchTheHash(): void
    {
        $this->assertSame(Sym::TRAVERSABLE, Interner::hash('Traversable'));
        $this->assertSame(Sym::CONSTRUCT, Interner::intern('__construct'));
        $this->assertSame('Traversable', Interner::lookup(Sym::TRAVERSABLE));
        $this->assertSame(Sym::EMPTY, Interner::intern(''));
    }

    public function testDeltaListsOnlyWhatWasInternedSinceMark(): void
    {
        Interner::intern('BeforeMark');
        Interner::mark();
        Interner::intern('BeforeMark');
        Interner::intern('AfterMark');
        $this->assertSame(['AfterMark'], Interner::delta());
        Interner::merge(['Merged']);
        $this->assertSame('Merged', Interner::lookup(Interner::hash('Merged')));
    }
}
