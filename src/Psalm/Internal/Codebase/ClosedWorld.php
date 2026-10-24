<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use PhpParser;
use Psalm\Codebase;
use Psalm\Internal\Analyzer\FunctionLike\ReturnTypeCollector;
use Psalm\Internal\Analyzer\ScopeAnalyzer;
use Psalm\Internal\Analyzer\StatementsAnalyzer;
use Psalm\Internal\Type\Comparator\UnionTypeComparator;
use Psalm\Internal\Type\TypeCombiner;
use Psalm\Storage\ClassLikeStorage;
use Psalm\Storage\FunctionLikeParameter;
use Psalm\Storage\FunctionLikeStorage;
use Psalm\Storage\MethodStorage;
use Psalm\Storage\PropertyStorage;
use Psalm\Type;
use Psalm\Type\Atomic;
use Psalm\Type\Atomic\TArray;
use Psalm\Type\Atomic\TFloat;
use Psalm\Type\Atomic\TGenericObject;
use Psalm\Type\Atomic\TInt;
use Psalm\Type\Atomic\TIntRange;
use Psalm\Type\Atomic\TIterable;
use Psalm\Type\Atomic\TKeyedArray;
use Psalm\Type\Atomic\TLiteralFloat;
use Psalm\Type\Atomic\TLiteralInt;
use Psalm\Type\Atomic\TLiteralString;
use Psalm\Type\Atomic\TNonEmptyString;
use Psalm\Type\Atomic\TNumericString;
use Psalm\Type\Atomic\TString;
use Psalm\Type\Union;

use function array_keys;
use function count;
use function fwrite;
use function getenv;
use function is_numeric;
use function ksort;
use function spl_object_id;
use function strtolower;

use const STDERR;

/**
 * Closed-world whole-program type inference.
 *
 * When every file that will ever run is part of the analyzed program, the types of a project function's
 * parameters are exactly the types its callers pass, its return type is what its body returns and a property's
 * type is the union of everything the program writes to it. This class collects those facts while Psalm analyzes
 * the program ("contributions"), writes them into the storages ("slots": parameter, return and property types)
 * and drives re-analysis until the slots reach a fixpoint: an analysis whose contributions produce exactly the
 * slot values it was run with is self-consistent, so every type it assumed is sound.
 *
 * The iteration starts from a pessimistic pass (unknown parameters are `mixed`), takes an optimistic first guess
 * (the contributions of that pass minus the `mixed` ones, and minus those of direct recursion) and then replaces
 * every slot by its contributions until nothing changes. Slots that keep changing are widened (literals
 * generalized, then the union of successive values, finally `mixed`) so the iteration terminates.
 *
 * Project docblock types are not used for project code in this mode (see Config::ignoresDocblockTypesIn): their
 * signatures are native types refined by inference.
 *
 * @internal
 */
final class ClosedWorld
{
    private static ?self $instance = null;

    /** Sound-phase changes after which a slot is joined with its previous value instead of replaced. */
    private const JOIN_AFTER = 6;

    /** Sound-phase changes after which a still-changing slot becomes `mixed`. */
    private const TOP_AFTER = 12;

    /** Passes after which the optimistic phase ends even if it has not settled. */
    private const OPTIMISTIC_PASSES = 12;

    /**
     * `optimistic`: `mixed` contributions are ignored (they are assumed to come from slots not inferred yet);
     * `sound`: every contribution counts. The optimistic fixpoint is the starting point of the sound iteration,
     * whose fixpoint is the result.
     */
    public string $phase = 'optimistic';

    public int $pass = 0;

    /** @var array<int, FunctionLikeStorage> every project function-like seen, by object id */
    private array $storages = [];

    /**
     * Argument contributions of the current pass: storage id => param offset => slot.
     *
     * @var array<int, array<int, ClosedWorldSlot>>
     */
    private array $args = [];

    /**
     * Arguments of method calls on receivers of unknown type: lowercase method name => param offset => slot.
     *
     * @var array<string, array<int, ClosedWorldSlot>>
     */
    private array $unknown_method_args = [];

    /** @var array<int, array<int, ClosedWorldSlot>> what bodies leave in by-reference parameters: storage id => offset */
    private array $outs = [];

    /** Whether some callable value reaching a dynamic call or a callable parameter is not a known literal. */
    private bool $unknown_callables = false;

    /**
     * Writes of global and static variables: slot key (`g:name`, `s:<storage id>:name`) => slot.
     *
     * @var array<string, ClosedWorldSlot>
     */
    private array $global_writes = [];

    /** @var array<string, Union> current types of global and static variables, by slot key */
    private array $global_types = [];

    /** @var array<string, true> globals written through `$GLOBALS[...]` (`*`: an unknown one): not inferred */
    private array $escaped_globals = [];

    /** @var array<int, ClosedWorldSlot> body return contributions: storage id => slot */
    private array $returns = [];

    /** @var array<int, true> storages whose body always exits (returns never) */
    private array $always_exits = [];

    /** @var array<int, true> storages whose body was analyzed in this pass */
    private array $analyzed = [];

    /** @var array<string, ClosedWorldSlot> property writes: "declaring class lc::name" => slot */
    private array $props = [];

    /** @var array<string, ClosedWorldSlot> writes to properties of receivers of unknown type: name => slot */
    private array $unknown_prop_writes = [];

    /** @var array<string, array<string, ClosedWorldSlot>> writes of undeclared (dynamic) properties: class lc => name => slot */
    private array $dynamic_props = [];

    /** @var array<string, ClosedWorldSlot> `$o->$name = v` with an unknown name: class lc => slot (widens every property) */
    private array $any_prop_writes = [];

    /** @var array<string, true> lowercase string literals of project code (possible callable names) */
    private array $strings = [];

    /**
     * Current slot values (what the storages hold), by slot key, as type ids: the fixpoint test.
     *
     * @var array<string, string>
     */
    private array $values = [];

    /** @var array<string, int> how many passes changed each slot */
    private array $changes = [];

    /** @var array<string, Union> previous slot values (for joining when a slot does not settle) */
    private array $previous = [];

    /** @var array<string, array{FunctionLikeParameter, ?Union}> original (declared) parameter types */
    private array $declared_params = [];

    /** @var array<int, array{?Union}> original (declared) return types by storage id */
    private array $declared_returns = [];

    /** @var array<string, array{PropertyStorage, ?Union}> original property types by "class lc::name" */
    private array $declared_props = [];

    /** @var array<string, array<string, PropertyStorage>> dynamic properties added to classes: class lc => name */
    private array $added_props = [];

    private function __construct(
        private readonly Codebase $codebase,
    ) {
    }

    public static function enable(Codebase $codebase): self
    {
        return self::$instance = new self($codebase);
    }

    public static function get(): ?self
    {
        return self::$instance;
    }

    public static function isEnabled(): bool
    {
        return self::$instance !== null;
    }

    /**
     * Whether the optimistic phase runs: a `never` result then stands for "not inferred yet" rather than for a
     * call that does not return, so it must not end control flow.
     */
    public static function optimistic(): bool
    {
        return self::$instance !== null && self::$instance->phase !== 'sound';
    }

    public static function hasMixed(Union $type): bool
    {
        foreach ($type->getAtomicTypes() as $atomic) {
            if ($atomic instanceof Atomic\TMixed) {
                return true;
            }
        }
        return false;
    }

    /** Whether a file's function-likes and classes are typed by inference (project code, not trusted stubs). */
    public function infers(?string $file_path): bool
    {
        return $file_path !== null && $this->codebase->config->ignoresDocblockTypesIn($file_path);
    }

    private function storageFile(FunctionLikeStorage $storage): ?string
    {
        return $storage->location?->file_path;
    }

    // ------------------------------------------------------------------------------------------------------------
    // Recording (called by the analyzers)
    // ------------------------------------------------------------------------------------------------------------

