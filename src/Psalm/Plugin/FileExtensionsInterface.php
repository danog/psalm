<?php

declare(strict_types=1);

namespace Psalm\Plugin;

use Closure;
use Psalm\Internal\Analyzer\FileAnalyzer;
use Psalm\Internal\Analyzer\ProjectAnalyzer;
use Psalm\Internal\Scanner\FileScanner;

/**
 * @api
 */
interface FileExtensionsInterface
{
    /**
     * @param string $fileExtension e.g. `'html'`
     * @param Closure(string, string, bool): FileScanner $factory builds the scanner of a file (path, name, will analyze)
     */
    public function addFileTypeScanner(string $fileExtension, Closure $factory): void;

    /**
     * @param string $fileExtension e.g. `'html'`
     * @param Closure(ProjectAnalyzer, string, string): FileAnalyzer $factory builds the analyzer of a file (path, name)
     */
    public function addFileTypeAnalyzer(string $fileExtension, Closure $factory): void;
}
