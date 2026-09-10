<?php

declare(strict_types=1);

namespace PhpParser\Node\Scalar;

class String_ extends \PhpParser\Node\Scalar
{
    public const KIND_SINGLE_QUOTED = 1;
    public const KIND_DOUBLE_QUOTED = 2;
    public const KIND_HEREDOC = 3;
    public const KIND_NOWDOC = 4;
    public string $value;
    protected static array $replacements = array (
  '\\' => '\\',
  '$' => '$',
  'n' => '
',
  'r' => '',
  't' => '	',
  'f' => '',
  'v' => '',
  'e' => '',
);
    public function __construct(string $value, array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
    public static function fromString(string $str, array $attributes = array (
), bool $parseUnicodeEscape = true): \PhpParser\Node\Scalar\String_
    {
        throw new \RuntimeException('vendor stub');
    }
    public static function parse(string $str, bool $parseUnicodeEscape = true): string
    {
        return '';
    }
    public static function parseEscapeSequences(string $str, ?string $quote, bool $parseUnicodeEscape = true): string
    {
        return '';
    }
    private static function codePointToUtf8(int $num): string
    {
        return '';
    }
    public function getType(): string
    {
        return '';
    }
}
