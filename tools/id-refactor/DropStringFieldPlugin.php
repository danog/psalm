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
        if (!str_starts_with($file, $root . '/src/') && !str_starts_with($file, $root . '/tests/')) {
            return null;
        }
        try {
            $h = new self();
            $h->codebase = $event->getCodebase();
            $h->types = $event->getNodeTypeProvider();
            $h->file = $file;
            $h->src = (string) file_get_contents($file);
            $h->in_trait = preg_match('/^\s*(?:final\s+|abstract\s+)?trait\s/m', $h->src) === 1;
            $h->parent = new SplObjectStorage();
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
        $t = $this->types->getType($fetch->var);
        if ($t === null) {
            return null;
        }
        $match = null;
        foreach ($t->getAtomicTypes() as $a) {
            if ($a instanceof \Psalm\Type\Atomic\TNull) {
                continue;
            }
            if (!$a instanceof TNamedObject) {
                return null;
            }
            $found = null;
            foreach ($cands as $c) {
                if (strcasecmp($a->value, $c[0]) === 0
                    || $this->codebase->classExtendsOrImplements(Interner::intern($a->value), Interner::intern($c[0]))
                ) {
                    $found = $c;
                }
            }
            if ($found === null || ($match !== null && $match !== $found)) {
                return null;
            }
            $match = $found;
        }
        return $match === null ? null : [$match[1], $match[2]];
    }

    private function text(Node $n): string
    {
        return substr($this->src, $n->getStartFilePos(), $n->getEndFilePos() + 1 - $n->getStartFilePos());
    }

    private function renameProp(Expr\PropertyFetch|Expr\NullsafePropertyFetch $f, string $to): void
    {
        $this->edits[] = [$f->name->getStartFilePos(), $f->name->getEndFilePos() + 1, $to];
    }

    private function walk(): void
    {
        $done = new SplObjectStorage();
        foreach ($this->parent as $n) {
            if (!$n instanceof Expr || isset($done[$n])) {
                continue;
            }
            $pair = $this->pair($n);
            if ($pair === null) {
                continue;
            }
            [, $id] = $pair;
            $p = $this->parent[$n];
            // a trait body is analyzed once per using class: `$this` is a different class in each
            if ($this->in_trait && $n->var instanceof Expr\Variable && $n->var->name === 'this') {
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
            if ($p instanceof Expr\AssignOp || $p instanceof Expr\AssignRef || ($p instanceof Arg && $p->byRef)
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
            if ($n instanceof Expr\NullsafePropertyFetch) {
                self::out(['kind' => 'manual', 'site' => $this->file . ':' . $n->getStartLine(), 'why' => 'nullsafe read']);
                continue;
            }
            $this->edits[] = [$n->getStartFilePos(), $n->getStartFilePos(), 'Interner::lookup('];
            $this->renameProp($n, $id);
            $this->edits[] = [$n->getEndFilePos() + 1, $n->getEndFilePos() + 1, ')'];
        }
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
