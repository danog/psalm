<?php

declare(strict_types=1);

namespace Psalm\Tools\IdConvert;

use Psalm\Type\Atomic;
use Psalm\Type\Atomic\TArray;
use Psalm\Type\Atomic\TIterable;
use Psalm\Type\Atomic\TKeyedArray;
use Psalm\Type\Atomic\TNull;
use Psalm\Type\Atomic\TNumericString;
use Psalm\Type\Atomic\TString;
use Psalm\Type\Union;

/** Psalm types seen as string slots: which array paths of a type hold only strings. */
final class Types
{
    /** Whether every non-null atom is a string (a name candidate at this position). */
    public static function stringish(?Union $t): bool
    {
        if ($t === null) {
            return false;
        }
        $any = false;
        foreach ($t->getAtomicTypes() as $a) {
            if ($a instanceof TNull) {
                continue;
            }
            if (!$a instanceof TString || $a instanceof TNumericString) {
                return false;
            }
            $any = true;
        }
        return $any;
    }

    public static function nullable(?Union $t): bool
    {
        return $t !== null && $t->isNullable();
    }

    /** Whether a string type says it holds a class-like name. */
    public static function classy(?Union $t): bool
    {
        foreach ($t?->getAtomicTypes() ?? [] as $a) {
            if ($a instanceof Atomic\TClassString || $a instanceof Atomic\TLiteralClassString
                || $a instanceof Atomic\TDependentGetClass
            ) {
                return true;
            }
        }
        return false;
    }

    /**
     * The key / value types one step down an array path of a union (every array-like atom), or null when an atom
     * is not an array (the path does not exist uniformly).
     *
     * @param '#k'|'#v' $step
     */
    public static function step(?Union $t, string $step): ?Union
    {
        if ($t === null) {
            return null;
        }
        $parts = [];
        foreach ($t->getAtomicTypes() as $a) {
            if ($a instanceof TNull) {
                continue;
            }
            if ($a instanceof TKeyedArray) {
                if ($step === '#k') {
                    if ($a->is_list) {
                        return null;
                    }
                    // literal shape keys stay literals: a shape's keys are not a slot
                    if ($a->fallback_params === null) {
                        return null;
                    }
                    $parts[] = $a->fallback_params[0];
                } else {
                    foreach ($a->properties as $p) {
                        $parts[] = $p;
                    }
                    if ($a->fallback_params !== null) {
                        $parts[] = $a->fallback_params[1];
                    }
                }
            } elseif ($a instanceof TArray || $a instanceof TIterable) {
                $parts[] = $step === '#k' ? $a->type_params[0] : $a->type_params[1];
            } else {
                return null;
            }
        }
        if ($parts === []) {
            return null;
        }
        return \Psalm\Type::combineUnionTypeArray($parts, null);
    }

    /**
     * The string paths of a type: "" for a string, "#k" / "#v" / "#v#k" ... inside arrays.
     *
     * @return array<string, bool> path => nullable
     */
    public static function paths(?Union $t, string $prefix = '', int $depth = 0): array
    {
        $out = [];
        if ($t === null || $depth > 4) {
            return $out;
        }
        if (self::stringish($t)) {
            $out[$prefix] = self::nullable($t);
            return $out;
        }
        foreach (['#k', '#v'] as $s) {
            $sub = self::step($t, $s);
            if ($sub !== null) {
                $out += self::paths($sub, $prefix . $s, $depth + 1);
            }
        }
        return $out;
    }

    /** The type at a path, or null. */
    public static function at(?Union $t, string $path): ?Union
    {
        while ($path !== '' && $t !== null) {
            $t = self::step($t, substr($path, 0, 2));
            $path = substr($path, 2);
        }
        return $t;
    }
}
