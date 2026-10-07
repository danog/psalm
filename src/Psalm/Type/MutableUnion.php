<?php

declare(strict_types=1);

namespace Psalm\Type;

use Override;
use Psalm\Internal\DataFlow\DataFlowNode;
use Psalm\Internal\Type\TypeCombiner;
use Psalm\Internal\TypeVisitor\FromDocblockSetter;
use Psalm\Type;
use Psalm\Type\Atomic\IdMemo;
use Psalm\Type\Atomic\Scalar;
use Psalm\Type\Atomic\TArray;
use Psalm\Type\Atomic\TClassString;
use Psalm\Type\Atomic\TFalse;
use Psalm\Type\Atomic\TFloat;
use Psalm\Type\Atomic\TInt;
use Psalm\Type\Atomic\TIntRange;
use Psalm\Type\Atomic\TLiteralFloat;
use Psalm\Type\Atomic\TLiteralInt;
use Psalm\Type\Atomic\TLiteralString;
use Psalm\Type\Atomic\TMixed;
use Psalm\Type\Atomic\TNamedObject;
use Psalm\Type\Atomic\TNever;
use Psalm\Type\Atomic\TString;
use Psalm\Type\Atomic\TTemplateParamClass;
use Psalm\Type\Atomic\TTrue;

use function array_values;
use function assert;
use function count;

/**
 * @api
 */
final class MutableUnion implements TypeNode
{
    use UnionTrait;

    /**
     * @var non-empty-list<Atomic> (empty only between mutations)
     */
    private array $types;

    /**
     * Whether the type originated in a docblock
     */
    public bool $from_docblock = false;

    /**
     * Whether the type originated from integer calculation
     */
    public bool $from_calculation = false;

    /**
     * Whether the type originated from a property
     *
     * This helps turn isset($foo->bar) into a different sort of issue
     */
    public bool $from_property = false;

    /**
     * Whether the type originated from *static* property
     *
     * Unlike non-static properties, static properties have no prescribed place
     * like __construct() to be initialized in
     */
    public bool $from_static_property = false;

    /**
     * Whether the value was reached from global state
     */
    public bool $from_global_state = false;

    /**
     * Whether the property that this type has been derived from has been initialized in a constructor
     */
    public bool $initialized = true;

    /**
     * Which class the type was initialised in
     */
    public ?string $initialized_class = null;

    /**
     * Whether or not the type has been checked yet
     */
    public bool $checked = false;

    public bool $failed_reconciliation = false;

    /**
     * Whether or not to ignore issues with possibly-null values
     */
    public bool $ignore_nullable_issues = false;

    /**
     * Whether or not to ignore issues with possibly-false values
     */
    public bool $ignore_falsable_issues = false;

    /**
     * Whether or not to ignore issues with isset on this type
     */
    public bool $ignore_isset = false;

    /**
     * Whether or not this variable is possibly undefined
     */
    public bool $possibly_undefined = false;

    /**
     * Whether or not this variable is possibly undefined
     */
    public bool $possibly_undefined_from_try = false;

    /**
     * whether this type had never set explicitly
     * since it's the bottom type, it's combined into everything else and lost
     */
    public bool $explicit_never = false;

    /**
     * Whether or not this union had a template, since replaced
     */
    public bool $had_template = false;

    /**
     * Whether or not this union comes from a template "as" default
     */
    public bool $from_template_default = false;

    /**
     * True if the type was passed or returned by reference, or if the type refers to an object's
     * property or an item in an array. Note that this is not true for locally created references
     * that don't refer to properties or array items (see Context::$references_in_scope).
     */
    public bool $by_ref = false;

    public bool $reference_free = false;

    public bool $allow_mutations = true;

    public bool $has_mutations = true;

    /** The memoized getId(true) / getId(false) strings (IdMemo::$id / IdMemo::$inexact_id), allocated on first use */
    private ?IdMemo $memo = null;

    /**
     * @var array<string, DataFlowNode>
     */
    public array $parent_nodes = [];

    public bool $different = false;

    public bool $propagate_parent_nodes = false;

    /**
     * @param non-empty-array<array-key, Atomic> $types
     * @return static
     * @psalm-external-mutation-free
     */
    public function setTypes(array $types): self
    {
        $this->checked = false;
        $this->types = self::listOfTypes(array_values($types));

        $from_docblock = false;
        foreach ($this->types as $type) {
            if ($type instanceof TNever) {
                $this->explicit_never = true;
            }
            $from_docblock = $from_docblock || $type->from_docblock;
        }
        $this->from_docblock = $from_docblock;
        $this->bustCache();

        return $this;
    }

