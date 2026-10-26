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

    /** @var array<string, ClosedWorldContributions> contributions by the root file whose analysis made them */
    private array $by_file = [];

    /** the contributions of the file being recorded */
    private ClosedWorldContributions $cur;

    /** all contributions, merged at the end of a pass */
    private ClosedWorldContributions $all;

    /**
     * Which root files read which slots (`call:<storage id>` for returns and by-reference outs, `P:` properties,
     * `G:` globals and statics): re-analyzed when the slot changes. Grows across passes.
     *
     * @var array<string, array<string, true>>
     */
    private array $consumers = [];

    /** @var array<int, array<string, true>> root files in which a function-like's body was analyzed */
    private array $body_roots = [];

    /** Whether some property type was read without a known file (then every file reads every property). */
    private bool $unattributed_property_reads = false;

    /** @var array<string, true> slot keys changed by the last apply() */
    private array $changed_keys = [];

    /** Whether the last apply() or simplification changes what every file may see (dynamic properties). */
    private bool $everything_dirty = false;

    /** @var array<string, true> files whose statements the last simplification rewrote */
    private array $rewritten_files = [];






    /** @var array<string, Union> current types of global and static variables, by slot key */
    private array $global_types = [];

    /** @var array<string, true> globals written through `$GLOBALS[...]` (`*`: an unknown one): not inferred */
    private array $escaped_globals = [];








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
        $this->cur = new ClosedWorldContributions();
        $this->all = new ClosedWorldContributions();
    }

    private function at(string $file): ClosedWorldContributions
    {
        return $this->by_file[$file] ??= new ClosedWorldContributions();
    }

    /** A root file reads a slot (see $consumers); null: a read the analysis cannot attribute. */
    public function consume(string $key, ?string $file): void
    {
        if ($file === null) {
            if (\str_starts_with($key, 'P:')) {
                if (!$this->unattributed_property_reads && getenv('CW_DEBUG')) {
                    $trace = \array_map(static fn(array $f): string => \basename($f['file'] ?? '?') . ':' . ($f['line'] ?? 0), \array_slice(\debug_backtrace(), 1, 4));
                    fwrite(STDERR, "[closed-world] unattributed read of $key from " . \implode(' < ', $trace) . "\n");
                }
                $this->unattributed_property_reads = true;
            }
            return;
        }
        $this->consumers[$key][$file] = true;
    }

    public static function enable(Codebase $codebase): self
    {
        \Psalm\Internal\Type\TypeExpander::resetClosedWorldCache();
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
     * @param list<int>|null $only these arguments only (deferred ones)
     * @return list<int> arguments not analyzed yet (closures): to record once they are
     */
    public function recordCall(
        StatementsAnalyzer $statements_analyzer,
        ?FunctionLikeStorage $storage,
        array $args,
        array $arg_function_params,
        bool $has_packed_var,
        ?array $only = null,
    ): array {
        if ($storage === null || !$this->infers($this->storageFile($storage))) {
            return [];
        }
        $deferred = [];
        if ($only !== null) {
            $id = spl_object_id($storage);
            $this->cur = $this->at($statements_analyzer->getRootFilePath());
            $offsets = [];
            foreach ($storage->params as $i => $param) {
                $offsets[$param->name] = $i;
            }
            foreach ($only as $argument_offset) {
                $arg = $args[$argument_offset];
                $arg_type = $statements_analyzer->node_data->getType($arg->value) ?? Type::getMixed();
                foreach ($arg_function_params[$argument_offset] ?? [] as $param) {
                    if (($i = $offsets[$param->name] ?? null) !== null) {
                        ($this->cur->args[$id][$i] ??= new ClosedWorldSlot())->add($arg_type, false);
                    }
                }
            }
            return [];
        }
        $id = spl_object_id($storage);
        $this->storages[$id] = $storage;
        $file = $statements_analyzer->getRootFilePath();
        $this->cur = $this->at($file);
        $this->consumers['call:' . $id][$file] = true;
        $count = \count($args);
        $has_unpack = $has_packed_var;
        $old = $this->cur->arities[$id] ?? [$count, $count];
        $this->cur->arities[$id] = [\min($old[0], $count), $has_unpack || $old[1] === null ? null : \max($old[1], $count)];
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
            $arg_type = $statements_analyzer->node_data->getType($arg->value);
            if ($arg_type === null && !$arg->unpack
                && ($arg->value instanceof PhpParser\Node\Expr\Closure || $arg->value instanceof PhpParser\Node\Expr\ArrowFunction)
            ) {
                $deferred[] = $argument_offset;
                foreach ($arg_function_params[$argument_offset] as $param) {
                    if (($i = $offsets[$param->name] ?? null) !== null) {
                        $bound[$i] = true;
                    }
                }
                continue;
            }
            $arg_type ??= Type::getMixed();
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
                ($this->cur->args[$id][$i] ??= new ClosedWorldSlot())->add($arg_type, $recursive);
            }
        }

        // parameters the call leaves to their default
        foreach ($storage->params as $i => $param) {
            if (isset($bound[$i]) || $param->is_variadic) {
                continue;
            }
            $default = $this->defaultType($param, $statements_analyzer);
            if ($default !== null) {
                ($this->cur->args[$id][$i] ??= new ClosedWorldSlot())->add($default, $recursive);
            } elseif ($has_packed_var) {
                ($this->cur->args[$id][$i] ??= new ClosedWorldSlot())->add(Type::getMixed(), $recursive);
            }
        }
        return $deferred;
    }

    /**
     * A method call on a receiver whose type is unknown: in a closed world it may reach any method of that name.
     *
     * @param list<PhpParser\Node\Arg> $args
     */
    public function recordUnknownMethodCall(StatementsAnalyzer $statements_analyzer, string $method_name, array $args): void
    {
        $this->cur = $this->at($statements_analyzer->getRootFilePath());
        $name = strtolower($method_name);
        foreach ($args as $offset => $arg) {
            $type = $statements_analyzer->node_data->getType($arg->value) ?? Type::getMixed();
            if ($arg->unpack || $arg->name !== null) {
                $type = Type::getMixed();
            }
            ($this->cur->unknown_method_args[$name][$offset] ??= new ClosedWorldSlot())->add($type, false);
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
        $root = $statements_analyzer->getRootFilePath();
        $this->cur = $this->at($root);
        $this->body_roots[$id][$root] = true;
        $this->cur->analyzed[$id] = true;
        if (getenv('CW_LOCALS')) {
            // diagnostics: the types each local had (what the transpiler picks the local's storage from)
            foreach ($var_types as $var_id => $types) {
                if ($types === [] || !\preg_match('/^\$[A-Za-z_\x80-\xff][A-Za-z0-9_\x80-\xff]*$/D', $var_id)) {
                    continue;
                }
                $all = Type::combineUnionTypeArray($types, $this->codebase);
                $this->cur->locals[$id][$var_id] = isset($this->cur->locals[$id][$var_id])
                    ? Type::combineUnionTypes($this->cur->locals[$id][$var_id], $all, $this->codebase)
                    : $all;
            }
        }
        foreach ($storage->params as $i => $param) {
            if (!$param->by_ref) {
                continue;
            }
            $slot = $this->cur->outs[$id][$i] ??= new ClosedWorldSlot();
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
            $this->cur->always_exits[$id] = true;
            return;
        }
        $slot = $this->cur->returns[$id] ??= new ClosedWorldSlot();
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
    public function recordPropertyWrite(string $file, string $property_id, Union $type): void
    {
        $this->cur = $this->at($file);
        [$class, $name] = self::splitPropertyId($property_id);
        $declaring = $this->codebase->properties->getDeclaringClassForProperty($property_id, true);
        if ($declaring !== null) {
            $class = $declaring;
        }
        // the write is checked against the property's type
        $this->consumers['P:' . strtolower($class) . '::' . $name][$file] = true;
        ($this->cur->props[strtolower($class) . '::' . $name] ??= new ClosedWorldSlot())->add($type, false);
    }

    /** A write of a value to a property the class does not declare (PHP creates a dynamic property). */
    public function recordDynamicPropertyWrite(string $file, string $fq_class_name, string $name, Union $type): void
    {
        $this->cur = $this->at($file);
        $class_lc = strtolower($fq_class_name);
        if (!$this->codebase->classlike_storage_provider->has($class_lc)) {
            return;
        }
        $class_storage = $this->codebase->classlike_storage_provider->get($class_lc);
        if (!$this->infers($class_storage->location?->file_path)) {
            return;
        }
        ($this->cur->dynamic_props[$class_lc][$name] ??= new ClosedWorldSlot())->add($type, false);
    }

    /**
     * `global $x` / `static $x`: the local is bound to the slot; its type is the slot's (null: not inferred).
     */
    public function bindGlobal(\Psalm\Context $context, string $var_id, string $key, ?string $file = null): ?Union
    {
        $context->closed_world_globals[$var_id] = $key;
        if ($file !== null) {
            $this->consumers['G:' . $key][$file] = true;
        }
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
        $root = $statements_analyzer->getRootFilePath();
        $this->cur = $this->at($root);
        if ($statements_analyzer->getSource() instanceof \Psalm\Internal\Analyzer\FileAnalyzer) {
            foreach ($context->vars_in_scope as $var_id => $_) {
                if (\preg_match('/^\$[A-Za-z_\x80-\xff][A-Za-z0-9_\x80-\xff]*$/', $var_id) && $var_id !== '$this'
                    && !isset($context->closed_world_globals[$var_id])
                ) {
                    $context->closed_world_globals[$var_id] = 'g:' . \substr($var_id, 1);
                    $this->consumers['G:g:' . \substr($var_id, 1)][$root] = true;
                }
            }
        }
        foreach ($context->closed_world_globals as $var_id => $key) {
            $type = $context->vars_in_scope[$var_id] ?? null;
            if ($type === null || $type === ($this->global_types[$key] ?? null)) {
                continue;
            }
            ($this->cur->global_writes[$key] ??= new ClosedWorldSlot())->add($type, false);
        }
    }

    /** The initial value of a static variable. */
    public function recordStaticDefault(string $file, string $key, Union $type): void
    {
        $this->cur = $this->at($file);
        ($this->cur->global_writes[$key] ??= new ClosedWorldSlot())->add($type, false);
    }

    private function applyGlobals(string $mode): int
    {
        $changed = 0;
        $keys = \array_unique([...array_keys($this->all->global_writes), ...array_keys($this->global_types)]);
        \sort($keys);
        foreach ($keys as $key) {
            $slot = $this->all->global_writes[$key] ?? new ClosedWorldSlot();
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
            $this->recordAnyPropertyWrite($statements_analyzer->getRootFilePath(), '*', $type);
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
                $this->recordPropertyWrite($statements_analyzer->getRootFilePath(), $class . '::$' . $name, $type);
                return;
            }
        }
        if ($receiver_type === null) {
            $this->recordUnknownPropertyWrite($statements_analyzer->getRootFilePath(), $name, $type);
            return;
        }
        foreach ($receiver_type->getAtomicTypes() as $atomic) {
            if ($atomic instanceof Atomic\TNamedObject
                && $this->codebase->propertyExists($atomic->value . '::$' . $name, false)
            ) {
                $this->recordPropertyWrite($statements_analyzer->getRootFilePath(), $atomic->value . '::$' . $name, $type);
            } elseif ($atomic instanceof Atomic\TNamedObject) {
                $this->recordDynamicPropertyWrite($statements_analyzer->getRootFilePath(), $atomic->value, $name, $type);
            } elseif (!$atomic instanceof Atomic\TNull) {
                $this->recordUnknownPropertyWrite($statements_analyzer->getRootFilePath(), $name, $type);
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

    /**
     * Whether reading `$array[$key]` may find no such key (PHP then gives null): any key of a generic array, a key
     * of a shape that is not one of its definitely present keys.
     */
    public static function offsetMayBeMissing(Union $array, Union $key): bool
    {
        foreach ($array->getAtomicTypes() as $atomic) {
            if ($atomic instanceof TArray) {
                return true;
            }
            if (!$atomic instanceof TKeyedArray) {
                continue;
            }
            foreach ($key->getAtomicTypes() as $k) {
                if ($k instanceof TLiteralInt || $k instanceof TLiteralString) {
                    $prop = $atomic->properties[$k->value] ?? null;
                    if ($prop === null || $prop->possibly_undefined) {
                        return true;
                    }
                    continue;
                }
                if ($atomic->fallback_params !== null) {
                    return true;
                }
                // a non-literal key of a sealed shape: present only if it is one of the definite keys
                return true;
            }
        }
        return false;
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
    public function recordUnknownPropertyWrite(string $file, string $name, Union $type): void
    {
        $this->cur = $this->at($file);
        ($this->cur->unknown_prop_writes[$name] ??= new ClosedWorldSlot())->add($type, false);
    }

    /** `$o->$name = v` with a name of unknown value: any property of the class may be written. */
    public function recordAnyPropertyWrite(string $file, string $fq_class_name, Union $type): void
    {
        $this->cur = $this->at($file);
        ($this->cur->any_prop_writes[strtolower($fq_class_name)] ??= new ClosedWorldSlot())->add($type, false);
    }

    /** @var array<string, true> lowercase names used in first-class callables (`f(...)`, `$o->m(...)`) */
    private array $fcc_names = [];

    /** Whether the program builds closures from callable values (`Closure::fromCallable`). */
    private bool $from_callable_used = false;

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
                || ($n instanceof PhpParser\Node\Expr\StaticCall && $n->name instanceof PhpParser\Node\Identifier
                    && strtolower($n->name->name) === 'fromcallable')
                || ($n instanceof PhpParser\Node\Expr\ArrayDimFetch && $n->var instanceof PhpParser\Node\Expr\Variable
                    && $n->var->name === 'GLOBALS')) as $node
            ) {
                if ($node instanceof PhpParser\Node\Expr\StaticCall && !$node->isFirstClassCallable()) {
                    $this->from_callable_used = true;
                    continue;
                }
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
                    $this->fcc_names[strtolower(\is_string($resolved) ? $resolved : $node->name->toString())] = true;
                    $this->fcc_names[strtolower($node->name->getLast())] = true;
                } elseif (($node instanceof PhpParser\Node\Expr\MethodCall
                        || $node instanceof PhpParser\Node\Expr\NullsafeMethodCall
                        || $node instanceof PhpParser\Node\Expr\StaticCall)
                    && $node->name instanceof PhpParser\Node\Identifier
                ) {
                    $this->strings[strtolower($node->name->name)] = true;
                    $this->fcc_names[strtolower($node->name->name)] = true;
                }
            }
        }
    }



    /**
     * The arguments of a call as the dynamic-call recording takes them: types by position, and the type of the
     * further ones when an argument is unpacked.
     *
     * @param array<PhpParser\Node\Arg> $args
     * @return array{list<Union>, ?Union}
     */
    public static function argTypes(StatementsAnalyzer $statements_analyzer, array $args): array
    {
        $types = [];
        $spread = null;
        foreach ($args as $arg) {
            $type = $statements_analyzer->node_data->getType($arg->value) ?? Type::getMixed();
            if (!$arg->unpack) {
                $types[] = $type;
                continue;
            }
            $sealed = $type->isSingle() ? $type->getSingleAtomic() : null;
            if ($sealed instanceof TKeyedArray && $sealed->is_list && $sealed->fallback_params === null) {
                foreach ($sealed->properties as $element) {
                    if ($element->possibly_undefined) {
                        $spread = $spread === null ? $element : Type::combineUnionTypes($spread, $element);
                    } else {
                        $types[] = $element;
                    }
                }
                continue;
            }
            $element = self::iterableValueType($type);
            $spread = $spread === null ? $element : Type::combineUnionTypes($spread, $element);
        }
        return [$types, $spread];
    }

    /**
     * Callables given to a builtin: it calls them with the parameters its signature gives the callable type
     * (templates resolved by the call), or with unknown arguments when the signature does not say.
     *
     * @param array<PhpParser\Node\Arg> $args
     * @param array<int, list<FunctionLikeParameter>> $arg_function_params
     */
    public function recordBuiltinCallbacks(
        StatementsAnalyzer $statements_analyzer,
        ?string $function_id,
        array $args,
        array $arg_function_params,
        ?\Psalm\Internal\Type\TemplateResult $template_result,
    ): void {
        $trusted = !$this->codebase->config->ignoresDocblockTypesIn($statements_analyzer->getFilePath());
        $root = $statements_analyzer->getRootFilePath();
        $type = static fn(int $i): Union => isset($args[$i])
            ? ($statements_analyzer->node_data->getType($args[$i]->value) ?? Type::getMixed())
            : Type::getMixed();
        $value = static fn(int $i): Union => isset($args[$i]) ? self::iterableValueType($type($i)) : Type::getMixed();
        $key = static function (int $i) use ($type): Union {
            $keys = [];
            foreach ($type($i)->getAtomicTypes() as $atomic) {
                if ($atomic instanceof TKeyedArray) {
                    $keys[] = $atomic->getGenericKeyType();
                } elseif ($atomic instanceof TArray) {
                    $keys[] = $atomic->type_params[0];
                } else {
                    return Type::getArrayKey();
                }
            }
            return $keys ? Type::combineUnionTypeArray($keys, null) : Type::getArrayKey();
        };
        // the builtins that call a callable with the elements of their arrays
        $known = match (strtolower((string) $function_id)) {
            'array_map' => [0, \array_map(static fn(int $i): Union => $value($i), \range(1, \max(1, \count($args) - 1)))],
            'array_filter' => [1, isset($args[2]) ? [Type::combineUnionTypes($value(0), $key(0)), $key(0)] : [$value(0)]],
            'usort', 'uasort' => [1, [$value(0), $value(0)]],
            'uksort' => [1, [$key(0), $key(0)]],
            'array_walk', 'array_walk_recursive' => [1, [$value(0), $key(0), $type(2)]],
            'array_reduce' => [1, [Type::getMixed(), $value(0)]],
            'preg_replace_callback' => [1, [Type::parseString('array<array-key, string>')]],
            'iterator_apply' => [1, []],
            'spl_autoload_register' => [0, [Type::getString()]],
            default => null,
        };
        if ($known !== null) {
            [$offset, $arg_types] = $known;
            if (isset($args[$offset])) {
                $this->recordDynamicCall($root, $type($offset), $arg_types, null, $trusted, \basename($statements_analyzer->getFilePath()) . ':' . ($args[$offset]->getStartLine()));
            }
            return;
        }
        foreach ($args as $offset => $arg) {
            foreach ($arg_function_params[$offset] ?? [] as $param) {
                if ($param->type === null || !self::isCallableParam($param->type)) {
                    continue;
                }
                $expected = $param->type;
                if ($template_result !== null) {
                    try {
                        $expected = \Psalm\Internal\Type\TemplateInferredTypeReplacer::replace(
                            $expected,
                            $template_result,
                            $this->codebase,
                        );
                    } catch (\Throwable) {
                    }
                }
                $arg_types = null;
                foreach ($expected->getAtomicTypes() as $atomic) {
                    if (($atomic instanceof Atomic\TCallable || $atomic instanceof Atomic\TClosure) && $atomic->params !== null) {
                        $types = [];
                        foreach ($atomic->params as $cp) {
                            $t = $cp->type ?? Type::getMixed();
                            $types[] = $t->hasTemplate() ? Type::getMixed() : $t;
                        }
                        $arg_types = $types;
                        break;
                    }
                }
                $callee = $statements_analyzer->node_data->getType($arg->value);
                if ($arg_types === null) {
                    $this->recordDynamicCall($root, $callee, [], Type::getMixed(), $trusted, \basename($statements_analyzer->getFilePath()) . ':' . $arg->getStartLine());
                } else {
                    $this->recordDynamicCall($root, $callee, $arg_types, null, $trusted, \basename($statements_analyzer->getFilePath()) . ':' . $arg->getStartLine());
                }
            }
        }
    }

    /** @var array<string, FunctionLikeStorage>|null project functions and closures by lowercase id */
    private ?array $function_by_id = null;

    /**
     * A dynamic call (`$f(...)`, `call_user_func_array($f, $args)`, a callable given to a builtin that calls it):
     * the callee's type names its targets - literal function names, closures (by their identity), `[$obj,
     * 'method']` arrays - and they receive the arguments. A callee the types do not pin down sends the
     * arguments to every function-like that may be called dynamically.
     *
     * @param list<Union> $arg_types arguments by position
     * @param ?Union $spread type of the further arguments when some were unpacked (or their number is unknown)
     */
    public function recordDynamicCall(
        string $file,
        ?Union $callee,
        array $arg_types,
        ?Union $spread,
        bool $trusted_site = false,
        string $where = '',
    ): void {
        $this->cur = $this->at($file);
        $targets = [];
        $resolved = $callee !== null;
        foreach ($callee?->getAtomicTypes() ?? [] as $atomic) {
            if ($atomic instanceof Atomic\TNull || $atomic instanceof Atomic\TFalse || $atomic instanceof Atomic\TNever) {
                // (a never callee - optimistically not inferred yet - calls nothing)
                continue;
            }
            $found = $this->callableTargets($atomic);
            if ($found === null) {
                $resolved = false;
                continue;
            }
            foreach ($found as $storage) {
                $targets[spl_object_id($storage)] = $storage;
            }
        }
        foreach ($targets as $id => $storage) {
            $this->storages[$id] = $storage;
            $this->consumers['call:' . $id][$file] = true;
            foreach ($storage->params as $i => $param) {
                $slot = $this->cur->args[$id][$i] ??= new ClosedWorldSlot();
                if ($param->is_variadic) {
                    foreach (\array_slice($arg_types, $i) as $t) {
                        $slot->add($t, false);
                    }
                    if ($spread !== null) {
                        $slot->add($spread, false);
                    }
                    break;
                }
                if (isset($arg_types[$i])) {
                    $slot->add($arg_types[$i], false);
                } elseif ($spread !== null) {
                    $slot->add($spread, false);
                }
                if (!isset($arg_types[$i]) && $param->default_type instanceof Union) {
                    $slot->add($param->default_type, false);
                }
            }
        }
        if (getenv('CW_DEBUG')) {
            fwrite(STDERR, '[closed-world] ' . ($resolved ? 'resolved (' . \count($targets) . ')' : 'unresolved') . ' dynamic call of ' . ($callee?->getId() ?? '?')
                . ($trusted_site ? ' (trusted)' : '') . ' with (' . \implode(', ', \array_map(static fn(Union $t): string => $t->getId(), $arg_types))
                . ($spread !== null ? ', ...' . $spread->getId() : '') . ") $where\n");
        }
        if (!$resolved && !$trusted_site) {
            // (a call inside trusted code reaches only callables given to it through its typed callable
            // parameters: those were recorded with the parameter's signature where they were given)
            $this->cur->unknown_callables = true;
            // a callee typed as a Closure object can only be a closure or a first-class callable
            $closure_only = $callee !== null && $callee->getAtomicTypes() !== [];
            foreach ($callee?->getAtomicTypes() ?? [] as $atomic) {
                if (!$atomic instanceof Atomic\TClosure && !$atomic instanceof Atomic\TNull && !$atomic instanceof Atomic\TNever) {
                    $closure_only = false;
                }
            }
            $argc = \count($arg_types);
            $key = ($closure_only ? 'c' : 'a') . $argc . ($spread !== null ? '+' : '');
            if (!isset($this->cur->dynamic_sites[$key])) {
                $this->cur->dynamic_sites[$key] = [
                    $closure_only,
                    $argc,
                    \array_map(static fn(): ClosedWorldSlot => new ClosedWorldSlot(), $arg_types),
                    $spread !== null ? new ClosedWorldSlot() : null,
                ];
            }
            foreach ($arg_types as $i => $t) {
                $this->cur->dynamic_sites[$key][2][$i]->add($t, false);
            }
            if ($spread !== null) {
                $this->cur->dynamic_sites[$key][3]?->add($spread, false);
            }
        }
    }

    /**
     * The project function-likes a callable value may call; null when the value does not tell (any of them).
     *
     * @return list<FunctionLikeStorage>|null
     */
    private function callableTargets(Atomic $atomic): ?array
    {
        if ($atomic instanceof Atomic\TClosure) {
            if ($atomic->callable_id === null) {
                return null;
            }
            $storage = $this->functionById($atomic->callable_id);
            if ($storage !== null) {
                return [$storage];
            }
            // a first-class callable of a method (`$o->m(...)`): its id is the method id
            if (\str_contains($atomic->callable_id, '::')) {
                return $this->methodTargets(...\explode('::', $atomic->callable_id, 2));
            }
            return []; // a builtin
        }
        if ($atomic instanceof TLiteralString && !$atomic instanceof Atomic\TLiteralClassString) {
            $name = \ltrim($atomic->value, '\\');
            if (\str_contains($name, '::')) {
                return $this->methodTargets(...\explode('::', $name, 2));
            }
            $storage = $this->functionById(strtolower($name));
            return $storage !== null ? [$storage] : [];
        }
        if ($atomic instanceof TKeyedArray && \count($atomic->properties) === 2
            && isset($atomic->properties[0], $atomic->properties[1])
            && $atomic->properties[1]->isSingleStringLiteral()
        ) {
            $method = $atomic->properties[1]->getSingleStringLiteral()->value;
            $targets = [];
            foreach ($atomic->properties[0]->getAtomicTypes() as $receiver) {
                if ($receiver instanceof Atomic\TNamedObject) {
                    $class = $receiver->value;
                } elseif ($receiver instanceof Atomic\TLiteralClassString) {
                    $class = $receiver->value;
                } elseif ($receiver instanceof TLiteralString) {
                    $class = $receiver->value;
                } else {
                    return null;
                }
                $found = $this->methodTargets($class, $method);
                if ($found === null) {
                    return null;
                }
                $targets = [...$targets, ...$found];
            }
            return $targets;
        }
        if ($atomic instanceof Atomic\TNamedObject) {
            // an invokable object: its __invoke is magic (keeps its declared parameter types)
            return [];
        }
        return null;
    }

    /** @return list<FunctionLikeStorage>|null */
    private function methodTargets(string $class, string $method): ?array
    {
        $class = \ltrim($class, '\\');
        if (!$this->codebase->classlike_storage_provider->has($class)) {
            return null;
        }
        $class_storage = $this->codebase->classlike_storage_provider->get($class);
        $declaring = $class_storage->declaring_method_ids[strtolower($method)] ?? null;
        if ($declaring === null) {
            // `__call` / `__callStatic` handle it: magic
            return [];
        }
        try {
            return [$this->codebase->methods->getStorage($declaring)];
        } catch (\Throwable) {
            return null;
        }
    }

    private function functionById(string $id): ?FunctionLikeStorage
    {
        if ($this->function_by_id === null) {
            $this->function_by_id = [];
            foreach ($this->codebase->file_storage_provider->getAll() as $file_storage) {
                if (!$this->codebase->config->isInProjectDirs($file_storage->file_path)) {
                    continue;
                }
                foreach ($file_storage->functions as $fid => $storage) {
                    $this->function_by_id[strtolower((string) $fid)] = $storage;
                }
            }
        }
        return $this->function_by_id[$id] ?? null;
    }

    /**
     * An instantiation whose class may be a subclass of the one the types name (`new static`, `new $class`), or
     * any class (null): every constructor it may run receives the arguments.
     *
     * @param list<Union> $arg_types
     */
    public function recordPolymorphicNew(string $file, ?string $fq_class_name, array $arg_types, ?Union $spread): void
    {
        $this->cur = $this->at($file);
        $key = $fq_class_name === null ? '*' : strtolower($fq_class_name);
        foreach ($arg_types as $i => $t) {
            ($this->cur->polymorphic_new[$key][$i] ??= new ClosedWorldSlot())->add($t, false);
        }
        if ($spread !== null) {
            ($this->cur->polymorphic_new[$key][-1] ??= new ClosedWorldSlot())->add($spread, false);
        }
        $this->cur->polymorphic_new[$key] ??= [];
    }

    /** The arguments of polymorphic instantiations, given to every constructor they may run. */
    private function expandPolymorphicNew(): void
    {
        foreach ($this->all->polymorphic_new as $key => $slots) {
            foreach ($this->codebase->classlike_storage_provider->getAll() as $class_lc => $class_storage) {
                if (!$this->infers($class_storage->location?->file_path)
                    || ($key !== '*' && $class_lc !== $key && !isset($class_storage->parent_classes[$key]))
                ) {
                    continue;
                }
                $declaring = $class_storage->declaring_method_ids['__construct'] ?? null;
                if ($declaring === null) {
                    continue;
                }
                try {
                    $ctor = $this->codebase->methods->getStorage($declaring);
                } catch (\Throwable) {
                    continue;
                }
                $id = spl_object_id($ctor);
                $this->storages[$id] = $ctor;
                foreach ($ctor->params as $i => $param) {
                    $slot = $this->all->args[$id][$i] ??= new ClosedWorldSlot();
                    if (isset($slots[$i])) {
                        $slot->merge($slots[$i]);
                    } elseif (isset($slots[-1])) {
                        $slot->merge($slots[-1]);
                    }
                    if (!isset($slots[$i]) && $param->default_type instanceof Union) {
                        $slot->add($param->default_type, false);
                    }
                }
            }
        }
    }

    /** A method called by a name the analysis does not know (`$o->$name()`, `[$o, $name]`). */
    public function recordUnknownMethodName(string $file): void
    {
        $this->cur = $this->at($file);
        $this->cur->unknown_callables = true;
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
        if (!$this->all->unknown_callables) {
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

    /**
     * Forget the contributions of the files about to be re-analyzed.
     *
     * @param list<string>|null $files null: all
     */
    public function startPass(?array $files): void
    {
        $this->pass++;
        \Psalm\Internal\Type\TypeExpander::resetClosedWorldCache();
        if ($files === null) {
            $this->by_file = [];
        }
        foreach ($files ?? [] as $file) {
            unset($this->by_file[$file]);
        }
        $this->changed_keys = [];
        $this->everything_dirty = false;
        $this->rewritten_files = [];
    }

    /**
     * The root files whose analysis read a slot that changed (or whose statements were rewritten): the worklist
     * of the next pass. Null: all of them.
     *
     * @return list<string>|null
     */
    public function filesToReanalyze(): ?array
    {
        if ($this->everything_dirty || getenv('CW_FULL')) {
            if (getenv('CW_DEBUG')) {
                fwrite(STDERR, "[closed-world] worklist: everything (dynamic properties or simplifications)\n");
            }
            return null;
        }
        $files = $this->rewritten_files;
        foreach ($this->changed_keys as $key => $_) {
            $parts = \explode(':', $key, 3);
            if ($parts[0] === 'p' || $parts[0] === 'r' || $parts[0] === 'o') {
                $id = (int) $parts[1];
                $files += $this->body_roots[$id] ?? [];
                if ($parts[0] !== 'p') {
                    $files += $this->consumers['call:' . $id] ?? [];
                }
                if (!isset($this->body_roots[$id]) && ($f = $this->storages[$id]->location?->file_path ?? null) !== null) {
                    $files[$f] = true;
                }
            } elseif ($parts[0] === 'P') {
                if ($this->unattributed_property_reads) {
                    if (getenv('CW_DEBUG')) {
                        fwrite(STDERR, "[closed-world] worklist: everything (a property read of unknown origin)\n");
                    }
                    return null;
                }
                $files += $this->consumers[$key] ?? [];
                [$class] = \explode('::', \substr($key, 2), 2);
                if ($this->codebase->classlike_storage_provider->has($class)
                    && ($f = $this->codebase->classlike_storage_provider->get($class)->location?->file_path) !== null
                ) {
                    $files[$f] = true;
                }
            } elseif ($parts[0] === 'G') {
                $files += $this->consumers[\substr($key, 2)] ?? [];
                $files += $this->consumers[$key] ?? [];
            }
        }
        $files = \array_keys($files);
        \sort($files);
        return $files;
    }

    /**
     * Write the contributions of the pass that just ended into the storages.
     *
     * @return int number of slots whose value changed (0: fixpoint reached)
     */
    public function apply(): int
    {
        $t0 = \microtime(true);
        // merged in file order: unions are combined the same way whichever files a pass re-analyzed
        \ksort($this->by_file);
        $this->all = ClosedWorldContributions::merge($this->by_file);
        $this->collectProjectStorages();
        $this->expandPolymorphicNew();
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
        if (getenv('CW_DEBUG')) {
            fwrite(STDERR, \sprintf("[closed-world] apply took %.2fs\n", \microtime(true) - $t0));
        }
        return $changed;
    }

    private function applyAll(string $mode): int
    {
        $changed = 0;

        // --- parameters and returns, grouped over method overrides -------------------------------------------
        [$param_groups, $return_groups] = $this->methodGroups();
        $this->param_groups = $param_groups;
        foreach ($this->storages as $id => $storage) {
            if (!$this->infers($this->storageFile($storage))) {
                // trusted code (runtime stubs) keeps its declared types
                continue;
            }
            $group = $param_groups[$id] ?? [$id];
            $return_group = $return_groups[$id] ?? [$id];
            $escaped = $this->isEscaped($storage);
            $dynamic = !$escaped && $this->isDynamicTarget($storage);
            foreach ($storage->params as $i => $param) {
                $key = 'p:' . $id . ':' . $i;
                $this->declared_params[$key] ??= [$param, $param->signature_type];
                $declared = $this->declared_params[$key][1];
                if ($escaped) {
                    $new = $declared;
                } else {
                    $slot = new ClosedWorldSlot();
                    foreach ($group as $member) {
                        if (isset($this->all->args[$member][$i])) {
                            $slot->merge($this->all->args[$member][$i]);
                        }
                    }
                    if ($dynamic) {
                        $this->addDynamicSites($slot, $storage, $i, $param);
                    }
                    if ($storage instanceof MethodStorage && $storage->cased_name !== null
                        && isset($this->all->unknown_method_args[strtolower($storage->cased_name)][$i])
                    ) {
                        $slot->merge($this->all->unknown_method_args[strtolower($storage->cased_name)][$i]);
                    }
                    $new = $slot->value($mode, $declared) ?? $declared;
                    if ($new !== null && $declared !== null && !$this->refines($new, $declared) && getenv('CW_DEBUG')) {
                        fwrite(STDERR, '[closed-world] not a refinement: ' . $new->getId() . ' of ' . $declared->getId() . ' for ' . $this->describe($key) . "\n");
                    }
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
                        foreach ($return_group as $member) {
                            if (isset($this->all->outs[$member][$i])) {
                                $slot->merge($this->all->outs[$member][$i]);
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
            foreach ($return_group as $member) {
                if (isset($this->all->returns[$member])) {
                    $slot->merge($this->all->returns[$member]);
                    $any = true;
                    $always_exits = false;
                } elseif (!isset($this->all->always_exits[$member])) {
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
            if ($changed === 0 && getenv('CW_LOCALS')) {
                foreach ($this->all->locals as $id => $vars) {
                    if (!isset($this->storages[$id])) {
                        continue;
                    }
                    $name = \substr($this->describe('r:' . $id), 0, -\strlen('(): return'));
                    foreach ($vars as $var_id => $type) {
                        fwrite(STDERR, '  L:' . $name . ' ' . $var_id . ' = ' . $type->getId() . "\n");
                    }
                }
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
                . ($storage->cased_name ?? ('{closure@' . \basename((string) $storage->location?->file_path) . ':'
                    . ($storage->location?->getLineNumber() ?? 0) . '}'));
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
        $this->changed_keys[$key] = true;
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
     * Whether a function-like may be called from places the analysis cannot attribute arguments from: magic
     * methods (called by the engine). Its parameters keep their declared types.
     */
    private function isEscaped(FunctionLikeStorage $storage): bool
    {
        if ($storage->location === null) {
            return true;
        }
        if (self::isClosure($storage)) {
            return false;
        }
        if ($storage instanceof MethodStorage) {
            $lc = strtolower($storage->cased_name);
            return \str_starts_with($lc, '__') && $lc !== '__construct';
        }
        return false;
    }

    /**
     * The arguments unresolved dynamic calls may pass to this parameter of a dynamic target: those of the calls
     * that can reach it (a Closure-typed callee only reaches closures and first-class callables; a call passing
     * fewer arguments than the target requires is an error, not a call of it).
     */
    private function addDynamicSites(
        ClosedWorldSlot $slot,
        FunctionLikeStorage $storage,
        int $i,
        FunctionLikeParameter $param,
    ): void {
        $required = 0;
        foreach ($storage->params as $j => $p) {
            if (!$p->is_optional && !$p->is_variadic) {
                $required = $j + 1;
            }
        }
        $closure_like = self::isClosure($storage) || $this->isFirstClassCallableTarget($storage);
        foreach ($this->all->dynamic_sites as [$closure_only, $argc, $slots, $spread]) {
            if (($closure_only && !$closure_like) || ($spread === null && $argc < $required)) {
                continue;
            }
            if ($param->is_variadic) {
                foreach ($slots as $j => $s) {
                    if ($j >= $i) {
                        $slot->merge($s);
                    }
                }
            } elseif (isset($slots[$i])) {
                $slot->merge($slots[$i]);
            } elseif ($spread === null && $param->default_type instanceof Union) {
                $slot->add($param->default_type, false);
            }
            if ($spread !== null) {
                $slot->merge($spread);
                if ($param->default_type instanceof Union) {
                    $slot->add($param->default_type, false);
                }
            }
        }
    }

    /** Whether a function or method is named by a first-class callable (`f(...)`, `$o->m(...)`, `Closure::fromCallable`). */
    private function isFirstClassCallableTarget(FunctionLikeStorage $storage): bool
    {
        $name = strtolower((string) $storage->cased_name);
        if ($this->from_callable_used) {
            return true;
        }
        if ($storage instanceof MethodStorage) {
            return isset($this->fcc_names[$name]);
        }
        return isset($this->fcc_names[$name]) || isset($this->fcc_names[\substr((string) \strrchr('\\' . $name, '\\'), 1)]);
    }

    private static function isClosure(FunctionLikeStorage $storage): bool
    {
        return !$storage instanceof MethodStorage
            && ($storage->cased_name === null || \str_ends_with($storage->cased_name, ':-:closure'));
    }

    /**
     * Whether a function-like may be the target of a dynamic call the types do not resolve: a closure, a function
     * or method the program names in a string (see nameMayBeBuilt). It receives the arguments of such calls.
     */
    private function isDynamicTarget(FunctionLikeStorage $storage): bool
    {
        $name = (string) $storage->cased_name;
        if ($storage instanceof MethodStorage) {
            $lc = strtolower($name);
            // `[$obj, 'name']`, `'Class::name'`, `[Class::class, 'name']`
            return $this->nameMayBeBuilt($lc)
                || $this->nameMayBeBuilt(strtolower((string) $storage->defining_fqcln) . '::' . $lc);
        }
        if (self::isClosure($storage)) {
            return true;
        }
        $fq = strtolower($name);
        if ($storage->location !== null) {
            foreach ($this->codebase->file_storage_provider->get($storage->location->file_path)->functions as $fid => $fs) {
                if ($fs === $storage) {
                    $fq = (string) $fid;
                    break;
                }
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
     * Methods whose calls may dispatch to one another, by direction. A call the analysis resolves to a method
     * (by the receiver's type) runs that method or an override of it in a subclass: an override receives the
     * arguments of calls of the methods it overrides, and a method returns (and leaves in its by-reference
     * parameters) what any of its overrides may. Storage id => ids of the methods it overrides, itself included
     * (`[0]`: parameter contributions), and => ids of the methods overriding it, itself included (`[1]`: return
     * and by-reference out contributions).
     *
     * @return array{array<int, list<int>>, array<int, list<int>>}
     */
    private function methodGroups(): array
    {
        /** @var array<int, array<int, true>> $overrides storage id => ids of the methods it directly overrides */
        $overrides = [];
        $storage_id = function (\Psalm\Internal\MethodIdentifier $method_id): ?int {
            try {
                $id = spl_object_id($this->codebase->methods->getStorage($method_id));
            } catch (\Throwable) {
                return null;
            }
            return isset($this->storages[$id]) ? $id : null;
        };
        // from every class: the method it has under a name (its own or inherited) overrides the ones its
        // ancestors and interfaces declare under that name (an inherited method may implement an interface of
        // the subclass only)
        foreach ($this->codebase->classlike_storage_provider->getAll() as $class_storage) {
            foreach ($class_storage->overridden_method_ids as $lc => $overridden_ids) {
                if ($lc === '__construct') {
                    continue;
                }
                $declaring = $class_storage->declaring_method_ids[$lc] ?? null;
                if ($declaring === null || ($id = $storage_id($declaring)) === null) {
                    continue;
                }
                $storage = $this->storages[$id];
                if (!$storage instanceof MethodStorage || $storage->cased_name === null) {
                    continue;
                }
                foreach ($overridden_ids as $overridden) {
                    $oid = $storage_id($overridden);
                    if ($oid !== null && $oid !== $id) {
                        $overrides[$id][$oid] = true;
                    }
                }
            }
        }
        $overridden_by = [];
        foreach ($overrides as $id => $targets) {
            foreach ($targets as $oid => $_) {
                $overridden_by[$oid][$id] = true;
            }
        }
        $closure = static function (array $edges): array {
            $out = [];
            foreach ($edges as $start => $_) {
                $seen = [$start => true];
                $stack = [$start];
                while ($stack !== []) {
                    $x = \array_pop($stack);
                    foreach ($edges[$x] ?? [] as $y => $_2) {
                        if (!isset($seen[$y])) {
                            $seen[$y] = true;
                            $stack[] = $y;
                        }
                    }
                }
                $list = array_keys($seen);
                \sort($list);
                $out[$start] = $list;
            }
            return $out;
        };
        return [$closure($overrides), $closure($overridden_by)];
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
                if (isset($this->all->props[strtolower($class_storage->name) . '::' . $name])) {
                    $slot->merge($this->all->props[strtolower($class_storage->name) . '::' . $name]);
                }
                if (isset($this->all->unknown_prop_writes[$name])) {
                    $slot->merge($this->all->unknown_prop_writes[$name]);
                }
                foreach ($this->all->any_prop_writes as $writer_class => $any) {
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
        foreach ($this->all->dynamic_props as $class_lc => $names) {
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
                // every access to the class may now resolve differently
                $this->everything_dirty = true;
                $changed++;
                // the write is now a declared write: account it for the property slot of this pass
                ($this->all->props[$class_lc . '::' . $name] ??= new ClosedWorldSlot())->merge($_slot);
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
        $this->cur = $this->at($statements_analyzer->getRootFilePath());
        $reachable = true;
        foreach ($asserted_var_ids as $var_id => $_) {
            if (isset($context->vars_in_scope[$var_id]) && $context->vars_in_scope[$var_id]->isNever()) {
                $reachable = false;
                break;
            }
        }
        $pos = (int) $branch->getAttribute('startFilePos');
        $this->cur->branches[$file][$pos] = ($this->cur->branches[$file][$pos] ?? false) || $reachable;
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
        foreach ($this->all->branches as $file => $branches) {
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

    /** Builtins that only call the callables they are given. */
    private const CALLBACK_BUILTINS = [
        'array_map' => true, 'array_filter' => true, 'usort' => true, 'uasort' => true, 'uksort' => true,
        'array_walk' => true, 'array_walk_recursive' => true, 'array_reduce' => true, 'preg_replace_callback' => true,
        'iterator_apply' => true, 'call_user_func' => true, 'call_user_func_array' => true,
    ];


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
                && !$this->isDynamicTarget($storage)
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
                } elseif ($node instanceof PhpParser\Node\Expr\FuncCall && $node->name instanceof PhpParser\Node\Name
                    && isset(self::CALLBACK_BUILTINS[strtolower($node->name->getLast())])
                ) {
                    // `usort($a, [$obj, 'cmp'])`: the array only names a method for the builtin to call
                    foreach ($node->getArgs() as $arg) {
                        $item = $arg->value;
                        $apos = (int) $item->getAttribute('startFilePos');
                        if (!$item instanceof PhpParser\Node\Expr\Array_ || \count($item->items) !== 2
                            || isset($this->rewrites[$file][$apos])
                            || !$item->items[1] instanceof PhpParser\Node\ArrayItem
                            || !$item->items[1]->value instanceof PhpParser\Node\Scalar\String_
                            || !$item->items[0] instanceof PhpParser\Node\ArrayItem
                            || $item->items[0]->key !== null || $item->items[1]->key !== null
                        ) {
                            continue;
                        }
                        $receiver = $record->node_data->getType($item->items[0]->value);
                        if ($receiver === null || !$receiver->isSingle()) {
                            continue;
                        }
                        $r = $receiver->getSingleAtomic();
                        if ($r instanceof Atomic\TNamedObject) {
                            $this->rewrites[$file][$apos] = ['method_fcc', $item->items[1]->value->value];
                            $added++;
                        } elseif ($r instanceof TLiteralString) {
                            $this->rewrites[$file][$apos] = ['static_fcc', [$r->value, $item->items[1]->value->value]];
                            $added++;
                        }
                    }
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
        if ($n + $added > 0) {
            $this->everything_dirty = true;
        }
        return $n + $added;
    }

    /** @var array<int, list<int>> method storage id => ids of the methods it overrides, itself included */
    private array $param_groups = [];

    /** Whether every call of the function-like (or of the methods it overrides) passes exactly its parameters. */
    private function hasExactArity(FunctionLikeStorage $storage): bool
    {
        $n = 0;
        foreach ($storage->params as $param) {
            if ($param->is_variadic) {
                return false;
            }
            $n++;
        }
        // calls of the methods it overrides may run it too
        $any = false;
        foreach ($this->param_groups[spl_object_id($storage)] ?? [spl_object_id($storage)] as $member) {
            $arity = $this->all->arities[$member] ?? null;
            if ($arity === null) {
                continue;
            }
            if ($arity[0] !== $n || $arity[1] !== $n) {
                return false;
            }
            $any = true;
        }
        return $any
            && !($storage instanceof MethodStorage && isset($this->all->unknown_method_args[strtolower((string) $storage->cased_name)]));
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
     * Arrays nested this deep in a slot's type are widened to arrays of anything (k-limiting): the values a
     * program builds recursively (a parser's stack of nodes and arrays of nodes and arrays...) would otherwise
     * gain one level of nesting per pass and never reach a fixpoint.
     */
    private const MAX_ARRAY_DEPTH = 3;

    /**
     * Drop the refinements that make iteration slow or endless: literal ints and floats, int ranges, arrays
     * nested deeper than MAX_ARRAY_DEPTH (and literal strings too, when widening). The parent data-flow nodes are
     * dropped as well.
     *
     * @param int $depth number of arrays enclosing the type
     */
    public static function generalize(Union $type, bool $strings = false, int $depth = 0): Union
    {
        $atomics = [];
        $changed = false;
        foreach ($type->getAtomicTypes() as $atomic) {
            $new = self::generalizeAtomic($atomic, $strings, $depth);
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

    private static function generalizeAtomic(Atomic $atomic, bool $strings, int $depth): Atomic
    {
        if (($atomic instanceof TKeyedArray || $atomic instanceof TArray) && $depth >= self::MAX_ARRAY_DEPTH) {
            return $atomic instanceof TKeyedArray && $atomic->is_list ? Type::getListAtomic(Type::getMixed()) : new TArray([Type::getArrayKey(), Type::getMixed()]);
        }
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
                $props[$k] = self::generalize($p, $strings, $depth + 1);
                $changed = $changed || $props[$k] !== $p;
            }
            $fallback = $atomic->fallback_params;
            if ($fallback !== null) {
                $f0 = self::generalize($fallback[0], $strings, $depth + 1);
                $f1 = self::generalize($fallback[1], $strings, $depth + 1);
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
                $params[$k] = self::generalize($p, $strings, $depth + ($atomic instanceof TArray ? 1 : 0));
                $changed = $changed || $params[$k] !== $p;
            }
            return $changed ? $atomic->setTypeParams($params) : $atomic;
        }
        return $atomic;
    }
}
