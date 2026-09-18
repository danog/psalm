<?php

declare(strict_types=1);

namespace Psalm\Tests;

/**
 * The classes PHP itself provides come from the version stubs in a compiled build (there is no reflection
 * of the interpreter's own classes), so a class declared only in Php80.phpstub must be registered with its
 * members once the preloaded stubs have been visited.
 */
final class VersionStubTest extends TestCase
{
    public function testAttributeClassComesFromTheVersionStub(): void
    {
        $this->project_analyzer->setPhpVersion('8.0', 'tests');

        $codebase = $this->project_analyzer->getCodebase();
        $codebase->config->visitPreloadedStubFiles($codebase);

        $this->assertTrue(
            $codebase->classlikes->classExists('Attribute'),
            'Attribute is declared by stubs/Php80.phpstub',
        );

        $storage = $codebase->classlike_storage_provider->get('Attribute');

        $this->assertTrue(
            isset($storage->methods['__construct']),
            'the stub declares Attribute::__construct',
        );
    }

    /** Analysis tests that name no PHP version analyze as 7.4, but the interpreter's classes still exist. */
    public function testAttributeClassExistsWhenAnalyzingAnOlderVersion(): void
    {
        $this->project_analyzer->setPhpVersion('7.4', 'tests');

        $codebase = $this->project_analyzer->getCodebase();
        $codebase->config->visitPreloadedStubFiles($codebase);

        $this->assertTrue(
            $codebase->classlikes->classExists('Attribute'),
            'the runtime provides Attribute regardless of the analyzed version',
        );
    }

    /** The analysis tests preload the stubs in server mode, as the traits do. */
    public function testAttributeClassComesFromTheVersionStubInServerMode(): void
    {
        $this->project_analyzer->setPhpVersion('8.0', 'tests');

        $codebase = $this->project_analyzer->getCodebase();
        $codebase->enterServerMode();
        $codebase->config->visitPreloadedStubFiles($codebase);

        $this->assertTrue(
            $codebase->classlikes->classExists('Attribute'),
            'Attribute is declared by stubs/Php80.phpstub',
        );

        $storage = $codebase->classlike_storage_provider->get('Attribute');

        $this->assertTrue(
            isset($storage->methods['__construct']),
            'the stub declares Attribute::__construct',
        );
    }
}
