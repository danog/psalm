<?php

declare(strict_types=1);

namespace Psalm\Internal\Composer;

use PhpToken;

use function array_merge;
use function array_pop;
use function count;
use function end;
use function explode;
use function file_exists;
use function file_get_contents;
use function implode;
use function is_dir;
use function is_file;
use function is_string;
use function json_decode;
use function ltrim;
use function preg_match;
use function realpath;
use function rtrim;
use function scandir;
use function str_contains;
use function str_ends_with;
use function str_replace;
use function str_starts_with;
use function strrpos;
use function strtr;
use function substr;
use function trim;

use const DIRECTORY_SEPARATOR;
use const T_CLASS;
use const T_DOUBLE_COLON;
use const T_ENUM;
use const T_INTERFACE;
use const T_NAMESPACE;
use const T_NAME_QUALIFIED;
use const T_NEW;
use const T_NS_SEPARATOR;
use const T_STRING;
use const T_TRAIT;

/**
 * Composer's autoload configuration, read rather than executed.
 *
 * Psalm normally locates a dependency's sources by asking the ClassLoader that `vendor/autoload.php`
 * registered. A compiled analyzer has no interpreter to require that file with, so the same question
 * is answered here from the data Composer writes next to it: the root `composer.json` and
 * `vendor/composer/installed.json` carry every package's PSR-4 and PSR-0 roots, its classmap paths
 * and the files it always loads. Reading them gives the interpreted and the compiled analyzer the
 * same view of a project's dependencies.
 *
 * The autoload section of a package, as composer.json declares it and installed.json repeats it:
 *
 * @psalm-type AutoloadSection = array{
 *     'psr-4'?: array<string, string|list<string>>,
 *     'psr-0'?: array<string, string|list<string>>,
 *     classmap?: list<string>,
 *     files?: list<string>,
 *     'exclude-from-classmap'?: list<string>
 * }
 * @psalm-type RootComposerJson = array{autoload?: AutoloadSection, 'autoload-dev'?: AutoloadSection}
 * @psalm-type InstalledPackage = array{name: string, 'install-path'?: string, autoload?: AutoloadSection}
 * @psalm-type InstalledJson = array{packages: list<InstalledPackage>}
 * @internal
 */
final class AutoloadMap
{
    /**
     * @param array<string, list<string>> $psr4 prefix (ending in a backslash) to source roots
     * @param array<string, list<string>> $psr0 prefix to source roots
     * @param list<string> $fallback_dirs roots registered under the empty PSR-4 prefix
     * @param list<string> $classmap_paths files and directories Composer builds a classmap from
     * @param list<string> $exclude_patterns path fragments excluded from those classmaps
     * @param list<string> $files the always-loaded files (function and constant definitions)
     * @param array<string, string> $seeded_classes classes Composer's generator maps by itself
     * @param string $vendor_path the project's vendor directory
     */
    private function __construct(
        private readonly array $psr4,
        private readonly array $psr0,
        private readonly array $fallback_dirs,
        private readonly array $classmap_paths,
        private readonly array $exclude_patterns,
        private readonly array $files,
        private readonly array $seeded_classes,
        private readonly string $vendor_path,
    ) {
    }

    /**
     * What `require 'vendor/autoload.php'` loads: the autoloader stub, Composer's runtime in vendor/composer,
     * and the always-loaded files. When Psalm runs from the project's own vendor/bin (the usual way), all of
     * these are already included in the analyzing process, and an `include` of one is not analyzed.
     *
     * @return list<string> real paths
     */
    public function getBootstrapFiles(): array
    {
        $paths = [$this->vendor_path . DIRECTORY_SEPARATOR . 'autoload.php'];
        $composer_dir = $this->vendor_path . DIRECTORY_SEPARATOR . 'composer';
        foreach (is_dir($composer_dir) ? (scandir($composer_dir) ?: []) : [] as $entry) {
            if (str_ends_with($entry, '.php')) {
                $paths[] = $composer_dir . DIRECTORY_SEPARATOR . $entry;
            }
        }
        $out = [];
        foreach ([...$paths, ...$this->getAutoloadedFiles()] as $path) {
            $real = realpath($path);
            if ($real !== false) {
                $out[] = $real;
            }
        }
        return $out;
    }

