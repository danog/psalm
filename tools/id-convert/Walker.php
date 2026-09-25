<?php

declare(strict_types=1);

namespace Psalm\Tools\IdConvert;

use PhpParser\Node;
use PhpParser\Node\Arg;
use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\Scalar;
use PhpParser\Node\Stmt;
use Psalm\Aliases;
use Psalm\Codebase;
use Psalm\Internal\Analyzer\ClassLikeAnalyzer;
use Psalm\Internal\MethodIdentifier;
use Psalm\NodeTypeProvider;
use Psalm\Storage\FunctionLikeStorage;
use Psalm\Storage\MethodStorage;
use Psalm\Type\Atomic;
use Psalm\Type\Atomic\TNamedObject;
use Psalm\Type\Atomic\TNull;
use Psalm\Type\Union;
use ReflectionFunction;
use Throwable;

/**
 * The body facts of one function-like (or of a file's top-level code): every value that moves between string
 * slots or into a string consumer, as a demand-driven walk. Each expression is visited with what its consumer
 * wants (a slot, a string, a comparison, or nothing in particular); the leaves (slot reads, literals, other
 * string values) meet that demand in a `flow` fact at their source range, which convert.php turns into an
 * intern / lookup wrap or a Sym literal when the two sides end up differently typed.
 *
 * Demands: ['s', slot] a slot (with array path), ['str'] a string consumer, ['interp'] a string interpolation,
 * ['any'] a consumer indifferent to the value (truth tests, null tests, statements), ['cmp', id] a comparison.
 * Sources: ['k' => 's', 's' => slot], ['k' => 'lit', 'v' => string], ['k' => 'x'] (another value), ['k' => 'null'].
 */
final class Walker
{
    private const LOCALS_OFF = ['compact', 'extract', 'get_defined_vars', 'func_get_args', 'func_get_arg', 'parse_str'];

    private string $src;
    private string $rel;
    /** the scope that owns the variables */
    private string $fn = '';
    /** @var array<string, true> the scope's parameters */
    private array $params = [];
    private ?string $self = null;
    private ?string $parent_class = null;
    private string $ns = '';
    private ?Aliases $aliases = null;
    private bool $test;
    /** @var array<string, list<?Union>> the types seen at each variable's occurrences */
    private array $var_types = [];
    /** @var array<string, true> the variables written somewhere */
    private array $var_written = [];
    /** @var array<string, true> the variables read somewhere */
    private array $var_reads = [];
    /** @var array<string, list<array<string, mixed>>> inline `@var` declarations of locals */
    private array $var_decls = [];
    private int $cmp = 0;
    /** @var list<array{int, array<string, mixed>}> leaves of the comparison being walked */
    private array $cmp_leaves = [];

    public function __construct(private Codebase $codebase, private string $file, private ?NodeTypeProvider $types)
    {
        $this->src = (string) file_get_contents($file);
        $this->rel = substr($file, strlen(ConvertPlugin::root()));
        $this->test = str_starts_with($this->rel, 'tests/');
    }

    public function setContext(?string $self, string $ns, ?Aliases $aliases): void
    {
        $this->self = $self;
        $this->ns = $ns;
        $this->aliases = $aliases;
        if ($self !== null) {
            try {
                $this->parent_class = $this->codebase->classlike_storage_provider->get($self)->parent_class;
            } catch (Throwable) {
                $this->parent_class = null;
            }
        }
    }

    // ---------------------------------------------------------------------------------------------- scopes

    public function functionLike(Node\FunctionLike $node, FunctionLikeStorage $storage): void
    {
        if ($node instanceof Stmt\ClassMethod) {
            $cls = $storage instanceof MethodStorage && $storage->defining_fqcln !== null
                ? $storage->defining_fqcln : (string) $this->self;
            $this->fn = strtolower($cls) . '::' . strtolower($node->name->name);
        } elseif ($node instanceof Stmt\Function_) {
            $this->fn = strtolower(ltrim($this->ns . '\\' . $node->name->name, '\\'));
        } else {
            $this->fn = self::closureId($this->rel, $node);
            // a closure's signature: its own slots (convert.php stops them unless a known caller ties them)
            $this->closureDecls($node, $storage);
        }
        foreach ($node->getParams() as $p) {
            if ($p->var instanceof Expr\Variable && is_string($p->var->name)) {
                $this->params[$p->var->name] = true;
            }
        }
        $stmts = $node instanceof Expr\ArrowFunction ? [new Stmt\Return_($node->expr, $node->expr->getAttributes())]
            : ($node->getStmts() ?? []);
        $this->stmts($stmts);
        $this->finishScope();
    }

    /** @param array<Node> $stmts top-level code of a file */
    public function file(array $stmts): void
    {
        $this->fn = 'file@' . $this->rel;
        $this->topLevel($stmts);
        $this->finishScope();
    }

    /** @param array<Node> $stmts */
    private function topLevel(array $stmts): void
    {
        foreach ($stmts as $s) {
            if ($s instanceof Stmt\Namespace_) {
                $this->ns = $s->name?->toString() ?? '';
                $this->topLevel($s->stmts);
            } elseif ($s instanceof Stmt\ClassLike || $s instanceof Stmt\Function_) {
                continue;
            } else {
                $this->stmts([$s]);
            }
        }
    }

    /** A declaration's default value, walked with the declared slot as its demand. */
    public function defaultValue(string $slot, Expr $e, ?string $self): void
    {
        $this->fn = 'decl@' . $this->rel;
        $this->self = $self;
        $this->expr($e, ['s', $slot]);
    }

    public static function closureId(string $rel, Node $node): string
    {
        return 'closure@' . $rel . ':' . $node->getStartFilePos();
    }

    private function closureDecls(Node\FunctionLike $node, FunctionLikeStorage $storage): void
    {
        foreach ($node->getParams() as $i => $p) {
            if (!$p->var instanceof Expr\Variable || !is_string($p->var->name)) {
                continue;
            }
            $decl = [];
            if ($p->type !== null) {
                $decl[] = ['t' => 'type', 'r' => $this->range($p->type), 'text' => $this->text($p->type)];
            }
            ConvertPlugin::out(['k' => 'slot', 's' => 'P:' . $this->fn . '|' . $p->var->name, 'file' => $this->file,
                'paths' => Types::paths($storage->params[$i]->type ?? null), 'decl' => $decl, 'closure' => true,
                'fn' => $this->fn, 'idx' => $i, 'member' => $p->var->name, 'byref' => $p->byRef, 'variadic' => $p->variadic,
                'tpl' => Types::templated($storage->params[$i]->type ?? null)]);
        }
        $rt = $node->getReturnType();
        ConvertPlugin::out(['k' => 'slot', 's' => 'R:' . $this->fn, 'file' => $this->file,
            'paths' => Types::paths($storage->return_type), 'closure' => true, 'fn' => $this->fn,
            'decl' => $rt !== null ? [['t' => 'type', 'r' => $this->range($rt), 'text' => $this->text($rt)]] : [],
            'tpl' => Types::templated($storage->return_type)]);
    }

    /** The scope's local slots: the string paths every occurrence of the variable agrees on. */
    private function finishScope(): void
    {
        foreach ($this->var_written as $name => $_) {
            if (!isset($this->var_reads[$name]) && !isset($this->params[$name])) {
                ConvertPlugin::out(['k' => 'unread', 's' => 'L:' . $this->fn . '|' . $name]);
            }
        }
        foreach ($this->var_types as $name => $types) {
            if ($name === 'this') {
                continue;
            }
            $paths = null;
            foreach ($types as $t) {
                if ($t === null || $t->isMixed()) {
                    $paths = [];
                    break;
                }
                if ($t->isNull() || $t->isNever() || self::isEmptyArray($t)) {
                    continue;
                }
                $p = Types::paths($t);
                if ($paths === null) {
                    $paths = $p;
                } else {
                    $both = array_intersect_key($paths, $p);
                    foreach ($both as $k => $nl) {
                        $both[$k] = $nl || $p[$k] || $t->isNullable();
                    }
                    $paths = $both;
                }
            }
            if (isset($this->params[$name])) {
                // a parameter: its declared paths stay only where the body agrees
                ConvertPlugin::out(['k' => 'body', 's' => 'P:' . $this->fn . '|' . $name, 'paths' => $paths ?? []]);
                continue;
            }
            if (($paths ?? []) === []) {
                continue;
            }
            ConvertPlugin::out(['k' => 'slot', 's' => 'L:' . $this->fn . '|' . $name, 'file' => $this->file,
                'paths' => $paths ?? [], 'decl' => $this->var_decls[$name] ?? [], 'fn' => $this->fn, 'member' => $name]);
        }
    }

    private static function isEmptyArray(Union $t): bool
    {
        foreach ($t->getAtomicTypes() as $a) {
            if (!$a instanceof Atomic\TArray || !$a->isEmptyArray()) {
                return false;
            }
        }
        return true;
    }

    // ---------------------------------------------------------------------------------------------- statements

    /** @param array<Node> $stmts */
    private function stmts(array $stmts): void
    {
        foreach ($stmts as $s) {
            $this->stmt($s);
        }
    }

