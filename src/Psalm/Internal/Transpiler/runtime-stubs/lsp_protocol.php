<?php

declare(strict_types=1);

/**
 * Typed versions of the LSP protocol data objects the compiled program constructs (completion items and
 * signature help, built by Psalm\Codebase; the language server itself is not part of the program). The
 * vendored felixfbecker/language-server-protocol classes are untyped and JSON-oriented (`mixed` data).
 */

namespace LanguageServerProtocol;

final class Command
{
    public ?string $title;

    public ?string $command;

    /** @var list<string>|null */
    public ?array $arguments;

    /** @param list<string>|null $arguments */
    public function __construct(?string $title = null, ?string $command = null, ?array $arguments = null)
    {
        $this->title = $title;
        $this->command = $command;
        $this->arguments = $arguments;
    }
}

final class Position
{
    public ?int $line;

    public ?int $character;

    public function __construct(?int $line = null, ?int $character = null)
    {
        $this->line = $line;
        $this->character = $character;
    }

    public function compare(Position $position): int
    {
        if ($this->line === $position->line && $this->character === $position->character) {
            return 0;
        }

        if ($this->line !== $position->line) {
            return (int) $this->line - (int) $position->line;
        }

        return (int) $this->character - (int) $position->character;
    }

    public function toOffset(string $content): int
    {
        $lines = explode("\n", $content);
        $slice = array_slice($lines, 0, (int) $this->line);
        $offset = 0;
        foreach ($slice as $line) {
            $offset += strlen($line);
        }

        return $offset + count($slice) + (int) $this->character;
    }
}

final class Range
{
    public ?Position $start;

    public ?Position $end;

    public function __construct(?Position $start = null, ?Position $end = null)
    {
        $this->start = $start;
        $this->end = $end;
    }

    public function includes(Position $position): bool
    {
        return $this->start !== null && $this->end !== null
            && $this->start->compare($position) <= 0 && $this->end->compare($position) >= 0;
    }
}

final class TextEdit
{
    public ?Range $range;

    public ?string $newText;

    public function __construct(?Range $range = null, ?string $newText = null)
    {
        $this->range = $range;
        $this->newText = $newText;
    }
}

abstract class MarkupKind
{
    public const PLAINTEXT = 'plaintext';
    public const MARKDOWN = 'markdown';
}

class MarkupContent
{
    public ?string $kind;

    public ?string $value;

    public function __construct(?string $kind = null, ?string $value = null)
    {
        $this->kind = $kind;
        $this->value = $value;
    }
}

abstract class InsertTextFormat
{
    public const PLAIN_TEXT = 1;
    public const SNIPPET = 2;
}

abstract class CompletionItemKind
{
    public const TEXT = 1;
    public const METHOD = 2;
    public const FUNCTION = 3;
    public const CONSTRUCTOR = 4;
    public const FIELD = 5;
    public const VARIABLE = 6;
    public const CLASS_ = 7;
    public const INTERFACE = 8;
    public const MODULE = 9;
    public const PROPERTY = 10;
    public const UNIT = 11;
    public const VALUE = 12;
    public const ENUM = 13;
    public const KEYWORD = 14;
    public const SNIPPET = 15;
    public const COLOR = 16;
    public const FILE = 17;
    public const REFERENCE = 18;
}

final class CompletionItem
{
    public ?string $label;

    public ?int $kind;

    public ?string $detail;

    public ?string $documentation;

    public ?string $sortText;

    public ?string $filterText;

    public ?string $insertText;

    public ?int $insertTextFormat;

    public ?TextEdit $textEdit;

    /** @var list<TextEdit>|null */
    public ?array $additionalTextEdits;

    public ?Command $command;

    public ?string $data;

    /** @param list<TextEdit>|null $additionalTextEdits */
    public function __construct(
        ?string $label = null,
        ?int $kind = null,
        ?string $detail = null,
        ?string $documentation = null,
        ?string $sortText = null,
        ?string $filterText = null,
        ?string $insertText = null,
        ?TextEdit $textEdit = null,
        ?array $additionalTextEdits = null,
        ?Command $command = null,
        ?string $data = null,
        ?int $insertTextFormat = null,
    ) {
        $this->label = $label;
        $this->kind = $kind;
        $this->detail = $detail;
        $this->documentation = $documentation;
        $this->sortText = $sortText;
        $this->filterText = $filterText;
        $this->insertText = $insertText;
        $this->textEdit = $textEdit;
        $this->additionalTextEdits = $additionalTextEdits;
        $this->command = $command;
        $this->data = $data;
        $this->insertTextFormat = $insertTextFormat;
    }
}

final class ParameterInformation
{
    /** @var string|array{int, int} */
    public string|array $label;

    public ?string $documentation;

    /** @param string|array{int, int} $label */
    public function __construct(string|array $label, ?string $documentation = null)
    {
        $this->label = $label;
        $this->documentation = $documentation;
    }
}

final class SignatureInformation
{
    public string $label;

    public ?string $documentation;

    /** @var list<ParameterInformation>|null */
    public ?array $parameters;

    public ?int $activeParameter;

    /** @param list<ParameterInformation>|null $parameters */
    public function __construct(
        string $label,
        ?array $parameters = null,
        ?string $documentation = null,
        ?int $activeParameter = null,
    ) {
        $this->label = $label;
        $this->parameters = $parameters;
        $this->documentation = $documentation;
        $this->activeParameter = $activeParameter;
    }
}
