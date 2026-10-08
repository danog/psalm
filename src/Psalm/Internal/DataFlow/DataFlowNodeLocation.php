<?php

declare(strict_types=1);

namespace Psalm\Internal\DataFlow;

use Psalm\CodeLocation;

/**
 * The location of a data-flow node built from its compact position (see DataFlowNode::getCodeLocation()): the
 * same location as `new CodeLocation($source, $node)` for the parsed node the position was taken from.
 *
 * @internal
 */
final class DataFlowNodeLocation extends CodeLocation
{
    /**
     * @psalm-mutation-free
     */
    public function __construct(
        string $file_path,
        string $file_name,
        int $file_start,
        int $file_end,
        int $line,
        ?int $docblock_start,
        ?int $docblock_start_line_number,
    ) {
        $this->file_start = $file_start;
        $this->file_end = $file_end;
        $this->raw_file_start = $file_start;
        $this->raw_file_end = $file_end;
        $this->file_path = $file_path;
        $this->file_name = $file_name;
        $this->single_line = false;
        $this->docblock_start = $docblock_start;
        $this->docblock_start_line_number = $docblock_start_line_number;
        $this->raw_line_number = $line;
    }
}
