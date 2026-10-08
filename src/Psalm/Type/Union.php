<?php

declare(strict_types=1);

namespace Psalm\Type;

use Override;
use Psalm\Type\Atomic\IdMemo;
use Psalm\Type\Atomic\TClassStringMap;
use Psalm\Type\Atomic\TObjectWithProperties;
use Psalm\Type\Atomic\TIterable;
use Psalm\Type\Atomic\TGenericObject;
use Psalm\Type\Atomic\TArray;
use Psalm\Type\Atomic\TKeyedArray;
use Psalm\Internal\DataFlow\DataFlowNode;
use Psalm\Internal\TypeVisitor\ExpansionTraitsCollector;
use Psalm\Internal\TypeVisitor\FromDocblockSetter;
use Psalm\Storage\ImmutableNonCloneableTrait;
use Psalm\Type\Atomic\TClassString;
use Psalm\Type\Atomic\TLiteralFloat;
use Psalm\Type\Atomic\TLiteralInt;
use Psalm\Type\Atomic\TLiteralString;

use function assert;
use function array_key_exists;
use function get_object_vars;

/**
 * @psalm-immutable
 * @psalm-type TProperties=array{
 *      from_docblock?: bool,
 *      from_calculation?: bool,
 *      from_property?: bool,
 *      from_static_property?: bool,
 *      from_global_state?: bool,
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
 *      parent_nodes?: array<int, DataFlowNode>
 * }
 * @api
 */
final class Union implements TypeNode
{
    use ImmutableNonCloneableTrait;
    use UnionTrait;

    /**
     * @psalm-readonly
     * @var non-empty-list<Atomic>
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
     * Whether the value was reached from global state: read from a static property, a
     * superglobal or a `global` variable, returned by a callee that reads globals, or fetched
     * from such a value. Mutating it needs the write-globals capability.
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
     * ExpansionTraitsCollector traits of this union (-1: not computed yet); memoized with the expansions below
     * (types are immutable), dropped on clone since a wither may change the atomics.
     */
    private int $expansion_traits = -1;

    /**
     * Memoized expansions (TypeExpander::expandUnion) by context key, valid for $expansions_epoch
     * (ClassLikes::$expansion_epoch). An expansion that returns this union itself is in $expansions_unchanged
     * instead (no self-reference: the compiled program would leak the cycle). Dropped on clone: an expansion
     * carries the flags of the union it expands.
     *
     * @var array<string, Union>
     */
    private array $expansions = [];

    /** @var array<string, true> */
    private array $expansions_unchanged = [];

    private int $expansions_epoch = 0;


    /**
     * @var array<int, DataFlowNode>
     */
    public array $parent_nodes = [];

    public bool $propagate_parent_nodes = false;

    public bool $different = false;

    private const PROPERTY_KEYS_FOR_UNSERIALIZE = [
        "\0" . self::class . "\0" . 'types' => 'types',
        'from_docblock' => 'from_docblock',
        'from_calculation' => 'from_calculation',
        'from_property' => 'from_property',
        'from_static_property' => 'from_static_property',
        'from_global_state' => 'from_global_state',
        'initialized' => 'initialized',
        'initialized_class' => 'initialized_class',
        'checked' => 'checked',
        'failed_reconciliation' => 'failed_reconciliation',
        'ignore_nullable_issues' => 'ignore_nullable_issues',
        'ignore_falsable_issues' => 'ignore_falsable_issues',
        'ignore_isset' => 'ignore_isset',
        'possibly_undefined' => 'possibly_undefined',
        'possibly_undefined_from_try' => 'possibly_undefined_from_try',
        'explicit_never' => 'explicit_never',
        'had_template' => 'had_template',
        'from_template_default' => 'from_template_default',
        'by_ref' => 'by_ref',
        'reference_free' => 'reference_free',
        'allow_mutations' => 'allow_mutations',
        'has_mutations' => 'has_mutations',
        'parent_nodes' => 'parent_nodes',
        'propagate_parent_nodes' => 'propagate_parent_nodes',
        'different' => 'different',
    ];

