<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use Psalm\Internal\Interner;
use Psalm\Plugin\EventHandler\AfterClassLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use SimpleXMLElement;

/** MapValueIdPlugin's declarations: `@var array<int, string>` -> `array<int, int>` (member lists: `list<int>`). */
final class MapValueIdDeclPlugin implements PluginEntryPointInterface, AfterClassLikeAnalysisInterface
{
    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function afterStatementAnalysis(AfterClassLikeAnalysisEvent $event): ?bool
    {
        $file = $event->getStatementsSource()->getFilePath();
        $cls = Interner::lookup($event->getClasslikeStorage()->id);
        $edits = [];
        foreach (MapValueIdPlugin::cfg() as [$c, $p, $kind]) {
            if (strcasecmp($c, $cls) !== 0) {
                continue;
            }
            foreach ($event->getStmt()->getProperties() as $prop) {
                $doc = $prop->getDocComment();
                if ($prop->props[0]->name->name !== $p || $doc === null
                    || !preg_match('/@var\s+(array<[^\n]*>)/', $doc->getText(), $m, PREG_OFFSET_CAPTURE)
                ) {
                    continue;
                }
                $s = $doc->getStartFilePos() + $m[1][1];
                if ($kind === 'inner_name' || $kind === 'inner_member') {
                    // the inner key type: array<int, array<string, V>> -> array<int, array<int, V>>
                    $edits[] = [$s, $s + strlen($m[1][0]), preg_replace('/^array<int, array<(?:lowercase-)?string, /', 'array<int, array<int, ', $m[1][0])];
                    continue;
                }
                $edits[] = [$s, $s + strlen($m[1][0]), $kind === 'member_list' ? 'array<int, list<int>>' : 'array<int, int>'];
            }
        }
        // source methods returning an inner map: @return array<string, V> -> array<int, V>
        foreach (json_decode((string) getenv('MAP_VALUE_SOURCES'), true) ?: [] as [$c, $m]) {
            if (strcasecmp($c, $cls) !== 0 || ($method = $event->getStmt()->getMethod($m)) === null || ($doc = $method->getDocComment()) === null) {
                continue;
            }
            if (preg_match('/@return\s+(array<string, )/', $doc->getText(), $mm, PREG_OFFSET_CAPTURE)) {
                $s = $doc->getStartFilePos() + $mm[1][1];
                $edits[] = [$s, $s + strlen($mm[1][0]), 'array<int, '];
            }
        }
        if ($edits !== []) {
            MapValueIdPlugin::out(['kind' => 'edit', 'file' => $file, 'site' => $file . ':vdecl', 'edits' => $edits]);
        }
        return null;
    }
}
