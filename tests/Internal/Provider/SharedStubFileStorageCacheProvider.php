<?php

declare(strict_types=1);

namespace Psalm\Tests\Internal\Provider;

use Override;
use Psalm\Config;
use Psalm\Internal\Provider\FileStorageCacheProvider;
use Psalm\Storage\FileStorage;

use function hash;
use function str_ends_with;
use function strtolower;

/**
 * In-memory file storage cache shared by every test of a process, restricted to Psalm's stub files: the
 * stubs are identical for every test, so they are parsed and scanned once and their storages are reused
 * (no per-test re-scan, no copies). Entries are keyed by the analyzed PHP version as well, since the
 * version-specific stubs extend the same classes. Keys are lower-cased paths: the file storage provider
 * asks with lower-cased paths.
 */
final class SharedStubFileStorageCacheProvider extends FileStorageCacheProvider
{
    public int $php_version_id = 0;

    public function __construct(Config $config)
    {
        parent::__construct($config, '', false);
    }

    #[Override]
    public function writeToCache(FileStorage $storage, string $file_contents): void
    {
        if (str_ends_with($storage->file_path, '.phpstub')) {
            $this->cache->saveItem(strtolower($storage->file_path), $storage, $this->php_version_id . ':' . hash('xxh128', $file_contents));
        }
    }

    #[Override]
    public function getLatestFromCache(string $file_path, string $file_contents): ?FileStorage
    {
        if (!str_ends_with($file_path, '.phpstub')) {
            return null;
        }
        return $this->cache->getItem(strtolower($file_path), $this->php_version_id . ':' . hash('xxh128', $file_contents));
    }
}