    /**
     * Stores an atomic under its key: in place of the one with the same key, else appended.
     *
     * @psalm-external-mutation-free
     */
    private function put(Atomic $type): void
    {
        $key = $type->getKey();
        foreach ($this->types as $i => $existing) {
            if ($existing->getKey() === $key) {
                $this->types[$i] = $type;
                return;
            }
        }
        $this->types[] = $type;
    }

    /**
     * Removes the atomic with this key, if any.
     *
     * @psalm-external-mutation-free
     */
    private function drop(string $key): bool
    {
        foreach ($this->types as $i => $existing) {
            if ($existing->getKey() === $key) {
                unset($this->types[$i]);
                /** @psalm-suppress InvalidPropertyAssignmentValue transiently empty */
                $this->types = array_values($this->types);
                return true;
            }
        }
        return false;
    }

    /**
     * @psalm-external-mutation-free
     */
    public function addType(Atomic $type): self
    {
        $this->put($type);

        if ($type instanceof TLiteralString || $type instanceof TLiteralInt || $type instanceof TLiteralFloat) {
            // a literal joins the others
        } elseif ($type instanceof TString) {
            if ($this->countLiteralStrings() > 0) {
                // the literal strings are covered by the wider string, and so are the bounded class-strings
                // unless the wider string is itself one
                $keep_typed_class_strings = $type instanceof TClassString
                    && ($type->as_type || $type instanceof TTemplateParamClass);
                $kept = [];
                foreach ($this->types as $existing) {
                    if ($existing instanceof TLiteralString) {
                        continue;
                    }
                    if (!$keep_typed_class_strings
                        && $existing instanceof TClassString
                        && ($existing->as_type || $existing instanceof TTemplateParamClass)
                    ) {
                        continue;
                    }
                    $kept[] = $existing;
                }
                /** @psalm-suppress InvalidPropertyAssignmentValue transiently empty */
                $this->types = $kept;
            }
        } elseif ($type instanceof TInt) {
            if ($this->countLiteralInts() > 0) {
                // we remove any literal that is already included in a wider type
                $int_type_in_range = TIntRange::convertToIntRange($type);
                $kept = [];
                foreach ($this->types as $existing) {
                    if ($existing instanceof TLiteralInt && $int_type_in_range->contains($existing->value)) {
                        continue;
                    }
                    $kept[] = $existing;
                }
                /** @psalm-suppress InvalidPropertyAssignmentValue transiently empty */
                $this->types = $kept;
            }
        } elseif ($type instanceof TFloat) {
            if ($this->countLiteralFloats() > 0) {
                $kept = [];
                foreach ($this->types as $existing) {
                    if (!$existing instanceof TLiteralFloat) {
                        $kept[] = $existing;
                    }
                }
                /** @psalm-suppress InvalidPropertyAssignmentValue transiently empty */
                $this->types = $kept;
            }
        } elseif ($type instanceof TNever) {
            $this->explicit_never = true;
        }

        $this->bustCache();

        return $this;
    }

    /**
     * Removes the atomic with this key. Removing `string`, `int` or `float` when no such atomic exists
     * removes the literals (and, for `string`, the class-strings) it would have covered instead, and
     * reports false as before.
     *
     * @psalm-external-mutation-free
     */
    public function removeType(string $type_string): bool
    {
        if ($this->drop($type_string)) {
            $this->bustCache();

            return true;
        }

        if ($type_string === 'string') {
            $kept = [];
            foreach ($this->types as $existing) {
                if ($existing instanceof TLiteralString
                    || ($existing instanceof TClassString
                        && ($existing->as_type || $existing instanceof TTemplateParamClass))
                ) {
                    continue;
                }
                $key = $existing->getKey();
                if ($key === 'class-string' || $key === 'trait-string') {
                    continue;
                }
                $kept[] = $existing;
            }
            if (count($kept) !== count($this->types)) {
                /** @psalm-suppress InvalidPropertyAssignmentValue transiently empty */
                $this->types = $kept;
                $this->bustCache();
            }
        } elseif ($type_string === 'int' && $this->countLiteralInts() > 0) {
            $kept = [];
            foreach ($this->types as $existing) {
                if (!$existing instanceof TLiteralInt) {
                    $kept[] = $existing;
                }
            }
            /** @psalm-suppress InvalidPropertyAssignmentValue transiently empty */
            $this->types = $kept;
            $this->bustCache();
        } elseif ($type_string === 'float' && $this->countLiteralFloats() > 0) {
            $kept = [];
            foreach ($this->types as $existing) {
                if (!$existing instanceof TLiteralFloat) {
                    $kept[] = $existing;
                }
            }
            /** @psalm-suppress InvalidPropertyAssignmentValue transiently empty */
            $this->types = $kept;
            $this->bustCache();
        }

        return false;
    }

