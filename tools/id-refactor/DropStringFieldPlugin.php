<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use PhpParser\Node;
use PhpParser\Node\Arg;
use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\Stmt;
use Psalm\Codebase;
use Psalm\Internal\Interner;
use Psalm\Internal\Sym;
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
 * Drops a string field that duplicates an id field (pzoom keeps only the StrId): for each configured
 * [class, string property, id property] (env DROP_FIELDS, JSON), every access to the string property on a value
 * of that class (or a subclass) is rewritten:
 *   Interner::intern($x->s)      => $x->id
 *   $a->s === $b->s (!==)        => $a->id === $b->id       (ids are exact: equal iff the strings are)
 *   isset($x->s)                 => isset($x->id)
 *   $x->s = E                    => $x->id = <E's id>      (Interner::intern(E), or E's id form)
 *   any other read               => Interner::lookup($x->id)
 * Accesses it cannot rewrite (nullsafe reads, compound assignments, references) are reported as `manual`.
 * Edits go to $ID_REFACTOR_OUT in apply.php's format; DropStringFieldDeclPlugin removes the declarations.
 */
final class DropStringFieldPlugin implements PluginEntryPointInterface, AfterFunctionLikeAnalysisInterface
{
    /** @var ?list<array{string, string, string}> */
    private static ?array $fields = null;

    private Codebase $codebase;
    private NodeTypeProvider $types;
    private string $file;
    private string $src;
    private bool $in_trait = false;
    private ?string $parentClass = null;
    private ?string $selfClass = null;
    /** @var ?SplObjectStorage<Arg, true> */
    private ?SplObjectStorage $spread_done = null;
    /** @var SplObjectStorage<Node, Node> */
    private SplObjectStorage $parent;
    /** @var list<array{int, int, string}> */
    private array $edits = [];

    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    /** @return list<array{string, string, string}> */
    private static function fields(): array
    {
        return self::$fields ??= json_decode((string) getenv('DROP_FIELDS'), true) ?: [];
    }

    public static function afterStatementAnalysis(AfterFunctionLikeAnalysisEvent $event): ?bool
    {
        $file = $event->getStatementsSource()->getFilePath();
        $root = dirname(__DIR__, 2);
        $extra = array_filter(explode(':', (string) getenv('ID_REFACTOR_ROOTS')));
        if (!str_starts_with($file, $root . '/src/') && !str_starts_with($file, $root . '/tests/')
            && !str_starts_with($file, $root . '/examples/')
            && array_filter($extra, static fn(string $r): bool => str_starts_with($file, $r)) === []
        ) {
            return null;
        }
        try {
            $h = new self();
            $h->codebase = $event->getCodebase();
            $h->types = $event->getNodeTypeProvider();
            $h->file = $file;
            $h->src = (string) file_get_contents($file);
            $h->in_trait = preg_match('/^\s*(?:final\s+|abstract\s+)?trait\s/m', $h->src) === 1;
            require_once __DIR__ . '/SymNames.php';
            $storage = $event->getFunctionlikeStorage();
            if ($storage instanceof \Psalm\Storage\MethodStorage && $storage->defining_fqcln !== null) {
                $cs = $h->codebase->classlike_storage_provider->find(Interner::intern($storage->defining_fqcln));
                $h->parentClass = $cs?->parent_class;
                $h->selfClass = $storage->defining_fqcln;
            }
            $h->parent = new SplObjectStorage();
            $h->spread_done = new SplObjectStorage();
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
            if ($n instanceof Expr\Closure || $n instanceof Expr\ArrowFunction || $n instanceof Stmt\Class_
                || $n instanceof Stmt\Function_
            ) {
                continue; // own analysis event, own node types
            }
            foreach ($n->getSubNodeNames() as $sub) {
                $v = $n->$sub;
                if ($v instanceof Node || is_array($v)) {
                    $this->index($v, $n);
                }
            }
        }
    }

