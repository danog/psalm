<?php

declare(strict_types=1);

namespace Psalm\Internal\TypeVisitor;

use Override;
use Psalm\Type\Atomic\TClassConstant;
use Psalm\Type\Atomic\TConditional;
use Psalm\Type\Atomic\TIntMask;
use Psalm\Type\Atomic\TIntMaskOf;
use Psalm\Type\Atomic\TKeyOf;
use Psalm\Type\Atomic\TNamedObject;
use Psalm\Type\Atomic\TPropertiesOf;
use Psalm\Type\Atomic\TTypeAlias;
use Psalm\Type\Atomic\TValueOf;
use Psalm\Type\TypeNode;
use Psalm\Type\TypeVisitor;

use function strtolower;

/**
 * What a type's expansion (TypeExpander::expandUnion) depends on besides the type itself, for memoizing it:
 * which of the expansion's class arguments (self, static/final, parent) can affect the result, and whether
 * the type holds something whose expansion reads storages that may still change during analysis (class
 * constants, type aliases, properties-of, key-of/value-of, int masks, conditional types), which is never
 * memoized. (pzoom resolves those in the storages once and only expands static and conditionals at a use.)
 *
 * @internal
 */
final class ExpansionTraitsCollector extends TypeVisitor
{
    /** `self` is named: the expansion depends on the self class */
    public const SELF = 1;
    /** `static` / `$this` (or a static object) is named: the expansion depends on the static class and finality */
    public const STATIC = 2;
    /** `parent` is named: the expansion depends on the parent class */
    public const PARENT = 4;
    /** the expansion reads storages that may still change: not memoized */
    public const UNMEMOIZABLE = 8;

    private int $traits = 0;

    /**
     * @psalm-external-mutation-free
     */
    #[Override]
    protected function enterNode(TypeNode $type): ?int
    {
        if ($type instanceof TClassConstant
            || $type instanceof TTypeAlias
            || $type instanceof TPropertiesOf
            || $type instanceof TKeyOf
            || $type instanceof TValueOf
            || $type instanceof TIntMask
            || $type instanceof TIntMaskOf
            || $type instanceof TConditional
        ) {
            $this->traits |= self::UNMEMOIZABLE;
            return self::STOP_TRAVERSAL;
        }

        if ($type instanceof TNamedObject) {
            $name_lc = strtolower($type->value);
            if ($type->is_static || $name_lc === 'static' || $name_lc === '$this') {
                $this->traits |= self::STATIC;
            } elseif ($name_lc === 'self') {
                $this->traits |= self::SELF;
            } elseif ($name_lc === 'parent') {
                $this->traits |= self::PARENT;
            }
        }

        return null;
    }

    /** @psalm-mutation-free */
    public function getTraits(): int
    {
        return $this->traits;
    }
}
