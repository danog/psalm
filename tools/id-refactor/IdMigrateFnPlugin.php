<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use PhpParser\Node;
use PhpParser\Node\Arg;
use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\NullableType;
use PhpParser\Node\Scalar\String_;
use PhpParser\Node\Stmt;
use Psalm\Codebase;
use Psalm\Internal\Interner;
use Psalm\Internal\MethodIdentifier;
use Psalm\Internal\Sym;
use Psalm\NodeTypeProvider;
use Psalm\Plugin\EventHandler\AfterClassLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\AfterFunctionLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\AfterFunctionLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use Psalm\Storage\ClassLikeStorage;
use Psalm\Storage\MethodStorage;
use Psalm\Type\Atomic\TNamedObject;
use ReflectionClass;
use SimpleXMLElement;
use SplObjectStorage;
use Throwable;

/** The method-body half of IdMigratePlugin (the two hook interfaces share a method name). */
require_once __DIR__ . '/SymNames.php';

final class IdMigrateFnPlugin implements PluginEntryPointInterface, AfterFunctionLikeAnalysisInterface
{
    /** @var ?array<int, string> */
    private static ?array $sym = null;

    private Codebase $codebase;
    private NodeTypeProvider $types;
    private string $file;
    private string $src;
    private string $method;
    private ?string $self;
    /** @var array<string, true> string parameters of this method eligible as slots */
    private array $params = [];
    /** @var array<string, string> declared class of each object-typed parameter */
    private array $param_classes = [];
    /** @var array<string, true> locals of this method that hold strings */
    private array $locals = [];
    /** @var SplObjectStorage<Node, Node> */
    private SplObjectStorage $parent;

    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function afterStatementAnalysis(AfterFunctionLikeAnalysisEvent $event): ?bool
    {
        require_once __DIR__ . '/IdMigratePlugin.php';
        $stmt = $event->getStmt();
        $file = $event->getStatementsSource()->getFilePath();
        $storage = $event->getFunctionlikeStorage();
        if (!IdMigratePlugin::inSrc($file) || !$stmt instanceof Stmt\ClassMethod || !$storage instanceof MethodStorage
            || $storage->defining_fqcln === null
        ) {
            return null;
        }
        try {
            $h = new self();
            $h->codebase = $event->getCodebase();
            $h->types = $event->getNodeTypeProvider();
            $h->file = $file;
            $h->src = (string) file_get_contents($file);
            $h->method = strtolower($storage->defining_fqcln . '::' . $stmt->name->name);
            $h->self = $storage->defining_fqcln;
            $h->run($stmt, $storage);
        } catch (Throwable $e) {
            IdMigratePlugin::out(['kind' => 'error', 'msg' => 'fn: ' . $e->getMessage() . ' @' . $file . ':' . $e->getLine()]);
        }
        return null;
    }

