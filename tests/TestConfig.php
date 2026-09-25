<?php

declare(strict_types=1);

namespace Psalm\Tests;

use Override;
use Psalm\Config;
use Psalm\Config\ProjectFileFilter;
use Psalm\Internal\IncludeCollector;
use SimpleXMLElement;

use function getcwd;

final class TestConfig extends Config
{
    #[Override]
    public function initProjectFilesNow(): bool
    {
        return true;
    }

    private static ?ProjectFileFilter $cached_project_files = null;

    public function __construct()
    {
        parent::__construct();

        foreach ($this->php_extensions as $ext => $_enabled) {
            $this->php_extensions[$ext] = true;
        }

        $this->throw_exception = true;
        $this->use_docblock_types = true;
        $this->level = 1;
        $this->cache_directory = null;
        $this->ignore_internal_falsable_issues = true;
        $this->ignore_internal_nullable_issues = true;
        $this->ensure_override_attribute = false;
        $this->strict_binary_operands = false;

        $this->base_dir = (string) getcwd();

        if (!self::$cached_project_files) {
            self::$cached_project_files = ProjectFileFilter::loadFromXMLElement(
                new SimpleXMLElement($this->getContents()),
                $this->base_dir,
                true,
            );
        }

        $this->setCustomErrorLevel(
            'MissingImmutableAnnotation',
            Config::REPORT_SUPPRESS,
        );
        $this->setCustomErrorLevel(
            'MissingPureAnnotation',
            Config::REPORT_SUPPRESS,
        );
        $this->setCustomErrorLevel(
            'MissingAbstractPureAnnotation',
            Config::REPORT_SUPPRESS,
        );
        $this->setCustomErrorLevel(
            'MissingInterfaceImmutableAnnotation',
            Config::REPORT_SUPPRESS,
        );

        $this->project_files = self::$cached_project_files;
        $this->setIncludeCollector(new IncludeCollector());

        $this->collectPredefinedConstants();
        $this->collectPredefinedFunctions();
    }

    /**
     * The tests write `TestConfig::loadFromXML(...)` and expect a config with the test defaults
     * (`throw_exception`, error level 1, the fixture project files) that the XML then refines.
     *
     * @param non-empty-string $file_contents
     */
    public static function loadFromXML(
        string $base_dir,
        string $file_contents,
        ?string $current_dir = null,
        ?string $file_path = null,
    ): Config {
        return Config::loadFromXMLInto(new self(), $base_dir, $file_contents, $current_dir, $file_path);
    }

    /**
     * @psalm-pure
     */
    protected function getContents(): string
    {
        return '<?xml version="1.0"?>
                <projectFiles>
                    <directory name="src" />
                    <file name="tests/somefile.php" />
                    <ignoreFiles>
                        <directory name="src/Psalm/Internal/Stubs" />
                    </ignoreFiles>
                </projectFiles>';
    }

    /**
     * @return false
     * @psalm-pure
     */
    #[Override]
    public function getComposerFilePathForClassLike(string $fq_classlike_name): bool
    {
        return false;
    }

    /**
     * @psalm-pure
     */
    #[Override]
    public function getProjectDirectories(): array
    {
        return [];
    }
}
