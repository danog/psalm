<?php

declare(strict_types=1);

namespace Psalm\Type\Atomic;

use Psalm\Internal\Sym;

use Psalm\Internal\Interner;

use Override;
use Psalm\Storage\UnserializeMemoryUsageSuppressionTrait;
use Psalm\Type;
use Psalm\Type\Atomic;

/**
 * Denotes a class constant whose value might not yet be known.
 *
 * @psalm-immutable
 * @api
 */
final class TClassConstant extends Atomic
{
    use UnserializeMemoryUsageSuppressionTrait;
    public function __construct(
        public int $fq_classlike_name,
        public int $const_name,
        bool $from_docblock = false,
    ) {
        parent::__construct($from_docblock);
    }

    #[Override]
    public function getKey(bool $include_extra = true): string
    {
        return 'class-constant(' . Interner::lookup($this->fq_classlike_name) . '::' . Interner::lookup($this->const_name) . ')';
    }

    #[Override]
    public function getId(bool $exact = true, bool $nested = false): string
    {
        return Interner::lookup($this->fq_classlike_name) . '::' . Interner::lookup($this->const_name);
    }

    #[Override]
    public function getAssertionString(): string
    {
        return 'class-constant(' . Interner::lookup($this->fq_classlike_name) . '::' . Interner::lookup($this->const_name) . ')';
    }

    /**
     * @param array<lowercase-string, string> $aliased_classes
     * @psalm-pure
     */
    #[Override]
    public function toPhpString(
        ?string $namespace,
        array $aliased_classes,
        ?int $this_class,
        int $analysis_php_version_id,
    ): ?string {
        return null;
    }

    /**
     * @psalm-pure
     */
    #[Override]
    public function canBeFullyExpressedInPhp(int $analysis_php_version_id): bool
    {
        return false;
    }

    /**
     * @param array<lowercase-string, string> $aliased_classes
     */
    #[Override]
    public function toNamespacedString(
        ?string $namespace,
        array $aliased_classes,
        ?int $this_class,
        bool $use_phpdoc_format,
    ): string {
        if ($this->fq_classlike_name === Sym::C_STATIC) {
            return 'static::' . Interner::lookup($this->const_name);
        }

        return Type::getStringFromFQCLN(Interner::lookup($this->fq_classlike_name), $namespace, $aliased_classes, $this_class)
            . '::'
            . Interner::lookup($this->const_name);
    }
}