    private function run(Stmt\ClassMethod $stmt, MethodStorage $storage): void
    {
        $cls = $this->codebase->classlike_storage_provider->find(Interner::intern($storage->defining_fqcln));
        $lc = strtolower($stmt->name->name);
        // may the signature change? not when an ancestor or interface declares the method (pzoom has no plugin API,
        // so public methods of final/leaf classes may change)
        $fixed_why = null;
        if ($cls === null) {
            $fixed_why = 'no-storage';
        } elseif ($cls->is_interface || $cls->is_trait) {
            $fixed_why = 'interface/trait';
        } elseif ($lc !== '__construct' && str_starts_with($lc, '__')) {
            $fixed_why = 'magic';
        } else {
            foreach ([...array_keys($cls->parent_classes), ...array_keys($cls->class_implements)] as $anc) {
                $as = $this->codebase->classlike_storage_provider->find(Interner::intern((string) $anc));
                if ($as !== null && isset($as->methods[Interner::intern($lc)])) {
                    $fixed_why = 'overrides';
                    break;
                }
            }
            if ($fixed_why === null && !$stmt->isPrivate() && !$stmt->isFinal() && !$cls->final) {
                // only an actual override shares the signature (breaking the plugin API is fine, as in pzoom)
                $self_lc = strtolower($storage->defining_fqcln);
                foreach ($this->codebase->classlike_storage_provider->getAll() as $sub) {
                    if (isset($sub->parent_classes[$self_lc]) && isset($sub->methods[Interner::intern($lc)])
                        && strcasecmp(\Psalm\Internal\Interner::lookup($sub->id), $storage->defining_fqcln) !== 0
                    ) {
                        $fixed_why = 'overridden';
                        break;
                    }
                }
            }
        }
        $doc = $stmt->getDocComment();
        $param_decls = [];
        foreach ($stmt->params as $p) {
            if (!$p->var instanceof Expr\Variable || !is_string($p->var->name)) {
                continue;
            }
            $pt = $p->type instanceof NullableType ? $p->type->type : $p->type;
            if ($pt instanceof Name) {
                $this->param_classes[$p->var->name] = (string) ($pt->attrs()->resolvedName ?? $pt->toString());
            }
            $nullable_default = $p->default instanceof Expr\ConstFetch && strtolower($p->default->name->toString()) === 'null';
            if ($p->flags !== 0 && $lc === '__construct' && IdMigratePlugin::declaresIn($this->file) && IdMigratePlugin::isPlainString($p->type)
                && ($p->default === null || $nullable_default) && !$p->byRef && !$p->variadic
            ) {
                // a promoted parameter is its property: one slot (the declaration is the parameter's)
                IdMigratePlugin::out(['kind' => 'decl', 'slot' => 'F:' . strtolower($storage->defining_fqcln) . '|' . $p->var->name,
                    'file' => $this->file, 'type' => [$p->type->getStartFilePos(), $p->type->getEndFilePos() + 1],
                    'nullable' => IdMigratePlugin::isNullable($p->type),
                    // the parameter's own `@var` docblock, else the constructor's `@param`
                    'doc' => IdMigratePlugin::docType($p->getDocComment(), '@var', null)
                        ?? IdMigratePlugin::docType($doc, '@param', $p->var->name), 'fixed' => true, 'why' => null]);
                continue;
            }
            if (IdMigratePlugin::isPlainString($p->type) && ($p->default === null || ($nullable_default && IdMigratePlugin::isNullable($p->type)))
                && !$p->byRef && !$p->variadic && $p->flags === 0
            ) {
                $this->params[$p->var->name] = true;
                $param_decls[$p->var->name] = ['kind' => 'decl', 'slot' => 'P:' . $this->method . '|' . $p->var->name, 'file' => $this->file,
                    'type' => [$p->type->getStartFilePos(), $p->type->getEndFilePos() + 1], 'nullable' => IdMigratePlugin::isNullable($p->type),
                    'doc' => IdMigratePlugin::docType($doc, '@param', $p->var->name),
                    'fixed' => $fixed_why === null, 'why' => $fixed_why];
            }
        }
        if (IdMigratePlugin::declaresIn($this->file) && IdMigratePlugin::isPlainString($stmt->returnType)) {
            IdMigratePlugin::out(['kind' => 'decl', 'slot' => 'R:' . $this->method, 'file' => $this->file,
                'type' => [$stmt->returnType->getStartFilePos(), $stmt->returnType->getEndFilePos() + 1],
                'nullable' => IdMigratePlugin::isNullable($stmt->returnType),
                'doc' => IdMigratePlugin::docType($doc, '@return', null),
                'fixed' => $fixed_why === null, 'why' => $fixed_why]);
        }
        $body = $stmt->stmts ?? [];
        $this->parent = new SplObjectStorage();
        $this->index($body, $stmt);
        $this->findLocals($body);
        // parameter declarations only for the parameters the body leaves convertible
        foreach ($param_decls as $name => $row) {
            if (!IdMigratePlugin::declaresIn($this->file)) {
                continue;
            }
            if (isset($this->params[$name])) {
                IdMigratePlugin::out($row);
            } else {
                IdMigratePlugin::out(['kind' => 'block', 'slot' => $row['slot'], 'why' => 'written/by-ref/captured in body']);
            }
        }
        foreach (IdMigratePlugin::declaresIn($this->file) ? $this->locals : [] as $name => $_) {
            IdMigratePlugin::out(['kind' => 'decl', 'slot' => 'L:' . $this->method . '|' . $name, 'file' => $this->file,
                'type' => null, 'doc' => null, 'fixed' => true, 'why' => null]);
        }
        $this->walk($body, $stmt);
    }

