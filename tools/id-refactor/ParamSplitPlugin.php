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
    /** @var array<string, true> locals held as their two parts (every definition is `X . '::$' . Y`) */
    private array $split_locals = [];
    /** @var array<string, true> the function's parameters (not locals: callers pass them) */
    private array $param_names = [];

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
            if ($storage instanceof MethodStorage && $storage->declaring_class !== null) {
                $h->self_class = Interner::lookupOrNull($storage->declaring_class);
                if ($stmt instanceof Stmt\ClassMethod) {
                    foreach (self::cfg() as $c) {
                        if (strcasecmp($c[1], $stmt->name->name) === 0 && self::isClass($h->codebase, $h->self_class, $c[0])) {
                            $h->own = $c;
                        }
                    }
                }
            }
            foreach ($stmt->getParams() as $prm) {
                if ($prm->var instanceof Expr\Variable && is_string($prm->var->name)) {
                    $h->param_names[$prm->var->name] = true;
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
        // one parameter per line, as the declaration had it
        $ls = strrpos(substr($this->src, 0, $param->getStartFilePos()), "\n") + 1;
        $sep = $ls < $m->name->getEndFilePos() ? ', ' : ",\n" . str_repeat(' ', strspn($this->src, ' ', $ls));
        $this->edit($param, 'string $' . $cls . $sep . 'string $' . $name);
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

    /**
     * Locals passed to a split parameter whose every definition is a statement `$v = X . '::$' . Y;`: they are held as
     * `$v_class` and `$v_name` (the parts pzoom passes), a string use reads `$v_class . '::$' . $v_name`.
     */
    private function splitLocals(): void
    {
        $cands = [];
        foreach ($this->order as $n) {
            if ($n instanceof Arg && $n->value instanceof Expr\Variable && is_string($n->value->name)
                && $this->splitOf($this->parent[$n], $n) !== null
                && ($this->own === null || $n->value->name !== $this->own[2])
            ) {
                $cands[$n->value->name] = true;
            }
        }
        $cands = array_diff_key($cands, $this->param_names);
        // copies of candidates are candidates (`$property_id = $new_property_id;`)
        do {
            $grew = false;
            foreach ($this->order as $n) {
                if ($n instanceof Expr\Assign && $n->var instanceof Expr\Variable && is_string($n->var->name)
                    && $n->expr instanceof Expr\Variable && is_string($n->expr->name)
                    && isset($cands[$n->var->name]) !== isset($cands[$n->expr->name])
                ) {
                    $cands[$n->var->name] = true;
                    $cands[$n->expr->name] = true;
                    $grew = true;
                }
            }
        } while ($grew);
        $cands = array_diff_key($cands, $this->param_names);
        // drop every candidate with a definition or use that cannot be held as two parts (to a fixpoint: a copy of a
        // dropped candidate drops too)
        $defs = [];
        foreach (array_keys($cands) as $v) {
            foreach ($this->order as $n) {
                if (!$n instanceof Expr\Variable || $n->name !== $v) {
                    continue;
                }
                $p = $this->parent[$n];
                if ($p instanceof Expr\Assign && $p->var === $n) {
                    $defs[$v][] = $p;
                    if (!$this->parent[$p] instanceof Stmt\Expression) {
                        unset($cands[$v]);
                    }
                } elseif ((($p instanceof Expr\AssignOp || $p instanceof Expr\AssignRef) && $p->var === $n)
                    || $p instanceof Node\Param || $p instanceof Expr\ClosureUse || $p instanceof Expr\Isset_
                    || $p instanceof Stmt\Unset_ || $p instanceof Stmt\Foreach_ || $p instanceof Stmt\Global_
                    || $p instanceof Stmt\Static_ || $p instanceof Node\ArrayItem
                    || ($p instanceof Arg && ($p->byRef || $this->byRefArg($this->parent[$p], $p)))
                ) {
                    unset($cands[$v]);
                }
            }
            foreach ($this->order as $n) {
                if ($n instanceof Expr\ArrowFunction || $n instanceof Expr\Closure) {
                    foreach ((new \PhpParser\NodeFinder())->find($n, static fn(Node $x): bool => $x instanceof Expr\Variable && $x->name === $v) as $_) {
                        unset($cands[$v]);
                    }
                }
            }
            if (!isset($defs[$v])) {
                unset($cands[$v]);
            }
        }
        do {
            $dropped = false;
            foreach (array_keys($cands) as $v) {
                foreach ($defs[$v] as $d) {
                    $copy = $d->expr instanceof Expr\Variable && is_string($d->expr->name);
                    if (($copy && !isset($cands[$d->expr->name])) || (!$copy && $this->parts($d->expr, 99) === null)) {
                        unset($cands[$v]);
                        $dropped = true;
                        break;
                    }
                }
            }
        } while ($dropped);
        $this->split_locals = $cands;
        foreach (array_keys($cands) as $v) {
            foreach ($defs[$v] as $d) {
                if ($d->expr instanceof Expr\Variable && is_string($d->expr->name)) {
                    $w = $d->expr->name;
                    $this->edit($d, '$' . $v . '_class = $' . $w . '_class; $' . $v . '_name = $' . $w . '_name');
                    continue;
                }
                [$c, $nm] = $this->parts($d->expr, 99);
                $this->edit($d, '$' . $v . '_class = ' . $c . '; $' . $v . '_name = ' . $nm);
            }
            foreach ($this->order as $n) {
                if (!$n instanceof Expr\Variable || $n->name !== $v) {
                    continue;
                }
                $p = $this->parent[$n];
                if ($p instanceof Expr\Assign && ($p->var === $n || ($p->expr === $n && $p->var instanceof Expr\Variable && isset($cands[$p->var->name])))) {
                    continue;
                }
                if ($p instanceof Arg && $this->splitOf($this->parent[$p], $p) !== null) {
                    continue; // calls() passes the parts
                }
                $this->replaceRead($n, '($' . $v . '_class . \'' . self::SEP . '\' . $' . $v . '_name)');
            }
        }
    }

    /** Whether the callee takes this argument by reference (a user method or function). */
    private function byRefArg(Node $call, Arg $arg): bool
    {
        try {
            $params = null;
            if (($call instanceof Expr\MethodCall || $call instanceof Expr\NullsafeMethodCall) && $call->name instanceof Identifier) {
                foreach ($this->types->getType($call->var)?->getAtomicTypes() ?? [] as $a) {
                    if ($a instanceof TNamedObject) {
                        $mid = $this->codebase->methods->getDeclaringMethodId(new \Psalm\Internal\MethodIdentifier($a->name, Interner::intern(strtolower($call->name->name))));
                        $params = $mid !== null ? $this->codebase->methods->getStorage($mid)->params : $params;
                    }
                }
            } elseif ($call instanceof Expr\StaticCall && $call->name instanceof Identifier && $call->class instanceof Name) {
                $n = strtolower($call->class->toString());
                $cls = in_array($n, ['self', 'static', 'parent'], true) ? $this->self_class
                    : (Interner::lookupOrNull($call->class->attrs()->resolvedId) ?? $call->class->toString());
                if ($cls !== null) {
                    $mid = $this->codebase->methods->getDeclaringMethodId(new \Psalm\Internal\MethodIdentifier(Interner::intern($cls), Interner::intern(strtolower($call->name->name))));
                    $params = $mid !== null ? $this->codebase->methods->getStorage($mid)->params : null;
                }
            } elseif ($call instanceof Expr\FuncCall && $call->name instanceof Name) {
                $fn = strtolower(Interner::lookupOrNull($call->name->attrs()->resolvedId) ?? $call->name->toString());
                if ($this->codebase->functions->functionExists(null, $fn)) {
                    $params = $this->codebase->functions->getStorage(null, $fn)->params;
                }
            }
            if ($params === null) {
                return !($call instanceof Expr\MethodCall || $call instanceof Expr\StaticCall || $call instanceof Expr\FuncCall || $call instanceof Expr\New_)
                    ? false : false;
            }
            foreach ($call->getArgs() as $i => $a) {
                if ($a === $arg) {
                    $ps = $a->name !== null ? null : ($params[$i] ?? end($params) ?: null);
                    if ($a->name !== null) {
                        foreach ($params as $pp) {
                            if ($pp->name === $a->name->name) {
                                $ps = $pp;
                            }
                        }
                    }
                    return $ps !== null && $ps->by_ref;
                }
            }
        } catch (Throwable) {
        }
        return false;
    }

    private function calls(): void
    {
        $this->splitLocals();
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
        if ($v instanceof Expr\Variable && is_string($v->name) && isset($this->split_locals[$v->name])) {
            return ['$' . $v->name . '_class', '$' . $v->name . '_name'];
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
                // concatenation converted it to a string (an identifier node, an int)
                $pt = $p instanceof String_ ? null : $this->types->getType($p);
                if (!$p instanceof String_ && ($pt === null || !$pt->isString())) {
                    $txt = '(string) ' . $txt;
                }
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
            if ($defs !== [] && $this->dominates($defs[0], $v) && !$this->redefinedInLoop($v)) {
                // the parts must not be reassigned between the definition and the use
                $p = $this->parts($defs[0]->expr, $depth + 1);
                if ($p !== null && !$this->reassignedBetween($defs[0]->expr, $defs[0]->getEndFilePos(), $v->getStartFilePos())) {
                    return $p;
                }
            }
        }
        return null;
    }

    /**
     * Whether a definition statement runs before the use on every path: it is a statement of a block (a `stmts`
     * list) that also holds, directly or nested, the use.
     */
    private function dominates(Expr\Assign $def, Expr $use): bool
    {
        $stmt = $this->parent[$def] ?? null;
        if (!$stmt instanceof Stmt\Expression) {
            return false;
        }
        $owner = $this->parent[$stmt] ?? null;
        if ($owner === null || !property_exists($owner, 'stmts') || !is_array($owner->stmts) || !in_array($stmt, $owner->stmts, true)) {
            return false;
        }
        for ($n = $use; isset($this->parent[$n]); $n = $this->parent[$n]) {
            if ($this->parent[$n] === $owner) {
                return in_array($n, $owner->stmts, true);
            }
        }
        return false;
    }

    /** Whether the use sits in a loop that assigns the variable again after the use (a later iteration's value). */
    private function redefinedInLoop(Expr\Variable $use): bool
    {
        for ($n = $use; isset($this->parent[$n]); $n = $this->parent[$n]) {
            $l = $this->parent[$n];
            if ($l instanceof Stmt\Foreach_ || $l instanceof Stmt\For_ || $l instanceof Stmt\While_ || $l instanceof Stmt\Do_) {
                foreach ($this->order as $a) {
                    if ($a instanceof Expr\Assign && $a->var instanceof Expr\Variable && $a->var->name === $use->name
                        && $a->getStartFilePos() > $use->getEndFilePos() && $a->getEndFilePos() <= $l->getEndFilePos()
                    ) {
                        return true;
                    }
                }
            }
        }
        return false;
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