    private function stmt(Node $s): void
    {
        $doc = $s->getDocComment();
        if ($doc !== null && str_contains($doc->getText(), '@var')) {
            $this->inlineVar($doc);
        }
        match (true) {
            $s instanceof Stmt\Expression => $this->expr($s->expr, ['any']),
            $s instanceof Stmt\Return_ => $s->expr !== null ? $this->expr($s->expr, $this->fn === '' || str_starts_with($this->fn, 'file@')
                ? ['str'] : ['s', 'R:' . $this->fn]) : null,
            $s instanceof Stmt\Echo_ => array_map(fn(Expr $e) => $this->expr($e, ['str']), $s->exprs),
            $s instanceof Stmt\If_ => $this->if($s),
            $s instanceof Stmt\While_ => [$this->expr($s->cond, ['any']), $this->stmts($s->stmts)],
            $s instanceof Stmt\Do_ => [$this->stmts($s->stmts), $this->expr($s->cond, ['any'])],
            $s instanceof Stmt\For_ => [array_map(fn(Expr $e) => $this->expr($e, ['any']), [...$s->init, ...$s->cond, ...$s->loop]),
                $this->stmts($s->stmts)],
            $s instanceof Stmt\Foreach_ => $this->foreach($s),
            $s instanceof Stmt\Switch_ => $this->switch($s),
            $s instanceof Stmt\TryCatch => $this->try($s),
            $s instanceof Stmt\Block => $this->stmts($s->stmts),
            $s instanceof Stmt\Unset_ => array_map(fn(Expr $e) => $this->expr($e, ['any']), $s->vars),
            $s instanceof Stmt\Global_ => array_map(fn(Expr $e) => $this->stopVar($e, 'global'), $s->vars),
            $s instanceof Stmt\Static_ => array_map(fn(Stmt\StaticVar $v) => [$this->stopVar($v->var, 'static'),
                $v->default !== null ? $this->expr($v->default, ['str']) : null], $s->vars),
            $s instanceof Stmt\Nop, $s instanceof Stmt\Break_, $s instanceof Stmt\Continue_, $s instanceof Stmt\Label,
            $s instanceof Stmt\Goto_, $s instanceof Stmt\InlineHTML, $s instanceof Stmt\Use_, $s instanceof Stmt\GroupUse,
            $s instanceof Stmt\Declare_, $s instanceof Stmt\ClassLike, $s instanceof Stmt\Function_,
            $s instanceof Stmt\HaltCompiler, $s instanceof Stmt\Const_ => null,
            default => $this->unknownStmt($s),
        };
    }

    private function unknownStmt(Node $s): void
    {
        foreach ($s->getSubNodeNames() as $sub) {
            $v = $s->$sub;
            foreach (is_array($v) ? $v : [$v] as $c) {
                if ($c instanceof Expr) {
                    $this->expr($c, ['str']);
                } elseif ($c instanceof Stmt) {
                    $this->stmt($c);
                }
            }
        }
    }

    private function if(Stmt\If_ $s): void
    {
        $this->expr($s->cond, ['any']);
        $this->stmts($s->stmts);
        foreach ($s->elseifs as $e) {
            $this->expr($e->cond, ['any']);
            $this->stmts($e->stmts);
        }
        if ($s->else !== null) {
            $this->stmts($s->else->stmts);
        }
    }

    private function try(Stmt\TryCatch $s): void
    {
        $this->stmts($s->stmts);
        foreach ($s->catches as $c) {
            if ($c->var !== null) {
                $this->stopVar($c->var, 'catch');
            }
            $this->stmts($c->stmts);
        }
        if ($s->finally !== null) {
            $this->stmts($s->finally->stmts);
        }
    }

    private function switch(Stmt\Switch_ $s): void
    {
        $id = $this->beginCmp();
        $this->expr($s->cond, ['cmp', $id, 0]);
        foreach ($s->cases as $c) {
            if ($c->cond !== null) {
                $this->expr($c->cond, ['cmp', $id, 1]);
            }
        }
        $this->endCmp($id, $s);
        foreach ($s->cases as $c) {
            $this->stmts($c->stmts);
        }
    }

    private function foreach(Stmt\Foreach_ $s): void
    {
        $arr = $this->arrSlot($s->expr);
        foreach (['#k' => $s->keyVar, '#v' => $s->valueVar] as $step => $v) {
            if ($v === null) {
                continue;
            }
            $this->bindTarget($v, $arr !== null ? $arr . $step : null, 'foreach');
        }
        $this->stmts($s->stmts);
    }

    /**
     * A target written from an array element that cannot be wrapped (foreach variables, destructuring): it is tied
     * to the element's slot, or stopped when the element has none.
     */
    private function bindTarget(Expr $target, ?string $elem, string $why): void
    {
        if ($target instanceof Expr\List_ || $target instanceof Expr\Array_) {
            foreach ($target->items as $it) {
                if ($it === null) {
                    continue;
                }
                $keyed = $it->key !== null;
                if ($keyed) {
                    $this->expr($it->key, ['any']);
                }
                // a keyed destructuring reads a shape's entries: no uniform element slot
                $this->bindTarget($it->value, $keyed || $elem === null ? null : $elem . '#v', $why);
            }
            return;
        }
        $slot = $this->targetSlot($target);
        if ($slot === null) {
            return;
        }
        if ($elem === null) {
            ConvertPlugin::out(['k' => 'stop', 's' => $slot, 'why' => $why . '-unknown', 'sub' => true]);
        } else {
            ConvertPlugin::out(['k' => 'tie', 'a' => $slot, 'b' => $elem, 'why' => $why, 'paths' => true]);
        }
    }

    private function stopVar(Expr $e, string $why): void
    {
        if ($e instanceof Expr\Variable && is_string($e->name)) {
            ConvertPlugin::out(['k' => 'stop', 's' => $this->varSlot($e->name), 'why' => $why, 'sub' => true]);
        }
    }

    private function inlineVar(\PhpParser\Comment\Doc $doc): void
    {
        if (!preg_match_all('/@(?:psalm-|phpstan-)?var\s+/', $doc->getText(), $m, PREG_OFFSET_CAPTURE)) {
            return;
        }
        foreach ($m[0] as [$whole, $at]) {
            $start = $at + strlen($whole);
            $rest = substr($doc->getText(), $start, 2000);
            $flat = (string) preg_replace_callback('/\n\s*\*/', static fn(array $x): string => str_repeat(' ', strlen($x[0])), $rest);
            try {
                $u = TypeStr::parse($flat);
            } catch (Throwable) {
                continue;
            }
            if (!preg_match('/^\s*\$(\w+)/', substr($flat, $u['end'], 200), $vm)) {
                continue;
            }
            $this->var_decls[$vm[1]][] = ['t' => 'doc', 'r' => [$doc->getStartFilePos() + $start,
                $doc->getStartFilePos() + $start + $u['end']], 'text' => substr($rest, 0, $u['end'])];
        }
    }

    // ---------------------------------------------------------------------------------------------- expressions

    /** @var list<Node> the expressions being walked (the consumer of a leaf is the one below it) */
    private array $stack = [];

    /** @param array<int, mixed> $d */
    private function expr(Expr $e, array $d): void
    {
        $this->stack[] = $e;
        try {
            $this->expr1($e, $d);
        } finally {
            array_pop($this->stack);
        }
    }

    /** What an expression is, for the report: its node kind, and the callee of a call. */
    private function describe(?Node $n): string
    {
        if ($n === null) {
            return '-';
        }
        $k = substr(strrchr('\\' . get_class($n), '\\'), 1);
        if ($n instanceof Expr\FuncCall && $n->name instanceof Name) {
            return 'fn:' . strtolower($n->name->getLast());
        }
        if (($n instanceof Expr\MethodCall || $n instanceof Expr\NullsafeMethodCall || $n instanceof Expr\StaticCall)
            && $n->name instanceof Identifier
        ) {
            return 'm:' . $n->name->name;
        }
        if ($n instanceof Expr\New_ && $n->class instanceof Name) {
            return 'new:' . $n->class->getLast();
        }
        return $k;
    }

    /** @param array<int, mixed> $d */
    private function expr1(Expr $e, array $d): void
    {
        switch (true) {
            case $e instanceof Expr\Ternary:
                if ($e->if === null) {
                    // `$a ?: $b`: the condition is also the value
                    $this->expr($e->cond, $d[0] === 'any' ? ['any'] : $d);
                } else {
                    $this->expr($e->cond, ['any']);
                    $this->expr($e->if, $d);
                }
                $this->expr($e->else, $d);
                return;
            case $e instanceof Expr\BinaryOp\Coalesce:
                // the left operand may be undefined: a wrap keeps `?? null` inside
                $this->expr($e->left, $d + ['co' => true]);
                $this->expr($e->right, $d);
                return;
            case $e instanceof Expr\Match_:
                $this->match($e, $d);
                return;
            case $e instanceof Expr\ErrorSuppress:
                $this->expr($e->expr, $d);
                return;
            case $e instanceof Expr\Array_ && ($d[0] === 's' || $d[0] === 'cmp'):
                $this->arrayLiteral($e, $d);
                return;
            case $e instanceof Expr\Assign:
                $target = $this->assign($e);
                if ($d[0] !== 'any') {
                    $this->use($target !== null ? ['k' => 's', 's' => $target] : ['k' => 'x'], $e, $d);
                }
                return;
        }
        $this->use($this->src($e), $e, $d);
    }

    /**
     * @param array<string, mixed> $src
     * @param array<int, mixed> $d
     */
    private function use(array $src, Expr $e, array $d): void
    {
        if ($src['k'] === 'none' || $src['k'] === 'null') {
            return;
        }
        $t = $this->type($e);
        switch ($d[0]) {
            case 'any':
                return;
            case 'cmp':
                $this->cmp_leaves[] = [$d[2], $this->leaf($src, $e, $t, isset($d['co']))];
                return;
            case 's':
                $this->note(['k' => 'f', 'dst' => $d[1]] + $this->leaf($src, $e, $t, isset($d['co'])));
                return;
            case 'str':
            case 'interp':
                if ($src['k'] === 's') {
                    $this->note(['k' => 'f', 'dst' => $d[0]] + (isset($d[1]) ? ['ctx' => $d[1]] : []) + $this->leaf($src, $e, $t, isset($d['co'])));
                }
                return;
        }
    }