    /**
     * The arguments of a call whose parameters were matched.
     *
     * @param list<PhpParser\Node\Arg> $args
     * @param array<int, list<FunctionLikeParameter>> $arg_function_params argument offset => params it binds
     */
    public function recordCall(
        StatementsAnalyzer $statements_analyzer,
        ?FunctionLikeStorage $storage,
        array $args,
        array $arg_function_params,
        bool $has_packed_var,
    ): void {
        if ($storage === null || !$this->infers($this->storageFile($storage))) {
            return;
        }
        $id = spl_object_id($storage);
        $this->storages[$id] = $storage;
        $count = \count($args);
        $has_unpack = $has_packed_var;
        $old = $this->arities[$id] ?? [$count, $count];
        $this->arities[$id] = [\min($old[0], $count), $has_unpack || $old[1] === null ? null : \max($old[1], $count)];
        $recursive = $statements_analyzer->getSource() instanceof \Psalm\Internal\Analyzer\FunctionLikeAnalyzer
            && $this->callerStorage($statements_analyzer) === $storage;

        $offsets = [];
        foreach ($storage->params as $i => $param) {
            $offsets[$param->name] = $i;
        }

        $bound = [];
        foreach ($args as $argument_offset => $arg) {
            if (!isset($arg_function_params[$argument_offset])) {
                continue;
            }
            $arg_type = $statements_analyzer->node_data->getType($arg->value) ?? Type::getMixed();
            if ($arg->unpack) {
                $arg_type = self::iterableValueType($arg_type);
            }
            foreach ($arg_function_params[$argument_offset] as $param) {
                $i = $offsets[$param->name] ?? null;
                if ($i === null) {
                    continue;
                }
                if (!$arg->unpack) {
                    $bound[$i] = true;
                }
                ($this->args[$id][$i] ??= new ClosedWorldSlot())->add($arg_type, $recursive);
            }
        }

        // parameters the call leaves to their default
        foreach ($storage->params as $i => $param) {
            if (isset($bound[$i]) || $param->is_variadic) {
                continue;
            }
            $default = $this->defaultType($param, $statements_analyzer);
            if ($default !== null) {
                ($this->args[$id][$i] ??= new ClosedWorldSlot())->add($default, $recursive);
            } elseif ($has_packed_var) {
                ($this->args[$id][$i] ??= new ClosedWorldSlot())->add(Type::getMixed(), $recursive);
            }
        }
    }

    /**
     * A method call on a receiver whose type is unknown: in a closed world it may reach any method of that name.
     *
     * @param list<PhpParser\Node\Arg> $args
     */
    public function recordUnknownMethodCall(StatementsAnalyzer $statements_analyzer, string $method_name, array $args): void
    {
        $name = strtolower($method_name);
        foreach ($args as $offset => $arg) {
            $type = $statements_analyzer->node_data->getType($arg->value) ?? Type::getMixed();
            if ($arg->unpack || $arg->name !== null) {
                $type = Type::getMixed();
            }
            ($this->unknown_method_args[$name][$offset] ??= new ClosedWorldSlot())->add($type, false);
        }
    }

    /**
     * The result of analyzing a function-like's body: its return contributions and what it leaves in its
     * by-reference parameters.
     *
     * @param array<string, list<Union>> $var_types every type each variable had in the body
     */
    public function recordBody(
        FunctionLikeStorage $storage,
        array $stmts,
        StatementsAnalyzer $statements_analyzer,
        array $var_types,
    ): void {
        if (!$this->infers($this->storageFile($storage))) {
            return;
        }
        $id = spl_object_id($storage);
        $this->storages[$id] = $storage;
        $this->analyzed[$id] = true;
        foreach ($storage->params as $i => $param) {
            if (!$param->by_ref) {
                continue;
            }
            $slot = $this->outs[$id][$i] ??= new ClosedWorldSlot();
            foreach ($var_types['$' . $param->name] ?? [Type::getMixed()] as $type) {
                $slot->add($type, false);
            }
        }
        if ($storage->has_yield || ($storage instanceof MethodStorage && $storage->abstract)) {
            return;
        }
        $yield_types = [];
        $parts = ReturnTypeCollector::getReturnTypes(
            $this->codebase,
            $statements_analyzer->node_data,
            $stmts,
            $yield_types,
            true,
        );
        $actions = ScopeAnalyzer::getControlActions($stmts, $statements_analyzer->node_data, [], false);
        if ($actions === [ScopeAnalyzer::ACTION_END]) {
            $this->always_exits[$id] = true;
            return;
        }
        $slot = $this->returns[$id] ??= new ClosedWorldSlot();
        foreach ($parts as $part) {
            if ($part->isNever()) {
                continue;
            }
            $slot->add($part->isVoid() ? Type::getNull() : $part, false);
        }
        $implicit = false;
        foreach ($actions as $action) {
            if ($action !== ScopeAnalyzer::ACTION_END && $action !== ScopeAnalyzer::ACTION_RETURN) {
                $implicit = true;
            }
        }
        if ($implicit || $parts === []) {
            $slot->add(Type::getNull(), false);
            $slot->void_paths = true;
        }
    }

    /** A write of a value to a declared property. */
    public function recordPropertyWrite(string $property_id, Union $type): void
    {
        [$class, $name] = self::splitPropertyId($property_id);
        $declaring = $this->codebase->properties->getDeclaringClassForProperty($property_id, true);
        if ($declaring !== null) {
            $class = $declaring;
        }
        ($this->props[strtolower($class) . '::' . $name] ??= new ClosedWorldSlot())->add($type, false);
    }

    /** A write of a value to a property the class does not declare (PHP creates a dynamic property). */
    public function recordDynamicPropertyWrite(string $fq_class_name, string $name, Union $type): void
    {
        $class_lc = strtolower($fq_class_name);
        if (!$this->codebase->classlike_storage_provider->has($class_lc)) {
            return;
        }
        $class_storage = $this->codebase->classlike_storage_provider->get($class_lc);
        if (!$this->infers($class_storage->location?->file_path)) {
            return;
        }
        ($this->dynamic_props[$class_lc][$name] ??= new ClosedWorldSlot())->add($type, false);
    }

    /**
     * A property bound by reference (`$r = &$o->p`, `foreach ($o->p as &$v)`...): writes through the reference
     * are not seen, so the property may hold anything.
     */
    public function recordPropertyAliased(string $property_id): void
    {
        $this->recordPropertyWrite($property_id, Type::getMixed());
    }

    /**
     * `global $x` / `static $x`: the local is bound to the slot; its type is the slot's (null: not inferred).
     */
    public function bindGlobal(\Psalm\Context $context, string $var_id, string $key): ?Union
    {
        $context->closed_world_globals[$var_id] = $key;
        if (isset($this->escaped_globals['*']) || isset($this->escaped_globals[$key])) {
            return null;
        }
        // not inferred yet: the value of a variable nothing wrote
        return $this->global_types[$key] ??= Type::getNull();
    }

    public function staticKey(StatementsAnalyzer $statements_analyzer, string $name): string
    {
        $storage = $this->callerStorage($statements_analyzer);
        return 's:' . ($storage === null ? $statements_analyzer->getFilePath() : spl_object_id($storage)) . ':' . $name;
    }

    /** After a call that may write globals: the bound locals hold their slots' types again. */
    public function refreshGlobals(\Psalm\Context $context): void
    {
        foreach ($context->closed_world_globals as $var_id => $key) {
            if (isset($context->vars_in_scope[$var_id])) {
                $context->vars_in_scope[$var_id] = $this->bindGlobal($context, $var_id, $key)
                    ?? Type::getMixed();
            }
        }
    }

