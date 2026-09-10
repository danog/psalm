<?php

declare(strict_types=1);

namespace Psalm\Type;

use Override;
use Psalm\Internal\DataFlow\DataFlowNode;
use Psalm\Internal\TypeVisitor\FromDocblockSetter;
use Psalm\Storage\ImmutableNonCloneableTrait;
use Psalm\Type\Atomic\TClassString;
use Psalm\Type\Atomic\TLiteralFloat;
use Psalm\Type\Atomic\TLiteralInt;
use Psalm\Type\Atomic\TLiteralString;

use function array_key_exists;
use function get_object_vars;

/**
 * @psalm-immutable
 * @psalm-type TProperties=array{
 *      from_docblock?: bool,
 *      from_calculation?: bool,
 *      from_property?: bool,
 *      from_static_property?: bool,
 *      initialized?: bool,
 *      initialized_class?: ?string,
 *      checked?: bool,
 *      failed_reconciliation?: bool,
 *      ignore_nullable_issues?: bool,
 *      ignore_falsable_issues?: bool,
 *      ignore_isset?: bool,
 *      possibly_undefined?: bool,
 *      possibly_undefined_from_try?: bool,
 *      explicit_never?: bool,
 *      had_template?: bool,
 *      from_template_default?: bool,
 *      by_ref?: bool,
 *      reference_free?: bool,
 *      allow_mutations?: bool,
 *      has_mutations?: bool,
 *      different?: bool,
 *      parent_nodes?: array<string, DataFlowNode>
 * }
 */
final class Union implements TypeNode
{
    use ImmutableNonCloneableTrait;
    use UnionTrait;

    /**
     * @psalm-readonly
     * @var non-empty-array<string, Atomic>
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
     * @var array<string, TLiteralString>
     */
    private array $literal_string_types = [];

    /**
     * @var array<string, TClassString>
     */
    private array $typed_class_strings = [];

    /**
     * @var array<string, TLiteralInt>
     */
    private array $literal_int_types = [];

    /**
     * @var array<string, TLiteralFloat>
     */
    private array $literal_float_types = [];

    /**
     * True if the type was passed or returned by reference, or if the type refers to an object's
     * property or an item in an array. Note that this is not true for locally created references
     * that don't refer to properties or array items (see Context::$references_in_scope).
     */
    public bool $by_ref = false;

    public bool $reference_free = false;

    public bool $allow_mutations = true;

    public bool $has_mutations = true;

    /**
     * This is a cache of getId on non-exact mode
     */
    private ?string $id = null;

    /**
     * This is a cache of getId on exact mode
     */
    private ?string $exact_id = null;


    /**
     * @var array<string, DataFlowNode>
     */
    public array $parent_nodes = [];

    public bool $propagate_parent_nodes = false;

    public bool $different = false;

    // serialized property keys: see __unserialize()

    /**
     * Suppresses memory usage when unserializing objects.
     *
     * @see \Psalm\Storage\UnserializeMemoryUsageSuppressionTrait
     */
    public function __unserialize(array $properties): void
    {
        // objects are never unserialized in the compiled program; property names cannot be looked up dynamically
        throw new \LogicException('Unserialization of ' . self::class . ' is not supported');
    }

    /**
     * @param TProperties $properties
     * @return static
     */
    public function setProperties(array $properties): self
    {
        $obj = null;
        /** @psalm-suppress ImpurePropertyAssignment We just cloned this object */
        if (array_key_exists('from_docblock', $properties) && $this->from_docblock !== $properties['from_docblock']) {
            $obj ??= clone $this;
            $obj->from_docblock = $properties['from_docblock'];
        }
        if (array_key_exists('from_calculation', $properties) && $this->from_calculation !== $properties['from_calculation']) {
            $obj ??= clone $this;
            $obj->from_calculation = $properties['from_calculation'];
        }
        if (array_key_exists('from_property', $properties) && $this->from_property !== $properties['from_property']) {
            $obj ??= clone $this;
            $obj->from_property = $properties['from_property'];
        }
        if (array_key_exists('from_static_property', $properties) && $this->from_static_property !== $properties['from_static_property']) {
            $obj ??= clone $this;
            $obj->from_static_property = $properties['from_static_property'];
        }
        if (array_key_exists('initialized', $properties) && $this->initialized !== $properties['initialized']) {
            $obj ??= clone $this;
            $obj->initialized = $properties['initialized'];
        }
        if (array_key_exists('initialized_class', $properties) && $this->initialized_class !== $properties['initialized_class']) {
            $obj ??= clone $this;
            $obj->initialized_class = $properties['initialized_class'];
        }
        if (array_key_exists('checked', $properties) && $this->checked !== $properties['checked']) {
            $obj ??= clone $this;
            $obj->checked = $properties['checked'];
        }
        if (array_key_exists('failed_reconciliation', $properties) && $this->failed_reconciliation !== $properties['failed_reconciliation']) {
            $obj ??= clone $this;
            $obj->failed_reconciliation = $properties['failed_reconciliation'];
        }
        if (array_key_exists('ignore_nullable_issues', $properties) && $this->ignore_nullable_issues !== $properties['ignore_nullable_issues']) {
            $obj ??= clone $this;
            $obj->ignore_nullable_issues = $properties['ignore_nullable_issues'];
        }
        if (array_key_exists('ignore_falsable_issues', $properties) && $this->ignore_falsable_issues !== $properties['ignore_falsable_issues']) {
            $obj ??= clone $this;
            $obj->ignore_falsable_issues = $properties['ignore_falsable_issues'];
        }
        if (array_key_exists('ignore_isset', $properties) && $this->ignore_isset !== $properties['ignore_isset']) {
            $obj ??= clone $this;
            $obj->ignore_isset = $properties['ignore_isset'];
        }
        if (array_key_exists('possibly_undefined', $properties) && $this->possibly_undefined !== $properties['possibly_undefined']) {
            $obj ??= clone $this;
            $obj->possibly_undefined = $properties['possibly_undefined'];
        }
        if (array_key_exists('possibly_undefined_from_try', $properties) && $this->possibly_undefined_from_try !== $properties['possibly_undefined_from_try']) {
            $obj ??= clone $this;
            $obj->possibly_undefined_from_try = $properties['possibly_undefined_from_try'];
        }
        if (array_key_exists('explicit_never', $properties) && $this->explicit_never !== $properties['explicit_never']) {
            $obj ??= clone $this;
            $obj->explicit_never = $properties['explicit_never'];
        }
        if (array_key_exists('had_template', $properties) && $this->had_template !== $properties['had_template']) {
            $obj ??= clone $this;
            $obj->had_template = $properties['had_template'];
        }
        if (array_key_exists('from_template_default', $properties) && $this->from_template_default !== $properties['from_template_default']) {
            $obj ??= clone $this;
            $obj->from_template_default = $properties['from_template_default'];
        }
        if (array_key_exists('by_ref', $properties) && $this->by_ref !== $properties['by_ref']) {
            $obj ??= clone $this;
            $obj->by_ref = $properties['by_ref'];
        }
        if (array_key_exists('reference_free', $properties) && $this->reference_free !== $properties['reference_free']) {
            $obj ??= clone $this;
            $obj->reference_free = $properties['reference_free'];
        }
        if (array_key_exists('allow_mutations', $properties) && $this->allow_mutations !== $properties['allow_mutations']) {
            $obj ??= clone $this;
            $obj->allow_mutations = $properties['allow_mutations'];
        }
        if (array_key_exists('has_mutations', $properties) && $this->has_mutations !== $properties['has_mutations']) {
            $obj ??= clone $this;
            $obj->has_mutations = $properties['has_mutations'];
        }
        if (array_key_exists('different', $properties) && $this->different !== $properties['different']) {
            $obj ??= clone $this;
            $obj->different = $properties['different'];
        }
        if (array_key_exists('parent_nodes', $properties) && $this->parent_nodes !== $properties['parent_nodes']) {
            $obj ??= clone $this;
            $obj->parent_nodes = $properties['parent_nodes'];
        }
        return $obj ?? $this;
    }

