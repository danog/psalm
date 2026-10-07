<?php

declare(strict_types=1);

namespace Psalm\Internal\Type;

use Psalm\Codebase;
use Psalm\Exception\CircularReferenceException;
use Psalm\Exception\UnresolvableConstantException;
use Psalm\Internal\Analyzer\Statements\Expression\Fetch\AtomicPropertyFetchAnalyzer;
use Psalm\Storage\Assertion\IsType;
use Psalm\Storage\Capabilities;
use Psalm\Type;
use Psalm\Type\Atomic;
use Psalm\Type\Atomic\TArray;
use Psalm\Type\Atomic\TCallable;
use Psalm\Type\Atomic\TClassConstant;
use Psalm\Type\Atomic\TClassString;
use Psalm\Type\Atomic\TClosure;
use Psalm\Type\Atomic\TConditional;
use Psalm\Type\Atomic\TEnumCase;
use Psalm\Type\Atomic\TGenericObject;
use Psalm\Type\Atomic\TInt;
use Psalm\Type\Atomic\TIntMask;
use Psalm\Type\Atomic\TIntMaskOf;
use Psalm\Type\Atomic\TIterable;
use Psalm\Type\Atomic\TKeyOf;
use Psalm\Type\Atomic\TKeyedArray;
use Psalm\Type\Atomic\TLiteralClassString;
use Psalm\Type\Atomic\TLiteralInt;
use Psalm\Type\Atomic\TMixed;
use Psalm\Type\Atomic\TClassStringMap;
use Psalm\Type\Atomic\TNamedObject;
use Psalm\Type\Atomic\TNever;
use Psalm\Type\Atomic\TNull;
use Psalm\Type\Atomic\TObjectWithProperties;
use Psalm\Type\Atomic\TPropertiesOf;
use Psalm\Type\Atomic\TTemplateParam;
use Psalm\Type\Atomic\TTypeAlias;
use Psalm\Type\Atomic\TValueOf;
use Psalm\Type\Atomic\TVoid;
use Psalm\Type\Union;
use ReflectionProperty;

use function array_any;
use function array_filter;
use function array_map;
use function array_merge;
use function array_values;
use function count;
use function is_string;
use function reset;
use function strtolower;

/**
 * @internal
 */
final class TypeExpander
{
    public static function expandUnion(
        Codebase $codebase,
        Union $return_type,
        ?string $self_class,
        string|TNamedObject|TTemplateParam|null $static_class_type,
        ?string $parent_class,
        bool $evaluate_class_constants = true,
        bool $evaluate_conditional_types = false,
        bool $final = false,
        bool $expand_generic = false,
        bool $expand_templates = false,
        bool $throw_on_unresolvable_constant = false,
    ): Union {
        $new_return_type_parts = [];
        // every expansion step returns the atomic it was given when there is nothing to resolve, so a union
        // whose atomics all came back unchanged is returned as it is: no recombination (pzoom expands in
        // place and only touches what needs expanding)
        $changed = false;

        foreach ($return_type->getAtomicTypes() as $return_type_part) {
            // expandAtomic takes its atomic by reference and may replace it: keep the original for the comparison
            $original_part = $return_type_part;
            $parts = self::expandAtomic(
                $codebase,
                $return_type_part,
                $self_class,
                $static_class_type,
                $parent_class,
                $evaluate_class_constants,
                $evaluate_conditional_types,
                $final,
                $expand_generic,
                $expand_templates,
                $throw_on_unresolvable_constant,
            );

            if (count($parts) !== 1 || $parts[0] !== $original_part) {
                $changed = true;
            }

            $new_return_type_parts = [...$new_return_type_parts, ...$parts];
        }

        if (!$changed && self::isCombineNormal($new_return_type_parts)) {
            // the input itself when the fresh union below would be identical to it -- it carries none of the flags
            // that union drops, and no null is to be moved last: no new union, and its memoized id is kept
            if (!$return_type->from_calculation
                && $return_type->initialized_class === null
                && !$return_type->checked
                && !$return_type->failed_reconciliation
                && !$return_type->ignore_isset
                && !$return_type->from_template_default
                && !$return_type->reference_free
                && $return_type->allow_mutations
                && $return_type->has_mutations
                && !$return_type->propagate_parent_nodes
                && !$return_type->different
                && !(count($new_return_type_parts) === 2 && $new_return_type_parts[0] instanceof TNull)
            ) {
                return $return_type;
            }
            // nothing to resolve and nothing the combiner would merge: the same atomics in a fresh union carrying
            // the flags an expansion keeps (below), exactly what recombining them produced, without the combiner
            // (pzoom expands in place). The combiner lists null after the other atomic.
            if (count($new_return_type_parts) === 2 && $new_return_type_parts[0] instanceof TNull) {
                $new_return_type_parts = [$new_return_type_parts[1], $new_return_type_parts[0]];
            }
            // (an explicit from_docblock is exact: an expansion keeps the original union's)
            return new Union($new_return_type_parts, [
                'from_docblock' => $return_type->from_docblock,
                'ignore_nullable_issues' => $return_type->ignore_nullable_issues,
                'ignore_falsable_issues' => $return_type->ignore_falsable_issues,
                'possibly_undefined' => $return_type->possibly_undefined,
                'possibly_undefined_from_try' => $return_type->possibly_undefined_from_try,
                'by_ref' => $return_type->by_ref,
                'initialized' => $return_type->initialized,
                'from_property' => $return_type->from_property,
                'from_static_property' => $return_type->from_static_property,
                'from_global_state' => $return_type->from_global_state,
                'explicit_never' => $return_type->explicit_never,
                'had_template' => $return_type->had_template,
                'parent_nodes' => $return_type->parent_nodes,
            ]);
        }

        return TypeCombiner::combine(
            $new_return_type_parts,
            $codebase,
            properties: [
                'from_docblock' => $return_type->from_docblock,
                'ignore_nullable_issues' => $return_type->ignore_nullable_issues,
                'ignore_falsable_issues' => $return_type->ignore_falsable_issues,
                'possibly_undefined' => $return_type->possibly_undefined,
                'possibly_undefined_from_try' => $return_type->possibly_undefined_from_try,
                'by_ref' => $return_type->by_ref,
                'initialized' => $return_type->initialized,
                'from_property' => $return_type->from_property,
                'from_static_property' => $return_type->from_static_property,
                'from_global_state' => $return_type->from_global_state,
                'explicit_never' => $return_type->explicit_never,
                'had_template' => $return_type->had_template,
                'parent_nodes' => $return_type->parent_nodes,
            ],
        );
    }