    /**
     * Built on the first lookup that the PSR roots cannot answer.
     *
     * @var array<string, string>|null
     */
    private ?array $classmap = null;

    /**
     * Every autoload rule in effect for a project: the root package's own, plus one set per
     * installed package. Returns null when the project has no Composer metadata to read.
     *
     * @param string $vendor_dir the vendor directory's name, as composer.json configures it
     */
    public static function fromProject(string $root_dir, string $vendor_dir): ?self
    {
        $root_dir = rtrim($root_dir, DIRECTORY_SEPARATOR);
        $vendor_path = $root_dir . DIRECTORY_SEPARATOR . rtrim($vendor_dir, DIRECTORY_SEPARATOR);
        $composer_dir = $vendor_path . DIRECTORY_SEPARATOR . 'composer';

        $root_json = self::decodeRootComposerJson($root_dir . DIRECTORY_SEPARATOR . 'composer.json');
        $installed = self::decodeInstalledJson($composer_dir . DIRECTORY_SEPARATOR . 'installed.json');

        if ($root_json === null && $installed === null) {
            return null;
        }

        /** @var list<array{string, AutoloadSection}> base directory and the rules relative to it */
        $sections = [];

        // The root package autoloads from paths relative to the project itself, and its dev
        // section is loaded too (Composer only drops that for `--no-dev` installs).
        if ($root_json !== null) {
            if (isset($root_json['autoload'])) {
                $sections[] = [$root_dir, $root_json['autoload']];
            }

            if (isset($root_json['autoload-dev'])) {
                $sections[] = [$root_dir, $root_json['autoload-dev']];
            }
        }

        if ($installed !== null) {
            foreach ($installed['packages'] as $package) {
                if (!isset($package['autoload'])) {
                    continue;
                }

                // install-path is relative to vendor/composer; older metadata only names the package.
                $base = isset($package['install-path'])
                    ? $composer_dir . DIRECTORY_SEPARATOR . $package['install-path']
                    : $vendor_path . DIRECTORY_SEPARATOR . $package['name'];

                $sections[] = [self::normalize($base), $package['autoload']];
            }
        }

        $psr4 = [];
        $psr0 = [];
        $fallback_dirs = [];
        $classmap_paths = [];
        $exclude_patterns = [];
        $files = [];

        foreach ($sections as [$base, $section]) {
            foreach ($section['files'] ?? [] as $file) {
                $files[] = self::normalize($base . DIRECTORY_SEPARATOR . $file);
            }

            foreach ($section['classmap'] ?? [] as $path) {
                $classmap_paths[] = self::normalize($base . DIRECTORY_SEPARATOR . $path);
            }

            foreach ($section['exclude-from-classmap'] ?? [] as $pattern) {
                $exclude_patterns[] = str_replace('\\', '/', $pattern);
            }

            foreach ($section['psr-4'] ?? [] as $prefix => $paths) {
                $dirs = self::rootsUnder($base, $paths);

                if ($prefix === '') {
                    $fallback_dirs = array_merge($fallback_dirs, $dirs);
                } else {
                    $psr4[$prefix] = array_merge($psr4[$prefix] ?? [], $dirs);
                }
            }

            foreach ($section['psr-0'] ?? [] as $prefix => $paths) {
                $dirs = self::rootsUnder($base, $paths);

                if ($prefix === '') {
                    $fallback_dirs = array_merge($fallback_dirs, $dirs);
                } else {
                    $psr0[$prefix] = array_merge($psr0[$prefix] ?? [], $dirs);
                }
            }
        }

        // Composer's runtime API belongs to no package: the generator writes it straight into the
        // classmap it dumps, so nothing in installed.json would point at it.
        $seeded_classes = [];
        $installed_versions = $composer_dir . DIRECTORY_SEPARATOR . 'InstalledVersions.php';

        if (is_file($installed_versions)) {
            $seeded_classes['Composer\\InstalledVersions'] = $installed_versions;
        }

        return new self(
            $psr4,
            $psr0,
            $fallback_dirs,
            $classmap_paths,
            $exclude_patterns,
            $files,
            $seeded_classes,
            $vendor_path,
        );
    }

