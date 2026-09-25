<?php

declare(strict_types=1);

namespace Psalm\Internal\Analyzer;

use LogicException;
use PhpParser;
use Psalm\CodeLocation;
use Psalm\Codebase;
use Psalm\Context;
use Psalm\Internal\Codebase\InternalCallMapHandler;
use Psalm\Internal\Interner;
use Psalm\Internal\MethodIdentifier;
use Psalm\Issue\InvalidEnumMethod;
use Psalm\Issue\InvalidStaticInvocation;
use Psalm\Issue\MethodSignatureMustOmitReturnType;
use Psalm\Issue\NonStaticSelfCall;
use Psalm\Issue\UndefinedMagicMethod;
use Psalm\Issue\UndefinedMethod;
use Psalm\IssueBuffer;
use Psalm\StatementsSource;
use Psalm\Storage\ClassLikeStorage;
use Psalm\Storage\MethodStorage;
use Psalm\Storage\UnserializeMemoryUsageSuppressionTrait;
use UnexpectedValueException;

use function in_array;
use function strtolower;

/**
 * @internal
 * @extends FunctionLikeAnalyzer<PhpParser\Node\Stmt\ClassMethod>
 */
final class MethodAnalyzer extends FunctionLikeAnalyzer
{
    use UnserializeMemoryUsageSuppressionTrait;
    // https://github.com/php/php-src/blob/a83923044c48982c80804ae1b45e761c271966d3/Zend/zend_enum.c#L77-L95
    private const FORBIDDEN_ENUM_METHODS = [
        '__construct',
        '__destruct',
        '__clone',
        '__get',
        '__set',
        '__unset',
        '__isset',
        '__tostring',
        '__debuginfo',
        '__serialize',
        '__unserialize',
        '__sleep',
        '__wakeup',
        '__set_state',
    ];

    /**
     * @psalm-mutation-free
     */
    public function __construct(
        PhpParser\Node\Stmt\ClassMethod $function,
        SourceAnalyzer $source,
        ?MethodStorage $storage = null,
    ) {
        $codebase = $source->getCodebase();

        $method_name_lc = strtolower((string) $function->name);

        $source_fqcln = (string) $source->getFQCLN();

        $source_fqcln_lc = strtolower($source_fqcln);

        $method_id = new MethodIdentifier(Interner::intern($source_fqcln), Interner::intern($method_name_lc));

        if (!$storage) {
            try {
                $storage = $codebase->methods->getStorage($method_id);
            } catch (UnexpectedValueException $e) {
                $class_storage = $codebase->classlike_storage_provider->get(Interner::intern($source_fqcln_lc));

                if (!$class_storage->parent_classes) {
                    throw $e;
                }

                $declaring_method_id = $codebase->methods->getDeclaringMethodId($method_id);

                if (!$declaring_method_id) {
                    throw $e;
                }

                // happens for fake constructors
                $storage = $codebase->methods->getStorage($declaring_method_id);
            }
        }

        parent::__construct($function, $source, $storage);
    }

    /**
     * Determines whether a given method is static or not
     *
     * @param  array<array-key, string>   $suppressed_issues
     */
    public static function checkStatic(
        MethodIdentifier $method_id,
        bool $self_call,
        bool $is_context_dynamic,
        Codebase $codebase,
        CodeLocation $code_location,
        array $suppressed_issues,
        ?bool &$is_dynamic_this_method = false,
    ): void {
        $codebase_methods = $codebase->methods;

        if (Interner::lookup($method_id->class_id) === 'Closure'
            && Interner::lookupLc($method_id->name_id) === 'fromcallable'
        ) {
            return;
        }

        $original_method_id = $method_id;
        $with_pseudo = true;

        $method_id = $codebase_methods->getDeclaringMethodId($method_id, $with_pseudo);

        if (!$method_id) {
            if (InternalCallMapHandler::inCallMap((string) $original_method_id)) {
                return;
            }

            throw new LogicException('Declaring method for ' . $original_method_id . ' should not be null');
        }

        $storage = $codebase_methods->getStorage($method_id, $with_pseudo);

        if (!$storage->is_static) {
            if ($self_call) {
                if (!$is_context_dynamic) {
                    if (IssueBuffer::accepts(
                        new NonStaticSelfCall(
                            'Method ' . $codebase_methods->getCasedMethodId($method_id) .
                                ' is not static, but is called ' .
                                'using self::',
                            $code_location,
                        ),
                        $suppressed_issues,
                    )) {
                        return;
                    }
                } else {
                    $is_dynamic_this_method = true;
                }
            } else {
                if (IssueBuffer::accepts(
                    new InvalidStaticInvocation(
                        'Method ' . $codebase_methods->getCasedMethodId($method_id) .
                            ' is not static, but is called ' .
                            'statically',
                        $code_location,
                    ),
                    $suppressed_issues,
                )) {
                    return;
                }
            }
        }
    }

