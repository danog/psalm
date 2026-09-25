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
use Psalm\Plugin\EventHandler\AfterClassLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\AfterFunctionLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\AfterFunctionLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use Psalm\Type\Atomic\TNamedObject;
use SimpleXMLElement;
use SplObjectStorage;
use Throwable;

/**
 * Turns the values of id-keyed maps into class ids (pzoom's `declaring_property_ids: FxHashMap<StrId, StrId>`).
 * Config (env MAP_VALUE_IDS, JSON): [[class, prop, kind]], kind:
 *   name         array<int, string> of class names                  -> array<int, int>
 *   member       array<int, string> of "Class::$key" member ids     -> array<int, int> (the class; the key names the member)
 *   member_list  array<int, list<string>> of member ids             -> array<int, list<int>>
 *   inner_name   array<int, array<string, V>> keyed by class names  -> array<int, array<int, V>> (keys: class ids)
 *   inner_member array<int, array<string, V>> keyed by "Class::m"   -> array<int, array<int, V>> (keys: class ids; the
 *                outer key names the member)
 * For the inner kinds, a configured method returning `S[k]` (env MAP_VALUE_SOURCES = [[class, method, kind]])
 * yields an inner map too, and so does a local assigned from one: key writes become class ids, a foreach key
 * variable reads its old string.
 * Writes `S[k] = V` / `S[k] ??= V` / `S[k][] = V` store V's class id (`X . '::$' . Y` and a local defined so give X's
 * id; a value read from a converted map stays). Reads: `Interner::intern(S[k])` is `S[k]`; any other read is the
 * old string (`Interner::lookup(S[k])`, a member `Interner::lookup(S[k]) . '::$' . Interner::lookup(k)`). Foreach
 * value variables over such maps are ids the same way. Everything else is reported as `manual`.
 */
final class MapValueIdPlugin implements PluginEntryPointInterface, AfterFunctionLikeAnalysisInterface
{
    /** @var ?list<array{string, string, string}> */
    private static ?array $cfg = null;

    private Codebase $codebase;
    private NodeTypeProvider $types;
    private string $file;
    private string $src;
    /** @var SplObjectStorage<Node, Node> */
    private SplObjectStorage $parent;
    /** @var list<Node> */
    private array $order = [];
    /** @var list<array{int, int, string}> */
    private array $edits = [];
    /** @var SplObjectStorage<Node, true> */
    private SplObjectStorage $done;
    /** @var array<string, list<array{string, string, int, int}>> value variable => [kind, key text, loop start, loop end] */
    private array $loop_vals = [];
    /** @var array<string, array{string, string}> local inner maps: variable => [kind, outer key text] */
    private array $inner_locals = [];
    /** @var array<string, array{string, string, int, int}> key variable of a foreach over an inner map => [kind, outer key, start, end] */
    private array $inner_keys = [];

    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    /** @return list<array{string, string, string}> */
    public static function cfg(): array
    {
        return self::$cfg ??= json_decode((string) getenv('MAP_VALUE_IDS'), true) ?: [];
    }

