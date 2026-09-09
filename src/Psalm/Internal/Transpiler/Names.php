<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use function array_map;
use function explode;
use function implode;
use function in_array;
use function ltrim;
use function preg_replace;
use function strtolower;
use function str_replace;
use function trim;

/**
 * Rust identifier mangling.
 *
 * @internal
 */
final class Names
{
    private const KEYWORDS = [
        'as', 'break', 'const', 'continue', 'crate', 'else', 'enum', 'extern', 'false', 'fn', 'for', 'if', 'impl', 'in',
        'let', 'loop', 'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return', 'self', 'Self', 'static', 'struct', 'super',
        'trait', 'true', 'type', 'unsafe', 'use', 'where', 'while', 'async', 'await', 'dyn', 'abstract', 'become', 'box',
        'do', 'final', 'macro', 'override', 'priv', 'typeof', 'unsized', 'virtual', 'yield', 'try', 'gen', 'union',
    ];

    /** Names reserved by the generated handle types. */
    private const RESERVED_METHODS = ['new', 'clone', 'from', 'try_from', 'into', 'default', 'drop', 'eq', 'ne', 'cmp', 'hash', 'fmt', 'to_string'];

    /** Escape an arbitrary identifier (variable, field, method name). */
    public static function ident(string|int $name): string
    {
        $name = (string) $name;
        $name = preg_replace('/[^A-Za-z0-9_]/', '_', $name) ?? $name;
        if ($name === '' || ($name[0] >= '0' && $name[0] <= '9')) {
            $name = 'k' . $name;
        }
        if (in_array($name, self::KEYWORDS, true)) {
            if ($name === 'self' || $name === 'Self' || $name === 'super' || $name === 'crate') {
                return 'v_' . $name;
            }
            return $name . '_';
        }
        return $name;
    }

    /** @var array<string, true>|null */
    private static ?array $reserved = null;

    /** Names of runtime functions/macros that generated code calls unqualified. */
    private static function reserved(): array
    {
        if (self::$reserved !== null) {
            return self::$reserved;
        }
        $names = ['cast', 'list', 'map', 'cat', 'sfmt', 'sprintf', 'echo', 'truthy', 'to_str', 'to_int', 'to_float', 'to_key', 'count',
            'identical', 'loose_eq', 'php_cmp', 'spaceship', 'never', 'is_instance', 'src_dir', 'src_file', 'consts', 'registry',
            'this', 'Some', 'None', 'Ok', 'Err', 'Default', 'Rc', 'Ref', 'RefMut', 'RefCell', 'Vec', 'String', 'Option', 'Result', 'Box'];
        $dir = dirname(__DIR__, 4) . '/rust/php-rt/src';
        if (is_dir($dir)) {
            $it = new \RecursiveIteratorIterator(new \RecursiveDirectoryIterator($dir));
            foreach ($it as $file) {
                if (substr((string) $file, -3) !== '.rs') {
                    continue;
                }
                $src = (string) file_get_contents((string) $file);
                if (preg_match_all('/^\s*pub fn ([a-z_][a-z0-9_]*)/m', $src, $m)) {
                    foreach ($m[1] as $fn) {
                        $names[] = $fn;
                    }
                }
                if (preg_match_all('/macro_rules! ([a-z_]+)/', $src, $m)) {
                    foreach ($m[1] as $fn) {
                        $names[] = $fn;
                    }
                }
            }
        }
        self::$reserved = array_fill_keys($names, true);
        return self::$reserved;
    }

    /** Rust local variable name for a PHP variable name (without `$`). */
    public static function var(string $name): string
    {
        $id = self::ident($name);
        if (isset(self::reserved()[$id])) {
            return $id . '_v';
        }
        return $id;
    }

    public static function field(string|int $name): string
    {
        $name = (string) $name;
        if ($name === 'new') {
            return 'new_';
        }
        return self::ident($name);
    }

    public static function method(string $name): string
    {
        $lc = strtolower($name);
        if (in_array($lc, self::RESERVED_METHODS, true)) {
            return 'm_' . $name;
        }
        if (str_starts_with($name, '__')) {
            return 'magic' . $name;
        }
        return self::ident($name);
    }

    /** `Foo\Bar\Baz` => `Foo_Bar_Baz` */
    public static function classMangle(string $fqcn): string
    {
        return str_replace('\\', '_', ltrim($fqcn, '\\'));
    }

