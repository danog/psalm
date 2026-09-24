<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use PhpParser\Node\Arg;
use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\Scalar\String_;
use Psalm\Internal\Interner;
use Psalm\Internal\MethodIdentifier;
use Psalm\Internal\Sym;
use Psalm\Plugin\EventHandler\AfterExpressionAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterExpressionAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use Psalm\Storage\ClassLikeStorage;
use Psalm\Type\Atomic\TNamedObject;
use ReflectionClass;
use SimpleXMLElement;
use Throwable;

/**
 * Mass refactoring to pzoom's id-keyed codebase API: every call to a string-keyed method `m(string $class, ...)` that
 * has an id twin `mById(int $class, ...)` (same parameters, class-name strings replaced by interned ids) is rewritten
 * to the twin when each class-name argument has a known id form, decided from Psalm's inferred types:
 *
 *   $named_object->value      -> $named_object->name          (TNamedObject: the interned name)
 *   $storage->name            -> $storage->id                 (ClassLikeStorage)
 *   $method_id->fq_class_name -> $method_id->class_id         (MethodIdentifier)
 *   'Literal' / X::class      -> Sym::CONSTANT                (precomputed Interner::hash; new constants generated)
 *
 * Edits and the remaining (unconvertible) call sites are appended as JSON lines to $ID_REFACTOR_OUT; apply.php
 * applies the edits and adds the missing Sym constants. Run: psalm --plugin=tools/id-refactor/IdRefactorPlugin.php
 */
final class IdRefactorPlugin implements PluginEntryPointInterface, AfterExpressionAnalysisInterface
{
    /** @var array<string, ?array{string, list<int>}> method key => [id twin name, class-name arg positions] */
    private static array $twins = [];

    /** @var ?array<int, string> Sym constant name by value */
    private static ?array $sym = null;

    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function afterExpressionAnalysis(AfterExpressionAnalysisEvent $event): ?bool
    {
        $expr = $event->getExpr();
        if (!$expr instanceof Expr\MethodCall && !$expr instanceof Expr\StaticCall && !$expr instanceof Expr\NullsafeMethodCall) {
            return null;
        }
        if (!$expr->name instanceof Identifier || $expr->isFirstClassCallable()) {
            return null;
        }
        try {
            self::handle($event, $expr);
        } catch (Throwable $e) {
            self::out(['kind' => 'error', 'msg' => $e->getMessage()]);
        }
        return null;
    }

    private static function handle(AfterExpressionAnalysisEvent $event, Expr\MethodCall|Expr\StaticCall|Expr\NullsafeMethodCall $call): void
    {
        $codebase = $event->getCodebase();
        $source = $event->getStatementsSource();
        $types = $source->getNodeTypeProvider();
        $method = $call->name->name;

        $classes = [];
        if ($call instanceof Expr\StaticCall) {
            if (!$call->class instanceof Name) {
                return;
            }
            $resolved = (string) ($call->class->attrs()->resolvedName ?? $call->class->toString());
            if (in_array(strtolower($resolved), ['self', 'static', 'parent'], true)) {
                $resolved = (string) $source->getFQCLN();
            }
            $classes[] = $resolved;
        } else {
            $recv = $types->getType($call->var);
            if ($recv === null) {
                return;
            }
            foreach ($recv->getAtomicTypes() as $a) {
                if ($a instanceof TNamedObject) {
                    $classes[] = $a->value;
                }
            }
        }
        $twin = null;
        foreach ($classes as $cls) {
            $t = self::twin($codebase, $cls, $method);
            if ($t === null) {
                return;
            }
            $twin ??= $t;
            if ($t !== $twin) {
                return;
            }
        }
        if ($twin === null) {
            return;
        }
        [$twin_name, $positions] = $twin;
        $file = $source->getFilePath();
        if (!str_contains($file, '/src/')) {
            return; // tests exercise the string APIs on purpose
        }
        $args = $call->getArgs();
        $edits = [];
        $unconverted = [];
        foreach ($positions as $pos) {
            $arg = $args[$pos] ?? null;
            if (!$arg instanceof Arg || $arg->unpack || $arg->name !== null) {
                $unconverted[] = ['pos' => $pos, 'why' => 'missing/unpacked/named'];
                continue;
            }
            $conv = self::convert($arg->value, $types, $codebase);
            if ($conv === null) {
                $t = $types->getType($arg->value);
                $unconverted[] = [
                    'pos' => $pos,
                    'expr' => self::text($file, $arg->value),
                    'node' => $arg->value->getType(),
                    'type' => $t?->getId() ?? '?',
                ];
                continue;
            }
            $edits[] = $conv[0];
            if ($conv[1] !== null) {
                self::out(['kind' => 'sym', 'name' => $conv[1][0], 'value' => $conv[1][1]]);
            }
        }
        $site = $file . ':' . $call->getStartLine() . ' ' . $classes[0] . '::' . $method;
        if ($unconverted !== []) {
            self::out(['kind' => 'skip', 'site' => $site, 'args' => $unconverted]);
            return;
        }
        $edits[] = [$call->name->getStartFilePos(), $call->name->getEndFilePos() + 1, $twin_name];
        self::out(['kind' => 'edit', 'file' => $file, 'site' => $site, 'edits' => $edits]);
    }