    public static function out(array $row): void
    {
        file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/map-value-id.jsonl',
            json_encode($row, JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
    }

    public static function afterStatementAnalysis(AfterFunctionLikeAnalysisEvent $event): ?bool
    {
        $file = $event->getStatementsSource()->getFilePath();
        require_once __DIR__ . '/MapIdSetPlugin.php';
        if (!MapIdSetPlugin::inScope($file)) {
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
        self::out(['kind' => 'manual', 'site' => $this->file . ':' . $n->getStartLine(), 'why' => $why]);
    }

    private function receiverIs(Expr $recv, string $class): bool
    {
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

    /** The kind of a configured map this expression is, or null. */
    private function mapKind(Expr $e): ?string
    {
        if (!($e instanceof Expr\PropertyFetch || $e instanceof Expr\NullsafePropertyFetch) || !$e->name instanceof Identifier) {
            return null;
        }
        foreach (self::cfg() as [$c, $p, $k]) {
            if ($e->name->name === $p && $this->receiverIs($e->var, $c)) {
                return $k;
            }
        }
        return null;
    }

    /**
     * An expression yielding a converted value: [kind, key text], kind name|member; a member list element read
     * `S[k][i]` too. Null otherwise.
     *
     * @return ?array{string, string}
     */
    private function valueOf(Expr $e): ?array
    {
        if ($e instanceof Expr\ArrayDimFetch && $e->dim !== null) {
            $k = $this->mapKind($e->var);
            if ($k === 'name' || $k === 'member') {
                return [$k, $this->text($e->dim)];
            }
            if ($e->var instanceof Expr\ArrayDimFetch && $e->var->dim !== null && $this->mapKind($e->var->var) === 'member_list') {
                return ['member', $this->text($e->var->dim)];
            }
        }
        if ($e instanceof Expr\Variable && is_string($e->name)) {
            return $this->loopVal($e);
        }
        return null;
    }

    /** @return ?array{string, string} a loop value variable's [kind, key text] at this use */
    private function loopVal(Expr\Variable $e): ?array
    {
        foreach ($this->loop_vals[$e->name] ?? [] as [$k, $key, $s, $end]) {
            if ($e->getStartFilePos() > $s && $e->getEndFilePos() <= $end) {
                return [$k, $key];
            }
        }
        return null;
    }

    /** The class-id text of a written value (a name, or a member id "X::$y"), or null. */
    private function classIdOf(Expr $v, string $kind, int $depth = 0): ?string
    {
        if ($this->valueOf($v) !== null) {
            return $this->text($v);
        }
        if ($v instanceof Expr\Ternary && $v->if !== null) {
            $a = $this->classIdOf($v->if, $kind, $depth);
            $b = $this->classIdOf($v->else, $kind, $depth);
            return $a !== null && $b !== null ? '(' . $this->text($v->cond) . ' ? ' . $a . ' : ' . $b . ')' : null;
        }
        if ($kind === 'name') {
            if ($v instanceof Expr\FuncCall && $v->name instanceof Name && strtolower($v->name->toString()) === 'strtolower'
                && count($v->getArgs()) === 1
            ) {
                // a class id stands for the class whatever the spelling
                return $this->classIdOf($v->getArgs()[0]->value, $kind, $depth);
            }
            return $this->nameId($v);
        }
        // member id: the parts before the `::$` literal
        $parts = $this->concatParts($v);
        if ($parts !== null) {
            $before = [];
            foreach ($parts as $p) {
                if ($p instanceof String_ && str_starts_with($p->value, '::$')) {
                    $before = array_values(array_filter($before, static fn(Expr $x): bool => !($x instanceof String_ && $x->value === '')));
                    return count($before) === 1 ? $this->nameId($before[0]) : null;
                }
                if ($p instanceof String_ && str_contains($p->value, '::$')) {
                    return null;
                }
                $before[] = $p;
            }
            return null;
        }
        if ($v instanceof Expr\Variable && is_string($v->name) && $depth < 3) {
            $defs = [];
            foreach ($this->order as $n) {
                if ($n instanceof Expr\Assign && $n->var instanceof Expr\Variable && $n->var->name === $v->name) {
                    $defs[] = $n;
                }
            }
            // the nearest definition before the use
            $defs = array_values(array_filter($defs, static fn(Expr\Assign $a): bool => $a->getEndFilePos() < $v->getStartFilePos()));
            usort($defs, static fn(Expr\Assign $a, Expr\Assign $b): int => $b->getStartFilePos() <=> $a->getStartFilePos());
            if ($defs !== []) {
                return $this->classIdOf($defs[0]->expr, $kind, $depth + 1);
            }
        }
        return null;
    }

    /** @return ?list<Expr> the operands of a string concatenation / interpolation, in order */
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

    private function nameId(Expr $v, int $depth = 0): string
    {
        if ($v instanceof String_) {
            [$text, $new] = SymNames::forLiteral($v->value);
            if ($new !== null) {
                self::out(['kind' => 'sym', 'name' => $new[0], 'value' => $new[1], 'literal' => $new[2]]);
            }
            return $text;
        }
        if ($v instanceof Expr\StaticCall && $v->class instanceof Name && strtolower($v->class->getLast()) === 'interner'
            && $v->name instanceof Identifier && in_array(strtolower($v->name->name), ['lookup', 'lookuplc'], true)
            && !$v->isFirstClassCallable() && count($v->getArgs()) === 1
        ) {
            return $this->text($v->getArgs()[0]->value);
        }
        if ($v instanceof Expr\FuncCall && $v->name instanceof Name && strtolower($v->name->toString()) === 'strtolower'
            && !$v->isFirstClassCallable() && count($v->getArgs()) === 1
        ) {
            // a class id stands for the class whatever the spelling
            return $this->nameId($v->getArgs()[0]->value, $depth);
        }
        if ($v instanceof Expr\Variable && is_string($v->name) && $this->loopVal($v) !== null) {
            return '$' . $v->name;
        }
        // a local: its nearest earlier definition, when that is a name conversion
        if ($v instanceof Expr\Variable && is_string($v->name) && $depth < 3) {
            $defs = [];
            foreach ($this->order as $n) {
                if ($n instanceof Expr\Assign && $n->var instanceof Expr\Variable && $n->var->name === $v->name
                    && $n->getEndFilePos() < $v->getStartFilePos()
                ) {
                    $defs[] = $n;
                }
            }
            usort($defs, static fn(Expr\Assign $a, Expr\Assign $b): int => $b->getStartFilePos() <=> $a->getStartFilePos());
            $d = $defs[0]->expr ?? null;
            if ($d instanceof Expr\StaticCall || ($d instanceof Expr\FuncCall && $d->name instanceof Name && strtolower($d->name->toString()) === 'strtolower')
                || ($d instanceof Expr\Variable && $this->loopVal($d) !== null)
            ) {
                $r = $this->nameId($d, $depth + 1);
                if (!str_starts_with($r, 'Interner::intern(')) {
                    return $r;
                }
            }
        }
        // a nullable name concatenated as '' before: the same string
        $t = $this->types->getType($v);
        return 'Interner::intern(' . ($t !== null && $t->isNullable() ? '(string) ' : '') . $this->text($v) . ')';
    }

    /** The old string of a converted value read. */
    private function oldString(string $read, string $kind, string $key): string
    {
        return $kind === 'name'
            ? 'Interner::lookup(' . $read . ')'
            : '(Interner::lookup(' . $read . ') . \'::$\' . Interner::lookup(' . $key . '))';
    }

    /** @return ?array{string, string} [kind, outer key text] when the expression is an inner map (keyed by class) */
    private function innerOf(Expr $e): ?array
    {
        // `A ?? B`, `A + B`: an inner map when either operand is one
        if ($e instanceof Expr\BinaryOp\Coalesce || $e instanceof Expr\BinaryOp\Plus) {
            return $this->innerOf($e->left) ?? ($e->right instanceof Expr\Array_ ? null : $this->innerOf($e->right));
        }
        if ($e instanceof Expr\ArrayDimFetch && $e->dim !== null) {
            $k = $this->mapKind($e->var);
            if ($k === 'inner_name' || $k === 'inner_member') {
                return [$k, $this->text($e->dim)];
            }
        }
        if (($e instanceof Expr\MethodCall || $e instanceof Expr\StaticCall) && $e->name instanceof Identifier) {
            foreach (json_decode((string) getenv('MAP_VALUE_SOURCES'), true) ?: [] as [$c, $m, $k]) {
                if (strcasecmp($m, $e->name->name) === 0 && ($e instanceof Expr\StaticCall || $this->receiverIs($e->var, $c))) {
                    // the member: the method id argument's name
                    $a = $e->getArgs()[0]->value ?? null;
                    return [$k, $a !== null ? $this->text($a) . '->name_id' : '0'];
                }
            }
        }
        if ($e instanceof Expr\Variable && is_string($e->name) && isset($this->inner_locals[$e->name])) {
            return $this->inner_locals[$e->name];
        }
        return null;
    }

    /** An inner map's key (a class name, or a "Class::member" id) as the class id text, or null. */
    private function innerKeyId(Expr $k, string $kind): ?string
    {
        if ($k instanceof Expr\Variable && is_string($k->name)) {
            foreach ([$this->inner_keys[$k->name] ?? null] as $ik) {
                if ($ik !== null && $k->getStartFilePos() > $ik[2] && $k->getEndFilePos() <= $ik[3]) {
                    return '$' . $k->name;
                }
            }
        }
        if ($kind === 'inner_name') {
            return $this->classIdOf($k, 'name');
        }
        // "C::m": the class part (the separator is `::`)
        $parts = $this->concatParts($k);
        if ($parts !== null) {
            $before = [];
            foreach ($parts as $p) {
                if ($p instanceof String_ && str_starts_with($p->value, '::')) {
                    $before = array_values(array_filter($before, static fn(Expr $x): bool => !($x instanceof String_ && $x->value === '')));
                    if (count($before) !== 1) {
                        return null;
                    }
                    $b = $before[0];
                    // a lowercased class name: the id of the class (ids are exact names; the class resolves by id)
                    if ($b instanceof Expr\FuncCall && $b->name instanceof Name && strtolower($b->name->toString()) === 'strtolower'
                        && count($b->getArgs()) === 1
                    ) {
                        $b = $b->getArgs()[0]->value;
                    }
                    return $this->nameId($b);
                }
                $before[] = $p;
            }
            return null;
        }
        if ($k instanceof Expr\Variable && is_string($k->name)) {
            $defs = [];
            foreach ($this->order as $n) {
                if ($n instanceof Expr\Assign && $n->var instanceof Expr\Variable && $n->var->name === $k->name
                    && $n->getEndFilePos() < $k->getStartFilePos()
                ) {
                    $defs[] = $n;
                }
            }
            usort($defs, static fn(Expr\Assign $a, Expr\Assign $b): int => $b->getStartFilePos() <=> $a->getStartFilePos());
            return $defs !== [] ? $this->innerKeyId($defs[0]->expr, $kind) : null;
        }
        return null;
    }

    /** Inner maps: key writes, literals assigned whole, locals from sources, foreach key variables. */
    private function inner(): void
    {
        // locals every assignment of which is an inner map (or a source method's result)
        $bad = [];
        foreach ($this->order as $n) {
            if (($n instanceof Expr\Assign || $n instanceof Expr\AssignRef || $n instanceof Expr\AssignOp)
                && $n->var instanceof Expr\Variable && is_string($n->var->name)
            ) {
                $e = $n->expr;
                while ($e instanceof Expr\BinaryOp\Coalesce) {
                    $e = $e->left;
                }
                $io = $n instanceof Expr\Assign ? $this->innerOf($e) : null;
                if ($io === null) {
                    $bad[$n->var->name] = true;
                } else {
                    $this->inner_locals[$n->var->name] = $io;
                }
            }
        }
        $this->inner_locals = array_diff_key($this->inner_locals, $bad);
        foreach ($this->order as $n) {
            // S[k][K2] = V (and ??=)
            if (($n instanceof Expr\Assign || $n instanceof Expr\AssignOp\Coalesce) && $n->var instanceof Expr\ArrayDimFetch
                && $n->var->dim !== null && ($io = $this->innerOf($n->var->var)) !== null
            ) {
                $id = $this->innerKeyId($n->var->dim, $io[0]);
                if ($id === null) {
                    $this->manual($n, 'inner key ' . $this->text($n->var->dim));
                } elseif ($id !== $this->text($n->var->dim)) {
                    $this->edit($n->var->dim, $id);
                }
                continue;
            }
            // S[k] = [K2 => V, ...]
            if ($n instanceof Expr\Assign && $n->var instanceof Expr\ArrayDimFetch && $n->expr instanceof Expr\Array_
                && ($io = $this->innerOf($n->var)) !== null
            ) {
                foreach ($n->expr->items as $it) {
                    if ($it === null || $it->key === null) {
                        continue;
                    }
                    $id = $this->innerKeyId($it->key, $io[0]);
                    if ($id === null) {
                        $this->manual($n, 'inner literal key ' . $this->text($it->key));
                    } elseif ($id !== $this->text($it->key)) {
                        $this->edit($it->key, $id);
                    }
                }
                continue;
            }
            // `<inner> + [K2 => V, ...]`: the literal's keys too
            if ($n instanceof Expr\BinaryOp\Plus && $n->right instanceof Expr\Array_ && ($io = $this->innerOf($n->left)) !== null) {
                foreach ($n->right->items as $it) {
                    if ($it === null || $it->key === null) {
                        continue;
                    }
                    $id = $this->innerKeyId($it->key, $io[0]);
                    if ($id === null) {
                        $this->manual($n, 'inner literal key ' . $this->text($it->key));
                    } elseif ($id !== $this->text($it->key)) {
                        $this->edit($it->key, $id);
                    }
                }
                continue;
            }
            // isset(S[k][K2])
            if ($n instanceof Expr\ArrayDimFetch && $n->dim !== null && ($io = $this->innerOf($n->var)) !== null
                && ($this->parent[$n] ?? null) instanceof Expr\Isset_
            ) {
                $id = $this->innerKeyId($n->dim, $io[0]);
                if ($id === null) {
                    $this->manual($n, 'inner isset key');
                } elseif ($id !== $this->text($n->dim)) {
                    $this->edit($n->dim, $id);
                }
                continue;
            }
            // foreach (S[k] as $key => ...): the key variable reads its old string
            if ($n instanceof Stmt\Foreach_ && $n->keyVar instanceof Expr\Variable && is_string($n->keyVar->name)
                && ($io = $this->innerOf($n->expr)) !== null
            ) {
                $key = $n->keyVar->name;
                $this->inner_keys[$key] = [$io[0], $io[1], $n->keyVar->getEndFilePos(), $n->getEndFilePos()];
                foreach ($this->order as $u) {
                    if ($u instanceof Expr\Variable && $u->name === $key && $u !== $n->keyVar
                        && $u->getStartFilePos() > $n->getStartFilePos() && $u->getEndFilePos() <= $n->getEndFilePos()
                    ) {
                        $p = $this->parent[$u];
                        if ($p instanceof Expr\ArrayDimFetch && $p->dim === $u && $this->innerOf($p->var) !== null) {
                            continue; // an inner map key again: stays the id
                        }
                        $old = $io[0] === 'inner_name'
                            ? 'Interner::lookup($' . $key . ')'
                            : '(Interner::lookup($' . $key . ') . \'::\' . Interner::lookupLc(' . $io[1] . '))';
                        $this->replaceRead($u, $old);
                    }
                }
            }
            if (($n instanceof Expr\FuncCall && $n->name instanceof Name && in_array(strtolower($n->name->toString()), ['array_keys', 'array_key_first', 'array_key_last', 'key'], true)
                && ($n->getArgs()[0] ?? null) !== null && $this->innerOf($n->getArgs()[0]->value) !== null)
            ) {
                $this->manual($n, 'inner map keys read');
            }
        }
    }

    private function walk(): void
    {
        $this->inner();
        foreach ($this->order as $n) {
            if (isset($this->done[$n])) {
                continue;
            }
            if ($n instanceof Stmt\Foreach_) {
                $this->foreach_($n);
                continue;
            }
            if (!$n instanceof Expr) {
                continue;
            }
            // writes
            if (($n instanceof Expr\Assign || $n instanceof Expr\AssignOp\Coalesce) && $n->var instanceof Expr\ArrayDimFetch) {
                $d = $n->var;
                $kind = null;
                if ($d->dim !== null && ($k = $this->mapKind($d->var)) !== null && !in_array($k, ['member_list', 'inner_name', 'inner_member'], true)) {
                    $kind = $k;
                } elseif ($d->dim === null && $d->var instanceof Expr\ArrayDimFetch && $this->mapKind($d->var->var) === 'member_list') {
                    $kind = 'member';
                } elseif ($d->dim !== null && in_array($this->mapKind($d->var), ['inner_name', 'inner_member'], true)) {
                    continue; // inner() rewrote its keys
                } elseif ($d->dim !== null && $this->mapKind($d->var) === 'member_list') {
                    $this->manual($n, 'member list replaced');
                    continue;
                }
                if ($kind !== null) {
                    $this->done[$d] = true;
                    $id = $this->classIdOf($n->expr, $kind);
                    if ($id === null) {
                        $this->manual($n, 'value ' . $this->text($n->expr));
                        continue;
                    }
                    if ($id !== $this->text($n->expr)) {
                        $this->edit($n->expr, $id);
                    }
                    $this->markDone($n->expr);
                    continue;
                }
            }
            $val = $this->valueOf($n);
            if ($val === null) {
                continue;
            }
            [$kind, $key] = $val;
            $p = $this->parent[$n];
            if ($p instanceof Expr\Isset_ || $p instanceof Stmt\Unset_ || $p instanceof Expr\Empty_) {
                continue;
            }
            if (($p instanceof Expr\Assign || $p instanceof Expr\AssignOp) && $p->var === $n) {
                continue;
            }
            // copied into another converted map: a write, handled above
            if ($p instanceof Expr\Assign && $p->var instanceof Expr\ArrayDimFetch && $this->valueOfTarget($p->var)) {
                continue;
            }
            $this->read($n, $kind, $key);
        }
    }

    private function valueOfTarget(Expr\ArrayDimFetch $d): bool
    {
        return ($d->dim !== null && $this->mapKind($d->var) !== null)
            || ($d->dim === null && $d->var instanceof Expr\ArrayDimFetch && $this->mapKind($d->var->var) === 'member_list');
    }

    private function markDone(Node $e): void
    {
        $this->done[$e] = true;
        foreach ($e->getSubNodeNames() as $sub) {
            foreach (is_array($e->$sub) ? $e->$sub : [$e->$sub] as $c) {
                if ($c instanceof Node) {
                    $this->markDone($c);
                }
            }
        }
    }

    /** A read of a converted value (not a write target). */
    private function read(Expr $n, string $kind, string $key): void
    {
        $p = $this->parent[$n];
        $t = $this->text($n);
        if ($p instanceof Arg) {
            $call = $this->parent[$p];
            if ($call instanceof Expr\StaticCall && $call->class instanceof Name && strtolower($call->class->getLast()) === 'interner'
                && $call->name instanceof Identifier && strtolower($call->name->name) === 'intern' && $kind === 'name'
            ) {
                $this->edit($call, $t);
                return;
            }
            // explode('::$', <member>)[0] / [$class] = explode(...)
            if ($call instanceof Expr\FuncCall && $call->name instanceof Name && strtolower($call->name->toString()) === 'explode'
                && $kind === 'member' && $call->getArgs()[1]->value === $n
            ) {
                $gp = $this->parent[$call];
                if ($gp instanceof Expr\ArrayDimFetch && $gp->var === $call && $gp->dim instanceof Node\Scalar\Int_ && $gp->dim->value === 0) {
                    $this->edit($gp, 'Interner::lookup(' . $t . ')');
                    return;
                }
                if ($gp instanceof Expr\Assign && $gp->expr === $call && ($gp->var instanceof Expr\List_ || $gp->var instanceof Expr\Array_)
                    && count($gp->var->items) === 1 && $gp->var->items[0] !== null && $gp->var->items[0]->key === null
                ) {
                    $this->edit($gp, $this->text($gp->var->items[0]->value) . ' = Interner::lookup(' . $t . ')');
                    return;
                }
            }
        }
        if ($p instanceof Expr\BinaryOp\Coalesce && $p->left === $n) {
            $this->edit($p, '(isset(' . $t . ') ? ' . $this->oldString($t, $kind, $key) . ' : ' . $this->text($p->right) . ')');
            $this->markDone($p->right);
            return;
        }
        $this->replaceRead($n, $this->oldString($t, $kind, $key));
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

    private function foreach_(Stmt\Foreach_ $f): void
    {
        $k = $this->mapKind($f->expr);
        if ($k === 'inner_name' || $k === 'inner_member') {
            return;
        }
        $key_text = null;
        if ($k === 'member_list') {
            $this->manual($f, 'foreach over a member list map');
            return;
        }
        if ($k === null && $f->expr instanceof Expr\ArrayDimFetch && $f->expr->dim !== null && $this->mapKind($f->expr->var) === 'member_list') {
            $k = 'member';
            $key_text = $this->text($f->expr->dim);
        }
        if ($k === null) {
            return;
        }
        if (!$f->valueVar instanceof Expr\Variable || !is_string($f->valueVar->name) || $f->byRef) {
            $this->manual($f, 'foreach binding');
            return;
        }
        $val = $f->valueVar->name;
        if ($key_text === null) {
            if ($f->keyVar instanceof Expr\Variable && is_string($f->keyVar->name)) {
                $key_text = '$' . $f->keyVar->name;
            } elseif ($k === 'member') {
                $key_text = '$' . $val . '_key';
                $this->edits[] = [$f->valueVar->getStartFilePos(), $f->valueVar->getStartFilePos(), $key_text . ' => '];
            } else {
                $key_text = '';
            }
        }
        // the value variable is an id from here on; reassignments in the body keep the old meaning
        foreach ($this->order as $n) {
            if ($n instanceof Expr\Assign && $n->var instanceof Expr\Variable && $n->var->name === $val
                && $n->getStartFilePos() > $f->getStartFilePos() && $n->getEndFilePos() < $f->getEndFilePos()
            ) {
                $this->manual($f, 'loop value reassigned');
                return;
            }
        }
        $this->loop_vals[$val][] = [$k, $key_text, $f->valueVar->getEndFilePos(), $f->getEndFilePos()];
        $this->done[$f->valueVar] = true;
    }
}