    /**
     * @param array<string, mixed> $src
     * @return array<string, mixed>
     */
    private function leaf(array $src, Expr $e, ?Union $t, bool $co = false): array
    {
        $leaf = ['r' => $this->range($e), 'src' => $src, 'nl' => $t === null || $t->isNullable() || $t->possibly_undefined];
        if ($t !== null && !$t->isNullable()) {
            $leaf['nl'] = false;
        }
        $leaf['sc'] = $t === null ? null : Types::stringish($t);
        $n = count($this->stack);
        $leaf['via'] = $this->describe($this->stack[$n - 1] === $e ? ($this->stack[$n - 2] ?? null) : ($this->stack[$n - 1] ?? null));
        if ($src['k'] === 'x') {
            $leaf['xk'] = $this->describe($e);
        }
        if ($co) {
            $leaf['co'] = true;
        }
        if ($src['k'] === 'x') {
            $leaf['ap'] = Types::paths($t);
        }
        if ($e instanceof Expr\Variable || $e instanceof Expr\PropertyFetch || $e instanceof Expr\ArrayDimFetch) {
            // inside a string: `"{$x}"` / `"$x"` (the edit then leaves the string)
            $s = $e->getStartFilePos();
            $leaf['brace'] = $s > 0 && $this->src[$s - 1] === '{';
        }
        return $leaf;
    }

    /** @param array<int, mixed> $d */
    private function match(Expr\Match_ $e, array $d): void
    {
        $true = $e->cond instanceof Expr\ConstFetch && strtolower($e->cond->name->toString()) === 'true';
        $id = $true ? null : $this->beginCmp();
        if ($id !== null) {
            $this->expr($e->cond, ['cmp', $id, 0]);
        } else {
            $this->expr($e->cond, ['any']);
        }
        foreach ($e->arms as $arm) {
            foreach ($arm->conds ?? [] as $c) {
                $this->expr($c, $id !== null ? ['cmp', $id, 1] : ['any']);
            }
        }
        if ($id !== null) {
            $this->endCmp($id, $e);
        }
        foreach ($e->arms as $arm) {
            $this->expr($arm->body, $d);
        }
    }

    private function beginCmp(): int
    {
        $this->cmp++;
        // comparisons nest (an operand may compare inside): each keeps its own leaves
        $this->cmp_stack[] = $this->cmp_leaves;
        $this->cmp_leaves = [];
        return $this->cmp;
    }

    /** @var list<list<array{int, array<string, mixed>}>> */
    private array $cmp_stack = [];

    private function endCmp(int $id, Node $n): void
    {
        if ($this->cmp_leaves !== []) {
            $this->note(['k' => 'cmp', 'id' => $this->rel . ':' . $n->getStartFilePos() . ':' . $id, 'leaves' => $this->cmp_leaves]);
        }
        $this->cmp_leaves = array_pop($this->cmp_stack) ?? [];
    }

    /**
     * An array literal meeting a slot demand: its keys meet the slot's `#k`, its values `#v`; a comparison demand
     * (in_array against a literal list) passes to each value.
     *
     * @param array<int, mixed> $d
     */
    private function arrayLiteral(Expr\Array_ $e, array $d): void
    {
        $spread = $e->items !== [];
        foreach ($e->items as $it) {
            $spread = $spread && $it !== null && $it->unpack;
        }
        if ($spread && $d[0] === 's') {
            // `[...$a, ...$b]` renumbers int keys: with ids as keys it becomes array_replace($a, $b)
            $this->note(['k' => 'spread', 's' => $d[1], 'r' => $this->range($e),
                'items' => array_map(fn(Node\ArrayItem $it): int => $it->getStartFilePos(), $e->items)]);
        }
        foreach ($e->items as $it) {
            if ($it === null) {
                continue;
            }
            if ($it->unpack) {
                if ($d[0] === 's') {
                    $this->expr($it->value, $d);
                } else {
                    $this->expr($it->value, ['str']);
                }
                continue;
            }
            if ($it->key !== null) {
                $this->expr($it->key, $d[0] === 's' ? ['s', $d[1] . '#k'] : ['any']);
            }
            $this->expr($it->value, $d[0] === 's' ? ['s', $d[1] . '#v'] : $d);
        }
    }

    /**
     * Walks an expression's parts and says what its value is.
     *
     * @return array<string, mixed>
     */
    private function src(Expr $e): array
    {
        switch (true) {
            case $e instanceof Scalar\String_:
                return ['k' => 'lit', 'v' => $e->value];
            case $e instanceof Expr\ClassConstFetch:
                return $this->classConst($e);
            case $e instanceof Expr\ConstFetch:
                return strtolower($e->name->toString()) === 'null' ? ['k' => 'null'] : ['k' => 'x'];
            case $e instanceof Expr\Variable:
                if (!is_string($e->name)) {
                    $this->expr($e->name, ['str', 'brace']);
                    $this->note(['k' => 'scopeoff', 'fn' => $this->fn, 'why' => 'variable-variable']);
                    return ['k' => 'x'];
                }
                if ($e->name === 'this') {
                    return ['k' => 'x'];
                }
                $this->var_types[$e->name][] = $this->type($e);
                $this->var_reads[$e->name] = true;
                return ['k' => 's', 's' => $this->varSlot($e->name)];
            case $e instanceof Expr\PropertyFetch || $e instanceof Expr\NullsafePropertyFetch:
                return $this->propFetch($e);
            case $e instanceof Expr\StaticPropertyFetch:
                return $this->staticPropFetch($e);
            case $e instanceof Expr\ArrayDimFetch:
                return $this->dimFetch($e);
            case $e instanceof Expr\Assign:
                $t = $this->assign($e);
                return $t !== null ? ['k' => 's', 's' => $t] : ['k' => 'x'];
            case $e instanceof Expr\AssignRef:
                foreach ([$e->var, $e->expr] as $side) {
                    $slot = $this->targetSlot($side);
                    if ($slot !== null) {
                        ConvertPlugin::out(['k' => 'stop', 's' => $slot, 'why' => 'reference', 'sub' => true]);
                    }
                    $this->expr($side, ['any']);
                }
                return ['k' => 'x'];
            case $e instanceof Expr\AssignOp:
                $slot = $this->targetSlot($e->var);
                if ($slot !== null && $e instanceof Expr\AssignOp\Concat) {
                    ConvertPlugin::out(['k' => 'stop', 's' => $slot, 'why' => 'concat-assign']);
                }
                if ($slot !== null && $e instanceof Expr\AssignOp\Plus && $this->isArray($e->var)) {
                    $this->expr($e->expr, ['s', $slot]);
                    return ['k' => 's', 's' => $slot];
                }
                if ($slot !== null && $e instanceof Expr\AssignOp\Coalesce) {
                    $this->expr($e->var, ['any']);
                    $this->expr($e->expr, ['s', $slot]);
                    return ['k' => 's', 's' => $slot];
                }
                $this->expr($e->var, ['str']);
                $this->expr($e->expr, ['str']);
                return ['k' => 'x'];
            case $e instanceof Expr\BinaryOp\Identical || $e instanceof Expr\BinaryOp\NotIdentical
                || $e instanceof Expr\BinaryOp\Equal || $e instanceof Expr\BinaryOp\NotEqual:
                $this->compare($e);
                return ['k' => 'x'];
            case $e instanceof Expr\BinaryOp\BooleanAnd || $e instanceof Expr\BinaryOp\BooleanOr
                || $e instanceof Expr\BinaryOp\LogicalAnd || $e instanceof Expr\BinaryOp\LogicalOr
                || $e instanceof Expr\BinaryOp\LogicalXor:
                $this->expr($e->left, ['any']);
                $this->expr($e->right, ['any']);
                return ['k' => 'x'];
            case $e instanceof Expr\BinaryOp\Plus && $this->isArray($e->left) && $this->isArray($e->right):
                // an array union: both operands' elements end up in the result
                $E = 'E:' . $this->rel . ':' . $e->getStartFilePos() . ':+';
                ConvertPlugin::out(['k' => 'slot', 's' => $E, 'file' => $this->file, 'paths' => Types::paths($this->type($e)),
                    'synthetic' => 'union', 'decl' => []]);
                $this->expr($e->left, ['s', $E]);
                $this->expr($e->right, ['s', $E]);
                return ['k' => 's', 's' => $E];
            case $e instanceof Expr\BinaryOp:
                // concatenation, ordering, arithmetic: string operands
                $this->expr($e->left, ['str']);
                $this->expr($e->right, ['str']);
                return ['k' => 'x'];
            case $e instanceof Expr\BooleanNot:
                $this->expr($e->expr, ['any']);
                return ['k' => 'x'];
            case $e instanceof Expr\Isset_:
                foreach ($e->vars as $v) {
                    $this->expr($v, ['any']);
                }
                return ['k' => 'x'];
            case $e instanceof Expr\Empty_:
                $this->expr($e->expr, ['any']);
                return ['k' => 'x'];
            case $e instanceof Expr\Instanceof_:
                $this->expr($e->expr, ['any']);
                if ($e->class instanceof Expr) {
                    $this->expr($e->class, ['str', 'paren']);
                }
                return ['k' => 'x'];
            case $e instanceof Expr\Cast\Bool_ || $e instanceof Expr\Cast\Array_ || $e instanceof Expr\Cast\Object_
                || $e instanceof Expr\Cast\Unset_:
                $this->expr($e->expr, $e instanceof Expr\Cast\Bool_ ? ['any'] : ['str']);
                return ['k' => 'x'];
            case $e instanceof Expr\Cast:
                $this->expr($e->expr, ['str']);
                return ['k' => 'x'];
            case $e instanceof Scalar\InterpolatedString || $e instanceof Scalar\Encapsed:
                $heredoc = $e->getAttribute('kind') === Scalar\String_::KIND_HEREDOC;
                foreach ($e->parts as $p) {
                    if ($p instanceof Expr) {
                        $this->expr($p, [$heredoc ? 'str' : 'interp']);
                        if ($heredoc) {
                            $this->note(['k' => 'heredoc', 'r' => $this->range($p)]);
                        }
                    }
                }
                return ['k' => 'x'];
            case $e instanceof Expr\Closure:
                $this->closureUses($e, array_map(static fn(Node\ClosureUse $u): array => [$u->var, $u->byRef], $e->uses));
                return ['k' => 'x'];
            case $e instanceof Expr\ArrowFunction:
                $this->arrowCaptures($e);
                return ['k' => 'x'];
            case $e instanceof Expr\FuncCall:
                return $this->funcCall($e);
            case $e instanceof Expr\MethodCall || $e instanceof Expr\NullsafeMethodCall || $e instanceof Expr\StaticCall
                || $e instanceof Expr\New_:
                return $this->call($e);
            case $e instanceof Expr\Array_:
                foreach ($e->items as $it) {
                    if ($it !== null) {
                        if ($it->key !== null) {
                            $this->expr($it->key, ['str']);
                        }
                        $this->expr($it->value, ['str']);
                    }
                }
                return ['k' => 'x'];
            case $e instanceof Expr\Exit_ || $e instanceof Expr\Print_ || $e instanceof Expr\Include_
                || $e instanceof Expr\Eval_ || $e instanceof Expr\ShellExec:
                foreach ($e->getSubNodeNames() as $sub) {
                    foreach (is_array($e->$sub) ? $e->$sub : [$e->$sub] as $c) {
                        if ($c instanceof Expr) {
                            $this->expr($c, ['str']);
                        }
                    }
                }
                return ['k' => 'x'];
            case $e instanceof Expr\Throw_ || $e instanceof Expr\Clone_ || $e instanceof Expr\PreInc
                || $e instanceof Expr\PostInc || $e instanceof Expr\PreDec || $e instanceof Expr\PostDec
                || $e instanceof Expr\UnaryMinus || $e instanceof Expr\UnaryPlus || $e instanceof Expr\BitwiseNot:
                $inner = $e instanceof Expr\Throw_ || $e instanceof Expr\Clone_ || $e instanceof Expr\UnaryMinus
                    || $e instanceof Expr\UnaryPlus || $e instanceof Expr\BitwiseNot ? $e->expr : $e->var;
                $this->expr($inner, $e instanceof Expr\Throw_ || $e instanceof Expr\Clone_ ? ['any'] : ['str']);
                return ['k' => 'x'];
            case $e instanceof Expr\Yield_:
                if ($e->key !== null) {
                    $this->expr($e->key, ['str']);
                }
                if ($e->value !== null) {
                    $this->expr($e->value, ['str']);
                }
                return ['k' => 'x'];
            case $e instanceof Expr\YieldFrom:
                $this->expr($e->expr, ['str']);
                return ['k' => 'x'];
            case $e instanceof Expr\List_:
                return ['k' => 'x'];
            case $e instanceof Scalar:
                return ['k' => 'x'];
        }
        // anything else: its parts are string consumers
        foreach ($e->getSubNodeNames() as $sub) {
            foreach (is_array($e->$sub) ? $e->$sub : [$e->$sub] as $c) {
                if ($c instanceof Expr) {
                    $this->expr($c, ['str']);
                } elseif ($c instanceof Arg) {
                    $this->expr($c->value, ['str']);
                }
            }
        }
        return ['k' => 'x'];
    }

