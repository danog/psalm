<?php

declare(strict_types=1);

namespace Psalm\Internal\Analyzer;

use function str_pad;

use const STR_PAD_LEFT;

/**
 * @internal
 */
final class IssueData
{
    public const SEVERITY_INFO = 'info';
    public const SEVERITY_ERROR = 'error';

    public readonly string $link;

    /**
     * @param self::SEVERITY_* $severity
     * @param ?list<DataFlowNodeData|array{label: string, entry_path_type: string}> $taint_trace
     * @param ?list<DataFlowNodeData> $other_references
     * @psalm-mutation-free
     */
    public function __construct(
        public string $severity,
        public int $line_from,
        public int $line_to,
        public readonly string $type,
        public readonly string $message,
        public readonly string $file_name,
        public readonly string $file_path,
        public readonly string $snippet,
        public readonly string $selected_text,
        public int $from,
        public int $to,
        public int $snippet_from,
        public int $snippet_to,
        public readonly int $column_from,
        public readonly int $column_to,
        public readonly int $shortcode = 0,
        public int $error_level = -1,
        public ?array $taint_trace = null,
        public ?array $other_references = null,
        public readonly ?string $dupe_key = null,
        ?string $documentation_url = null,
    ) {
        $this->link = match (true) {
            $documentation_url !== null => $documentation_url,
            $shortcode > 0 => 'https://psalm.dev/' . str_pad((string) $shortcode, 3, "0", STR_PAD_LEFT),
            default => '',
        };
    }
    /**
     * All properties by name (the report formats' view of an issue).
     *
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return [
            'severity' => $this->severity,
            'line_from' => $this->line_from,
            'line_to' => $this->line_to,
            'type' => $this->type,
            'message' => $this->message,
            'file_name' => $this->file_name,
            'file_path' => $this->file_path,
            'snippet' => $this->snippet,
            'selected_text' => $this->selected_text,
            'from' => $this->from,
            'to' => $this->to,
            'snippet_from' => $this->snippet_from,
            'snippet_to' => $this->snippet_to,
            'column_from' => $this->column_from,
            'column_to' => $this->column_to,
            'shortcode' => $this->shortcode,
            'error_level' => $this->error_level,
            'taint_trace' => $this->taint_trace,
            'other_references' => $this->other_references,
            'dupe_key' => $this->dupe_key,
            'link' => $this->link,
        ];
    }
}