    /** @param array<Node>|Node $nodes */
    private function index(array|Node $nodes, Node $parent): void
    {
        foreach (is_array($nodes) ? $nodes : [$nodes] as $n) {
            if (!$n instanceof Node) {
                continue;
            }
            $this->parent[$n] = $parent;
            if ($n instanceof Expr\Closure || $n instanceof Expr\ArrowFunction || $n instanceof Stmt\Class_) {
                continue; // other scopes: their variables are not this method's
            }
            foreach ($n->getSubNodeNames() as $sub) {
                $v = $n->$sub;
                if ($v instanceof Node || is_array($v)) {
                    $this->index(is_array($v) ? array_filter($v, static fn($x) => $x instanceof Node) : $v, $n);
                }
            }
        }
    }

    /** Locals assigned only by plain `$x = E` with string-typed E, and never written otherwise. @param array<Node> $body */
    private function findLocals(array $body): void
    {
        $assigned = [];
        $bad = [];
        foreach ($this->parent as $n) {
            if ($n instanceof Expr\Assign && $n->var instanceof Expr\Variable && is_string($n->var->name)) {
                $t = $this->types->getType($n->expr);
                if ($t !== null && $t->isString() && !$t->isNullable()) {
                    $assigned[$n->var->name] = true;
                } else {
                    $bad[$n->var->name] = true;
                }
            } elseif (($n instanceof Expr\AssignOp || $n instanceof Expr\AssignRef) && $n->var instanceof Expr\Variable && is_string($n->var->name)) {
                $bad[$n->var->name] = true;
            } elseif ($n instanceof Stmt\Foreach_) {
                foreach ([$n->keyVar, $n->valueVar] as $v) {
                    if ($v instanceof Expr\Variable && is_string($v->name)) {
                        $bad[$v->name] = true;
                    }
                }
            } elseif ($n instanceof Expr\List_ || ($n instanceof Expr\Array_ && $this->parent[$n] instanceof Expr\Assign)) {
                foreach ($n->items as $it) {
                    if ($it !== null && $it->value instanceof Expr\Variable && is_string($it->value->name)) {
                        $bad[$it->value->name] = true;
                    }
                }
            } elseif ($n instanceof Stmt\Global_ || $n instanceof Stmt\Static_ || $n instanceof Stmt\Unset_) {
                foreach ($n->vars as $v) {
                    $v = $v instanceof Node\StaticVar ? $v->var : $v;
                    if ($v instanceof Expr\Variable && is_string($v->name)) {
                        $bad[$v->name] = true;
                    }
                }
            } elseif ($n instanceof Stmt\Catch_ && $n->var !== null && is_string($n->var->name)) {
                $bad[$n->var->name] = true;
            } elseif ($n instanceof Expr\Closure || $n instanceof Expr\ArrowFunction) {
                // a nested scope sees the variable (use / arrow capture): its reads are not rewritten here
                $finder = new \PhpParser\NodeFinder();
                foreach ($finder->find($n instanceof Expr\Closure ? [...$n->uses, ...$n->stmts] : [$n->expr],
                    static fn(Node $x): bool => $x instanceof Expr\Variable && is_string($x->name)) as $v) {
                    $bad[$v->name] = true;
                }
                foreach ($n instanceof Expr\Closure ? $n->uses : [] as $u) {
                    if (is_string($u->var->name)) {
                        $bad[$u->var->name] = true;
                    }
                }
            } elseif ($n instanceof Expr\Variable && !is_string($n->name)) {
                $this->params = [];
                return; // variable variables
            } elseif ($n instanceof Expr\FuncCall && $n->name instanceof Name
                && in_array(strtolower($n->name->toString()), ['compact', 'extract', 'get_defined_vars', 'func_get_args'], true)
            ) {
                $this->params = [];
                return;
            } elseif ($n instanceof Arg && $n->value instanceof Expr\Variable && is_string($n->value->name)
                && $this->argByRef($n)
            ) {
                $bad[$n->value->name] = true;
            }
        }
        foreach ($assigned as $name => $_) {
            if (!isset($bad[$name]) && !isset($this->params[$name]) && $name !== 'this') {
                $this->locals[$name] = true;
            }
        }
        foreach ($bad as $name => $_) {
            unset($this->params[$name]);
        }
    }