    private function compare(Expr\BinaryOp $e): void
    {
        if ($this->isNull($e->left) || $this->isNull($e->right) || $this->isBool($e->left) || $this->isBool($e->right)) {
            $this->expr($e->left, ['any']);
            $this->expr($e->right, ['any']);
            return;
        }
        $id = $this->beginCmp();
        $this->expr($e->left, ['cmp', $id, 0]);
        $this->expr($e->right, ['cmp', $id, 1]);
        $this->endCmp($id, $e);
    }

    /** @return array<string, mixed> */
    private function classConst(Expr\ClassConstFetch $e): array
    {
        if (!$e->name instanceof Identifier) {
            $this->expr($e->name, ['str', 'brace']);
            return ['k' => 'x'];
        }
        if (!$e->class instanceof Name) {
            $this->expr($e->class, ['str', 'paren']);
            return ['k' => 'x'];
        }
        $cls = $this->className($e->class);
        if (strtolower($e->name->name) === 'class') {
            $lc = strtolower($e->class->toString());
            if ($lc === 'static') {
                return ['k' => 'x'];
            }
            return $cls === null ? ['k' => 'x'] : ['k' => 'lit', 'v' => $cls, 'cls' => true];
        }
        if ($cls === null) {
            return ['k' => 'x'];
        }
        try {
            $storage = $this->codebase->classlike_storage_provider->get($cls);
        } catch (Throwable) {
            return ['k' => 'x'];
        }
        // the declaring class-like of an inherited constant
        $decl = $this->constDeclarer($storage->name, $e->name->name);
        return $decl === null ? ['k' => 'x'] : ['k' => 's', 's' => 'C:' . strtolower($decl) . '::' . $e->name->name];
    }

    private function constDeclarer(string $cls, string $name, int $depth = 0): ?string
    {
        if ($depth > 10) {
            return null;
        }
        try {
            $s = $this->codebase->classlike_storage_provider->get($cls);
        } catch (Throwable) {
            return null;
        }
        if (isset($s->constants[$name])) {
            return $s->constants[$name]->declaring_class ?? $s->name;
        }
        foreach ([$s->parent_class, ...array_values($s->direct_class_interfaces), ...array_values($s->used_traits)] as $up) {
            if ($up !== null && ($r = $this->constDeclarer($up, $name, $depth + 1)) !== null) {
                return $r;
            }
        }
        return null;
    }

    /** @return array<string, mixed> */
    private function propFetch(Expr\PropertyFetch|Expr\NullsafePropertyFetch $e): array
    {
        $this->expr($e->var, ['any']);
        if (!$e->name instanceof Identifier) {
            $this->expr($e->name, ['str', 'brace']);
            return ['k' => 'x'];
        }
        $slots = $this->propSlots($e->var, $e->name->name);
        if ($slots === null) {
            $this->note(['k' => 'unres', 'm' => 'prop', 'name' => $e->name->name, 'r' => $this->range($e)]);
            return ['k' => 'x'];
        }
        if ($slots === []) {
            return ['k' => 'x'];
        }
        if (count($slots) === 1) {
            return ['k' => 's', 's' => $slots[0]];
        }
        // a union receiver: the classes' slots may differ; a simple receiver is dispatched on at run time
        $recv = $this->simpleReceiver($e->var);
        if ($recv === null || $e instanceof Expr\NullsafePropertyFetch) {
            $this->tieAll($slots, 'receiver-union');
            return ['k' => 's', 's' => $slots[0]];
        }
        $alts = [];
        foreach ($this->classesOf($this->type($e->var)) ?? [] as $c) {
            $slot = $this->propSlot($c, $e->name->name);
            if ($slot !== null) {
                $alts[$slot][] = $c;
            }
        }
        return ['k' => 's', 's' => $slots[0], 'alts' => $alts, 'recv' => $recv];
    }

    /** The source of a receiver that can be evaluated twice (a variable or a property chain on one). */
    private function simpleReceiver(Expr $e): ?string
    {
        $x = $e;
        while ($x instanceof Expr\PropertyFetch && $x->name instanceof Identifier) {
            $x = $x->var;
        }
        return $x instanceof Expr\Variable && is_string($x->name) ? $this->text($e) : null;
    }

    /** @return array<string, mixed> */
    private function staticPropFetch(Expr\StaticPropertyFetch $e): array
    {
        if (!$e->name instanceof Identifier) {
            $this->expr($e->name, ['str', 'brace']);
            return ['k' => 'x'];
        }
        if (!$e->class instanceof Name) {
            $this->expr($e->class, ['str', 'paren']);
            return ['k' => 'x'];
        }
        $cls = $this->className($e->class);
        $slot = $cls === null ? null : $this->propSlot($cls, $e->name->name);
        return $slot === null ? ['k' => 'x'] : ['k' => 's', 's' => $slot];
    }

    /** @return array<string, mixed> */
    private function dimFetch(Expr\ArrayDimFetch $e): array
    {
        $bt = $this->type($e->var);
        if ($bt !== null && $bt->isString()) {
            // a string offset: the string itself is read
            $this->expr($e->var, ['str']);
            if ($e->dim !== null) {
                $this->expr($e->dim, ['any']);
            }
            return ['k' => 'x'];
        }
        $base = $this->src($e->var);
        $slot = $base['k'] === 's' ? $base['s'] : null;
        if ($e->dim !== null) {
            $this->expr($e->dim, $slot !== null ? ['s', $slot . '#k'] : ['str']);
        }
        return $slot !== null ? ['k' => 's', 's' => $slot . '#v'] : ['k' => 'x'];
    }

    /** Walks an assignment; returns the slot written. */
    private function assign(Expr\Assign $e): ?string
    {
        $v = $e->var;
        if ($v instanceof Expr\List_ || $v instanceof Expr\Array_) {
            $this->bindTarget($v, $this->arrSlot($e->expr), 'destructure');
            return null;
        }
        $slot = $this->targetSlot($v);
        if ($v instanceof Expr\Variable && is_string($v->name)) {
            $this->var_types[$v->name][] = $this->type($e->expr) ?? $this->type($v);
        }
        $this->expr($e->expr, $slot !== null ? ['s', $slot] : ['str']);
        return $slot;
    }

