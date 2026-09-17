<?php

/** Typed stand-ins for CLI-only dependencies: the port runs single-threaded. */

namespace Fidry\CpuCoreCounter;

final class CpuCoreCounter
{
    public function getCount(): int
    {
        return 1;
    }

    public function getAvailableForParallelisation(): int
    {
        return 1;
    }
}

namespace SebastianBergmann\Diff\Output;

/** Options of the unified diff output (`fromFile`/`toFile` headers). */
final class StrictUnifiedDiffOutputBuilder
{
    public string $from_file;

    public string $to_file;

    /** @param array{fromFile?: string, toFile?: string} $options */
    public function __construct(array $options = [])
    {
        $this->from_file = $options['fromFile'] ?? '';
        $this->to_file = $options['toFile'] ?? '';
    }
}

namespace SebastianBergmann\Diff;

use SebastianBergmann\Diff\Output\StrictUnifiedDiffOutputBuilder;

/**
 * Line diff of the dry-run output (`--alter --dry-run`): a plain unified-style listing of the removed and
 * added lines (no longest-common-subsequence minimisation; every differing region is listed whole).
 */
final class Differ
{
    public function __construct(private readonly StrictUnifiedDiffOutputBuilder $output_builder)
    {
    }

    public function diff(string $from, string $to): string
    {
        if ($from === $to) {
            return '';
        }
        $from_lines = explode("\n", $from);
        $to_lines = explode("\n", $to);
        $n = count($from_lines);
        $m = count($to_lines);
        $prefix = 0;
        while ($prefix < $n && $prefix < $m && $from_lines[$prefix] === $to_lines[$prefix]) {
            $prefix++;
        }
        $suffix = 0;
        while ($suffix < $n - $prefix && $suffix < $m - $prefix
            && $from_lines[$n - 1 - $suffix] === $to_lines[$m - 1 - $suffix]
        ) {
            $suffix++;
        }
        $out = '--- ' . $this->output_builder->from_file . "\n+++ " . $this->output_builder->to_file . "\n";
        $out .= '@@ -' . ($prefix + 1) . ',' . ($n - $prefix - $suffix) . ' +' . ($prefix + 1) . ',' . ($m - $prefix - $suffix) . " @@\n";
        for ($i = $prefix; $i < $n - $suffix; $i++) {
            $out .= '-' . $from_lines[$i] . "\n";
        }
        for ($i = $prefix; $i < $m - $suffix; $i++) {
            $out .= '+' . $to_lines[$i] . "\n";
        }
        return $out;
    }
}

namespace Symfony\Component\Console\Output;

/** Collects the text a console helper writes. */
final class BufferedOutput
{
    private string $buffer = '';

    public function write(string $text): void
    {
        $this->buffer .= $text;
    }

    public function writeln(string $text): void
    {
        $this->buffer .= $text . "\n";
    }

    public function fetch(): string
    {
        $content = $this->buffer;
        $this->buffer = '';
        return $content;
    }
}

namespace Symfony\Component\Console\Helper;

use Symfony\Component\Console\Output\BufferedOutput;

/** A plain-text table renderer (the box-drawing layout of the console component, ASCII borders). */
final class Table
{
    /** @var list<string> */
    private array $headers = [];

    /** @var list<list<string>> */
    private array $rows = [];

    public function __construct(private readonly BufferedOutput $output)
    {
    }

    /** @param list<string> $headers */
    public function setHeaders(array $headers): void
    {
        $this->headers = $headers;
    }

    /** @param list<string|int> $row */
    public function addRow(array $row): void
    {
        $cells = [];
        foreach ($row as $cell) {
            $cells[] = (string) $cell;
        }
        $this->rows[] = $cells;
    }

    public function render(): void
    {
        $widths = [];
        foreach ([$this->headers, ...$this->rows] as $row) {
            foreach ($row as $i => $cell) {
                foreach (explode("\n", $cell) as $line) {
                    $widths[$i] = max($widths[$i] ?? 0, strlen($line));
                }
            }
        }
        $separator = '+';
        foreach ($widths as $w) {
            $separator .= str_repeat('-', $w + 2) . '+';
        }
        $this->output->writeln($separator);
        if ($this->headers !== []) {
            $this->writeRow($this->headers, $widths);
            $this->output->writeln($separator);
        }
        foreach ($this->rows as $row) {
            $this->writeRow($row, $widths);
        }
        $this->output->writeln($separator);
    }

    /**
     * @param list<string> $row
     * @param array<int, int> $widths
     */
    private function writeRow(array $row, array $widths): void
    {
        $height = 1;
        $cells = [];
        foreach ($row as $i => $cell) {
            $lines = explode("\n", $cell);
            $cells[$i] = $lines;
            $height = max($height, count($lines));
        }
        for ($l = 0; $l < $height; $l++) {
            $line = '|';
            foreach ($widths as $i => $w) {
                $text = $cells[$i][$l] ?? '';
                $line .= ' ' . str_pad($text, $w) . ' |';
            }
            $this->output->writeln($line);
        }
    }
}