    /**
     * After each statement: the global-bound locals whose type is no longer the bound one were written (or
     * narrowed, which contributes a subset): their types are contributions to the slots. At file scope every
     * variable is a global.
     */
    public function recordStatementVars(StatementsAnalyzer $statements_analyzer, \Psalm\Context $context): void
    {
        if (!$this->infers($statements_analyzer->getFilePath())) {
            return;
        }
        if ($statements_analyzer->getSource() instanceof \Psalm\Internal\Analyzer\FileAnalyzer) {
            foreach ($context->vars_in_scope as $var_id => $_) {
                if (\preg_match('/^\$[A-Za-z_\x80-\xff][A-Za-z0-9_\x80-\xff]*$/', $var_id) && $var_id !== '$this') {
                    $context->closed_world_globals[$var_id] ??= 'g:' . \substr($var_id, 1);
                }
            }
        }
        foreach ($context->closed_world_globals as $var_id => $key) {
            $type = $context->vars_in_scope[$var_id] ?? null;
            if ($type === null || $type === ($this->global_types[$key] ?? null)) {
                continue;
            }
            ($this->global_writes[$key] ??= new ClosedWorldSlot())->add($type, false);
        }
    }

    /** The initial value of a static variable. */
    public function recordStaticDefault(string $key, Union $type): void
    {
        ($this->global_writes[$key] ??= new ClosedWorldSlot())->add($type, false);
    }

    private function applyGlobals(string $mode): int
    {
        $changed = 0;
        $keys = \array_unique([...array_keys($this->global_writes), ...array_keys($this->global_types)]);
        \sort($keys);
        foreach ($keys as $key) {
            $slot = $this->global_writes[$key] ?? new ClosedWorldSlot();
            if (isset($this->escaped_globals['*']) || isset($this->escaped_globals[$key])) {
                $new = Type::getMixed();
            } else {
                // the first pass read-modified-wrote it as `mixed`: its contributions are discarded (a global
                // nothing wrote yet holds null)
                $new = $mode === 'first' ? Type::getNull() : $slot->value($mode, null);
                if ($new === null && $mode === 'sound') {
                    // nothing ever written: reads give null
                    $new = $slot->isEmpty() ? Type::getNull() : Type::getMixed();
                }
            }
            $k = 'G:' . $key;
            $new = $this->settle($k, $new);
            if ($this->assign($k, $new)) {
                if ($new === null) {
                    unset($this->global_types[$key]);
                } else {
                    $this->global_types[$key] = $new;
                }
                $changed++;
            }
        }
        return $changed;
    }

    /** A write of a value to `$o->p` (by the types of `$o`). */
    public function recordPropertyFetchWrite(
        StatementsAnalyzer $statements_analyzer,
        PhpParser\Node\Expr\PropertyFetch|PhpParser\Node\Expr\StaticPropertyFetch $fetch,
        Union $type,
    ): void {
        if (!$fetch->name instanceof PhpParser\Node\Identifier && !$fetch->name instanceof PhpParser\Node\VarLikeIdentifier) {
            $this->recordAnyPropertyWrite('*', $type);
            return;
        }
        $name = $fetch->name->name;
        $receiver = $fetch instanceof PhpParser\Node\Expr\PropertyFetch ? $fetch->var : $fetch->class;
        $receiver_type = $receiver instanceof PhpParser\Node\Expr ? $statements_analyzer->node_data->getType($receiver) : null;
        if ($receiver instanceof PhpParser\Node\Name) {
            $class = $receiver->getAttribute('resolvedName');
            $lc = strtolower($receiver->toString());
            if ($lc === 'self' || $lc === 'static' || $lc === 'parent') {
                $class = $statements_analyzer->getFQCLN();
                if ($lc === 'parent' && $class !== null) {
                    $class = $this->codebase->classlike_storage_provider->get($class)->parent_class;
                }
            }
            if (\is_string($class)) {
                $this->recordPropertyWrite($class . '::$' . $name, $type);
                return;
            }
        }
        if ($receiver_type === null) {
            $this->recordUnknownPropertyWrite($name, $type);
            return;
        }
        foreach ($receiver_type->getAtomicTypes() as $atomic) {
            if ($atomic instanceof Atomic\TNamedObject
                && $this->codebase->properties->propertyExists($atomic->value . '::$' . $name, false)
            ) {
                $this->recordPropertyWrite($atomic->value . '::$' . $name, $type);
            } elseif ($atomic instanceof Atomic\TNamedObject) {
                $this->recordDynamicPropertyWrite($atomic->value, $name, $type);
            } elseif (!$atomic instanceof Atomic\TNull) {
                $this->recordUnknownPropertyWrite($name, $type);
            }
        }
    }

    /**
     * `$r = &<expr>`: a write through `$r` is not seen as a write of a property or array item it aliases. The
     * property may then hold anything; the array's items too.
     */
    public function recordReferenceTo(
        StatementsAnalyzer $statements_analyzer,
        PhpParser\Node\Expr $target,
        \Psalm\Context $context,
    ): void {
        $root = $target;
        $through_item = false;
        while ($root instanceof PhpParser\Node\Expr\ArrayDimFetch) {
            $root = $root->var;
            $through_item = true;
        }
        if ($root instanceof PhpParser\Node\Expr\PropertyFetch || $root instanceof PhpParser\Node\Expr\StaticPropertyFetch) {
            $this->recordPropertyFetchWrite($statements_analyzer, $root, Type::getMixed());
            return;
        }
        if (!$through_item) {
            return;
        }
        $root_id = \Psalm\Internal\Analyzer\Statements\Expression\ExpressionIdentifier::getExtendedVarId(
            $root,
            $statements_analyzer->getFQCLN(),
            $statements_analyzer,
        );
        if ($root_id !== null && isset($context->vars_in_scope[$root_id])) {
            $context->vars_in_scope[$root_id] = self::withMixedItems($context->vars_in_scope[$root_id]);
        }
    }

    private static function withMixedItems(Union $type): Union
    {
        $atomics = [];
        foreach ($type->getAtomicTypes() as $atomic) {
            if ($atomic instanceof TKeyedArray) {
                $atomic = $atomic->getGenericArrayType();
            }
            if ($atomic instanceof TArray) {
                $atomic = new TArray([$atomic->type_params[0], Type::getMixed()]);
            }
            $atomics[] = $atomic;
        }
        return new Union($atomics);
    }

    /** Whether a parameter type takes callables. */
    public static function isCallableParam(Union $type): bool
    {
        foreach ($type->getAtomicTypes() as $atomic) {
            if ($atomic instanceof Atomic\TCallable
                || $atomic instanceof Atomic\TClosure
                || $atomic instanceof Atomic\TCallableString
                || $atomic instanceof Atomic\TCallableObject
                || ($atomic instanceof TKeyedArray && $atomic->is_callable)
            ) {
                return true;
            }
        }
        return false;
    }

    /** A property write on a receiver of unknown type. */
    public function recordUnknownPropertyWrite(string $name, Union $type): void
    {
        ($this->unknown_prop_writes[$name] ??= new ClosedWorldSlot())->add($type, false);
    }

    /** `$o->$name = v` with a name of unknown value: any property of the class may be written. */
    public function recordAnyPropertyWrite(string $fq_class_name, Union $type): void
    {
        ($this->any_prop_writes[strtolower($fq_class_name)] ??= new ClosedWorldSlot())->add($type, false);
    }

    /** @var list<string> lowercase literal prefixes of concatenated strings (`'prefix_' . $x` may name a callable) */
    private array $string_prefixes = [];

    private bool $strings_collected = false;

