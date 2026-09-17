<?php

declare(strict_types=1);

namespace Psalm\Internal\Provider;

use Psalm\Config;
use Psalm\Storage\ClassLikeStorage;
use UnexpectedValueException;

use function array_merge;
use function dirname;
use function file_exists;
use function filemtime;
use function hash;
use function strtolower;

use const DIRECTORY_SEPARATOR;

/**
 * @internal
 */
class ClassLikeStorageCacheProvider
{
    /**
     * In-memory cache (the port keeps no persistent cache): file path + class name => [contents hash, storage].
     *
     * @var array<string, list{string, ClassLikeStorage}>
     */
    private array $items = [];

    public function __construct(Config $config, string $composerLock, bool $persistent = true)
    {
        $storage_dir = dirname(__DIR__, 2) . DIRECTORY_SEPARATOR . 'Storage' . DIRECTORY_SEPARATOR;

        $dependent_files = [
            $storage_dir . 'FileStorage.php',
            $storage_dir . 'FunctionLikeStorage.php',
            $storage_dir . 'ClassLikeStorage.php',
            $storage_dir . 'MethodStorage.php',
        ];

        if ($config->eventDispatcher->hasAfterClassLikeVisitHandlers()) {
            $dependent_files = array_merge($dependent_files, $config->plugin_paths);
        }
        
        $dependencies = [$composerLock];

        // an in-memory cache is not invalidated by source changes: skip the dependency inventory
        foreach ($persistent ? $dependent_files : [] as $dependent_file_path) {
            if (!file_exists($dependent_file_path)) {
                throw new UnexpectedValueException($dependent_file_path . ' must exist');
            }

            $dependencies []= filemtime($dependent_file_path);
        }

        // dependencies only matter to a persistent cache
        $dependencies = [];
    }

    public function consolidate(): void
    {
    }

    public function writeToCache(ClassLikeStorage $storage, string $file_path, string $file_contents): void
    {
        $fq_classlike_name_lc = strtolower($storage->name);

        $this->items[$file_path."\0".$fq_classlike_name_lc] = [hash('xxh128', $file_contents), $storage];
    }

    /**
     * @param lowercase-string $fq_classlike_name_lc
     */
    public function getLatestFromCache(
        string $fq_classlike_name_lc,
        ?string $file_path,
        string $file_contents,
    ): ClassLikeStorage {
        $key = $file_path."\0".$fq_classlike_name_lc;
        $hash = hash('xxh128', $file_contents);
        if (isset($this->items[$key]) && $this->items[$key][0] === $hash) {
            return $this->items[$key][1];
        }

        throw new UnexpectedValueException('No cached storage for ' . $fq_classlike_name_lc);
    }
}
