<?php

declare(strict_types=1);

namespace Psalm\Type;

use InvalidArgumentException;
use Override;
use Psalm\CodeLocation;
use Psalm\Codebase;
use Psalm\Context;
use Psalm\Internal\DataFlow\DataFlowNode;
use Psalm\Internal\TypeVisitor\CanContainObjectTypeVisitor;
use Psalm\Internal\TypeVisitor\ClasslikeReplacer;
use Psalm\Internal\TypeVisitor\ContainsClassLikeVisitor;
use Psalm\Internal\TypeVisitor\ContainsLiteralVisitor;
use Psalm\Internal\TypeVisitor\TemplateTypeCollector;
use Psalm\Internal\TypeVisitor\TypeChecker;
use Psalm\Internal\TypeVisitor\TypeScanner;
use Psalm\StatementsSource;
use Psalm\Storage\FileStorage;
use Psalm\Type\Atomic\IdMemo;
use Psalm\Type\Atomic\TArray;
use Psalm\Type\Atomic\TArrayKey;
use Psalm\Type\Atomic\TBool;
use Psalm\Type\Atomic\TCallable;
use Psalm\Type\Atomic\TClassString;
use Psalm\Type\Atomic\TClassStringMap;
use Psalm\Type\Atomic\TClosure;
use Psalm\Type\Atomic\TConditional;
use Psalm\Type\Atomic\TEmptyMixed;
use Psalm\Type\Atomic\TFalse;
use Psalm\Type\Atomic\TFloat;
use Psalm\Type\Atomic\TInt;
use Psalm\Type\Atomic\TIntRange;
use Psalm\Type\Atomic\TKeyedArray;
use Psalm\Type\Atomic\TLiteralFloat;
use Psalm\Type\Atomic\TLiteralInt;
use Psalm\Type\Atomic\TLiteralString;
use Psalm\Type\Atomic\TLowercaseString;
use Psalm\Type\Atomic\TMixed;
use Psalm\Type\Atomic\TNamedObject;
use Psalm\Type\Atomic\TNever;
use Psalm\Type\Atomic\TNonEmptyLowercaseString;
use Psalm\Type\Atomic\TNonEmptyNonspecificLiteralString;
use Psalm\Type\Atomic\TNonEmptyString;
use Psalm\Type\Atomic\TNonspecificLiteralInt;
use Psalm\Type\Atomic\TNonspecificLiteralString;
use Psalm\Type\Atomic\TNull;
use Psalm\Type\Atomic\TString;
use Psalm\Type\Atomic\TTemplateParam;
use Psalm\Type\Atomic\TTemplateParamClass;
use Psalm\Type\Atomic\TTrue;
use Psalm\Type\Atomic\TTypeAlias;
use UnexpectedValueException;

use function array_filter;
use function array_key_exists;
use function array_keys;
use function array_unique;
use function array_values;
use function count;
use function implode;
use function ksort;
use function reset;
use function sort;
use function str_contains;
use function strpos;

/**
 * @psalm-immutable
 * @psalm-import-type TProperties from Union
 * @api
 */
trait UnionTrait
{
    /**
     * The construction properties of this union (see TProperties).
     *
     * @return TProperties
     * @psalm-mutation-free
     */
    public function getConstructionProperties(): array
    {
        return [
            'from_docblock' => $this->from_docblock,
            'from_calculation' => $this->from_calculation,
            'from_property' => $this->from_property,
            'from_static_property' => $this->from_static_property,
            'from_global_state' => $this->from_global_state,
            'initialized' => $this->initialized,
            'initialized_class' => $this->initialized_class,
            'checked' => $this->checked,
            'failed_reconciliation' => $this->failed_reconciliation,
            'ignore_nullable_issues' => $this->ignore_nullable_issues,
            'ignore_falsable_issues' => $this->ignore_falsable_issues,
            'ignore_isset' => $this->ignore_isset,
            'possibly_undefined' => $this->possibly_undefined,
            'possibly_undefined_from_try' => $this->possibly_undefined_from_try,
            'explicit_never' => $this->explicit_never,
            'had_template' => $this->had_template,
            'from_template_default' => $this->from_template_default,
            'by_ref' => $this->by_ref,
            'reference_free' => $this->reference_free,
            'allow_mutations' => $this->allow_mutations,
            'has_mutations' => $this->has_mutations,
            'different' => $this->different,
            'parent_nodes' => $this->parent_nodes,
        ];
    }

    /**
     * Constructs a Union instance
     *
     * @param non-empty-list<Atomic> $types
     * @param TProperties $properties
     * @psalm-mutation-free
     */
    public function __construct(array $types, array $properties = [])
    {
        if (array_key_exists('from_docblock', $properties)) {
            $this->from_docblock = $properties['from_docblock'];
        }
        if (array_key_exists('from_calculation', $properties)) {
            $this->from_calculation = $properties['from_calculation'];
        }
        if (array_key_exists('from_property', $properties)) {
            $this->from_property = $properties['from_property'];
        }
        if (array_key_exists('from_static_property', $properties)) {
            $this->from_static_property = $properties['from_static_property'];
        }
        if (array_key_exists('from_global_state', $properties)) {
            $this->from_global_state = $properties['from_global_state'];
        }
        if (array_key_exists('initialized', $properties)) {
            $this->initialized = $properties['initialized'];
        }
        if (array_key_exists('initialized_class', $properties)) {
            $this->initialized_class = $properties['initialized_class'];
        }
        if (array_key_exists('checked', $properties)) {
            $this->checked = $properties['checked'];
        }
        if (array_key_exists('failed_reconciliation', $properties)) {
            $this->failed_reconciliation = $properties['failed_reconciliation'];
        }
        if (array_key_exists('ignore_nullable_issues', $properties)) {
            $this->ignore_nullable_issues = $properties['ignore_nullable_issues'];
        }
        if (array_key_exists('ignore_falsable_issues', $properties)) {
            $this->ignore_falsable_issues = $properties['ignore_falsable_issues'];
        }
        if (array_key_exists('ignore_isset', $properties)) {
            $this->ignore_isset = $properties['ignore_isset'];
        }
        if (array_key_exists('possibly_undefined', $properties)) {
            $this->possibly_undefined = $properties['possibly_undefined'];
        }
        if (array_key_exists('possibly_undefined_from_try', $properties)) {
            $this->possibly_undefined_from_try = $properties['possibly_undefined_from_try'];
        }
        if (array_key_exists('explicit_never', $properties)) {
            $this->explicit_never = $properties['explicit_never'];
        }
        if (array_key_exists('had_template', $properties)) {
            $this->had_template = $properties['had_template'];
        }
        if (array_key_exists('from_template_default', $properties)) {
            $this->from_template_default = $properties['from_template_default'];
        }
        if (array_key_exists('by_ref', $properties)) {
            $this->by_ref = $properties['by_ref'];
        }
        if (array_key_exists('reference_free', $properties)) {
            $this->reference_free = $properties['reference_free'];
        }
        if (array_key_exists('allow_mutations', $properties)) {
            $this->allow_mutations = $properties['allow_mutations'];
        }
        if (array_key_exists('has_mutations', $properties)) {
            $this->has_mutations = $properties['has_mutations'];
        }
        if (array_key_exists('different', $properties)) {
            $this->different = $properties['different'];
        }
        if (array_key_exists('parent_nodes', $properties)) {
            $this->parent_nodes = $properties['parent_nodes'];
        }
        $this->checked = false;
        $this->memo = null;

        $this->types = self::listOfTypes($types);

        // an explicit from_docblock is exact; otherwise a union is from a docblock when any of its atomics is
        $exact_from_docblock = array_key_exists('from_docblock', $properties);
        $from_docblock = $this->from_docblock;
        foreach ($this->types as $type) {
            if ($type instanceof TNever) {
                $this->explicit_never = true;
            }
            $from_docblock = $from_docblock || $type->from_docblock;
        }
        if (!$exact_from_docblock) {
            $this->from_docblock = $from_docblock;
        }
    }

