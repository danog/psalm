<?php

declare(strict_types=1);

namespace Psalm\Internal\Analyzer;

use Psalm\Storage\ImmutableNonCloneableTrait;

/**
 * @psalm-immutable
 * @internal
 * @psalm-type DataFlowNodeDataArray = array{label: string, line_from: int, line_to: int, file_name: string, file_path: string, snippet: string, from: int, to: int, snippet_from: int, column_from: int, column_to: int}
 */
final class DataFlowNodeData
{
    use ImmutableNonCloneableTrait;

    /**
     * @psalm-mutation-free
     */
    public function __construct(
        public readonly string $label,
        public readonly int $line_from,
        public readonly int $line_to,
        public readonly string $file_name,
        public readonly string $file_path,
        public readonly string $snippet,
        public readonly int $from,
        public readonly int $to,
        public readonly int $snippet_from,
        public readonly int $column_from,
        public readonly int $column_to,
    ) {
    }

    /**
     * The node as the report serializers emit it (every public property, in declaration order).
     *
     * @return DataFlowNodeDataArray
     */
    public function toArray(): array
    {
        return [
            'label' => $this->label,
            'line_from' => $this->line_from,
            'line_to' => $this->line_to,
            'file_name' => $this->file_name,
            'file_path' => $this->file_path,
            'snippet' => $this->snippet,
            'from' => $this->from,
            'to' => $this->to,
            'snippet_from' => $this->snippet_from,
            'column_from' => $this->column_from,
            'column_to' => $this->column_to,
        ];
    }
}