    private function argByRef(Arg $arg): bool
    {
        $call = $this->parent[$arg] ?? null;
        if (!$call instanceof Expr\CallLike) {
            return false;
        }
        [$callee, $idx] = $this->calleeParam($call, $arg);
        if ($callee === null) {
            // unknown callees (builtins taking references: preg_match etc.) are assumed to write
            return !$call instanceof Expr\FuncCall || !$this->builtinNoRef($call);
        }
        try {
            if (str_starts_with($callee, 'F:')) {
                return false; // a promoted constructor parameter is never by reference here
            }
            [$mid] = explode('|', substr($callee, 2), 2);
            $ms = $this->codebase->methods->getStorage(new MethodIdentifier(...array_map(\Psalm\Internal\Interner::intern(...), explode('::', $mid, 2))));
            return ($ms->params[$idx] ?? null)?->by_ref ?? false;
        } catch (Throwable) {
            return true;
        }
    }

    private function builtinNoRef(Expr\FuncCall $call): bool
    {
        if (!$call->name instanceof Name) {
            return false;
        }
        return !in_array(strtolower($call->name->getLast()), ['preg_match', 'preg_match_all', 'preg_replace_callback',
            'str_replace', 'settype', 'parse_str', 'sscanf', 'similar_text', 'array_push', 'array_unshift', 'sort',
            'usort', 'ksort', 'end', 'reset', 'next', 'prev', 'array_splice', 'array_shift', 'array_pop'], true);
    }

    /** @return array{?string, int} [declaring method id "class::method" (lowercase), parameter index] */
    private function calleeParam(Expr\CallLike $call, Arg $arg): array
    {
        $class = null;
        $method = null;
        if ($call instanceof Expr\MethodCall || $call instanceof Expr\NullsafeMethodCall) {
            if (!$call->name instanceof Identifier) {
                return [null, 0];
            }
            $t = $this->types->getType($call->var);
            if ($t === null) {
                return [null, 0];
            }
            $named = [];
            foreach ($t->getAtomicTypes() as $a) {
                if ($a instanceof TNamedObject) {
                    $named[] = SymNames::named($a);
                } elseif (!$a instanceof \Psalm\Type\Atomic\TNull) {
                    return [null, 0];
                }
            }
            if (count($named) !== 1) {
                return [null, 0];
            }
            [$class, $method] = [$named[0], $call->name->name];
        } elseif ($call instanceof Expr\StaticCall) {
            if (!$call->class instanceof Name || !$call->name instanceof Identifier) {
                return [null, 0];
            }
            $class = (string) ($call->class->attrs()->resolvedName ?? $call->class->toString());
            if (in_array(strtolower($class), ['self', 'static'], true)) {
                $class = $this->self;
            } elseif (strtolower($class) === 'parent') {
                $class = $this->codebase->classlike_storage_provider->find(Interner::intern((string) $this->self))?->parent_class;
                if ($class === null) {
                    return [null, 0];
                }
            }
            $method = $call->name->name;
        } elseif ($call instanceof Expr\New_) {
            if (!$call->class instanceof Name) {
                return [null, 0];
            }
            $class = (string) ($call->class->attrs()->resolvedName ?? $call->class->toString());
            $method = '__construct';
        } else {
            return [null, 0];
        }
        try {
            $decl = $this->codebase->methods->getDeclaringMethodId(new MethodIdentifier(\Psalm\Internal\Interner::intern((string) $class), \Psalm\Internal\Interner::intern(strtolower($method))));
        } catch (Throwable) {
            return [null, 0];
        }
        if ($decl === null) {
            return [null, 0];
        }
        $ms = $this->codebase->methods->getStorage($decl);
        $idx = null;
        foreach ($call->getArgs() as $j => $a) {
            if ($a === $arg) {
                if ($a->name !== null) {
                    foreach ($ms->params as $k => $p) {
                        if ($p->name === $a->name->name) {
                            $idx = $k;
                        }
                    }
                } else {
                    $idx = $j;
                }
            }
        }
        if ($idx === null || $arg->unpack) {
            return [null, 0];
        }
        $pname = ($ms->params[$idx] ?? null)?->name;
        if ($pname === null) {
            return [null, 0];
        }
        if (($ms->params[$idx] ?? null)?->promoted_property) {
            return ['F:' . strtolower($ms->defining_fqcln ?? \Psalm\Internal\Interner::lookup($decl->class_id)) . '|' . $pname, $idx];
        }
        $defining = $ms->defining_fqcln ?? \Psalm\Internal\Interner::lookup($decl->class_id);
        return ['P:' . strtolower($defining . '::' . \Psalm\Internal\Interner::lookupLc($decl->name_id)) . '|' . $pname, $idx];
    }