    /**
     * The atomics as a list holding one type per key (pzoom's `Vec<TAtomic>`; Psalm's map kept one atomic
     * per getKey(), the later one winning, and that stays true). A single atomic needs no keys at all.
     *
     * @param non-empty-list<Atomic> $types
     * @return non-empty-list<Atomic>
     * @psalm-pure
     */
    private static function listOfTypes(array $types): array
    {
        if (count($types) === 1) {
            return $types; // kept as it is: no copy
        }
        $by_key = [];
        foreach ($types as $type) {
            $by_key[$type->getKey()] = $type;
        }
        /** @var non-empty-list<Atomic> */
        return array_values($by_key);
    }

    /**
     * The literal-typed atomics (pzoom scans its Vec; Psalm kept side maps of them).
     *
     * @psalm-mutation-free
     */
    private function countLiteralInts(): int
    {
        $n = 0;
        foreach ($this->types as $type) {
            if ($type instanceof TLiteralInt) {
                $n++;
            }
        }
        return $n;
    }

    /** @psalm-mutation-free */
    private function countLiteralStrings(): int
    {
        $n = 0;
        foreach ($this->types as $type) {
            if ($type instanceof TLiteralString) {
                $n++;
            }
        }
        return $n;
    }

    /** @psalm-mutation-free */
    private function countLiteralFloats(): int
    {
        $n = 0;
        foreach ($this->types as $type) {
            if ($type instanceof TLiteralFloat) {
                $n++;
            }
        }
        return $n;
    }

    /** @psalm-mutation-free */
    private function firstLiteralInt(): ?TLiteralInt
    {
        foreach ($this->types as $type) {
            if ($type instanceof TLiteralInt) {
                return $type;
            }
        }
        return null;
    }

    /** @psalm-mutation-free */
    private function firstLiteralString(): ?TLiteralString
    {
        foreach ($this->types as $type) {
            if ($type instanceof TLiteralString) {
                return $type;
            }
        }
        return null;
    }

    /** @psalm-mutation-free */
    private function firstLiteralFloat(): ?TLiteralFloat
    {
        foreach ($this->types as $type) {
            if ($type instanceof TLiteralFloat) {
                return $type;
            }
        }
        return null;
    }

    /**
     * @return array<string, TLiteralInt>
     * @psalm-mutation-free
     */
    private function collectLiteralInts(): array
    {
        $literals = [];
        foreach ($this->types as $type) {
            if ($type instanceof TLiteralInt) {
                $literals[$type->getKey()] = $type;
            }
        }
        return $literals;
    }

    /**
     * @return array<string, TLiteralString>
     * @psalm-mutation-free
     */
    private function collectLiteralStrings(): array
    {
        $literals = [];
        foreach ($this->types as $type) {
            if ($type instanceof TLiteralString) {
                $literals[$type->getKey()] = $type;
            }
        }
        return $literals;
    }

    /**
     * @return array<string, TLiteralFloat>
     * @psalm-mutation-free
     */
    private function collectLiteralFloats(): array
    {
        $literals = [];
        foreach ($this->types as $type) {
            if ($type instanceof TLiteralFloat) {
                $literals[$type->getKey()] = $type;
            }
        }
        return $literals;
    }

    /**
     * A class-string with a bound (`class-string<Foo>`) or a template class-string.
     *
     * @psalm-mutation-free
     */
    private function hasTypedClassString(): bool
    {
        foreach ($this->types as $type) {
            if ($type instanceof TClassString && ($type->as_type || $type instanceof TTemplateParamClass)) {
                return true;
            }
        }
        return false;
    }

    /**
     * The atomics keyed by Atomic::getKey(), for code that edits a keyed copy and builds a union from it.
     * Built on demand: the union itself holds a list.
     *
     * @psalm-mutation-free
     * @return non-empty-array<string, Atomic>
     */
    public function getAtomicTypesByKey(): array
    {
        $by_key = [];
        foreach ($this->types as $type) {
            $by_key[$type->getKey()] = $type;
        }
        return $by_key;
    }

    /**
     * The atomic with this key (Atomic::getKey()), if any.
     *
     * @psalm-mutation-free
     */
    public function find(string $key): ?Atomic
    {
        foreach ($this->types as $type) {
            if ($type->getKey() === $key) {
                return $type;
            }
        }
        return null;
    }

    /**
     * @psalm-mutation-free
     */
    public function has(string $key): bool
    {
        foreach ($this->types as $type) {
            if ($type->getKey() === $key) {
                return true;
            }
        }
        return false;
    }

    /**
     * @psalm-mutation-free
     * @return non-empty-list<Atomic>
     */
    public function getAtomicTypes(): array
    {
        return $this->types;
    }

