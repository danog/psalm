<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use function str_repeat;

/**
 * Indented source builder.
 *
 * @internal
 */
final class Writer
{
    private string $out = '';
    private int $indent = 0;

    public function line(string $s = ''): void
    {
        $this->out .= ($s === '' ? '' : str_repeat('    ', $this->indent) . $s) . "\n";
    }

    public function raw(string $s): void
    {
        $this->out .= $s;
    }

    public function open(string $s): void
    {
        $this->line($s);
        $this->indent++;
    }

    public function close(string $s = '}'): void
    {
        $this->indent--;
        $this->line($s);
    }

    public function indent(): void
    {
        $this->indent++;
    }

    public function dedent(): void
    {
        $this->indent--;
    }

    public function get(): string
    {
        return $this->out;
    }

    public function isEmpty(): bool
    {
        return $this->out === '';
    }
}