    /** The [string prop, id prop] pair a property fetch reads, or null. */
    private function pair(Expr $fetch): ?array
    {
        if (!($fetch instanceof Expr\PropertyFetch || $fetch instanceof Expr\NullsafePropertyFetch)
            || !$fetch->name instanceof Identifier
        ) {
            return null;
        }
        $prop = $fetch->name->name;
        $cands = array_values(array_filter(self::fields(), static fn(array $f): bool => $f[1] === $prop));
        if ($cands === []) {
            return null;
        }
        // `$this` in a trait whose every user drops the property alike: whatever Psalm narrowed `$this` to here
        if ($this->in_trait && $fetch->var instanceof Expr\Variable && $fetch->var->name === 'this'
            && $this->traitUsersCovered($prop)
        ) {
            return [$prop, $cands[0][2], $cands[0][3] ?? 'lookup'];
        }
        $t = $this->types->getType($fetch->var);
        $has_object = false;
        foreach ($t?->getAtomicTypes() ?? [] as $a0) {
            $has_object = $has_object || $a0 instanceof TNamedObject;
        }
        if (!$has_object && $fetch->var instanceof Expr\Variable && is_string($fetch->var->name)) {
            $t = null;
            // no type recorded here (inside isset() Psalm keeps none): the class of a `new C(...)` the function
            // assigns to that variable
            foreach ($this->parent as $m) {
                if ($m instanceof Expr\Assign && $m->var instanceof Expr\Variable && $m->var->name === $fetch->var->name
                    && $m->expr instanceof Expr\New_ && $m->expr->class instanceof Name
                ) {
                    $t = new \Psalm\Type\Union([new TNamedObject($m->expr->class->attrs()->resolvedId ?? Interner::intern($m->expr->class->toString()))]);
                }
            }
        }
        if ($t === null) {
            return null;
        }
        $match = null;
        $parent_only = null;
        $kept = [];
        $dropped = [];
        // a template parameter stands for its bound
        $atomics = [];
        $queue = array_values($t->getAtomicTypes());
        while ($queue !== []) {
            $a = array_shift($queue);
            if ($a instanceof \Psalm\Type\Atomic\TTemplateParam) {
                foreach ($a->as->getAtomicTypes() as $b) {
                    $queue[] = $b;
                }
                continue;
            }
            $atomics[] = $a;
        }
        foreach ($atomics as $a) {
            if ($a instanceof \Psalm\Type\Atomic\TNull) {
                continue;
            }
            if (!$a instanceof TNamedObject) {
                return null; // not an object of a known class
            }
            $cls = SymNames::named($a);
            $found = null;
            foreach ($cands as $c) {
                if (strcasecmp($cls, $c[0]) === 0
                    || $this->codebase->classExtendsOrImplements(Interner::intern($cls), Interner::intern($c[0]))
                ) {
                    $found = [$c[1], $c[2], $c[3] ?? 'lookup'];
                }
            }
            if ($found === null) {
                // a class without the property at all (a wider declared type, e.g. FunctionLikeStorage for a
                // MethodStorage): the access can only be on the configured class
                $ks = $this->codebase->classlike_storage_provider->find(Interner::intern($cls));
                if ($ks !== null && !isset($ks->declaring_property_ids[Interner::intern($prop)])) {
                    // a parent of the configured class (the declared type of a variable holding it)
                    foreach ($cands as $c) {
                        if ($this->codebase->classExtendsOrImplements(Interner::intern($c[0]), Interner::intern($cls))) {
                            $parent_only = [$c[1], $c[2], $c[3] ?? 'lookup'];
                        }
                    }
                    continue;
                }
                $kept[] = $cls;
                continue;
            }
            if ($match !== null && $match !== $found) {
                return null;
            }
            $match = $found;
            $dropped[] = $cls;
        }
        if ($match !== null && $kept !== []) {
            return ['mixed', $match[1], array_values(array_unique($kept)), $match[2]];
        }
        return $match ?? ($kept === [] ? $parent_only : null);
    }

    private function text(Node $n): string
    {
        return substr($this->src, $n->getStartFilePos(), $n->getEndFilePos() + 1 - $n->getStartFilePos());
    }

    private function renameProp(Expr\PropertyFetch|Expr\NullsafePropertyFetch $f, string $to): void
    {
        $this->edits[] = [$f->name->getStartFilePos(), $f->name->getEndFilePos() + 1, $to];
    }

