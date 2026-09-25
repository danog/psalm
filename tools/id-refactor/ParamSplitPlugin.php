<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use PhpParser\Node;
use PhpParser\Node\Arg;
use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\Scalar\String_;
use PhpParser\Node\Stmt;
use Psalm\Codebase;
use Psalm\Internal\Interner;
use Psalm\NodeTypeProvider;
use Psalm\Plugin\EventHandler\AfterFunctionLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterFunctionLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use Psalm\Storage\MethodStorage;
use Psalm\Type\Atomic\TNamedObject;
use SimpleXMLElement;
use SplObjectStorage;
use Throwable;

/**
 * Splits a member-id string parameter ("Class::$prop") into its two parts (pzoom passes (class StrId, member StrId)):
 * env PARAM_SPLITS = JSON [[class, method, param, [class part, name part]]].
 *
 * The callee: `string $param` becomes `string $cls, string $name`; `[$cls, $name] = explode('::$', $param);` goes,
 * `$param = ltrim($param, '\\');` becomes `$cls = ltrim($cls, '\\');`, any other use reads `$cls . '::$' . $name`.
 * The calls: the argument `X . '::$' . Y` (or an interpolation, or a local whose nearest definition is one) passes
 * `X, Y`; a split parameter of the enclosing function passes its parts; anything else passes
 * `explode('::$', A)[0], explode('::$', A)[1]`.
 */
final class ParamSplitPlugin implements PluginEntryPointInterface, AfterFunctionLikeAnalysisInterface
{
    private const SEP = '::$';

    private Codebase $codebase;
    private NodeTypeProvider $types;
    private string $file;
    private string $src;
    private ?string $self_class = null;
    /** @var ?array{string, string, string, array{string, string}} the split this function itself declares */
    private ?array $own = null;
    /** @var list<Node> */
    private array $order = [];
    /** @var SplObjectStorage<Node, Node> */
    private SplObjectStorage $parent;
    /** @var list<array{int, int, string}> */
    private array $edits = [];

    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    /** @return list<array{string, string, string, array{string, string}}> */
    private static function cfg(): array
    {
        static $c = null;
        return $c ??= json_decode((string) getenv('PARAM_SPLITS'), true) ?: [];
    }