    /**
     * @return static
     */
    public function setDifferent(bool $different): self
    {
        if ($different === $this->different) {
            return $this;
        }
        $cloned = clone $this;
        $cloned->different = $different;
        return $cloned;
    }

    /**
     * @param array<string, DataFlowNode> $parent_nodes
     * @return static
     */
    public function setParentNodes(array $parent_nodes, bool $propagate_changes = false): self
    {
        if ($parent_nodes === $this->parent_nodes) {
            return $this;
        }
        $cloned = clone $this;
        $cloned->parent_nodes = $parent_nodes;
        $cloned->propagate_parent_nodes = $propagate_changes;
        return $cloned;
    }


    /**
     * @param array<string, DataFlowNode> $parent_nodes
     * @return static
     */
    public function addParentNodes(array $parent_nodes): self
    {
        if (!$parent_nodes) {
            return $this;
        }
        $parent_nodes = $this->parent_nodes + $parent_nodes;
        if ($parent_nodes === $this->parent_nodes) {
            return $this;
        }
        $cloned = clone $this;
        $cloned->parent_nodes = $parent_nodes;
        return $cloned;
    }

    /** @return static */
    public function setPossiblyUndefined(bool $possibly_undefined, ?bool $from_try = null): self
    {
        $from_try ??= $this->possibly_undefined_from_try;
        if ($this->possibly_undefined === $possibly_undefined
            && $this->possibly_undefined_from_try == $from_try
        ) {
            return $this;
        }
        $cloned = clone $this;
        $cloned->possibly_undefined = $possibly_undefined;
        $cloned->possibly_undefined_from_try = $from_try;
        return $cloned;
    }

    /** @return static */
    public function setByRef(bool $by_ref): static
    {
        if ($by_ref === $this->by_ref) {
            return $this;
        }
        $cloned = clone $this;
        $cloned->by_ref = $by_ref;
        return $cloned;
    }

    /**
     * @psalm-mutation-free
     * @param non-empty-array<Atomic>  $types
     */
    public function setTypes(array $types): self
    {
        if ($types === $this->types) {
            return $this;
        }
        return $this->getBuilder()->setTypes($types)->freeze();
    }

    /**
     * @psalm-mutation-free
     */
    public function getBuilder(): MutableUnion
    {
        /** @psalm-suppress InvalidArgument It's actually filtered internally */
        return new MutableUnion($this->getAtomicTypes(), $this->getConstructionProperties());
    }

    /**
     * @psalm-mutation-free
     */
    public function setFromDocblock(bool $fromDocblock = true): self
    {
        $cloned = clone $this;
        /** @psalm-suppress ImpureMethodCall Acting on clone */
        (new FromDocblockSetter($fromDocblock))->traverse($cloned);
        return $cloned;
    }

    /**
     * @phpcsSuppress SlevomatCodingStandard.TypeHints.ParameterTypeHint.MissingAnyTypeHint
     */
    #[Override]
    public static function visitMutable(MutableTypeVisitor $visitor, &$node, bool $cloned): bool
    {
        $result = true;
        $changed = false;
        $types = $node->types;
        foreach ($types as &$type) {
            $type_orig = $type;
            $result = $visitor->traverse($type);
            $changed = $changed || $type_orig !== $type;
            if (!$result) {
                break;
            }
        }
        unset($type);

        if ($changed) {
            $node = $node->setTypes($types);
        }

        return $result;
    }
}
