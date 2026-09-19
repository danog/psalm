<?php

declare(strict_types=1);

namespace Psalm\Tests;

use Psalm\Context;
use Psalm\Internal\MethodIdentifier;

use function array_keys;
use function implode;

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
        $codebase->config->visitStubFiles($codebase);

        $this->assertTrue(
            $codebase->classlikes->classExists('Attribute'),
            'Attribute is declared by stubs/CoreGenericClasses.phpstub',
        );

        $storage = $codebase->classlike_storage_provider->get('Attribute');

        $this->assertTrue(
            isset($storage->methods['__construct']),
            'the stub declares Attribute::__construct',
        );
    }

    /** The analysis tests preload the stubs in server mode, as the traits do. */
    public function testAttributeClassComesFromTheVersionStubInServerMode(): void
    {
        $this->project_analyzer->setPhpVersion('8.0', 'tests');

        $codebase = $this->project_analyzer->getCodebase();
        $codebase->enterServerMode();
        $codebase->config->visitPreloadedStubFiles($codebase);
        $codebase->config->visitStubFiles($codebase);

        $this->assertTrue(
            $codebase->classlikes->classExists('Attribute'),
            'Attribute is declared by stubs/CoreGenericClasses.phpstub',
        );

        $storage = $codebase->classlike_storage_provider->get('Attribute');

        $this->assertTrue(
            isset($storage->methods['__construct']),
            'the stub declares Attribute::__construct',
        );
    }

    /**
     * IteratorAggregate::getIterator is declared without a native return type, so a class implementing it
     * without one (ArrayObject) must not be reported as a signature mismatch.
     */
    public function testIteratorAggregateHasNoSignatureReturnType(): void
    {
        $this->project_analyzer->setPhpVersion('8.0', 'tests');

        $codebase = $this->project_analyzer->getCodebase();
        $codebase->config->visitPreloadedStubFiles($codebase);
        $codebase->config->visitStubFiles($codebase);

        $storage = $codebase->classlike_storage_provider->get('IteratorAggregate');

        $method = $storage->methods['getiterator'] ?? null;

        $this->assertNull(
            $method?->signature_return_type,
            'the stub declares getIterator() without a native return type;'
            . ' storage name=' . $storage->name
            . ' interface=' . var_export($storage->is_interface, true)
            . ' file=' . ($storage->location?->file_path ?? 'none')
            . ' methods=' . implode(',', array_keys($storage->methods))
            . ' defining=' . ($method?->defining_fqcln ?? 'none')
            . ' method_file=' . ($method?->location?->file_path ?? 'none')
            . ' signature=' . ($method?->signature_return_type === null ? 'null' : (string) $method->signature_return_type)
            . ' return=' . ($method?->return_type === null ? 'null' : (string) $method->return_type),
        );
    }

    /**
     * A method declared without a native return type has no signature return type, whatever its
     * docblock says: `Stringable::__toString()` in the version stub carries only `@return string`,
     * and a class whose own `__toString()` is untyped must not be reported as a signature mismatch.
     */
    public function testDocblockOnlyReturnLeavesTheSignatureReturnTypeUnset(): void
    {
        $this->project_analyzer->setPhpVersion('8.0', 'tests');

        $file_path = self::$src_dir_path . 'somefile.php';

        $this->addFile(
            $file_path,
            '<?php
                interface LocalStringable {
                    /** @return string */
                    public function render();
                }

                final class Renderer implements LocalStringable {
                    public function render() {
                        return "x";
                    }

                    public function __toString() {
                        return "x";
                    }
                }',
        );

        $this->analyzeFile($file_path, new Context());

        $codebase = $this->project_analyzer->getCodebase();

        $local = $codebase->methods->getStorage(new MethodIdentifier('LocalStringable', 'render'));

        $this->assertNull(
            $local->signature_return_type,
            'a docblock @return does not give the method a signature return type',
        );

        $stringable = $codebase->methods->getStorage(new MethodIdentifier('Stringable', '__tostring'));

        $stringable_storage = $codebase->classlike_storage_provider->get('Stringable');

        $this->assertNull(
            $stringable->signature_return_type,
            'Stringable::__toString in the version stub has no native return type;'
            . ' storage file=' . ($stringable_storage->location?->file_path ?? 'none')
            . ' methods=' . implode(',', array_keys($stringable_storage->methods))
            . ' defining=' . ($stringable->defining_fqcln ?? 'none')
            . ' method_file=' . ($stringable->location?->file_path ?? 'none')
            . ' signature=' . (string) $stringable->signature_return_type,
        );

        $this->assertSame(
            'string',
            (string) $stringable->return_type,
            'its docblock @return is still read',
        );
    }

    /**
     * A compiled program carries its own shims for the reflection classes, and they describe far less
     * than Psalm's stub does. The stub is the description the analyzer must use.
     */
    public function testReflectionClassesComeFromPsalmsOwnStub(): void
    {
        $this->assertReflectionClassesAreStubbed('8.0', false);
    }

    /** The analysis tests read the stubs in server mode. */
    public function testReflectionClassesComeFromPsalmsOwnStubInServerMode(): void
    {
        // getAttributes() is @since 8.0, so the stub only declares it from that version on
        $this->assertReflectionClassesAreStubbed('8.0', true);
    }

    private function assertReflectionClassesAreStubbed(string $php_version, bool $server_mode): void
    {
        $this->project_analyzer->setPhpVersion($php_version, 'tests');

        $codebase = $this->project_analyzer->getCodebase();

        if ($server_mode) {
            $codebase->enterServerMode();
        }

        $codebase->config->visitPreloadedStubFiles($codebase);
        $codebase->config->visitStubFiles($codebase);

        foreach (['ReflectionClass', 'ReflectionFunction', 'ReflectionMethod', 'ReflectionProperty'] as $name) {
            $storage = $codebase->classlike_storage_provider->get($name);

            $this->assertTrue(
                $codebase->methods->methodExists($codebase, new MethodIdentifier($name, 'getattributes')),
                $name . '::getAttributes() is declared by stubs/Reflection.phpstub;'
                . ' storage file=' . ($storage->location?->file_path ?? 'none')
                . ' parent=' . ($storage->parent_class ?? 'none')
                . ' methods=' . implode(',', array_keys($storage->methods)),
            );
        }
    }

    /**
     * The version stubs add methods to classes Reflection.phpstub already describes. Analysing at
     * 8.4 must see both halves, and the template the class was declared with.
     */
    public function testReflectionClassKeepsWhatEveryStubAddsToIt(): void
    {
        $this->project_analyzer->setPhpVersion('8.4', 'tests');

        $codebase = $this->project_analyzer->getCodebase();
        $codebase->config->visitPreloadedStubFiles($codebase);
        $codebase->config->visitStubFiles($codebase);

        $storage = $codebase->classlike_storage_provider->get('ReflectionClass');

        $this->assertTrue(
            isset($storage->methods['newlazyghost']),
            'stubs/Php84.phpstub adds newLazyGhost();'
            . ' file=' . ($storage->location?->file_path ?? 'none')
            . ' templates=' . implode(',', array_keys($storage->template_types ?? []))
            . ' methods=' . implode(',', array_keys($storage->methods)),
        );

        $this->assertSame(
            ['T'],
            array_keys($storage->template_types ?? []),
            'and the class keeps the template every stub declares it with',
        );

        $this->assertTrue(
            isset($storage->methods['getattributes']),
            'while keeping what stubs/Reflection.phpstub declared',
        );
    }
}
