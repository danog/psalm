<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use PhpParser\Node\Arg;
use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\Scalar\String_;
use Psalm\Codebase;
use Psalm\Internal\Analyzer\MethodAnalyzer;
use Psalm\Internal\Interner;
use Psalm\Internal\MethodIdentifier;
use Psalm\Internal\Sym;
use Psalm\NodeTypeProvider;
use Psalm\Plugin\EventHandler\AfterExpressionAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterExpressionAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use Psalm\StatementsSource;
use Psalm\Storage\ClassLikeStorage;
use Psalm\Type\Atomic\TNamedObject;
use ReflectionClass;
use SimpleXMLElement;
use Throwable;

/**
 * Collapses every string/id method pair (`m(string $class, ..)` + `mById(int $class, ..)`) into one id-taking `m`:
 * calls to `mById` are renamed to `m`; calls to the string `m` keep the name and pass ids: the argument's id form
 * when Psalm's types give one (TNamedObject->value => ->name, ClassLikeStorage->name => ->id,
 * MethodIdentifier->fq_class_name => ->class_id, a src literal / X::class => a Sym constant), else
 * `Interner::intern(<arg>)` (the migration frontier, pushed to the name sources by later rounds).
 * Calls inside the pair methods' own declarations are left for the declaration rewrite.
 */
final class IdApiPlugin implements PluginEntryPointInterface, AfterExpressionAnalysisInterface
{
    /** @var array<string, ?array{string, list<int>, string}> call key => [twin name, positions, 'string'|'id'] */
    private static array $pairs = [];
    /** @var ?array<int, string> */
    private static ?array $sym = null;

    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function afterExpressionAnalysis(AfterExpressionAnalysisEvent $event): ?bool
    {
        $call = $event->getExpr();
        if (!$call instanceof Expr\MethodCall && !$call instanceof Expr\StaticCall && !$call instanceof Expr\NullsafeMethodCall) {
            return null;
        }
        if (!$call->name instanceof Identifier || $call->isFirstClassCallable()) {
            return null;
        }
        try {
            self::handle($event, $call);
        } catch (Throwable $e) {
            self::out(['kind' => 'error', 'msg' => $e->getMessage() . ' @' . $event->getStatementsSource()->getFilePath() . ':' . $call->getStartLine()]);
        }
        return null;
    }

    private static function handle(AfterExpressionAnalysisEvent $event, Expr\MethodCall|Expr\StaticCall|Expr\NullsafeMethodCall $call): void
    {
        $codebase = $event->getCodebase();
        $source = $event->getStatementsSource();
        $file = $source->getFilePath();
        $types = $source->getNodeTypeProvider();
        $classes = self::receiverClasses($call, $source, $types);
        if ($classes === null || $classes === []) {
            return;
        }
        $pair = null;
        foreach ($classes as $cls) {
            $p = self::pair($codebase, $cls, $call->name->name);
            if ($p === null || ($pair !== null && $p !== $pair)) {
                return;
            }
            $pair = $p;
        }
        [$other, $positions, $role, $decl_class] = $pair;
        // inside the pair's own declarations (the string body, the id body): the declaration rewrite handles them
        $encl = self::enclosing($source);
        if ($encl !== null && strcasecmp($encl[0], $decl_class) === 0) {
            $lcm = strtolower($encl[1]);
            $lcc = strtolower($call->name->name);
            $base = str_ends_with($lcc, 'byid') ? substr($lcc, 0, -4) : $lcc;
            if ($lcm === $base || $lcm === $base . 'byid') {
                return;
            }
        }
        $site = $file . ':' . $call->getStartLine();
        if ($role === 'id') {
            self::out(['kind' => 'edit', 'file' => $file, 'site' => $site,
                'edits' => [[$call->name->getStartFilePos(), $call->name->getEndFilePos() + 1, $other]]]);
            return;
        }
        $in_src = str_contains($file, '/src/');
        $args = $call->getArgs();
        $edits = [];
        $wrapped = 0;
        foreach ($positions as $pos) {
            $arg = $args[$pos] ?? null;
            if (!$arg instanceof Arg || $arg->unpack) {
                self::out(['kind' => 'manual', 'site' => $site, 'why' => 'missing/unpacked arg']);
                return;
            }
            $conv = self::convert($arg->value, $types, $codebase, $in_src);
            if ($conv !== null) {
                $edits[] = $conv[0];
                if ($conv[1] !== null) {
                    self::out(['kind' => 'sym', 'name' => $conv[1][0], 'value' => $conv[1][1]]);
                }
                continue;
            }
            $s = $arg->value->getStartFilePos();
            $e = $arg->value->getEndFilePos() + 1;
            $edits[] = [$s, $s, 'Interner::intern('];
            $edits[] = [$e, $e, ')'];
            $wrapped++;
        }
        self::out(['kind' => 'edit', 'file' => $file, 'site' => $site, 'wrapped' => $wrapped, 'edits' => $edits]);
    }

