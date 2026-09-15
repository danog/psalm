<?php

declare(strict_types=1);

namespace Psalm\Internal\Provider;

use Psalm\Config;
use Psalm\Internal\Cache;
use Psalm\Internal\Codebase\Analyzer;
use Psalm\Internal\Codebase\MutationLevelResolver;

/**
 * Used to determine which files reference other files, necessary for using the --diff
 * option from the command line.
 *
 * @psalm-import-type FileMapType from Analyzer
 * @psalm-import-type MutationInfo from MutationLevelResolver
 * @internal
 */
final class FileReferenceCacheProvider
{
    private const REFERENCE_CACHE_NAME = 'references';
    private const CLASSLIKE_FILE_CACHE_NAME = 'classlike_files';
    private const ANALYZED_METHODS_CACHE_NAME = 'analyzed_methods';
    private const METHOD_DEPENDENCIES_CACHE_NAME = 'class_method_dependencies';
    private const ISSUES_CACHE_NAME = 'issues';
    private const FILE_MAPS_CACHE_NAME = 'file_maps';
    private const TYPE_COVERAGE_CACHE_NAME = 'type_coverage';
    private const UNKNOWN_MEMBER_CACHE_NAME = 'unknown_member_references';
    private const METHOD_PARAM_USE_CACHE_NAME = 'method_param_uses';
    private const CODE_USE_GRAPH_CACHE_NAME = 'code_use_graph';
    /** @var Cache<array> */
    private readonly Cache $cache;

    public function __construct(Config $config, string $composerLock, public readonly bool $persistent = true)
    {
        $this->cache = new Cache($config, 'file_reference', [$composerLock], $persistent);
    }

    public function consolidate(): void
    {
        $this->cache->consolidate();
    }

    /** @return array<string, array{a: array<int, string>, i: array<int, string>}>|null */
    public function getCachedFileReferences(): ?array
    {
        return $this->cache->getItem(self::REFERENCE_CACHE_NAME);
    }

    /** @return array<string, string>|null */
    public function getCachedClassLikeFiles(): ?array
    {
        return $this->cache->getItem(self::CLASSLIKE_FILE_CACHE_NAME);
    }

    /** @return array<string, array<string, bool>>|null */
    public function getCachedNonMethodClassReferences(): ?array
    {
        return $this->cache->getItem(self::NONMETHOD_CLASS_REFERENCE_CACHE_NAME);
    }

    /** @return array<string, array<string, bool>>|null */
    public function getCachedMethodClassReferences(): ?array
    {
        return $this->cache->getItem(self::METHOD_CLASS_REFERENCE_CACHE_NAME);
    }

    /** @return array<string, array<string, bool>>|null */
    public function getCachedMethodMemberReferences(): ?array
    {
        return $this->cache->getItem(self::CLASS_METHOD_CACHE_NAME);
    }

    /** @return array<string, array<string, bool>>|null */
    public function getCachedMixedMemberNameReferences(): ?array
    {
        return $this->cache->getItem(self::UNKNOWN_MEMBER_CACHE_NAME);
    }

    /** @return array<string, array<int, array<string, bool>>>|null */
    public function getCachedMethodParamUses(): ?array
    {
        return $this->cache->getItem(self::METHOD_PARAM_USE_CACHE_NAME);
    }

    /** @return array<string, array<int, \Psalm\Internal\Analyzer\IssueData>>|null */
    public function getCachedIssues(): ?array
    {
        return $this->cache->getItem(self::ISSUES_CACHE_NAME);
    }

    /** @param array<string, array{a: array<int, string>, i: array<int, string>}> $file_references */
    public function setCachedFileReferences(array $file_references): void
    {
        $this->cache->saveItem(self::REFERENCE_CACHE_NAME, $file_references);
    }

    /** @param array<string, string> $file_references */
    public function setCachedClassLikeFiles(array $file_references): void
    {
        $this->cache->saveItem(self::CLASSLIKE_FILE_CACHE_NAME, $file_references);
    }

    /** @param array<string, array<string, bool>> $file_class_references */
    public function setCachedNonMethodClassReferences(array $file_class_references): void
    {
        $this->cache->saveItem(self::NONMETHOD_CLASS_REFERENCE_CACHE_NAME, $file_class_references);
    }

    /** @param array<string, array<string, bool>> $method_class_references */
    public function setCachedMethodClassReferences(array $method_class_references): void
    {
        $this->cache->saveItem(self::METHOD_CLASS_REFERENCE_CACHE_NAME, $method_class_references);
    }

    /** @param array<string, array<string, bool>> $member_references */
    public function setCachedMethodMemberReferences(array $member_references): void
    {
        $this->cache->saveItem(self::CLASS_METHOD_CACHE_NAME, $member_references);
    }

    /** @param array<string, array<string, bool>> $member_references */
    public function setCachedMixedMemberNameReferences(array $references): void
    {
        $this->cache->saveItem(self::UNKNOWN_MEMBER_CACHE_NAME, $references);
    }

    /** @param array<string, array<int, array<string, bool>>> $uses */
    public function setCachedMethodParamUses(array $uses): void
    {
        $this->cache->saveItem(self::METHOD_PARAM_USE_CACHE_NAME, $uses);
    }

    /** @param array<string, array<int, \Psalm\Internal\Analyzer\IssueData>> $issues */
    public function setCachedIssues(array $issues): void
    {
        $this->cache->saveItem(self::ISSUES_CACHE_NAME, $issues);
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
        $this->cache->saveItem(self::CODE_USE_GRAPH_CACHE_NAME, $data);
    }

    /**
     * @return array<string, array<string, int>>|false
     */
    public function getAnalyzedMethodCache(): array|false
    {
        /** @var null|array<string, array<string, int>> $cache_item */
        $cache_item = $this->cache->getItem(self::ANALYZED_METHODS_CACHE_NAME);

        return $cache_item ?? false;
    }

    /**
     * @param array<string, array<string, int>> $analyzed_methods
     */
    public function setAnalyzedMethodCache(array $analyzed_methods): void
    {
        $this->cache->saveItem(self::ANALYZED_METHODS_CACHE_NAME, $analyzed_methods);
    }

    /**
     * @return array<string, FileMapType>|false
     */
    public function getFileMapCache(): array|false
    {
        /** @var array<string, FileMapType>|null $cache_item */
        $cache_item = $this->cache->getItem(self::FILE_MAPS_CACHE_NAME);

        return $cache_item ?? false;
    }

    /**
     * @param array<string, FileMapType> $file_maps
     */
    public function setFileMapCache(array $file_maps): void
    {
        $this->cache->saveItem(self::FILE_MAPS_CACHE_NAME, $file_maps);
    }

    /**
     * @return array<string, array{int, int}>|false
     */
    public function getTypeCoverage(): array|false
    {
        /** @var array<string, array{int, int}>|null $cache_item */
        $cache_item = $this->cache->getItem(self::TYPE_COVERAGE_CACHE_NAME);

        return $cache_item ?? false;
    }

    /**
     * @param array<string, array{int, int}> $mixed_counts
     */
    public function setTypeCoverage(array $mixed_counts): void
    {
        $this->cache->saveItem(self::TYPE_COVERAGE_CACHE_NAME, $mixed_counts);
    }
}
