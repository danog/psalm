<?php

declare(strict_types=1);

namespace Psalm\Internal\Provider\ReturnTypeProvider;

use Override;
use Psalm\CodeLocation;
use Psalm\Context;
use Psalm\Internal\Analyzer\ClassAnalyzer;
use Psalm\Internal\Analyzer\SourceAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\Fetch\AtomicPropertyFetchAnalyzer;
use Psalm\Internal\Analyzer\StatementsAnalyzer;
use Psalm\Internal\Interner;
use Psalm\Internal\Sym;
use Psalm\Plugin\EventHandler\Event\FunctionReturnTypeProviderEvent;
use Psalm\Plugin\EventHandler\FunctionReturnTypeProviderInterface;
use Psalm\Type;
use Psalm\Type\Atomic;
use Psalm\Type\Atomic\TArray;
use Psalm\Type\Atomic\TGenericObject;
use Psalm\Type\Atomic\TKeyedArray;
use Psalm\Type\Atomic\TNamedObject;
use Psalm\Type\Atomic\TObjectWithProperties;
use Psalm\Type\Union;
use UnitEnum;
use stdClass;

use function reset;
use function strtolower;

/**
 * @internal
 */
final class GetObjectVarsReturnTypeProvider implements FunctionReturnTypeProviderInterface
{
    /**
     * @psalm-pure
     * @return list<lowercase-string>
     */
    #[Override]
    public static function getFunctionIds(): array
    {
        return ['get_object_vars'];
    }

    private static ?TArray $fallback = null;

    /**
     * @return TArray|TKeyedArray
     */
    public static function getGetObjectVarsReturnType(
        Union $first_arg_type,
        SourceAnalyzer $statements_source,
        Context $context,
        CodeLocation $location,
    ): Atomic {
        self::$fallback ??= new TArray([Type::getString(), Type::getMixed()]);

        if ($first_arg_type->isSingle()) {
            $atomics = $first_arg_type->getAtomicTypes();
            $object_type = reset($atomics);

            if ($object_type instanceof Atomic\TEnumCase) {
                $properties = ['name' => new Union([Type::getAtomicStringFromLiteral($object_type->case_name)])];
                $codebase = $statements_source->getCodebase();
                $enum_classlike_storage = $codebase->classlike_storage_provider->get($object_type->name);
                if ($enum_classlike_storage->enum_type === null) {
                    return TKeyedArray::make($properties);
                }
                $enum_case_storage = $enum_classlike_storage->enum_cases[Interner::intern($object_type->case_name)];
                $case_value = $enum_case_storage->getValue($statements_source->getCodebase()->classlikes);

                if ($case_value !== null) {
                    $properties['value'] = new Union([$case_value]);
                }

                return TKeyedArray::make($properties);
            }

            if ($object_type instanceof TObjectWithProperties) {
                if ([] === $object_type->properties) {
                    return self::$fallback;
                }
                return TKeyedArray::make($object_type->properties);
            }

            if ($object_type instanceof TNamedObject) {
                if (strtolower(Interner::lookup($object_type->name)) === strtolower(stdClass::class)) {
                    return self::$fallback;
                }
                $codebase = $statements_source->getCodebase();
                $class_storage = $codebase->classlikes->getStorageFor(Interner::lookup($object_type->name));

                if (null === $class_storage) {
                    return self::$fallback;
                }

                if ([] === $class_storage->appearing_property_ids) {
                    if ($class_storage->final) {
                        return Type::getEmptyArrayAtomic();
                    }

                    return self::$fallback;
                }

                $properties = [];
                foreach ($class_storage->appearing_property_ids as $name_id => $property_id) {
                    $name = Interner::lookup($name_id);
                    if (ClassAnalyzer::checkPropertyVisibility(
                        $property_id,
                        $context,
                        $statements_source,
                        $location,
                        $statements_source->getSuppressedIssues(),
                        false,
                    ) === true) {
                        $property_type = $codebase->properties->getPropertyType(
                            $property_id,
                            false,
                            $statements_source,
                            $context,
                        );
                        if (!$property_type) {
                            continue;
                        }

                        $property_type = $object_type instanceof TGenericObject
                            ? AtomicPropertyFetchAnalyzer::localizePropertyType(
                                $codebase,
                                $property_type,
                                $object_type,
                                $class_storage,
                                $class_storage,
                            )
                            : $property_type
                        ;
                        $properties[$name] = $property_type;
                    }
                }

                if ([] === $properties) {
                    if ($class_storage->final) {
                        return Type::getEmptyArrayAtomic();
                    }

                    return self::$fallback;
                }

                return TKeyedArray::make(
                    $properties,
                    null,
                    $class_storage->final
                        || $class_storage->name === UnitEnum::class
                        || $codebase->interfaceExtends($class_storage->id, Sym::UNIT_ENUM)
                            ? null
                            : [Type::getString(), Type::getMixed()],
                );
            }
        }
        return self::$fallback;
    }

    #[Override]
    public static function getFunctionReturnType(FunctionReturnTypeProviderEvent $event): ?Union
    {
        $statements_source = $event->getStatementsSource();
        $call_args = $event->getCallArgs();
        if (!$statements_source instanceof StatementsAnalyzer
            || !$call_args
        ) {
            return Type::getMixed();
        }

        if (($first_arg_type = $statements_source->node_data->getType($call_args[0]->value))
             && $first_arg_type->isObjectType()
        ) {
            return new Union([self::getGetObjectVarsReturnType(
                $first_arg_type,
                $statements_source,
                $event->getContext(),
                $event->getCodeLocation(),
            )]);
        }

        return null;
    }
}