    /**
     * For a call to a method of a string/id pair: [the other method's name, class-name positions, role, declaring
     * class]. Role 'string' for m, 'id' for mById.
     *
     * @return ?array{string, list<int>, string, string}
     */
    private static function pair(Codebase $codebase, string $cls, string $method): ?array
    {
        $key = strtolower($cls . '::' . $method);
        if (array_key_exists($key, self::$pairs)) {
            return self::$pairs[$key];
        }
        self::$pairs[$key] = null;
        $lc = strtolower($method);
        $is_id = str_ends_with($lc, 'byid') && strlen($lc) > 4;
        $string_name = $is_id ? substr($method, 0, -4) : $method;
        $id_name = $is_id ? $method : $method . 'ById';
        try {
            $mdecl = $codebase->methods->getDeclaringMethodId(new MethodIdentifier($cls, strtolower($string_name)));
            $tdecl = $codebase->methods->getDeclaringMethodId(new MethodIdentifier($cls, strtolower($id_name)));
            if ($tdecl === null) {
                return null;
            }
            $ts = $codebase->methods->getStorage($tdecl);
            if ($mdecl === null) {
                // an id method without a string twin (findById): renamed only if the string name is free
                return null;
            }
            $ms = $codebase->methods->getStorage($mdecl);
        } catch (Throwable) {
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
        $decl_class = $tdecl->fq_class_name;
        return self::$pairs[$key] = $is_id
            ? [$ms->cased_name ?? $string_name, $positions, 'id', $decl_class]
            : [$ts->cased_name ?? $id_name, $positions, 'string', $decl_class];
    }

    /** @return ?array{string, string} [class, method] of the enclosing method */
    private static function enclosing(StatementsSource $source): ?array
    {
        $s = $source;
        for ($i = 0; $i < 4; $i++) {
            if ($s instanceof MethodAnalyzer) {
                $id = $s->getMethodId();
                return [$id->fq_class_name, $id->method_name];
            }
            $next = $s->getSource();
            if ($next === $s) {
                return null;
            }
            $s = $next;
        }
        return null;
    }

    /** @return ?list<string> */
    private static function receiverClasses(Expr\MethodCall|Expr\StaticCall|Expr\NullsafeMethodCall $call, StatementsSource $source, NodeTypeProvider $types): ?array
    {
        if ($call instanceof Expr\StaticCall) {
            if (!$call->class instanceof Name) {
                return null;
            }
            $resolved = (string) ($call->class->attrs()->resolvedName ?? $call->class->toString());
            if (in_array(strtolower($resolved), ['self', 'static', 'parent'], true)) {
                $resolved = (string) $source->getFQCLN();
            }
            return [$resolved];
        }
        $recv = $types->getType($call->var);
        if ($recv === null) {
            return null;
        }
        $classes = [];
        foreach ($recv->getAtomicTypes() as $a) {
            if ($a instanceof TNamedObject) {
                $classes[] = $a->value;
            } elseif (!$a instanceof \Psalm\Type\Atomic\TNull) {
                return null;
            }
        }
        return $classes;
    }

    /** @return ?array{array{int, int, string}, ?array{string, int}} */
    private static function convert(Expr $e, NodeTypeProvider $types, Codebase $codebase, bool $in_src): ?array
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
                $named = strcasecmp($a->value, TNamedObject::class) === 0
                    || $codebase->classExtendsOrImplements($a->value, TNamedObject::class);
                if ($prop === 'value' && $named) {
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
            return $to === null ? null : [[$e->name->getStartFilePos(), $e->name->getEndFilePos() + 1, $to], null];
        }
        if (!$in_src) {
            return null;
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
        if (self::$sym === null) {
            self::$sym = [];
            foreach ((new ReflectionClass(Sym::class))->getConstants() as $n => $v) {
                if (is_int($v)) {
                    self::$sym[$v] = $n;
                }
            }
        }
        $range = [$e->getStartFilePos(), $e->getEndFilePos() + 1];
        if (isset(self::$sym[$value])) {
            return [[...$range, 'Sym::' . self::$sym[$value]], null];
        }
        $words = [];
        foreach (explode('\\', $literal) as $p) {
            $words[] = strtoupper((string) preg_replace('/(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])/', '_', $p));
        }
        $name = 'C_' . preg_replace('/[^A-Z0-9_]/', '_', implode('__', $words));
        return [[...$range, 'Sym::' . $name], [$name, $value]];
    }

    /** @param array<string, mixed> $row */
    private static function out(array $row): void
    {
        $path = getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/id-api.jsonl';
        file_put_contents($path, json_encode($row, JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
    }
}