    /**
     * The file declaring a class, following the order Composer's own loader uses:
     * the classmap, then the longest matching PSR-4 root, then PSR-0.
     */
    public function findFile(string $class): string|false
    {
        $class = ltrim($class, '\\');

        if ($class === '') {
            return false;
        }

        $classmap = $this->classMap();

        if (isset($classmap[$class])) {
            return $classmap[$class];
        }

        $logical_psr4 = str_replace('\\', DIRECTORY_SEPARATOR, $class) . '.php';

        $sub_path = $class;
        while (false !== $last_pos = strrpos($sub_path, '\\')) {
            $sub_path = substr($sub_path, 0, $last_pos);
            $search = $sub_path . '\\';

            foreach ($this->psr4[$search] ?? [] as $dir) {
                $file = $dir . DIRECTORY_SEPARATOR . substr($logical_psr4, $last_pos + 1);

                if (file_exists($file)) {
                    return $file;
                }
            }
        }

        // PSR-0 spells the underscores in the class name (but not in the namespace) as directories.
        if (false !== $pos = strrpos($class, '\\')) {
            $logical_psr0 = substr($logical_psr4, 0, $pos + 1)
                . strtr(substr($logical_psr4, $pos + 1), '_', DIRECTORY_SEPARATOR);
        } else {
            $logical_psr0 = strtr($class, '_', DIRECTORY_SEPARATOR) . '.php';
        }

        foreach ($this->psr0 as $prefix => $dirs) {
            if (!str_starts_with($class, $prefix)) {
                continue;
            }

            foreach ($dirs as $dir) {
                $file = $dir . DIRECTORY_SEPARATOR . $logical_psr0;

                if (file_exists($file)) {
                    return $file;
                }
            }
        }

        foreach ($this->fallback_dirs as $dir) {
            foreach ([$logical_psr4, $logical_psr0] as $logical) {
                $file = $dir . DIRECTORY_SEPARATOR . $logical;

                if (file_exists($file)) {
                    return $file;
                }
            }
        }

        return false;
    }

    /**
     * The PSR-4 roots, in the shape Composer's ClassLoader reports them.
     *
     * @return array<string, list<string>>
     */
    public function getPrefixesPsr4(): array
    {
        return $this->psr4;
    }

    /**
     * The files Composer loads unconditionally — where a dependency's functions and constants live.
     *
     * @return list<string>
     */
    public function getAutoloadedFiles(): array
    {
        $files = [];

        foreach ($this->files as $file) {
            if (is_file($file)) {
                $files[] = $file;
            }
        }

        return $files;
    }

    /**
     * The root package's manifest, decoded in the shape Composer documents for it.
     *
     * @return RootComposerJson|null
     */
    private static function decodeRootComposerJson(string $path): ?array
    {
        $contents = is_file($path) ? file_get_contents($path) : false;

        if ($contents === false) {
            return null;
        }

        return json_decode($contents, true);
    }

    /**
     * The installed packages, as Composer 2 records them (`{"packages": [...], "dev": ...}`).
     *
     * @return InstalledJson|null
     */
    private static function decodeInstalledJson(string $path): ?array
    {
        $contents = is_file($path) ? file_get_contents($path) : false;

        if ($contents === false) {
            return null;
        }

        return json_decode($contents, true);
    }

    /**
     * A PSR entry names one source root or several; both are resolved under the package.
     *
     * @param string|list<string> $paths
     * @return list<string>
     */
    private static function rootsUnder(string $base, string|array $paths): array
    {
        $dirs = [];

        foreach (is_string($paths) ? [$paths] : $paths as $path) {
            $dirs[] = self::normalize($base . DIRECTORY_SEPARATOR . $path);
        }

        return $dirs;
    }

    /**
     * The classmap, built the way Composer builds it: every class declared by a file under one of
     * the declared classmap paths. Built once, on the first lookup that needs it.
     *
     * @return array<string, string>
     */
    private function classMap(): array
    {
        if ($this->classmap !== null) {
            return $this->classmap;
        }

        $classmap = $this->seeded_classes;

        foreach ($this->classmap_paths as $path) {
            foreach ($this->phpFilesIn($path) as $file) {
                foreach (self::classesIn($file) as $class) {
                    $classmap[$class] ??= $file;
                }
            }
        }

        return $this->classmap = $classmap;
    }