    /** The slot a read expression reads, or null. */
    private function slotOf(Expr $e): ?string
    {
        if ($e instanceof Expr\Variable && is_string($e->name)) {
            if (isset($this->params[$e->name])) {
                return 'P:' . $this->method . '|' . $e->name;
            }
            if (isset($this->locals[$e->name])) {
                return 'L:' . $this->method . '|' . $e->name;
            }
            return null;
        }
        if (($e instanceof Expr\PropertyFetch || $e instanceof Expr\NullsafePropertyFetch) && $e->name instanceof Identifier) {
            $t = $this->types->getType($e->var);
            $cls = null;
            if ($e->var instanceof Expr\Variable && $e->var->name === 'this') {
                $cls = $this->self;
            } elseif ($t === null && $e->var instanceof Expr\Variable && is_string($e->var->name)
                && isset($this->param_classes[$e->var->name])
            ) {
                // no type recorded here: the parameter's declared class
                $cls = $this->param_classes[$e->var->name];
            } elseif ($t !== null) {
                $named = [];
                foreach ($t->getAtomicTypes() as $a) {
                    if ($a instanceof TNamedObject) {
                        $named[] = SymNames::named($a);
                    } elseif (!$a instanceof \Psalm\Type\Atomic\TNull) {
                        return null;
                    }
                }
                $cls = count($named) === 1 ? $named[0] : null;
            }
            return $cls === null ? null : $this->propSlot($cls, $e->name->name);
        }
        if ($e instanceof Expr\StaticPropertyFetch && $e->class instanceof Name && $e->name instanceof Node\VarLikeIdentifier) {
            $cls = (string) ($e->class->attrs()->resolvedName ?? $e->class->toString());
            if (in_array(strtolower($cls), ['self', 'static'], true)) {
                $cls = $this->self;
            }
            return $cls === null ? null : $this->propSlot($cls, $e->name->name);
        }
        if ($e instanceof Expr\MethodCall || $e instanceof Expr\StaticCall || $e instanceof Expr\NullsafeMethodCall) {
            $dummy = new Arg(new Expr\Variable('x'));
            // the callee's declaring id: reuse calleeParam's resolution through a synthetic lookup
            $id = $this->calleeId($e);
            return $id === null ? null : 'R:' . $id;
        }
        return null;
    }

