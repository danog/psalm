<?php

declare(strict_types=1);

namespace Psalm\Internal\Json;

use RuntimeException;

use function bin2hex;
use function is_array;
use function is_string;
use function json_encode;
use function json_last_error_msg;
use function preg_replace_callback;

use const JSON_PRETTY_PRINT;
use const JSON_UNESCAPED_SLASHES;
use const JSON_UNESCAPED_UNICODE;

/**
 * Provides ability of pretty printed JSON output.
 *
 * @internal
 *
 * @psalm-type JsonScalar = scalar|null
 * @psalm-type JsonValue = JsonScalar|list<JsonScalar|list<JsonScalar>|array<string, JsonScalar>>|array<string, JsonScalar|list<JsonScalar>|array<string, JsonScalar>>
 */
final class Json
{
    public const PRETTY = JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE;
    // from https://stackoverflow.com/a/11709412
    private const INVALID_UTF_REGEXP = <<<'EOF'
    /(
        [\xC0-\xC1] # Invalid UTF-8 Bytes
        | [\xF5-\xFF] # Invalid UTF-8 Bytes
        | \xE0[\x80-\x9F] # Overlong encoding of prior code point
        | \xF0[\x80-\x8F] # Overlong encoding of prior code point
        | [\xC2-\xDF](?![\x80-\xBF]) # Invalid UTF-8 Sequence Start
        | [\xE0-\xEF](?![\x80-\xBF]{2}) # Invalid UTF-8 Sequence Start
        | [\xF0-\xF4](?![\x80-\xBF]{3}) # Invalid UTF-8 Sequence Start
        | (?<=[\x00-\x7F\xF5-\xFF])[\x80-\xBF] # Invalid UTF-8 Sequence Middle
        | (?<!
            [\xC2-\xDF]
            |[\xE0-\xEF]
            |[\xE0-\xEF][\x80-\xBF]
            |[\xF0-\xF4]
            |[\xF0-\xF4][\x80-\xBF]
            |[\xF0-\xF4][\x80-\xBF]{2}
        )[\x80-\xBF] # Overlong Sequence
        | (?<=[\xE0-\xEF])[\x80-\xBF](?![\x80-\xBF]) # Short 3 byte sequence
        | (?<=[\xF0-\xF4])[\x80-\xBF](?![\x80-\xBF]{2}) # Short 4 byte sequence
        | (?<=[\xF0-\xF4][\x80-\xBF])[\x80-\xBF](?![\x80-\xBF]) # Short 4 byte sequence (2)
    )/x
    EOF;

    /**
     * @var int
     */
    public const DEFAULT = 0;

    /**
     * @param list<JsonValue>|array<string, JsonValue> $data
     * @psalm-pure
     */
    public static function encode(array $data, ?int $options = null): string
    {
        if ($options === null) {
            $options = self::DEFAULT;
        }

        $result = json_encode($data, $options);

        if ($result == false) {
            $result = json_encode(self::scrub($data), $options);
        }

        if ($result === false) {
            /** @psalm-suppress ImpureFunctionCall */
            throw new RuntimeException('Cannot create JSON string: '.json_last_error_msg());
        }

        return $result;
    }

    /**
     * A scrubbed copy: the elements change as they are copied rather than through a by-reference
     * walk, whose callback parameter has no type a generated program can express.
     *
     * @param list<JsonValue>|array<string, JsonValue> $data
     * @return list<JsonValue>|array<string, JsonValue>
     * @psalm-pure
     */
    private static function scrub(array $data): array
    {
        $scrubbed = [];

        foreach ($data as $key => $value) {
            if (is_string($value)) {
                $scrubbed[$key] = self::scrubString($value);
            } elseif (is_array($value)) {
                $scrubbed[$key] = self::scrub($value);
            } else {
                $scrubbed[$key] = $value;
            }
        }

        return $scrubbed;
    }

    /**
     * The string with every byte that is not valid UTF-8 spelled out, so that it can be encoded.
     *
     * @psalm-pure
     */
    private static function scrubString(string $value): string
    {
        /** @psalm-suppress ImpureFunctionCall */
        return preg_replace_callback(
            self::INVALID_UTF_REGEXP,
            static fn(array $matches): string => '<Invalid UTF-8: 0x' . bin2hex((string) ($matches[0] ?? '')) . '>',
            $value,
        ) ?? $value;
    }
}
