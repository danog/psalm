<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use Psalm\Plugin\EventHandler\AfterClassLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use SimpleXMLElement;

/**
 * Replaces a docblock tag of a method: env DOC_TAG_EDITS = JSON list of [class, method, old tag line text,
 * new tag line text] (the old text must occur in the method's docblock; an empty old text appends the new line).
 */
final class DocTagPlugin implements PluginEntryPointInterface, AfterClassLikeAnalysisInterface
{
    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function afterStatementAnalysis(AfterClassLikeAnalysisEvent $event): ?bool
    {
        $stmt = $event->getStmt();
        $name = $event->getClasslikeStorage()->name;
        $file = $event->getStatementsSource()->getFilePath();
        foreach (json_decode((string) getenv('DOC_TAG_EDITS'), true) ?: [] as [$class, $method, $old, $new]) {
            if (strcasecmp($class, $name) !== 0) {
                continue;
            }
            $m = $stmt->getMethod($method);
            $doc = $m?->getDocComment();
            if ($doc === null) {
                self::out(['kind' => 'manual', 'site' => "$class::$method", 'why' => 'no docblock']);
                continue;
            }
            $text = $doc->getText();
            $pos = $old === '' ? strrpos($text, '*/') : strpos($text, $old);
            if ($pos === false) {
                self::out(['kind' => 'manual', 'site' => "$class::$method", 'why' => "tag not found: $old"]);
                continue;
            }
            $base = $doc->getStartFilePos();
            $edit = $old === ''
                ? [$base + $pos, $base + $pos, '* ' . $new . "\n     "]
                : [$base + $pos, $base + $pos + strlen($old), $new];
            self::out(['kind' => 'edit', 'file' => $file, 'site' => "$class::$method:$old", 'edits' => [$edit]]);
        }
        return null;
    }

    /** @param array<string, mixed> $row */
    private static function out(array $row): void
    {
        file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/doctag.jsonl',
            json_encode($row, JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
    }
}
