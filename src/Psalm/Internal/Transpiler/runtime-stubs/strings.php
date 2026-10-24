<?php

/**
 * PHP implementations of string builtins the runtime has no native counterpart for (transpiled with the program).
 */

/**
 * `parse_str()`: a query string into variables-as-array, with PHP's bracket syntax for nested and appended keys
 * (`a[b][]=1`). Keys and values are URL-decoded; a dot or space in a top-level key becomes an underscore.
 *
 * @param-out array<array-key, mixed> $result
 */
function parse_str(string $string, &$result): void
{
    $result = [];
    foreach (explode('&', $string) as $pair) {
        if ($pair === '') {
            continue;
        }
        $eq = strpos($pair, '=');
        $key = urldecode($eq === false ? $pair : substr($pair, 0, $eq));
        $value = $eq === false ? '' : urldecode(substr($pair, $eq + 1));
        $open = strpos($key, '[');
        $base = $open === false ? $key : substr($key, 0, $open);
        $base = str_replace(['.', ' '], '_', $base);
        if ($base === '') {
            continue;
        }
        if ($open === false || !str_ends_with($key, ']')) {
            $result[$base] = $value;
            continue;
        }
        $subkeys = explode('][', substr($key, $open + 1, -1));
        $result[$base] = __rt_parse_str_set($result[$base] ?? [], $subkeys, $value);
    }
}

/**
 * @param list<string> $subkeys
 * @return array<array-key, mixed>
 */
function __rt_parse_str_set(mixed $into, array $subkeys, string $value): array
{
    $into = is_array($into) ? $into : [];
    $k = array_shift($subkeys);
    if ($subkeys === []) {
        if ($k === '') {
            $into[] = $value;
        } else {
            $into[$k] = $value;
        }
        return $into;
    }
    if ($k === '') {
        $into[] = __rt_parse_str_set([], $subkeys, $value);
    } else {
        $into[$k] = __rt_parse_str_set($into[$k] ?? [], $subkeys, $value);
    }
    return $into;
}
