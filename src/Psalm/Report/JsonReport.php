<?php

declare(strict_types=1);

namespace Psalm\Report;

use Override;
use Psalm\Internal\Analyzer\DataFlowNodeData;
use Psalm\Internal\Analyzer\IssueData;
use Psalm\Internal\Json\Json;
use Psalm\Report;

use function array_map;
use function array_values;

/**
 * @psalm-external-mutation-free
 * @api
 */
final class JsonReport extends Report
{
    /**
     * @psalm-mutation-free
     */
    #[Override]
    public function create(): string
    {
        $options = $this->pretty ? Json::PRETTY : Json::DEFAULT;

        $issues_data = array_map(
            /** @return array<string, scalar|null|list<array<string, scalar|null>>> */
            static function (IssueData $issue_data): array {
                $data = $issue_data->toArray();
                unset($data['dupe_key']);

                if (null !== $data['taint_trace']) {
                    $data['taint_trace'] = array_map(
                        /** @param DataFlowNodeData|array{label: string, entry_path_type: string} $trace */
                        static fn(DataFlowNodeData|array $trace): array => $trace instanceof DataFlowNodeData ? $trace->toArray() : $trace,
                        $data['taint_trace'],
                    );
                }

                if (null !== $data['other_references']) {
                    $data['other_references'] = array_map(
                        static fn(DataFlowNodeData $reference): array => $reference->toArray(),
                        $data['other_references'],
                    );
                }

                return $data;
            },
            $this->issues_data,
        );

        return Json::encode(array_values($issues_data), $options) . "\n";
    }
}
