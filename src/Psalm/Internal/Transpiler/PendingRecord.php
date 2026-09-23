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
            if ($type->possibly_undefined && $type->hasMixed()) {
                // a possibly-unset variable's placeholder type says nothing about its values
                continue;
            }
            // a statement snapshot mostly sees the same Union object as the previous one: keep each type once
            $oid = spl_object_id($type);
            if (isset($this->var_type_seen[$var_id][$oid])) {
                continue;
            }
            $this->var_type_seen[$var_id][$oid] = true;
            $this->var_types[$var_id][] = $type;
        }
    }

    /** @var array<string, array<int, true>> var id => spl_object_ids of the Unions already in var_types */
    private array $var_type_seen = [];
}
