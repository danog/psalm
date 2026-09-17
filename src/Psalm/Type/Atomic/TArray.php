<?php

declare(strict_types=1);

namespace Psalm\Type\Atomic;

use Override;
use Psalm\Codebase;
use Psalm\Internal\Analyzer\StatementsAnalyzer;
use Psalm\Internal\Type\TemplateResult;
use Psalm\Type\Atomic;
use Psalm\Type\Union;

use function assert;
use function count;
use Psalm\Type\MutableTypeVisitor;
use Psalm\Type\TypeVisitor;
use Psalm\Type\TypeNode;

/**
 * Denotes a simple array of the form `array<TKey, TValue>`. It expects an array with two elements, both union types.
 *
 * @psalm-immutable
 * @api
 */
class TArray extends Atomic
{
    /**
     * @use GenericTrait<array{Union, Union}>
     */
    use GenericTrait;

    #[Override]
    protected function getIntersectionId(bool $exact): string
    {
        // the id of an iterable/array never carries intersections (as before: only named objects did)
        return '';
    }

    /** @param array<lowercase-string, string> $aliased_classes */
    #[Override]
    protected function getNamespacedBase(
        ?string $namespace,
        array $aliased_classes,
        ?string $this_class,
        bool $use_phpdoc_format,
    ): string {
        return $this->value;
    }

    /** @param array<lowercase-string, string> $aliased_classes */
    #[Override]
    protected function getIntersectionNamespacedString(
        ?string $namespace,
        array $aliased_classes,
        ?string $this_class,
    ): string {
        // the namespaced string of an iterable/array never carries intersections (as before: only named objects did)
        return '';
    }

    /**
     * @var array{Union, Union}
     */
    public array $type_params;

    public string $value = 'array';

    /**
     * Constructs a new instance of a generic type
     *
     * @param array{Union, Union} $type_params
     */
    public function __construct(array $type_params, bool $from_docblock = false)
    {
        $this->type_params = $type_params;
        parent::__construct($from_docblock);
    }

    /**
     * @psalm-pure
     */
    #[Override]
    public function getKey(bool $include_extra = true): string
    {
        return 'array';
    }

    /**
     * @param  array<lowercase-string, string> $aliased_classes
     */
    #[Override]
    public function toPhpString(
        ?string $namespace,
        array $aliased_classes,
        ?string $this_class,
        int $analysis_php_version_id,
    ): string {
        return $this->getKey();
    }

    #[Override]
    public function canBeFullyExpressedInPhp(int $analysis_php_version_id): bool
    {
        return $this->type_params[0]->isArrayKey() && $this->type_params[1]->isMixed();
    }

    #[Override]
    public function equals(Atomic $other_type, bool $ensure_source_equality): bool
    {
        if ($other_type::class !== static::class) {
            return false;
        }

        if ($this instanceof TNonEmptyArray
            && $other_type instanceof TNonEmptyArray
            && $this->count !== $other_type->count
        ) {
            return false;
        }

        if (count($this->type_params) !== count($other_type->type_params)) {
            return false;
        }

        foreach ($this->type_params as $i => $type_param) {
            if (!$type_param->equals($other_type->type_params[$i], $ensure_source_equality, false)) {
                return false;
            }
        }

        return true;
    }

    #[Override]
    public function getAssertionString(): string
    {
        if ($this->type_params[0]->isMixed() && $this->type_params[1]->isMixed()) {
            return 'array';
        }

        return $this->getId();
    }

    public function isEmptyArray(): bool
    {
        return $this->type_params[1]->isNever();
    }

    /**
     * @return static
     */
    #[Override]
    public function replaceTemplateTypesWithStandins(
        TemplateResult $template_result,
        Codebase $codebase,
        ?StatementsAnalyzer $statements_analyzer = null,
        ?Atomic $input_type = null,
        ?int $input_arg_offset = null,
        ?string $calling_class = null,
        ?string $calling_function = null,
        bool $replace = true,
        bool $add_lower_bound = false,
        int $depth = 0,
    ): self {
        $type_params = $this->replaceTypeParamsTemplateTypesWithStandins(
            $template_result,
            $codebase,
            $statements_analyzer,
            $input_type,
            $input_arg_offset,
            $calling_class,
            $calling_function,
            $replace,
            $add_lower_bound,
            $depth,
        );
        if ($type_params) {
            $cloned = clone $this;
            $cloned->type_params = $type_params;
            return $cloned;
        }
        return $this;
    }

    /**
     * @return static
     */
    #[Override]
    public function replaceTemplateTypesWithArgTypes(TemplateResult $template_result, ?Codebase $codebase): self
    {
        $type_params = $this->replaceTypeParamsTemplateTypesWithArgTypes(
            $template_result,
            $codebase,
        );
        if ($type_params) {
            $cloned = clone $this;
            $cloned->type_params = $type_params;
            return $cloned;
        }
        return $this;
    }

    #[Override]
    public function visit(TypeVisitor $visitor): bool
    {
        foreach ($this->type_params as $child) {
            if ($visitor->traverse($child) === false) {
                return false;
            }
        }
        return true;
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
        $values = $self->type_params;
        $changed = false;
        $result = true;
        foreach ($values as &$child) {
            $child_orig = $child;
            $result = $visitor->traverse($child);
            $changed = $changed || $child !== $child_orig;
        }
        unset($child);
        if ($changed) {
            if (!$cloned) {
                $self = clone $self;
                $cloned = true;
            }
            $self->type_params = $values;
        }
        if ($result === false) {
            $node = $self;
            return false;
        }
        $node = $self;
        return true;
    }
}