    /**
     * The slot an assignment target writes (walking the parts that are read: receivers, dimension keys).
     */
    private function targetSlot(Expr $v): ?string
    {
        if ($v instanceof Expr\Variable) {
            if (!is_string($v->name)) {
                $this->note(['k' => 'scopeoff', 'fn' => $this->fn, 'why' => 'variable-variable']);
                return null;
            }
            if ($v->name !== 'this') {
                $this->var_written[$v->name] = true;
            }
            return $v->name === 'this' ? null : $this->varSlot($v->name);
        }
        if ($v instanceof Expr\PropertyFetch || $v instanceof Expr\NullsafePropertyFetch) {
            $this->expr($v->var, ['any']);
            if (!$v->name instanceof Identifier) {
                $this->expr($v->name, ['str', 'brace']);
                return null;
            }
            $slots = $this->propSlots($v->var, $v->name->name);
            if ($slots === null) {
                $this->note(['k' => 'unres', 'm' => 'prop', 'name' => $v->name->name, 'r' => $this->range($v)]);
                return null;
            }
            if ($slots === []) {
                return null;
            }
            $this->tieAll($slots, 'receiver-union');
            return $slots[0];
        }
        if ($v instanceof Expr\StaticPropertyFetch) {
            $s = $this->staticPropFetch($v);
            return $s['k'] === 's' ? $s['s'] : null;
        }
        if ($v instanceof Expr\ArrayDimFetch) {
            $base = $this->targetSlot($v->var);
            if ($v->dim !== null) {
                $this->expr($v->dim, $base !== null ? ['s', $base . '#k'] : ['str']);
            }
            return $base !== null ? $base . '#v' : null;
        }
        $this->expr($v, ['any']);
        return null;
    }

    private function varSlot(string $name): string
    {
        return (isset($this->params[$name]) ? 'P:' : 'L:') . $this->fn . '|' . $name;
    }

    /**
     * The slot an array-valued expression's elements live in: the slot it reads, or a synthetic slot (E) that the
     * expression's leaves flow into (a ternary of two slots, a literal, a call).
     */
    private function arrSlot(Expr $e): ?string
    {
        while ($e instanceof Expr\ErrorSuppress) {
            $e = $e->expr;
        }
        if ($e instanceof Expr\Variable || $e instanceof Expr\PropertyFetch || $e instanceof Expr\NullsafePropertyFetch
            || $e instanceof Expr\StaticPropertyFetch || $e instanceof Expr\ArrayDimFetch || $e instanceof Expr\MethodCall
            || $e instanceof Expr\NullsafeMethodCall || $e instanceof Expr\StaticCall || $e instanceof Expr\ClassConstFetch
            || $e instanceof Expr\Assign || ($e instanceof Expr\FuncCall && !$e->isFirstClassCallable())
        ) {
            $s = $this->src($e);
            if ($s['k'] === 's') {
                return $s['s'];
            }
            $t = $this->type($e);
            if ($s['k'] === 'x' && $t !== null && Types::paths($t) !== []) {
                // a foreign array: its own synthetic slot, stopped where it holds strings
                $E = 'E:' . $this->rel . ':' . $e->getStartFilePos() . ':x';
                ConvertPlugin::out(['k' => 'slot', 's' => $E, 'file' => $this->file, 'paths' => Types::paths($t),
                    'synthetic' => 'foreign', 'decl' => []]);
                // the foreign array enters here (its keys / values interned where they hold names)
                $this->note(['k' => 'f', 'dst' => $E, 'src' => ['k' => 'x'], 'r' => $this->range($e), 'nl' => false,
                    'sc' => false, 'ap' => Types::paths($t), 'via' => 'foreign', 'xk' => $this->describe($e)]);
                return $E;
            }
            return null;
        }
        $E = 'E:' . $this->rel . ':' . $e->getStartFilePos() . ':a';
        ConvertPlugin::out(['k' => 'slot', 's' => $E, 'file' => $this->file, 'paths' => Types::paths($this->type($e)),
            'synthetic' => 'array-expr', 'decl' => []]);
        $this->expr($e, ['s', $E]);
        return $E;
    }

    /** @param list<string> $slots */
    private function tieAll(array $slots, string $why): void
    {
        for ($i = 1; $i < count($slots); $i++) {
            ConvertPlugin::out(['k' => 'tie', 'a' => $slots[0], 'b' => $slots[$i], 'why' => $why, 'paths' => true]);
        }
    }

    /**
     * The property slots `$var->name` may read: [] when the property is not a declared one of a known class,
     * null when the receiver is unknown.
     *
     * @return ?list<string>
     */
    private function propSlots(Expr $var, string $name): ?array
    {
        $classes = $var instanceof Expr\Variable && $var->name === 'this' && $this->self !== null
            ? ($this->selfIsTrait() ? $this->traitUsers() : [$this->self]) : $this->classesOf($this->type($var));
        if ($classes === null) {
            return null;
        }
        $out = [];
        foreach ($classes as $c) {
            $s = $this->propSlot($c, $name);
            if ($s === null) {
                // an undeclared (magic / dynamic) property
                return null;
            }
            $out[$s] = true;
        }
        if (count($out) > 1 && $var instanceof Expr\Variable && $var->name === 'this') {
            // a trait's one body serves every class using it: their properties are one slot kind
            $this->tieAll(array_keys($out), 'trait-users');
            return [array_key_first($out)];
        }
        return array_keys($out);
    }

    /** @var array<string, list<string>> */
    private static array $trait_users = [];

    /**
     * The classes using the current trait (its one body serves them all).
     *
     * @return list<string>
     */
    private function traitUsers(): array
    {
        $lc = strtolower((string) $this->self);
        if (!isset(self::$trait_users[$lc])) {
            $users = [];
            foreach ($this->codebase->classlike_storage_provider->getAll() as $st) {
                if (isset($st->used_traits[$lc]) && !$st->is_trait) {
                    $users[] = $st->name;
                }
            }
            self::$trait_users[$lc] = $users === [] ? [(string) $this->self] : $users;
        }
        return self::$trait_users[$lc];
    }

    /** Whether the current class-like is a trait. */
    private function selfIsTrait(): bool
    {
        try {
            return $this->self !== null && $this->codebase->classlike_storage_provider->get($this->self)->is_trait;
        } catch (Throwable) {
            return false;
        }
    }

    private function propSlot(string $cls, string $name): ?string
    {
        try {
            $storage = $this->codebase->classlike_storage_provider->get($cls);
        } catch (Throwable) {
            return null;
        }
        $decl = $storage->declaring_property_ids[$name] ?? null;
        if ($decl === null) {
            return null;
        }
        try {
            $ds = $this->codebase->classlike_storage_provider->get($decl);
        } catch (Throwable) {
            return null;
        }
        return 'F:' . strtolower($ds->name) . '::$' . $name;
    }

    /** @return ?list<string> the named-object classes of a receiver type, null if some atom is not one */
    private function classesOf(?Union $t): ?array
    {
        if ($t === null) {
            return null;
        }
        $out = [];
        foreach ($t->getAtomicTypes() as $a) {
            if ($a instanceof TNull) {
                continue;
            }
            if ($a instanceof Atomic\TTemplateParam) {
                $inner = $this->classesOf($a->as);
                if ($inner === null) {
                    return null;
                }
                $out = [...$out, ...$inner];
                continue;
            }
            if (!$a instanceof TNamedObject) {
                return null;
            }
            $out[] = $a->value;
            foreach ($a->extra_types as $x) {
                if ($x instanceof TNamedObject) {
                    $out[] = $x->value;
                }
            }
        }
        return $out === [] ? null : array_values(array_unique($out));
    }

    private function className(Name $n): ?string
    {
        $lc = strtolower($n->toString());
        if ($lc === 'self' || $lc === 'static') {
            return $this->self;
        }
        if ($lc === 'parent') {
            return $this->parent_class;
        }
        $resolved = $n->getAttribute('resolvedName');
        if (is_string($resolved)) {
            return $resolved;
        }
        if ($this->aliases !== null) {
            return ClassLikeAnalyzer::getFQCLNFromNameObject($n, $this->aliases);
        }
        return $n->isFullyQualified() ? $n->toString() : ltrim($this->ns . '\\' . $n->toString(), '\\');
    }

    // ---------------------------------------------------------------------------------------------- calls

    /** @return array<string, mixed> */
    private function call(Expr\MethodCall|Expr\NullsafeMethodCall|Expr\StaticCall|Expr\New_ $e): array
    {
        $fns = null;
        if ($e instanceof Expr\New_) {
            if ($e->class instanceof Stmt\Class_) {
                $this->args($e->args, null);
                return ['k' => 'x'];
            }
            if ($e->class instanceof Expr) {
                $this->expr($e->class, ['str', 'paren']);
            } else {
                $cls = $this->className($e->class);
                $fns = $cls === null ? null : $this->methodFns([$cls], '__construct');
            }
        } elseif ($e instanceof Expr\StaticCall) {
            if ($e->class instanceof Expr) {
                $this->expr($e->class, ['str', 'paren']);
            } elseif ($e->name instanceof Identifier) {
                $cls = $this->className($e->class);
                $fns = $cls === null ? null : $this->methodFns([$cls], strtolower($e->name->name));
            }
        } else {
            $this->expr($e->var, ['any']);
            if ($e->name instanceof Identifier) {
                $classes = $this->classesOf($this->type($e->var));
                $fns = $classes === null ? null : $this->methodFns($classes, strtolower($e->name->name));
            }
        }
        if (($e instanceof Expr\MethodCall || $e instanceof Expr\NullsafeMethodCall || $e instanceof Expr\StaticCall)
            && !$e->name instanceof Identifier
        ) {
            $this->expr($e->name, ['str', 'brace']);
        }
        if ($e->isFirstClassCallable()) {
            if ($fns !== null) {
                foreach ($fns as $fn) {
                    ConvertPlugin::out(['k' => 'stop', 's' => 'R:' . $fn, 'why' => 'first-class-callable', 'fnall' => $fn]);
                }
            }
            return ['k' => 'x'];
        }
        if ($fns === null) {
            if (!$e instanceof Expr\New_) {
                $this->note(['k' => 'unres', 'm' => 'call', 'name' => $e->name instanceof Identifier ? $e->name->name : null,
                    'r' => $this->range($e)]);
            }
            $this->args($e->getArgs(), null);
            return ['k' => 'x'];
        }
        $this->tieFns($fns);
        $this->args($e->getArgs(), $fns[0]);
        return $e instanceof Expr\New_ ? ['k' => 'x'] : ['k' => 's', 's' => 'R:' . $fns[0]];
    }