    /**
     * The names a callable may be built from: every string literal of the project, the literal prefixes of
     * concatenations and the names of first-class callables (`f(...)`, `$o->m(...)`). A function-like named by
     * one of them may be called from where the analysis cannot see its arguments.
     */
    private function collectStrings(): void
    {
        if ($this->strings_collected) {
            return;
        }
        $this->strings_collected = true;
        $finder = new PhpParser\NodeFinder();
        foreach ($this->codebase->file_storage_provider->getAll() as $file_storage) {
            if (!$this->infers($file_storage->file_path)) {
                continue;
            }
            try {
                $stmts = $this->codebase->getStatementsForFile($file_storage->file_path);
            } catch (\Throwable) {
                continue;
            }
            foreach ($finder->find($stmts, static fn(PhpParser\Node $n): bool =>
                $n instanceof PhpParser\Node\Scalar\String_
                || $n instanceof PhpParser\Node\Expr\BinaryOp\Concat
                || $n instanceof PhpParser\Node\Scalar\InterpolatedString
                || ($n instanceof PhpParser\Node\Expr\CallLike && $n->isFirstClassCallable())
                || ($n instanceof PhpParser\Node\Expr\ArrayDimFetch && $n->var instanceof PhpParser\Node\Expr\Variable
                    && $n->var->name === 'GLOBALS')) as $node
            ) {
                if ($node instanceof PhpParser\Node\Expr\ArrayDimFetch) {
                    // `$GLOBALS['x']`: the global may be written without a `global` binding
                    $this->escaped_globals[$node->dim instanceof PhpParser\Node\Scalar\String_
                        ? 'g:' . $node->dim->value : '*'] = true;
                } elseif ($node instanceof PhpParser\Node\Scalar\String_) {
                    $this->strings[strtolower(\ltrim($node->value, '\\'))] = true;
                } elseif ($node instanceof PhpParser\Node\Expr\BinaryOp\Concat) {
                    $left = $node->left;
                    while ($left instanceof PhpParser\Node\Expr\BinaryOp\Concat) {
                        $left = $left->left;
                    }
                    $prefix = $left instanceof PhpParser\Node\Scalar\String_ ? $left->value : '';
                    $this->string_prefixes[] = strtolower(\ltrim($prefix, '\\'));
                } elseif ($node instanceof PhpParser\Node\Scalar\InterpolatedString) {
                    $first = $node->parts[0] ?? null;
                    $prefix = $first instanceof PhpParser\Node\InterpolatedStringPart ? $first->value : '';
                    $this->string_prefixes[] = strtolower(\ltrim($prefix, '\\'));
                } elseif ($node instanceof PhpParser\Node\Expr\FuncCall && $node->name instanceof PhpParser\Node\Name) {
                    $resolved = $node->name->getAttribute('resolvedName');
                    $this->strings[strtolower(\is_string($resolved) ? $resolved : $node->name->toString())] = true;
                    $this->strings[strtolower($node->name->getLast())] = true;
                } elseif (($node instanceof PhpParser\Node\Expr\MethodCall
                        || $node instanceof PhpParser\Node\Expr\NullsafeMethodCall
                        || $node instanceof PhpParser\Node\Expr\StaticCall)
                    && $node->name instanceof PhpParser\Node\Identifier
                ) {
                    $this->strings[strtolower($node->name->name)] = true;
                }
            }
        }
    }

    /**
     * A value used as a callable (called dynamically, or passed where a callable is expected). A string that is
     * not a known literal may name any function-like whose name the program can build.
     */
    public function recordCallableValue(?Union $type): void
    {
        if ($type === null) {
            $this->unknown_callables = true;
            return;
        }
        foreach ($type->getAtomicTypes() as $atomic) {
            if ($atomic instanceof TLiteralString
                || $atomic instanceof Atomic\TClosure
                || $atomic instanceof Atomic\TNull
                || $atomic instanceof Atomic\TFalse
            ) {
                continue;
            }
            if ($atomic instanceof TKeyedArray && $atomic->is_list && isset($atomic->properties[1])
                && $atomic->properties[1]->isSingleStringLiteral()
            ) {
                continue;
            }
            if ($atomic instanceof Atomic\TNamedObject && !$atomic instanceof Atomic\TClosure) {
                // an invokable object: its __invoke is magic, hence escaped
                continue;
            }
            $this->unknown_callables = true;
        }
    }

    /** A method called by a name the analysis does not know (`$o->$name()`, `[$o, $name]`). */
    public function recordUnknownMethodName(): void
    {
        $this->unknown_callables = true;
    }

    /** Whether the function named so (lowercase, fully qualified) may be called through a string. */
    public function mayBeCalledByName(string $lc): bool
    {
        return $this->nameMayBeBuilt(\ltrim($lc, '\\'));
    }

    /**
     * Whether a lowercase name may be the value of a string the program builds and uses as a callable: a string
     * literal of the program, or (when some callable value is not a known literal) a concatenation starting with
     * a literal prefix of the name. A name built without any literal part (from input) is outside this model.
     */
    private function nameMayBeBuilt(string $lc): bool
    {
        if (isset($this->strings[$lc])) {
            return true;
        }
        if (!$this->unknown_callables) {
            return false;
        }
        foreach ($this->string_prefixes as $prefix) {
            if ($prefix !== '' && \str_starts_with($lc, $prefix)) {
                return true;
            }
        }
        return false;
    }

    private function callerStorage(StatementsAnalyzer $statements_analyzer): ?FunctionLikeStorage
    {
        $source = $statements_analyzer->getSource();
        if ($source instanceof \Psalm\Internal\Analyzer\FunctionLikeAnalyzer) {
            try {
                return $source->getFunctionLikeStorage($statements_analyzer);
            } catch (\Throwable) {
                return null;
            }
        }
        return null;
    }

    private function defaultType(FunctionLikeParameter $param, StatementsAnalyzer $statements_analyzer): ?Union
    {
        $default = $param->default_type;
        if ($default === null) {
            return null;
        }
        if ($default instanceof Union) {
            return $default;
        }
        try {
            return new Union([\Psalm\Internal\Codebase\ConstantTypeResolver::resolve(
                $this->codebase->classlikes,
                $default,
                $statements_analyzer,
            )]);
        } catch (\Throwable) {
            return Type::getMixed();
        }
    }

    private static function iterableValueType(Union $type): Union
    {
        $values = [];
        foreach ($type->getAtomicTypes() as $atomic) {
            if ($atomic instanceof TKeyedArray) {
                $values[] = $atomic->getGenericValueType();
            } elseif ($atomic instanceof TArray || $atomic instanceof TIterable) {
                $values[] = $atomic->type_params[1];
            } else {
                return Type::getMixed();
            }
        }
        return $values ? Type::combineUnionTypeArray($values, null) : Type::getMixed();
    }

    /** @return array{string, string} */
    private static function splitPropertyId(string $property_id): array
    {
        $pos = \strpos($property_id, '::$');
        if ($pos === false) {
            return [$property_id, ''];
        }
        return [\substr($property_id, 0, $pos), \substr($property_id, $pos + 3)];
    }

    // ------------------------------------------------------------------------------------------------------------
    // Fixpoint
    // ------------------------------------------------------------------------------------------------------------

    /** Forget the contributions of the previous pass (before re-analyzing). */
    public function startPass(): void
    {
        $this->pass++;
        $this->args = [];
        $this->arities = [];
        $this->branches = [];
        $this->global_writes = [];
        $this->outs = [];
        $this->unknown_callables = false;
        $this->unknown_method_args = [];
        $this->returns = [];
        $this->always_exits = [];
        $this->analyzed = [];
        $this->props = [];
        $this->unknown_prop_writes = [];
        $this->dynamic_props = [];
        $this->any_prop_writes = [];
    }

    /**
     * Write the contributions of the pass that just ended into the storages.
     *
     * @return int number of slots whose value changed (0: fixpoint reached)
     */
    public function apply(): int
    {
        $this->collectProjectStorages();
        $this->collectStrings();
        if ($this->phase === 'optimistic' && $this->pass >= self::OPTIMISTIC_PASSES) {
            $this->phase = 'sound';
        }
        $changed = $this->applyAll($this->pass === 0 ? 'first' : $this->phase);
        if ($changed === 0 && $this->phase === 'optimistic') {
            // the optimistic fixpoint: now account for the `mixed` contributions too
            $this->phase = 'sound';
            $changed = $this->applyAll('sound');
        }
        $this->report($changed);
        return $changed;
    }

