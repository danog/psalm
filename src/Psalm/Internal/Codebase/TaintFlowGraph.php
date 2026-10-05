<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use Override;
use Psalm\CodeLocation;
use Psalm\Codebase;
use Psalm\Config;
use Psalm\Internal\Analyzer\ClassLikeAnalyzer;
use Psalm\Internal\Analyzer\ProjectAnalyzer;
use Psalm\Internal\DataFlow\DataFlowNode;
use Psalm\Internal\DataFlow\Path;
use Psalm\Issue\TaintedCallable;
use Psalm\Issue\TaintedCookie;
use Psalm\Issue\TaintedCustom;
use Psalm\Issue\TaintedEval;
use Psalm\Issue\TaintedExtract;
use Psalm\Issue\TaintedFile;
use Psalm\Issue\TaintedHeader;
use Psalm\Issue\TaintedHtml;
use Psalm\Issue\TaintedInclude;
use Psalm\Issue\TaintedLdap;
use Psalm\Issue\TaintedLlmPrompt;
use Psalm\Issue\TaintedNosql;
use Psalm\Issue\TaintedSSRF;
use Psalm\Issue\TaintedShell;
use Psalm\Issue\TaintedSleep;
use Psalm\Issue\TaintedSql;
use Psalm\Issue\TaintedSystemSecret;
use Psalm\Issue\TaintedTextWithQuotes;
use Psalm\Issue\TaintedUnserialize;
use Psalm\Issue\TaintedUserSecret;
use Psalm\Issue\TaintedXpath;
use Psalm\IssueBuffer;
use Psalm\Progress\Phase;
use Psalm\Progress\Progress;
use Psalm\Storage\Capabilities;
use Psalm\Storage\FunctionLikeStorage;
use Psalm\Storage\MethodStorage;
use Psalm\Type\Atomic\TArray;
use Psalm\Type\Atomic\TKeyedArray;
use Psalm\Type\TaintKind;
use Psalm\Type\Union;

use function array_unshift;
use function end;
use function in_array;
use function ksort;
use function str_starts_with;
use function strpos;
use function substr;
use function min;
use function strcmp;

/**
 * @internal
 */
final class TaintFlowGraph extends DataFlowGraph
{
    /**
     * The separator DataFlowNode uses to build a specialized node id from its
     * unspecialized base id and specialization key (see DataFlowNode::make()).
     */
    public const SPECIALIZATION_SEPARATOR = ' specialized in ';

    /** @var array<string, DataFlowNode> */
    private array $sources = [];

    /** @var array<string, DataFlowNode> */
    private array $nodes = [];

    /** @var array<string, DataFlowNode> */
    private array $sinks = [];

    /**
     * Unspecialized ID => (Specialization key => Specialized ID)
     *
     * @var array<string, array<string, string>>
     */
    private array $specializations = [];

    /**
     * Specialization key => true
     *
     * @var array<string, true>
     */
    private array $specialized_calls = [];

    /**
     * Call sites specialized speculatively, before knowing whether the callee is pure:
     * specialization key => (function-like node of the callee => true)
     *
     * @var array<string, array<string, true>>
     */
    private array $speculative_calls = [];

    /**
     * Speculatively specialized call sites of a callee that turned out not to be pure,
     * which are resolved as if they were not specialized: specialization key => true
     *
     * @var array<string, true>
     */
    private array $despecialized_calls = [];