    /**
     * Whether combining these atomics would give them back as they are: a single atomic, or one atomic and
     * null. Anything else (two objects, scalars and literals, two arrays) the combiner may merge.
     *
     * @param list<Atomic> $atomics
     * @psalm-pure
     */
    private static function isCombineNormal(array $atomics): bool
    {
        if (count($atomics) === 1) {
            return self::isCombineNormalAtomic($atomics[0]);
        }

        if (count($atomics) === 2) {
            $a = $atomics[0];
            $b = $atomics[1];

            return ($a instanceof TNull xor $b instanceof TNull)
                && self::isCombineNormalAtomic($a instanceof TNull ? $b : $a);
        }

        return false;
    }

    /**
     * An atomic the combiner hands back as it is. A container is rebuilt by the combiner (its type parameters
     * pass through the combiner again, which resets what they carry from a docblock), so it still combines.
     *
     * @psalm-pure
     */
    private static function isCombineNormalAtomic(Atomic $atomic): bool
    {
        return !$atomic instanceof TNull
            && !$atomic instanceof TMixed
            && !$atomic instanceof TNever
            && !$atomic instanceof TArray
            && !$atomic instanceof TKeyedArray
            && !$atomic instanceof TIterable
            && !$atomic instanceof TGenericObject
            && !$atomic instanceof TCallable
            && !$atomic instanceof TClosure
            && !$atomic instanceof TObjectWithProperties
            && !$atomic instanceof TClassStringMap;
    }

