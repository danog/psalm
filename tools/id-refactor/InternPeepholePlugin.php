<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use Psalm\Plugin\EventHandler\AfterExpressionAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterExpressionAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use SimpleXMLElement;

/**
 * Round trips through the interner left by earlier rewrites: Interner::intern(Interner::lookup(X)) and
 * Interner::intern(Interner::lookupLc(X)) are X (an id), Interner::lookup(Interner::intern(S)) is S (a string).
 * Paths: env ID_REFACTOR_ROOTS.
 */
final class InternPeepholePlugin implements PluginEntryPointInterface, AfterExpressionAnalysisInterface
{
    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    private static function internerCall(Expr $e, array $names): ?Expr
    {
        if ($e instanceof Expr\StaticCall && $e->class instanceof Name && $e->name instanceof Identifier
            && strtolower($e->class->getLast()) === 'interner' && in_array(strtolower($e->name->name), $names, true)
            && count($e->getArgs()) === 1 && !$e->getArgs()[0]->unpack
        ) {
            return $e->getArgs()[0]->value;
        }
        return null;
    }

    public static function afterExpressionAnalysis(AfterExpressionAnalysisEvent $event): ?bool
    {
        $e = $event->getExpr();
        $file = $event->getStatementsSource()->getFilePath();
        $roots = array_filter(explode(':', (string) getenv('ID_REFACTOR_ROOTS')));
        if (array_filter($roots, static fn(string $r): bool => str_starts_with($file, $r)) === []) {
            return null;
        }
        $inner = null;
        if (($arg = self::internerCall($e, ['intern'])) !== null) {
            $inner = self::internerCall($arg, ['lookup', 'lookuplc']);
        } elseif (($arg = self::internerCall($e, ['lookup', 'lookuplc'])) !== null) {
            $inner = self::internerCall($arg, ['intern']);
        }
        if ($inner === null) {
            return null;
        }
        $src = (string) file_get_contents($file);
        $text = substr($src, $inner->getStartFilePos(), $inner->getEndFilePos() + 1 - $inner->getStartFilePos());
        file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/peephole.jsonl', json_encode([
            'kind' => 'edit', 'file' => $file, 'site' => $file . ':' . $e->getStartFilePos(),
            'edits' => [[$e->getStartFilePos(), $e->getEndFilePos() + 1, $text]],
        ], JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
        return null;
    }
}