    /** Constructor calls passing a dropped promoted parameter: the argument becomes its id. */
    private function ctorArgs(Expr\New_|Expr\StaticCall $call): void
    {
        if ($call instanceof Expr\StaticCall) {
            if (!$call->name instanceof Identifier || strtolower($call->name->name) !== '__construct'
                || !$call->class instanceof Name || strtolower($call->class->toString()) !== 'parent'
            ) {
                return;
            }
            $class = $this->parentClass;
        } else {
            if (!$call->class instanceof Name) {
                return;
            }
            $class = (string) ((isset($call->class->attrs()->resolvedId) ? Interner::lookupOrNull($call->class->attrs()->resolvedId) : $call->class->toString()));
            if (in_array(strtolower($class), ['self', 'static'], true)) {
                $class = $this->selfClass;
            }
        }
        if ($class === null) {
            return;
        }
        try {
            $mid = $this->codebase->methods->getDeclaringMethodId(new \Psalm\Internal\MethodIdentifier(\Psalm\Internal\Interner::intern($class), \Psalm\Internal\Sym::CONSTRUCT));
            if ($mid === null) {
                return;
            }
            $ms = $this->codebase->methods->getStorage($mid);
        } catch (Throwable) {
            return;
        }
        $declaring = $ms->defining_fqcln ?? \Psalm\Internal\Interner::lookup($mid->class_id);
        foreach (self::fields() as [$fclass, $prop, $id]) {
            if (strcasecmp($declaring, $fclass) !== 0) {
                continue;
            }
            foreach ($ms->params as $i => $param) {
                if ($param->name !== $prop || !$param->promoted_property) {
                    continue;
                }
                foreach ($call->getArgs() as $j => $a) {
                    if ($a->unpack && $j <= $i) {
                        // a spread list supplying this parameter: every element becomes its id (once per list)
                        if (isset($this->spread_done[$a])) {
                            break;
                        }
                        $this->spread_done[$a] = true;
                        $v = $a->value;
                        $this->edits[] = [$v->getStartFilePos(), $v->getStartFilePos(), 'array_map(Interner::intern(...), '];
                        $this->edits[] = [$v->getEndFilePos() + 1, $v->getEndFilePos() + 1, ')'];
                        break;
                    }
                    if (($a->name !== null && $a->name->name === $prop) || ($a->name === null && $j === $i)) {
                        if ($a->name !== null) {
                            $this->edits[] = [$a->name->getStartFilePos(), $a->name->getEndFilePos() + 1, $id];
                        }
                        $v = $a->value;
                        if ($v instanceof Expr\StaticCall && $v->class instanceof Name && $v->name instanceof Identifier
                            && strtolower($v->class->getLast()) === 'interner' && strtolower($v->name->name) === 'lookup'
                            && count($v->getArgs()) === 1
                        ) {
                            $this->edits[] = [$v->getStartFilePos(), $v->getEndFilePos() + 1, $this->text($v->getArgs()[0]->value)];
                            continue;
                        }
                        if ($v instanceof \PhpParser\Node\Scalar\String_) {
                            [$text, $new] = SymNames::forLiteral($v->value);
                            if ($new !== null) {
                                self::out(['kind' => 'sym', 'name' => $new[0], 'value' => $new[1], 'literal' => $new[2]]);
                            }
                            $this->edits[] = [$v->getStartFilePos(), $v->getEndFilePos() + 1, $text];
                        } else {
                            $this->edits[] = [$v->getStartFilePos(), $v->getStartFilePos(), 'Interner::intern('];
                            $this->edits[] = [$v->getEndFilePos() + 1, $v->getEndFilePos() + 1, ')'];
                        }
                    }
                }
            }
        }
    }

