<?php

declare(strict_types=1);

namespace Psalm\Internal\Analyzer\Statements\Expression\Call\Method;

use Psalm\CodeLocation;
use Psalm\Context;
use Psalm\Internal\Analyzer\ClassLikeAnalyzer;
use Psalm\Internal\Analyzer\TraitAnalyzer;
use Psalm\Internal\Codebase\InternalCallMapHandler;
use Psalm\Internal\Interner;
use Psalm\Internal\MethodIdentifier;
use Psalm\Issue\InaccessibleMethod;
use Psalm\IssueBuffer;
use Psalm\StatementsSource;
use UnexpectedValueException;

use function array_pop;
use function end;
use function strtolower;

/**
 * @internal
 */
final class MethodVisibilityAnalyzer
{
    /**
     * @param  array<array-key, string>         $suppressed_issues
     * @return false|null
     */
    public static function analyze(
        MethodIdentifier $method_id,
        Context $context,
        StatementsSource $source,
        CodeLocation $code_location,
        array $suppressed_issues,
    ): ?bool {
        $codebase = $source->getCodebase();
        $codebase_methods = $codebase->methods;
        $codebase_classlikes = $codebase->classlikes;

        $fq_classlike_name = Interner::lookup($method_id->class_id);
        $method_name = Interner::lookupLc($method_id->name_id);

        $with_pseudo = true;

        if ($codebase_methods->visibility_provider->has($fq_classlike_name)) {
            $method_visible = $codebase_methods->visibility_provider->isMethodVisible(
                $source,
                $fq_classlike_name,
                $method_name,
                $context,
                $code_location,
            );

            if ($method_visible === false) {
                if (IssueBuffer::accepts(
                    new InaccessibleMethod(
                        'Cannot access method ' . $codebase_methods->getCasedMethodId($method_id) .
                            ' from context ' . Interner::lookupOrNull($context->self),
                        $code_location,
                    ),
                    $suppressed_issues,
                )) {
                    return false;
                }
            } elseif ($method_visible === true) {
                return false;
            }
        }

        $declaring_method_id = $codebase_methods->getDeclaringMethodId($method_id, $with_pseudo);

        if (!$declaring_method_id) {
            if ($method_name === '__construct'
                || (Interner::lookup($method_id->class_id) === 'Closure'
                    && (Interner::lookupLc($method_id->name_id) === 'fromcallable'
                        || Interner::lookupLc($method_id->name_id) === '__invoke'))
            ) {
                return null;
            }

            if (InternalCallMapHandler::inCallMap((string) $method_id)) {
                return null;
            }

            throw new UnexpectedValueException('$declaring_method_id not expected to be null here');
        }

        $appearing_method_id = $codebase_methods->getAppearingMethodId($method_id);

        $appearing_method_class = null;
        $appearing_class_storage = null;
        $appearing_method_name = null;

        if ($appearing_method_id) {
            $appearing_method_class = Interner::lookup($appearing_method_id->class_id);
            $appearing_method_name = Interner::lookupLc($appearing_method_id->name_id);

            // if the calling class is the same, we know the method exists, so it must be visible
            if ($appearing_method_class === Interner::lookupOrNull($context->self)) {
                return null;
            }

            $appearing_class_storage = $codebase->classlike_storage_provider->get(Interner::intern($appearing_method_class));
        }

        $declaring_method_class = Interner::lookup($declaring_method_id->class_id);

        if ($source->getSource() instanceof TraitAnalyzer
            && strtolower($declaring_method_class) === strtolower((string) $source->getFQCLN())
        ) {
            return null;
        }

        $storage = $codebase->methods->getStorage($declaring_method_id, $with_pseudo);
        $visibility = $storage->visibility;

        if ($appearing_method_name
            && isset($appearing_class_storage->trait_visibility_map[$appearing_method_name])
        ) {
            $visibility = $appearing_class_storage->trait_visibility_map[$appearing_method_name];
        }

        // Get oldest ancestor declaring $method_id
        $overridden_method_ids = $codebase_methods->getOverriddenMethodIds($method_id);
        // Remove traits and interfaces
        while (($oldest_declaring_method_id = end($overridden_method_ids))
            && !$codebase_classlikes->hasFullyQualifiedClassName(
                Interner::lookup($oldest_declaring_method_id->class_id),
                null,
                $context,
            )
        ) {
            array_pop($overridden_method_ids);
        }
        if (empty($overridden_method_ids)) {
            // We prefer appearing method id over declaring method id because declaring method id could be a trait
            $oldest_ancestor_declaring_method_id = $appearing_method_id;
        } else {
            // Oldest ancestor is at end of array
            $oldest_ancestor_declaring_method_id = array_pop($overridden_method_ids);
        }
        $oldest_ancestor_declaring_method_class = (isset($oldest_ancestor_declaring_method_id->class_id) ? Interner::lookup($oldest_ancestor_declaring_method_id->class_id) : null);

        switch ($visibility) {
            case ClassLikeAnalyzer::VISIBILITY_PUBLIC:
                return null;

            case ClassLikeAnalyzer::VISIBILITY_PRIVATE:
                if (!($context->self !== null) || $appearing_method_class !== Interner::lookup($context->self)) {
                    if (IssueBuffer::accepts(
                        new InaccessibleMethod(
                            'Cannot access private method ' . $codebase_methods->getCasedMethodId($method_id) .
                                ' from context ' . Interner::lookupOrNull($context->self),
                            $code_location,
                        ),
                        $suppressed_issues,
                    )) {
                        return false;
                    }
                }

                return null;

            case ClassLikeAnalyzer::VISIBILITY_PROTECTED:
                if (!($context->self !== null)) {
                    if (IssueBuffer::accepts(
                        new InaccessibleMethod(
                            'Cannot access protected method ' . $method_id,
                            $code_location,
                        ),
                        $suppressed_issues,
                    )) {
                        return false;
                    }

                    return null;
                }

                if ($oldest_ancestor_declaring_method_class !== null
                    && $codebase_classlikes->classExtends(Interner::intern($oldest_ancestor_declaring_method_class), $context->self)
                ) {
                    return null;
                }

                if ($oldest_ancestor_declaring_method_class !== null
                    && !$codebase_classlikes->classExtends($context->self, Interner::intern($oldest_ancestor_declaring_method_class))
                    && !$codebase_classlikes->classExtends(Interner::intern($declaring_method_class), $context->self)
                ) {
                    if (IssueBuffer::accepts(
                        new InaccessibleMethod(
                            'Cannot access protected method ' . $codebase_methods->getCasedMethodId($method_id) .
                                ' from context ' . Interner::lookup($context->self),
                            $code_location,
                        ),
                        $suppressed_issues,
                    )) {
                        return false;
                    }
                }
        }

        return null;
    }
}
