<?php

declare(strict_types=1);

namespace Psalm\Tests\Config\Plugin;

use Closure;
use Override;
use Psalm\Internal\Analyzer\FileAnalyzer;
use Psalm\Internal\Analyzer\ProjectAnalyzer;
use Psalm\Internal\Scanner\FileScanner;
use Psalm\Plugin\FileExtensionsInterface;
use Psalm\Plugin\PluginFileExtensionsInterface;
use SimpleXMLElement;

final class FileTypeSelfRegisteringPlugin implements PluginFileExtensionsInterface
{
    public const FLAG_SCANNER_TWICE = 1;
    public const FLAG_ANALYZER_TWICE = 2;

    public static string $extension = '';

    /** @var (Closure(string, string, bool): FileScanner)|null */
    public static ?Closure $scanner_factory = null;

    /** @var (Closure(ProjectAnalyzer, string, string): FileAnalyzer)|null */
    public static ?Closure $analyzer_factory = null;

    public static int $flags = 0;

    #[Override]
    public function processFileExtensions(FileExtensionsInterface $fileExtensions, ?SimpleXMLElement $config = null): void
    {
        $scanner_factory = self::$scanner_factory
            ?? static fn(string $file_path, string $file_name, bool $will_analyze): FileScanner
                => new FileScanner($file_path, $file_name, $will_analyze);
        $analyzer_factory = self::$analyzer_factory
            ?? static fn(ProjectAnalyzer $project_analyzer, string $file_path, string $file_name): FileAnalyzer
                => new FileAnalyzer($project_analyzer, $file_path, $file_name);

        $fileExtensions->addFileTypeScanner(self::$extension, $scanner_factory);
        $fileExtensions->addFileTypeAnalyzer(self::$extension, $analyzer_factory);

        if (self::$flags & self::FLAG_SCANNER_TWICE) {
            $fileExtensions->addFileTypeScanner(self::$extension, $scanner_factory);
        }
        if (self::$flags & self::FLAG_ANALYZER_TWICE) {
            $fileExtensions->addFileTypeAnalyzer(self::$extension, $analyzer_factory);
        }
    }
}