    private function calleeId(Expr\MethodCall|Expr\StaticCall|Expr\NullsafeMethodCall $call): ?string
    {
        if (!$call->name instanceof Identifier) {
            return null;
        }
        if ($call instanceof Expr\StaticCall) {
            if (!$call->class instanceof Name) {
                return null;
            }
            $class = (string) ($call->class->attrs()->resolvedName ?? $call->class->toString());
            if (in_array(strtolower($class), ['self', 'static'], true)) {
                $class = $this->self;
            } elseif (strtolower($class) === 'parent') {
                return null;
            }
        } else {
            $t = $this->types->getType($call->var);
            $named = [];
            foreach ($t?->getAtomicTypes() ?? [] as $a) {
                if ($a instanceof TNamedObject) {
                    $named[] = SymNames::named($a);
                } elseif (!$a instanceof \Psalm\Type\Atomic\TNull) {
                    return null;
                }
            }
            if (count($named) !== 1) {
                return null;
            }
            $class = $named[0];
        }
        try {
            $decl = $this->codebase->methods->getDeclaringMethodId(new MethodIdentifier(\Psalm\Internal\Interner::intern((string) $class), \Psalm\Internal\Interner::intern(strtolower($call->name->name))));
            if ($decl === null) {
                return null;
            }
            $ms = $this->codebase->methods->getStorage($decl);
        } catch (Throwable) {
            return null;
        }
        if (!in_array($ms->signature_return_type?->getId(), ['string', 'null|string', 'string|null'], true)) {
            return null;
        }
        return strtolower(($ms->defining_fqcln ?? \Psalm\Internal\Interner::lookup($decl->class_id)) . '::' . \Psalm\Internal\Interner::lookupLc($decl->name_id));
    }

    private function propSlot(string $cls, string $prop): ?string
    {
        $storage = $this->codebase->classlike_storage_provider->find(Interner::intern($cls));
        if ($storage === null) {
            return null;
        }
        $declaring = $storage->declaring_property_ids[Interner::intern($prop)] ?? null;
        if ($declaring === null) {
            return null;
        }
        $ds = $this->codebase->classlike_storage_provider->find(Interner::intern($declaring));
        $ps = $ds?->properties[Interner::intern($prop)] ?? null;
        if ($ps === null || !in_array($ps->signature_type?->getId(), ['string', 'null|string', 'string|null'], true)) {
            return null;
        }
        return 'F:' . strtolower(\Psalm\Internal\Interner::lookup($ds->id)) . '|' . $prop;
    }