    /**
     * Suppresses memory usage when unserializing objects.
     *
     * @see \Psalm\Storage\UnserializeMemoryUsageSuppressionTrait
     */
    public function __unserialize(array $properties): void
    {
        foreach (self::PROPERTY_KEYS_FOR_UNSERIALIZE as $key => $property_name) {
            /** @psalm-suppress PossiblyUndefinedStringArrayOffset */
            $this->$property_name = $properties[$key];
        }

        // parent nodes are keyed by DataFlowNode::$key, which is process-local: the nodes (unserialized before
        // this union) were re-keyed for this process, so are they here
        if ($this->parent_nodes) {
            $parent_nodes = [];
            foreach ($this->parent_nodes as $parent_node) {
                $parent_nodes[$parent_node->key] = $parent_node;
            }
            $this->parent_nodes = $parent_nodes;
        }
    }

    /**
     * pzoom's `TUnion::eq`, as combineUnionTypes uses it: the same atomics (compared first, since most pairs
     * differ there; a wither's clone shares the atomics array, so `===` is a pointer comparison, else an
     * element-wise identity check in C, then a per-key id comparison) and the same flags a combination merges.
     * Per-atomic docblock provenance is not compared.
     *
     * @psalm-mutation-free
     */
    public function isCombineEquivalent(Union $other): bool
    {
        if ($this === $other) {
            return true;
        }

        return $this->hasCombineEquivalentAtomics($other)
            && $this->from_docblock === $other->from_docblock
            && $this->from_calculation === $other->from_calculation
            && $this->from_global_state === $other->from_global_state
            && $this->ignore_nullable_issues === $other->ignore_nullable_issues
            && $this->ignore_falsable_issues === $other->ignore_falsable_issues
            && $this->reference_free === $other->reference_free
            && $this->allow_mutations === $other->allow_mutations
            && $this->initialized === $other->initialized
            && $this->explicit_never === $other->explicit_never
            && $this->had_template === $other->had_template
            && $this->failed_reconciliation === $other->failed_reconciliation
            && $this->possibly_undefined === $other->possibly_undefined
            && $this->possibly_undefined_from_try === $other->possibly_undefined_from_try
            && $this->by_ref === $other->by_ref
            && $this->hasSameParentNodes($other);
    }

    /**
     * The same data-flow nodes: a node is its id (the graphs key by it), so two unions carrying nodes with the
     * same ids in the same order are equally sourced even when the node objects differ (a property fetch makes
     * a fresh node for the same id on every visit).
     *
     * @psalm-mutation-free
     */
    public function hasSameParentNodes(Union $other): bool
    {
        if ($this->parent_nodes === $other->parent_nodes) {
            return true;
        }
        if (count($this->parent_nodes) !== count($other->parent_nodes)) {
            return false;
        }
        foreach ($this->parent_nodes as $key => $_) {
            if (!isset($other->parent_nodes[$key])) {
                return false;
            }
        }
        return true;
    }

    /**
     * The atomics half of isCombineEquivalent(): the same atomic types by id (containers structurally), with
     * the same per-atomic docblock provenance when $same_docblock is set (the combiner ORs it per atomic).
     *
     * @psalm-mutation-free
     */
    public function hasCombineEquivalentAtomics(Union $other, bool $same_docblock = false): bool
    {
        if ($this->types !== $other->types) {
            if (count($this->types) !== count($other->types)) {
                return false;
            }
            // lists built from the same atomics keep their order, so compare by position and only look
            // the key up when the orders differ
            foreach ($this->types as $i => $atomic) {
                $theirs = $other->types[$i];
                if ($theirs === $atomic) {
                    continue;
                }
                if ($theirs->getKey() !== $atomic->getKey()) {
                    $theirs = $other->find($atomic->getKey());
                    if ($theirs === null) {
                        return false;
                    }
                }
                if ($theirs !== $atomic) {
                    if ($theirs::class !== $atomic::class) {
                        return false;
                    }
                    // containers compare their shape structurally (pzoom's TAtomic equality); for a fresh keyed
                    // array the id would be its whole shape spelled out. Other atomics are their memoized id.
                    if ($atomic instanceof TKeyedArray
                        || $atomic instanceof TArray
                        || $atomic instanceof TGenericObject
                        || $atomic instanceof TIterable
                        || $atomic instanceof TObjectWithProperties
                        || $atomic instanceof TClassStringMap
                    ) {
                        if (!$atomic->equals($theirs, true)) {
                            return false;
                        }
                    } elseif ($theirs->getId() !== $atomic->getId()) {
                        return false;
                    }
                    if ($same_docblock && $theirs->from_docblock !== $atomic->from_docblock) {
                        return false;
                    }
                }
            }
        }

        return true;
    }

