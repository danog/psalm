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
 * MapIdSetPlugin's declarations: the configured properties' `@var` and methods' `@return` map types become
 * `array<int, true>`; the twins' declarations go.
 */
final class MapIdSetDeclPlugin implements PluginEntryPointInterface, AfterClassLikeAnalysisInterface
{
    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function afterStatementAnalysis(AfterClassLikeAnalysisEvent $event): ?bool
    {
        $file = $event->getStatementsSource()->getFilePath();
        if (!MapIdSetPlugin::inScope($file)) {
            return null;
        }
        $cls = Interner::lookup($event->getClasslikeStorage()->id);
        $src = (string) file_get_contents($file);
        $edits = [];
        $stmt = $event->getStmt();
        foreach (MapIdSetPlugin::cfg()['props'] as [$c, $p]) {
            if (strcasecmp($c, $cls) !== 0) {
                continue;
            }
            foreach ($stmt->getProperties() as $prop) {
                if ($prop->props[0]->name->name === $p && ($doc = $prop->getDocComment()) !== null) {
                    $r = self::tagType($doc, '@var');
                    if ($r !== null) {
                        $edits[] = [$r[0], $r[1], 'array<int, true>'];
                    }
                }
            }
        }
        foreach (MapIdSetPlugin::cfg()['twins'] as [$c, $twin]) {
            if (strcasecmp($c, $cls) !== 0) {
                continue;
            }
            foreach ($stmt->getProperties() as $prop) {
                if ($prop->props[0]->name->name === $twin) {
                    // the declaration with its docblock and the blank line before it
                    $s = $prop->getDocComment()?->getStartFilePos() ?? $prop->getStartFilePos();
                    $s = strrpos(substr($src, 0, $s), "\n") + 1;
                    $e = strpos($src, "\n", $prop->getEndFilePos()) + 1;
                    if (substr($src, $s - 2, 2) === "\n\n") {
                        $s--;
                    }
                    $edits[] = [$s, $e, ''];
                }
            }
        }
        foreach (MapIdSetPlugin::cfg()['methods'] as [$c, $m]) {
            if (strcasecmp($c, $cls) !== 0) {
                continue;
            }
            $method = $stmt->getMethod($m);
            if ($method !== null && ($doc = $method->getDocComment()) !== null) {
                $r = self::tagType($doc, '@return');
                if ($r !== null) {
                    $edits[] = [$r[0], $r[1], 'array<int, true>'];
                }
            }
        }
        if ($edits !== []) {
            MapIdSetPlugin::out(['kind' => 'edit', 'file' => $file, 'site' => $file . ':decl', 'edits' => $edits]);
        }
        return null;
    }

    /** @return ?array{int, int} */
    private static function tagType(\PhpParser\Comment\Doc $doc, string $tag): ?array
    {
        if (!preg_match('/' . preg_quote($tag, '/') . '\s+(array<[^>]*>)/', $doc->getText(), $m, PREG_OFFSET_CAPTURE)) {
            return null;
        }
        return [$doc->getStartFilePos() + $m[1][1], $doc->getStartFilePos() + $m[1][1] + strlen($m[1][0])];
    }
}
