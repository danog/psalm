<?php

declare(strict_types=1);

namespace Psalm\Internal;

use InvalidArgumentException;
use Override;
use Psalm\Storage\ImmutableNonCloneableTrait;
use Psalm\Storage\UnserializeMemoryUsageSuppressionTrait;
use Stringable;

use function explode;
use function ltrim;
use function str_contains;
use function strtolower;

/**
 * @psalm-immutable
 * @internal
 */
final class MethodIdentifier implements Stringable
{
    use ImmutableNonCloneableTrait;
    use UnserializeMemoryUsageSuppressionTrait;

    /**
     * The interned class name (pzoom's `MethodIdentifier(StrId, StrId)`): storages are looked up by it.
     */
    public readonly int $class_id;

    /**
     * The interned (lowercase) method name: the class-like storages key their method maps by it.
     */
    public readonly int $name_id;

    /**
     * Memo of __toString (the identifier is stringified as a map key on most hot paths); public so the
     * cache serializer sees it like every other property.
     *
     * @internal
     * @var non-empty-string|null
     */
    public ?string $string_memo = null;

    /**
     * @param lowercase-string $method_name
     * @psalm-mutation-free
     */
    public function __construct(public readonly string $fq_class_name, public readonly string $method_name)
    {
        $this->class_id = Interner::intern($fq_class_name);
        $this->name_id = Interner::intern($method_name);
    }


    /**
     * @psalm-pure
     */
    public static function isValidMethodIdReference(string $method_id): bool
    {
        return str_contains($method_id, '::');
    }

    /**
     * @psalm-pure
     */
    public static function fromMethodIdReference(string $method_id): self
    {
        if (!static::isValidMethodIdReference($method_id)) {
            throw new InvalidArgumentException('Invalid method id reference provided: ' . $method_id);
        }
        // remove leading backslash if it exists
        $method_id = ltrim($method_id, '\\');
        $method_id_parts = explode('::', $method_id);
        return new self($method_id_parts[0], strtolower($method_id_parts[1]));
    }

    /** @return non-empty-string */
    #[Override]
    public function __toString(): string
    {
        if ($this->string_memo !== null) {
            return $this->string_memo;
        }
        $string = $this->fq_class_name . '::' . $this->method_name;
        /** @psalm-suppress ImpurePropertyAssignment, InaccessibleProperty Cache */
        $this->string_memo = $string;
        return $string;
    }
}