    private function walk(): void
    {
        $done = new SplObjectStorage();
        foreach ($this->parent as $n) {
            if ($n instanceof Expr\New_ || $n instanceof Expr\StaticCall) {
                $this->ctorArgs($n);
            }
            if (!$n instanceof Expr || isset($done[$n])) {
                continue;
            }
            $pair = $this->pair($n);
            if ($pair === null) {
                continue;
            }
            if ($pair[0] === 'mixed') {
                $this->mixedRead($n, $pair[1], $pair[2], $pair[3]);
                continue;
            }
            [, $id, $fn] = $pair;
            $p = $this->parent[$n];
            // a trait body is analyzed once per using class: `$this` is a different class in each
            if ($this->in_trait && $n->var instanceof Expr\Variable && $n->var->name === 'this'
                && !$this->traitUsersCovered($n->name instanceof Identifier ? $n->name->name : '')
            ) {
                self::out(['kind' => 'manual', 'site' => $this->file . ':' . $n->getStartLine(), 'why' => '$this in a trait']);
                continue;
            }
            /** @var Expr\PropertyFetch|Expr\NullsafePropertyFetch $n */
            if ($p instanceof Expr\Assign && $p->var === $n) {
                // `$x->s = E;` next to an assignment of the id itself (the constructor's pair): the statement goes
                $stmt = $this->parent[$p];
                if ($stmt instanceof Stmt\Expression && $this->assignsId($n, $id)) {
                    $this->edits[] = [$stmt->getStartFilePos(), $stmt->getEndFilePos() + 1, ''];
                    continue;
                }
                $this->renameProp($n, $id);
                $this->edits[] = [$p->expr->getStartFilePos(), $p->expr->getStartFilePos(), 'Interner::intern('];
                $this->edits[] = [$p->expr->getEndFilePos() + 1, $p->expr->getEndFilePos() + 1, ')'];
                continue;
            }
            if ((($p instanceof Expr\AssignOp || $p instanceof Expr\AssignRef) && $p->var === $n) || ($p instanceof Arg && $p->byRef)
                || $p instanceof Stmt\Unset_ || $p instanceof Expr\PreInc || $p instanceof Expr\PostInc
            ) {
                self::out(['kind' => 'manual', 'site' => $this->file . ':' . $n->getStartLine(), 'why' => $p->getType()]);
                continue;
            }
            if ($p instanceof Expr\Isset_) {
                $this->renameProp($n, $id);
                continue;
            }
            if ($p instanceof Arg) {
                $call = $this->parent[$p];
                if ($call instanceof Expr\StaticCall && $call->class instanceof Name && $call->name instanceof Identifier
                    && strtolower($call->class->getLast()) === 'interner' && strtolower($call->name->name) === 'intern'
                ) {
                    $this->edits[] = [$call->getStartFilePos(), $call->getEndFilePos() + 1,
                        $this->text($n->var) . ($n instanceof Expr\NullsafePropertyFetch ? '?->' : '->') . $id];
                    continue;
                }
            }
            if (($p instanceof Expr\BinaryOp\Identical || $p instanceof Expr\BinaryOp\NotIdentical)) {
                $other = $p->left === $n ? $p->right : $p->left;
                $opair = $this->pair($other);
                if ($opair !== null && $opair[1] === $id) {
                    $this->renameProp($n, $id);
                    /** @var Expr\PropertyFetch|Expr\NullsafePropertyFetch $other */
                    $this->renameProp($other, $id);
                    $done[$other] = true;
                    continue;
                }
            }
            // a nullsafe read of a nullable pair: null propagates through lookupOrNull
            if ($n instanceof Expr\NullsafePropertyFetch && $fn !== 'lookupOrNull') {
                self::out(['kind' => 'manual', 'site' => $this->file . ':' . $n->getStartLine(), 'why' => 'nullsafe read']);
                continue;
            }
            $arrow = $n instanceof Expr\NullsafePropertyFetch ? '?->' : '->';
            if ($p instanceof Expr\BinaryOp\Coalesce && $p->left === $n) {
                // `$x->s ?? E` (a null or unset receiver falls through to E)
                $recv = $this->text($n->var);
                $this->edits[] = [$p->getStartFilePos(), $p->getEndFilePos() + 1, $fn === 'lookupOrNull'
                    ? '(Interner::lookupOrNull(' . $recv . $arrow . $id . ') ?? ' . $this->text($p->right) . ')'
                    : '(isset(' . $recv . $arrow . $id . ') ? Interner::' . $fn . '(' . $recv . $arrow . $id . ') : ' . $this->text($p->right) . ')'];
                $done[$p->right] = true;
                continue;
            }
            $this->replaceRead($n, 'Interner::' . $fn . '(' . $this->text($n->var) . $arrow . $id . ')');
        }
    }

