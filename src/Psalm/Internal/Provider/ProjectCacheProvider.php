<?php

declare(strict_types=1);

namespace Psalm\Internal\Provider;

use Psalm\Config;

use function file_exists;
use function file_get_contents;
use function file_put_contents;
use function json_decode;
use function json_encode;
use function touch;

use const DIRECTORY_SEPARATOR;
use const JSON_THROW_ON_ERROR;

/**
 * Used to determine which files reference other files, necessary for using the --diff
 * option from the command line.
 *
 * @internal
 */
class ProjectCacheProvider
{
    private const GOOD_RUN_NAME = 'good_run';

    private const CUSTOM_TAINTS_NAME = 'custom_taints';

    /**
     * Load the custom taint mapping persisted by a previous run, so that the taint bits baked into the
     * reused file/classlike storage cache keep matching their taint names (otherwise cached sinks and
     * sources silently stop matching, and no taint issues are reported when running from cache).
     *
     * @return array{count: int, custom: array<int, string>, map: array<string, int>}|null
     */
    public function loadCustomTaints(): ?array
    {
        $cache_directory = Config::getInstance()->getCacheDirectory();

        if ($cache_directory === null) {
            return null;
        }

        $taints_location = $cache_directory . DIRECTORY_SEPARATOR . self::CUSTOM_TAINTS_NAME;

        if (!file_exists($taints_location)) {
            return null;
        }

        $contents = file_get_contents($taints_location);

        if ($contents === false || $contents === '') {
            return null;
        }

        return self::decodeCustomTaints($contents);
    }

    /**
     * The custom taints cache file is JSON (the compiled analyzer has no serialize()).
     *
     * @return array{count: int, custom: array<int, string>, map: array<string, int>}|null
     */
    private static function decodeCustomTaints(string $contents): ?array
    {
        return json_decode($contents, true);
    }

    /**
     * @param array{count: int, custom: array<int, string>, map: array<string, int>} $data
     */
    public function saveCustomTaints(array $data): void
    {
        $cache_directory = Config::getInstance()->getCacheDirectory();

        if ($cache_directory === null) {
            return;
        }

        file_put_contents(
            $cache_directory . DIRECTORY_SEPARATOR . self::CUSTOM_TAINTS_NAME,
            json_encode($data, JSON_THROW_ON_ERROR),
        );
    }

    public function canDiffFiles(): bool
    {
        $cache_directory = Config::getInstance()->getCacheDirectory();

        return $cache_directory !== null && file_exists($cache_directory . DIRECTORY_SEPARATOR . self::GOOD_RUN_NAME);
    }

    public function processSuccessfulRun(float $start_time, string $psalm_version): void
    {
        $cache_directory = Config::getInstance()->getCacheDirectory();

        if ($cache_directory === null) {
            return;
        }

        $run_cache_location = $cache_directory . DIRECTORY_SEPARATOR . self::GOOD_RUN_NAME;

        file_put_contents($run_cache_location, $psalm_version);

        touch($run_cache_location, (int)$start_time);
    }
}
