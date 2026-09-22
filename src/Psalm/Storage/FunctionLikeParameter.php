<?php

declare(strict_types=1);

namespace Psalm\Storage;

use Override;
use Psalm\CodeLocation;
use Psalm\Internal\Scanner\UnresolvedConstantComponent;
use Psalm\Type\MutableTypeVisitor;
use Psalm\Type\TypeNode;
use Psalm\Type\TypeVisitor;
use Psalm\Type\Union;

/**
 * @api
 */
final class FunctionLikeParameter implements HasAttributesInterface, TypeNode
{
    use CustomMetadataTrait;
    use UnserializeMemoryUsageSuppressionTrait;

    public bool $has_docblock_type = false;

    public ?CodeLocation $signature_type_location = null;

    public int $sinks = 0;

    public bool $assert_untainted = false;

    public bool $type_inferred = false;

    public bool $expect_variable = false;

    public bool $promoted_property = false;

    /**
     * @var list<AttributeStorage>
     */
    public array $attributes = [];

    public ?string $description = null;

    /**
     * @param string $name parameter name, without the "$" prefix
     * @psalm-mutation-free
     */
    public function __construct(
        public string $name,
        public bool $by_ref,
        public ?Union $type = null,
        public ?Union $signature_type = null,
        public ?CodeLocation $location = null,
        public ?CodeLocation $type_location = null,
        public bool $is_optional = true,
        public bool $is_nullable = false,
        public bool $is_variadic = false,
        public Union|UnresolvedConstantComponent|null $default_type = null,
        public ?Union $out_type = null,
    ) {
        $this->signature_type_location = $type_location;
    }

    /** @psalm-mutation-free */
    public function getId(): string
    {
        return ($this->type ? $this->type->getId() : 'mixed')
            . ($this->is_variadic ? '...' : '')
            . ($this->is_optional ? '=' : '');
    }

    /** @psalm-mutation-free */
    public function setType(Union $type): self
    {
        if ($this->type === $type) {
            return $this;
        }
        $cloned = clone $this;
        $cloned->type = $type;
        return $cloned;
    }

    /**
     * @internal Should only be used by the MutableTypeVisitor.
     * @psalm-external-mutation-free
     */
    #[Override]
    public function visit(TypeVisitor $visitor): bool
    {
        if ($this->type && !$visitor->traverse($this->type)) {
            return false;
        }
        if ($this->signature_type && !$visitor->traverse($this->signature_type)) {
            return false;
        }
        if ($this->out_type && !$visitor->traverse($this->out_type)) {
            return false;
        }
        if ($this->default_type instanceof Union && !$visitor->traverse($this->default_type)) {
            return false;
        }

        return true;
    }

    /**
     * @phpcsSuppress SlevomatCodingStandard.TypeHints.ParameterTypeHint.MissingAnyTypeHint
     */
    #[Override]
    public static function visitMutable(MutableTypeVisitor $visitor, &$node, bool $cloned): bool
    {
        if ($node->type instanceof TypeNode) {
            $value = $node->type;
            $result = $visitor->traverse($value);
            if ($value !== $node->type) {
                if (!$cloned) {
                    $node = clone $node;
                    $cloned = true;
                }
                $node->type = $value;
            }
            if (!$result) {
                return false;
            }
        }
        if ($node->signature_type instanceof TypeNode) {
            $value = $node->signature_type;
            $result = $visitor->traverse($value);
            if ($value !== $node->signature_type) {
                if (!$cloned) {
                    $node = clone $node;
                    $cloned = true;
                }
                $node->signature_type = $value;
            }
            if (!$result) {
                return false;
            }
        }
        if ($node->out_type instanceof TypeNode) {
            $value = $node->out_type;
            $result = $visitor->traverse($value);
            if ($value !== $node->out_type) {
                if (!$cloned) {
                    $node = clone $node;
                    $cloned = true;
                }
                $node->out_type = $value;
            }
            if (!$result) {
                return false;
            }
        }
        if ($node->default_type instanceof TypeNode) {
            $value = $node->default_type;
            $result = $visitor->traverse($value);
            if ($value !== $node->default_type) {
                if (!$cloned) {
                    $node = clone $node;
                    $cloned = true;
                }
                $node->default_type = $value;
            }
            if (!$result) {
                return false;
            }
        }

        return true;
    }

    /**
     * @psalm-mutation-free
     * @return list<AttributeStorage>
     */
    #[Override]
    public function getAttributeStorages(): array
    {
        return $this->attributes;
    }
}
