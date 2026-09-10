<?php

declare(strict_types=1);

namespace Psalm\Internal;

use function array_diff;
use function array_unique;
use function array_values;
use function get_included_files;
use function preg_grep;

use const PREG_GREP_INVERT;

/**
 * Include collector
 *
 * Used to execute code that may cause file inclusions, and report what files have been included
 * NOTE: dependencies of this class should be kept at minimum, as it's used before autoloader is
 * registered.
 *
 * @internal
 */
final class IncludeCollector
{
    /** @var list<string> */
    private array $included_files = [];

    /**
     * @template T
     * @param callable():T $f
     * @return T
     */
    public function runAndCollect(callable $f)
    {
        $before = get_included_files();
        $ret = $f();
        $after = get_included_files();

        $included = array_diff($after, $before);

        $this->included_files = array_values(array_unique([...$this->included_files, ...$included]));

        return $ret;
    }

    /**
     * @param list<string> $files
     * @psalm-external-mutation-free
     */
    public function addIncludedFiles(array $files): void
    {
        $this->included_files = array_values(array_unique([...$this->included_files, ...$files]));
    }

    /** @return list<string> */
    public function getIncludedFiles(): array
    {
        return $this->included_files;
    }

    /**
     * Whether a file was loaded (or registered as loaded) by the autoloaders.
     */
    public function hasIncludedFile(string $file): bool
    {
        $real = realpath($file);

        foreach ($this->included_files as $included_file) {
            if ($included_file === $file || ($real !== false && realpath($included_file) === $real)) {
                return true;
            }
        }

        return false;
    }

    /**
     * @return list<string>
     * @psalm-mutation-free
     */
    public function getFilteredIncludedFiles(): array
    {
        return array_values(preg_grep('@^phar://@', $this->getIncludedFiles(), PREG_GREP_INVERT));
    }
}
