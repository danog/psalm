<?php

declare(strict_types=1);

namespace Psalm\Internal\TypeVisitor;

use Psalm\Internal\Interner;

use Override;
use Psalm\Type\Atomic\TClassConstant;
use Psalm\Type\Atomic\TClassString;
use Psalm\Type\Atomic\TLiteralClassString;
use Psalm\Type\Atomic\TNamedObject;
use Psalm\Type\MutableTypeVisitor;
use Psalm\Type\TypeNode;

use function strtolower;

/**
 * @internal
 */
final class ClasslikeReplacer extends MutableTypeVisitor
{
    private readonly string $old;

    /**
     * @psalm-mutation-free
     */
    public function __construct(
        string $old,
        private readonly string $new,
    ) {
        $this->old = strtolower($old);
    }

    #[Override]
    protected function enterNode(TypeNode &$type): ?int
    {
        if ($type instanceof TClassConstant) {
            if ($type->fq_classlike_name === Interner::lookup($this->old)) {
                $type = new TClassConstant(
                    Interner::intern($this->new),
                    $type->const_name,
                    $type->from_docblock,
                );
            }
        } elseif ($type instanceof TClassString) {
            if ($type->as !== 'object' && strtolower($type->as) === $this->old) {
                $type = new TClassString(
                    $this->new,
                    $type->as_type,
                    $type->is_loaded,
                    $type->is_interface,
                    $type->is_enum,
                    $type->from_docblock,
                );
            }
        } elseif ($type instanceof TNamedObject || $type instanceof TLiteralClassString) {
            if ($type->value === $this->old) {
                $type = $type->setValue(Interner::intern($this->new));
            }
        }
        return null;
    }
}
