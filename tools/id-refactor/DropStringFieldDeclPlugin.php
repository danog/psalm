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
            require_once __DIR__ . '/SymNames.php';
            $codebase = $event->getCodebase();
            $src = (string) file_get_contents($file);
            $emit = static function (array $row): void {
                file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/drop-field.jsonl',
                    json_encode($row, JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
            };
            foreach (json_decode((string) getenv('DROP_FIELDS'), true) ?: [] as [$class, $prop, $id]) {
                if (strcasecmp(\Psalm\Internal\Interner::lookup($storage->id), $class) !== 0) {
                    continue;
                }
                // does the class already have the id property (declared here or inherited)?
                $has_id = isset($storage->declaring_property_ids[\Psalm\Internal\Interner::intern($id)]);
                foreach ($stmt->getProperties() as $p) {
                    if (count($p->props) !== 1 || $p->props[0]->name->name !== $prop) {
                        continue;
                    }
                    if ($has_id) {
                        $doc = $p->getDocComment();
                        $start = $doc !== null ? $doc->getStartFilePos() : $p->getStartFilePos();
                        $ls = strrpos(substr($src, 0, $start), "\n") + 1;
                        $le = strpos($src, "\n", $p->getEndFilePos()) + 1;
                        if (substr($src, $le, 1) === "\n") {
                            $le++;
                        }
                        $emit(['kind' => 'edit', 'file' => $file, 'site' => $file . ':decl:' . $prop, 'edits' => [[$ls, $le, '']]]);
                        continue;
                    }
                    $edits = self::retype($p->type, $p->props[0]->name, $p->props[0]->default, $id, $file, $emit);
                    $emit(['kind' => 'edit', 'file' => $file, 'site' => $file . ':decl:' . $prop, 'edits' => $edits]);
                }
                $ctor = $stmt->getMethod('__construct');
                foreach ($ctor?->params ?? [] as $param) {
                    if ($param->flags === 0 || !$param->var instanceof \PhpParser\Node\Expr\Variable || $param->var->name !== $prop) {
                        continue;
                    }
                    if ($has_id) {
                        // the constructor derives the id from the string: `$this->id = Interner::intern($prop);`
                        // -> the promoted parameter is the id, the separate declaration and assignment go
                        $assign = null;
                        foreach ($ctor->stmts ?? [] as $st) {
                            if ($st instanceof Stmt\Expression && $st->expr instanceof \PhpParser\Node\Expr\Assign
                                && $st->expr->var instanceof \PhpParser\Node\Expr\PropertyFetch
                                && $st->expr->var->name instanceof \PhpParser\Node\Identifier && $st->expr->var->name->name === $id
                                && $st->expr->expr instanceof \PhpParser\Node\Expr\StaticCall
                                && strtolower($st->expr->expr->class->toString()) === 'interner'
                                && count($st->expr->expr->getArgs()) === 1
                                && $st->expr->expr->getArgs()[0]->value instanceof \PhpParser\Node\Expr\Variable
                                && $st->expr->expr->getArgs()[0]->value->name === $prop
                            ) {
                                $assign = $st;
                            }
                        }
                        $id_decl = null;
                        foreach ($stmt->getProperties() as $pp) {
                            if (count($pp->props) === 1 && $pp->props[0]->name->name === $id) {
                                $id_decl = $pp;
                            }
                        }
                        if ($assign === null || $id_decl === null) {
                            $emit(['kind' => 'manual', 'site' => $file . ':' . $param->getStartLine(), 'why' => 'promoted ' . $prop . ' next to ' . $id]);
                            continue;
                        }
                        $edits = self::retype($param->type, $param->var, $param->default, $id, $file, $emit);
                        foreach ([$id_decl, $assign] as $gone) {
                            $doc = $gone->getDocComment();
                            $start = $doc !== null ? $doc->getStartFilePos() : $gone->getStartFilePos();
                            $ls = strrpos(substr($src, 0, $start), "\n") + 1;
                            $le = strpos($src, "\n", $gone->getEndFilePos()) + 1;
                            if ($gone === $id_decl && substr($src, $le, 1) === "\n") {
                                $le++;
                            }
                            $edits[] = [$ls, $le, ''];
                        }
                        // the constructor's other reads of the string parameter
                        $finder = new \PhpParser\NodeFinder();
                        foreach ($finder->find($ctor->stmts ?? [], static fn(\PhpParser\Node $x): bool =>
                            $x instanceof \PhpParser\Node\Expr\Variable && $x->name === $prop) as $v) {
                            if ($v->getStartFilePos() >= $assign->getStartFilePos() && $v->getEndFilePos() <= $assign->getEndFilePos()) {
                                continue;
                            }
                            $edits[] = [$v->getStartFilePos(), $v->getEndFilePos() + 1, 'Interner::lookup($this->' . $id . ')'];
                        }
                        // its @param line
                        $cdoc = $ctor->getDocComment();
                        if ($cdoc !== null && preg_match('/@param\s+\S+\s+\$' . preg_quote($prop, '/') . '\b/', $cdoc->getText(), $m, PREG_OFFSET_CAPTURE)) {
                            $b = $cdoc->getStartFilePos() + $m[0][1];
                            $edits[] = [$b, $b + strlen($m[0][0]), '@param int $' . $id];
                        }
                        $emit(['kind' => 'edit', 'file' => $file, 'site' => $file . ':promoted:' . $prop, 'edits' => $edits]);
                        continue;
                    }
                    $edits = self::retype($param->type, $param->var, $param->default, $id, $file, $emit);
                    $emit(['kind' => 'edit', 'file' => $file, 'site' => $file . ':promoted:' . $prop, 'edits' => $edits]);
                }
            }
        } catch (Throwable $e) {
            file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/drop-field.jsonl',
                json_encode(['kind' => 'error', 'msg' => 'decl: ' . $e->getMessage()]) . "\n", FILE_APPEND | LOCK_EX);
        }
        return null;
    }

    /** @return list<array{int, int, string}> type -> int, name -> $id, a literal default -> its Sym constant */
    private static function retype(?\PhpParser\Node $type, \PhpParser\Node $name, ?\PhpParser\Node $default, string $id, string $file, callable $emit): array
    {
        $edits = [];
        if ($type !== null) {
            $edits[] = [$type->getStartFilePos(), $type->getEndFilePos() + 1, $type instanceof \PhpParser\Node\NullableType ? '?int' : 'int'];
        }
        $edits[] = [$name->getStartFilePos(), $name->getEndFilePos() + 1, '$' . $id];
        if ($default instanceof \PhpParser\Node\Scalar\String_) {
            [$text, $new] = SymNames::forLiteral($default->value);
            if ($new !== null) {
                $emit(['kind' => 'sym', 'name' => $new[0], 'value' => $new[1], 'literal' => $new[2]]);
            }
            $edits[] = [$default->getStartFilePos(), $default->getEndFilePos() + 1, $text];
        } elseif ($default instanceof \PhpParser\Node\Expr\ConstFetch && strtolower($default->name->toString()) === 'null') {
            // null stays null
        } elseif ($default !== null) {
            $emit(['kind' => 'manual', 'site' => $file . ':' . $default->getStartLine(), 'why' => 'non-literal default']);
        }
        return $edits;
    }
}
