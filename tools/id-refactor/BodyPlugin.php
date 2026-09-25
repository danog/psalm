<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use Psalm\Internal\Interner;
use Psalm\Plugin\EventHandler\AfterClassLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use SimpleXMLElement;

/**
 * Replaces method bodies with a decided form (env BODY_REWRITES = JSON [[class, method, body source]]): the step
 * before a seeded id migration when a body writes the parameter being migrated (pzoom's form of the method,
 * still over the old parameter type, so the migration then only changes types and call sites).
 */
final class BodyPlugin implements PluginEntryPointInterface, AfterClassLikeAnalysisInterface
{
    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function afterStatementAnalysis(AfterClassLikeAnalysisEvent $event): ?bool
    {
        $cls = Interner::lookup($event->getClasslikeStorage()->id);
        $file = $event->getStatementsSource()->getFilePath();
        $src = (string) file_get_contents($file);
        foreach (json_decode((string) getenv('BODY_REWRITES'), true) ?: [] as [$c, $m, $body]) {
            if (strcasecmp($c, $cls) !== 0) {
                continue;
            }
            $method = $event->getStmt()->getMethod($m);
            if ($method === null || $method->stmts === null) {
                continue;
            }
            // between the braces: indent the body one level deeper than the method
            $open = strpos($src, '{', $method->name->getEndFilePos());
            $close = $method->getEndFilePos();
            $line_start = strrpos(substr($src, 0, $method->getStartFilePos()), "\n") + 1;
            $indent = str_repeat(' ', strspn($src, ' ', $line_start) + 4);
            $text = "\n" . implode("\n", array_map(
                static fn(string $l): string => $l === '' ? '' : $indent . $l,
                explode("\n", trim($body)),
            )) . "\n" . substr($indent, 4);
            file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/body.jsonl', json_encode([
                'kind' => 'edit', 'file' => $file, 'site' => $file . ':body:' . $m,
                'edits' => [[$open + 1, $close, $text]],
            ], JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
        }
        return null;
    }
}