    /**
     * Whether the taint nodes of a call to $storage at $call_location are specialized to the call site,
     * so that taint flowing into one call does not flow out of the other calls.
     *
     * This is sound only for callees that are pure: a function-like with a side effect could store the
     * taint somewhere it is read back by another call. Function-likes marked pure (or with
     * `@psalm-taint-specialize`) are always specialized. The purity of an unannotated project
     * function-like is only known once the whole codebase has been analysed (see
     * {@see MutationLevelResolver}), so its calls are specialized speculatively when it cannot be
     * overridden, and resolved as unspecialized calls if it turns out not to be pure
     * (see {@see self::despecializeImpureCalls()}).
     */
    public static function isCallSpecialized(
        ?self $graph,
        Codebase $codebase,
        FunctionLikeStorage $storage,
        CodeLocation $call_location,
    ): bool {
        if ($storage->specialize_call) {
            return true;
        }

        // a builtin has no body: the taints flow through each of its calls as its declaration says, whether it is
        // pure or not (`reset()` moves the pointer of the array it returns an element of)
        if ($storage->location === null
            ? $storage->cased_name !== null && InternalCallMapHandler::inCallMap(
                $storage instanceof MethodStorage && $storage->defining_fqcln !== null
                    ? $storage->defining_fqcln . '::' . $storage->cased_name
                    : $storage->cased_name,
            )
            : in_array($storage->location->file_path, $codebase->config->internal_stubs, true)
        ) {
            return true;
        }

        if ($graph === null
            || $storage->has_mutations_annotation
            || $storage->location === null
            || !$codebase->config->isInProjectDirs($storage->location->file_path)
        ) {
            return false;
        }

        if ($storage instanceof MethodStorage) {
            // `@method` pseudo-methods have no defining class, nor a body whose purity could be inferred
            if ($storage->cased_name === '__construct'
                || $storage->defining_fqcln === null
                || $storage->defining_fqcln === ''
            ) {
                return false;
            }

            // an override could have side effects even if this implementation doesn't
            if ($storage->visibility !== ClassLikeAnalyzer::VISIBILITY_PRIVATE
                && !$storage->final
                && !$codebase->classlike_storage_provider->get($storage->defining_fqcln)->final
            ) {
                return false;
            }
        }

        $function_node_id = CodeUseGraph::functionLikeNodeForStorage($storage);

        if ($function_node_id === null) {
            return false;
        }

        $graph->speculative_calls[DataFlowNode::getSpecializationKey($call_location)][$function_node_id] = true;

        return true;
    }

    /**
     * The node as it would be without a speculative specialization: speculative specializations
     * only concern taints, so the variable use graph (see {@see VariableUseGraph}) keeps the nodes
     * it had before.
     *
     * @psalm-mutation-free
     */
    public function withoutSpeculativeSpecialization(DataFlowNode $node): DataFlowNode
    {
        if ($node->unspecialized_id === null
            || $node->specialization_key === null
            || !isset($this->speculative_calls[$node->specialization_key])
        ) {
            return $node;
        }

        return $node->withSpecialization($node->unspecialized_id, null, null, $node->context);
    }

    /**
     * Adds paths into $node from the parent nodes found in the array keys and values of $type,
     * at any depth, as the array assignments that put them there. Returns whether it added any.
     *
     * A value without parent nodes of its own carries its taint in those of its array keys and
     * values: an array fetch from it takes the parent nodes of the fetched value. Once $node is
     * made a parent node of such a value, the fetch goes through $node instead, which these
     * paths keep leading to the same taint.
     *
     * @psalm-capabilities read-props|write-this-props|write-props|write-refs
     */
    public function addPathsFromNestedParentNodes(DataFlowNode $node, Union $type, CodeLocation $location): bool
    {
        $added = false;

        foreach ($type->getAtomicTypes() as $atomic_type) {
            if ($atomic_type instanceof TKeyedArray) {
                foreach ($atomic_type->properties as $key => $property_type) {
                    $added = $this->addPathsFromParentNodes(
                        $node,
                        $property_type,
                        'arrayvalue-assignment-\'' . $key . '\'',
                        $location,
                    ) || $added;
                }

                $type_params = $atomic_type->fallback_params;
            } elseif ($atomic_type instanceof TArray) {
                $type_params = $atomic_type->type_params;
            } else {
                continue;
            }

            if ($type_params !== null) {
                $added = $this->addPathsFromParentNodes($node, $type_params[0], 'arraykey-assignment', $location)
                    || $added;
                $added = $this->addPathsFromParentNodes($node, $type_params[1], 'arrayvalue-assignment', $location)
                    || $added;
            }
        }

        return $added;
    }

