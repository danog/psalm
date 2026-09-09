<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Stmt;
use Psalm\Type\Union;
use SplObjectStorage;

/**
 * Statement snapshots collected before the owning function-like has finished analysis.
 *
 * @internal
 */
final class PendingRecord
{
    /** @var SplObjectStorage<Stmt, array<string, Union>> */
    public SplObjectStorage $stmt_vars;

    /** @var array<string, list<Union>> */
    public array $var_types = [];

    public function __construct()
    {
        $this->stmt_vars = new SplObjectStorage();
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
