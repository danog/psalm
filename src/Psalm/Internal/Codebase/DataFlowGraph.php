<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use Psalm\Internal\DataFlow\DataFlowNode;

use function array_key_last;
use function count;
use function str_starts_with;
use function strlen;
use function substr;

/**
 * @internal
 * @psalm-capabilities read-props|write-this-props|write-props|write-refs
 */
abstract class DataFlowGraph
{
    abstract public function addNode(DataFlowNode $node): void;

    abstract public function addPath(
        DataFlowNode $from,
        DataFlowNode $to,
        string $path_type,
        int $added_taints = 0,
        int $removed_taints = 0,
    ): void;

    /**
     * @param list<string> $previous_path_types
     * @psalm-pure
     */
    protected static function shouldIgnoreFetch(
        string $path_type,
        string $expression_type,
        array $previous_path_types,
    ): bool {
        $el = strlen($expression_type);

        // arraykey-fetch requires a matching arraykey-assignment at the same level
        // otherwise the tainting is not valid
        if (str_starts_with($path_type, $expression_type . '-fetch-')
            || ($path_type === 'arraykey-fetch' && $expression_type === 'arrayvalue')
        ) {
            $fetch_nesting = 0;

            for ($x = count($previous_path_types)-1; $x >= 0; $x--) {
                $previous_path_type = $previous_path_types[$x];
                if ($previous_path_type === $expression_type . '-assignment') {
                    if ($fetch_nesting === 0) {
                        // a value assigned under any key: the keys of the array don't hold it
                        return $path_type === 'arraykey-fetch';
                    }

                    $fetch_nesting--;
                }

                if (str_starts_with($previous_path_type, $expression_type . '-fetch')) {
                    $fetch_nesting++;
                }

                if (str_starts_with($previous_path_type, $expression_type . '-assignment-')) {
                    if ($fetch_nesting > 0) {
                        $fetch_nesting--;
                        continue;
                    }

                    if (substr($previous_path_type, $el + 12) === substr($path_type, $el + 7)) {
                        return false;
                    }

                    return true;
                }
            }
        }

        return false;
    }

    /**
     * Whether an edge of type $path_type, from an array to the array it becomes once its value under a key is
     * replaced (see ArrayAssignmentAnalyzer::getOverwritePathType()), drops a flow whose open assignments are
     * $open_assignments: a flow of that value, the innermost of them being the assignment to that key. A flow of
     * the array's other values, of its keys or of all of it goes on, also one whose key isn't known, which may be
     * that one.
     *
     * @param list<string> $open_assignments
     * @psalm-pure
     */
    protected static function isOverwritten(string $path_type, array $open_assignments): bool
    {
        if (!str_starts_with($path_type, 'arrayvalue-overwrite-')) {
            return false;
        }

        $innermost = array_key_last($open_assignments);

        return $innermost !== null
            && $open_assignments[$innermost] === 'arrayvalue-assignment-' . substr($path_type, 21);
    }
}