    /**
     * Adds paths of type $path_type into $node from the parent nodes of $type, or else from those
     * nested in it (see addPathsFromNestedParentNodes()). Returns whether it added any.
     *
     * @psalm-capabilities read-props|write-this-props|write-props|write-refs
     */
    private function addPathsFromParentNodes(
        DataFlowNode $node,
        Union $type,
        string $path_type,
        CodeLocation $location,
    ): bool {
        if ($type->parent_nodes) {
            foreach ($type->parent_nodes as $parent_node) {
                $this->addPath($parent_node, $node, $path_type);
            }

            return true;
        }

        $nested_node = DataFlowNode::getForAssignment($node->label . ' ' . $path_type, $location);

        if (!$this->addPathsFromNestedParentNodes($nested_node, $type, $location)) {
            return false;
        }

        $this->addNode($nested_node);
        $this->addPath($nested_node, $node, $path_type);

        return true;
    }

    /**
     * Resolves the speculatively specialized call sites of callees that turned out not to be pure
     * (or whose purity is unknown) as unspecialized calls.
     *
     * @psalm-capabilities read-props|write-this-props|write-props|write-refs
     */
    private function despecializeImpureCalls(Codebase $codebase): void
    {
        if (!$this->speculative_calls) {
            return;
        }

        $mutation_levels = $codebase->code_use_graph->getMutationLevels();

        foreach ($this->speculative_calls as $specialization_key => $callees) {
            foreach ($callees as $function_node_id => $_) {
                if (($mutation_levels[$function_node_id] ?? Capabilities::ALL) !== Capabilities::NONE) {
                    $this->despecialized_calls[$specialization_key] = true;
                    break;
                }
            }
        }
    }

    /**
     * @psalm-external-mutation-free
     */
    #[Override]
    public function addNode(DataFlowNode $node): void
    {
        $this->nodes[$node->id] = $node;

        if ($node->unspecialized_id !== null) {
            /** @var string $node->specialization_key */
            $this->specialized_calls[$node->specialization_key] = true;
            $this->specializations[$node->unspecialized_id][$node->specialization_key] = $node->id;
        }
    }

    /**
     * Leaves out the paths no taint goes through, and those to the uses only the variable use
     * graph tracks: the analysis adds every path to the data flow graph, whichever graphs it
     * builds, so that types get the same parent nodes either way.
     *
     * @psalm-capabilities read-props|write-this-props|write-props|write-refs
     */
    #[Override]
    public function addPath(
        DataFlowNode $from,
        DataFlowNode $to,
        string $path_type,
        int $added_taints = 0,
        int $removed_taints = 0,
    ): void {
        if ($removed_taints === TaintKind::ALL
            || $to->id === DataFlowNode::getForVariableUse()->id
            || $to->id === DataFlowNode::getForClosureUse()->id
        ) {
            return;
        }

        parent::addPath($from, $to, $path_type, $added_taints, $removed_taints);
    }

    /**
     * Adds a path between two nodes from the graph of another worker (see addGraph()), which may have added
     * one already, from another file: then a flow can take either. Whichever came first, so that the
     * resolution doesn't depend on the order the workers' graphs are merged in. (The analysis of a file adds
     * a path again to replace it, e.g. with the taints a call escapes: see addPath().)
     *
     * Two paths of the same type are merged into one: a flow keeps the taints either keeps, and gets those
     * either adds. Two paths of different types aren't: they handle open assignments differently (see
     * shouldIgnoreFetch()). The one of the first type stays, and the other goes through a node of its own.
     *
     * @psalm-external-mutation-free
     */
    private function mergePath(string $from_id, ?DataFlowNode $from, string $to_id, Path $path): void
    {
        $existing = $this->forward_edges[$from_id][$to_id] ?? null;

        if ($existing === null) {
            $this->forward_edges[$from_id][$to_id] = $path;

            return;
        }

        if ($existing->type === $path->type) {
            $this->forward_edges[$from_id][$to_id] = new Path(
                $path->type,
                min($existing->length, $path->length),
                ($existing->added_taints & ~$existing->removed_taints) | ($path->added_taints & ~$path->removed_taints),
                $existing->removed_taints & $path->removed_taints,
            );

            return;
        }

        [$kept, $moved] = strcmp($existing->type, $path->type) < 0 ? [$existing, $path] : [$path, $existing];

        $this->forward_edges[$from_id][$to_id] = $kept;

        $variant = DataFlowNode::getForPathVariant($from_id, $from, $moved->type);
        $this->nodes[$variant->id] = $variant;
        $this->mergePath($from_id, $from, $variant->id, $moved);
        $this->forward_edges[$variant->id][$to_id] = new Path('=', 0);
    }

