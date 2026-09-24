<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use Psalm\Internal\Analyzer\MethodAnalyzer;
use Psalm\Plugin\EventHandler\AfterCodebasePopulatedInterface;
use Psalm\Plugin\EventHandler\AfterExpressionAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterCodebasePopulatedEvent;
use Psalm\Plugin\EventHandler\Event\AfterExpressionAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use Psalm\Type\Atomic;
use Psalm\Type\Union;
use SimpleXMLElement;
use Throwable;

/**
 * Inventory of the string-typed names left in psalm-port's src (the counterpart of pzoom_scan.py):
 *  - `slot`: every property / parameter / return whose type holds a class-like (or member) name as a string:
 *    class-string / interface-string / enum-string types, or a string / lowercase-string / non-empty-string
 *    typed slot whose name says it is a name; array slots keyed or valued by such strings too;
 *  - `boundary`: every strtolower / mb_strtolower / strcasecmp / Interner::intern / Interner::lookup call site,
 *    by enclosing method (pzoom's intern / lookup / to_ascii_lowercase boundaries).
 * scan-report.php compares them with pzoom's inventory.
 */
final class ClassNameScanPlugin implements PluginEntryPointInterface, AfterCodebasePopulatedInterface, AfterExpressionAnalysisInterface
{
    private const NAMEISH = '/(^|_)(fq|fqn|fqcn|fqcln|class|classlike|interface|trait|enum|parent|self|static|declaring|'
        . 'appearing|implementing|extended|mixin|name|names|type_name|method|function|property|const|id|ids)(_|$|s$)/i';

    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function afterCodebasePopulated(AfterCodebasePopulatedEvent $event): void
    {
        $codebase = $event->getCodebase();
        foreach ($codebase->classlike_storage_provider->getAll() as $storage) {
            $file = $storage->location?->file_path ?? '';
            if (!str_starts_with($file, self::root() . '/src/')) {
                continue;
            }
            foreach ($storage->properties as $pkey => $prop) {
                $pname = is_int($pkey) ? \Psalm\Internal\Interner::lookup($pkey) : (string) $pkey;
                $t = $prop->type ?? $prop->signature_type;
                $k = self::stringKind($t, (string) $pname);
                if ($k !== null) {
                    self::out(['kind' => 'slot', 'slot' => 'property', 'class' => $storage->name, 'name' => (string) $pname,
                        'type' => $t?->getId(), 'what' => $k, 'static' => $prop->is_static, 'file' => $file,
                        'line' => $prop->location?->getLineNumber()]);
                }
            }
            foreach ($storage->methods as $ms) {
                if ($ms->defining_fqcln !== null && strcasecmp($ms->defining_fqcln, $storage->name) !== 0) {
                    continue;
                }
                $mid = $storage->name . '::' . ($ms->cased_name ?? '?');
                foreach ($ms->params as $p) {
                    $t = $p->type ?? $p->signature_type;
                    $k = self::stringKind($t, $p->name);
                    if ($k !== null) {
                        self::out(['kind' => 'slot', 'slot' => 'param', 'class' => $storage->name, 'method' => $mid,
                            'name' => $p->name, 'type' => $t?->getId(), 'what' => $k, 'file' => $file,
                            'line' => $p->location?->getLineNumber(), 'public_api' => !str_contains($storage->name, '\\Internal\\')]);
                    }
                }
                $rt = $ms->return_type ?? $ms->signature_return_type;
                $k = self::stringKind($rt, (string) $ms->cased_name);
                if ($k !== null) {
                    self::out(['kind' => 'slot', 'slot' => 'return', 'class' => $storage->name, 'method' => $mid,
                        'name' => (string) $ms->cased_name, 'type' => $rt?->getId(), 'what' => $k, 'file' => $file,
                        'line' => $ms->location?->getLineNumber()]);
                }
            }
        }
    }

    /** 'class-string' | 'name-string' | 'array-key' | 'array-value', or null when the slot holds no name string */
    private static function stringKind(?Union $t, string $name): ?string
    {
        if ($t === null) {
            return null;
        }
        $nameish = preg_match(self::NAMEISH, $name) === 1;
        foreach ($t->getAtomicTypes() as $a) {
            $r = self::atomicKind($a, $nameish, 0);
            if ($r !== null) {
                return $r;
            }
        }
        return null;
    }

    private static function atomicKind(Atomic $a, bool $nameish, int $depth): ?string
    {
        if ($a instanceof Atomic\TClassString || $a instanceof Atomic\TLiteralClassString) {
            return $depth === 0 ? 'class-string' : 'nested-class-string';
        }
        if ($a instanceof Atomic\TString && !$a instanceof Atomic\TLiteralString && $nameish) {
            return $depth === 0 ? 'name-string' : 'nested-name-string';
        }
        if ($depth < 2 && ($a instanceof Atomic\TArray || $a instanceof Atomic\TKeyedArray)) {
            $params = $a instanceof Atomic\TArray ? $a->type_params : ($a->fallback_params ?? []);
            if (count($params) === 2) {
                foreach ($params[0]->getAtomicTypes() as $ka) {
                    if ($ka instanceof Atomic\TClassString || ($ka instanceof Atomic\TString && $nameish)) {
                        return 'array-key';
                    }
                }
                foreach ($params[1]->getAtomicTypes() as $va) {
                    $r = self::atomicKind($va, $nameish, $depth + 1);
                    if ($r !== null) {
                        return 'array-value:' . $r;
                    }
                }
            }
        }
        return null;
    }

    public static function afterExpressionAnalysis(AfterExpressionAnalysisEvent $event): ?bool
    {
        $e = $event->getExpr();
        $file = $event->getStatementsSource()->getFilePath();
        if (!str_starts_with($file, self::root() . '/src/')) {
            return null;
        }
        $kind = null;
        if ($e instanceof Expr\FuncCall && $e->name instanceof Name) {
            $fn = strtolower($e->name->getLast());
            if (in_array($fn, ['strtolower', 'mb_strtolower', 'strcasecmp', 'strncasecmp', 'ucfirst', 'lcfirst'], true)) {
                $kind = 'casefold:' . $fn;
            }
        } elseif ($e instanceof Expr\StaticCall && $e->class instanceof Name && $e->name instanceof Identifier) {
            $cls = strtolower((string) ($e->class->attrs()->resolvedName ?? $e->class->toString()));
            if (str_ends_with($cls, 'interner')) {
                $kind = 'interner:' . strtolower($e->name->name);
            }
        }
        if ($kind === null) {
            return null;
        }
        $method = '<file>';
        $s = $event->getStatementsSource();
        for ($i = 0; $i < 4; $i++) {
            if ($s instanceof MethodAnalyzer) {
                $method = (string) $s->getMethodId();
                break;
            }
            try {
                $next = $s->getSource();
            } catch (Throwable) {
                break;
            }
            if ($next === $s) {
                break;
            }
            $s = $next;
        }
        self::out(['kind' => 'boundary', 'what' => $kind, 'method' => $method, 'file' => $file, 'line' => $e->getStartLine(),
            'pos' => $e->getStartFilePos()]);
        return null;
    }

    private static function root(): string
    {
        return dirname(__DIR__, 2);
    }

    /** @param array<string, mixed> $row */
    private static function out(array $row): void
    {
        $path = getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/classname-scan.jsonl';
        file_put_contents($path, json_encode($row, JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
    }
}
