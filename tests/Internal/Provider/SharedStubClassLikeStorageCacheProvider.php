<?php

declare(strict_types=1);

namespace Psalm\Tests\Internal\Provider;

use Override;
use Psalm\Config;
use Psalm\Internal\Provider\ClassLikeStorageCacheProvider;
use Psalm\Storage\ClassLikeStorage;
use UnexpectedValueException;

use function hash;
use function strtolower;

/**
 * In-memory class storage cache shared by every test of a process (see SharedStubFileStorageCacheProvider):
 * only consulted for classes of cached stub files, since exhuming happens from a cached file storage.
 */
final class SharedStubClassLikeStorageCacheProvider extends ClassLikeStorageCacheProvider
{
    public int $php_version_id = 0;

    /** @var array<string, list{string, ClassLikeStorage}> */
    private array $shared = [];

    public function __construct(Config $config)
    {
        parent::__construct($config, '', false);
    }

    /** Whether the storage of a class of a cached stub file is cached (a test may swap the class cache). */
    public function hasStorage(string $file_path, string $fq_classlike_name_lc, string $file_contents): bool
    {
        $key = strtolower($file_path) . "\0" . $fq_classlike_name_lc;

        return isset($this->shared[$key]) && $this->shared[$key][0] === $this->php_version_id . ':' . hash('xxh128', $file_contents);
    }

    #[Override]
    public function writeToCache(ClassLikeStorage $storage, string $file_path, string $file_contents): void
    {
        $this->shared[strtolower($file_path) . "\0" . strtolower($storage->name)] = [$this->php_version_id . ':' . hash('xxh128', $file_contents), $storage];
    }

    #[Override]
    public function getLatestFromCache(string $fq_classlike_name_lc, ?string $file_path, string $file_contents): ClassLikeStorage
    {
        $key = strtolower((string) $file_path) . "\0" . $fq_classlike_name_lc;
        $storage = isset($this->shared[$key]) && $this->shared[$key][0] === $this->php_version_id . ':' . hash('xxh128', $file_contents) ? $this->shared[$key][1] : null;
        if ($storage === null) {
            throw new UnexpectedValueException('No shared stub storage for ' . $fq_classlike_name_lc . ' in ' . (string) $file_path);
        }
        return $storage;
    }
}