    /**
     * @psalm-mutation-free
     */
    public function __toString(): string
    {
        $types = [];

        $printed_int = false;
        $printed_float = false;
        $printed_string = false;

        foreach ($this->types as $type) {
            if ($type instanceof TLiteralFloat) {
                if ($printed_float) {
                    continue;
                }

                $printed_float = true;
            } elseif ($type instanceof TLiteralString) {
                if ($printed_string) {
                    continue;
                }

                $printed_string = true;
            } elseif ($type instanceof TLiteralInt) {
                if ($printed_int) {
                    continue;
                }

                $printed_int = true;
            }

            $types[] = $type->getId(false);
        }

        sort($types);
        return implode('|', $types);
    }

    /**
     * @psalm-mutation-free
     */
    public function getKey(): string
    {
        $types = [];

        $printed_int = false;
        $printed_float = false;
        $printed_string = false;

        foreach ($this->types as $type) {
            if ($type instanceof TLiteralFloat) {
                if ($printed_float) {
                    continue;
                }

                $types[] = 'float';
                $printed_float = true;
            } elseif ($type instanceof TLiteralString) {
                if ($printed_string) {
                    continue;
                }

                $types[] = 'string';
                $printed_string = true;
            } elseif ($type instanceof TLiteralInt) {
                if ($printed_int) {
                    continue;
                }

                $types[] = 'int';
                $printed_int = true;
            } else {
                $types[] = $type->getKey();
            }
        }

        sort($types);
        return implode('|', $types);
    }

    /**
     * @psalm-mutation-free
     */
    public function getId(bool $exact = true): string
    {
        $memo = $this->memo;
        if ($memo !== null) {
            if ($exact && $memo->id !== null) {
                return $memo->id;
            } elseif (!$exact && $memo->inexact_id !== null) {
                return $memo->inexact_id;
            }
        }

        $types = [];
        foreach ($this->types as $type) {
            $types[] = $type->getId($exact);
        }
        $types = array_unique($types);
        sort($types);

        if (count($types) > 1) {
            foreach ($types as $i => $type) {
                if (strpos($type, ' as ') && !str_contains($type, '(')) {
                    $types[$i] = '(' . $type . ')';
                }
            }
        }

        $id = implode('|', $types);

        /** @psalm-suppress ImpurePropertyAssignment, InaccessibleProperty Cache */
        $memo = $this->memo ??= new IdMemo();
        if ($exact) {
            /** @psalm-suppress ImpurePropertyAssignment Cache */
            $memo->id = $id;
        } else {
            /** @psalm-suppress ImpurePropertyAssignment Cache */
            $memo->inexact_id = $id;
        }

        return $id;
    }

    /**
     * @param  array<lowercase-string, string> $aliased_classes
     * @psalm-mutation-free
     */
    public function toNamespacedString(
        ?string $namespace,
        array $aliased_classes,
        ?string $this_class,
        bool $use_phpdoc_format,
    ): string {
        $other_types = [];

        $literal_ints = [];
        $literal_strings = [];

        $has_non_literal_int = false;
        $has_non_literal_string = false;

        foreach ($this->types as $type) {
            $type_string = $type->toNamespacedString($namespace, $aliased_classes, $this_class, $use_phpdoc_format);
            if ($type instanceof TLiteralInt) {
                $literal_ints[] = $type_string;
            } elseif ($type instanceof TLiteralString) {
                $literal_strings[] = $type_string;
            } else {
                if ($type::class === TString::class) {
                    $has_non_literal_string = true;
                } elseif ($type::class === TInt::class) {
                    $has_non_literal_int = true;
                }
                $other_types[] = $type_string;
            }
        }

        if (count($literal_ints) <= 3 && !$has_non_literal_int) {
            $other_types = [...$other_types, ...$literal_ints];
        } else {
            $other_types[] = 'int';
        }

        if (count($literal_strings) <= 3 && !$has_non_literal_string) {
            $other_types = [...$other_types, ...$literal_strings];
        } else {
            $other_types[] = 'string';
        }

        sort($other_types);
        return implode('|', array_unique($other_types));
    }

    /**
     * @psalm-mutation-free
     * @param  array<lowercase-string, string> $aliased_classes
     */
    public function toPhpString(
        ?string $namespace,
        array   $aliased_classes,
        ?string $this_class,
        int     $analysis_php_version_id,
    ): ?string {
        if (!$this->isSingleAndMaybeNullable()) {
            if ($analysis_php_version_id < 8_00_00) {
                return null;
            }
        } elseif ($analysis_php_version_id < 7_00_00
            || ($this->has('null') && $analysis_php_version_id < 7_01_00)
        ) {
            return null;
        }

        $types = $this->getAtomicTypesByKey();

        $nullable = false;

        if (isset($types['null']) && count($types) > 1) {
            unset($types['null']);

            $nullable = true;
        }

        $falsable = false;

        if (isset($types['false']) && count($types) > 1) {
            unset($types['false']);

            $falsable = true;
        }

        $php_types = [];

        foreach ($types as $atomic_type) {
            $php_type = $atomic_type->toPhpString(
                $namespace,
                $aliased_classes,
                $this_class,
                $analysis_php_version_id,
            );

            if (!$php_type) {
                return null;
            }

            $php_types[] = $php_type;
        }

        if ($falsable) {
            if ($nullable) {
                $php_types['null'] = 'null';
            }
            $php_types['false'] = 'false';
            ksort($php_types);
            return implode('|', array_unique($php_types));
        }

        if ($analysis_php_version_id < 8_00_00) {
            return ($nullable ? '?' : '') . implode('|', array_unique($php_types));
        }
        if ($nullable) {
            $php_types['null'] = 'null';
        }
        return implode('|', array_unique($php_types));
    }

