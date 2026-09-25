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
use Psalm\Type\Atomic\TNamedObject;
use SimpleXMLElement;
use SplObjectStorage;
use Throwable;

/**
 * Turns name maps (`array<lowercase-string, string>`: lowercased name => cased name) into id sets
 * (`array<int, true>`, pzoom's `IndexSet<StrId>` / `Vec<StrId>`), and drops their id twins.
 *
 * Config (env MAP_ID_SETS, JSON): {"props": [[class, prop]], "methods": [[class, method]], "twins": [[class, twin, prop]]}.
 * A set expression is a configured property on a value of that class, a call of a configured method, a local
 * variable every assignment of which is a set expression, `A + B` / `A ?: B` of sets, or `[...A, ...B]` /
 * array_merge(A, B) of sets (rewritten to `A + B`: int keys would be renumbered). Consumers:
 *   isset(S[K]) / unset(S[K]) / S[K] = V   K (a lowercased name) becomes the cased name's id; V becomes `true`
 *   foreach (S as $k => $v)                 `$k` binds the id: its uses read `strtolower(Interner::lookup($k))` (or `$k`
 *                                           where an id is wanted), the value variable's uses `Interner::lookup($k)`
 *   in_array(V, S)                          isset(S[<V's id>])
 *   array_values(S) / array_keys(S)         the names / lowercased names of array_keys(S)
 *   reset(S)                                Interner::lookup(array_key_first(S))
 *   truthiness, count(), assignment, `return` from a configured method, passing to a configured method: unchanged
 * Twin reads (`X->twin`) read the property itself; `X->twin = <anything>;` statements are removed.
 * Everything else is reported as `manual`. MapIdSetDeclPlugin changes the declarations.
 */
final class MapIdSetPlugin implements PluginEntryPointInterface, AfterFunctionLikeAnalysisInterface
{
    /** @var ?array{props: list<array{string, string}>, methods: list<array{string, string}>, twins: list<array{string, string, string}>} */
    private static ?array $cfg = null;

    private Codebase $codebase;
    private NodeTypeProvider $types;
    private string $file;
    private string $src;
    /** @var SplObjectStorage<Node, Node> */
    private SplObjectStorage $parent;
    /** @var list<Node> */
    private array $order = [];
    /** @var array<string, bool> local variable => every assignment is a set */
    private array $set_locals = [];
    /** @var list<array{int, int, string}> */
    private array $edits = [];
    /** @var SplObjectStorage<Node, true> */
    private SplObjectStorage $done;
    /** @var array<string, string> a loop variable bound over a set => the variable now holding its id */
    private array $loop_ids = [];
    /** @var array<string, true> local maps keyed by set ids */
    private array $id_keyed = [];
    private bool $quiet = false;

    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    /** @return array{props: list<array{string, string}>, methods: list<array{string, string}>, twins: list<array{string, string, string}>} */
    public static function cfg(): array
    {
        return self::$cfg ??= (json_decode((string) getenv('MAP_ID_SETS'), true) ?: []) + ['props' => [], 'methods' => [], 'twins' => []];
    }

    public static function inScope(string $file): bool
    {
        $root = dirname(__DIR__, 2);
        $extra = array_filter(explode(':', (string) getenv('ID_REFACTOR_ROOTS')));
        return str_starts_with($file, $root . '/src/') || str_starts_with($file, $root . '/tests/')
            || str_starts_with($file, $root . '/examples/')
            || array_filter($extra, static fn(string $r): bool => str_starts_with($file, $r)) !== [];
    }