    private function applyAll(string $mode): int
    {
        $changed = 0;

        // --- parameters and returns, grouped over method overrides -------------------------------------------
        $groups = $this->methodGroups();
        foreach ($this->storages as $id => $storage) {
            $group = $groups[$id] ?? [$id];
            $escaped = $this->isEscaped($storage);
            foreach ($storage->params as $i => $param) {
                $key = 'p:' . $id . ':' . $i;
                $this->declared_params[$key] ??= [$param, $param->type];
                $declared = $this->declared_params[$key][1];
                if ($escaped) {
                    $new = $declared;
                } else {
                    $slot = new ClosedWorldSlot();
                    foreach ($group as $member) {
                        if (isset($this->args[$member][$i])) {
                            $slot->merge($this->args[$member][$i]);
                        }
                    }
                    if ($storage instanceof MethodStorage && $storage->cased_name !== null
                        && isset($this->unknown_method_args[strtolower($storage->cased_name)][$i])
                    ) {
                        $slot->merge($this->unknown_method_args[strtolower($storage->cased_name)][$i]);
                    }
                    $new = $slot->value($mode, $declared) ?? $declared;
                    if ($new !== null && $declared !== null && !$this->refines($new, $declared)) {
                        // a value the native type coerces (int into a float parameter, ...): the parameter
                        // holds the native type
                        $new = $declared;
                    }
                }
                $new = $this->settle($key, $new);
                if ($this->assign($key, $new)) {
                    $param->type = $new;
                    $param->type_location = null;
                    $changed++;
                }
                if ($param->by_ref) {
                    $key = 'o:' . $id . ':' . $i;
                    $this->declared_params[$key] ??= [$param, $param->out_type];
                    if ($escaped) {
                        $out = $this->declared_params[$key][1];
                    } else {
                        $slot = new ClosedWorldSlot();
                        foreach ($group as $member) {
                            if (isset($this->outs[$member][$i])) {
                                $slot->merge($this->outs[$member][$i]);
                            }
                        }
                        // what the caller passed stays when the body leaves the parameter alone
                        $out = $slot->value($mode === 'sound' ? 'sound' : 'optimistic', null);
                        if ($out !== null && $new !== null) {
                            $out = Type::combineUnionTypes($out, $new, $this->codebase);
                        }
                        $out ??= $mode === 'sound' ? Type::getMixed() : $new;
                    }
                    $out = $this->settle($key, $out);
                    if ($this->assign($key, $out)) {
                        $param->out_type = $out;
                        $changed++;
                    }
                }
            }

            // return type
            if ($this->isMagicWithoutReturn($storage)) {
                continue;
            }
            $key = 'r:' . $id;
            $this->declared_returns[$id] ??= [$storage->return_type];
            $declared = $storage->signature_return_type;
            if ($storage->has_yield || ($declared !== null && ($declared->isVoid() || $declared->isNever()))) {
                continue;
            }
            $slot = new ClosedWorldSlot();
            $any = false;
            $always_exits = true;
            foreach ($group as $member) {
                if (isset($this->returns[$member])) {
                    $slot->merge($this->returns[$member]);
                    $any = true;
                    $always_exits = false;
                } elseif (!isset($this->always_exits[$member])) {
                    $always_exits = false;
                } else {
                    $any = true;
                }
            }
            if (!$any) {
                continue;
            }
            if ($always_exits) {
                $new = Type::getNever();
            } else {
                $new = $slot->value($mode, $declared);
                if ($new !== null && $slot->void_paths && $slot->isOnlyNull()) {
                    $new = Type::getVoid();
                }
                if ($new !== null && $declared !== null && !$this->refines($new, $declared)) {
                    $new = $declared;
                }
                if ($new === null && $mode !== 'sound' && $declared === null) {
                    // not inferred yet: optimistically nothing (a call of it then contributes nothing, see
                    // optimistic()); the sound phase drops this assumption
                    $new = Type::getNever();
                }
            }
            $new = $this->settle($key, $new);
            if ($this->assign($key, $new)) {
                $storage->return_type = $new ?? $this->declared_returns[$id][0];
                $changed++;
            }
        }

        // --- globals and statics ------------------------------------------------------------------------------
        $changed += $this->applyGlobals($mode);

        // --- properties --------------------------------------------------------------------------------------
        $changed += $this->applyDynamicProperties();
        $changed += $this->applyProperties($mode);
        return $changed;
    }

    private function report(int $changed): void
    {
        fwrite(STDERR, "[closed-world] pass {$this->pass} ({$this->phase}): $changed slots changed\n");
        if (getenv('CW_DEBUG')) {
            foreach ($this->values as $key => $value) {
                fwrite(STDERR, '  ' . $this->describe($key) . " = $value\n");
            }
        }
    }

    /** A readable name of a slot key (debug output). */
    private function describe(string $key): string
    {
        $parts = \explode(':', $key);
        if (($parts[0] === 'p' || $parts[0] === 'r' || $parts[0] === 'o') && isset($this->storages[(int) $parts[1]])) {
            $storage = $this->storages[(int) $parts[1]];
            $name = ($storage instanceof MethodStorage ? $storage->defining_fqcln . '::' : '')
                . ($storage->cased_name ?? '?');
            if ($parts[0] === 'p' || $parts[0] === 'o') {
                return $name . '(' . ($parts[0] === 'o' ? 'out ' : '') . '$'
                    . ($storage->params[(int) $parts[2]]->name ?? '?') . ')';
            }
            return $name . '(): return';
        }
        return $key;
    }

    /** Record a slot's new value; whether it changed. */
    private function assign(string $key, ?Union $new): bool
    {
        $id = $new === null ? '-' : $new->getId();
        if (($this->values[$key] ?? null) === $id) {
            return false;
        }
        $this->values[$key] = $id;
        if ($new !== null) {
            $this->previous[$key] = $new;
        }
        return true;
    }

    /** Widening: a slot that keeps changing is joined with its previous value, then given up on (mixed). */
    private function settle(string $key, ?Union $new): ?Union
    {
        $old = $this->values[$key] ?? null;
        $new_id = $new === null ? '-' : $new->getId();
        if ($old === null || $old === $new_id || $this->phase !== 'sound') {
            return $new;
        }
        $n = $this->changes[$key] = ($this->changes[$key] ?? 0) + 1;
        if ($n >= self::TOP_AFTER) {
            return Type::getMixed();
        }
        if ($n >= self::JOIN_AFTER && $new !== null && isset($this->previous[$key])) {
            return self::generalize(Type::combineUnionTypes($this->previous[$key], $new, $this->codebase), true);
        }
        return $new;
    }

    private function refines(Union $inferred, Union $declared): bool
    {
        if ($declared->isMixed()) {
            return true;
        }
        try {
            return UnionTypeComparator::isContainedBy($this->codebase, $inferred, $declared);
        } catch (\Throwable) {
            return false;
        }
    }

    private function isMagicWithoutReturn(FunctionLikeStorage $storage): bool
    {
        if (!$storage instanceof MethodStorage || $storage->cased_name === null) {
            return false;
        }
        $lc = strtolower($storage->cased_name);
        return $lc === '__construct' || $lc === '__destruct' || $lc === '__clone' || $lc === '__set'
            || $lc === '__unset' || $lc === '__wakeup';
    }

    /**
     * Whether a function-like may be called from places the analysis cannot attribute arguments from: callables
     * built from strings, closures, magic methods. Its parameters keep their declared types.
     */
    private function isEscaped(FunctionLikeStorage $storage): bool
    {
        $name = $storage->cased_name;
        if ($name === null || $storage->location === null) {
            return true;
        }
        if ($storage instanceof MethodStorage) {
            $lc = strtolower($name);
            if (\str_starts_with($lc, '__') && $lc !== '__construct') {
                return true;
            }
            // `[$obj, 'name']`, `'Class::name'`, `[Class::class, 'name']`
            return $this->nameMayBeBuilt($lc)
                || $this->nameMayBeBuilt(strtolower((string) $storage->defining_fqcln) . '::' . $lc);
        }
        if (\str_contains($name, 'closure') || $storage->cased_name === '{closure}') {
            return true;
        }
        $fq = strtolower($name);
        foreach ($this->codebase->file_storage_provider->get($storage->location->file_path)->functions as $fid => $fs) {
            if ($fs === $storage) {
                $fq = (string) $fid;
                break;
            }
        }
        return $this->nameMayBeBuilt($fq);
    }

