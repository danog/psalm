<?php

declare(strict_types=1);

namespace Psalm\Tests\Internal\Codebase;

use Psalm\Internal\Interner;

use Override;
use Psalm\Internal\Codebase\ClassLikes;
use Psalm\Internal\Provider\ClassLikeStorageProvider;
use Psalm\Storage\ClassLikeStorage;
use Psalm\Tests\TestCase;

final class ClassLikesTest extends TestCase
{
    private ClassLikes $classlikes;

    private ClassLikeStorageProvider $storage_provider;

    #[Override]
    public function setUp(): void
    {
        parent::setUp();
        $this->classlikes = $this->project_analyzer->getCodebase()->classlikes;
        $this->storage_provider = $this->project_analyzer->getCodebase()->classlike_storage_provider;
    }

    public function testWillDetectClassImplementingAliasedInterface(): void
    {
        $this->classlikes->addClassAlias(Interner::intern('Foo'), 'Bar');

        $classStorage = new ClassLikeStorage(Interner::intern('Baz'));
        $classStorage->class_implements['bar'] = 'Bar';

        $this->storage_provider->addMore(['baz' => $classStorage]);

        self::assertTrue($this->classlikes->classImplements(Interner::intern('Baz'), Interner::intern('Foo')));
    }

    public function testWillResolveAliasedAliases(): void
    {
        $this->classlikes->addClassAlias(Interner::intern('Foo'), 'Bar');
        $this->classlikes->addClassAlias(Interner::intern('Bar'), 'Baz');
        $this->classlikes->addClassAlias(Interner::intern('Baz'), 'Qoo');

        self::assertSame('Foo', $this->classlikes->getUnAliasedName('Qoo'));
    }
}