    /** @param list<string> $fns */
    private function tieFns(array $fns): void
    {
        for ($i = 1; $i < count($fns); $i++) {
            ConvertPlugin::out(['k' => 'tiefn', 'a' => $fns[0], 'b' => $fns[$i]]);
        }
    }

    /**
     * The declaring methods a call on these classes reaches ("class::method", lowercase); null if one is unknown.
     *
     * @param list<string> $classes
     * @return ?list<string>
     */
    private function methodFns(array $classes, string $lc): ?array
    {
        $out = [];
        foreach ($classes as $c) {
            try {
                $decl = $this->codebase->methods->getDeclaringMethodId(new MethodIdentifier($c, $lc));
            } catch (Throwable) {
                return null;
            }
            if ($decl === null) {
                return null;
            }
            $out[strtolower($decl->fq_class_name) . '::' . $lc] = true;
        }
        return array_keys($out);
    }

    /** @return ?list<\Psalm\Storage\FunctionLikeParameter> */
    private function fnParams(string $fn): ?array
    {
        try {
            if (str_contains($fn, '::')) {
                [$c, $m] = explode('::', $fn, 2);
                return $this->codebase->methods->getStorage(new MethodIdentifier($c, $m))->params;
            }
            return $this->codebase->functions->getStorage(null, $fn)->params;
        } catch (Throwable) {
            return null;
        }
    }

    /**
     * Arguments of a call to a project function-like (its parameter slots are the demands) or to an unknown
     * callee (strings).
     *
     * @param array<Arg|Node\VariadicPlaceholder> $args
     */
    private function args(array $args, ?string $fn): void
    {
        $params = $fn !== null ? $this->fnParams($fn) : null;
        foreach ($args as $i => $a) {
            if (!$a instanceof Arg) {
                continue;
            }
            $p = null;
            if ($params !== null) {
                if ($a->name !== null) {
                    foreach ($params as $pp) {
                        if ($pp->name === $a->name->name) {
                            $p = $pp;
                        }
                    }
                } else {
                    $p = $params[$i] ?? null;
                    if ($p === null && $params !== [] && end($params)->is_variadic) {
                        $p = end($params);
                    }
                }
            }
            if ($a->unpack) {
                $s = $this->src($a->value);
                $ps = [];
                foreach (array_slice($params ?? [], $i) as $pp) {
                    $ps[] = 'P:' . $fn . '|' . $pp->name;
                }
                // the spread fills the parameters from here on: its values meet them (a wrap for the whole list)
                $this->note(['k' => 'unpack', 'r' => $this->range($a->value), 'src' => $s, 'ps' => $ps]);
                continue;
            }
            if ($p === null) {
                $this->expr($a->value, ['str']);
                continue;
            }
            $slot = 'P:' . $fn . '|' . $p->name;
            if ($p->by_ref) {
                $t = $this->targetSlot($a->value);
                if ($t !== null) {
                    ConvertPlugin::out(['k' => 'tie', 'a' => $t, 'b' => $slot, 'why' => 'by-ref', 'paths' => true]);
                }
                continue;
            }
            if ($p->is_variadic) {
                ConvertPlugin::out(['k' => 'stop', 's' => $slot, 'why' => 'variadic', 'sub' => true]);
                $this->expr($a->value, ['str']);
                continue;
            }
            if ($a->value instanceof Expr\Array_ && ($props = $this->propertyShape($fn, $p)) !== null) {
                // a shape keyed by the callee class's property names (setProperties): each entry is that property
                foreach ($a->value->items as $it) {
                    if ($it !== null && $it->key instanceof Scalar\String_ && isset($props[$it->key->value])) {
                        $this->expr($it->value, ['s', $props[$it->key->value]]);
                    } elseif ($it !== null) {
                        $this->expr($it->value, ['str']);
                    }
                }
                continue;
            }
            $this->expr($a->value, ['s', $slot]);
        }
    }

    /**
     * The property slots a shape parameter's keys name, when every key is a property of the method's class.
     *
     * @return ?array<string, string>
     */
    private function propertyShape(string $fn, \Psalm\Storage\FunctionLikeParameter $p): ?array
    {
        if (!str_contains($fn, '::') || $p->type === null) {
            return null;
        }
        $shape = null;
        foreach ($p->type->getAtomicTypes() as $at) {
            if (!$at instanceof Atomic\TKeyedArray || $at->is_list || $shape !== null) {
                return null;
            }
            $shape = $at;
        }
        if ($shape === null) {
            return null;
        }
        [$cls] = explode('::', $fn, 2);
        $out = [];
        foreach ($shape->properties as $key => $_) {
            $slot = is_string($key) ? $this->propSlot($cls, $key) : null;
            if ($slot === null) {
                return null;
            }
            $out[$key] = $slot;
        }
        return $out;
    }

    private function funcName(Expr\FuncCall $e): ?string
    {
        if (!$e->name instanceof Name) {
            return null;
        }
        $n = $e->name;
        if (!$n->isFullyQualified() && $this->ns !== '' && !$n->isQualified()) {
            $candidate = strtolower($this->ns . '\\' . $n->toString());
            try {
                $this->codebase->functions->getStorage(null, $candidate);
                return $candidate;
            } catch (Throwable) {
            }
        }
        if ($n->isQualified() && !$n->isFullyQualified()) {
            return strtolower(ltrim($this->ns . '\\' . $n->toString(), '\\'));
        }
        return strtolower($n->toLowerString() === '' ? '' : ltrim($n->toString(), '\\'));
    }

    /** @return array<string, mixed> */
    private function funcCall(Expr\FuncCall $e): array
    {
        $name = $this->funcName($e);
        if ($name === null) {
            // a dynamic callee: a closure variable or a callable string
            $this->expr($e->name, ['str', 'paren']);
            $this->args($e->getArgs(), null);
            return ['k' => 'x'];
        }
        if ($e->isFirstClassCallable()) {
            return ['k' => 'x'];
        }
        if (in_array($name, self::LOCALS_OFF, true)) {
            $this->note(['k' => 'scopeoff', 'fn' => $this->fn, 'why' => $name]);
        }
        // a project function
        $params = $this->fnParams($name);
        $user = false;
        try {
            $user = !$this->codebase->functions->getStorage(null, $name)->stubbed
                && ($this->codebase->functions->getStorage(null, $name)->location?->file_path ?? null) !== null
                && ConvertPlugin::inProject((string) $this->codebase->functions->getStorage(null, $name)->location?->file_path);
        } catch (Throwable) {
        }
        if ($params !== null && $user) {
            $this->args($e->getArgs(), $name);
            return ['k' => 's', 's' => 'R:' . $name];
        }
        return $this->builtin($name, $e);
    }