    /**
     * The id twin of $cls::$method and the positions of the class-name parameters, or null.
     *
     * @return ?array{string, list<int>}
     */
    private static function twin(\Psalm\Codebase $codebase, string $cls, string $method): ?array
    {
        $key = strtolower($cls . '::' . $method);
        if (array_key_exists($key, self::$twins)) {
            return self::$twins[$key];
        }
        self::$twins[$key] = null;
        if (str_ends_with(strtolower($method), 'byid')) {
            return null;
        }
        try {
            $mid = new MethodIdentifier($cls, strtolower($method));
            $tid = new MethodIdentifier($cls, strtolower($method . 'ById'));
            $mdecl = $codebase->methods->getDeclaringMethodId($mid);
            $tdecl = $codebase->methods->getDeclaringMethodId($tid);
            if ($mdecl === null || $tdecl === null) {
                return null;
            }
            $ms = $codebase->methods->getStorage($mdecl);
            $ts = $codebase->methods->getStorage($tdecl);
        } catch (Throwable) {
            return null;
        }
        if (count($ms->params) !== count($ts->params) && count($ms->params) + 1 !== count($ts->params)) {
            return null;
        }
        $positions = [];
        foreach ($ms->params as $i => $p) {
            $tp = $ts->params[$i] ?? null;
            if ($tp === null) {
                return null;
            }
            $pt = $p->signature_type?->getId();
            $tpt = $tp->signature_type?->getId();
            if ($pt === $tpt) {
                continue;
            }
            if ($pt === 'string' && $tpt === 'int') {
                $positions[] = $i;
                continue;
            }
            return null;
        }
        if ($positions === []) {
            return null;
        }
        return self::$twins[$key] = [$ts->cased_name ?? ($method . 'ById'), $positions];
    }

    /**
     * The edit turning a class-name expression into its id form ([start, end, text]), and a Sym constant to
     * create ([name, value]) when one is used. A property fetch only has its property name replaced.
     *
     * @return ?array{array{int, int, string}, ?array{string, int}}
     */
    private static function convert(Expr $e, \Psalm\NodeTypeProvider $types, \Psalm\Codebase $codebase): ?array
    {
        if (($e instanceof Expr\PropertyFetch || $e instanceof Expr\NullsafePropertyFetch) && $e->name instanceof Identifier) {
            $base = $types->getType($e->var);
            if ($base === null) {
                return null;
            }
            $prop = $e->name->name;
            $to = null;
            foreach ($base->getAtomicTypes() as $a) {
                if (!$a instanceof TNamedObject) {
                    return null;
                }
                $cand = null;
                if ($prop === 'value' && $codebase->classExtendsOrImplements($a->value, TNamedObject::class)
                    || $prop === 'value' && strcasecmp($a->value, TNamedObject::class) === 0) {
                    $cand = 'name';
                } elseif ($prop === 'name' && strcasecmp($a->value, ClassLikeStorage::class) === 0) {
                    $cand = 'id';
                } elseif ($prop === 'fq_class_name' && strcasecmp($a->value, MethodIdentifier::class) === 0) {
                    $cand = 'class_id';
                }
                if ($cand === null || ($to !== null && $to !== $cand)) {
                    return null;
                }
                $to = $cand;
            }
            if ($to === null) {
                return null;
            }
            return [[$e->name->getStartFilePos(), $e->name->getEndFilePos() + 1, $to], null];
        }
        $literal = null;
        if ($e instanceof String_) {
            $literal = ltrim($e->value, '\\');
        } elseif ($e instanceof Expr\ClassConstFetch && $e->class instanceof Name && $e->name instanceof Identifier
            && strtolower($e->name->name) === 'class'
        ) {
            $n = (string) ($e->class->attrs()->resolvedName ?? $e->class->toString());
            if (in_array(strtolower($n), ['self', 'static', 'parent'], true)) {
                return null;
            }
            $literal = ltrim($n, '\\');
        }
        if ($literal === null || $literal === '') {
            return null;
        }
        $value = Interner::hash($literal);
        $sym = self::symByValue();
        $range = [$e->getStartFilePos(), $e->getEndFilePos() + 1];
        if (isset($sym[$value])) {
            return [[...$range, 'Sym::' . $sym[$value]], null];
        }
        $name = self::symName($literal);
        return [[...$range, 'Sym::' . $name], [$name, $value]];
    }

    /** @return array<int, string> */
    private static function symByValue(): array
    {
        if (self::$sym === null) {
            self::$sym = [];
            foreach ((new ReflectionClass(Sym::class))->getConstants() as $n => $v) {
                if (is_int($v)) {
                    self::$sym[$v] = $n;
                }
            }
        }
        return self::$sym;
    }

    private static function symName(string $literal): string
    {
        $parts = preg_split('/\\\\/', $literal) ?: [$literal];
        $words = [];
        foreach ($parts as $p) {
            $words[] = strtoupper((string) preg_replace('/(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])/', '_', $p));
        }
        return 'C_' . preg_replace('/[^A-Z0-9_]/', '_', implode('__', $words));
    }

    private static function text(string $file, Expr $e): string
    {
        return substr((string) file_get_contents($file), $e->getStartFilePos(), $e->getEndFilePos() - $e->getStartFilePos() + 1);
    }

    /** @param array<string, mixed> $row */
    private static function out(array $row): void
    {
        $path = getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/id-refactor.jsonl';
        file_put_contents($path, json_encode($row, JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
    }
}