    /**
     * @psalm-external-mutation-free
     */
    public function addSource(DataFlowNode $node): void
    {
        $this->sources[$node->id] = $node;
    }

    /**
     * @psalm-external-mutation-free
     */
    public function addSink(DataFlowNode $node): void
    {
        $this->sinks[$node->id] = $node;
        // in the rare case the sink is the _next_ node, this is necessary
        $this->nodes[$node->id] = $node;
    }

    /**
     * @psalm-external-mutation-free
     */
    public function addGraph(self $other): void
    {
        $this->sources += $other->sources;
        $this->sinks += $other->sinks;
        $this->nodes += $other->nodes;
        $this->specialized_calls += $other->specialized_calls;

        foreach ($other->speculative_calls as $key => $map) {
            $this->speculative_calls[$key] = ($this->speculative_calls[$key] ?? []) + $map;
        }

        foreach ($other->forward_edges as $key => $map) {
            if (!isset($this->forward_edges[$key])) {
                $this->forward_edges[$key] = $map;

                continue;
            }

            $from = $this->nodes[$key] ?? $this->sources[$key] ?? null;

            foreach ($map as $to_id => $path) {
                $this->mergePath($key, $from, $to_id, $path);
            }
        }

        foreach ($other->specializations as $key => $map) {
            if (!isset($this->specializations[$key])) {
                $this->specializations[$key] = $map;
            } else {
                $this->specializations[$key] += $map;
            }
        }
    }

    /**
     * @psalm-mutation-free
     */
    public function getPredecessorPath(DataFlowNode $source): string
    {
        $location_summary = '';

        if ($source->code_location) {
            $location_summary = $source->code_location->getShortSummary();
        }

        $source_descriptor = $source->label . ($location_summary ? ' (' . $location_summary . ')' : '');

        $previous_source = $source->taintSource;

        if ($previous_source) {
            if ($previous_source === $source) {
                return '';
            }

            if ($source->code_location
                && $previous_source->code_location
                && $previous_source->code_location->getHash() === $source->code_location->getHash()
                && $previous_source->taintSource
            ) {
                return $this->getPredecessorPath($previous_source->taintSource) . ' -> ' . $source_descriptor;
            }

            return $this->getPredecessorPath($previous_source) . ' -> ' . $source_descriptor;
        }

        return $source_descriptor;
    }

    /**
     * @psalm-mutation-free
     */
    public function getSuccessorPath(DataFlowNode $sink): string
    {
        $location_summary = '';

        if ($sink->code_location) {
            $location_summary = $sink->code_location->getShortSummary();
        }

        $sink_descriptor = $sink->label . ($location_summary ? ' (' . $location_summary . ')' : '');

        $next_sink = $sink->taintSource;

        if ($next_sink) {
            if ($next_sink === $sink) {
                return '';
            }

            if ($sink->code_location
                && $next_sink->code_location
                && $next_sink->code_location->getHash() === $sink->code_location->getHash()
                && $next_sink->taintSource
            ) {
                return $sink_descriptor . ' -> ' . $this->getSuccessorPath($next_sink->taintSource);
            }

            return $sink_descriptor . ' -> ' . $this->getSuccessorPath($next_sink);
        }

        return $sink_descriptor;
    }

    /**
     * @return list<array{location: ?CodeLocation, label: string, entry_path_type: string}>
     * @psalm-pure
     */
    public function getIssueTrace(DataFlowNode $source): array
    {
        $out = [];
        do {
            /** @var DataFlowNode $source */
            $previous_source = $source->taintSource;
            if ($previous_source === $source) {
                break;
            }
            $path_types = $source->path_types;
            array_unshift($out, [
                'location' => $source->code_location,
                'label' => $source->label,
                'entry_path_type' => end($path_types) ?: '',
            ]);
            $source = $previous_source;
        } while ($previous_source);

        return $out;
    }