    /**
     * @param-out Atomic $return_type
     * @return non-empty-list<Atomic>
     * @psalm-suppress ConflictingReferenceConstraint, ReferenceConstraintViolation The output type is always Atomic
     * @psalm-suppress ComplexMethod
     */
    public static function expandAtomic(
        Codebase $codebase,
        Atomic &$return_type,
        ?string $self_class,
        string|TNamedObject|TTemplateParam|null $static_class_type,
        ?string $parent_class,
        bool $evaluate_class_constants = true,
        bool $evaluate_conditional_types = false,
        bool $final = false,
        bool $expand_generic = false,
        bool $expand_templates = false,
        bool $throw_on_unresolvable_constant = false,
    ): array {
        if ($return_type instanceof TEnumCase) {
            return [$return_type];
        }

        if ($return_type instanceof TNamedObject
            || $return_type instanceof TTemplateParam
        ) {
            if ($return_type->extra_types) {
                $new_intersection_types = [];

                $extra_types = [];
                foreach ($return_type->extra_types as $extra_type) {
                    self::expandAtomic(
                        $codebase,
                        $extra_type,
                        $self_class,
                        $static_class_type,
                        $parent_class,
                        $evaluate_class_constants,
                        $evaluate_conditional_types,
                        $expand_generic,
                        $expand_templates,
                        $throw_on_unresolvable_constant,
                    );

                    if ($extra_type instanceof TNamedObject && $extra_type->extra_types) {
                        $new_intersection_types = [...$new_intersection_types, ...$extra_type->extra_types];
                        $extra_type = $extra_type->setIntersectionTypes([]);
                    }
                    $extra_types[$extra_type->getKey()] = $extra_type;
                }

                /** @psalm-suppress ArgumentTypeCoercion */
                $return_type = $return_type->setIntersectionTypes(array_merge($extra_types, $new_intersection_types));
            }

            if ($return_type instanceof TNamedObject) {
                $return_type = self::expandNamedObject(
                    $codebase,
                    $return_type,
                    $self_class,
                    $static_class_type,
                    $parent_class,
                    $final,
                    $expand_generic,
                );
            }
        }

        if ($return_type instanceof TClassString
            && $return_type->as_type
        ) {
            $new_as_type = $return_type->as_type;

            self::expandAtomic(
                $codebase,
                $new_as_type,
                $self_class,
                $static_class_type,
                $parent_class,
                $evaluate_class_constants,
                $evaluate_conditional_types,
                $final,
                $expand_generic,
                $expand_templates,
                $throw_on_unresolvable_constant,
            );

            if ($new_as_type instanceof TNamedObject && $new_as_type !== $return_type->as_type) {
                $return_type = $return_type->setAs(
                    $new_as_type->value,
                    $new_as_type,
                );
            }
        } elseif ($return_type instanceof TTemplateParam) {
            $new_as_type = self::expandUnion(
                $codebase,
                $return_type->as,
                $self_class,
                $static_class_type,
                $parent_class,
                $evaluate_class_constants,
                $evaluate_conditional_types,
                $final,
                $expand_generic,
                $expand_templates,
                $throw_on_unresolvable_constant,
            );

            if ($expand_templates) {
                return $new_as_type->getAtomicTypes();
            }

            $return_type = $return_type->replaceAs($new_as_type);
        }

        if ($return_type instanceof TClassConstant) {
            if ($self_class) {
                $return_type = $return_type->replaceClassLike(
                    'self',
                    $self_class,
                );
            }
            if (is_string($static_class_type) || $self_class) {
                $return_type = $return_type->replaceClassLike(
                    'static',
                    is_string($static_class_type) ? $static_class_type : $self_class,
                );
            }

            if ($evaluate_class_constants && $codebase->classOrInterfaceOrEnumExists($return_type->fq_classlike_name)) {
                if (strtolower($return_type->const_name) === 'class') {
                    return [new TLiteralClassString($return_type->fq_classlike_name)];
                }

                try {
                    $class_constant = $codebase->classlikes->getClassConstantType(
                        $return_type->fq_classlike_name,
                        $return_type->const_name,
                        ReflectionProperty::IS_PRIVATE,
                    );
                } catch (CircularReferenceException) {
                    $class_constant = null;
                }

                if ($class_constant) {
                    return $class_constant->getAtomicTypes();
                }
            }

            return [$return_type];
        }

        if ($return_type instanceof TPropertiesOf) {
            return self::expandPropertiesOf(
                $codebase,
                $return_type,
                $self_class,
                $static_class_type,
            );
        }

        if ($return_type instanceof TTypeAlias) {
            $declaring_fq_classlike_name = $return_type->declaring_fq_classlike_name;

            if ($declaring_fq_classlike_name === 'self' && $self_class) {
                $declaring_fq_classlike_name = $self_class;
            }

            if (!($evaluate_class_constants
                && $codebase->classlikes->doesClassLikeExist(strtolower($declaring_fq_classlike_name))
            )) {
                return [$return_type];
            }

            $class_storage = ($codebase->classlike_storage_provider->getOrNull($declaring_fq_classlike_name) ?? throw \Psalm\Internal\Provider\ClassLikeStorageProvider::missing($declaring_fq_classlike_name));

            $type_alias_name = $return_type->alias_name;

            if (!isset($class_storage->type_aliases[$type_alias_name])) {
                return [$return_type];
            }

            $resolved_type_alias = $class_storage->type_aliases[$type_alias_name];
            $replacement_atomic_types = $resolved_type_alias->replacement_atomic_types;

            if (!$replacement_atomic_types) {
                return [$return_type];
            }

            $recursively_fleshed_out_types = [];
            foreach ($replacement_atomic_types as $replacement_atomic_type) {
                $more_recursively_fleshed_out_types = self::expandAtomic(
                    $codebase,
                    $replacement_atomic_type,
                    $self_class,
                    $static_class_type,
                    $parent_class,
                    $evaluate_class_constants,
                    $evaluate_conditional_types,
                    $final,
                    $expand_generic,
                    $expand_templates,
                    $throw_on_unresolvable_constant,
                );

                $recursively_fleshed_out_types = [
                    ...$more_recursively_fleshed_out_types,
                    ...$recursively_fleshed_out_types,
                ];
            }

            return $recursively_fleshed_out_types;
        }

        if ($return_type instanceof TKeyOf
            || $return_type instanceof TValueOf
        ) {
            return self::expandKeyOfValueOf(
                $codebase,
                $return_type,
                $self_class,
                $static_class_type,
                $parent_class,
                $evaluate_class_constants,
                $evaluate_conditional_types,
                $final,
                $expand_generic,
                $expand_templates,
                $throw_on_unresolvable_constant,
            );
        }

        if ($return_type instanceof TIntMask) {
            if (!$evaluate_class_constants) {
                return [new TInt()];
            }

            $potential_ints = [];

            foreach ($return_type->values as $value_type) {
                $new_value_type = self::expandAtomic(
                    $codebase,
                    $value_type,
                    $self_class,
                    $static_class_type,
                    $parent_class,
                    $evaluate_class_constants,
                    $evaluate_conditional_types,
                    $final,
                    $expand_generic,
                    $expand_templates,
                    $throw_on_unresolvable_constant,
                );

                $new_value_type = reset($new_value_type);

                if (!$new_value_type instanceof TLiteralInt) {
                    return [new TInt()];
                }

                $potential_ints[] = $new_value_type->value;
            }

            return TypeParser::getComputedIntsFromMask($potential_ints);
        }

        if ($return_type instanceof TIntMaskOf) {
            if (!$evaluate_class_constants) {
                return [new TInt()];
            }

            $value_type = $return_type->value;

            $new_value_types = self::expandAtomic(
                $codebase,
                $value_type,
                $self_class,
                $static_class_type,
                $parent_class,
                $evaluate_class_constants,
                $evaluate_conditional_types,
                $final,
                $expand_generic,
                $expand_templates,
                $throw_on_unresolvable_constant,
            );

            $potential_ints = [];

            foreach ($new_value_types as $new_value_type) {
                if (!$new_value_type instanceof TLiteralInt) {
                    return [new TInt()];
                }

                $potential_ints[] = $new_value_type->value;
            }

            return TypeParser::getComputedIntsFromMask($potential_ints);
        }

        if ($return_type instanceof TConditional) {
            return self::expandConditional(
                $codebase,
                $return_type,
                $self_class,
                $static_class_type,
                $parent_class,
                $evaluate_class_constants,
                $evaluate_conditional_types,
                $final,
                $expand_generic,
                $expand_templates,
                $throw_on_unresolvable_constant,
            );
        }


        if ($return_type instanceof TArray
            || $return_type instanceof TGenericObject
            || $return_type instanceof TIterable
        ) {
            $type_params = $return_type->type_params;
            foreach ($type_params as &$type_param) {
                $type_param = self::expandUnion(
                    $codebase,
                    $type_param,
                    $self_class,
                    $static_class_type,
                    $parent_class,
                    $evaluate_class_constants,
                    $evaluate_conditional_types,
                    $final,
                    $expand_generic,
                    $expand_templates,
                    $throw_on_unresolvable_constant,
                );
            }
            unset($type_param);

            // `Foo[pure]` for a class with type templates
            if ($return_type instanceof TGenericObject
                && $codebase->classlike_storage_provider->has($return_type->value)
            ) {
                $type_params = PurityArguments::align(
                    $type_params,
                    ($codebase->classlike_storage_provider->getOrNull($return_type->value) ?? throw \Psalm\Internal\Provider\ClassLikeStorageProvider::missing($return_type->value)),
                );
            }

            /** @psalm-suppress InvalidArgument Psalm bug */
            $return_type = $return_type->setTypeParams($type_params);

            // the purity of an iterable may name a type alias (`iterable[Storage]`)
            if ($return_type instanceof TIterable) {
                $return_type = $return_type->setPurity(self::expandUnion(
                    $codebase,
                    $return_type->purity,
                    $self_class,
                    $static_class_type,
                    $parent_class,
                    $evaluate_class_constants,
                    $evaluate_conditional_types,
                    $final,
                    $expand_generic,
                    $expand_templates,
                    $throw_on_unresolvable_constant,
                ));
            }
        } elseif ($return_type instanceof TKeyedArray) {
            $properties = $return_type->properties;
            $changed = false;
            foreach ($properties as $k => $property_type) {
                $property_type = self::expandUnion(
                    $codebase,
                    $property_type,
                    $self_class,
                    $static_class_type,
                    $parent_class,
                    $evaluate_class_constants,
                    $evaluate_conditional_types,
                    $final,
                    $expand_generic,
                    $expand_templates,
                    $throw_on_unresolvable_constant,
                );
                if ($property_type !== $properties[$k]) {
                    $changed = true;
                    $properties[$k] = $property_type;
                }
            }
            unset($property_type);
            $fallback_params = $return_type->fallback_params;
            if ($fallback_params) {
                foreach ($fallback_params as $k => $property_type) {
                    $property_type = self::expandUnion(
                        $codebase,
                        $property_type,
                        $self_class,
                        $static_class_type,
                        $parent_class,
                        $evaluate_class_constants,
                        $evaluate_conditional_types,
                        $final,
                        $expand_generic,
                        $expand_templates,
                        $throw_on_unresolvable_constant,
                    );
                    if ($property_type !== $fallback_params[$k]) {
                        $changed = true;
                        $fallback_params[$k] = $property_type;
                    }
                }
                unset($property_type);
            }
            if ($changed) {
                $return_type = TKeyedArray::make(
                    $properties,
                    $return_type->class_strings,
                    $fallback_params,
                    $return_type->is_list,
                    $return_type->from_docblock,
                );
            }
        }

        if ($return_type instanceof TObjectWithProperties) {
            $properties = $return_type->properties;
            foreach ($properties as &$property_type) {
                $property_type = self::expandUnion(
                    $codebase,
                    $property_type,
                    $self_class,
                    $static_class_type,
                    $parent_class,
                    $evaluate_class_constants,
                    $evaluate_conditional_types,
                    $final,
                    $expand_generic,
                    $expand_templates,
                    $throw_on_unresolvable_constant,
                );
            }
            unset($property_type);
            $return_type = $return_type->setProperties($properties);
        }

        if ($return_type instanceof TCallable
            || $return_type instanceof TClosure
        ) {
            $params = $return_type->params;
            if ($params) {
                foreach ($params as &$param) {
                    if ($param->type) {
                        $param = $param->setType(self::expandUnion(
                            $codebase,
                            $param->type,
                            $self_class,
                            $static_class_type,
                            $parent_class,
                            $evaluate_class_constants,
                            $evaluate_conditional_types,
                            $final,
                            $expand_generic,
                            $expand_templates,
                            $throw_on_unresolvable_constant,
                        ));
                    }
                }
                unset($param);
            }
            $sub_return_type = $return_type->return_type;
            if ($sub_return_type) {
                $sub_return_type = self::expandUnion(
                    $codebase,
                    $sub_return_type,
                    $self_class,
                    $static_class_type,
                    $parent_class,
                    $evaluate_class_constants,
                    $evaluate_conditional_types,
                    $final,
                    $expand_generic,
                    $expand_templates,
                    $throw_on_unresolvable_constant,
                );
            }

            $return_type = $return_type->replace(
                $params,
                $sub_return_type,
            );

            // the purity may name a type alias (`Closure[Storage]`)
            if (!Capabilities::isPurityType($return_type->purity) || $return_type->purity->hasTypeAlias()) {
                $return_type = $return_type->setPurity(self::expandUnion(
                    $codebase,
                    $return_type->purity,
                    $self_class,
                    $static_class_type,
                    $parent_class,
                    $evaluate_class_constants,
                    $evaluate_conditional_types,
                    $final,
                    $expand_generic,
                    $expand_templates,
                    $throw_on_unresolvable_constant,
                ));
            }
        }

        return [$return_type];
    }

