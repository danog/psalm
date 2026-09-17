<?php

declare(strict_types=1);

namespace Psalm\Internal\Provider;

use Psalm\Config;
use Psalm\Internal\Codebase\Analyzer;
use Psalm\Internal\Codebase\MutationLevelResolver;
use Psalm\Internal\Codebase\MutationInfo;

/**
 * Used to determine which files reference other files, necessary for using the --diff
 * option from the command line.
 *
 * @psalm-import-type FileMapType from Analyzer
 * @internal
 */
final class FileReferenceCacheProvider
{
    /** @var array<string, array{a: array<int, string>, i: array<int, string>}>|null */
    private ?array $file_references = null;

    /** @var array<string, string>|null */
    private ?array $classlike_files = null;

    /** @var array<string, array<string, bool>>|null */
    private ?array $nonmethod_class_references = null;

    /** @var array<string, array<string, bool>>|null */
    private ?array $method_class_references = null;

    /** @var array<string, array<string, bool>>|null */
    private ?array $method_member_references = null;

    /** @var array<string, array<string, bool>>|null */
    private ?array $mixed_member_name_references = null;

    /** @var array<string, array<int, array<string, bool>>>|null */
    private ?array $method_param_uses = null;

    /** @var array<string, array<string, bool>>|null */
    private ?array $method_dependencies = null;

    /**
     * @var array{
     *     edges: array<string, array<string, string>>,
     *     node_files: array<string, string>,
     *     mutation_info: array<string, \Psalm\Internal\Codebase\MutationInfo>
     * }|null
     */
    private ?array $code_use_graph = null;

    /** @var array<string, array<int, \Psalm\Internal\Analyzer\IssueData>>|null */
    private ?array $issues = null;

    /** @var array<string, array<string, int>>|null */
    private ?array $analyzed_methods = null;

    /** @var array<string, FileMapType>|null */
    private ?array $file_maps = null;

    /** @var array<string, array{int, int}>|null */
    private ?array $type_coverage = null;

    /** The port keeps no persistent cache: every item lives in memory for the run. */
    public function __construct(Config $config, string $composerLock, public readonly bool $persistent = true)
    {
    }

    public function consolidate(): void
    {
    }

    /** @return array<string, array{a: array<int, string>, i: array<int, string>}>|null */
    public function getCachedFileReferences(): ?array
    {
        return $this->file_references;
    }

    /** @return array<string, string>|null */
    public function getCachedClassLikeFiles(): ?array
    {
        return $this->classlike_files;
    }

    /** @return array<string, array<string, bool>>|null */
    public function getCachedNonMethodClassReferences(): ?array
    {
        return $this->nonmethod_class_references;
    }

    /** @return array<string, array<string, bool>>|null */
    public function getCachedMethodClassReferences(): ?array
    {
        return $this->method_class_references;
    }

    /** @return array<string, array<string, bool>>|null */
    public function getCachedMethodMemberReferences(): ?array
    {
        return $this->method_member_references;
    }

    /** @return array<string, array<string, bool>>|null */
    public function getCachedMixedMemberNameReferences(): ?array
    {
        return $this->mixed_member_name_references;
    }

    /** @return array<string, array<int, array<string, bool>>>|null */
    public function getCachedMethodParamUses(): ?array
    {
        return $this->method_param_uses;
    }

    /** @return array<string, array<string, bool>>|null */
    public function getCachedMethodDependencies(): ?array
    {
        return $this->method_dependencies;
    }

    /**
     * @return array{
     *     edges: array<string, array<string, string>>,
     *     node_files: array<string, string>,
     *     mutation_info: array<string, \Psalm\Internal\Codebase\MutationInfo>
     * }|null
     */
    public function getCachedCodeUseGraph(): ?array
    {
        return $this->code_use_graph;
    }

    /** @return array<string, array<int, \Psalm\Internal\Analyzer\IssueData>>|null */
    public function getCachedIssues(): ?array
    {
        return $this->issues;
    }

    /** @param array<string, array{a: array<int, string>, i: array<int, string>}> $file_references */
    public function setCachedFileReferences(array $file_references): void
    {
        $this->file_references = $file_references;
    }

    /** @param array<string, string> $file_references */
    public function setCachedClassLikeFiles(array $file_references): void
    {
        $this->classlike_files = $file_references;
    }

    /** @param array<string, array<string, bool>> $file_class_references */
    public function setCachedNonMethodClassReferences(array $file_class_references): void
    {
        $this->nonmethod_class_references = $file_class_references;
    }

    /** @param array<string, array<string, bool>> $method_class_references */
    public function setCachedMethodClassReferences(array $method_class_references): void
    {
        $this->method_class_references = $method_class_references;
    }

    /** @param array<string, array<string, bool>> $member_references */
    public function setCachedMethodMemberReferences(array $member_references): void
    {
        $this->method_member_references = $member_references;
    }

    /** @param array<string, array<string, bool>> $references */
    public function setCachedMixedMemberNameReferences(array $references): void
    {
        $this->mixed_member_name_references = $references;
    }

    /** @param array<string, array<int, array<string, bool>>> $uses */
    public function setCachedMethodParamUses(array $uses): void
    {
        $this->method_param_uses = $uses;
    }

    /** @param array<string, array<string, bool>> $dependencies */
    public function setCachedMethodDependencies(array $dependencies): void
    {
        $this->method_dependencies = $dependencies;
    }

    /** @param array<string, array<int, \Psalm\Internal\Analyzer\IssueData>> $issues */
    public function setCachedIssues(array $issues): void
    {
        $this->issues = $issues;
    }

    /**
     * @param array{
     *     edges: array<string, array<string, string>>,
     *     node_files: array<string, string>,
     *     mutation_info: array<string, MutationInfo>
     * } $data
     */
    public function setCachedCodeUseGraph(array $data): void
    {
        $this->code_use_graph = $data;
    }

    /**
     * @return array<string, array<string, int>>|false
     */
    public function getAnalyzedMethodCache(): array|false
    {
        /** @var null|array<string, array<string, int>> $cache_item */
        $cache_item = $this->analyzed_methods;

        return $cache_item ?? false;
    }

    /**
     * @param array<string, array<string, int>> $analyzed_methods
     */
    public function setAnalyzedMethodCache(array $analyzed_methods): void
    {
        $this->analyzed_methods = $analyzed_methods;
    }

    /**
     * @return array<string, FileMapType>|false
     */
    public function getFileMapCache(): array|false
    {
        /** @var array<string, FileMapType>|null $cache_item */
        $cache_item = $this->file_maps;

        return $cache_item ?? false;
    }

    /**
     * @param array<string, FileMapType> $file_maps
     */
    public function setFileMapCache(array $file_maps): void
    {
        $this->file_maps = $file_maps;
    }

    /**
     * @return array<string, array{int, int}>|false
     */
    public function getTypeCoverage(): array|false
    {
        /** @var array<string, array{int, int}>|null $cache_item */
        $cache_item = $this->type_coverage;

        return $cache_item ?? false;
    }

    /**
     * @param array<string, array{int, int}> $mixed_counts
     */
    public function setTypeCoverage(array $mixed_counts): void
    {
        $this->type_coverage = $mixed_counts;
    }
}
