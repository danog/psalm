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
                $issue_data = $issue_data->toArray();
                unset($issue_data['dupe_key']);

                if (null !== $issue_data['taint_trace']) {
                    $issue_data['taint_trace'] = array_map(
                        static fn(DataFlowNodeData|array $trace): array => $trace instanceof DataFlowNodeData ? $trace->toArray() : $trace,
                        $issue_data['taint_trace'],
                    );
                }

                if (null !== $issue_data['other_references']) {
                    $issue_data['other_references'] = array_map(
                        static fn(DataFlowNodeData $reference): array => $reference->toArray(),
                        $issue_data['other_references'],
                    );
                }

                return $issue_data;
            },
            $this->issues_data,
        );

        return Json::encode(array_values($issues_data), $options) . "\n";
    }
}