    /**
     * Builtins that move array elements: their results are synthetic slots (E) tied to their arguments'; the other
     * builtins take strings.
     *
     * @return array<string, mixed>
     */
    private function builtin(string $name, Expr\FuncCall $e): array
    {
        $args = $e->getArgs();
        $unpacked = false;
        foreach ($args as $a) {
            $unpacked = $unpacked || $a->unpack;
        }
        $E = 'E:' . $this->rel . ':' . $e->getStartFilePos();
        $arr = function (int $i) use ($args): ?string {
            return isset($args[$i]) ? $this->arrSlot($args[$i]->value) : null;
        };
        $tie = static function (?string $a, string $b, string $why): void {
            if ($a !== null) {
                ConvertPlugin::out(['k' => 'tie', 'a' => $a, 'b' => $b, 'why' => $why, 'paths' => true]);
            }
        };
        $Et = $this->type($e);
        $synthetic = function (string $why) use ($E, $Et): array {
            ConvertPlugin::out(['k' => 'slot', 's' => $E, 'file' => $this->file, 'paths' => Types::paths($Et),
                'synthetic' => $why, 'decl' => []]);
            return ['k' => 's', 's' => $E];
        };
        if ($unpacked && !in_array($name, ['array_merge', 'array_replace', 'max', 'min', 'array_push'], true)) {
            foreach ($args as $a) {
                $this->expr($a->value, ['str']);
                $s = $a->unpack ? $this->src($a->value) : null;
                if ($s !== null && $s['k'] === 's') {
                    ConvertPlugin::out(['k' => 'stop', 's' => $s['s'], 'why' => 'unpacked-arg', 'sub' => true]);
                }
            }
            return ['k' => 'x'];
        }
        switch ($name) {
            case 'array_keys':
                $a = $arr(0);
                if ($a !== null) {
                    $tie($a . '#k', $E . '#v', $name);
                }
                if (isset($args[1])) {
                    $this->expr($args[1]->value, $a !== null ? ['s', $a . '#v'] : ['str']);
                }
                return $synthetic($name);
            case 'array_values':
                $a = $arr(0);
                $tie($a !== null ? $a . '#v' : null, $E . '#v', $name);
                return $synthetic($name);
            case 'array_flip':
                $a = $arr(0);
                if ($a !== null) {
                    $tie($a . '#v', $E . '#k', $name);
                    $tie($a . '#k', $E . '#v', $name);
                }
                return $synthetic($name);
            case 'array_count_values':
                $a = $arr(0);
                $tie($a !== null ? $a . '#v' : null, $E . '#k', $name);
                return $synthetic($name);
            case 'array_unique':
            case 'array_reverse':
            case 'array_slice':
            case 'array_filter':
            case 'array_diff':
            case 'array_diff_key':
            case 'array_diff_assoc':
            case 'array_intersect':
            case 'array_intersect_key':
            case 'array_intersect_assoc':
            case 'array_merge':
            case 'array_replace':
            case 'array_merge_recursive':
            case 'array_replace_recursive':
            case 'array_udiff':
            case 'array_uintersect':
            case 'array_diff_ukey':
                return $this->arraySetOp($name, $e, $args, $E, $arr, $tie, $synthetic);
            case 'array_map':
                return $this->arrayMap($e, $args, $E, $arr, $tie, $synthetic);
            case 'array_combine':
                $k = $arr(0);
                $v = $arr(1);
                $tie($k !== null ? $k . '#v' : null, $E . '#k', $name);
                $tie($v !== null ? $v . '#v' : null, $E . '#v', $name);
                return $synthetic($name);
            case 'array_fill_keys':
                $k = $arr(0);
                $tie($k !== null ? $k . '#v' : null, $E . '#k', $name);
                if (isset($args[1])) {
                    $this->expr($args[1]->value, ['s', $E . '#v']);
                }
                return $synthetic($name);
            case 'array_fill':
                foreach ([0, 1] as $i) {
                    if (isset($args[$i])) {
                        $this->expr($args[$i]->value, ['any']);
                    }
                }
                if (isset($args[2])) {
                    $this->expr($args[2]->value, ['s', $E . '#v']);
                }
                return $synthetic($name);
            case 'array_pad':
                $a = $arr(0);
                $tie($a, $E, $name);
                if (isset($args[1])) {
                    $this->expr($args[1]->value, ['any']);
                }
                if (isset($args[2])) {
                    $this->expr($args[2]->value, ['s', $E . '#v']);
                }
                return $synthetic($name);
            case 'array_key_exists':
            case 'key_exists':
                $a = $arr(1);
                if (isset($args[0])) {
                    $this->expr($args[0]->value, $a !== null ? ['s', $a . '#k'] : ['str']);
                }
                return ['k' => 'x'];
            case 'in_array':
            case 'array_search':
            case 'array_keys_search':
                return $this->inArray($name, $args, $arr);
            case 'array_key_first':
            case 'array_key_last':
            case 'key':
            case 'array_rand':
                $a = $arr(0);
                foreach (array_slice($args, 1) as $x) {
                    $this->expr($x->value, ['any']);
                }
                return $a !== null ? ['k' => 's', 's' => $a . '#k'] : ['k' => 'x'];
            case 'current':
            case 'reset':
            case 'end':
            case 'array_pop':
            case 'array_shift':
            case 'next':
            case 'prev':
                $a = $arr(0);
                return $a !== null ? ['k' => 's', 's' => $a . '#v'] : ['k' => 'x'];
            case 'array_push':
            case 'array_unshift':
                $a = $arr(0);
                foreach (array_slice($args, 1) as $x) {
                    if ($x->unpack) {
                        $s = $this->src($x->value);
                        if ($a !== null && $s['k'] === 's') {
                            $tie($s['s'], $a, $name);
                        } elseif ($a !== null) {
                            ConvertPlugin::out(['k' => 'stop', 's' => $a . '#v', 'why' => $name . '-unpack', 'sub' => true]);
                        }
                        continue;
                    }
                    $this->expr($x->value, $a !== null ? ['s', $a . '#v'] : ['str']);
                }
                if ($name === 'array_unshift' && $a !== null) {
                    ConvertPlugin::out(['k' => 'stop', 's' => $a . '#k', 'why' => 'renumbers', 'sub' => true]);
                }
                return ['k' => 'x'];
            case 'count':
            case 'sizeof':
            case 'array_is_list':
            case 'is_array':
            case 'is_string':
            case 'is_int':
            case 'is_null':
            case 'is_object':
            case 'is_scalar':
            case 'is_numeric':
            case 'is_bool':
            case 'is_float':
            case 'is_iterable':
            case 'is_countable':
            case 'is_callable':
            case 'spl_object_id':
            case 'spl_object_hash':
            case 'get_class':
            case 'get_debug_type':
            case 'gettype':
            case 'iterator_count':
                foreach ($args as $x) {
                    $this->expr($x->value, $name === 'is_numeric' || $name === 'is_callable' ? ['str'] : ['any']);
                }
                return ['k' => 'x'];
            case 'implode':
            case 'join':
                foreach ($args as $x) {
                    $t = $this->type($x->value);
                    if ($t !== null && !$t->isString() && Types::paths($t) !== []) {
                        $s = $this->src($x->value);
                        if ($s['k'] === 's') {
                            $this->note(['k' => 'arrout', 's' => $s['s'], 'r' => $this->range($x->value), 'why' => $name]);
                        }
                    } else {
                        $this->expr($x->value, ['str']);
                    }
                }
                return ['k' => 'x'];
            case 'sort':
            case 'rsort':
            case 'asort':
            case 'arsort':
            case 'natsort':
            case 'natcasesort':
            case 'array_multisort':
                $a = $arr(0);
                if ($a !== null) {
                    ConvertPlugin::out(['k' => 'stop', 's' => $a . '#v', 'why' => $name, 'sub' => true]);
                }
                return ['k' => 'x'];
            case 'ksort':
            case 'krsort':
                $a = $arr(0);
                if ($a !== null) {
                    ConvertPlugin::out(['k' => 'stop', 's' => $a . '#k', 'why' => $name, 'sub' => true]);
                }
                return ['k' => 'x'];
            case 'usort':
            case 'uasort':
            case 'uksort':
                $a = $arr(0);
                $cb = $args[1]->value ?? null;
                $step = $name === 'uksort' ? '#k' : '#v';
                if ($cb !== null && $this->callbackTie($cb, [$a !== null ? $a . $step : null, $a !== null ? $a . $step : null], null)) {
                    return ['k' => 'x'];
                }
                if ($a !== null) {
                    ConvertPlugin::out(['k' => 'stop', 's' => $a . $step, 'why' => $name, 'sub' => true]);
                }
                if ($cb !== null) {
                    $this->expr($cb, ['any']);
                }
                return ['k' => 'x'];
            case 'array_any':
            case 'array_all':
            case 'array_find':
            case 'array_find_key':
                $a = $arr(0);
                $cb = $args[1]->value ?? null;
                if ($cb !== null && !$this->callbackTie($cb, [$a !== null ? $a . '#v' : null, $a !== null ? $a . '#k' : null], null)) {
                    $this->expr($cb, ['any']);
                    if ($a !== null) {
                        $this->note(['k' => 'arrout', 's' => $a, 'r' => $this->range($args[0]->value), 'why' => $name]);
                    }
                }
                if ($a !== null && $name === 'array_find') {
                    return ['k' => 's', 's' => $a . '#v'];
                }
                if ($a !== null && $name === 'array_find_key') {
                    return ['k' => 's', 's' => $a . '#k'];
                }
                return ['k' => 'x'];
            case 'array_walk':
                $a = $arr(0);
                $cb = $args[1]->value ?? null;
                if ($cb !== null && !$this->callbackTie($cb, [$a !== null ? $a . '#v' : null, $a !== null ? $a . '#k' : null], null)) {
                    $this->expr($cb, ['any']);
                    if ($a !== null) {
                        ConvertPlugin::out(['k' => 'stop', 's' => $a, 'why' => $name, 'sub' => true]);
                    }
                }
                return ['k' => 'x'];
            case 'array_reduce':
                $a = $arr(0);
                $cb = $args[1]->value ?? null;
                if (isset($args[2])) {
                    $this->expr($args[2]->value, ['s', $E]);
                }
                if ($cb !== null && !$this->callbackTie($cb, [$E, $a !== null ? $a . '#v' : null], $E)) {
                    $this->expr($cb, ['any']);
                    ConvertPlugin::out(['k' => 'stop', 's' => $E, 'why' => $name, 'sub' => true]);
                }
                return $synthetic($name);
            case 'iterator_to_array':
            case 'array_column':
            case 'array_chunk':
            case 'array_splice':
            case 'array_walk_recursive':
                foreach ($args as $x) {
                    $s = $this->src($x->value);
                    if ($s['k'] === 's') {
                        ConvertPlugin::out(['k' => 'stop', 's' => $s['s'], 'why' => $name, 'sub' => true]);
                    }
                }
                return ['k' => 'x'];
        }
        if (in_array($name, ['strtolower', 'mb_strtolower'], true) && count($args) === 1) {
            // lowering a name: the same name when it is lowercase already (an id then needs no string at all)
            $arg = $args[0]->value;
            $s = $this->src($arg);
            $lower = ['lc' => Types::lc($this->type($arg)), 'call' => $this->range($e), 'arg' => $this->range($arg)];
            if ($s['k'] === 's') {
                return $s + ['lower' => $lower];
            }
            $this->use($s, $arg, ['str']);
            return ['k' => 'x', 'lower' => $lower];
        }
        // any other builtin: string arguments (a by-reference argument is written by it)
        $refs = [];
        try {
            foreach ((new ReflectionFunction($name))->getParameters() as $rp) {
                $refs[$rp->getPosition()] = $rp->isPassedByReference();
                if ($rp->isVariadic()) {
                    for ($k = $rp->getPosition() + 1; $k < 20; $k++) {
                        $refs[$k] = $rp->isPassedByReference();
                    }
                }
            }
        } catch (Throwable) {
        }
        foreach ($args as $i => $x) {
            if ($refs[$i] ?? false) {
                $t = $this->targetSlot($x->value);
                if ($t !== null) {
                    ConvertPlugin::out(['k' => 'stop', 's' => $t, 'why' => 'by-ref:' . $name, 'sub' => true]);
                }
                continue;
            }
            $t = $this->type($x->value);
            if ($t !== null && !$t->isString() && !$t->isNullable() && Types::paths($t) !== [] && !Types::stringish($t)) {
                // an array handed to a builtin: its names must be strings there
                $s = $this->src($x->value);
                if ($s['k'] === 's') {
                    $this->note(['k' => 'arrout', 's' => $s['s'], 'r' => $this->range($x->value), 'why' => $name]);
                }
                continue;
            }
            $this->expr($x->value, ['str']);
        }
        return ['k' => 'x'];
    }

