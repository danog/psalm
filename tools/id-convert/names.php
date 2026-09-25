<?php

/**
 * Canonical spellings of names (pzoom resolves names case-sensitively, so a name literal written in lowercase for
 * Psalm's old lowercase keys must become the declared spelling): PHP's own classes, interfaces, traits, enums and
 * their methods (by reflection), and the class-likes declared under a source root.
 */

declare(strict_types=1);

final class CanonicalNames
{
    /** @var ?array<string, string> lowercase name => declared name */
    private static ?array $classes = null;
    /** @var ?array<string, ?string> lowercase method name => declared name (null when spellings differ) */
    private static ?array $methods = null;

    public static function init(string $root): void
    {
        if (self::$classes !== null) {
            return;
        }
        self::$classes = [];
        self::$methods = [];
        foreach ([...get_declared_classes(), ...get_declared_interfaces(), ...get_declared_traits()] as $c) {
            $r = new ReflectionClass($c);
            if ($r->isUserDefined()) {
                continue;
            }
            self::$classes[strtolower($r->getName())] = $r->getName();
            foreach ($r->getMethods() as $m) {
                $lc = strtolower($m->getName());
                $prev = self::$methods[$lc] ?? false;
                self::$methods[$lc] = $prev === false || $prev === $m->getName() ? $m->getName() : null;
            }
        }
        // the project's class-likes and their methods
        foreach (['src', 'tests', 'examples'] as $dir) {
            if (!is_dir($root . $dir)) {
                continue;
            }
            $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root . $dir, FilesystemIterator::SKIP_DOTS));
            foreach ($it as $f) {
                if (!str_ends_with((string) $f, '.php')) {
                    continue;
                }
                $src = (string) file_get_contents((string) $f);
                $ns = preg_match('/^namespace\s+([^;{\s]+)/m', $src, $m) ? $m[1] . '\\' : '';
                if (preg_match_all('/^\s*(?:final\s+|abstract\s+|readonly\s+)*(?:class|interface|trait|enum)\s+(\w+)/m', $src, $cm)) {
                    foreach ($cm[1] as $c) {
                        self::$classes[strtolower($ns . $c)] ??= $ns . $c;
                    }
                }
                if (preg_match_all('/function\s+(\w+)\s*\(/', $src, $mm)) {
                    foreach ($mm[1] as $name) {
                        $lc = strtolower($name);
                        $prev = self::$methods[$lc] ?? false;
                        self::$methods[$lc] = $prev === false || $prev === $name ? $name : null;
                    }
                }
            }
        }
    }

    /** Whether a literal names a known class-like (not a keyword such as `self` or `resource`). */
    public static function isClass(string $literal): bool
    {
        return isset(self::$classes[strtolower(ltrim($literal, '\\'))]);
    }

    /** The declared spelling of a class-like or method name literal, or the literal itself. */
    public static function of(string $literal): string
    {
        $bare = ltrim($literal, '\\');
        $lc = strtolower($bare);
        if (isset(self::$classes[$lc])) {
            return self::$classes[$lc];
        }
        // method names stay as written: Psalm's method maps are keyed by lowercase names (still lowered at the
        // declaration: `strtolower($stmt->name->name)`), so a lowercase method literal is already that key
        return $bare;
    }
}