    /**
     * @param-out TNamedObject|TTemplateParam $return_type
     */
    private static function expandNamedObject(
        Codebase $codebase,
        TNamedObject &$return_type,
        ?string $self_class,
        string|TNamedObject|TTemplateParam|null $static_class_type,
        ?string $parent_class,
        bool $final = false,
        bool &$expand_generic = false,
    ): TNamedObject|TTemplateParam {
        if ($expand_generic
            && $return_type::class === TNamedObject::class
            && !$return_type->extra_types
            && $codebase->classOrInterfaceExists($return_type->value)
            // a class can exist without having been scanned, and then there is nothing to expand
            && $codebase->classlike_storage_provider->has(
                $codebase->classlikes->getUnAliasedName($return_type->value),
            )
        ) {
            $value = $codebase->classlikes->getUnAliasedName($return_type->value);
            $container_class_storage = ($codebase->classlike_storage_provider->getOrNull(
                $value,
            ) ?? throw \Psalm\Internal\Provider\ClassLikeStorageProvider::missing($value));

            if ($container_class_storage->template_types
                && array_any(
                    $container_class_storage->template_types,
                    static fn($type_map): bool => !reset($type_map)->hasMixed(),
                )
            ) {
                $return_type = new TGenericObject(
                    $return_type->value,
                    array_values(
                        array_map(
                            static fn($type_map) => PurityArguments::getOmittedArgument(reset($type_map)),
                            $container_class_storage->template_types,
                        ),
                    ),
                );

                // we don't want to expand generic types recursively
                $expand_generic = false;
            }
        }

        $return_type_lc = strtolower($return_type->value);

        if ($static_class_type && ($return_type_lc === 'static' || $return_type_lc === '$this')) {
            $is_static = $return_type->is_static;
            $is_static_resolved = null;
            if (!$final) {
                $is_static = true;
                $is_static_resolved = true;
            }
            if (is_string($static_class_type)) {
                $return_type = $return_type->setValueIsStatic(
                    $static_class_type,
                    $is_static,
                    $is_static_resolved,
                );
            } else {
                if ($return_type instanceof TGenericObject
                    && $static_class_type instanceof TGenericObject
                ) {
                    $return_type = $return_type->setValueIsStatic(
                        $static_class_type->value,
                        $is_static,
                        $is_static_resolved,
                    );

                    if ($codebase->classlike_storage_provider->has($static_class_type->value)) {
                        $type_params = PurityArguments::trim(
                            $return_type->type_params,
                            ($codebase->classlike_storage_provider->getOrNull($static_class_type->value) ?? throw \Psalm\Internal\Provider\ClassLikeStorageProvider::missing($static_class_type->value)),
                        );

                        if ($type_params !== [] && count($type_params) !== count($return_type->type_params)) {
                            $return_type = $return_type->setTypeParams($type_params);
                        }
                    }
                } elseif ($static_class_type instanceof TNamedObject) {
                    $return_type = $static_class_type->setIsStatic(
                        $is_static,
                        $is_static_resolved,
                    );
                } else {
                    $return_type = $static_class_type;
                }
            }
        } elseif ($return_type->is_static
            && !$return_type->is_static_resolved
            && $return_type::class === TNamedObject::class
            && $static_class_type instanceof TNamedObject
            && $codebase->classExtends($static_class_type->value, $return_type->value)
        ) {
            // The called class already includes the declaring class's constraints.
            $return_type = $static_class_type->setIntersectionTypes(
                array_merge($return_type->extra_types, $static_class_type->extra_types),
            )->setIsStatic(!$final, true);
        } elseif ($return_type->is_static && !$return_type->is_static_resolved
            && ($static_class_type instanceof TNamedObject
                || $static_class_type instanceof TTemplateParam)
        ) {
            $return_type_types = $return_type->getIntersectionTypes();
            $cloned_static = $static_class_type->setIntersectionTypes([]);
            $extra_static = $static_class_type->extra_types;

            if ($cloned_static->getKey(false) !== $return_type->getKey(false)) {
                $return_type_types[$cloned_static->getKey()] = $cloned_static;
            }

            foreach ($extra_static as $extra_static_type) {
                if ($extra_static_type->getKey(false) !== $return_type->getKey(false)) {
                    $return_type_types[$extra_static_type->getKey()] = $extra_static_type;
                }
            }
            $return_type = $return_type->setIntersectionTypes($return_type_types)
                ->setIsStatic(true, true);
        } elseif ($return_type->is_static
            && is_string($static_class_type)
            && $final
            && (
                $return_type->value === $self_class
                || ($self_class !== null &&
                    ($codebase->classExtends($return_type->value, $self_class)
                        || $codebase->classExtends($self_class, $return_type->value)
                    )
                )
            )
        ) {
            $return_type = $return_type->setValueIsStatic(
                $static_class_type,
                false,
            );
        } elseif ($self_class && $return_type_lc === 'self') {
            $return_type = $return_type->setValue($self_class);
        } elseif ($parent_class && $return_type_lc === 'parent') {
            $return_type = $return_type->setValue($parent_class);
        } else {
            $new_value = $codebase->classlikes->getUnAliasedName($return_type->value);
            $return_type = $return_type->setValue($new_value);
        }

        return $return_type;
    }