    /**
     * @param list<Arg> $args
     * @return array<string, mixed>
     */
    private function arraySetOp(string $name, Expr\FuncCall $e, array $args, string $E, callable $arr, callable $tie, callable $synthetic): array
    {
        $keyed = in_array($name, ['array_diff_key', 'array_intersect_key', 'array_diff_ukey'], true);
        $valued = in_array($name, ['array_diff', 'array_intersect', 'array_udiff', 'array_uintersect'], true);
        foreach ($args as $i => $x) {
            if ($name === 'array_slice' && $i > 0) {
                $this->expr($x->value, ['any']);
                continue;
            }
            if ($name === 'array_filter' && $i === 1) {
                // the callback sees the values (or keys / both, by mode)
                $mode = isset($args[2]) ? $this->text($args[2]->value) : '';
                $p0 = str_contains($mode, 'USE_KEY') ? $E . '#k' : $E . '#v';
                $p1 = str_contains($mode, 'USE_BOTH') ? $E . '#k' : null;
                if (!$this->callbackTie($x->value, [$p0, $p1], null)) {
                    $this->expr($x->value, ['any']);
                }
                continue;
            }
            if (($name === 'array_filter' && $i === 2) || ($name === 'array_unique' && $i === 1)
                || ($name === 'array_reverse' && $i === 1)
            ) {
                $this->expr($x->value, ['any']);
                continue;
            }
            if (in_array($name, ['array_udiff', 'array_uintersect', 'array_diff_ukey'], true) && $i === count($args) - 1) {
                if (!$this->callbackTie($x->value, [$E . '#v', $E . '#v'], null)) {
                    $this->expr($x->value, ['any']);
                    ConvertPlugin::out(['k' => 'stop', 's' => $E, 'why' => $name, 'sub' => true]);
                }
                continue;
            }
            $a = $arr($i);
            if ($a === null) {
                continue;
            }
            if ($i === 0 || !($keyed || $valued)) {
                $tie($a, $E, $name);
            } elseif ($keyed) {
                $tie($a . '#k', $E . '#k', $name);
            } else {
                $tie($a . '#v', $E . '#v', $name);
            }
        }
        if ($name === 'array_merge' || $name === 'array_merge_recursive') {
            // string keys merge by name; ids as keys would be renumbered: array_replace keeps them
            $this->note(['k' => 'merge', 's' => $E, 'r' => $this->range($e->name)]);
        }
        return $synthetic($name);
    }

    /**
     * @param list<Arg> $args
     * @return array<string, mixed>
     */
    private function arrayMap(Expr\FuncCall $e, array $args, string $E, callable $arr, callable $tie, callable $synthetic): array
    {
        if (count($args) !== 2) {
            foreach ($args as $x) {
                $s = $this->src($x->value);
                if ($s['k'] === 's') {
                    ConvertPlugin::out(['k' => 'stop', 's' => $s['s'], 'why' => 'array_map-n', 'sub' => true]);
                }
            }
            return ['k' => 'x'];
        }
        $a = $arr(1);
        if ($a !== null) {
            $tie($a . '#k', $E . '#k', 'array_map');
        }
        if (!$this->callbackTie($args[0]->value, [$a !== null ? $a . '#v' : null], $E . '#v')) {
            // a named callable: the values go out as strings and come back as strings
            $this->expr($args[0]->value, ['any']);
            if ($a !== null) {
                $this->note(['k' => 'arrout', 's' => $a, 'r' => $this->range($args[1]->value), 'why' => 'array_map']);
            }
            ConvertPlugin::out(['k' => 'stop', 's' => $E . '#v', 'why' => 'array_map-callable', 'sub' => true]);
        }
        return $synthetic('array_map');
    }

    /**
     * Ties an inline closure's parameters (and return) to the given slots. False when the callback is not an
     * inline closure.
     *
     * @param list<?string> $params
     */
    private function callbackTie(Expr $cb, array $params, ?string $ret): bool
    {
        if (!$cb instanceof Expr\Closure && !$cb instanceof Expr\ArrowFunction) {
            return false;
        }
        $this->expr($cb, ['any']);
        $id = self::closureId($this->rel, $cb);
        ConvertPlugin::out(['k' => 'known', 'fn' => $id]);
        foreach ($cb->params as $i => $p) {
            if (!$p->var instanceof Expr\Variable || !is_string($p->var->name)) {
                continue;
            }
            $slot = 'P:' . $id . '|' . $p->var->name;
            if (($params[$i] ?? null) !== null) {
                ConvertPlugin::out(['k' => 'tie', 'a' => $slot, 'b' => $params[$i], 'why' => 'callback', 'paths' => true]);
            } else {
                ConvertPlugin::out(['k' => 'stop', 's' => $slot, 'why' => 'callback-param', 'sub' => true]);
            }
        }
        if ($ret !== null) {
            ConvertPlugin::out(['k' => 'tie', 'a' => 'R:' . $id, 'b' => $ret, 'why' => 'callback-return', 'paths' => true]);
        }
        return true;
    }

    /**
     * @param list<Arg> $args
     * @return array<string, mixed>
     */
    private function inArray(string $name, array $args, callable $arr): array
    {
        if (!isset($args[1])) {
            return ['k' => 'x'];
        }
        $h = $args[1]->value;
        if ($h instanceof Expr\Array_) {
            // against a literal list: a comparison of the needle with each element
            $id = $this->beginCmp();
            $this->expr($args[0]->value, ['cmp', $id, 0]);
            foreach ($h->items as $it) {
                if ($it !== null) {
                    $this->expr($it->value, ['cmp', $id, 1]);
                }
            }
            $this->endCmp($id, $h);
            foreach (array_slice($args, 2) as $x) {
                $this->expr($x->value, ['any']);
            }
            return ['k' => 'x'];
        }
        $a = $arr(1);
        $this->expr($args[0]->value, $a !== null ? ['s', $a . '#v'] : ['str']);
        foreach (array_slice($args, 2) as $x) {
            $this->expr($x->value, ['any']);
        }
        return $name === 'array_search' && $a !== null ? ['k' => 's', 's' => $a . '#k'] : ['k' => 'x'];
    }

    /** @param list<array{Expr\Variable, bool}> $uses */
    private function closureUses(Expr\Closure $e, array $uses): void
    {
        $id = self::closureId($this->rel, $e);
        foreach ($uses as [$v, $byref]) {
            if (!is_string($v->name)) {
                continue;
            }
            $this->var_types[$v->name][] = $this->type($v);
            ConvertPlugin::out(['k' => 'tie', 'a' => 'L:' . $id . '|' . $v->name, 'b' => $this->varSlot($v->name),
                'why' => 'use', 'paths' => true]);
        }
    }

    /** An arrow function sees the enclosing scope's variables: its free variables are tied to them. */
    private function arrowCaptures(Expr\ArrowFunction $e): void
    {
        $id = self::closureId($this->rel, $e);
        $own = [];
        foreach ($e->params as $p) {
            if ($p->var instanceof Expr\Variable && is_string($p->var->name)) {
                $own[$p->var->name] = true;
            }
        }
        $finder = new \PhpParser\NodeFinder();
        $seen = [];
        foreach ($finder->findInstanceOf([$e->expr], Expr\Variable::class) as $v) {
            if (!is_string($v->name) || $v->name === 'this' || isset($own[$v->name]) || isset($seen[$v->name])) {
                continue;
            }
            $seen[$v->name] = true;
            ConvertPlugin::out(['k' => 'tie', 'a' => 'L:' . $id . '|' . $v->name, 'b' => $this->varSlot($v->name),
                'why' => 'arrow-capture', 'paths' => true]);
        }
    }

    // ---------------------------------------------------------------------------------------------- helpers

    private function isArray(Expr $e): bool
    {
        $t = $this->type($e);
        if ($t === null) {
            return false;
        }
        foreach ($t->getAtomicTypes() as $a) {
            if (!$a instanceof Atomic\TArray && !$a instanceof Atomic\TKeyedArray && !$a instanceof TNull) {
                return false;
            }
        }
        return true;
    }

    private function isNull(Expr $e): bool
    {
        return $e instanceof Expr\ConstFetch && strtolower($e->name->toString()) === 'null';
    }

    private function isBool(Expr $e): bool
    {
        return $e instanceof Expr\ConstFetch && in_array(strtolower($e->name->toString()), ['true', 'false'], true);
    }

    private function type(Expr $e): ?Union
    {
        return $this->types?->getType($e);
    }

    /** @param array<string, mixed> $row */
    private function note(array $row): void
    {
        ConvertPlugin::out($row + ['file' => $this->file, 'fnscope' => $this->fn]);
    }

    /** @return array{int, int} */
    private function range(Node $n): array
    {
        return [$n->getStartFilePos(), $n->getEndFilePos() + 1];
    }

    private function text(Node $n): string
    {
        return substr($this->src, $n->getStartFilePos(), $n->getEndFilePos() + 1 - $n->getStartFilePos());
    }
}
