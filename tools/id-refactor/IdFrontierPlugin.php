<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use PhpParser\Node;
use PhpParser\Node\Arg;
use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\Scalar\String_;
use PhpParser\NodeFinder;
use Psalm\Codebase;
use Psalm\Internal\Analyzer\ClosureAnalyzer;
use Psalm\Internal\Analyzer\FunctionLikeAnalyzer;
use Psalm\Internal\Analyzer\MethodAnalyzer;
use Psalm\Internal\Interner;
use Psalm\Internal\MethodIdentifier;
use Psalm\Internal\Sym;
use Psalm\NodeTypeProvider;
use Psalm\Plugin\EventHandler\AfterExpressionAnalysisInterface;
use Psalm\Plugin\EventHandler\AfterFunctionLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterExpressionAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\AfterFunctionLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use Psalm\StatementsSource;
use Psalm\Storage\ClassLikeStorage;
use Psalm\Storage\MethodStorage;
use Psalm\Type\Atomic\TNamedObject;
use ReflectionClass;
use SimpleXMLElement;
use Throwable;

/**
 * Facts for pushing the Interner::intern() frontier toward the name sources (frontier.php turns them into edits).
 * `site`: an `Interner::intern(E)` call, E classified; the rest as below (twin facts are gone with the twins).
 *
 * Former description: facts for the id refactor. Moves class-name strings to interned ids the way pzoom
 * keys everything by StrId:
 *
 *  - `twin`: a call to a string-keyed method with an id twin (`m(string $class..)` / `mById(int $class..)`), each
 *    class-name argument classified (below);
 *  - `call`: a call to an internal method, with each argument bound to a plain `string` parameter classified;
 *  - `fn`: an internal method's plain `string` parameters: declaration ranges, @param range, every use, and whether
 *    the method may change signature (not overridable, overrides nothing, internal namespace).
 *
 * An argument is `edit` (an id form known from types: TNamedObject->value => ->name, ClassLikeStorage->name => ->id,
 * MethodIdentifier->fq_class_name => ->class_id, literal / X::class => Sym constant), `param` (a parameter of the
 * enclosing method, convertible when that parameter is), or `no`.
 */
final class IdFrontierPlugin implements PluginEntryPointInterface, AfterExpressionAnalysisInterface, AfterFunctionLikeAnalysisInterface
{
    /** @var array<string, ?array{string, list<int>}> */
    private static array $twins = [];
    /** @var ?array<int, string> */
    private static ?array $sym = null;

    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function afterExpressionAnalysis(AfterExpressionAnalysisEvent $event): ?bool
    {
        $expr = $event->getExpr();
        if ($expr instanceof Expr\StaticCall && $expr->class instanceof Name && $expr->name instanceof Identifier
            && strtolower($expr->name->name) === 'intern'
            && str_ends_with(strtolower((string) ($expr->class->attrs()->resolvedName ?? $expr->class->toString())), 'internal\\interner')
            && count($expr->getArgs()) === 1 && str_contains($event->getStatementsSource()->getFilePath(), '/src/')
            && !str_ends_with($event->getStatementsSource()->getFilePath(), '/Interner.php')
        ) {
            try {
                $source = $event->getStatementsSource();
                $file = $source->getFilePath();
                $e = $expr->getArgs()[0]->value;
                $c = self::classify($e, $source, $source->getNodeTypeProvider(), $event->getCodebase(), $file);
                if ($c['k'] === 'edit') {
                    // the whole call becomes E's id form
                    $src = (string) file_get_contents($file);
                    $es = $e->getStartFilePos();
                    $etext = substr($src, $es, $e->getEndFilePos() + 1 - $es);
                    [$cs, $ce, $ct] = $c['e'];
                    $c['idtext'] = substr($etext, 0, $cs - $es) . $ct . substr($etext, $ce - $es);
                }
                self::out(['kind' => 'site', 'file' => $file, 'site' => $file . ':' . $expr->getStartLine() . ':' . $expr->getStartFilePos(),
                    'r' => [$expr->getStartFilePos(), $expr->getEndFilePos() + 1], 'arg' => $c]);
            } catch (Throwable $e) {
                self::out(['kind' => 'error', 'msg' => 'site: ' . $e->getMessage()]);
            }
            return null;
        }
        if (!$expr instanceof Expr\MethodCall && !$expr instanceof Expr\StaticCall && !$expr instanceof Expr\NullsafeMethodCall) {
            return null;
        }
        if (!$expr->name instanceof Identifier || $expr->isFirstClassCallable()) {
            return null;
        }
        $file = $event->getStatementsSource()->getFilePath();
        if (!str_contains($file, '/src/')) {
            return null;
        }
        try {
            self::call($event, $expr, $file);
        } catch (Throwable $e) {
            self::out(['kind' => 'error', 'msg' => $e->getMessage() . ' @' . $file . ':' . $expr->getStartLine()]);
        }
        return null;
    }

