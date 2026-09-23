<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use Psalm\Internal\Interner;
use Psalm\Storage\ClassConstantStorage;
use Psalm\Storage\ClassLikeStorage;
use Psalm\Storage\EnumCaseStorage;

use function preg_match;
use function sprintf;
use function str_contains;
use function str_replace;

/**
 * @internal
 * @psalm-immutable
 */
final class StorageByPatternResolver
{
    public const RESOLVE_CONSTANTS = 1;
    public const RESOLVE_ENUMS = 2;

    /**
     * @return array<string,ClassConstantStorage>
     * @psalm-mutation-free
     */
    public function resolveConstants(
        ClassLikeStorage $class_like_storage,
        string $pattern,
    ): array {
        $constants = $class_like_storage->constants;

        if (!str_contains($pattern, '*')) {
            if (isset($constants[Interner::intern($pattern)])) {
                return [$pattern => $constants[Interner::intern($pattern)]];
            }

            return [];
        } elseif ($pattern === '*') {
            return self::byName($constants);
        }

        $regex_pattern = sprintf('#^%s$#', str_replace('*', '.*?', $pattern));
        $matched_constants = [];

        foreach ($constants as $constant_id => $class_constant_storage) {
            $constant = Interner::lookup($constant_id);
            if (preg_match($regex_pattern, $constant) === 0) {
                continue;
            }

            $matched_constants[$constant] = $class_constant_storage;
        }

        return $matched_constants;
    }

    /**
     * @return array<string,EnumCaseStorage>
     * @psalm-mutation-free
     */
    public function resolveEnums(
        ClassLikeStorage $class_like_storage,
        string $pattern,
    ): array {
        $enum_cases = $class_like_storage->enum_cases;
        if (!str_contains($pattern, '*')) {
            if (isset($enum_cases[Interner::intern($pattern)])) {
                return [$pattern => $enum_cases[Interner::intern($pattern)]];
            }

            return [];
        } elseif ($pattern === '*') {
            return self::byName($enum_cases);
        }

        $regex_pattern = sprintf('#^%s$#', str_replace('*', '.*?', $pattern));
        $matched_enums = [];
        foreach ($enum_cases as $enum_case_name_id => $enum_case_storage) {
            $enum_case_name = Interner::lookup($enum_case_name_id);
            if (preg_match($regex_pattern, $enum_case_name) === 0) {
                continue;
            }

            $matched_enums[$enum_case_name] = $enum_case_storage;
        }

        return $matched_enums;
    }

    /**
     * The id-keyed member map keyed by member name (the resolved maps are keyed by name).
     *
     * @template T
     * @param array<int, T> $members
     * @return array<string, T>
     */
    private static function byName(array $members): array
    {
        $by_name = [];
        foreach ($members as $id => $member) {
            $by_name[Interner::lookup($id)] = $member;
        }
        return $by_name;
    }
}