    /**
     * @param  array<array-key, string>     $suppressed_issues
     * @param  lowercase-string|null  $calling_method_id
     */
    public static function checkMethodExists(
        Codebase $codebase,
        MethodIdentifier $method_id,
        CodeLocation $code_location,
        array $suppressed_issues,
        ?string $calling_method_id = null,
        bool $with_pseudo = false,
        ?string $written_name = null,
    ): ?bool {
        if ($codebase->methodExists(
            method_id: $method_id,
            calling_method_id: $calling_method_id,
            code_location: !$calling_method_id
                || $calling_method_id !== strtolower((string) $method_id)
                ? $code_location
                : null,
            source_file_path: $code_location->file_path,
            with_pseudo: $with_pseudo,
        )) {
            // pzoom resolves method names case-sensitively: a call written with another casing is undefined
            $declared = $written_name !== null ? self::declaredCasingOf($codebase, $method_id, $written_name) : null;
            if ($declared === null) {
                return true;
            }

            if (IssueBuffer::accepts(
                new UndefinedMethod(
                    'Method ' . Interner::lookup($method_id->class_id) . '::' . $written_name
                        . ' does not exist (incorrect casing of ' . $declared . ')',
                    $code_location,
                    (string) $method_id,
                ),
                $suppressed_issues,
            )) {
                return false;
            }

            return null;
        }

        if ($with_pseudo) {
            if (IssueBuffer::accepts(
                new UndefinedMagicMethod(
                    'Magic method ' . $method_id . ' does not exist',
                    $code_location,
                    (string) $method_id,
                ),
                $suppressed_issues,
            )) {
                return false;
            }
        } else {
            if (IssueBuffer::accepts(
                new UndefinedMethod('Method ' . $method_id . ' does not exist', $code_location, (string) $method_id),
                $suppressed_issues,
            )) {
                return false;
            }
        }

        return null;
    }

    /**
     * The declared spelling of a method that exists but is written with another casing (pzoom's casing
     * hint), or null when the spelling matches or nothing is declared to compare against.
     *
     * @psalm-mutation-free
     */
    public static function declaredCasingOf(
        Codebase $codebase,
        MethodIdentifier $method_id,
        string $written_name,
    ): ?string {
        $declaring_method_id = $codebase->methods->getDeclaringMethodId($method_id, true);
        if ($declaring_method_id === null) {
            return null;
        }
        try {
            $declared = $codebase->methods->getStorage($declaring_method_id, true)->cased_name;
        } catch (UnexpectedValueException) {
            return null;
        }
        if ($declared === null || $declared === $written_name || strtolower($declared) !== strtolower($written_name)) {
            return null;
        }
        return $declared;
    }

