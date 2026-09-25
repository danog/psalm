<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use PhpParser\Node;
use Psalm\Internal\Interner;
use Psalm\Plugin\EventHandler\AfterClassLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use SimpleXMLElement;

/**
 * Replaces expressions (or statements) of one method by their exact source text: env EXPR_REPLACE = JSON list of
 * [class, method, old source text, new source text, expected count]. Every AST node of the method whose source
 * text is exactly the old text is replaced (the outermost when nested); a count other than the expected one is
 * reported as `manual` and nothing is edited for that entry.
 */
final class ExprReplacePlugin implements PluginEntryPointInterface, AfterClassLikeAnalysisInterface
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
        $out = static function (array $row): void {
            file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/expr-replace.jsonl',
                json_encode($row, JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
        };
        foreach (json_decode((string) getenv('EXPR_REPLACE'), true) ?: [] as [$c, $m, $old, $new, $expected]) {
            if (strcasecmp($c, $cls) !== 0 || ($method = $event->getStmt()->getMethod($m)) === null) {
                continue;
            }
            $hits = [];
            foreach ((new \PhpParser\NodeFinder())->find($method->stmts ?? [], static fn(Node $n): bool => true) as $n) {
                $s = $n->getStartFilePos();
                $e = $n->getEndFilePos() + 1;
                if ($s >= 0 && substr($src, $s, $e - $s) === $old) {
                    $hits[$s . ':' . $e] = [$s, $e];
                }
            }
            // outermost only
            $hits = array_values(array_filter($hits, static function (array $h) use ($hits): bool {
                foreach ($hits as $o) {
                    if ($o !== $h && $o[0] <= $h[0] && $o[1] >= $h[1]) {
                        return false;
                    }
                }
                return true;
            }));
            if (count($hits) !== $expected) {
                $out(['kind' => 'manual', 'site' => "$c::$m", 'why' => count($hits) . " hits (expected $expected) for: $old"]);
                continue;
            }
            $out(['kind' => 'edit', 'file' => $file, 'site' => "$file:$c::$m:" . md5($old),
                'edits' => array_map(static fn(array $h): array => [$h[0], $h[1], $new], $hits)]);
        }
        return null;
    }
}
