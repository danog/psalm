<?php

declare(strict_types=1);

namespace Psalm\Internal;

use InvalidArgumentException;
use Override;
use Psalm\Storage\ImmutableNonCloneableTrait;
use Stringable;
use Psalm\Storage\UnserializeMemoryUsageSuppressionTrait;

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
     * Memo of __toString (the identifier is stringified as a map key on most hot paths); public so the
     * cache serializer sees it like every other property.
     *
     * @internal
     * @var non-empty-string|null
     */
    public ?string $string_memo = null;

    /**
     * Memo of the interned id of the lowercase class name when that names an existing class-like (pzoom's
     * `MethodIdentifier(StrId, StrId)`): storage lookups through the identifier are then int lookups. 0 until
     * known. Ids are per process, so it is not serialized (see __serialize()).
     *
     * @internal
     */
    public int $class_id = 0;

    /**
     * @param lowercase-string $method_name
     * @psalm-mutation-free
     */
    public function __construct(public readonly string $fq_class_name, public readonly string $method_name)
    {
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

    /**
     * @return array{fq_class_name: string, method_name: lowercase-string, string_memo: non-empty-string|null}
     * @psalm-mutation-free
     */
    public function __serialize(): array
    {
        return [
            'fq_class_name' => $this->fq_class_name,
            'method_name' => $this->method_name,
            'string_memo' => $this->string_memo,
        ];
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
