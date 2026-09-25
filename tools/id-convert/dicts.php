<?php

/**
 * Class-like names in the dictionaries' types (CallMap, PropertyMap...) in their declared spelling: class names are
 * case-sensitive ids, so a type string spelling `libXMLError` or `domdocument` would name no class. Keys (the
 * lowercase lookup keys) stay as they are.
 *
 * php dicts.php ROOT
 */

declare(strict_types=1);

ini_set('memory_limit', '-1');

require __DIR__ . '/names.php';

$root = rtrim($argv[1], '/') . '/';
CanonicalNames::init($root);

$files = 0;
$names = 0;
foreach ([...glob($root . 'dictionaries/CallMap*.php') ?: [], ...glob($root . 'dictionaries/*PropertyMap.php') ?: []] as $file) {
    $tokens = token_get_all((string) file_get_contents($file));
    $out = '';
    $changed = false;
    foreach ($tokens as $k => $t) {
        if (!is_array($t) || $t[0] !== T_CONSTANT_ENCAPSED_STRING) {
            $out .= is_array($t) ? $t[1] : $t;
            continue;
        }
        // a key: the next significant token is `=>`
        for ($j = $k + 1; isset($tokens[$j]) && is_array($tokens[$j]) && $tokens[$j][0] === T_WHITESPACE; $j++);
        if (isset($tokens[$j]) && is_array($tokens[$j]) && $tokens[$j][0] === T_DOUBLE_ARROW) {
            $out .= $t[1];
            continue;
        }
        $quote = $t[1][0];
        $value = substr($t[1], 1, -1);
        $new = (string) preg_replace_callback(
            // a name, not a shape key (`name:` / `name?:`) nor a parameter name
            '/(?<![\w\\\\$-])\\\\?[A-Za-z_][A-Za-z0-9_]*(?:\\\\\\\\?[A-Za-z_][A-Za-z0-9_]*)*(?![\w-])(?!\??:(?!:))/',
            static function (array $m) use (&$names): string {
                $name = str_replace('\\\\', '\\', $m[0]);
                if (!CanonicalNames::isClass($name)) {
                    return $m[0];
                }
                $canonical = CanonicalNames::of($name);
                if ($canonical === ltrim($name, '\\')) {
                    return $m[0];
                }
                $names++;
                $lead = str_starts_with($name, '\\') ? '\\' : '';
                return $lead . ($m[0] !== $name ? str_replace('\\', '\\\\', $canonical) : $canonical);
            },
            $value,
        );
        if ($new !== $value) {
            $changed = true;
        }
        $out .= $quote . $new . $quote;
    }
    if ($changed) {
        file_put_contents($file, $out);
        $files++;
    }
}
echo "canonicalized $names class names in $files dictionaries\n";