    private static function out(array $row): void
    {
        file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/param-split.jsonl',
            json_encode($row, JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
    }

    public static function afterStatementAnalysis(AfterFunctionLikeAnalysisEvent $event): ?bool
    {
        $file = $event->getStatementsSource()->getFilePath();
        require_once __DIR__ . '/MapIdSetPlugin.php';
        require_once __DIR__ . '/SymNames.php';
        if (!MapIdSetPlugin::inScope($file)) {
            return null;
        }
        try {
            $h = new self();
            $h->codebase = $event->getCodebase();
            $h->types = $event->getNodeTypeProvider();
            $h->file = $file;
            $h->src = (string) file_get_contents($file);
            $h->parent = new SplObjectStorage();
            $stmt = $event->getStmt();
            $storage = $event->getFunctionlikeStorage();
            if ($storage instanceof MethodStorage && $storage->defining_fqcln !== null) {
                $h->self_class = $storage->defining_fqcln;
                if ($stmt instanceof Stmt\ClassMethod) {
                    foreach (self::cfg() as $c) {
                        if (strcasecmp($c[1], $stmt->name->name) === 0 && self::isClass($h->codebase, $h->self_class, $c[0])) {
                            $h->own = $c;
                        }
                    }
                }
            }
            $body = $stmt instanceof Expr\ArrowFunction ? [$stmt->expr] : ($stmt->getStmts() ?? []);
            $h->index($body, $stmt);
            if ($h->own !== null && $stmt instanceof Stmt\ClassMethod) {
                $h->callee($stmt);
            }
            $h->calls();
            if ($h->edits !== []) {
                self::out(['kind' => 'edit', 'file' => $file, 'site' => $file . ':' . $stmt->getStartFilePos(), 'edits' => $h->edits]);
            }
        } catch (Throwable $e) {
            self::out(['kind' => 'error', 'msg' => $e->getMessage() . ' @' . $file . ':' . $e->getLine()]);
        }
        return null;
    }

    private static function isClass(Codebase $cb, string $cls, string $target): bool
    {
        return strcasecmp($cls, $target) === 0
            || ($cb->classOrInterfaceOrEnumExists(Interner::intern($cls)) && $cb->classExtendsOrImplements(Interner::intern($cls), Interner::intern($target)));
    }

    /** @param array<mixed>|Node $nodes */
    private function index(array|Node $nodes, Node $parent): void
    {
        foreach (is_array($nodes) ? $nodes : [$nodes] as $n) {
            if (!$n instanceof Node) {
                continue;
            }
            $this->parent[$n] = $parent;
            $this->order[] = $n;
            if ($n instanceof Expr\Closure || $n instanceof Expr\ArrowFunction || $n instanceof Stmt\Class_
                || $n instanceof Stmt\Function_
            ) {
                continue;
            }
            foreach ($n->getSubNodeNames() as $sub) {
                $v = $n->$sub;
                if ($v instanceof Node || is_array($v)) {
                    $this->index($v, $n);
                }
            }
        }
    }

    private function text(Node $n): string
    {
        return substr($this->src, $n->getStartFilePos(), $n->getEndFilePos() + 1 - $n->getStartFilePos());
    }

    private function edit(Node $n, string $to): void
    {
        $this->edits[] = [$n->getStartFilePos(), $n->getEndFilePos() + 1, $to];
    }

    private function manual(Node $n, string $why): void
    {
        self::out(['kind' => 'manual', 'site' => $this->file . ':' . $n->getStartLine(), 'why' => $why]);
    }

    private function callee(Stmt\ClassMethod $m): void
    {
        [, , $pname, [$cls, $name]] = $this->own;
        $param = null;
        foreach ($m->params as $p) {
            if ($p->var instanceof Expr\Variable && $p->var->name === $pname) {
                $param = $p;
            }
        }
        if ($param === null) {
            return;
        }
        $this->edit($param, 'string $' . $cls . ', string $' . $name);
        // a docblock @param for it
        $doc = $m->getDocComment();
        if ($doc !== null && preg_match('/@param\s+\S+\s+\$' . preg_quote($pname, '/') . '\b[^\n]*/', $doc->getText(), $mm, PREG_OFFSET_CAPTURE)) {
            $s = $doc->getStartFilePos() + $mm[0][1];
            $this->edits[] = [$s, $s + strlen($mm[0][0]), '@param string $' . $cls . "\n     * @param string $" . $name];
        }
        $whole = '($' . $cls . ' . \'' . self::SEP . '\' . $' . $name . ')';
        foreach ($this->order as $n) {
            if (!$n instanceof Expr\Variable || $n->name !== $pname) {
                continue;
            }
            $p = $this->parent[$n];
            // [$cls, $name] = explode('::$', $param);
            if ($p instanceof Arg) {
                $call = $this->parent[$p];
                $st = $this->parent[$call] ?? null;
                if ($call instanceof Expr\FuncCall && $call->name instanceof Name && strtolower($call->name->toString()) === 'explode'
                    && $st instanceof Expr\Assign && ($st->var instanceof Expr\List_ || $st->var instanceof Expr\Array_)
                ) {
                    $names = array_map(fn($it) => $it?->value instanceof Expr\Variable ? $it->value->name : null, $st->var->items);
                    $stmt = $this->parent[$st];
                    if ($stmt instanceof Stmt\Expression) {
                        $line_s = strrpos(substr($this->src, 0, $stmt->getStartFilePos()), "\n") + 1;
                        $line_e = strpos($this->src, "\n", $stmt->getEndFilePos()) + 1;
                        if ($names === [$cls, $name]) {
                            $this->edits[] = [$line_s, $line_e, ''];
                        } else {
                            $assign = [];
                            foreach ($names as $i => $nm) {
                                if ($nm !== null) {
                                    $assign[] = '$' . $nm . ' = $' . ($i === 0 ? $cls : $name) . ';';
                                }
                            }
                            $this->edit($stmt, implode(' ', $assign));
                        }
                        continue;
                    }
                }
                // $param = ltrim($param, '\\');
                if ($call instanceof Expr\FuncCall && $call->name instanceof Name && strtolower($call->name->toString()) === 'ltrim'
                    && $st instanceof Expr\Assign && $st->var instanceof Expr\Variable && $st->var->name === $pname
                ) {
                    $this->edit($st, '$' . $cls . ' = ltrim($' . $cls . ', ' . $this->text($call->getArgs()[1]->value) . ')');
                    continue;
                }
            }
            if ($p instanceof Expr\Assign && $p->var === $n) {
                continue; // the ltrim assignment's target (rewritten with its statement)
            }
            // an argument of a split call passes the parts (calls() handles it)
            if ($p instanceof Arg && $this->splitOf($this->parent[$p], $p) !== null) {
                continue;
            }
            $this->replaceRead($n, $whole);
        }
    }

    private function replaceRead(Expr $n, string $expr): void
    {
        $p = $this->parent[$n] ?? null;
        if ($p instanceof \PhpParser\Node\Scalar\InterpolatedString) {
            $s = $n->getStartFilePos();
            $e = $n->getEndFilePos() + 1;
            if ($this->src[$s - 1] === '{' && $this->src[$e] === '}') {
                $s--;
                $e++;
            }
            $this->edits[] = [$s, $e, '" . ' . $expr . ' . "'];
            return;
        }
        $this->edit($n, $expr);
    }

    /**
     * The split configuration of the callee when $arg passes its split parameter.
     *
     * @return ?array{string, string, string, array{string, string}}
     */
    private function splitOf(Node $call, Arg $arg): ?array
    {
        if (!($call instanceof Expr\MethodCall || $call instanceof Expr\StaticCall || $call instanceof Expr\NullsafeMethodCall)
            || !$call->name instanceof Identifier || $call->isFirstClassCallable()
        ) {
            return null;
        }
        foreach (self::cfg() as $c) {
            if (strcasecmp($c[1], $call->name->name) !== 0) {
                continue;
            }
            $cls = null;
            if ($call instanceof Expr\StaticCall) {
                if ($call->class instanceof Name) {
                    $n = strtolower($call->class->toString());
                    $cls = in_array($n, ['self', 'static', 'parent'], true) ? $this->self_class
                        : (Interner::lookupOrNull($call->class->attrs()->resolvedId) ?? $call->class->toString());
                }
            } else {
                $t = $this->types->getType($call->var);
                foreach ($t?->getAtomicTypes() ?? [] as $a) {
                    if ($a instanceof TNamedObject) {
                        $cls = SymNames::named($a);
                    }
                }
            }
            if ($cls === null || !self::isClass($this->codebase, $cls, $c[0])) {
                continue;
            }
            // the parameter's position
            $ms = $this->codebase->methods->getStorage(new \Psalm\Internal\MethodIdentifier(Interner::intern($c[0]), Interner::intern(strtolower($c[1]))));
            foreach ($ms->params as $i => $ps) {
                if ($ps->name !== $c[2]) {
                    continue;
                }
                $args = $call->getArgs();
                if (($arg->name !== null && $arg->name->name === $c[2]) || ($arg->name === null && ($args[$i] ?? null) === $arg)) {
                    return $c;
                }
            }
        }
        return null;
    }

    private function calls(): void
    {
        foreach ($this->order as $n) {
            if (!$n instanceof Arg) {
                continue;
            }
            $c = $this->splitOf($this->parent[$n], $n);
            if ($c === null) {
                continue;
            }
            if ($n->unpack) {
                $this->manual($n, 'spread argument');
                continue;
            }
            $parts = $this->parts($n->value, 0);
            $pre = $n->name !== null ? [$c[3][0] . ': ', $c[3][1] . ': '] : ['', ''];
            if ($parts === null) {
                $a = $this->text($n->value);
                $parts = ['explode(\'' . self::SEP . '\', ' . $a . ')[0]', 'explode(\'' . self::SEP . '\', ' . $a . ')[1]'];
                $this->manual($n, 'explode fallback: ' . $a);
            }
            $this->edit($n, $pre[0] . $parts[0] . ', ' . $pre[1] . $parts[1]);
        }
    }

    /** @return ?array{string, string} the class and name texts an id expression is built from */
    private function parts(Expr $v, int $depth): ?array
    {
        // the enclosing function's own split parameter
        if ($v instanceof Expr\Variable && $this->own !== null && $v->name === $this->own[2]) {
            return ['$' . $this->own[3][0], '$' . $this->own[3][1]];
        }
        $ops = $this->concatParts($v);
        if ($ops !== null) {
            $before = [];
            $after = [];
            $seen = false;
            foreach ($ops as $p) {
                if (!$seen && $p instanceof String_ && str_contains($p->value, self::SEP)) {
                    [$l, $r] = explode(self::SEP, $p->value, 2);
                    if ($l !== '') {
                        $before[] = var_export($l, true);
                    }
                    if ($r !== '') {
                        $after[] = var_export($r, true);
                    }
                    $seen = true;
                    continue;
                }
                $txt = $p instanceof String_ ? var_export($p->value, true) : ($p instanceof Expr\Variable || $p instanceof Expr\PropertyFetch
                    || $p instanceof Expr\StaticCall || $p instanceof Expr\MethodCall || $p instanceof Expr\FuncCall
                    ? $this->text($p) : '(' . $this->text($p) . ')');
                if ($p instanceof String_ && $p->value === '') {
                    continue;
                }
                if ($seen) {
                    $after[] = $txt;
                } else {
                    $before[] = $txt;
                }
            }
            if (!$seen || $before === [] || $after === []) {
                return null;
            }
            return [implode(' . ', $before), implode(' . ', $after)];
        }
        if ($v instanceof Expr\Variable && is_string($v->name) && $depth < 2) {
            $defs = [];
            foreach ($this->order as $n) {
                if ($n instanceof Expr\Assign && $n->var instanceof Expr\Variable && $n->var->name === $v->name
                    && $n->getEndFilePos() < $v->getStartFilePos()
                ) {
                    $defs[] = $n;
                }
            }
            usort($defs, static fn(Expr\Assign $a, Expr\Assign $b): int => $b->getStartFilePos() <=> $a->getStartFilePos());
            if ($defs !== []) {
                // the parts must not be reassigned between the definition and the use
                $p = $this->parts($defs[0]->expr, $depth + 1);
                if ($p !== null && !$this->reassignedBetween($defs[0]->expr, $defs[0]->getEndFilePos(), $v->getStartFilePos())) {
                    return $p;
                }
            }
        }
        return null;
    }

    private function reassignedBetween(Expr $def, int $from, int $to): bool
    {
        $vars = [];
        foreach ((new \PhpParser\NodeFinder())->find($def, static fn(Node $x): bool => $x instanceof Expr\Variable && is_string($x->name)) as $x) {
            $vars[$x->name] = true;
        }
        foreach ($this->order as $n) {
            if (($n instanceof Expr\Assign || $n instanceof Expr\AssignOp || $n instanceof Expr\AssignRef)
                && $n->var instanceof Expr\Variable && isset($vars[$n->var->name])
                && $n->getStartFilePos() > $from && $n->getStartFilePos() < $to
            ) {
                return true;
            }
            if ($n instanceof Stmt\Foreach_ && $n->getStartFilePos() > $from && $n->getStartFilePos() < $to) {
                foreach ([$n->keyVar, $n->valueVar] as $fv) {
                    if ($fv instanceof Expr\Variable && isset($vars[$fv->name])) {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    /** @return ?list<Expr> */
    private function concatParts(Expr $e): ?array
    {
        if ($e instanceof Expr\BinaryOp\Concat) {
            return [...($this->concatParts($e->left) ?? [$e->left]), ...($this->concatParts($e->right) ?? [$e->right])];
        }
        if ($e instanceof \PhpParser\Node\Scalar\InterpolatedString) {
            $out = [];
            foreach ($e->parts as $p) {
                $out[] = $p instanceof \PhpParser\Node\InterpolatedStringPart ? new String_($p->value) : $p;
            }
            return $out;
        }
        return null;
    }
}
