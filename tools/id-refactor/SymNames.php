<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use Psalm\Internal\Interner;
use Psalm\Internal\Sym;
use ReflectionClass;

/** A string literal's id as source text: an existing Sym constant, or a new one (recorded for apply.php). */
final class SymNames
{
    /** @var ?array<int, string> */
    private static ?array $by_value = null;

    /** @return array{string, ?array{string, int, string}} [text, new constant [name, value, literal] or null] */
    public static function forLiteral(string $literal): array
    {
        $literal = ltrim($literal, '\\');
        $value = Interner::hash($literal);
        if (self::$by_value === null) {
            self::$by_value = [];
            foreach ((new ReflectionClass(Sym::class))->getConstants() as $n => $v) {
                if (is_int($v)) {
                    self::$by_value[$v] = $n;
                }
            }
        }
        if (isset(self::$by_value[$value])) {
            return ['Sym::' . self::$by_value[$value], null];
        }
        $words = [];
        foreach (explode('\\', $literal) as $p) {
            $words[] = strtoupper((string) preg_replace('/(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])/', '_', $p));
        }
        $name = 'C_' . preg_replace('/[^A-Z0-9_]/', '_', implode('__', $words));
        return ['Sym::' . $name, [$name, $value, $literal]];
    }

    /** The class name a named-object atom stands for (before or after its string field was dropped). */
    public static function named(\Psalm\Type\Atomic\TNamedObject $a): string
    {
        /** @psalm-suppress UndefinedPropertyFetch, MixedReturnStatement */
        return property_exists($a, 'value') ? $a->value : Interner::lookup($a->name);
    }
}