    public function connectSinksAndSources(Progress $progress): void
    {
        $progress->startPhase(Phase::TAINT_GRAPH_RESOLUTION);

        $project_analyzer = ProjectAnalyzer::getInstance();
        $codebase = $project_analyzer->getCodebase();

        $this->despecializeImpureCalls($codebase);

        // Remove all specializations without an outgoing edge
        foreach ($this->specializations as $k => &$map) {
            foreach ($map as $kk => $specialized_id) {
                if (!isset($this->forward_edges[$specialized_id])) {
                    unset($map[$kk]);
                }
            }
            if (!$map) {
                unset($this->specializations[$k]);
            }
        } unset($map);

        $resolution = new TaintFlowResolution(
            $this,
            $this->forward_edges,
            $this->nodes,
            $this->sources,
            $this->sinks,
            $this->specializations,
            $this->specialized_calls,
            $this->despecialized_calls,
            Config::getInstance(),
            $project_analyzer,
            $codebase,
        );

        $this->sinks = [];
        $this->sources = [];

        $resolution->resolve($progress);

        $progress->taskDone(0);
    }

    /**
     * Reports the flow of taints $taints from $predecessor into $sink.
     */
    public function reportTaintedFlow(
        DataFlowNode $predecessor,
        DataFlowNode $sink,
        int $taints,
        Config $config,
        Codebase $codebase,
    ): void {
        if ($predecessor->code_location === null) {
            return;
        }

        if ($sink->code_location
            && $config->reportIssueInFile('TaintedInput', $sink->code_location->file_path)
        ) {
            $issue_location = $sink->code_location;
        } else {
            $issue_location = $predecessor->code_location;
        }

        $issue_trace = $this->getIssueTrace($predecessor);
        $path = $this->getPredecessorPath($predecessor)
            . ' -> ' . $this->getSuccessorPath($sink);

        $max = $codebase->taint_count;
        for ($x = 0; $x < $max; $x++) {
            $t = 1 << $x;
            if (!($taints & $t)) {
                continue;
            }
            $issue = match ($t) {
                TaintKind::INPUT_CALLABLE => new TaintedCallable(
                    'Detected tainted text',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_UNSERIALIZE => new TaintedUnserialize(
                    'Detected tainted code passed to unserialize or similar',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_INCLUDE => new TaintedInclude(
                    'Detected tainted code passed to include or similar',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_EVAL => new TaintedEval(
                    'Detected tainted code passed to eval or similar',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_SQL => new TaintedSql(
                    'Detected tainted SQL',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_NOSQL => new TaintedNosql(
                    'Detected tainted NoSQL query',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_HTML => new TaintedHtml(
                    'Detected tainted HTML',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_HAS_QUOTES => new TaintedTextWithQuotes(
                    'Detected tainted text with possible quotes',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_SHELL => new TaintedShell(
                    'Detected tainted shell code',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::USER_SECRET => new TaintedUserSecret(
                    'Detected tainted user secret leaking',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::SYSTEM_SECRET => new TaintedSystemSecret(
                    'Detected tainted system secret leaking',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_SSRF => new TaintedSSRF(
                    'Detected tainted network request',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_LDAP => new TaintedLdap(
                    'Detected tainted LDAP request',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_COOKIE => new TaintedCookie(
                    'Detected tainted cookie',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_FILE => new TaintedFile(
                    'Detected tainted file handling',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_HEADER => new TaintedHeader(
                    'Detected tainted header',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_XPATH => new TaintedXpath(
                    'Detected tainted xpath query',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_SLEEP => new TaintedSleep(
                    'Detected tainted sleep',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_EXTRACT => new TaintedExtract(
                    'Detected tainted extract',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                TaintKind::INPUT_LLM_PROMPT => new TaintedLlmPrompt(
                    'Detected tainted LLM prompt',
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
                default => new TaintedCustom(
                    'Detected tainted ' . $codebase->custom_taints[$t],
                    $issue_location,
                    $issue_trace,
                    $path,
                ),
            };

            IssueBuffer::maybeAdd($issue);
        }
    }
}
