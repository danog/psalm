<?php

declare(strict_types=1);

namespace Psalm\Tests\Internal\Provider;

use Override;
use Psalm\Config;
use Psalm\Internal\Provider\ClassLikeStorageCacheProvider;
use Psalm\Storage\ClassLikeStorage;

use function hash;
use function strtolower;

/**
 * In-memory class storage cache shared by every test of a process (see SharedStubFileStorageCacheProvider):
 * only consulted for classes of cached stub files, since exhuming happens from a cached file storage.
 */
final class SharedStubClassLikeStorageCacheProvider extends ClassLikeStorageCacheProvider
{
    public int $php_version_id = 0;

    public function __construct(Config $config)
    {
        parent::__construct($config, '', false);
    }

    #[Override]
    public function writeToCache(ClassLikeStorage $storage, string $file_path, string $file_contents): void
    {
        $this->cache->saveItem($file_path . "\0" . strtolower($storage->name), $storage, $this->php_version_id . ':' . hash('xxh128', $file_contents));
    }

    #[Override]
    public function getLatestFromCache(string $fq_classlike_name_lc, ?string $file_path, string $file_contents): ClassLikeStorage
    {
        return $this->cache->getItem($file_path . "\0" . $fq_classlike_name_lc, $this->php_version_id . ':' . hash('xxh128', $file_contents));
    }
}
