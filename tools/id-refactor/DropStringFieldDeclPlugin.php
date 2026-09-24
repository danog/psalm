<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use PhpParser\Node\Stmt;
use Psalm\Plugin\EventHandler\AfterClassLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use SimpleXMLElement;
use Throwable;

/** DropStringFieldPlugin's declaration half: removes the configured string property declarations (with docblocks). */
final class DropStringFieldDeclPlugin implements PluginEntryPointInterface, AfterClassLikeAnalysisInterface
{
    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function afterStatementAnalysis(AfterClassLikeAnalysisEvent $event): ?bool
    {
        $stmt = $event->getStmt();
        $storage = $event->getClasslikeStorage();
        $file = $event->getStatementsSource()->getFilePath();
        try {
            foreach (json_decode((string) getenv('DROP_FIELDS'), true) ?: [] as [$class, $prop]) {
                if (strcasecmp($storage->name, $class) !== 0) {
                    continue;
                }
                foreach ($stmt->getProperties() as $p) {
                    if (count($p->props) === 1 && $p->props[0]->name->name === $prop) {
                        $doc = $p->getDocComment();
                        $start = $doc !== null ? $doc->getStartFilePos() : $p->getStartFilePos();
                        $src = (string) file_get_contents($file);
                        // the whole lines, and one blank line after
                        $ls = strrpos(substr($src, 0, $start), "\n") + 1;
                        $le = strpos($src, "\n", $p->getEndFilePos()) + 1;
                        if (substr($src, $le, 1) === "\n") {
                            $le++;
                        }
                        file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/drop-field.jsonl',
                            json_encode(['kind' => 'edit', 'file' => $file, 'site' => $file . ':decl:' . $prop,
                                'edits' => [[$ls, $le, '']]], JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
                    }
                }
            }
        } catch (Throwable $e) {
            file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/drop-field.jsonl',
                json_encode(['kind' => 'error', 'msg' => 'decl: ' . $e->getMessage()]) . "\n", FILE_APPEND | LOCK_EX);
        }
        return null;
    }
}
