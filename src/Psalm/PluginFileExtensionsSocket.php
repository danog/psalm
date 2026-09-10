<?php

declare(strict_types=1);

namespace Psalm;

use Closure;
use LogicException;
use Override;
use Psalm\Internal\Analyzer\FileAnalyzer;
use Psalm\Internal\Analyzer\ProjectAnalyzer;
use Psalm\Internal\Scanner\FileScanner;
use Psalm\Plugin\FileExtensionsInterface;

use function in_array;
use function sprintf;

final class PluginFileExtensionsSocket implements FileExtensionsInterface
{
    /**
     * @var array<string, Closure(string, string, bool): FileScanner>
     */
    private array $additionalFileTypeScanners = [];

    /**
     * @var array<string, Closure(ProjectAnalyzer, string, string): FileAnalyzer>
     */
    private array $additionalFileTypeAnalyzers = [];

    /**
     * @var list<string>
     */
    private array $additionalFileExtensions = [];

    /**
     * @internal
     * @psalm-mutation-free
     */
    public function __construct(
        private readonly Config $config,
    ) {
    }

    /**
     * @param string $fileExtension e.g. `'html'`
     * @param Closure(string, string, bool): FileScanner $factory
     */
    #[Override]
    public function addFileTypeScanner(string $fileExtension, Closure $factory): void
    {
        if (isset($this->config->getFiletypeScanners()[$fileExtension])
            || isset($this->additionalFileTypeScanners[$fileExtension])
        ) {
            throw new LogicException(
                sprintf('Cannot redeclare scanner for file-type %s', $fileExtension),
                1_622_727_272,
            );
        }
        $this->additionalFileTypeScanners[$fileExtension] = $factory;
        $this->addFileExtension($fileExtension);
    }

    /**
     * @return array<string, Closure(string, string, bool): FileScanner>
     */
    public function getAdditionalFileTypeScanners(): array
    {
        return $this->additionalFileTypeScanners;
    }

    /**
     * @param string $fileExtension e.g. `'html'`
     * @param Closure(ProjectAnalyzer, string, string): FileAnalyzer $factory
     */
    #[Override]
    public function addFileTypeAnalyzer(string $fileExtension, Closure $factory): void
    {
        if (isset($this->config->getFiletypeAnalyzers()[$fileExtension])
            || isset($this->additionalFileTypeAnalyzers[$fileExtension])
        ) {
            throw new LogicException(
                sprintf('Cannot redeclare analyzer for file-type %s', $fileExtension),
                1_622_727_282,
            );
        }
        $this->additionalFileTypeAnalyzers[$fileExtension] = $factory;
        $this->addFileExtension($fileExtension);
    }

    /**
     * @return array<string, Closure(ProjectAnalyzer, string, string): FileAnalyzer>
     */
    public function getAdditionalFileTypeAnalyzers(): array
    {
        return $this->additionalFileTypeAnalyzers;
    }

    /**
     * @return list<string> e.g. `['html', 'perl']`
     */
    public function getAdditionalFileExtensions(): array
    {
        return $this->additionalFileExtensions;
    }

    /**
     * @param string $fileExtension e.g. `'html'`
     * @psalm-external-mutation-free
     */
    private function addFileExtension(string $fileExtension): void
    {
        /** @psalm-suppress RedundantCondition */
        if (!in_array($fileExtension, $this->additionalFileExtensions, true)
            && !in_array($fileExtension, $this->config->getFileExtensions(), true)
        ) {
            $this->additionalFileExtensions[] = $fileExtension;
        }
    }
}