    /** Make sure every project function-like is known (also those never called). */
    private function collectProjectStorages(): void
    {
        foreach ($this->codebase->file_storage_provider->getAll() as $file_storage) {
            if (!$this->infers($file_storage->file_path)) {
                continue;
            }
            foreach ($file_storage->functions as $storage) {
                $this->storages[spl_object_id($storage)] ??= $storage;
            }
        }
        foreach ($this->codebase->classlike_storage_provider->getAll() as $class_storage) {
            if (!$this->infers($class_storage->location?->file_path)) {
                continue;
            }
            foreach ($class_storage->methods as $storage) {
                $this->storages[spl_object_id($storage)] ??= $storage;
            }
        }
        ksort($this->storages);
    }

    /**
     * Methods whose calls may dispatch to one another (overrides, in both directions) share parameter and
     * return slots: storage id => ids of the group.
     *
     * @return array<int, list<int>>
     */
    private function methodGroups(): array
    {
        $parent = [];
        $find = static function (int $x) use (&$parent, &$find): int {
            while (($parent[$x] ?? $x) !== $x) {
                $x = $parent[$x] = $parent[$parent[$x]] ?? $parent[$x];
            }
            return $x;
        };
        foreach ($this->storages as $id => $storage) {
            if (!$storage instanceof MethodStorage || $storage->cased_name === null || $storage->defining_fqcln === null) {
                continue;
            }
            $lc = strtolower($storage->cased_name);
            if ($lc === '__construct') {
                continue;
            }
            $parent[$id] ??= $id;
            try {
                $class_storage = $this->codebase->classlike_storage_provider->get($storage->defining_fqcln);
            } catch (\Throwable) {
                continue;
            }
            foreach ($class_storage->overridden_method_ids[$lc] ?? [] as $overridden) {
                try {
                    $other = $this->codebase->methods->getStorage($overridden);
                } catch (\Throwable) {
                    continue;
                }
                $oid = spl_object_id($other);
                if (!isset($this->storages[$oid])) {
                    continue;
                }
                $parent[$oid] ??= $oid;
                $a = $find($id);
                $b = $find($oid);
                if ($a !== $b) {
                    $parent[$a] = $b;
                }
            }
        }
        $members = [];
        foreach (array_keys($parent) as $id) {
            $members[$find($id)][] = $id;
        }
        $groups = [];
        foreach ($members as $list) {
            foreach ($list as $id) {
                $groups[$id] = $list;
            }
        }
        return $groups;
    }

    private function applyProperties(string $mode): int
    {
        $changed = 0;
        foreach ($this->codebase->classlike_storage_provider->getAll() as $class_lc => $class_storage) {
            if (!$this->infers($class_storage->location?->file_path)) {
                continue;
            }
            foreach ($class_storage->properties as $name => $prop) {
                $key = 'P:' . strtolower($class_storage->name) . '::' . $name;
                $this->declared_props[$key] ??= [$prop, $prop->type];
                $declared = $prop->signature_type;
                $slot = new ClosedWorldSlot();
                if (isset($this->props[strtolower($class_storage->name) . '::' . $name])) {
                    $slot->merge($this->props[strtolower($class_storage->name) . '::' . $name]);
                }
                if (isset($this->unknown_prop_writes[$name])) {
                    $slot->merge($this->unknown_prop_writes[$name]);
                }
                foreach ($this->any_prop_writes as $writer_class => $any) {
                    if ($writer_class === '*' || $writer_class === strtolower($class_storage->name)
                        || isset($class_storage->parent_classes[$writer_class])
                        || ($this->codebase->classlike_storage_provider->has($writer_class)
                            && $this->codebase->classExtendsOrImplements($writer_class, $class_storage->name))
                    ) {
                        $slot->merge($any);
                    }
                }
                if (!$prop->has_default && $declared === null && !$prop->is_promoted
                    && !$this->initializedBeforeRead($class_storage, $name)
                ) {
                    // an untyped property without a default starts out null
                    $slot->add(Type::getNull(), false);
                }
                if ($slot->isEmpty()) {
                    if ($prop->has_default || $declared !== null || isset($this->added_props[$class_lc][$name])) {
                        continue;
                    }
                }
                // the first pass read-modified-wrote properties as `mixed`: its contributions are discarded
                $new = $mode === 'first' ? null : $slot->value($mode, $declared);
                if ($new === null && $mode !== 'sound' && $declared === null) {
                    // not inferred yet: what an untyped property holds before anything is written
                    $new = Type::getNull();
                }
                if ($new !== null && $declared !== null && !$this->refines($new, $declared)) {
                    $new = $declared;
                }
                $new = $this->settle($key, $new);
                if ($this->assign($key, $new)) {
                    $prop->type = $new ?? $this->declared_props[$key][1];
                    $changed++;
                }
            }
        }
        return $changed;
    }

    /**
     * Declare the properties the program creates dynamically (`$post->title = ...` on a class without that
     * property), so that they are typed and become real fields.
     */
    private function applyDynamicProperties(): int
    {
        $changed = 0;
        foreach ($this->dynamic_props as $class_lc => $names) {
            $class_storage = $this->codebase->classlike_storage_provider->get($class_lc);
            foreach ($names as $name => $_slot) {
                if (isset($class_storage->declaring_property_ids[$name])) {
                    continue;
                }
                $prop = new PropertyStorage();
                $prop->visibility = \Psalm\Internal\Analyzer\ClassLikeAnalyzer::VISIBILITY_PUBLIC;
                $prop->is_static = false;
                $prop->location = $class_storage->location;
                $prop->type = Type::getMixed();
                $this->added_props[$class_lc][$name] = $prop;
                $this->declareProperty($class_storage, $name, $prop);
                $changed++;
                // the write is now a declared write: account it for the property slot of this pass
                ($this->props[$class_lc . '::' . $name] ??= new ClosedWorldSlot())->merge($_slot);
            }
        }
        return $changed;
    }

    /** Whether the property is one the program creates dynamically (declared by this inference). */
    public function isDynamicProperty(string $fq_class_name, string $name): bool
    {
        return isset($this->added_props[strtolower($fq_class_name)][$name]);
    }

    private function declareProperty(ClassLikeStorage $class_storage, string $name, PropertyStorage $prop): void
    {
        $class_storage->properties[$name] = $prop;
        $class_storage->declaring_property_ids[$name] = $class_storage->name;
        $class_storage->appearing_property_ids[$name] = $class_storage->name . '::$' . $name;
        $class_storage->inheritable_property_ids[$name] = $class_storage->name . '::$' . $name;
        // descendants inherit it
        foreach ($this->codebase->classlike_storage_provider->getAll() as $other) {
            if ($other === $class_storage || !isset($other->parent_classes[strtolower($class_storage->name)])) {
                continue;
            }
            if (isset($other->declaring_property_ids[$name])) {
                continue;
            }
            $other->declaring_property_ids[$name] = $class_storage->name;
            $other->appearing_property_ids[$name] = $class_storage->name . '::$' . $name;
            $other->inheritable_property_ids[$name] = $class_storage->name . '::$' . $name;
        }
    }