    /**
     * @param TProperties $properties
     * @return static
     * @psalm-suppress ImpurePropertyAssignment, InaccessibleProperty We just cloned this object
     */
    public function setProperties(array $properties): self
    {
        $obj = null;
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
        if (array_key_exists('from_global_state', $properties) && $this->from_global_state !== $properties['from_global_state']) {
            $obj ??= clone $this;
            $obj->from_global_state = $properties['from_global_state'];
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
     * @param array<int, DataFlowNode> $parent_nodes
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
     * @param array<int, DataFlowNode> $parent_nodes
     * @return static
     */
    public function addParentNodes(array $parent_nodes): self
    {
        if (!$parent_nodes) {
            return $this;
        }
        $parent_nodes = DataFlowNode::combineParentNodes($this->parent_nodes, $parent_nodes);
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
     * @param non-empty-list<Atomic>|non-empty-array<string, Atomic>  $types
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
        return new MutableUnion($this->types, $this->getConstructionProperties(), true);
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
     * @param TypeNode $node
     * @param-out TypeNode $node
     *
     * @phpcsSuppress SlevomatCodingStandard.TypeHints.ParameterTypeHint.MissingAnyTypeHint
     */
    #[Override]
    public static function visitMutable(MutableTypeVisitor $visitor, &$node, bool $cloned): bool
    {
        assert($node instanceof self);
        $self = $node;
        $result = true;
        $changed = false;
        $types = $self->types;
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
            $self = $self->setTypes($types);
        }

        $node = $self;

        return $result;
    }

    /** @psalm-mutation-free */
    private function __clone()
    {
        $this->expansion_traits = -1;
        $this->expansions = [];
        $this->expansions_unchanged = [];
        $this->expansions_epoch = 0;
    }

    /**
     * ExpansionTraitsCollector traits of this union, memoized.
     *
     * @internal
     * @psalm-mutation-free
     */
    public function getExpansionTraits(): int
    {
        if ($this->expansion_traits === -1) {
            $collector = new ExpansionTraitsCollector();
            $collector->traverseArray($this->types);
            /** @psalm-suppress ImpurePropertyAssignment memo of an immutable value */
            $this->expansion_traits = $collector->getTraits();
        }
        return $this->expansion_traits;
    }

    /**
     * The memoized expansion for a context key in an expansion epoch, null if none.
     *
     * @internal
     * @psalm-mutation-free
     */
    public function getMemoizedExpansion(int $epoch, string $key): ?Union
    {
        if ($this->expansions_epoch !== $epoch) {
            return null;
        }
        if (isset($this->expansions_unchanged[$key])) {
            return $this;
        }
        return $this->expansions[$key] ?? null;
    }

    /**
     * Memoize the expansion for a context key in an expansion epoch.
     *
     * @internal
     * @psalm-mutation-free
     */
    public function memoizeExpansion(int $epoch, string $key, Union $expansion): void
    {
        /** @psalm-suppress ImpurePropertyAssignment memo of an immutable value */
        if ($this->expansions_epoch !== $epoch) {
            $this->expansions = [];
            $this->expansions_unchanged = [];
            $this->expansions_epoch = $epoch;
        }
        /** @psalm-suppress ImpurePropertyAssignment memo of an immutable value */
        if ($expansion === $this) {
            $this->expansions_unchanged[$key] = true;
        } else {
            $this->expansions[$key] = $expansion;
        }
    }
}
