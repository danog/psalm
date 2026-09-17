<?php

declare(strict_types=1);

namespace Psalm\Report;

use Override;
use Psalm\Internal\Analyzer\DataFlowNodeData;
use Psalm\Internal\Analyzer\IssueData;
use Psalm\Report;
use Spatie\ArrayToXml\ArrayToXml;

use function array_map;
use function get_object_vars;

/**
 * @api
 * @psalm-import-type DataFlowNodeDataArray from DataFlowNodeData
 */
final class XmlReport extends Report
{
    #[Override]
    public function create(): string
    {
        $xml = ArrayToXml::convert(
            [
                'item' => array_map(
                    /** @return array<string, scalar|null|list<array<string, scalar|null>>> */
                    static function (IssueData $issue_data): array {
                        $data = $issue_data->toArray();
                        unset($data['dupe_key']);

                        if (null !== $data['taint_trace']) {
                            $data['taint_trace'] = array_map(
                                /**
                                 * @param DataFlowNodeData|array{label: string, entry_path_type: string} $trace
                                 * @return DataFlowNodeDataArray|array{label: string, entry_path_type: string}
                                 */
                        static fn(DataFlowNodeData|array $trace): array => $trace instanceof DataFlowNodeData ? $trace->toArray() : $trace,
                                $data['taint_trace'],
                            );
                        }

                        // replace null values, as XML serializers tend to have problems with them
                        $data['taint_trace'] ??= '';

                        if (null !== $data['other_references']) {
                            $data['other_references'] = array_map(
                                /** @return DataFlowNodeDataArray */
                                static fn(DataFlowNodeData $reference): array => $reference->toArray(),
                                $data['other_references'],
                            );
                        }

                        // replace null values, as XML serializers tend to have problems with them
                        $data['other_references'] ??= '';

                        return $data;
                    },
                    $this->issues_data,
                ),
            ],
            'report',
            true,
            'UTF-8',
            '1.0',
            ['preserveWhiteSpace' => false, 'formatOutput' => true],
        );

        return $xml;
    }
}