    public function setFromDocblock(bool $fromDocblock = true): self
    {
        $this->from_docblock = $fromDocblock;

        (new FromDocblockSetter($fromDocblock))->traverseArray($this->types);

        return $this;
    }

    /**
     * @psalm-external-mutation-free
     */
    public function bustCache(): void
    {
        $this->memo = null;
    }

    /**
     * @psalm-external-mutation-free
     */
    public function substitute(Union|MutableUnion $old_type, Union|MutableUnion|null $new_type = null): self
    {
        if ($this->hasMixed() && !$this->isEmptyMixed()) {
            return $this;
        }
        $old_type = $old_type->getBuilder();
        if ($new_type) {
            $new_type = $new_type->getBuilder();
        }

        if ($new_type && $new_type->ignore_nullable_issues) {
            $this->ignore_nullable_issues = true;
        }

        if ($new_type && $new_type->ignore_falsable_issues) {
            $this->ignore_falsable_issues = true;
        }

        foreach ($old_type->types as $old_type_part) {
            $had = $this->has($old_type_part->getKey());
            $this->removeType($old_type_part->getKey());
            if (!$had) {
                if ($old_type_part instanceof TFalse
                    && $this->has('bool')
                    && !$this->has('true')
                ) {
                    $this->removeType('bool');
                    $this->put(new TTrue);
                } elseif ($old_type_part instanceof TTrue
                    && $this->has('bool')
                    && !$this->has('false')
                ) {
                    $this->removeType('bool');
                    $this->put(new TFalse);
                } elseif ($this->has('iterable')) {
                    if ($old_type_part instanceof TNamedObject
                        && $old_type_part->value === 'Traversable'
                        && !$this->has('array')
                    ) {
                        $this->removeType('iterable');
                        $this->put(Type::getArrayAtomic());
                    }

                    if ($old_type_part instanceof TArray
                        && !$this->has('traversable')
                    ) {
                        $this->removeType('iterable');
                        $this->put(new TNamedObject('Traversable'));
                    }
                } elseif ($this->has('array-key')) {
                    if ($old_type_part instanceof TString
                        && !$this->has('int')
                    ) {
                        $this->removeType('array-key');
                        $this->put(new TInt());
                    }

                    if ($old_type_part instanceof TInt
                        && !$this->has('string')
                    ) {
                        $this->removeType('array-key');
                        $this->put(new TString());
                    }
                }
            }
        }

        if ($new_type) {
            foreach ($new_type->types as $new_type_part) {
                $existing = $this->find($new_type_part->getKey());
                if ($existing === null
                    || ($new_type_part instanceof Scalar
                        && $new_type_part::class === $existing::class)
                ) {
                    $this->put($new_type_part);
                } else {
                    $this->put(TypeCombiner::combine([$new_type_part, $existing])->getSingleAtomic());
                }
            }
        } else {
            /** @psalm-suppress TypeDoesNotContainType transiently empty */
            if (count($this->types) === 0) {
                $this->put(new TMixed());
            }
        }

        $this->bustCache();

        return $this;
    }

    /**
     * @psalm-mutation-free
     */
    public function getBuilder(): self
    {
        return $this;
    }

    /**
     * @psalm-mutation-free
     */
    public function freeze(): Union
    {
        return new Union($this->getAtomicTypes(), $this->getConstructionProperties());
    }

    /**
     * @phpcsSuppress SlevomatCodingStandard.TypeHints.ParameterTypeHint.MissingAnyTypeHint
     * @param TypeNode $node
     */
    #[Override]
    public static function visitMutable(MutableTypeVisitor $visitor, &$node, bool $cloned): bool
    {
        assert($node instanceof self);
        $result = true;
        $changed = false;
        foreach ($node->types as &$type) {
            $type_orig = $type;
            $result = $visitor->traverse($type);
            $changed = $changed || $type_orig !== $type;
            if (!$result) {
                break;
            }
        }
        unset($type);

        if ($changed) {
            $node->setTypes($node->types);
        }

        return $result;
    }
}
