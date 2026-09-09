<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Expr\ArrowFunction;
use PhpParser\Node\Expr\Closure;
use PhpParser\Node\Stmt;
use PhpParser\Node\Stmt\ClassMethod;
use PhpParser\Node\Stmt\Function_;
use Psalm\Internal\Provider\NodeDataProvider;
use Psalm\Storage\FunctionLikeStorage;
use Psalm\Type\Union;
use SplObjectStorage;

/**
 * Everything the emitter needs to know about one analyzed function-like.
 *
 * @internal
 */
final class FunctionRecord
{
    /** @var SplObjectStorage<Stmt, array<string, Union>> variable types after each statement */
    public SplObjectStorage $stmt_vars;

    /** @var array<string, Union> variable types at function entry (params, closure uses) */
    public array $entry_vars = [];

    /** @var array<string, Union> variable types at function exit */
    public array $exit_vars = [];

    /** @var array<string, list<Union>> all observed types per variable */
    public array $var_types = [];

    public function __construct(
        public Closure|Function_|ClassMethod|ArrowFunction $node,
        public FunctionLikeStorage $storage,
        public NodeDataProvider $node_data,
        public string $file_path,
        public ?string $fq_class_name,
        public ?string $method_name,
    ) {
        $this->stmt_vars = new SplObjectStorage();
    }

    public function isClosure(): bool
    {
        return $this->node instanceof Closure || $this->node instanceof ArrowFunction;
    }

    /**
     * @param array<string, Union> $vars
     */
    public function addVarTypes(array $vars): void
    {
        foreach ($vars as $var_id => $type) {
            if ($var_id[0] !== '$' || str_contains($var_id, '->') || str_contains($var_id, '[') || str_contains($var_id, '::')) {
                continue;
            }
            $this->var_types[$var_id][] = $type;
        }
    }
}
