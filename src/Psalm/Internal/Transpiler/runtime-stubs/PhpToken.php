<?php

declare(strict_types=1);

/**
 * A PHP token as produced by the tokenizer (implemented natively in the Rust runtime).
 */
class PhpToken implements Stringable
{
    public int $id;

    public string $text;

    public int $line;

    public int $pos;

    public function __construct(int $id, string $text, int $line = -1, int $pos = -1)
    {
        $this->id = $id;
        $this->text = $text;
        $this->line = $line;
        $this->pos = $pos;
    }

    /**
     * @return list<static>
     */
    public static function tokenize(string $code, int $flags = 0): array
    {
        $tokens = [];
        foreach (__rt_tokenize($code) as [$id, $text, $line, $pos]) {
            $tokens[] = new static($id, $text, $line, $pos);
        }
        return $tokens;
    }

    /**
     * @param int|string|array<int|string> $kind
     */
    public function is(int|string|array $kind): bool
    {
        if (is_array($kind)) {
            foreach ($kind as $k) {
                if ($this->is($k)) {
                    return true;
                }
            }
            return false;
        }
        if (is_int($kind)) {
            return $this->id === $kind;
        }
        return $this->text === $kind;
    }

    public function isIgnorable(): bool
    {
        return $this->id === T_WHITESPACE || $this->id === T_COMMENT || $this->id === T_DOC_COMMENT || $this->id === T_OPEN_TAG;
    }

    public function getTokenName(): ?string
    {
        if ($this->id < 256) {
            return chr($this->id);
        }
        $name = token_name($this->id);
        return $name === 'UNKNOWN' ? null : $name;
    }

    public function __toString(): string
    {
        return $this->text;
    }
}