    /**
     * Whether every constructor run assigns the property before anything can read it: the constructor starts
     * with straight-line `$this->x = <expr not involving $this>;` statements that include this property. (A
     * class instantiated without its constructor - reflection, unserialize - is outside this model.)
     */
    private function initializedBeforeRead(ClassLikeStorage $class_storage, string $name): bool
    {
        $ctor = $class_storage->methods['__construct'] ?? null;
        if ($ctor === null || !\Psalm\Internal\Transpiler\Transpiler::isEnabled()) {
            return false;
        }
        $record = \Psalm\Internal\Transpiler\Transpiler::get()->classes[strtolower($class_storage->name)] ?? null;
        if ($record === null) {
            return false;
        }
        $node = null;
        foreach ($record->node->getMethods() as $method) {
            if (strtolower($method->name->name) === '__construct') {
                $node = $method;
            }
        }
        if ($node === null || $node->stmts === null) {
            return false;
        }
        $finder = new PhpParser\NodeFinder();
        foreach ($node->stmts as $stmt) {
            if (!$stmt instanceof PhpParser\Node\Stmt\Expression
                || !$stmt->expr instanceof PhpParser\Node\Expr\Assign
                || !$stmt->expr->var instanceof PhpParser\Node\Expr\PropertyFetch
                || !$stmt->expr->var->var instanceof PhpParser\Node\Expr\Variable
                || $stmt->expr->var->var->name !== 'this'
                || !$stmt->expr->var->name instanceof PhpParser\Node\Identifier
            ) {
                return false;
            }
            $uses_this = $finder->findFirst([$stmt->expr->expr], static fn(PhpParser\Node $n): bool =>
                ($n instanceof PhpParser\Node\Expr\Variable && $n->name === 'this')
                || $n instanceof PhpParser\Node\Expr\Closure
                || $n instanceof PhpParser\Node\Expr\ArrowFunction
                || ($n instanceof PhpParser\Node\Expr\StaticCall
                    && $n->class instanceof PhpParser\Node\Name
                    && \in_array(strtolower($n->class->toString()), ['self', 'static', 'parent'], true)));
            if ($uses_this !== null) {
                return false;
            }
            if ($stmt->expr->var->name->name === $name) {
                return true;
            }
        }
        return false;
    }

    // ------------------------------------------------------------------------------------------------------------
    // Dead branches
    // ------------------------------------------------------------------------------------------------------------

    /** @var array<string, array<int, true>> file => start of a branch (if, elseif, else) removed as dead */
    private array $dead = [];

    /** @var array<string, array<int, bool>> this pass: file => branch start => whether it was ever reachable */
    private array $branches = [];

    /**
     * A branch entered under its condition's assertions: when one of them leaves a variable no possible type,
     * the branch cannot run (with these types).
     *
     * @param array<string, mixed> $asserted_var_ids
     */
    public function recordBranch(
        StatementsAnalyzer $statements_analyzer,
        PhpParser\Node $branch,
        \Psalm\Context $context,
        array $asserted_var_ids,
    ): void {
        $file = $statements_analyzer->getFilePath();
        if (!$this->infers($file)) {
            return;
        }
        $reachable = true;
        foreach ($asserted_var_ids as $var_id => $_) {
            if (isset($context->vars_in_scope[$var_id]) && $context->vars_in_scope[$var_id]->isNever()) {
                $reachable = false;
                break;
            }
        }
        $pos = (int) $branch->getAttribute('startFilePos');
        $this->branches[$file][$pos] = ($this->branches[$file][$pos] ?? false) || $reachable;
    }

    /**
     * At a sound fixpoint, a branch whose condition cannot hold never runs: it is removed from the program, so
     * that it neither contributes to the inference nor gets emitted. Removing code only narrows the types, under
     * which the removed code stays dead.
     *
     * @return int number of newly removed branches
     */
    public function findDeadBranches(): int
    {
        if ($this->phase !== 'sound') {
            return 0;
        }
        $n = 0;
        foreach ($this->branches as $file => $branches) {
            foreach ($branches as $pos => $reachable) {
                if (!$reachable && !isset($this->dead[$file][$pos])) {
                    $this->dead[$file][$pos] = true;
                    $n++;
                }
            }
        }
        if ($n > 0) {
            fwrite(STDERR, "[closed-world] $n dead branches removed\n");
        }
        return $n;
    }

    /**
     * Rewrites of constructs whose dynamic part the whole-program types pin down, decided at a sound fixpoint and
     * applied whenever the file is parsed: file => node start => [kind, data].
     *
     * - `var`: `$$name` where `$name` is always one literal string: the variable it names;
     * - `args`: `func_get_args()` in a function every call of which passes exactly its parameters: the list of
     *   the parameters (PHP returns their current values);
     * - `extract`: `extract($sealed_shape);` as a statement: one assignment per key.
     *
     * @var array<string, array<int, array{string, mixed}>>
     */
    private array $rewrites = [];

    /**
     * Call arities seen this pass, by storage id: [min, max] argument counts, max null when an argument was
     * unpacked.
     *
     * @var array<int, array{int, ?int}>
     */
    private array $arities = [];

    /**
     * At a sound fixpoint: dead branches and dynamic constructs the types resolve.
     *
     * @return int number of new simplifications (the program changed: analyze again)
     */
    public function simplifyProgram(): int
    {
        if ($this->phase !== 'sound') {
            return 0;
        }
        $n = $this->findDeadBranches();
        if (!\Psalm\Internal\Transpiler\Transpiler::isEnabled()) {
            return $n;
        }
        $added = 0;
        foreach (\Psalm\Internal\Transpiler\Transpiler::get()->functions as $record) {
            $stmts = $record->node->getStmts();
            if (!$this->infers($record->file_path) || $stmts === null) {
                continue;
            }
            $file = $record->file_path;
            $storage = $record->storage;
            $exact_arity = !$record->node instanceof PhpParser\Node\Expr\Closure
                && !$record->node instanceof PhpParser\Node\Expr\ArrowFunction
                && !$this->isEscaped($storage)
                && $this->hasExactArity($storage);
            foreach (self::ownNodes($stmts) as $node) {
                $pos = (int) $node->getAttribute('startFilePos');
                if (isset($this->rewrites[$file][$pos])) {
                    continue;
                }
                if ($node instanceof PhpParser\Node\Expr\Variable && $node->name instanceof PhpParser\Node\Expr) {
                    $type = $record->node_data->getType($node->name);
                    if ($type !== null && $type->isSingleStringLiteral()
                        && \preg_match('/^[A-Za-z_\x80-\xff][A-Za-z0-9_\x80-\xff]*$/', $name = $type->getSingleStringLiteral()->value)
                        && $name !== 'this'
                    ) {
                        $this->rewrites[$file][$pos] = ['var', $name];
                        $added++;
                    }
                } elseif ($node instanceof PhpParser\Node\Expr\FuncCall && $node->name instanceof PhpParser\Node\Name
                    && strtolower($node->name->getLast()) === 'func_get_args' && $exact_arity && !$node->getArgs()
                ) {
                    $names = [];
                    foreach ($storage->params as $param) {
                        if ($param->is_variadic || $param->by_ref) {
                            continue 2;
                        }
                        $names[] = $param->name;
                    }
                    $this->rewrites[$file][$pos] = ['args', $names];
                    $added++;
                } elseif ($node instanceof PhpParser\Node\Stmt\Expression && $node->expr instanceof PhpParser\Node\Expr\FuncCall
                    && $node->expr->name instanceof PhpParser\Node\Name
                    && strtolower($node->expr->name->getLast()) === 'extract' && \count($node->expr->getArgs()) === 1
                    && ($keys = $this->sealedKeys($record->node_data->getType($node->expr->getArgs()[0]->value))) !== null
                ) {
                    $this->rewrites[$file][$pos] = ['extract', $keys];
                    $added++;
                }
            }
        }
        if ($added > 0) {
            fwrite(STDERR, "[closed-world] $added dynamic constructs resolved\n");
        }
        return $n + $added;
    }

