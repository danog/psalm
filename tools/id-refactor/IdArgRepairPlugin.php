<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use Psalm\Internal\Interner;
use Psalm\Internal\MethodIdentifier;
use Psalm\Plugin\EventHandler\AfterExpressionAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterExpressionAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use Psalm\Type\Atomic\TNamedObject;
use SimpleXMLElement;
use Throwable;

require_once __DIR__ . '/SymNames.php';

/**
 * Repairs calls after a parameter became an interned id: an argument whose type is a string, passed where the
 * callee's signature says `int`, becomes its id: `Interner::lookup(X)` -> `X`, a literal -> its Sym constant,
 * anything else -> `Interner::intern(arg)`. Paths: env ID_REFACTOR_ROOTS (colon-separated prefixes).
 */
final class IdArgRepairPlugin implements PluginEntryPointInterface, AfterExpressionAnalysisInterface
{
    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function afterExpressionAnalysis(AfterExpressionAnalysisEvent $event): ?bool
    {
        $call = $event->getExpr();
        $source = $event->getStatementsSource();
        $file = $source->getFilePath();
        $roots = array_filter(explode(':', (string) getenv('ID_REFACTOR_ROOTS')));
        if (array_filter($roots, static fn(string $r): bool => str_starts_with($file, $r)) === []) {
            return null;
        }
        if (!$call instanceof Expr\New_ && !$call instanceof Expr\MethodCall && !$call instanceof Expr\StaticCall
            && !$call instanceof Expr\NullsafeMethodCall
        ) {
            return null;
        }
        try {
            $codebase = $event->getCodebase();
            $types = $source->getNodeTypeProvider();
            $class = null;
            $method = null;
            if ($call instanceof Expr\New_) {
                if (!$call->class instanceof Name) {
                    return null;
                }
                $class = (string) ((isset($call->class->attrs()->resolvedId) ? Interner::lookupOrNull($call->class->attrs()->resolvedId) : $call->class->toString()));
                if (in_array(strtolower($class), ['self', 'static'], true)) {
                    $class = (string) $source->getFQCLN();
                }
                $method = '__construct';
            } elseif ($call instanceof Expr\StaticCall) {
                if (!$call->class instanceof Name || !$call->name instanceof Identifier) {
                    return null;
                }
                $class = (string) ((isset($call->class->attrs()->resolvedId) ? Interner::lookupOrNull($call->class->attrs()->resolvedId) : $call->class->toString()));
                if (in_array(strtolower($class), ['self', 'static'], true)) {
                    $class = (string) $source->getFQCLN();
                }
                $method = $call->name->name;
            } else {
                if (!$call->name instanceof Identifier) {
                    return null;
                }
                $t = $types->getType($call->var);
                foreach ($t?->getAtomicTypes() ?? [] as $a) {
                    if ($a instanceof TNamedObject) {
                        $class = SymNames::named($a);
                    }
                }
                $method = $call->name->name;
            }
            if ($class === null || in_array(strtolower($class), ['parent'], true)) {
                return null;
            }
            $decl = $codebase->methods->getDeclaringMethodId(new MethodIdentifier(\Psalm\Internal\Interner::intern($class), \Psalm\Internal\Interner::intern(strtolower($method))));
            if ($decl === null) {
                return null;
            }
            $ms = $codebase->methods->getStorage($decl);
            $src = (string) file_get_contents($file);
            $edits = [];
            if ($call->isFirstClassCallable()) {
                return null;
            }
            foreach ($call->getArgs() as $j => $arg) {
                $param = null;
                if ($arg->name !== null) {
                    foreach ($ms->params as $p) {
                        if ($p->name === $arg->name->name) {
                            $param = $p;
                        }
                    }
                } else {
                    $param = $ms->params[$j] ?? null;
                }
                if ($param === null || $arg->unpack || $param->signature_type?->getId() !== 'int') {
                    continue;
                }
                $at = $types->getType($arg->value);
                if ($at === null || !$at->isString()) {
                    continue;
                }
                $v = $arg->value;
                if ($v instanceof Expr\StaticCall && $v->class instanceof Name && $v->name instanceof Identifier
                    && strtolower($v->class->getLast()) === 'interner' && strtolower($v->name->name) === 'lookup'
                    && count($v->getArgs()) === 1
                ) {
                    $inner = $v->getArgs()[0]->value;
                    $edits[] = [$v->getStartFilePos(), $v->getEndFilePos() + 1,
                        substr($src, $inner->getStartFilePos(), $inner->getEndFilePos() + 1 - $inner->getStartFilePos())];
                } else {
                    $edits[] = [$v->getStartFilePos(), $v->getStartFilePos(), '\\Psalm\\Internal\\Interner::intern('];
                    $edits[] = [$v->getEndFilePos() + 1, $v->getEndFilePos() + 1, ')'];
                }
            }
            if ($edits !== []) {
                self::out(['kind' => 'edit', 'file' => $file, 'site' => $file . ':' . $call->getStartFilePos(), 'edits' => $edits]);
            }
        } catch (Throwable $e) {
            self::out(['kind' => 'error', 'msg' => $e->getMessage() . ' @' . $file . ':' . $call->getStartLine()]);
        }
        return null;
    }

    /** @param array<string, mixed> $row */
    private static function out(array $row): void
    {
        file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/arg-repair.jsonl',
            json_encode($row, JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
    }
}
