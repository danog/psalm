<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use PhpParser\Node;
use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Stmt;
use Psalm\Internal\Interner;
use Psalm\Plugin\EventHandler\AfterClassLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use SimpleXMLElement;

/**
 * Statement-level edits to a method body (env STMT_EDITS = JSON [[class, method, action, arg...]]):
 *   ["C", "m", "prepend", "<php statements>"]    insert statements at the start of the body
 *   ["C", "m", "drop-assign", "var", "fn"]        remove top-level `$var = fn(...);` statements (with their comments)
 *   ["C", "m", "drop-call", "name", n]           remove the n-th (1-based) top-level statement of the body that is
 *                                                a call of a method named `name`
 */
final class StmtEditPlugin implements PluginEntryPointInterface, AfterClassLikeAnalysisInterface
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
        $edits = [];
        foreach (json_decode((string) getenv('STMT_EDITS'), true) ?: [] as $e) {
            [$c, $m, $action] = $e;
            if (strcasecmp($c, $cls) !== 0) {
                continue;
            }
            $method = $event->getStmt()->getMethod($m);
            if ($method === null || $method->stmts === null || $method->stmts === []) {
                continue;
            }
            $first = $method->stmts[0];
            $line_start = static fn(int $pos): int => strrpos(substr($src, 0, $pos), "\n") + 1;
            $indent = str_repeat(' ', strspn($src, ' ', $line_start($first->getStartFilePos())));
            if ($action === 'prepend') {
                $at = $line_start($first->getComments() !== [] ? $first->getComments()[0]->getStartFilePos() : $first->getStartFilePos());
                $text = implode('', array_map(static fn(string $l): string => ($l === '' ? '' : $indent . $l) . "\n", explode("\n", trim($e[3])))) . "\n";
                $edits[] = [$at, $at, $text];
            } elseif ($action === 'drop-assign') {
                // top-level `$var = fn(...);` statements
                foreach ($method->stmts as $st) {
                    if ($st instanceof Stmt\Expression && $st->expr instanceof Expr\Assign && $st->expr->var instanceof Expr\Variable
                        && $st->expr->var->name === $e[3] && $st->expr->expr instanceof Expr\FuncCall
                        && $st->expr->expr->name instanceof Node\Name && strcasecmp($st->expr->expr->name->toString(), $e[4]) === 0
                    ) {
                        $s = $line_start($st->getComments() !== [] ? $st->getComments()[0]->getStartFilePos() : $st->getStartFilePos());
                        $end = strpos($src, "\n", $st->getEndFilePos()) + 1;
                        if (($src[$end] ?? '') === "\n") {
                            $end++;
                        }
                        $edits[] = [$s, $end, ''];
                    }
                }
            } elseif ($action === 'drop-call') {
                $n = 0;
                foreach ($method->stmts as $st) {
                    if ($st instanceof Stmt\Expression && ($st->expr instanceof Expr\MethodCall || $st->expr instanceof Expr\StaticCall)
                        && $st->expr->name instanceof Identifier && strcasecmp($st->expr->name->name, $e[3]) === 0
                        && ++$n === $e[4]
                    ) {
                        $s = $line_start($st->getStartFilePos());
                        $end = strpos($src, "\n", $st->getEndFilePos()) + 1;
                        // with the blank line after it
                        if (($src[$end] ?? '') === "\n") {
                            $end++;
                        }
                        $edits[] = [$s, $end, ''];
                    }
                }
            }
        }
        if ($edits !== []) {
            file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/stmt-edit.jsonl', json_encode([
                'kind' => 'edit', 'file' => $file, 'site' => $file . ':stmt', 'edits' => $edits,
            ], JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
        }
        return null;
    }
}