    /** @param array<Node> $nodes */
    private function walk(array $nodes, Node $fn): void
    {
        foreach ($this->parent as $n) {
            if (!$n instanceof Expr) {
                continue;
            }
            $parent = $this->parent[$n];
            // flows into slots
            if ($n instanceof Expr\Assign && $parent !== null) {
                $target = $this->slotOf($n->var);
                if ($target !== null) {
                    $this->flow($target, $n->expr);
                }
                continue;
            }
            if ($n instanceof Expr\CallLike && !$n->isFirstClassCallable()) {
                foreach ($n->getArgs() as $a) {
                    [$callee] = $this->calleeParam($n, $a);
                    if ($callee !== null && $this->isStringParam($callee)) {
                        $this->flow($callee, $a->value, $a->name !== null ? [$a->name->getStartFilePos(), $a->name->getEndFilePos() + 1] : null);
                    }
                }
            }
            // reads
            $slot = $this->slotOf($n);
            if ($slot === null) {
                continue;
            }
            if ($parent instanceof Expr\Assign && $parent->var === $n) {
                continue; // write target
            }
            if ($parent instanceof Expr\AssignOp || $parent instanceof Expr\AssignRef || $parent instanceof Expr\PreInc
                || $parent instanceof Expr\PostInc || $parent instanceof Expr\PreDec || $parent instanceof Expr\PostDec
            ) {
                IdMigratePlugin::out(['kind' => 'block', 'slot' => $slot, 'why' => $parent->getType()]);
                continue;
            }
            // null tests: an id is null exactly when the string was
            if ($parent instanceof Expr\Isset_
                || (($parent instanceof Expr\BinaryOp\Identical || $parent instanceof Expr\BinaryOp\NotIdentical)
                    && (($parent->left === $n && $this->isNull($parent->right)) || ($parent->right === $n && $this->isNull($parent->left))))
            ) {
                IdMigratePlugin::out(['kind' => 'use', 'slot' => $slot, 'ctx' => 'nullcmp', 'file' => $this->file, 'r' => $this->range($n)]);
                continue;
            }
            if ($parent instanceof Stmt\Unset_ || $parent instanceof Expr\Empty_) {
                IdMigratePlugin::out(['kind' => 'block', 'slot' => $slot, 'why' => 'unset/empty']);
                continue;
            }
            // `$x ?: Y` / `$x ?? Y`: the whole expression picks the string or the fallback
            if (($parent instanceof Expr\Ternary && $parent->if === null && $parent->cond === $n)
                || ($parent instanceof Expr\BinaryOp\Coalesce && $parent->left === $n)
            ) {
                $alt = $parent instanceof Expr\Ternary ? $parent->else : $parent->right;
                IdMigratePlugin::out(['kind' => 'use', 'slot' => $slot, 'ctx' => 'fallback', 'file' => $this->file, 'r' => $this->range($n),
                    'expr' => $this->range($parent), 'alt' => $this->range($alt),
                    'nullable' => (bool) $this->types->getType($n)?->isNullable()]);
                continue;
            }
            // flows are recorded from the target side (Assign / Arg / Return)
            if ($parent instanceof Expr\Assign && $parent->expr === $n && $this->slotOf($parent->var) !== null) {
                continue;
            }
            if ($parent instanceof Arg) {
                $call = $this->parent[$parent];
                if ($call instanceof Expr\CallLike) {
                    if ($this->isInternCall($call)) {
                        IdMigratePlugin::out(['kind' => 'use', 'slot' => $slot, 'ctx' => 'intern', 'file' => $this->file,
                            'r' => $this->range($n), 'call' => $this->range($call)]);
                        continue;
                    }
                    [$callee] = $this->calleeParam($call, $parent);
                    if ($callee !== null && $this->isStringParam($callee)) {
                        continue;
                    }
                }
            }
            if ($parent instanceof Stmt\Return_) {
                $fnret = 'R:' . $this->method;
                IdMigratePlugin::out(['kind' => 'flow', 'slot' => $fnret, 'file' => $this->file,
                    'src' => ['k' => 'slot', 'slot' => $slot, 'r' => $this->range($n)]]);
                continue;
            }
            // a truth test (if/elseif/while/ternary conditions, !, &&, ||): for a nullable name slot it tests null
            if ($parent instanceof Stmt\If_ && $parent->cond === $n || $parent instanceof Stmt\ElseIf_ && $parent->cond === $n
                || $parent instanceof Stmt\While_ && $parent->cond === $n || $parent instanceof Expr\BooleanNot
                || $parent instanceof Expr\BinaryOp\BooleanAnd || $parent instanceof Expr\BinaryOp\BooleanOr
                || ($parent instanceof Expr\Ternary && $parent->cond === $n && $parent->if !== null)
            ) {
                IdMigratePlugin::out(['kind' => 'use', 'slot' => $slot, 'ctx' => 'truthy', 'file' => $this->file, 'r' => $this->range($n)]);
                continue;
            }
            // property fetches of a slot-typed object are reads too; method calls on the value are string uses
            IdMigratePlugin::out(['kind' => 'use', 'slot' => $slot, 'ctx' => 'other', 'file' => $this->file, 'r' => $this->range($n),
                'nullable' => (bool) $this->types->getType($n)?->isNullable()]);
        }
        // returns of non-slot expressions into this method's own return slot
        foreach ($this->parent as $n) {
            if ($n instanceof Stmt\Return_ && $n->expr !== null && $this->slotOf($n->expr) === null) {
                $this->flow('R:' . $this->method, $n->expr);
            }
        }
    }

    private function isNull(Expr $e): bool
    {
        return $e instanceof Expr\ConstFetch && strtolower($e->name->toString()) === 'null';
    }

    private function isStringParam(string $callee): bool
    {
        if (str_starts_with($callee, 'F:')) {
            [$cls, $prop] = explode('|', substr($callee, 2), 2);
            return $this->propSlot($cls, $prop) !== null;
        }
        [$mid, $pname] = explode('|', substr($callee, 2), 2);
        [$c, $m] = explode('::', $mid, 2);
        try {
            $ms = $this->codebase->methods->getStorage(new MethodIdentifier(\Psalm\Internal\Interner::intern($c), \Psalm\Internal\Interner::intern($m)));
        } catch (Throwable) {
            return false;
        }
        foreach ($ms->params as $p) {
            if ($p->name === $pname) {
                return in_array($p->signature_type?->getId(), ['string', 'null|string', 'string|null'], true);
            }
        }
        return false;
    }