    /** @param array<string, mixed> $row */
    public static function out(array $row): void
    {
        file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/map-id-set.jsonl',
            json_encode($row, JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
    }

    // ------------------------------------------------------------------ bodies
    public static function afterStatementAnalysis(AfterFunctionLikeAnalysisEvent $event): ?bool
    {
        $file = $event->getStatementsSource()->getFilePath();
        if (!self::inScope($file)) {
            return null;
        }
        try {
            require_once __DIR__ . '/SymNames.php';
            $h = new self();
            $h->codebase = $event->getCodebase();
            $h->types = $event->getNodeTypeProvider();
            $h->file = $file;
            $h->src = (string) file_get_contents($file);
            $h->parent = new SplObjectStorage();
            $h->done = new SplObjectStorage();
            $stmt = $event->getStmt();
            $body = $stmt instanceof Expr\ArrowFunction ? [$stmt->expr] : ($stmt->getStmts() ?? []);
            $h->index($body, $stmt);
            $h->findSetLocals();
            $h->quiet = true;
            $h->walk();
            $h->quiet = false;
            // second pass: with the loop variables known, local maps keyed through them become id-keyed
            $loop_ids = $h->loop_ids;
            $h->edits = [];
            $h->done = new SplObjectStorage();
            $h->loop_ids = $loop_ids;
            $h->findIdKeyed();
            $h->walk();
            if ($h->edits !== []) {
                self::out(['kind' => 'edit', 'file' => $file, 'site' => $file . ':' . $stmt->getStartFilePos(), 'edits' => $h->edits]);
            }
        } catch (Throwable $e) {
            self::out(['kind' => 'error', 'msg' => $e->getMessage() . ' @' . $file . ':' . $e->getLine()]);
        }
        return null;
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
        if ($this->quiet) {
            return;
        }
        self::out(['kind' => 'manual', 'site' => $this->file . ':' . $n->getStartLine(), 'why' => $why]);
    }

    private function receiverIs(Expr $recv, string $class): bool
    {
        if ($recv instanceof Expr\Variable && $recv->name === 'this') {
            $t = $this->types->getType($recv);
            if ($t === null) {
                return false;
            }
        }
        $t = $this->types->getType($recv);
        if ($t === null) {
            return false;
        }
        $any = false;
        foreach ($t->getAtomicTypes() as $a) {
            if ($a instanceof \Psalm\Type\Atomic\TNull) {
                continue;
            }
            if (!$a instanceof TNamedObject) {
                return false;
            }
            $n = SymNames::named($a);
            if (strcasecmp($n, $class) !== 0
                && !$this->codebase->classExtendsOrImplements(Interner::intern($n), Interner::intern($class))
            ) {
                return false;
            }
            $any = true;
        }
        return $any;
    }

    /** The configured property this fetch reads, or null. */
    private function isPropSet(Expr $e): bool
    {
        if (!($e instanceof Expr\PropertyFetch || $e instanceof Expr\NullsafePropertyFetch) || !$e->name instanceof Identifier) {
            return false;
        }
        foreach (self::cfg()['props'] as [$c, $p]) {
            if ($e->name->name === $p && $this->receiverIs($e->var, $c)) {
                return true;
            }
        }
        return false;
    }

    /** The twin's property for a twin fetch, or null. */
    private function twinOf(Expr $e): ?string
    {
        if (!$e instanceof Expr\PropertyFetch || !$e->name instanceof Identifier) {
            return null;
        }
        foreach (self::cfg()['twins'] as [$c, $twin, $p]) {
            if ($e->name->name === $twin && $this->receiverIs($e->var, $c)) {
                return $p;
            }
        }
        return null;
    }

    private function isMethodSet(Expr $e): bool
    {
        if (!($e instanceof Expr\MethodCall || $e instanceof Expr\NullsafeMethodCall) || !$e->name instanceof Identifier) {
            return false;
        }
        foreach (self::cfg()['methods'] as [$c, $m]) {
            if (strcasecmp($e->name->name, $m) === 0 && $this->receiverIs($e->var, $c)) {
                return true;
            }
        }
        return false;
    }

    /** Whether an expression evaluates to a converted set. */
    private function isSet(Expr $e): bool
    {
        if ($this->isPropSet($e) || $this->isMethodSet($e)) {
            return true;
        }
        if ($e instanceof Expr\Variable && is_string($e->name) && ($this->set_locals[$e->name] ?? false)) {
            return true;
        }
        if ($e instanceof Expr\BinaryOp\Plus || ($e instanceof Expr\Ternary && $e->if === null)) {
            $l = $e instanceof Expr\Ternary ? $e->cond : $e->left;
            $r = $e instanceof Expr\Ternary ? $e->else : $e->right;
            return $this->operandsAreSets([$l, $r]);
        }
        if ($e instanceof Expr\Array_ && $e->items !== []) {
            $ops = [];
            foreach ($e->items as $it) {
                if ($it === null || !$it->unpack) {
                    return false;
                }
                $ops[] = $it->value;
            }
            return $this->operandsAreSets($ops);
        }
        if ($e instanceof Expr\FuncCall && $e->name instanceof Name && strtolower($e->name->toString()) === 'array_merge'
            && $e->getArgs() !== []
        ) {
            $ops = [];
            foreach ($e->getArgs() as $a) {
                if ($a->unpack) {
                    return false;
                }
                $ops[] = $a->value;
            }
            return $this->operandsAreSets($ops);
        }
        // array_filter(S, fn(string $name) => ...): filtered on the ids
        if ($e instanceof Expr\FuncCall && $e->name instanceof Name && strtolower($e->name->toString()) === 'array_filter'
            && count($e->getArgs()) === 2 && $this->isSet($e->getArgs()[0]->value)
            && ($cb = $e->getArgs()[1]->value) instanceof Expr\ArrowFunction && count($cb->params) === 1
        ) {
            return true;
        }
        return false;
    }

    /** Operands of a union: sets, and at least one set with the rest name-map literals (`[lc => Name]`). */
    private function operandsAreSets(array $ops): bool
    {
        $any = false;
        foreach ($ops as $o) {
            if ($this->isSet($o)) {
                $any = true;
            } elseif (!$this->isNameLiteral($o)) {
                return false;
            }
        }
        return $any;
    }

    private function isNameLiteral(Expr $e): bool
    {
        if (!$e instanceof Expr\Array_ || $e->items === []) {
            return false;
        }
        foreach ($e->items as $it) {
            if ($it === null || $it->unpack || $it->key === null || $it->byRef) {
                return false;
            }
        }
        return true;
    }

    /** The rewritten source of a set expression (unions normalized to `+`, name literals to id literals). */
    private function setText(Expr $e): string
    {
        if ($this->isNameLiteral($e)) {
            /** @var Expr\Array_ $e */
            return '[' . implode(', ', array_map(fn($it) => $this->idOfName($it->value) . ' => true', $e->items)) . ']';
        }
        if ($e instanceof Expr\BinaryOp\Plus) {
            return $this->setText($e->left) . ' + ' . $this->setText($e->right);
        }
        if ($e instanceof Expr\Ternary && $e->if === null) {
            return $this->setText($e->cond) . ' ?: ' . $this->setText($e->else);
        }
        if ($e instanceof Expr\Array_) {
            return '(' . implode(' + ', array_map(fn($it) => $this->setText($it->value), $e->items)) . ')';
        }
        if ($e instanceof Expr\FuncCall && $e->name instanceof Name && strtolower($e->name->toString()) === 'array_merge') {
            return '(' . implode(' + ', array_map(fn(Arg $a) => $this->setText($a->value), $e->getArgs())) . ')';
        }
        if ($e instanceof Expr\FuncCall && $e->name instanceof Name && strtolower($e->name->toString()) === 'array_filter') {
            /** @var Expr\ArrowFunction $cb */
            $cb = $e->getArgs()[1]->value;
            // over the id list, back to a set (array_filter's key callback is typed array-key)
            return 'array_fill_keys(array_filter(array_keys(' . $this->setText($e->getArgs()[0]->value) . '), ' . $this->keyCallback($cb) . '), true)';
        }
        return $this->text($e);
    }

    /** `fn(string $n) => B` over names as `fn(int $n) => B'` over ids. */
    private function keyCallback(Expr\ArrowFunction $cb): string
    {
        $param = $cb->params[0];
        $name = $param->var->name;
        $base = $cb->getStartFilePos();
        $t = $this->text($cb);
        $edits = [];
        if ($param->type !== null) {
            $edits[] = [$param->type->getStartFilePos(), $param->type->getEndFilePos() + 1, 'int'];
        }
        $walk = function (Node $n, ?Node $parent, ?Node $gp) use (&$walk, &$edits, $name): void {
            if ($n instanceof Expr\Variable && $n->name === $name && $parent instanceof Arg && $gp instanceof Expr\StaticCall
                && $gp->class instanceof Name && strtolower($gp->class->getLast()) === 'interner' && $gp->name instanceof Identifier
                && strtolower($gp->name->name) === 'intern'
            ) {
                $edits[] = [$gp->getStartFilePos(), $gp->getEndFilePos() + 1, '$' . $name];
                return;
            }
            if ($n instanceof Expr\Variable && $n->name === $name && !$parent instanceof Node\Param) {
                $edits[] = [$n->getStartFilePos(), $n->getEndFilePos() + 1, 'Interner::lookup($' . $name . ')'];
                return;
            }
            foreach ($n->getSubNodeNames() as $sub) {
                foreach (is_array($n->$sub) ? $n->$sub : [$n->$sub] as $c) {
                    if ($c instanceof Node) {
                        $walk($c, $n, $parent);
                    }
                }
            }
        };
        $walk($cb->expr, $cb, null);
        usort($edits, fn($a, $b) => $b[0] <=> $a[0]);
        $prev = PHP_INT_MAX;
        foreach ($edits as [$s0, $e0, $r]) {
            if ($e0 > $prev) {
                continue; // nested in an edit already made
            }
            $t = substr($t, 0, $s0 - $base) . $r . substr($t, $e0 - $base);
            $prev = $s0;
        }
        return $t;
    }

    /** Locals assigned only from sets (and written only through `$l[K] = V`). */
    private function findSetLocals(): void
    {
        $cand = [];
        for ($pass = 0; $pass < 3; $pass++) {
            $cand = [];
            foreach ($this->order as $n) {
                if ($n instanceof Expr\Assign && $n->var instanceof Expr\Variable && is_string($n->var->name)) {
                    $cand[$n->var->name] = ($cand[$n->var->name] ?? true) && $this->isSet($n->expr);
                } elseif (($n instanceof Expr\AssignRef || $n instanceof Expr\AssignOp)
                    && $n->var instanceof Expr\Variable && is_string($n->var->name)
                ) {
                    $cand[$n->var->name] = false;
                } elseif ($n instanceof Stmt\Foreach_ && $n->valueVar instanceof Expr\Variable && is_string($n->valueVar->name)) {
                    $cand[$n->valueVar->name] = false;
                }
            }
            $this->set_locals = array_filter($cand);
        }
    }

    /** A cased name expression's id text. */
    private function idOfName(Expr $v): ?string
    {
        if ($v instanceof Expr\Variable && is_string($v->name) && isset($this->loop_ids[$v->name])) {
            return '$' . $this->loop_ids[$v->name];
        }
        if ($v instanceof String_) {
            return $this->symFor($v->value);
        }
        if ($v instanceof Expr\ClassConstFetch && $v->class instanceof Name && $v->name instanceof Identifier
            && strtolower($v->name->name) === 'class' && !in_array(strtolower($v->class->toString()), ['self', 'static', 'parent'], true)
        ) {
            return $this->symFor((string) (Interner::lookupOrNull($v->class->attrs()->resolvedId) ?? $v->class->toString()));
        }
        if ($v instanceof Expr\StaticCall && $v->class instanceof Name && strtolower($v->class->getLast()) === 'interner'
            && $v->name instanceof Identifier && strtolower($v->name->name) === 'lookup' && count($v->getArgs()) === 1
        ) {
            return $this->text($v->getArgs()[0]->value);
        }
        return 'Interner::intern(' . $this->text($v) . ')';
    }

    private function symFor(string $name): string
    {
        if (!str_starts_with($this->file, dirname(__DIR__, 2) . '/src/')) {
            return 'Interner::intern(' . var_export(ltrim($name, '\\'), true) . ')';
        }
        [$text, $new] = SymNames::forLiteral($name);
        if ($new !== null) {
            self::out(['kind' => 'sym', 'name' => $new[0], 'value' => $new[1], 'literal' => $new[2]]);
        }
        return $text;
    }

    /** The cased class name a lowercased literal names (from the codebase), or null. */
    private function casedOf(string $lc): ?string
    {
        $s = $this->codebase->classlike_storage_provider->find(Interner::intern($lc));
        if ($s !== null) {
            return Interner::lookup($s->id);
        }
        if (class_exists($lc) || interface_exists($lc) || trait_exists($lc) || enum_exists($lc)) {
            return (new \ReflectionClass($lc))->getName();
        }
        // declared in a stub (an extension that is not loaded here)
        $parts = explode('\\', $lc);
        $short = array_pop($parts);
        $ns = implode('\\', $parts);
        foreach (new \RecursiveIteratorIterator(new \RecursiveDirectoryIterator(dirname(__DIR__, 2) . '/stubs')) as $f) {
            if (!$f->isFile() || !preg_match('/\.php(stub)?$/', $f->getFilename())) {
                continue;
            }
            $src = (string) file_get_contents($f->getPathname());
            if (preg_match('/^\s*(?:final\s+|abstract\s+|readonly\s+)*(?:class|interface|trait|enum)\s+(' . preg_quote($short, '/') . ')\b/mi', $src, $m)
                && preg_match('/^namespace\s+(' . str_replace('\\\\', '\\\\', preg_quote($ns, '/')) . ')\s*[;{]/mi', $src, $n) === ($ns === '' ? 0 : 1)
            ) {
                return ($ns === '' ? '' : $n[1] . '\\') . $m[1];
            }
        }
        return null;
    }

    /** A lowercased-name key expression's id text, or null when it cannot be derived. */
    private function idOfKey(Expr $k, Node $at): ?string
    {
        if ($k instanceof String_) {
            $c = $this->casedOf($k->value);
            return $c !== null ? $this->symFor($c) : null;
        }
        if ($k instanceof Expr\FuncCall && $k->name instanceof Name && strtolower($k->name->toString()) === 'strtolower'
            && count($k->getArgs()) === 1
        ) {
            return $this->idOfName($k->getArgs()[0]->value);
        }
        if ($k instanceof Expr\Variable && is_string($k->name)) {
            // a foreach key bound over a converted set: already the id
            foreach ($this->order as $n) {
                if ($n instanceof Stmt\Foreach_ && $n->keyVar instanceof Expr\Variable && $n->keyVar->name === $k->name
                    && $this->isSet($n->expr)
                ) {
                    return '$' . $k->name;
                }
            }
            // a foreach key over a local map keyed by set ids
            foreach ($this->order as $n) {
                if ($n instanceof Stmt\Foreach_ && $n->keyVar instanceof Expr\Variable && $n->keyVar->name === $k->name
                    && $n->expr instanceof Expr\Variable && isset($this->id_keyed[$n->expr->name])
                ) {
                    return '$' . $k->name;
                }
            }
            // the single assignment `$k = strtolower(E)` before this use
            $def = null;
            $count = 0;
            foreach ($this->order as $n) {
                if ($n instanceof Expr\Assign && $n->var instanceof Expr\Variable && $n->var->name === $k->name) {
                    $count++;
                    $def = $n;
                }
            }
            if ($count === 1 && $def !== null && $def->getStartFilePos() < $at->getStartFilePos()
                && $def->expr instanceof Expr\StaticCall && $def->expr->class instanceof Name
                && strtolower($def->expr->class->getLast()) === 'interner' && $def->expr->name instanceof Identifier
                && strtolower($def->expr->name->name) === 'lookup' && count($def->expr->getArgs()) === 1
            ) {
                return $this->text($def->expr->getArgs()[0]->value);
            }
            if ($count === 1 && $def !== null && $def->getStartFilePos() < $at->getStartFilePos()) {
                $e = $def->expr;
                if ($e instanceof Expr\FuncCall && $e->name instanceof Name && strtolower($e->name->toString()) === 'strtolower'
                    && count($e->getArgs()) === 1
                ) {
                    $src = $e->getArgs()[0]->value;
                    if ($src instanceof Expr\Variable || $src instanceof String_ || $src instanceof Expr\StaticCall
                        || $src instanceof Expr\ClassConstFetch || $src instanceof Expr\PropertyFetch
                        || $src instanceof Expr\BinaryOp\Concat || $src instanceof Expr\ArrayDimFetch
                    ) {
                        return $this->idOfName($src);
                    }
                }
            }
            // a lowercased name cut from a member id string ("class::member"): the declared class's id, through the
            // storage provider (the boundary until member ids are id pairs)
            if ($count === 1 && $def !== null && $def->expr instanceof Expr\ArrayDimFetch
                && $def->expr->var instanceof Expr\FuncCall && $def->expr->var->name instanceof Name
                && strtolower($def->expr->var->name->toString()) === 'explode' && $this->hasCodebaseVar()
            ) {
                $this->manual($at, 'boundary: member id string (auto)');
                return '$codebase->classlike_storage_provider->canonicalId(Interner::intern(' . $this->text($k) . '))';
            }
        }
        return null;
    }

    private function hasCodebaseVar(): bool
    {
        foreach ($this->order as $n) {
            if ($n instanceof Expr\Variable && $n->name === 'codebase') {
                return true;
            }
        }
        return false;
    }

    /**
     * Local maps written with `$m[strtolower($v)] = ...` where `$v` is bound over a set: keyed by that id instead
     * (their foreach keys are then ids).
     */
    private function findIdKeyed(): void
    {
        foreach ($this->order as $n) {
            if ($n instanceof Expr\Assign && $n->var instanceof Expr\ArrayDimFetch && $n->var->var instanceof Expr\Variable
                && is_string($n->var->var->name) && ($d = $n->var->dim) instanceof Expr\FuncCall && $d->name instanceof Name
                && strtolower($d->name->toString()) === 'strtolower' && count($d->getArgs()) === 1
                && ($v = $d->getArgs()[0]->value) instanceof Expr\Variable && is_string($v->name) && isset($this->loop_ids[$v->name])
            ) {
                $this->id_keyed[$n->var->var->name] = true;
                $this->edit($d, '$' . $this->loop_ids[$v->name]);
            }
        }
    }

    private function walk(): void
    {
        foreach ($this->order as $n) {
            if (!$n instanceof Expr || isset($this->done[$n])) {
                continue;
            }
            if (($twin = $this->twinOf($n)) !== null) {
                $p = $this->parent[$n];
                if ($p instanceof Expr\Assign && $p->var === $n) {
                    $st = $this->parent[$p];
                    if ($st instanceof Stmt\Expression) {
                        $s = strrpos(substr($this->src, 0, $st->getStartFilePos()), "\n") + 1;
                        $this->edits[] = [$s, strpos($this->src, "\n", $st->getEndFilePos()) + 1, ''];
                        $this->done[$p->expr] = true;
                        continue;
                    }
                }
                /** @var Expr\PropertyFetch $n */
                $this->edits[] = [$n->name->getStartFilePos(), $n->name->getEndFilePos() + 1, $twin];
                continue;
            }
            if (!$this->isSet($n)) {
                continue;
            }
            $this->consume($n);
        }
    }

    /** Marks a set expression's sub-expressions handled (a `+` of sets is consumed once, as a whole). */
    private function markDone(Expr $e): void
    {
        $this->done[$e] = true;
        foreach ($e->getSubNodeNames() as $sub) {
            $v = $e->$sub;
            foreach (is_array($v) ? $v : [$v] as $c) {
                if ($c instanceof Expr) {
                    $this->markDone($c);
                } elseif ($c instanceof Node\ArrayItem || $c instanceof Arg) {
                    $this->markDone($c->value);
                }
            }
        }
    }

    private function consume(Expr $s): void
    {
        $this->markDone($s);
        // the set's rewritten text replaces it where the expression itself stays (keep())
        $st = $this->setText($s);
        $keep = function () use ($s, $st): void {
            if ($st !== $this->text($s)) {
                $this->edit($s, $st);
            }
        };
        $p = $this->parent[$s];
        // plain, shape-preserving contexts
        if ($p instanceof Expr\BinaryOp\Plus || ($p instanceof Expr\Ternary && $p->if === null)) {
            $keep();
            return; // the enclosing set is consumed itself
        }
        if (($p instanceof Expr\Assign || $p instanceof Expr\AssignOp\Plus) && $p->expr === $s) {
            if (($p->var instanceof Expr\Variable && ($this->set_locals[$p->var->name] ?? false)) || $this->isPropSet($p->var)) {
                $keep();
                return;
            }
        }
        if ($p instanceof Expr\Assign && $p->var === $s) {
            return; // `S = <set>` (checked from the other side), or `S = []`
        }
        if ($p instanceof Expr\BooleanNot || $p instanceof Expr\Empty_ || $p instanceof Stmt\If_ || $p instanceof Expr\BinaryOp\BooleanAnd
            || $p instanceof Expr\BinaryOp\BooleanOr || $p instanceof Stmt\ElseIf_ || ($p instanceof Expr\Ternary && $p->cond === $s)
        ) {
            $keep();
            return;
        }
        if ($p instanceof Arg) {
            $call = $this->parent[$p];
            if ($call instanceof Expr\FuncCall && $call->name instanceof Name) {
                $fn = strtolower($call->name->toString());
                if ($fn === 'count') {
                    $keep();
                    return;
                }
                if ($fn === 'in_array' && $call->getArgs()[1] ?? null) {
                    if ($call->getArgs()[1]->value === $s) {
                        $id = $this->idOfName($call->getArgs()[0]->value);
                        $this->edit($call, 'isset(' . $st . '[' . $id . '])');
                        $this->markDone($call->getArgs()[0]->value);
                        return;
                    }
                }
                if ($fn === 'array_values' || $fn === 'array_keys') {
                    if (count($call->getArgs()) === 1) {
                        // the lowercased names: the ids are of the cased names (lookupLc is only for lowercase-interned ids)
                        $this->edit($call, $fn === 'array_values'
                            ? 'array_map(Interner::lookup(...), array_keys(' . $st . '))'
                            : 'array_map(static fn(int $id): string => strtolower(Interner::lookup($id)), array_keys(' . $st . '))');
                        return;
                    }
                }
                if ($fn === 'reset') {
                    $this->edit($call, 'Interner::lookup(array_key_first(' . $st . '))');
                    return;
                }
            }
            if ($call instanceof Expr\MethodCall && $this->isTwinDerivation($call)) {
                return;
            }
        }
        if ($p instanceof Stmt\Return_) {
            // from a configured method
            $fn = $p;
            while (isset($this->parent[$fn]) && !($fn instanceof Stmt\ClassMethod)) {
                $fn = $this->parent[$fn];
            }
            if ($fn instanceof Stmt\ClassMethod) {
                foreach (self::cfg()['methods'] as [, $m]) {
                    if (strcasecmp($fn->name->name, $m) === 0) {
                        $keep();
                        return;
                    }
                }
            }
        }
        if (($p instanceof Expr\ArrayDimFetch) && $p->var === $s) {
            $this->dim($p, $st);
            return;
        }
        if ($p instanceof Stmt\Foreach_ && $p->expr === $s) {
            $keep();
            $this->foreach_($p);
            return;
        }
        $this->manual($s, 'context ' . $p->getType());
    }

    /** `$this->nameIdSet(S)` feeding a twin assignment (the statement goes with the twin). */
    private function isTwinDerivation(Expr\MethodCall $call): bool
    {
        $a = $this->parent[$call] ?? null;
        return $a instanceof Expr\Assign && $this->twinOf($a->var) !== null;
    }

    private function dim(Expr\ArrayDimFetch $d, string $set): void
    {
        $p = $this->parent[$d];
        if ($d->dim === null) {
            $this->manual($d, 'append');
            return;
        }
        if ($p instanceof Expr\Assign && $p->var === $d) {
            $id = $p->expr instanceof Expr\ConstFetch && strtolower($p->expr->name->toString()) === 'true'
                ? ($this->idOfKey($d->dim, $d) ?? $this->idOfName($d->dim))
                : $this->idOfName($p->expr);
            $this->edit($p, $set . '[' . $id . '] = true');
            $this->markDone($p->expr);
            $this->markDone($d->dim);
            return;
        }
        $id = $this->idOfKey($d->dim, $d);
        if ($id === null) {
            $this->manual($d, 'key ' . $this->text($d->dim));
            return;
        }
        $this->markDone($d->dim);
        if ($p instanceof Expr\Isset_ || $p instanceof Stmt\Unset_) {
            $this->edit($d, $set . '[' . $id . ']');
            return;
        }
        if ($p instanceof Expr\BinaryOp\Coalesce && $p->left === $d) {
            $this->manual($d, 'coalesce read');
            return;
        }
        // a value read: the cased name, which the set no longer holds (present by the caller's invariant)
        $this->edit($d, '(isset(' . $set . '[' . $id . ']) ? Interner::lookup(' . $id . ') : null)');
    }

    private function foreach_(Stmt\Foreach_ $f): void
    {
        $key = $f->keyVar instanceof Expr\Variable && is_string($f->keyVar->name) ? $f->keyVar->name : null;
        $val = $f->valueVar instanceof Expr\Variable && is_string($f->valueVar->name) ? $f->valueVar->name : null;
        if ($val === null || $f->byRef) {
            $this->manual($f, 'foreach binding');
            return;
        }
        $id_var = $key ?? ($val . '_id');
        // uses of the bound variables in the body
        $assigned = [];
        $uses = [];
        $scan = function (Node $n) use (&$scan, &$assigned, &$uses, $key, $val): void {
            if ($n instanceof Expr\Closure || $n instanceof Expr\ArrowFunction) {
                if ($n instanceof Expr\Closure) {
                    foreach ($n->uses as $u) {
                        if ($u->var->name === $key || $u->var->name === $val) {
                            $uses[] = $u->var;
                        }
                    }
                }
                if ($n instanceof Expr\ArrowFunction) {
                    $scan($n->expr);
                }
                return;
            }
            if ($n instanceof Expr\Variable && ($n->name === $key || $n->name === $val)) {
                $p = $this->parent[$n] ?? null;
                if (($p instanceof Expr\Assign || $p instanceof Expr\AssignOp || $p instanceof Expr\AssignRef) && $p->var === $n) {
                    $assigned[$n->name] = true;
                }
                $uses[] = $n;
            }
            foreach ($n->getSubNodeNames() as $sub) {
                $v = $n->$sub;
                foreach (is_array($v) ? $v : [$v] as $c) {
                    if ($c instanceof Node) {
                        $scan($c);
                    }
                }
            }
        };
        foreach ($f->stmts as $s) {
            $scan($s);
        }
        $binding = '$' . $id_var . ' => $_';
        if ($key !== null && $val === '_') {
            $binding = '$' . $id_var . ' => $_';
        }
        $this->edits[] = [$f->keyVar?->getStartFilePos() ?? $f->valueVar->getStartFilePos(), $f->valueVar->getEndFilePos() + 1, $binding];
        if ($assigned !== [] || array_filter($uses, fn($u) => !$u instanceof Expr\Variable) !== []) {
            // reassigned in the body (or captured): rebind the old names at the top of the body
            $pro = [];
            if ($key !== null && $key !== $id_var) {
                $pro[] = '$' . $key . ' = strtolower(Interner::lookup($' . $id_var . '));';
            }
            if ($val !== '_') {
                $pro[] = '$' . $val . ' = Interner::lookup($' . $id_var . ');';
            }
            $this->manual($f, 'loop variable reassigned: prologue');
            if ($key !== null && isset($assigned[$key])) {
                // the key variable is the id: it cannot also hold its string
                $this->manual($f, 'key reassigned');
                return;
            }
            $open = strpos($this->src, '{', $f->valueVar->getEndFilePos());
            $this->edits[] = [$open + 1, $open + 1, "\n" . str_repeat(' ', 12) . implode(' ', $pro)];
            return;
        }
        $this->loop_ids[$val] = $id_var;
        if ($key !== null) {
            $this->loop_ids[$key] = $id_var;
        }
        foreach ($uses as $u) {
            $is_key = $u->name === $key;
            $p = $this->parent[$u] ?? null;
            $is_id_ctx = false;
            if ($p instanceof Arg) {
                $call = $this->parent[$p];
                if ($call instanceof Expr\StaticCall && $call->class instanceof Name && strtolower($call->class->getLast()) === 'interner'
                    && $call->name instanceof Identifier && strtolower($call->name->name) === 'intern'
                ) {
                    $this->edit($call, '$' . $id_var);
                    continue;
                }
                if ($call instanceof Expr\FuncCall && $call->name instanceof Name && strtolower($call->name->toString()) === 'strtolower') {
                    $gp = $this->parent[$call] ?? null;
                    if ($gp instanceof Expr\ArrayDimFetch && $gp->dim === $call && ($this->isSet($gp->var)
                        || ($gp->var instanceof Expr\Variable && isset($this->id_keyed[$gp->var->name])))
                    ) {
                        continue; // idOfKey sees `strtolower($v)` -> handled by dim() via idOfName
                    }
                }
            }
            if ($p instanceof Expr\ArrayDimFetch && $p->dim === $u && $this->isSet($p->var)) {
                continue; // dim() maps it through idOfKey
            }
            $this->replaceRead($u, $is_key ? 'strtolower(Interner::lookup($' . $id_var . '))' : 'Interner::lookup($' . $id_var . ')');
        }
    }

    /** Replaces a read with an expression; inside an interpolated string the expression is concatenated in. */
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
            if ($this->src[$p->getStartFilePos()] !== '"') {
                $this->manual($n, 'heredoc interpolation');
                return;
            }
            $this->edits[] = [$s, $e, '" . ' . $expr . ' . "'];
            return;
        }
        $this->edit($n, $expr);
    }
}