    /**
     * @return non-empty-list<Atomic>
     */
    private static function expandConditional(
        Codebase $codebase,
        TConditional &$return_type,
        ?string $self_class,
        string|TNamedObject|TTemplateParam|null $static_class_type,
        ?string $parent_class,
        bool $evaluate_class_constants = true,
        bool $evaluate_conditional_types = false,
        bool $final = false,
        bool $expand_generic = false,
        bool $expand_templates = false,
        bool $throw_on_unresolvable_constant = false,
    ): array {
        $new_as_type = self::expandUnion(
            $codebase,
            $return_type->as_type,
            $self_class,
            $static_class_type,
            $parent_class,
            $evaluate_class_constants,
            $evaluate_conditional_types,
            $final,
            $expand_generic,
            $expand_templates,
            $throw_on_unresolvable_constant,
        );

        if ($evaluate_conditional_types) {
            $assertion = null;

            if ($return_type->conditional_type->isSingle()) {
                foreach ($return_type->conditional_type->getAtomicTypes() as $condition_atomic_type) {
                    $candidate = self::expandAtomic(
                        $codebase,
                        $condition_atomic_type,
                        $self_class,
                        $static_class_type,
                        $parent_class,
                        $evaluate_class_constants,
                        $evaluate_conditional_types,
                        $final,
                        $expand_generic,
                        $expand_templates,
                        $throw_on_unresolvable_constant,
                    );

                    if (count($candidate) === 1) {
                        $assertion = new IsType($candidate[0]);
                    }
                }
            }

            $if_conditional_return_types = [];

            foreach ($return_type->if_type->getAtomicTypes() as $if_atomic_type) {
                $candidate_types = self::expandAtomic(
                    $codebase,
                    $if_atomic_type,
                    $self_class,
                    $static_class_type,
                    $parent_class,
                    $evaluate_class_constants,
                    $evaluate_conditional_types,
                    $final,
                    $expand_generic,
                    $expand_templates,
                    $throw_on_unresolvable_constant,
                );

                $if_conditional_return_types = [...$if_conditional_return_types, ...$candidate_types];
            }

            $else_conditional_return_types = [];

            foreach ($return_type->else_type->getAtomicTypes() as $else_atomic_type) {
                $candidate_types = self::expandAtomic(
                    $codebase,
                    $else_atomic_type,
                    $self_class,
                    $static_class_type,
                    $parent_class,
                    $evaluate_class_constants,
                    $evaluate_conditional_types,
                    $final,
                    $expand_generic,
                    $expand_templates,
                    $throw_on_unresolvable_constant,
                );

                $else_conditional_return_types = [...$else_conditional_return_types, ...$candidate_types];
            }

            if ($assertion && $return_type->param_name === (string) $return_type->if_type) {
                $if_conditional_return_type = TypeCombiner::combine(
                    $if_conditional_return_types,
                    $codebase,
                );

                $if_conditional_return_type = SimpleAssertionReconciler::reconcile(
                    $assertion,
                    $codebase,
                    $if_conditional_return_type,
                );


                if ($if_conditional_return_type) {
                    $if_conditional_return_types = $if_conditional_return_type->getAtomicTypes();
                }
            }

            if ($assertion && $return_type->param_name === (string) $return_type->else_type) {
                $else_conditional_return_type = TypeCombiner::combine(
                    $else_conditional_return_types,
                    $codebase,
                );

                $else_conditional_return_type = SimpleNegatedAssertionReconciler::reconcile(
                    $codebase,
                    $assertion,
                    $else_conditional_return_type,
                );

                if ($else_conditional_return_type) {
                    $else_conditional_return_types = $else_conditional_return_type->getAtomicTypes();
                }
            }

            $all_conditional_return_types = [...$if_conditional_return_types, ...$else_conditional_return_types];

            $number_of_types = count($all_conditional_return_types);
            // we filter TNever that have no bearing on the return type
            if ($number_of_types > 1) {
                $all_conditional_return_types = array_filter(
                    $all_conditional_return_types,
                    static fn(Atomic $atomic_type): bool => !$atomic_type instanceof TNever,
                );
            }

            // if we still have more than one type, we remove TVoid and replace it by TNull
            $number_of_types = count($all_conditional_return_types);
            if ($number_of_types > 1) {
                $all_conditional_return_types = array_filter(
                    $all_conditional_return_types,
                    static fn(Atomic $atomic_type): bool => !$atomic_type instanceof TVoid,
                );

                if (count($all_conditional_return_types) !== $number_of_types) {
                    $all_conditional_return_types[] = new TNull(true);
                }
            }

            if ($all_conditional_return_types) {
                $combined = TypeCombiner::combine(
                    array_values($all_conditional_return_types),
                    $codebase,
                );

                $return_type = $return_type->setTypes($new_as_type);

                return $combined->getAtomicTypes();
            }
        }

        $return_type = $return_type->setTypes(
            $new_as_type,
            self::expandUnion(
                $codebase,
                $return_type->conditional_type,
                $self_class,
                $static_class_type,
                $parent_class,
                $evaluate_class_constants,
                $evaluate_conditional_types,
                $final,
                $expand_generic,
                $expand_templates,
                $throw_on_unresolvable_constant,
            ),
            self::expandUnion(
                $codebase,
                $return_type->if_type,
                $self_class,
                $static_class_type,
                $parent_class,
                $evaluate_class_constants,
                $evaluate_conditional_types,
                $final,
                $expand_generic,
                $expand_templates,
                $throw_on_unresolvable_constant,
            ),
            self::expandUnion(
                $codebase,
                $return_type->else_type,
                $self_class,
                $static_class_type,
                $parent_class,
                $evaluate_class_constants,
                $evaluate_conditional_types,
                $final,
                $expand_generic,
                $expand_templates,
                $throw_on_unresolvable_constant,
            ),
        );
        return [$return_type];
    }