    /**
     * @return list<string>
     */
    private function phpFilesIn(string $path): array
    {
        if (is_file($path)) {
            return $this->isExcluded($path) ? [] : [$path];
        }

        if (!is_dir($path)) {
            return [];
        }

        $files = [];
        $queue = [$path];

        while ($queue !== []) {
            $dir = array_pop($queue);
            $entries = scandir($dir);

            foreach ($entries === false ? [] : $entries as $entry) {
                if ($entry === '.' || $entry === '..') {
                    continue;
                }

                $full = $dir . DIRECTORY_SEPARATOR . $entry;

                if (is_dir($full)) {
                    $queue[] = $full;
                } elseif ((str_ends_with($entry, '.php') || str_ends_with($entry, '.inc'))
                    && !$this->isExcluded($full)
                ) {
                    $files[] = $full;
                }
            }
        }

        return $files;
    }

    private function isExcluded(string $file): bool
    {
        $normalized = str_replace('\\', '/', $file);

        foreach ($this->exclude_patterns as $pattern) {
            if (str_contains($normalized, $pattern)) {
                return true;
            }
        }

        return false;
    }

    /**
     * The classes, interfaces, traits and enums a file declares.
     *
     * @return list<string>
     */
    private static function classesIn(string $file): array
    {
        $contents = file_get_contents($file);

        if ($contents === false
            || !preg_match('{\b(?:class|interface|trait|enum)\s}i', $contents)
        ) {
            return [];
        }

        $tokens = PhpToken::tokenize($contents);
        $namespace = '';
        $classes = [];
        $count = count($tokens);

        for ($i = 0; $i < $count; ++$i) {
            $id = $tokens[$i]->id;

            if ($id === T_NAMESPACE) {
                $namespace = '';

                for ($j = $i + 1; $j < $count; ++$j) {
                    $next = $tokens[$j]->id;

                    if ($next === T_STRING || $next === T_NS_SEPARATOR || $next === T_NAME_QUALIFIED) {
                        $namespace .= $tokens[$j]->text;
                    } elseif ($tokens[$j]->text === '{' || $tokens[$j]->text === ';') {
                        break;
                    }
                }

                $namespace = trim($namespace, '\\');
                continue;
            }

            if ($id !== T_CLASS && $id !== T_INTERFACE && $id !== T_TRAIT && $id !== T_ENUM) {
                continue;
            }

            // `Foo::class` and `new class {}` declare nothing.
            $prev = self::previousMeaningful($tokens, $i);

            if ($prev !== null && ($prev->id === T_DOUBLE_COLON || $prev->id === T_NEW)) {
                continue;
            }

            for ($j = $i + 1; $j < $count; ++$j) {
                if ($tokens[$j]->id === T_STRING) {
                    $classes[] = $namespace === '' ? $tokens[$j]->text : $namespace . '\\' . $tokens[$j]->text;
                    break;
                }

                if ($tokens[$j]->text === '{' || $tokens[$j]->text === '(') {
                    break;
                }
            }
        }

        return $classes;
    }

    /**
     * @param list<PhpToken> $tokens
     */
    private static function previousMeaningful(array $tokens, int $i): ?PhpToken
    {
        for ($j = $i - 1; $j >= 0; --$j) {
            if (!$tokens[$j]->isIgnorable()) {
                return $tokens[$j];
            }
        }

        return null;
    }

    /**
     * Resolve `..` segments without touching the filesystem: install paths are written relative to
     * vendor/composer, and the file may legitimately not exist yet.
     */
    private static function normalize(string $path): string
    {
        $path = str_replace('\\', DIRECTORY_SEPARATOR, $path);
        $out = [];

        foreach (explode(DIRECTORY_SEPARATOR, $path) as $i => $segment) {
            if ($segment === '.' || ($segment === '' && $i > 0)) {
                continue;
            }

            if ($segment === '..' && $out !== [] && end($out) !== '..' && end($out) !== '') {
                array_pop($out);
                continue;
            }

            $out[] = $segment;
        }

        return rtrim(implode(DIRECTORY_SEPARATOR, $out), DIRECTORY_SEPARATOR);
    }
}
