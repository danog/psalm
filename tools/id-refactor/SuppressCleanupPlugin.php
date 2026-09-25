<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use Psalm\Plugin\EventHandler\AfterFileAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterFileAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use SimpleXMLElement;

/**
 * Removes the @psalm-suppress entries Psalm reports as UnusedPsalmSuppress (env UNUSED_SUPPRESS: JSON list of
 * [file, line, column]): the issue name at that position goes; a tag left empty goes with its line, and a
 * docblock left with no content goes entirely.
 */
final class SuppressCleanupPlugin implements PluginEntryPointInterface, AfterFileAnalysisInterface
{
    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function afterAnalyzeFile(AfterFileAnalysisEvent $event): void
    {
        $file = $event->getStatementsSource()->getFilePath();
        $todo = array_values(array_filter(json_decode((string) getenv('UNUSED_SUPPRESS'), true) ?: [],
            static fn(array $t): bool => $t[0] === $file));
        if ($todo === []) {
            return;
        }
        $src = (string) file_get_contents($file);
        $lines = explode("\n", $src);
        $offsets = [];
        $o = 0;
        foreach ($lines as $i => $l) {
            $offsets[$i + 1] = $o;
            $o += strlen($l) + 1;
        }
        $edits = [];
        foreach ($todo as [, $line, $col]) {
            $text = $lines[$line - 1];
            if (!preg_match('/@psalm-suppress\s+([A-Za-z0-9_, ]+)/', $text, $m, PREG_OFFSET_CAPTURE)) {
                continue;
            }
            $names = array_values(array_filter(array_map('trim', explode(',', $m[1][0]))));
            // the issue name starting at the reported column
            $at = $col - 1;
            $victim = null;
            foreach ($names as $n) {
                $p = strpos($text, $n, $m[1][1]);
                if ($p !== false && $p <= $at && $at < $p + strlen($n)) {
                    $victim = $n;
                }
            }
            if ($victim === null) {
                continue;
            }
            $rest = array_values(array_filter($names, static fn(string $n): bool => $n !== $victim));
            $line_start = $offsets[$line];
            if ($rest !== []) {
                $s = $line_start + $m[1][1];
                $edits[] = [$s, $s + strlen(rtrim($m[1][0])), implode(', ', $rest)];
                continue;
            }
            // the whole tag goes; a single-line docblock with nothing else goes too
            if (preg_match('#^\s*/\*\*\s*@psalm-suppress[^*]*\*/\s*$#', $text)) {
                $edits[] = [$line_start, $line_start + strlen($text) + 1, ''];
            } elseif (preg_match('#^\s*\*\s*@psalm-suppress#', $text)) {
                $edits[] = [$line_start, $line_start + strlen($text) + 1, ''];
            } else {
                $s = $line_start + $m[0][1];
                $edits[] = [$s, $s + strlen($m[0][0]), ''];
            }
        }
        if ($edits !== []) {
            file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/suppress.jsonl', json_encode(
                ['kind' => 'edit', 'file' => $file, 'site' => $file . ':suppress', 'edits' => $edits], JSON_UNESCAPED_SLASHES) . "\n",
                FILE_APPEND | LOCK_EX);
        }
    }
}