    public static function isMethodVisible(
        MethodIdentifier $method_id,
        Context $context,
        StatementsSource $source,
    ): bool {
        $codebase = $source->getCodebase();

        $fq_classlike_name = Interner::lookup($method_id->class_id);
        $method_name = Interner::lookupLc($method_id->name_id);

        if ($codebase->methods->visibility_provider->has($fq_classlike_name)) {
            $method_visible = $codebase->methods->visibility_provider->isMethodVisible(
                $source,
                $fq_classlike_name,
                $method_name,
                $context,
                null,
            );

            if ($method_visible !== null) {
                return $method_visible;
            }
        }

        $declaring_method_id = $codebase->methods->getDeclaringMethodId($method_id);

        if (!$declaring_method_id) {
            // this can happen for methods in the callmap that were not reflected
            return true;
        }

        $appearing_method_id = $codebase->methods->getAppearingMethodId($method_id);

        $appearing_method_class = null;

        if ($appearing_method_id) {
            $appearing_method_class = Interner::lookup($appearing_method_id->class_id);

            // if the calling class is the same, we know the method exists, so it must be visible
            if ($appearing_method_class === Interner::lookupOrNull($context->self)) {
                return true;
            }
        }

        $declaring_method_class = Interner::lookup($declaring_method_id->class_id);

        if ($source->getSource() instanceof TraitAnalyzer
            && strtolower($declaring_method_class) === strtolower((string) $source->getFQCLN())
        ) {
            return true;
        }

        $storage = $codebase->methods->getStorage($declaring_method_id);

        switch ($storage->visibility) {
            case ClassLikeAnalyzer::VISIBILITY_PUBLIC:
                return true;

            case ClassLikeAnalyzer::VISIBILITY_PRIVATE:
                return ($context->self !== null) && $appearing_method_class === Interner::lookup($context->self);

            case ClassLikeAnalyzer::VISIBILITY_PROTECTED:
                if (!($context->self !== null)) {
                    return false;
                }

                if ($appearing_method_class
                    && $codebase->classExtends(Interner::intern($appearing_method_class), $context->self)
                ) {
                    return true;
                }

                if ($appearing_method_class
                    && !$codebase->classExtends($context->self, Interner::intern($appearing_method_class))
                ) {
                    return false;
                }
        }

        return true;
    }

    /**
     * Check that __clone, __construct, and __destruct do not have a return type
     * hint in their signature.
     */
    public static function checkMethodSignatureMustOmitReturnType(
        MethodStorage $method_storage,
        CodeLocation $code_location,
    ): void {
        if ($method_storage->signature_return_type === null) {
            return;
        }

        if ($method_storage->cased_name === null) {
            return;
        }

        $method_name_lc = strtolower($method_storage->cased_name);
        $methodsOfInterest = ['__clone', '__construct', '__destruct'];

        if (in_array($method_name_lc, $methodsOfInterest, true)) {
            IssueBuffer::maybeAdd(
                new MethodSignatureMustOmitReturnType(
                    'Method ' . $method_storage->cased_name . ' must not declare a return type',
                    $code_location,
                ),
            );
        }
    }

    /**
     * @psalm-mutation-free
     */
    public function getMethodId(?int $context_self = null): MethodIdentifier
    {
        $function_name = (string)$this->function->name;

        return new MethodIdentifier(
            Interner::intern((Interner::lookupOrNull($context_self) ?? (string) $this->source->getFQCLN())),
            Interner::intern(strtolower($function_name)),
        );
    }

    public static function checkForbiddenEnumMethod(MethodStorage $method_storage, ClassLikeStorage $enum_storage): void
    {
        if ($method_storage->cased_name === null || $method_storage->location === null) {
            return;
        }

        $method_name_lc = strtolower($method_storage->cased_name);
        if (in_array($method_name_lc, self::FORBIDDEN_ENUM_METHODS, true)) {
            IssueBuffer::maybeAdd(new InvalidEnumMethod(
                'Enums cannot define ' . $method_storage->cased_name,
                $method_storage->location,
                $method_storage->defining_fqcln . '::' . $method_storage->cased_name,
            ));
        }

        if ($method_name_lc === 'cases') {
            IssueBuffer::maybeAdd(new InvalidEnumMethod(
                'Enums cannot define ' . $method_storage->cased_name,
                $method_storage->location,
                $method_storage->defining_fqcln . '::' . $method_storage->cased_name,
            ));
        }

        if ($enum_storage->enum_type && ($method_name_lc === 'from' || $method_name_lc === 'tryfrom')) {
            IssueBuffer::maybeAdd(new InvalidEnumMethod(
                'Enums cannot define ' . $method_storage->cased_name,
                $method_storage->location,
                $method_storage->defining_fqcln . '::' . $method_storage->cased_name,
            ));
        }
    }
}