    private static function call(AfterExpressionAnalysisEvent $event, Expr\MethodCall|Expr\StaticCall|Expr\NullsafeMethodCall $call, string $file): void
    {
        $codebase = $event->getCodebase();
        $source = $event->getStatementsSource();
        $types = $source->getNodeTypeProvider();
        $method = $call->name->name;
        $classes = self::receiverClasses($call, $source, $types);
        if ($classes === null) {
            return;
        }
        $site = $file . ':' . $call->getStartLine() . ':' . $call->getStartFilePos();
        $args = $call->getArgs();

        // calls to internal methods: the arguments of their plain string parameters
        if (count($classes) !== 1) {
            return;
        }
        try {
            $mid = new MethodIdentifier($classes[0], strtolower($method));
            $decl = $codebase->methods->getDeclaringMethodId($mid);
        } catch (Throwable) {
            return;
        }
        if ($decl === null) {
            return;
        }
        $storage = $codebase->methods->getStorage($decl);
        $cargs = [];
        foreach ($storage->params as $i => $p) {
            if ($p->signature_type?->getId() !== 'string' || $p->is_variadic) {
                continue;
            }
            $arg = null;
            foreach ($args as $j => $a) {
                if (($a->name !== null && $a->name->name === $p->name) || ($a->name === null && $j === $i && !$a->unpack)) {
                    $arg = $a;
                }
            }
            if ($arg === null) {
                $cargs[$p->name] = ['k' => 'absent'];
                continue;
            }
            $c = self::classify($arg->value, $source, $types, $codebase, $file);
            if ($arg->name !== null) {
                $c['named'] = [$arg->name->getStartFilePos(), $arg->name->getEndFilePos() + 1];
            }
            $cargs[$p->name] = $c;
        }
        if ($cargs !== []) {
            self::out(['kind' => 'call', 'site' => $site, 'file' => $file, 'callee' => strtolower((string) $decl), 'args' => $cargs]);
        }
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
                if (strtolower($resolved) === 'parent') {
                    return null;
                }
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
            if (!$a instanceof TNamedObject) {
                if ($a instanceof \Psalm\Type\Atomic\TNull) {
                    continue;
                }
                return null;
            }
            $classes[] = $a->value;
        }
        return $classes;
    }

    /** @return array<string, mixed> */
    private static function classify(Expr $e, StatementsSource $source, NodeTypeProvider $types, Codebase $codebase, string $file): array
    {
        $edit = self::convert($e, $types, $codebase);
        if ($edit !== null) {
            if ($edit[1] !== null) {
                self::out(['kind' => 'sym', 'name' => $edit[1][0], 'value' => $edit[1][1]]);
            }
            return ['k' => 'edit', 'e' => $edit[0]];
        }
        if ($e instanceof Expr\Variable && is_string($e->name)) {
            $fn = self::enclosing($source);
            if ($fn !== null) {
                return ['k' => 'param', 'f' => $fn, 'p' => $e->name, 'r' => [$e->getStartFilePos(), $e->getEndFilePos() + 1]];
            }
        }
        $t = $types->getType($e);
        return ['k' => 'no', 'shape' => $e->getType() . ' : ' . ($t?->getId() ?? '?'),
            'text' => substr((string) file_get_contents($file), $e->getStartFilePos(), min(80, $e->getEndFilePos() - $e->getStartFilePos() + 1))];
    }

    /** The lowercase declaring id of the method whose body this is (not a closure), or null. */
    private static function enclosing(StatementsSource $source): ?string
    {
        $s = $source;
        for ($i = 0; $i < 3 && $s !== null; $i++) {
            if ($s instanceof ClosureAnalyzer) {
                return null;
            }
            if ($s instanceof MethodAnalyzer) {
                return strtolower((string) $s->getMethodId());
            }
            if ($s instanceof FunctionLikeAnalyzer) {
                return null;
            }
            $next = $s->getSource();
            if ($next === $s) {
                return null;
            }
            $s = $next;
        }
        return null;
    }

    public static function afterStatementAnalysis(AfterFunctionLikeAnalysisEvent $event): ?bool
    {
        try {
            self::fn($event);
        } catch (Throwable $e) {
            self::out(['kind' => 'error', 'msg' => 'fn: ' . $e->getMessage()]);
        }
        return null;
    }

    private static function fn(AfterFunctionLikeAnalysisEvent $event): void
    {
        $stmt = $event->getStmt();
        $storage = $event->getFunctionlikeStorage();
        $source = $event->getStatementsSource();
        $file = $source->getFilePath();
        if (!$stmt instanceof Node\Stmt\ClassMethod || !$storage instanceof MethodStorage || !str_contains($file, '/src/')) {
            return;
        }
        $codebase = $event->getCodebase();
        $fq = $storage->defining_fqcln ?? null;
        if ($fq === null) {
            return;
        }
        $id = strtolower($fq . '::' . $stmt->name->name);
        $cls = $codebase->classlike_storage_provider->get($fq);
        $lc = strtolower($stmt->name->name);
        // declared by an ancestor or an interface: the signature is shared
        $overrides = false;
        foreach ([...array_keys($cls->parent_classes), ...array_keys($cls->class_implements), ...array_keys($cls->parent_interfaces)] as $anc) {
            try {
                $as = $codebase->classlike_storage_provider->get((string) $anc);
            } catch (Throwable) {
                continue;
            }
            foreach ($as->methods as $k => $ms) {
                if (strtolower((string) ($ms->cased_name ?? $k)) === $lc) {
                    $overrides = true;
                    break 2;
                }
            }
        }
        $fixed_why = !str_starts_with($fq, 'Psalm\\Internal\\') ? 'public-api'
            : ($overrides ? 'overrides'
            : ($cls->is_interface || $cls->is_trait ? 'interface/trait'
            : (!($stmt->isPrivate() || $stmt->isFinal() || $cls->final) ? 'overridable'
            : ($lc === '__construct' ? 'ctor' : null))));
        $fixed = $fixed_why === null;
        $params = [];
        $finder = new NodeFinder();
        $doc = $stmt->getDocComment();
        foreach ($stmt->params as $param) {
            if (!$param->var instanceof Expr\Variable || !is_string($param->var->name)) {
                continue;
            }
            $name = $param->var->name;
            $type = $param->type;
            if (!$type instanceof Identifier || strtolower($type->name) !== 'string' || $param->default !== null
                || $param->byRef || $param->variadic || $param->flags !== 0
            ) {
                continue;
            }
            $uses = [];
            $bad = null;
            foreach ($finder->find($stmt->stmts ?? [], static fn(Node $n): bool =>
                $n instanceof Expr\Variable && $n->name === $name) as $v) {
                $uses[] = [$v->getStartFilePos(), $v->getEndFilePos() + 1];
            }
            foreach ($finder->find($stmt->stmts ?? [], static fn(Node $n): bool => true) as $n) {
                if (($n instanceof Expr\Assign || $n instanceof Expr\AssignOp || $n instanceof Expr\AssignRef)
                    && $n->var instanceof Expr\Variable && $n->var->name === $name) {
                    $bad = 'reassigned';
                } elseif ($n instanceof Expr\ClosureUse && $n->var->name === $name) {
                    $bad = 'captured';
                } elseif ($n instanceof Expr\ArrowFunction && $finder->findFirst([$n->expr], static fn(Node $x): bool =>
                    $x instanceof Expr\Variable && $x->name === $name) !== null) {
                    $bad = 'arrow-captured';
                } elseif ($n instanceof Expr\FuncCall && $n->name instanceof Name
                    && in_array(strtolower($n->name->toString()), ['compact', 'extract', 'get_defined_vars', 'func_get_args'], true)) {
                    $bad = 'dynamic-vars';
                } elseif ($n instanceof Expr\Variable && !is_string($n->name)) {
                    $bad = 'variable-variable';
                } elseif ($n instanceof Node\Stmt\Unset_ || $n instanceof Node\Stmt\Global_ || $n instanceof Node\Stmt\Static_) {
                    foreach ($n->vars as $uv) {
                        if ($uv instanceof Expr\Variable && $uv->name === $name) {
                            $bad = 'unset/global/static';
                        }
                    }
                } elseif ($n instanceof Expr\List_ || $n instanceof Expr\Array_) {
                    // destructuring into the parameter
                    foreach ($n->items as $it) {
                        if ($it !== null && $it->value instanceof Expr\Variable && $it->value->name === $name && $it->byRef) {
                            $bad = 'by-ref';
                        }
                    }
                }
            }
            $docr = null;
            if ($doc !== null && preg_match('/@param\s+(\S+)\s+\$' . preg_quote($name, '/') . '\b/', $doc->getText(), $m, PREG_OFFSET_CAPTURE)) {
                $base = $doc->getStartFilePos();
                $docr = [[$base + $m[1][1], $base + $m[1][1] + strlen($m[1][0])], [$base + $m[0][1] + strlen($m[0][0]) - strlen($name) - 1, $base + $m[0][1] + strlen($m[0][0])]];
            }
            $params[$name] = [
                'type' => [$type->getStartFilePos(), $type->getEndFilePos() + 1],
                'var' => [$param->var->getStartFilePos(), $param->var->getEndFilePos() + 1],
                'doc' => $docr,
                'uses' => $uses,
                'bad' => $bad,
            ];
        }
        if ($params !== []) {
            self::out(['kind' => 'fn', 'id' => $id, 'file' => $file, 'fixed' => $fixed, 'fixed_why' => $fixed_why, 'params' => $params]);
        }
        self::locals($stmt, $id, $file, $event->getNodeTypeProvider(), $codebase);
    }

    /**
     * Locals assigned exactly once, by an expression statement `$x = E;` whose E has an id form, and never written
     * otherwise: `$x_id = <E's id form>;` can follow the assignment and stand in for $x where an id is needed.
     */
    private static function locals(Node\Stmt\ClassMethod $stmt, string $id, string $file, NodeTypeProvider $types, Codebase $codebase): void
    {
        $finder = new NodeFinder();
        $all = $stmt->stmts ?? [];
        $writes = [];
        $single = [];
        foreach ($finder->find($all, static fn(Node $n): bool => true) as $n) {
            $targets = [];
            if ($n instanceof Expr\Assign || $n instanceof Expr\AssignOp || $n instanceof Expr\AssignRef) {
                $targets[] = $n->var;
                if ($n instanceof Expr\AssignRef) { $targets[] = $n->expr; }
            } elseif ($n instanceof Node\Stmt\Foreach_) {
                $targets[] = $n->valueVar;
                if ($n->keyVar) { $targets[] = $n->keyVar; }
            } elseif ($n instanceof Expr\List_ || ($n instanceof Expr\Array_ && false)) {
                foreach ($n->items as $it) { if ($it) { $targets[] = $it->value; } }
            } elseif ($n instanceof Node\Stmt\Unset_ || $n instanceof Node\Stmt\Global_ || $n instanceof Node\Stmt\Static_) {
                foreach ($n->vars as $v) { $targets[] = $v instanceof Node\StaticVar ? $v->var : $v; }
            } elseif ($n instanceof Expr\ClosureUse && $n->byRef) {
                $targets[] = $n->var;
            } elseif ($n instanceof Node\Stmt\Catch_ && $n->var) {
                $targets[] = $n->var;
            } elseif ($n instanceof Expr\Variable && !is_string($n->name)) {
                return; // variable variables: no local is safe
            } elseif ($n instanceof Expr\FuncCall && $n->name instanceof Name
                && in_array(strtolower($n->name->toString()), ['compact', 'extract', 'get_defined_vars'], true)) {
                return;
            } elseif ($n instanceof Arg && $n->value instanceof Expr\Variable) {
                $targets[] = $n->value; // may be by reference: counted as a write unless it is the one assignment
            }
            foreach ($targets as $t) {
                if ($t instanceof Expr\Variable && is_string($t->name)) {
                    $writes[$t->name] = ($writes[$t->name] ?? 0) + 1;
                }
            }
        }
        foreach ($finder->find($all, static fn(Node $n): bool => $n instanceof Node\Stmt\Expression
            && $n->expr instanceof Expr\Assign && $n->expr->var instanceof Expr\Variable && is_string($n->expr->var->name)) as $es) {
            $single[$es->expr->var->name][] = $es;
        }
        $src = (string) file_get_contents($file);
        foreach ($single as $name => $list) {
            if (count($list) !== 1 || ($writes[$name] ?? 0) !== 1) {
                continue;
            }
            foreach ($stmt->params as $prm) {
                if ($prm->var instanceof Expr\Variable && $prm->var->name === $name) { continue 2; }
            }
            $es = $list[0];
            $e = $es->expr->expr;
            $conv = self::convert($e, $types, $codebase);
            if ($conv === null) {
                continue;
            }
            [$cs, $ce, $ct] = $conv[0];
            $es0 = $e->getStartFilePos();
            $etext = substr($src, $es0, $e->getEndFilePos() + 1 - $es0);
            $idexpr = substr($etext, 0, $cs - $es0) . $ct . substr($etext, $ce - $es0);
            if ($conv[1] !== null) {
                self::out(['kind' => 'sym', 'name' => $conv[1][0], 'value' => $conv[1][1]]);
            }
            $line_start = strrpos(substr($src, 0, $es->getStartFilePos()), "\n");
            $indent = substr($src, $line_start + 1, $es->getStartFilePos() - $line_start - 1);
            if (trim($indent) !== '') {
                continue;
            }
            $uses = [];
            foreach ($finder->find($all, static fn(Node $n): bool => $n instanceof Expr\Variable && $n->name === $name) as $v) {
                $uses[] = [$v->getStartFilePos(), $v->getEndFilePos() + 1];
            }
            if ($finder->findFirst($all, static fn(Node $n): bool => $n instanceof Expr\Variable && $n->name === $name . '_id') !== null) {
                continue;
            }
            self::out(['kind' => 'local', 'f' => $id, 'p' => $name, 'file' => $file,
                'insert' => [$es->getEndFilePos() + 1, "\n" . $indent . '$' . $name . '_id = ' . $idexpr . ';'], 'uses' => $uses]);
        }
    }

    /** @return ?array{string, list<int>} */
    private static function twin(Codebase $codebase, string $cls, string $method): ?array
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
            $mdecl = $codebase->methods->getDeclaringMethodId(new MethodIdentifier($cls, strtolower($method)));
            $tdecl = $codebase->methods->getDeclaringMethodId(new MethodIdentifier($cls, strtolower($method . 'ById')));
            if ($mdecl === null || $tdecl === null) {
                return null;
            }
            $ms = $codebase->methods->getStorage($mdecl);
            $ts = $codebase->methods->getStorage($tdecl);
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
        if ($positions === [] || count($ts->params) < count($ms->params)) {
            return null;
        }
        return self::$twins[$key] = [$ts->cased_name ?? ($method . 'ById'), $positions];
    }

    /** @return ?array{array{int, int, string}, ?array{string, int}} */
    private static function convert(Expr $e, NodeTypeProvider $types, Codebase $codebase): ?array
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
        $parts = explode('\\', $literal);
        $words = [];
        foreach ($parts as $p) {
            $words[] = strtoupper((string) preg_replace('/(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])/', '_', $p));
        }
        $name = 'C_' . preg_replace('/[^A-Z0-9_]/', '_', implode('__', $words));
        return [[...$range, 'Sym::' . $name], [$name, $value]];
    }

    /** @param array<string, mixed> $row */
    private static function out(array $row): void
    {
        $path = getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/id-frontier.jsonl';
        file_put_contents($path, json_encode($row, JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
    }
}