    /**
     * @return non-empty-list<Atomic>
     */
    private static function expandPropertiesOf(
        Codebase $codebase,
        TPropertiesOf &$return_type,
        ?string $self_class,
        string|TNamedObject|TTemplateParam|null $static_class_type,
    ): array {
        if ($self_class) {
            $return_type = $return_type->replaceClassLike(
                'self',
                $self_class,
            );
            $return_type = $return_type->replaceClassLike(
                'static',
                is_string($static_class_type) ? $static_class_type : $self_class,
            );
        }

        $class_storage = null;
        if ($codebase->classExists($return_type->classlike_type->value)) {
            $class_storage = ($codebase->classlike_storage_provider->getOrNull($return_type->classlike_type->value) ?? throw \Psalm\Internal\Provider\ClassLikeStorageProvider::missing($return_type->classlike_type->value));
        } else {
            foreach ($return_type->classlike_type->extra_types as $type) {
                if ($type instanceof TNamedObject && $codebase->classExists($type->value)) {
                    $class_storage = ($codebase->classlike_storage_provider->getOrNull($type->value) ?? throw \Psalm\Internal\Provider\ClassLikeStorageProvider::missing($type->value));
                    break;
                }
            }
        }

        if (!$class_storage) {
            return [$return_type];
        }

        $all_sealed = true;
        $properties = [];
        foreach ([$class_storage->name, ...array_values($class_storage->parent_classes)] as $class) {
            if (!$codebase->classExists($class)) {
                continue;
            }
            $storage = ($codebase->classlike_storage_provider->getOrNull($class) ?? throw \Psalm\Internal\Provider\ClassLikeStorageProvider::missing($class));
            if (!$storage->final) {
                $all_sealed = false;
            }
            foreach ($storage->properties as $key => $property) {
                if (isset($properties[$key])) {
                    continue;
                }
                if ($return_type->visibility_filter !== null
                    && $property->visibility !== $return_type->visibility_filter
                ) {
                    continue;
                }
                if ($property->is_static || !$property->type) {
                    continue;
                }
                $type = $return_type->classlike_type instanceof TGenericObject
                    ? AtomicPropertyFetchAnalyzer::localizePropertyType(
                        $codebase,
                        $property->type,
                        $return_type->classlike_type,
                        $storage,
                        $storage,
                    )
                    : $property->type
                ;
                $properties[$key] = $type;
            }
        }

        if ($properties === []) {
            return [$return_type];
        }
        return [TKeyedArray::make(
            $properties,
            null,
            $all_sealed ? null : [Type::getString(), Type::getMixed()],
        )];
    }