    /** Whether every call of the function-like (or of its override group) passes exactly its parameters. */
    private function hasExactArity(FunctionLikeStorage $storage): bool
    {
        $n = 0;
        foreach ($storage->params as $param) {
            if ($param->is_variadic) {
                return false;
            }
            $n++;
        }
        $arity = $this->arities[spl_object_id($storage)] ?? null;
        return $arity !== null && $arity[0] === $n && $arity[1] === $n
            && !($storage instanceof MethodStorage && isset($this->unknown_method_args[strtolower((string) $storage->cased_name)]));
    }

    /**
     * The keys of a sealed shape that are all definitely present and valid variable names.
     *
     * @return list<string>|null
     */
    private function sealedKeys(?Union $type): ?array
    {
        if ($type === null || !$type->isSingle()) {
            return null;
        }
        $atomic = $type->getSingleAtomic();
        if (!$atomic instanceof TKeyedArray || $atomic->fallback_params !== null) {
            return null;
        }
        $keys = [];
        foreach ($atomic->properties as $key => $value) {
            if (!\is_string($key) || $value->possibly_undefined || $key === 'this'
                || !\preg_match('/^[A-Za-z_\x80-\xff][A-Za-z0-9_\x80-\xff]*$/', $key)
            ) {
                return null;
            }
            $keys[] = $key;
        }
        return $keys;
    }

    /**
     * The nodes of a body, not those of nested function-likes and classes.
     *
     * @param array<PhpParser\Node> $stmts
     * @return list<PhpParser\Node>
     */
    private static function ownNodes(array $stmts): array
    {
        $found = [];
        $visit = static function (PhpParser\Node $node) use (&$visit, &$found): void {
            if ($node instanceof PhpParser\Node\FunctionLike || $node instanceof PhpParser\Node\Stmt\ClassLike) {
                return;
            }
            $found[] = $node;
            foreach ($node->getSubNodeNames() as $name) {
                $sub = $node->$name;
                if ($sub instanceof PhpParser\Node) {
                    $visit($sub);
                } elseif (\is_array($sub)) {
                    foreach ($sub as $item) {
                        if ($item instanceof PhpParser\Node) {
                            $visit($item);
                        }
                    }
                }
            }
        };
        foreach ($stmts as $stmt) {
            $visit($stmt);
        }
        return $found;
    }

    /** @param list<PhpParser\Node\Stmt> $stmts */
    public function pruneStatements(string $file_path, array &$stmts): void
    {
        if (!isset($this->dead[$file_path]) && !isset($this->rewrites[$file_path])) {
            return;
        }
        $traverser = new PhpParser\NodeTraverser();
        $traverser->addVisitor(new ClosedWorldRewriter($file_path, $this->dead[$file_path] ?? [], $this->rewrites[$file_path] ?? []));
        $stmts = $traverser->traverse($stmts);
    }

    // ------------------------------------------------------------------------------------------------------------
    // Types
    // ------------------------------------------------------------------------------------------------------------

    /**
     * Drop the refinements that make iteration slow or endless: literal ints and floats, int ranges (and literal
     * strings too, when widening). The parent data-flow nodes are dropped as well.
     */
    public static function generalize(Union $type, bool $strings = false): Union
    {
        $atomics = [];
        $changed = false;
        foreach ($type->getAtomicTypes() as $atomic) {
            $new = self::generalizeAtomic($atomic, $strings);
            $changed = $changed || $new !== $atomic;
            $atomics[] = $new;
        }
        if (!$changed && !$type->parent_nodes) {
            return $type;
        }
        $result = $changed ? TypeCombiner::combine($atomics) : $type;
        return $result->setParentNodes([]);
    }

    /**
     * The type without its nested `mixed` parts: what an optimistic slot holds (an array of unknown items is
     * assumed to have none yet).
     */
    public static function stripMixed(Union $type): Union
    {
        $atomics = [];
        $changed = false;
        foreach ($type->getAtomicTypes() as $atomic) {
            $new = self::stripMixedAtomic($atomic);
            $changed = $changed || $new !== $atomic;
            if ($new !== null) {
                $atomics[] = $new;
            }
        }
        if (!$changed) {
            return $type;
        }
        return $atomics ? TypeCombiner::combine($atomics) : Type::getNever();
    }

    private static function stripMixedAtomic(Atomic $atomic): ?Atomic
    {
        if ($atomic instanceof Atomic\TMixed) {
            return null;
        }
        if ($atomic instanceof TKeyedArray) {
            $props = [];
            $changed = false;
            foreach ($atomic->properties as $k => $p) {
                $props[$k] = self::stripMixed($p);
                if ($props[$k]->isNever()) {
                    $props[$k] = $p;
                }
                $changed = $changed || $props[$k] !== $p;
            }
            $fallback = $atomic->fallback_params;
            if ($fallback !== null) {
                $f1 = self::stripMixed($fallback[1]);
                $changed = $changed || $f1 !== $fallback[1];
                $fallback = [$fallback[0], $f1];
            }
            return $changed
                ? TKeyedArray::make($props, $atomic->class_strings, $fallback, $atomic->is_list, $atomic->from_docblock)
                : $atomic;
        }
        if ($atomic instanceof TArray) {
            $value = self::stripMixed($atomic->type_params[1]);
            return $value === $atomic->type_params[1] ? $atomic : $atomic->setTypeParams([$atomic->type_params[0], $value]);
        }
        return $atomic;
    }

    /** @var array<string, true>|null lowercase names of the project's functions */
    private static ?array $function_names = null;

    /**
     * Literal strings that may be callables are kept (they are resolved at dynamic calls); other literals are
     * generalized.
     */
    private static function isCallableName(string $value): bool
    {
        if (\str_contains($value, '::')) {
            return true;
        }
        if (!\preg_match('/^\\\\?[A-Za-z_][A-Za-z0-9_\\\\]*$/', $value)) {
            return false;
        }
        $lc = strtolower(\ltrim($value, '\\'));
        if (self::$function_names === null) {
            self::$function_names = [];
            if (self::$instance !== null) {
                foreach (self::$instance->codebase->file_storage_provider->getAll() as $file_storage) {
                    foreach ($file_storage->functions as $fid => $_) {
                        self::$function_names[strtolower((string) $fid)] = true;
                    }
                }
            }
        }
        return isset(self::$function_names[$lc]) || (!\str_contains($lc, '\\') && \function_exists($lc));
    }

    private static function generalizeAtomic(Atomic $atomic, bool $strings): Atomic
    {
        if ($atomic instanceof TLiteralInt || $atomic instanceof TIntRange) {
            return new TInt();
        }
        if ($atomic instanceof TLiteralFloat) {
            return new TFloat();
        }
        if ($atomic instanceof TLiteralString && !$atomic instanceof Atomic\TLiteralClassString
            && ($strings || !self::isCallableName($atomic->value))
        ) {
            if (is_numeric($atomic->value)) {
                return new TNumericString();
            }
            return $atomic->value === '' ? new TString() : new TNonEmptyString();
        }
        if ($atomic instanceof TKeyedArray) {
            $props = [];
            $changed = false;
            foreach ($atomic->properties as $k => $p) {
                $props[$k] = self::generalize($p, $strings);
                $changed = $changed || $props[$k] !== $p;
            }
            $fallback = $atomic->fallback_params;
            if ($fallback !== null) {
                $f0 = self::generalize($fallback[0], $strings);
                $f1 = self::generalize($fallback[1], $strings);
                $changed = $changed || $f0 !== $fallback[0] || $f1 !== $fallback[1];
                $fallback = [$f0, $f1];
            }
            if (!$changed) {
                return $atomic;
            }
            return TKeyedArray::make($props, $atomic->class_strings, $fallback, $atomic->is_list, $atomic->from_docblock);
        }
        if ($atomic instanceof TArray || $atomic instanceof TIterable || $atomic instanceof TGenericObject) {
            $params = [];
            $changed = false;
            foreach ($atomic->type_params as $k => $p) {
                $params[$k] = self::generalize($p, $strings);
                $changed = $changed || $params[$k] !== $p;
            }
            return $changed ? $atomic->setTypeParams($params) : $atomic;
        }
        return $atomic;
    }
}