    private function isInternCall(Expr\CallLike $call): bool
    {
        return $call instanceof Expr\StaticCall && $call->class instanceof Name && $call->name instanceof Identifier
            && strtolower($call->name->name) === 'intern'
            && in_array(strtolower($call->class->getLast()), ['interner'], true)
            && count($call->getArgs()) === 1;
    }

    /** @param ?array{int, int} $named */
    private function flow(string $target, Expr $src, ?array $named = null): void
    {
        $slot = $this->slotOf($src);
        if ($slot !== null) {
            $c = ['k' => 'slot', 'slot' => $slot, 'r' => $this->range($src)];
        } elseif (($edit = $this->idForm($src)) !== null) {
            $c = ['k' => 'edit', 'e' => $edit[0], 'r' => $this->range($src)];
            if ($edit[1] !== null) {
                IdMigratePlugin::out(['kind' => 'sym', 'name' => $edit[1][0], 'value' => $edit[1][1]]);
            }
        } elseif ($src instanceof Expr\ConstFetch && strtolower($src->name->toString()) === 'null') {
            $c = ['k' => 'null'];
        } else {
            $c = ['k' => 'str', 'r' => $this->range($src), 'shape' => $src->getType()];
        }
        if ($named !== null) {
            $c['named'] = $named;
        }
        $c['nullable'] = (bool) $this->types->getType($src)?->isNullable();
        IdMigratePlugin::out(['kind' => 'flow', 'slot' => $target, 'file' => $this->file, 'src' => $c]);
    }

    /** @return array{int, int} */
    private function range(Node $n): array
    {
        return [$n->getStartFilePos(), $n->getEndFilePos() + 1];
    }

    /** @return ?array{array{int, int, string}, ?array{string, int}} */
    private function idForm(Expr $e): ?array
    {
        if (($e instanceof Expr\PropertyFetch || $e instanceof Expr\NullsafePropertyFetch) && $e->name instanceof Identifier) {
            $base = $this->types->getType($e->var);
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
                $named = strcasecmp(SymNames::named($a), TNamedObject::class) === 0
                    || $this->codebase->classExtendsOrImplements(Interner::intern(SymNames::named($a)), Interner::intern(TNamedObject::class));
                if ($prop === 'value' && $named) {
                    $cand = 'name';
                } elseif ($prop === 'name' && strcasecmp(SymNames::named($a), ClassLikeStorage::class) === 0) {
                    $cand = 'id';
                } elseif ($prop === 'fq_class_name' && strcasecmp(SymNames::named($a), MethodIdentifier::class) === 0) {
                    $cand = 'class_id';
                }
                if ($cand === null || ($to !== null && $to !== $cand)) {
                    return null;
                }
                $to = $cand;
            }
            return $to === null ? null : [[$e->name->getStartFilePos(), $e->name->getEndFilePos() + 1, $to], null];
        }
        // Interner::lookup($id): its id is the argument
        if ($e instanceof Expr\StaticCall && $e->class instanceof Name && $e->name instanceof Identifier
            && strtolower($e->name->name) === 'lookup' && count($e->getArgs()) === 1
            && in_array(strtolower($e->class->getLast()), ['interner'], true)
        ) {
            $inner = $e->getArgs()[0]->value;
            $text = substr($this->src, $inner->getStartFilePos(), $inner->getEndFilePos() + 1 - $inner->getStartFilePos());
            return [[$e->getStartFilePos(), $e->getEndFilePos() + 1, $text], null];
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
        if ($literal === null) {
            return null;
        }
        $value = Interner::hash($literal);
        if (self::$sym === null) {
            self::$sym = [];
            foreach ((new ReflectionClass(Sym::class))->getConstants() as $cn => $v) {
                if (is_int($v)) {
                    self::$sym[$v] = $cn;
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
        if ($name === 'C_') {
            $name = 'C_EMPTY';
        }
        return [[...$range, 'Sym::' . $name], [$name, $value]];
    }
}
