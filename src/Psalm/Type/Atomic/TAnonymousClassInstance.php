<?php

declare(strict_types=1);

namespace Psalm\Type\Atomic;

use Override;

/**
 * Denotes an anonymous class (i.e. `new class{}`) with potential methods
 *
 * @psalm-immutable
 */
final class TAnonymousClassInstance extends TNamedObject
{
    /**
     * @param string $value the name of the object
     * @param array<string, TNamedObject|TTemplateParam|TIterable|TObjectWithProperties> $extra_types
     */
    public function __construct(
        string $value,
        bool $is_static = false,
        public ?string $extends = null,
        array $extra_types = [],
    ) {
        parent::__construct($value, $is_static, false, $extra_types);
    }

    #[Override]
    public function toPhpString(
        ?string $namespace,
        array $aliased_classes,
        ?string $this_class,
        int $analysis_php_version_id,
    ): ?string {
        return $analysis_php_version_id >= 7_02_00 ? ($this->extends ?? 'object') : null;
    }

    /**
     * @param  array<lowercase-string, string> $aliased_classes
     */
    #[Override]
    public function toNamespacedString(
        ?string $namespace,
        array $aliased_classes,
        ?string $this_class,
        bool $use_phpdoc_format,
    ): string {
        return $this->extends ?? 'object';
    }
}