    /**
     * @param TKeyOf|TValueOf $return_type
     * @return non-empty-list<Atomic>
     */
    private static function expandKeyOfValueOf(
        Codebase $codebase,
        Atomic &$return_type,
        ?string $self_class,
        string|TNamedObject|TTemplateParam|null $static_class_type,
        ?string $parent_class,
        bool $evaluate_class_constants = true,
        bool $evaluate_conditional_types = false,
        bool $final = false,
        bool $expand_generic = false,
        bool $expand_templates = false,
        bool $throw_on_unresolvable_constant = false,
    ): array {
        // Expand class constants to their atomics
        $type_atomics = [];
        foreach ($return_type->type->getAtomicTypes() as $type_param) {
            if (!$evaluate_class_constants || !$type_param instanceof TClassConstant) {
                $type_param_expanded = self::expandAtomic(
                    $codebase,
                    $type_param,
                    $self_class,
                    $static_class_type,
                    $parent_class,
                    $evaluate_class_constants,
                    $evaluate_conditional_types,
                    $final,
                    $expand_generic,
                    $expand_templates,
                    $throw_on_unresolvable_constant,
                );
                $type_atomics = [...$type_atomics, ...$type_param_expanded];
                continue;
            }

            if ($self_class) {
                $type_param = $type_param->replaceClassLike('self', $self_class);
            }

            if ($throw_on_unresolvable_constant
                && !$codebase->classOrInterfaceOrEnumExists($type_param->fq_classlike_name)
            ) {
                throw new UnresolvableConstantException($type_param->fq_classlike_name, $type_param->const_name);
            }

            try {
                $constant_type = $codebase->classlikes->getClassConstantType(
                    $type_param->fq_classlike_name,
                    $type_param->const_name,
                    ReflectionProperty::IS_PRIVATE,
                    null,
                    [],
                    false,
                    $return_type instanceof TValueOf,
                );
            } catch (CircularReferenceException) {
                return [$return_type];
            }

            if (!$constant_type
                || (
                    $return_type instanceof TKeyOf
                    && !TKeyOf::isViableTemplateType($constant_type)
                )
                || (
                    $return_type instanceof TValueOf
                    && !TValueOf::isViableTemplateType($constant_type)
                )
            ) {
                if ($throw_on_unresolvable_constant) {
                    throw new UnresolvableConstantException($type_param->fq_classlike_name, $type_param->const_name);
                } else {
                    return [$return_type];
                }
            }

            $type_atomics = array_merge(
                $type_atomics,
                $constant_type->getAtomicTypes(),
            );
        }
        if ($type_atomics === []) {
            return [$return_type];
        }

        if ($return_type instanceof TKeyOf) {
            $new_return_types = TKeyOf::getArrayKeyType(new Union($type_atomics));
        } else {
            $new_return_types = TValueOf::getValueType(new Union($type_atomics), $codebase);
        }

        if ($new_return_types === null) {
            return [$return_type];
        }

        return $new_return_types->getAtomicTypes();
    }
}
