<?php

declare(strict_types=1);

namespace Spatie\ArrayToXml;

use function array_keys;
use function htmlspecialchars;
use function implode;
use function is_array;
use function is_bool;
use function is_int;
use function preg_replace;
use function str_repeat;
use function str_replace;

/**
 * Typed port of the array-to-XML conversion psalm's XML report uses (spatie/array-to-xml): string keys
 * become elements, sequential arrays repeat their element once per item, scalars become escaped text,
 * exactly as DOMDocument::saveXML() renders spatie's document (formatted with two-space indentation on
 * request). The special `_attributes`/`_cdata` keys are not supported: the report never uses them.
 *
 * @psalm-type XmlScalar = scalar|null
 * @psalm-type XmlValue = XmlScalar|list<XmlScalar|list<XmlScalar>|array<string, XmlScalar>>|array<string, XmlScalar|list<XmlScalar>|array<string, XmlScalar>>
 */
final class ArrayToXml
{
    /**
     * @param array<string, XmlValue> $array
     * @param array<string, bool> $domProperties `formatOutput` indents the document
     */
    public static function convert(
        array $array,
        string $rootElement = '',
        bool $replaceSpacesByUnderScoresInKeyNames = true,
        ?string $xmlEncoding = null,
        string $xmlVersion = '1.0',
        array $domProperties = [],
        ?bool $xmlStandalone = null,
        bool $addXmlDeclaration = true,
    ): string {
        $format = $domProperties['formatOutput'] ?? false;
        $out = '';
        if ($addXmlDeclaration) {
            $out .= '<?xml version="' . $xmlVersion . '"'
                . ($xmlEncoding !== null ? ' encoding="' . $xmlEncoding . '"' : '')
                . ($xmlStandalone !== null ? ' standalone="' . ($xmlStandalone ? 'yes' : 'no') . '"' : '')
                . '?>' . "\n";
        }
        $lines = self::lines($rootElement === '' ? 'root' : $rootElement, $array, 0, $format, $replaceSpacesByUnderScoresInKeyNames);
        return $out . implode($format ? "\n" : '', $lines) . "\n";
    }

    /**
     * The rendering of one key's value: one element, or one element per item of a sequential array.
     *
     * @param XmlValue $value
     * @return list<string>
     */
    private static function lines(string $name, $value, int $depth, bool $format, bool $replaceSpaces): array
    {
        $indent = $format ? str_repeat('  ', $depth) : '';
        if ($replaceSpaces) {
            $name = str_replace(' ', '_', $name);
        }
        if (!is_array($value)) {
            $text = self::text($value);
            return [$indent . ($text === '' ? '<' . $name . '/>' : '<' . $name . '>' . $text . '</' . $name . '>')];
        }
        if (self::allIntKeys($value)) {
            if ($value === []) {
                return [$indent . '<' . $name . '/>'];
            }
            $out = [];
            foreach ($value as $item) {
                foreach (self::lines($name, $item, $depth, $format, $replaceSpaces) as $line) {
                    $out[] = $line;
                }
            }
            return $out;
        }
        $children = [];
        foreach ($value as $key => $data) {
            foreach (self::lines((string) $key, $data, $depth + 1, $format, $replaceSpaces) as $line) {
                $children[] = $line;
            }
        }
        if ($children === []) {
            return [$indent . '<' . $name . '/>'];
        }
        if ($format) {
            return [$indent . '<' . $name . '>' . "\n" . implode("\n", $children) . "\n" . $indent . '</' . $name . '>'];
        }
        return [$indent . '<' . $name . '>' . implode('', $children) . '</' . $name . '>'];
    }

    /**
     * The escaped text of a scalar value (booleans as PHP casts them, null as nothing).
     *
     * @param XmlScalar $value
     */
    private static function text($value): string
    {
        if ($value === null) {
            return '';
        }
        if (is_bool($value)) {
            return $value ? '1' : '';
        }
        $escaped = htmlspecialchars((string) $value);
        return (string) preg_replace('/[\x00-\x08\x0B\x0C\x0E-\x1F]/', '', $escaped);
    }

    /** @param array<array-key, XmlValue> $value */
    private static function allIntKeys(array $value): bool
    {
        foreach (array_keys($value) as $key) {
            if (!is_int($key)) {
                return false;
            }
        }
        return true;
    }
}