    /** Last segment of a class name. */
    public static function classShort(string $fqcn): string
    {
        $parts = explode('\\', ltrim($fqcn, '\\'));
        return self::typeIdent(end($parts));
    }

    /** Escape a type-level identifier. */
    public static function typeIdent(string $name): string
    {
        $name = preg_replace('/[^A-Za-z0-9_]/', '_', $name) ?? $name;
        if (in_array($name, self::KEYWORDS, true) || in_array($name, ['Str', 'List', 'Map', 'Option', 'Result', 'Vec', 'Box', 'Rc', 'Mixed', 'Late', 'Throw', 'Never', 'ArrayKey'], true)) {
            return $name . '_';
        }
        return $name;
    }

    /** Fully qualified Rust path of a class handle type. */
    public static function classPath(string $fqcn): string
    {
        return 'crate::' . self::modulePath($fqcn) . '::' . self::classShort($fqcn);
    }

    /** Module path (without crate prefix) for a class' namespace. */
    public static function modulePath(string $fqcn): string
    {
        $parts = explode('\\', ltrim($fqcn, '\\'));
        array_pop($parts);
        if (!$parts) {
            return 'g';
        }
        return implode('::', array_map([self::class, 'moduleSegment'], $parts));
    }

    public static function moduleSegment(string $ns): string
    {
        $s = strtolower(preg_replace('/(?<=[a-z0-9])(?=[A-Z])/', '_', $ns) ?? $ns);
        $s = preg_replace('/[^a-z0-9_]/', '_', $s) ?? $s;
        if (in_array($s, self::KEYWORDS, true)) {
            $s .= '_';
        }
        return $s;
    }

    /** Module path segments as a list. */
    public static function moduleSegments(string $fqcn): array
    {
        return explode('::', self::modulePath($fqcn));
    }

    public static function function(string $fq_name): string
    {
        $parts = explode('\\', ltrim($fq_name, '\\'));
        $name = end($parts);
        return self::ident($name);
    }

    /** Rust path for a namespaced function. */
    public static function functionPath(string $fq_name): string
    {
        return 'crate::' . self::modulePath($fq_name) . '::' . self::function($fq_name);
    }

    public static function constant(string $name): string
    {
        return self::ident($name);
    }

    /** Emit a Rust byte-string / Str literal for arbitrary PHP bytes. */
    public static function strLit(string|int $s): string
    {
        $s = (string) $s;
        $lit = self::rustStringLiteral($s);
        if ($lit[0] === 'b') {
            return 'Str::from_static_bytes(' . $lit . ')';
        }
        return 'Str::from_static(' . $lit . ')';
    }

    public static function rustStringLiteral(string|int $s): string
    {
        $s = (string) $s;
        if (preg_match('//u', $s) && !str_contains($s, "\0")) {
            $out = '';
            $len = strlen($s);
            for ($i = 0; $i < $len; $i++) {
                $c = $s[$i];
                $out .= match ($c) {
                    '\\' => '\\\\',
                    '"' => '\\"',
                    "\n" => '\\n',
                    "\r" => '\\r',
                    "\t" => '\\t',
                    default => ord($c) < 32 || ord($c) === 127 ? sprintf('\\u{%x}', ord($c)) : $c,
                };
            }
            return '"' . $out . '"';
        }
        $out = '';
        $len = strlen($s);
        for ($i = 0; $i < $len; $i++) {
            $o = ord($s[$i]);
            $out .= $o >= 32 && $o < 127 && $s[$i] !== '\\' && $s[$i] !== '"' ? $s[$i] : sprintf('\\x%02x', $o);
        }
        return 'b"' . $out . '"';
    }

    public static function bytesLiteral(string $s): string
    {
        $lit = self::rustStringLiteral($s);
        if ($lit[0] === 'b') {
            return $lit;
        }
        return $lit . '.as_bytes()';
    }

    /** A `b"..."` literal usable in patterns. */
    public static function byteStrLiteral(string $s): string
    {
        $out = '';
        $len = strlen($s);
        for ($i = 0; $i < $len; $i++) {
            $o = ord($s[$i]);
            $out .= $o >= 32 && $o < 127 && $s[$i] !== '\\' && $s[$i] !== '"' ? $s[$i] : sprintf('\\x%02x', $o);
        }
        return 'b"' . $out . '"';
    }
}
