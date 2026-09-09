<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Stmt\ClassMethod;
use Psalm\Storage\MethodStorage;

/**
 * @internal
 */
final class MethodModel
{
    /** @var list<RustType> */
    public array $param_types = [];

    public RustType $return_type;

    /** static method whose body refers to `static` (needs a copy per calling class) */
    public bool $uses_lsb = false;

    public function __construct(
        public readonly string $name,
        public readonly ClassModel $declaring,
        public readonly MethodStorage $storage,
        public readonly ?ClassMethod $node,
        public readonly ?FunctionRecord $record,
    ) {
        $this->return_type = RustType::unit();
    }

    public function lc(): string
    {
        return strtolower($this->name);
    }

    public function rustName(): string
    {
        return Names::method($this->name);
    }

    public function isPrivate(): bool
    {
        return $this->storage->visibility === \Psalm\Internal\Analyzer\ClassLikeAnalyzer::VISIBILITY_PRIVATE;
    }

    public function isStatic(): bool
    {
        return $this->storage->is_static;
    }

    public function isAbstract(): bool
    {
        return $this->storage->abstract || $this->node === null || $this->node->stmts === null;
    }
}