    /**
     * @psalm-mutation-free
     */
    public function canBeFullyExpressedInPhp(int $analysis_php_version_id): bool
    {
        if (!$this->isSingleAndMaybeNullable() && $analysis_php_version_id < 8_00_00) {
            return false;
        }

        $types = $this->getAtomicTypesByKey();

        if (isset($types['null'])) {
            if (count($types) > 1) {
                unset($types['null']);
            } else {
                return false;
            }
        }

        foreach ($types as $t) {
            if (!$t->canBeFullyExpressedInPhp($analysis_php_version_id)) {
                return false;
            }
        }

        return true;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasType(string $type_string): bool
    {
        return $this->has($type_string);
    }

    /**
     * @psalm-mutation-free
     */
    public function hasArray(): bool
    {
        return $this->has('array');
    }

    /**
     * @return TArray|TKeyedArray|TClassStringMap
     */
    public function getArray(): Atomic
    {
        $array = $this->find('array');
        if ($array instanceof TArray || $array instanceof TKeyedArray || $array instanceof TClassStringMap) {
            return $array;
        }
        throw new UnexpectedValueException('No array type');
    }

    /**
     * @psalm-mutation-free
     */
    public function hasIterable(): bool
    {
        return $this->has('iterable');
    }

    /**
     * @psalm-mutation-free
     * @psalm-api
     */
    public function hasIterableType(Codebase $codebase): bool
    {
        if ($this->has('iterable')) {
            return true;
        }
        foreach ($this->types as $t) {
            if ($t->isIterable($codebase)) {
                return true;
            }
        }
        return false;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasList(): bool
    {
        $array = $this->find('array');
        return $array instanceof TKeyedArray && $array->is_list;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasClassStringMap(): bool
    {
        return $this->find('array') instanceof TClassStringMap;
    }

    /**
     * @psalm-mutation-free
     */
    public function isTemplatedClassString(): bool
    {
        if (!$this->isSingle()) {
            return false;
        }
        $has = false;
        foreach ($this->types as $t) {
            if ($t instanceof TTemplateParamClass) {
                if ($has) {
                    return false;
                }
                $has = true;
            }
        }
        return $has;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasArrayAccessInterface(Codebase $codebase): bool
    {
        foreach ($this->types as $t) {
            if ($t->hasArrayAccessInterface($codebase)) {
                return true;
            }
        }
        return false;
    }

    /**
     * Whether the union names a type alias that has not been expanded yet.
     *
     * @psalm-mutation-free
     */
    public function hasTypeAlias(): bool
    {
        foreach ($this->types as $t) {
            if ($t instanceof TTypeAlias) {
                return true;
            }
        }
        return false;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasCallableType(): bool
    {
        foreach ($this->types as $t) {
            if ($t instanceof TCallable || $t instanceof TClosure) {
                return true;
            }
        }
        return false;
    }

    /**
     * @psalm-mutation-free
     * @return list<TCallable>
     */
    public function getCallableTypes(): array
    {
        return array_values(array_filter(
            $this->types,
            static fn($type): bool => $type instanceof TCallable,
        ));
    }

    /**
     * @psalm-mutation-free
     * @return list<TClosure>
     */
    public function getClosureTypes(): array
    {
        return array_values(array_filter(
            $this->types,
            static fn($type): bool => $type instanceof TClosure,
        ));
    }

    /**
     * @psalm-mutation-free
     */
    public function hasObject(): bool
    {
        return $this->has('object');
    }

    /**
     * @psalm-mutation-free
     */
    public function hasObjectType(): bool
    {
        foreach ($this->types as $type) {
            if ($type->isObjectType()) {
                return true;
            }
        }

        return false;
    }

    /**
     * @psalm-mutation-free
     */
    public function canContainObjectType(Codebase $codebase): bool
    {
        $object_type_visitor = new CanContainObjectTypeVisitor($codebase);

        $object_type_visitor->traverseArray($this->types);

        return $object_type_visitor->matches();
    }

    /**
     * @psalm-mutation-free
     */
    public function isObjectType(): bool
    {
        foreach ($this->types as $type) {
            if (!$type->isObjectType()) {
                return false;
            }
        }

        return true;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasNamedObjectType(): bool
    {
        foreach ($this->types as $type) {
            if ($type->isNamedObjectType()) {
                return true;
            }
        }

        return false;
    }

    /**
     * @psalm-mutation-free
     */
    public function isStaticObject(): bool
    {
        foreach ($this->types as $type) {
            if (!$type instanceof TNamedObject
                || !$type->is_static
            ) {
                return false;
            }
        }

        return true;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasStaticObject(): bool
    {
        foreach ($this->types as $type) {
            if ($type instanceof TNamedObject
                && $type->is_static
            ) {
                return true;
            }
        }

        return false;
    }

    /**
     * @psalm-mutation-free
     */
    public function isNullable(): bool
    {
        if ($this->has('null')) {
            return true;
        }

        foreach ($this->types as $type) {
            if ($type instanceof TTemplateParam && $type->as->isNullable()) {
                return true;
            }
        }

        return false;
    }

    /**
     * @psalm-mutation-free
     */
    public function isFalsable(): bool
    {
        if ($this->has('false')) {
            return true;
        }

        foreach ($this->types as $type) {
            if ($type instanceof TTemplateParam && $type->as->isFalsable()) {
                return true;
            }
        }

        return false;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasBool(): bool
    {
        return $this->has('bool') || $this->has('false') || $this->has('true');
    }

    /**
     * @psalm-mutation-free
     */
    public function hasNull(): bool
    {
        return $this->has('null');
    }

    /**
     * @psalm-mutation-free
     */
    public function hasString(): bool
    {
        return $this->has('string')
            || $this->has('class-string')
            || $this->has('trait-string')
            || $this->has('numeric-string')
            || $this->has('callable-string')
            || $this->has('array-key')
            || $this->countLiteralStrings() > 0
            || $this->hasTypedClassString();
    }

    /**
     * @psalm-mutation-free
     */
    public function hasLowercaseString(): bool
    {
        $string = $this->find('string');
        return $string instanceof TLowercaseString || $string instanceof TNonEmptyLowercaseString;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasLiteralClassString(): bool
    {
        return $this->hasTypedClassString();
    }

    /**
     * @psalm-mutation-free
     */
    public function hasInt(): bool
    {
        if ($this->has('int') || $this->has('array-key') || $this->countLiteralInts() > 0) {
            return true;
        }
        foreach ($this->types as $t) {
            if ($t instanceof TIntRange) {
                return true;
            }
        }
        return false;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasArrayKey(): bool
    {
        return $this->has('array-key');
    }

    /**
     * @psalm-mutation-free
     */
    public function hasFloat(): bool
    {
        return $this->has('float') || $this->countLiteralFloats() > 0;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasScalar(): bool
    {
        return $this->has('scalar');
    }

    /**
     * @psalm-mutation-free
     */
    public function hasNumeric(): bool
    {
        return $this->has('numeric');
    }

    /**
     * @psalm-mutation-free
     */
    public function hasScalarType(): bool
    {
        return $this->has('int')
            || $this->has('float')
            || $this->has('string')
            || $this->has('class-string')
            || $this->has('trait-string')
            || $this->has('bool')
            || $this->has('false')
            || $this->has('true')
            || $this->has('numeric')
            || $this->has('numeric-string')
            || $this->countLiteralInts() > 0
            || $this->countLiteralFloats() > 0
            || $this->countLiteralStrings() > 0
            || $this->hasTypedClassString();
    }

    /**
     * @psalm-mutation-free
     */
    public function hasTemplate(): bool
    {
        foreach ($this->types as $t) {
            if ($t instanceof TTemplateParam) {
                return true;
            }

            if ($t instanceof TNamedObject) {
                foreach ($t->extra_types as $sub) {
                    if ($sub instanceof TTemplateParam) {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasConditional(): bool
    {
        foreach ($this->types as $t) {
            if ($t instanceof TConditional) {
                return true;
            }
        }
        return false;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasTemplateOrStatic(): bool
    {
        foreach ($this->types as $t) {
            if ($t instanceof TTemplateParam) {
                return true;
            }
            if ($t instanceof TNamedObject) {
                if ($t->is_static) {
                    return true;
                }
                foreach ($t->extra_types as $sub) {
                    if ($sub instanceof TTemplateParam) {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasMixed(): bool
    {
        return $this->has('mixed');
    }

    /**
     * @psalm-mutation-free
     */
    public function isMixed(bool $check_templates = false): bool
    {
        foreach ($this->types as $t) {
            $key = $t->getKey();
            if ($key === 'mixed' || $t instanceof TMixed) {
                continue;
            }
            if ($check_templates
                && $t instanceof TTemplateParam
                && $t->as->isMixed()
            ) {
                continue;
            }
            return false;
        }
        return true;
    }

    /**
     * @psalm-mutation-free
     */
    public function isEmptyMixed(): bool
    {
        return $this->find('mixed') instanceof TEmptyMixed
            && count($this->types) === 1;
    }

    /**
     * @psalm-mutation-free
     */
    public function isVanillaMixed(): bool
    {
        $mixed = $this->find('mixed');
        return $mixed !== null
            && $mixed::class === TMixed::class
            && !$mixed->from_loop_isset
            && count($this->types) === 1;
    }

    /**
     * @psalm-mutation-free
     */
    public function isArrayKey(): bool
    {
        return $this->has('array-key') && count($this->types) === 1;
    }

    /**
     * @psalm-mutation-free
     */
    public function isNull(): bool
    {
        return count($this->types) === 1 && $this->has('null');
    }

    /**
     * @psalm-mutation-free
     */
    public function isFalse(): bool
    {
        return count($this->types) === 1 && $this->has('false');
    }

    /**
     * @psalm-mutation-free
     */
    public function isAlwaysFalsy(): bool
    {
        foreach ($this->getAtomicTypes() as $atomic_type) {
            if (!$atomic_type->isFalsy()) {
                return false;
            }
        }

        return true;
    }

    /**
     * @psalm-mutation-free
     */
    public function isTrue(): bool
    {
        return count($this->types) === 1 && $this->has('true');
    }

    /**
     * @psalm-mutation-free
     */
    public function isAlwaysTruthy(): bool
    {
        if ($this->possibly_undefined || $this->possibly_undefined_from_try) {
            return false;
        }

        foreach ($this->getAtomicTypes() as $atomic_type) {
            if (!$atomic_type->isTruthy()) {
                return false;
            }
        }

        return true;
    }

    /**
     * @psalm-mutation-free
     */
    public function isVoid(): bool
    {
        return $this->has('void') && count($this->types) === 1;
    }

    /**
     * @psalm-mutation-free
     */
    public function isNever(): bool
    {
        return $this->has('never') && count($this->types) === 1;
    }

    /**
     * @psalm-mutation-free
     */
    public function isGenerator(): bool
    {
        return count($this->types) === 1
            && (($single_type = reset($this->types)) instanceof TNamedObject)
            && ($single_type->value === 'Generator');
    }

    /**
     * @psalm-mutation-free
     */
    public function isSingle(): bool
    {
        $type_count = count($this->types);

        $int_literal_count = $this->countLiteralInts();
        $string_literal_count = $this->countLiteralStrings();
        $float_literal_count = $this->countLiteralFloats();

        if (($int_literal_count && $string_literal_count)
            || ($int_literal_count && $float_literal_count)
            || ($string_literal_count && $float_literal_count)
        ) {
            return false;
        }

        if ($int_literal_count || $string_literal_count || $float_literal_count) {
            $type_count -= $int_literal_count + $string_literal_count + $float_literal_count - 1;
        }

        return $type_count === 1;
    }

    /**
     * @psalm-mutation-free
     */
    public function isSingleAndMaybeNullable(): bool
    {
        $is_nullable = $this->has('null');

        $type_count = count($this->types);

        if ($type_count === 1 && $is_nullable) {
            return false;
        }

        $int_literal_count = $this->countLiteralInts();
        $string_literal_count = $this->countLiteralStrings();
        $float_literal_count = $this->countLiteralFloats();

        if (($int_literal_count && $string_literal_count)
            || ($int_literal_count && $float_literal_count)
            || ($string_literal_count && $float_literal_count)
        ) {
            return false;
        }

        if ($int_literal_count || $string_literal_count || $float_literal_count) {
            $type_count -= $int_literal_count + $string_literal_count + $float_literal_count - 1;
        }

        return ($type_count - (int) $is_nullable) === 1;
    }

    /**
     * @psalm-mutation-free
     * @return bool true if this is an int
     */
    public function isInt(bool $check_templates = false): bool
    {
        foreach ($this->types as $type) {
            if (!($type instanceof TInt
                || ($check_templates
                    && $type instanceof TTemplateParam
                    && $type->as->isInt()
                )
            )) {
                return false;
            }
        }
        return true;
    }

    /**
     * @psalm-mutation-free
     * @return bool true if this is a float
     */
    public function isFloat(): bool
    {
        if (!$this->isSingle()) {
            return false;
        }

        return $this->has('float') || $this->countLiteralFloats() > 0;
    }

    /**
     * @psalm-mutation-free
     * @return bool true if this is a string
     */
    public function isString(bool $check_templates = false): bool
    {
        foreach ($this->types as $type) {
            if (!($type instanceof TString
                || ($check_templates
                    && $type instanceof TTemplateParam
                    && $type->as->isString()
                ))
            ) {
                return false;
            }
        }
        return true;
    }

    /**
     * @psalm-mutation-free
     * @return bool true if this is a string
     */
    public function isNonEmptyString(bool $check_templates = false): bool
    {
        foreach ($this->types as $type) {
            if (!($type instanceof TNonEmptyString
                    || $type instanceof TNonEmptyNonspecificLiteralString
                    || ($type instanceof TLiteralString && $type->value !== '')
                    || ($check_templates
                        && $type instanceof TTemplateParam
                        && $type->as->isNonEmptyString()
                    )
                )
            ) {
                return false;
            }
        }
        return true;
    }

    /**
     * @psalm-mutation-free
     * @return bool true if this is a boolean
     */
    public function isBool(): bool
    {
        if (!$this->isSingle()) {
            return false;
        }

        return $this->has('bool');
    }

    /**
     * @psalm-mutation-free
     * @return bool true if this is an array
     */
    public function isArray(): bool
    {
        if (!$this->isSingle()) {
            return false;
        }

        return $this->has('array');
    }

    /**
     * @psalm-mutation-free
     * @return bool true if this is a string literal with only one possible value
     */
    public function isSingleStringLiteral(): bool
    {
        return count($this->types) === 1 && $this->countLiteralStrings() === 1;
    }

    /**
     * @psalm-mutation-free
     * @return bool true if this type is a safe operand for string concatenation (int|string|array-key)
     */
    public function isConcatSafe(): bool
    {
        foreach ($this->types as $type) {
            if (!($type instanceof TInt)
            && !($type instanceof TString)
            && !($type instanceof TArrayKey)) {
                return false;
            }
        }
        return true;
    }

    /**
     * @throws InvalidArgumentException if isSingleStringLiteral is false
     * @psalm-mutation-free
     * @return TLiteralString the only string literal represented by this union type
     */
    public function getSingleStringLiteral(): TLiteralString
    {
        if (count($this->types) !== 1 || $this->countLiteralStrings() !== 1) {
            throw new InvalidArgumentException('Not a string literal');
        }

        return $this->firstLiteralString() ?? throw new UnexpectedValueException('No literal string');
    }

    /**
     * @psalm-mutation-free
     */
    public function allStringLiterals(): bool
    {
        foreach ($this->types as $atomic_key_type) {
            if (!$atomic_key_type instanceof TLiteralString) {
                return false;
            }
        }

        return true;
    }

    /**
     * @psalm-mutation-free
     */
    public function allIntLiterals(): bool
    {
        foreach ($this->types as $atomic_key_type) {
            if (!$atomic_key_type instanceof TLiteralInt) {
                return false;
            }
        }

        return true;
    }

    /**
     * @psalm-mutation-free
     */
    public function allFloatLiterals(): bool
    {
        foreach ($this->types as $atomic_key_type) {
            if (!$atomic_key_type instanceof TLiteralFloat) {
                return false;
            }
        }

        return true;
    }

    /**
     * @psalm-mutation-free
     * @psalm-assert-if-true array<
     *     array-key,
     *     TLiteralString|TLiteralInt|TLiteralFloat|TFalse|TTrue
     * > $this->getAtomicTypes()
     */
    public function allSpecificLiterals(): bool
    {
        foreach ($this->types as $atomic_key_type) {
            if (!$atomic_key_type instanceof TLiteralString
                && !$atomic_key_type instanceof TLiteralInt
                && !$atomic_key_type instanceof TLiteralFloat
                && !$atomic_key_type instanceof TFalse
                && !$atomic_key_type instanceof TTrue
            ) {
                return false;
            }
        }

        return true;
    }

    /**
     * @psalm-mutation-free
     * @psalm-assert-if-true array<
     *     array-key,
     *     TLiteralString|TLiteralInt|TLiteralFloat|TNonspecificLiteralString|TNonSpecificLiteralInt|TFalse|TTrue
     * > $this->getAtomicTypes()
     */
    public function allLiterals(): bool
    {
        foreach ($this->types as $atomic_key_type) {
            if (!$atomic_key_type instanceof TLiteralString
                && !$atomic_key_type instanceof TLiteralInt
                && !$atomic_key_type instanceof TLiteralFloat
                && !$atomic_key_type instanceof TNonspecificLiteralString
                && !$atomic_key_type instanceof TNonspecificLiteralInt
                && !$atomic_key_type instanceof TFalse
                && !$atomic_key_type instanceof TTrue
            ) {
                return false;
            }
        }

        return true;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasLiteralValue(): bool
    {
        return $this->countLiteralInts() > 0
            || $this->countLiteralStrings() > 0
            || $this->countLiteralFloats() > 0
            || $this->has('false')
            || $this->has('true');
    }

    /**
     * @psalm-mutation-free
     */
    public function isSingleLiteral(): bool
    {
        return count($this->types) === 1
            && $this->countLiteralInts()
                + $this->countLiteralStrings()
                + $this->countLiteralFloats() === 1
        ;
    }

    /**
     * @psalm-mutation-free
     */
    public function getSingleLiteral(): TLiteralInt|TLiteralString|TLiteralFloat
    {
        if (!$this->isSingleLiteral()) {
            throw new InvalidArgumentException("Not a single literal");
        }

        return $this->firstLiteralInt()
            ?? $this->firstLiteralString()
            ?? $this->firstLiteralFloat()
            ?? throw new InvalidArgumentException("Not a single literal")
        ;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasLiteralString(): bool
    {
        return $this->countLiteralStrings() > 0;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasLiteralInt(): bool
    {
        return $this->countLiteralInts() > 0;
    }

    /**
     * @psalm-mutation-free
     * @return bool true if this is an int literal with only one possible value
     */
    public function isSingleIntLiteral(): bool
    {
        return count($this->types) === 1 && $this->countLiteralInts() === 1;
    }

    /**
     * @throws InvalidArgumentException if isSingleIntLiteral is false
     * @psalm-mutation-free
     * @return TLiteralInt the only int literal represented by this union type
     */
    public function getSingleIntLiteral(): TLiteralInt
    {
        if (count($this->types) !== 1 || $this->countLiteralInts() !== 1) {
            throw new InvalidArgumentException('Not an int literal');
        }

        return $this->firstLiteralInt() ?? throw new UnexpectedValueException('No literal int');
    }

    /**
     * @param  array<array-key, string>    $suppressed_issues
     * @param  array<string, bool> $phantom_classes
     */
    public function check(
        StatementsSource $source,
        CodeLocation $code_location,
        array $suppressed_issues,
        array $phantom_classes = [],
        bool $inferred = true,
        bool $inherited = false,
        bool $prevent_template_covariance = false,
        ?Context $context = null,
    ): bool {
        $checker = new TypeChecker(
            $source,
            $code_location,
            $suppressed_issues,
            $phantom_classes,
            $inferred,
            $inherited,
            $prevent_template_covariance,
            $context,
        );

        $checker->traverseArray($this->types);

        return !$checker->hasErrors();
    }

    /**
     * @param array<string, bool> $phantom_classes
     */
    public function queueClassLikesForScanning(
        Codebase $codebase,
        ?FileStorage $file_storage = null,
        array $phantom_classes = [],
    ): void {
        $scanner_visitor = new TypeScanner(
            $codebase->scanner,
            $file_storage,
            $phantom_classes,
        );

        /** @psalm-suppress ImpureMethodCall */
        $scanner_visitor->traverseArray($this->types);
    }

    /**
     * @param  lowercase-string $fq_class_like_name
     * @psalm-mutation-free
     */
    public function containsClassLike(string $fq_class_like_name): bool
    {
        $classlike_visitor = new ContainsClassLikeVisitor($fq_class_like_name);

        /** @psalm-suppress ImpureMethodCall Actually mutation-free */
        $classlike_visitor->traverseArray($this->types);

        return $classlike_visitor->matches();
    }

    /**
     * @return static
     */
    public function replaceClassLike(string $old, string $new): self
    {
        $type = $this;
        (new ClasslikeReplacer(
            $old,
            $new,
        ))->traverse($type);
        return $type;
    }

    /** @psalm-mutation-free */
    public function containsAnyLiteral(): bool
    {
        $literal_visitor = new ContainsLiteralVisitor();

        /** @psalm-suppress ImpureMethodCall Actually mutation-free */
        $literal_visitor->traverseArray($this->types);

        return $literal_visitor->matches();
    }

    /**
     * @psalm-mutation-free
     * @return list<TTemplateParam>
     */
    public function getTemplateTypes(): array
    {
        $template_type_collector = new TemplateTypeCollector();

        /** @psalm-suppress ImpureMethodCall Actually mutation-free */
        $template_type_collector->traverseArray($this->types);

        return $template_type_collector->getTemplateTypes();
    }

    /**
     * The ids of $parent_nodes, with those of nodes narrowed to a scalar (see
     * DataFlowNode::getForNarrowingToScalar()) replaced by the ids of the nodes they narrow, or null
     * if there is none of those.
     *
     * @param array<string, DataFlowNode> $parent_nodes
     * @return list<array-key>|null
     * @psalm-pure
     */
    private static function getParentNodeIdsWithoutNarrowing(array $parent_nodes): ?array
    {
        $parent_node_ids = [];
        $has_narrowing = false;

        foreach ($parent_nodes as $parent_node_id => $parent_node) {
            $narrowed_node_id = $parent_node->getNarrowedNodeId();

            if ($narrowed_node_id !== null) {
                $parent_node_id = $narrowed_node_id;
                $has_narrowing = true;
            }

            $parent_node_ids[$parent_node_id] = true;
        }

        return $has_narrowing ? array_keys($parent_node_ids) : null;
    }

    /**
     * @psalm-mutation-free
     */
    /**
     * Whether both unions hold the same atomics: exactly `getId() === getId()` (a union's id is the sorted set of
     * its atomics' ids), without building or comparing the union strings. Each atomic's id is memoized.
     *
     * @psalm-mutation-free
     */
    public function hasSameAtomics(self $other_type): bool
    {
        if ($other_type === $this) {
            return true;
        }

        // the same atomics on both sides: one atomic per key in a union (same count), each of ours must have its
        // structural equal under the same key (pzoom compares TUnion::types structurally; id strings of large
        // keyed arrays were 1.7 G instructions per run here)
        if (count($this->types) !== count($other_type->types)) {
            return false;
        }
        foreach ($this->types as $atomic) {
            $other_atomic = $other_type->find($atomic->getKey());
            if ($other_atomic === null || ($other_atomic !== $atomic && !$atomic->equals($other_atomic, false))) {
                return false;
            }
            // a named object's id carries what its key and equals() do not (`&static`, intersections): cheap to
            // compare, unlike a keyed array's
            if ($other_atomic !== $atomic && $atomic instanceof TNamedObject
                && $atomic->getId() !== $other_atomic->getId()
            ) {
                return false;
            }
        }
        return true;
    }

    public function equals(
        self $other_type,
        bool $ensure_source_equality = true,
        bool $ensure_parent_node_equality = true,
        bool $ensure_possibly_undefined_equality = true,
    ): bool {
        if ($other_type === $this) {
            return true;
        }

        $memo = $this->memo;
        $other_memo = $other_type->memo;
        if ($memo !== null && $other_memo !== null) {
            if ($other_memo->inexact_id !== null && $memo->inexact_id !== null && $other_memo->inexact_id !== $memo->inexact_id) {
                return false;
            }

            if ($other_memo->id !== null && $memo->id !== null && $other_memo->id !== $memo->id) {
                return false;
            }
        }

        if ($this->possibly_undefined !== $other_type->possibly_undefined && $ensure_possibly_undefined_equality) {
            return false;
        }

        if ($this->had_template !== $other_type->had_template) {
            return false;
        }

        if ($this->possibly_undefined_from_try !== $other_type->possibly_undefined_from_try) {
            return false;
        }

        if ($this->from_calculation !== $other_type->from_calculation) {
            return false;
        }

        if ($this->initialized !== $other_type->initialized) {
            return false;
        }

        if ($ensure_source_equality && $this->from_docblock !== $other_type->from_docblock) {
            return false;
        }

        if (count($this->types) !== count($other_type->types)) {
            return false;
        }

        if ($ensure_parent_node_equality && $this->parent_nodes !== $other_type->parent_nodes) {
            // A value narrowed to a type that cannot carry taints only hides its parent nodes from
            // the taint graph (see Reconciler): its data flow is the same.
            $parent_node_ids = self::getParentNodeIdsWithoutNarrowing($this->parent_nodes);
            $other_parent_node_ids = self::getParentNodeIdsWithoutNarrowing($other_type->parent_nodes);

            if (($parent_node_ids === null && $other_parent_node_ids === null)
                || ($parent_node_ids ?? array_keys($this->parent_nodes))
                    !== ($other_parent_node_ids ?? array_keys($other_type->parent_nodes))
            ) {
                return false;
            }
        }

        if ($this->different || $other_type->different) {
            return false;
        }

        // one atomic per key on both sides (same count): each of ours must have its equal under the same key
        foreach ($this->types as $atomic_type) {
            $other_atomic_type = $other_type->find($atomic_type->getKey());
            if ($other_atomic_type === null
                || !$atomic_type->equals($other_atomic_type, $ensure_source_equality)
            ) {
                return false;
            }
        }

        return true;
    }

    /**
     * @psalm-mutation-free
     * @return array<string, TLiteralString>
     */
    public function getLiteralStrings(): array
    {
        return $this->collectLiteralStrings();
    }

    /**
     * @psalm-mutation-free
     * @return array<string, TLiteralInt>
     */
    public function getLiteralInts(): array
    {
        return $this->collectLiteralInts();
    }

    /**
     * @psalm-mutation-free
     * @return array<string, TIntRange>
     */
    public function getRangeInts(): array
    {
        $ranges = [];
        foreach ($this->getAtomicTypes() as $atomic) {
            if ($atomic instanceof TIntRange) {
                $ranges[$atomic->getKey()] = $atomic;
            }
        }

        return $ranges;
    }

    /**
     * @psalm-mutation-free
     * @return array<string, TLiteralFloat>
     */
    public function getLiteralFloats(): array
    {
        return $this->collectLiteralFloats();
    }

    /**
     * @psalm-mutation-free
     * @return bool true if this is a float literal with only one possible value
     */
    public function isSingleFloatLiteral(): bool
    {
        return count($this->types) === 1 && $this->countLiteralFloats() === 1;
    }

    /**
     * @psalm-mutation-free
     * @throws InvalidArgumentException if isSingleFloatLiteral is false
     * @return TLiteralFloat the only float literal represented by this union type
     */
    public function getSingleFloatLiteral(): TLiteralFloat
    {
        if (count($this->types) !== 1 || $this->countLiteralFloats() !== 1) {
            throw new InvalidArgumentException('Not a float literal');
        }

        return $this->firstLiteralFloat() ?? throw new UnexpectedValueException('No literal float');
    }

    /**
     * @psalm-mutation-free
     */
    public function hasLiteralFloat(): bool
    {
        return $this->countLiteralFloats() > 0;
    }

    /**
     * @psalm-mutation-free
     */
    public function getSingleAtomic(): Atomic
    {
        return $this->types[0];
    }

    /**
     * @psalm-api
     * @psalm-mutation-free
     */
    public function isEmptyArray(): bool
    {
        return count($this->types) === 1
            && ($array = $this->find('array')) instanceof TArray
            && $array->isEmptyArray();
    }

    /**
     * @psalm-mutation-free
     * @psalm-suppress TypeDoesNotContainType a MutableUnion is empty between mutations
     */
    public function isUnionEmpty(): bool
    {
        return $this->types === [];
    }

    public function getTaintsToRemove(): int
    {
        // a value has a taint only if every type it can have holds it
        $taints_to_remove = TaintKind::ALL_INPUT;
        foreach ($this->types as $atomic) {
            $taints_to_remove &= self::getAtomicTaintsToRemove($atomic);
            if ($taints_to_remove === 0) {
                break;
            }
        }

        return $taints_to_remove;
    }

    /**
     * @psalm-pure
     */
    private static function getAtomicTaintsToRemove(Atomic $atomic): int
    {
        // a value the code wrote, or null, cannot hold what the client sends
        if ($atomic instanceof TLiteralString
            || $atomic instanceof TNonspecificLiteralString
            || $atomic instanceof TNull
        ) {
            return TaintKind::ALL_INPUT;
        }

        // numeric types can't be tainted (except sleep & custom taints), neither can bool
        if ($atomic instanceof TInt || $atomic instanceof TFloat) {
            return TaintKind::ALL_INPUT & ~TaintKind::NUMERIC_ONLY;
        }

        if ($atomic instanceof TBool) {
            return TaintKind::ALL_INPUT & ~TaintKind::BOOL_ONLY;
        }

        // a plain string can't carry a NoSQL query (only arrays/objects can),
        // so casting user input to string escapes the nosql taint
        if ($atomic instanceof TString) {
            return TaintKind::ARRAY_ONLY;
        }

        // an array holds what its keys and its values can hold: the keys of an array shape are written by the code
        if ($atomic instanceof TKeyedArray) {
            $taints_to_remove = TaintKind::ALL_INPUT;
            foreach ($atomic->properties as $property) {
                $taints_to_remove &= $property->getTaintsToRemove();
            }

            if ($atomic->fallback_params !== null) {
                $taints_to_remove &= self::getArrayKeyTaintsToRemove($atomic->fallback_params[0])
                    & $atomic->fallback_params[1]->getTaintsToRemove();
            }

            return $taints_to_remove;
        }

        if ($atomic instanceof TArray) {
            return self::getArrayKeyTaintsToRemove($atomic->type_params[0])
                & $atomic->type_params[1]->getTaintsToRemove();
        }

        return 0;
    }

    /**
     * Unlike a string value, a string key can carry a NoSQL query operator.
     *
     * @psalm-pure
     */
    private static function getArrayKeyTaintsToRemove(Union $key_type): int
    {
        $taints_to_remove = TaintKind::ALL_INPUT;
        foreach ($key_type->getAtomicTypes() as $atomic) {
            if ($atomic instanceof TLiteralString || $atomic instanceof TNever) {
                continue;
            }

            if (!$atomic instanceof TInt) {
                return 0;
            }

            $taints_to_remove &= TaintKind::ALL_INPUT & ~TaintKind::NUMERIC_ONLY;
        }

        return $taints_to_remove;
    }
    #[Override]
    public function visit(TypeVisitor $visitor): bool
    {
        foreach ($this->types as $type) {
            if ($visitor->traverse($type) === false) {
                return false;
            }
        }

        return true;
    }
}