    /** Replaces a read with an expression; inside an interpolated string the expression is concatenated in. */
    private function replaceRead(Expr $n, string $expr, bool $maybe_not_string = false): void
    {
        $p = $this->parent[$n];
        if ($p instanceof \PhpParser\Node\Scalar\InterpolatedString) {
            $s = $n->getStartFilePos();
            $e = $n->getEndFilePos() + 1;
            if ($this->src[$s - 1] === '{' && $this->src[$e] === '}') {
                $s--;
                $e++;
            }
            if ($this->src[$p->getStartFilePos()] !== '"') {
                self::out(['kind' => 'manual', 'site' => $this->file . ':' . $n->getStartLine(), 'why' => 'heredoc interpolation']);
                return;
            }
            $q = '"';
            $this->edits[] = [$s, $e, $q . ' . ' . ($maybe_not_string ? '(string) ' : '') . $expr . ' . ' . $q];
            return;
        }
        $this->edits[] = [$n->getStartFilePos(), $n->getEndFilePos() + 1, $expr];
    }

    /** Whether every class using this file's trait drops the property the same way (its `$this` reads may move). */
    private function traitUsersCovered(string $prop): bool
    {
        if (!preg_match('/^namespace\s+([^;]+);/m', $this->src, $ns) || !preg_match('/^\s*(?:final\s+|abstract\s+)?trait\s+(\w+)/m', $this->src, $tm)) {
            return false;
        }
        $trait_lc = strtolower($ns[1] . '\\' . $tm[1]);
        $pairs = [];
        foreach ($this->codebase->classlike_storage_provider->getAll() as $st) {
            if (!isset($st->used_traits[Interner::intern($ns[1] . '\\' . $tm[1])])) {
                continue;
            }
            $found = null;
            foreach (self::fields() as [$c, $p, $id]) {
                if ($p === $prop && (strcasecmp(\Psalm\Internal\Interner::lookup($st->id), $c) === 0
                    || $this->codebase->classExtendsOrImplements($st->id, Interner::intern($c)))
                ) {
                    $found = $id;
                }
            }
            if ($found === null) {
                return false;
            }
            $pairs[$found] = true;
        }
        return count($pairs) === 1;
    }

    /**
     * A read of the property on a union of classes that drop it and classes that keep it: a conditional on the
     * receiver's class (only for a side-effect-free receiver).
     *
     * @param list<string> $dropped the classes keeping the property
     */
    private function mixedRead(Expr\PropertyFetch|Expr\NullsafePropertyFetch $n, string $id, array $dropped, string $fn = 'lookup'): void
    {
        $p = $this->parent[$n];
        $pure = static function (Expr $e) use (&$pure): bool {
            return $e instanceof Expr\Variable
                || (($e instanceof Expr\PropertyFetch) && $e->name instanceof Identifier && $pure($e->var));
        };
        if ($n instanceof Expr\NullsafePropertyFetch || !$pure($n->var)
            || ($p instanceof Expr\Assign && $p->var === $n) || $p instanceof Expr\AssignOp || $p instanceof Expr\Isset_
        ) {
            self::out(['kind' => 'manual', 'site' => $this->file . ':' . $n->getStartLine(), 'why' => 'mixed receiver']);
            return;
        }
        $recv = $this->text($n->var);
        $conds = implode(' || ', array_map(static fn(string $c): string => $recv . ' instanceof \\' . $c, $dropped));
        $this->replaceRead($n, '(' . $conds . ' ? ' . $this->text($n) . ' : Interner::' . $fn . '(' . $recv . '->' . $id . '))', true);
    }

    private function assignsId(Expr\PropertyFetch|Expr\NullsafePropertyFetch $f, string $id): bool
    {
        $recv = $this->text($f->var);
        foreach ($this->parent as $m) {
            if ($m instanceof Expr\Assign && $m->var instanceof Expr\PropertyFetch && $m->var->name instanceof Identifier
                && $m->var->name->name === $id && $this->text($m->var->var) === $recv
            ) {
                return true;
            }
        }
        return false;
    }

    /** @param array<string, mixed> $row */
    public static function out(array $row): void
    {
        $path = getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/drop-field.jsonl';
        file_put_contents($path, json_encode($row, JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
    }
}
