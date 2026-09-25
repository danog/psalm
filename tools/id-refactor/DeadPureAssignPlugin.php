<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use PhpParser\Node;
use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\Stmt;
use Psalm\Plugin\EventHandler\AfterFunctionLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterFunctionLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use SimpleXMLElement;

/**
 * Removes `$v = <pure call>;` statements whose variable the function never reads (dead name conversions left by
 * earlier id migrations: `$method_name = Interner::lookupLc($id);` in a body that only uses the id). Pure calls:
 * Interner::{lookup, lookupLc, lookupOrNull, intern, internOrNull} and strtolower / strcasecmp over such.
 */
final class DeadPureAssignPlugin implements PluginEntryPointInterface, AfterFunctionLikeAnalysisInterface
{
    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    private static function pure(Expr $e): bool
    {
        if ($e instanceof Expr\StaticCall && $e->class instanceof Name && strtolower($e->class->getLast()) === 'interner'
            && $e->name instanceof Identifier
            && in_array(strtolower($e->name->name), ['lookup', 'lookuplc', 'lookupornull', 'intern', 'internornull'], true)
            && !$e->isFirstClassCallable()
        ) {
            foreach ($e->getArgs() as $a) {
                if (!self::simple($a->value)) {
                    return false;
                }
            }
            return true;
        }
        if ($e instanceof Expr\FuncCall && $e->name instanceof Name && strtolower($e->name->toString()) === 'strtolower'
            && !$e->isFirstClassCallable() && count($e->getArgs()) === 1
        ) {
            return self::pure($e->getArgs()[0]->value) || self::simple($e->getArgs()[0]->value);
        }
        return false;
    }

    /** A side-effect-free operand: a variable, a property / constant fetch chain, a literal. */
    private static function simple(Expr $e): bool
    {
        return $e instanceof Expr\Variable || $e instanceof Node\Scalar
            || (($e instanceof Expr\PropertyFetch || $e instanceof Expr\NullsafePropertyFetch) && self::simple($e->var))
            || $e instanceof Expr\ClassConstFetch || $e instanceof Expr\ConstFetch
            || ($e instanceof Expr\ArrayDimFetch && self::simple($e->var) && ($e->dim === null || self::simple($e->dim)))
            || self::pure($e);
    }

    public static function afterStatementAnalysis(AfterFunctionLikeAnalysisEvent $event): ?bool
    {
        $file = $event->getStatementsSource()->getFilePath();
        require_once __DIR__ . '/MapIdSetPlugin.php';
        if (!MapIdSetPlugin::inScope($file)) {
            return null;
        }
        $stmt = $event->getStmt();
        if ($stmt instanceof Expr\ArrowFunction) {
            return null;
        }
        $src = (string) file_get_contents($file);
        $finder = new \PhpParser\NodeFinder();
        $body = $stmt->getStmts() ?? [];
        // every variable occurrence in the function (closures' `use` included: they read the variable)
        $reads = [];
        $dead = [];
        foreach ($finder->find($body, static fn(Node $n): bool => true) as $n) {
            if ($n instanceof Stmt\Expression && $n->expr instanceof Expr\Assign && $n->expr->var instanceof Expr\Variable
                && is_string($n->expr->var->name) && self::pure($n->expr->expr)
            ) {
                $dead[$n->expr->var->name][] = $n;
            }
        }
        foreach ($finder->find($body, static fn(Node $n): bool => $n instanceof Expr\Variable && is_string($n->name)) as $v) {
            $reads[$v->name] = ($reads[$v->name] ?? 0) + 1;
        }
        foreach ($finder->find($body, static fn(Node $n): bool => $n instanceof Expr\ClosureUse) as $u) {
            $reads[$u->var->name] = ($reads[$u->var->name] ?? 0) + 100;
        }
        // compact() / extract() / $$var: names are read dynamically
        foreach ($finder->find($body, static fn(Node $n): bool => ($n instanceof Expr\FuncCall && $n->name instanceof Name
            && in_array(strtolower($n->name->toString()), ['compact', 'extract', 'get_defined_vars'], true))
            || ($n instanceof Expr\Variable && !is_string($n->name))) as $_) {
            return null;
        }
        // by-reference parameters (outputs), static and global variables outlive the call: never dead
        foreach ($stmt->getParams() as $prm) {
            if ($prm->byRef && $prm->var instanceof Expr\Variable && is_string($prm->var->name)) {
                unset($dead[$prm->var->name]);
            }
        }
        foreach ($finder->find($body, static fn(Node $n): bool => $n instanceof Stmt\Static_ || $n instanceof Stmt\Global_) as $sg) {
            foreach ($sg->vars as $v) {
                $v = $v instanceof Node\StaticVar ? $v->var : $v;
                if ($v instanceof Expr\Variable && is_string($v->name)) {
                    unset($dead[$v->name]);
                }
            }
        }
        $edits = [];
        foreach ($dead as $name => $stmts) {
            // only the assignments themselves mention the variable (each assignment's target is one occurrence,
            // and variables inside the pure right-hand sides are counted as reads of their own names)
            if (($reads[$name] ?? 0) !== count($stmts) || $name === 'this') {
                continue;
            }
            foreach ($stmts as $st) {
                $s = strrpos(substr($src, 0, $st->getStartFilePos()), "\n") + 1;
                $e = strpos($src, "\n", $st->getEndFilePos()) + 1;
                if (trim(substr($src, $s, $st->getStartFilePos() - $s)) !== '' || trim(substr($src, $st->getEndFilePos() + 1, $e - $st->getEndFilePos() - 1)) !== '') {
                    continue; // not alone on its line(s)
                }
                if (($src[$e] ?? '') === "\n" && ($src[$s - 2] ?? '') === "\n") {
                    $e++; // with a blank line after it when one precedes it too
                }
                $edits[] = [$s, $e, ''];
            }
        }
        if ($edits !== []) {
            file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/dead-pure.jsonl', json_encode([
                'kind' => 'edit', 'file' => $file, 'site' => $file . ':' . $stmt->getStartFilePos(), 'edits' => $edits,
            ], JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
        }
        return null;
    }
}
