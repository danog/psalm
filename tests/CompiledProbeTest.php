<?php

declare(strict_types=1);

namespace Psalm\Tests;

use Psalm\Context;
use Psalm\Internal\Codebase\InternalCallMapHandler;
use Psalm\Internal\MethodIdentifier;
use Psalm\Internal\Type\Comparator\CallableTypeComparator;
use Psalm\Type\Atomic\TClassString;
use Psalm\Type\Atomic\TLiteralString;
use Psalm\IssueBuffer;

use function json_encode;

use const JSON_THROW_ON_ERROR;


/**
 * Narrow probes for inference a compiled build gets wrong: each one names the single thing it
 * measures, and the whole set is reported at once so one run tells the whole story.
 */
final class CompiledProbeTest extends TestCase
{
    public function testEmptyArrayAndTemplateInference(): void
    {
        $file_path = self::$src_dir_path . 'somefile.php';

        $this->addFile(
            $file_path,
            '<?php
                /** @return array<string> */
                function getStrings(): array { return []; }

                /**
                 * @template TKey as array-key
                 * @template TValue
                 */
                final class Coll {
                    /** @param array<TKey, TValue> $items */
                    public function __construct(public array $items) {}
                }

                $empty = [];
                $strings = getStrings();
                $coll = new Coll([]);
                $filled = new Coll(["a" => 1]);
                $sorted = [];
                sort($sorted);
                $combined = array_combine(getStrings(), getStrings());',
        );

        $context = new Context();
        $this->analyzeFile($file_path, $context);

        $actual = [];

        foreach (['$empty', '$strings', '$coll', '$filled', '$sorted', '$combined'] as $var_id) {
            $actual[$var_id] = isset($context->vars_in_scope[$var_id])
                ? (string) $context->vars_in_scope[$var_id]
                : 'absent';
        }

        $this->assertSame(
            [
                '$empty' => 'array<never, never>',
                '$strings' => 'array<array-key, string>',
                '$coll' => 'Coll<never, never>',
                '$filled' => 'Coll<string, int>',
                '$sorted' => 'array<never, never>',
                '$combined' => 'array<string, string>|false',
            ],
            $actual,
        );
    }
    public function testIntersectionAndStubbedClasses(): void
    {
        $file_path = self::$src_dir_path . 'somefile2.php';

        $this->addFile(
            $file_path,
            '<?php
                interface IA {}
                interface IB { public function foo(): void; }

                function make(): IA { throw new RuntimeException("x"); }

                $a = make();
                $narrowed = null;

                if ($a instanceof IB) {
                    $narrowed = $a;
                }

                $iso = DateTime::ISO8601;
                $countable = new ArrayObject([1, 2]);',
        );

        $context = new Context();
        $this->analyzeFile($file_path, $context);

        $actual = [];

        foreach (['$narrowed', '$countable'] as $var_id) {
            $actual[$var_id] = isset($context->vars_in_scope[$var_id])
                ? (string) $context->vars_in_scope[$var_id]
                : 'absent';
        }

        $this->assertSame(
            [
                // $iso is only there to prove the constant resolves; its type differs between a
                // reflected DateTime (plain string) and a declared one (the literal)
                '$narrowed' => 'IA&IB|null',
                '$countable' => 'ArrayObject<int<0, 1>, int>',
            ],
            $actual,
        );
    }
    /**
     * A variable the try block definitely assigns is defined afterwards: the analyzer clears the
     * `possibly undefined from try` flag once it knows every catch leaves or the body completed.
     */
    public function testTryBlockAssignmentsAreDefinedAfterwards(): void
    {
        $file_path = self::$src_dir_path . 'somefile3.php';

        $this->addFile(
            $file_path,
            '<?php
                try {
                    $withFinally = "a";
                } finally {
                }

                try {
                    $withLeavingCatch = "b";
                } catch (Exception $e) {
                    throw new RuntimeException("x");
                }

                try {
                    $plain = "c";
                } catch (Exception $e) {
                }',
        );

        $context = new Context();
        $this->analyzeFile($file_path, $context);

        $actual = [];

        foreach (['$withFinally', '$withLeavingCatch', '$plain'] as $var_id) {
            $type = $context->vars_in_scope[$var_id] ?? null;
            $actual[$var_id] = $type === null
                ? 'absent'
                : ($type->possibly_undefined ? 'maybe' : 'defined')
                    . '/' . ($type->possibly_undefined_from_try ? 'from-try' : 'not-from-try');
        }

        $this->assertSame(
            [
                '$withFinally' => 'defined/not-from-try',
                '$withLeavingCatch' => 'defined/not-from-try',
                '$plain' => 'maybe/from-try',
            ],
            $actual,
        );
    }
    /**
     * A method reached through an intersection (`Type&FooType`) exists as surely as one on the class
     * itself: the analyzer looks the call up on every member of the intersection.
     */
    public function testIntersectionMethodsAreFound(): void
    {
        $file_path = self::$src_dir_path . 'somefile4.php';

        $this->addFile(
            $file_path,
            '<?php
                class Base {
                    /** @psalm-assert FooType $this */
                    public function assertFoo(): void {
                        if (!$this instanceof FooType) { throw new RuntimeException("x"); }
                    }

                    /** @psalm-assert BarType $this */
                    public function assertBar(): void {
                        if (!$this instanceof BarType) { throw new RuntimeException("x"); }
                    }
                }

                interface FooType { public function foo(): void; }
                interface BarType { public function bar(): void; }

                function takesBase(Base $t): void {
                    $t->assertFoo();
                    $t->assertBar();
                    $t->foo();
                    $t->bar();
                }

                interface Plain {}
                interface WithMethod { public function baz(): int; }

                function takesPlain(Plain $p): void {
                    if ($p instanceof WithMethod) {
                        $p->baz();
                    }
                }',
        );

        $this->project_analyzer->getConfig()->throw_exception = false;

        $this->analyzeFile($file_path, new Context());

        $issues = [];

        foreach (IssueBuffer::getIssuesData() as $file_issues) {
            foreach ($file_issues as $issue) {
                $issues[] = $issue->type . ': ' . $issue->message;
            }
        }

        $this->assertSame([], $issues);
    }
    /**
     * The description of a class PHP itself provides comes from the call map, not from whatever the
     * runtime happens to reflect: at analysis PHP 7.4 DateTime::format still returns `false|string`.
     */
    public function testInternalClassesAreDescribedByTheCallMap(): void
    {
        $file_path = self::$src_dir_path . 'somefile5.php';

        $this->addFile(
            $file_path,
            '<?php
                $datetime = new DateTime();
                $a = $datetime->format("Y-m-d");
                $b = $datetime->getTimestamp();',
        );

        $context = new Context();
        $this->analyzeFile($file_path, $context);

        $storage = $this->project_analyzer->getCodebase()->classlike_storage_provider->get('datetime');

        $actual = [
            'user_defined' => $storage->user_defined ? 'yes' : 'no',
            'format' => (string) ($storage->methods['format']->return_type ?? null),
            'getTimestamp' => (string) ($storage->methods['gettimestamp']->return_type ?? null),
            '$a' => (string) ($context->vars_in_scope['$a'] ?? null),
            '$b' => (string) ($context->vars_in_scope['$b'] ?? null),
        ];

        $this->assertSame(
            [
                'user_defined' => 'no',
                'format' => 'false|string',
                'getTimestamp' => 'false|int',
                '$a' => 'false|string',
                '$b' => 'false|int',
            ],
            $actual,
        );
    }
    /**
     * The call map follows the analysis PHP version: get_headers took an int `format` until 8.0 and
     * a bool `associative` from 8.0 on, so which one a test sees says which map was loaded.
     */
    public function testTheCallMapFollowsTheAnalysisVersion(): void
    {
        $actual = [];

        foreach (['7.4', '8.0'] as $version) {
            $this->project_analyzer->setPhpVersion($version, 'tests');

            $codebase = $this->project_analyzer->getCodebase();
            $callable = InternalCallMapHandler::getCallableFromCallMapById($codebase, 'get_headers', [], null);
            $second = ($callable->params ?? [])[1] ?? null;

            $actual[$version] = $codebase->getMajorAnalysisPhpVersion()
                . '.' . $codebase->getMinorAnalysisPhpVersion()
                . ' => ' . ($second === null ? 'absent' : $second->name . ':' . $second->type);
        }

        $this->assertSame(
            [
                '7.4' => '7.4 => format:int',
                '8.0' => '8.0 => associative:bool',
            ],
            $actual,
            json_encode($actual, JSON_THROW_ON_ERROR),
        );
    }
    /**
     * Three descriptions a compiled build has been getting wrong, measured directly rather than
     * through the issue a test happens to report.
     */
    public function testDescriptionsOfThingsPhpProvides(): void
    {
        $file_path = self::$src_dir_path . 'somefile6.php';

        $this->project_analyzer->setPhpVersion('8.0', 'tests');

        $this->addFile(
            $file_path,
            '<?php
                /** @param class-string<object&callable(string):void> $className */
                function takesCallableObject(string $className): void {}

                $filtered = array_filter(["a", ""], "strlen");',
        );

        $context = new Context();
        $this->project_analyzer->getConfig()->throw_exception = false;
        $this->analyzeFile($file_path, $context);

        $codebase = $this->project_analyzer->getCodebase();

        $strlen = CallableTypeComparator::getCallableFromAtomic($codebase, new TLiteralString('strlen'));
        $create = $codebase->methods->getStorage(new MethodIdentifier('DateTime', 'createfrominterface'));
        $param = $codebase->file_storage_provider->get($file_path)
            ->functions['takescallableobject']->params[0]->type;
        $class_string = $param?->getSingleAtomic();
        $as_type = $class_string instanceof TClassString ? $class_string->as_type : null;

        $this->assertSame(
            [
                'strlen_mutations' => 0,
                'createFromInterface' => 'static',
                // PHP keeps the intersection elsewhere than as_type; what matters is that a
                // compiled build says the same thing
                'class_string_as' => 'absent',
            ],
            [
                'strlen_mutations' => $strlen?->allowed_mutations ?? -1,
                'createFromInterface' => (string) $create->return_type,
                'class_string_as' => $as_type === null ? 'absent' : $as_type->getId(),
            ],
            json_encode([
                'strlen_mutations' => $strlen?->allowed_mutations ?? -1,
                'createFromInterface' => (string) $create->return_type,
                'class_string_as' => $as_type === null ? 'absent' : $as_type->getId(),
            ], JSON_THROW_ON_ERROR),
        );
    }
}
