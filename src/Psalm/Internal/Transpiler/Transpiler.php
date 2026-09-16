<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Expr\ArrowFunction;
use PhpParser\Node\Expr\Closure;
use PhpParser\Node\Stmt;
use PhpParser\Node\Stmt\ClassLike;
use PhpParser\Node\Stmt\ClassMethod;
use PhpParser\Node\Stmt\Function_;
use Psalm\Codebase;
use Psalm\Config;
use Psalm\Context;
use Psalm\Internal\Analyzer\ClassLikeAnalyzer;
use Psalm\Internal\Analyzer\FunctionLikeAnalyzer;
use Psalm\Internal\Analyzer\StatementsAnalyzer;
use Psalm\Storage\ClassLikeStorage;
use Psalm\Type\Union;
use Psalm\Storage\FunctionLikeStorage;
use RuntimeException;

use function basename;
use function realpath;
use function count;
use function file_get_contents;
use function fwrite;
use function glob;
use function preg_match;
use function preg_split;
use function rtrim;
use function str_replace;
use function str_starts_with;
use function strrpos;
use function substr;
use function trim;
use function is_dir;
use function mkdir;
use function spl_object_id;
use function strtolower;

use const STDERR;

/**
 * Coordinates the PHP -> Rust transpiler: collects analysis results while Psalm
 * analyzes the project, then emits a Rust crate.
 *
 * @internal
 */
final class Transpiler
{
    private static ?Transpiler $instance = null;

    /** @var array<string, FunctionRecord> keyed by spl_object_id of the function node and the class it was analyzed for */
    public array $functions = [];

    /** @var array<lowercase-string, ClassRecord> */
    public array $classes = [];

    /** @var array<int, FunctionRecord> statement analyzer id => currently recorded function */
    /** @var \WeakMap<StatementsAnalyzer, PendingRecord> per-statement snapshots of a body being analyzed, keyed by the live analyzer object (object ids are reused once an analyzer is freed) */
    private \WeakMap $active;

    private function __construct(
        public string $out_dir,
        public Config $config,
        public string $root_dir,
    ) {
        $this->active = new \WeakMap();
    }

    /**
     * Additional crates: files under `prefix` (relative to the root directory) are emitted into crate `dir`,
     * which depends on the main crate and on all crates listed before it.
     *
     * @var list<array{dir: string, prefix: string}>
     */
    public array $splits = [];

    /**
     * @param list<string> $splits `DIR:PREFIX` specs for additional crates
     * @param list<string> $data_globs globs of data files to compile
     */
    public static function enable(string $out_dir, Config $config, string $root_dir, array $splits = [], array $data_globs = []): void
    {
        if (!is_dir($out_dir) && !mkdir($out_dir, 0777, true)) {
            throw new RuntimeException("Could not create transpiler output directory $out_dir");
        }
        $t = new self($out_dir, $config, $root_dir);
        foreach ($splits as $spec) {
            $pos = strrpos($spec, ':');
            if ($pos === false) {
                throw new RuntimeException("--transpile-rust-split expects DIR:PREFIX, got $spec");
            }
            $dir = substr($spec, 0, $pos);
            $prefix = rtrim($root_dir, '/') . '/' . trim(substr($spec, $pos + 1), '/') . '/';
            if (!is_dir($dir) && !mkdir($dir, 0777, true)) {
                throw new RuntimeException("Could not create transpiler output directory $dir");
            }
            $t->splits[] = ['dir' => $dir, 'prefix' => $prefix];
        }
        $t->data_globs = $data_globs;
        foreach ($data_globs as $glob) {
            foreach (glob(str_starts_with($glob, '/') ? $glob : rtrim($root_dir, '/') . '/' . $glob) ?: [] as $path) {
                $real = realpath($path);
                if ($real !== false) {
                    $t->data_files[$real] = true;
                }
            }
        }
        self::$instance = $t;
    }

    /** @var list<string> globs (relative to the root directory) of data files compiled as includable values */
    public array $data_globs = [];

    /** @var array<string, true> absolute paths of the data files (`return [...]` dictionaries), never parsed or analyzed */
    public array $data_files = [];

    /**
     * Whether a file is one of the data files compiled mechanically: Psalm neither scans nor analyzes them
     * (their array literals are huge), the include site's docblock is their type.
     */
    public function isDataFile(string $file_path): bool
    {
        $real = realpath($file_path);
        return $real !== false && isset($this->data_files[$real]);
    }

    /** @var array<string, list<Stmt>> top-level (non-declaration) statements of analyzed project files */
    public array $file_stmts = [];

    /** Records the leftover top-level statements of an analyzed file (the value an `include` of it yields). */
    public function recordFile(string $file_path, array $leftover_stmts): void
    {
        if (!$this->config->isInProjectDirs($file_path)) {
            return;
        }
        $this->file_stmts[$file_path] = $leftover_stmts;
    }

    /** Index of the crate a file is emitted into (0 = the main crate). */
    public function crateOfFile(string $file_path): int
    {
        if (self::isRuntimeStubFile($file_path)) {
            // the runtime stubs (exceptions, SPL, PHPUnit shim, ...) are always part of the main crate
            return 0;
        }
        foreach ($this->splits as $i => $split) {
            if (str_starts_with($file_path, $split['prefix'])) {
                return $i + 1;
            }
        }
        return 0;
    }

    public function crateCount(): int
    {
        return count($this->splits) + 1;
    }

    public function crateDir(int $i): string
    {
        return $i === 0 ? $this->out_dir : $this->splits[$i - 1]['dir'];
    }

    /** Rust crate name (the output directory's basename). */
    public function crateName(int $i): string
    {
        return str_replace('-', '_', basename($this->crateDir($i)));
    }

    public static function isEnabled(): bool
    {
        return self::$instance !== null;
    }

    /** Runtime stubs are PHP implementations of builtin classes, transpiled like project code. */
    public static function runtimeStubDir(): string
    {
        return __DIR__ . '/runtime-stubs';
    }

    public static function isRuntimeStubFile(string $file_path): bool
    {
        return str_starts_with($file_path, self::runtimeStubDir() . '/');
    }

    /** @var array<lowercase-string, true>|null */
    private static ?array $runtime_stub_classes = null;

    /**
     * Whether a runtime stub declares this class-like; other definitions (Psalm's stubs, vendor) are then ignored
     * so that the stub is the only definition Psalm ever sees.
     */
    public static function isRuntimeStubClass(string $fq_classlike_name_lc): bool
    {
        if (self::$runtime_stub_classes === null) {
            self::$runtime_stub_classes = [];
            foreach (glob(self::runtimeStubDir() . '/*.php') ?: [] as $file) {
                $code = (string) file_get_contents($file);
                $namespace = '';
                foreach (preg_split('/\R/', $code) ?: [] as $line) {
                    if (preg_match('/^namespace\s+([A-Za-z0-9_\\\\]+)\s*[;{]/', $line, $m)) {
                        $namespace = $m[1] . '\\';
                    } elseif (preg_match('/^(?:abstract\s+|final\s+|readonly\s+)*(?:class|interface|trait|enum)\s+([A-Za-z0-9_]+)/', $line, $m)) {
                        self::$runtime_stub_classes[strtolower($namespace . $m[1])] = true;
                    }
                }
            }
        }
        return isset(self::$runtime_stub_classes[$fq_classlike_name_lc]);
    }

    public static function get(): Transpiler
    {
        if (self::$instance === null) {
            throw new RuntimeException('Transpiler is not enabled');
        }

        return self::$instance;
    }

    public function recordFunctionLike(
        FunctionLikeAnalyzer $analyzer,
        Closure|Function_|ClassMethod|ArrowFunction $node,
        FunctionLikeStorage $storage,
        StatementsAnalyzer $statements_analyzer,
        Context $context,
    ): void {
        $file_path = $analyzer->getFilePath();

        if (!$this->config->isInProjectDirs($file_path)) {
            return;
        }

        $fq_class_name = $analyzer->getFQCLN();
        if ($fq_class_name !== null && $context->self !== null && strtolower($fq_class_name) !== strtolower($context->self)) {
            $codebase = $statements_analyzer->getCodebase();
            if ($codebase->classlike_storage_provider->has($fq_class_name)
                && $codebase->classlike_storage_provider->get($fq_class_name)->is_trait
            ) {
                // a trait method analyzed for a using class: its body belongs to that class
                $fq_class_name = $context->self;
            }
        }

        $record = new FunctionRecord(
            $node,
            $storage,
            $statements_analyzer->node_data,
            $file_path,
            $fq_class_name,
            $node instanceof ClassMethod ? $node->name->name : null,
        );

        $record->exit_vars = $context->vars_in_scope;
        $record->addVarTypes($context->vars_in_scope);

        // statements were recorded while the body was being analyzed, keyed by analyzer
        if (isset($this->active[$statements_analyzer])) {
            $pending = $this->active[$statements_analyzer];
            $record->stmt_vars = $pending->stmt_vars;
            foreach ($pending->var_types as $var_id => $types) {
                foreach ($types as $type) {
                    $record->var_types[$var_id][] = $type;
                }
            }
            unset($this->active[$statements_analyzer]);
        }

        if (getenv('DBG_REC') && $node instanceof ClassMethod && $node->name->name === getenv('DBG_REC')) {
            $t = $record->var_types['$directory'] ?? [];
            fwrite(STDERR, "[rec] {$record->fq_class_name}::{$node->name->name} self={$context->self} key=" . spl_object_id($statements_analyzer) . " pending=" . (isset($pending) ? 'yes' : 'no') . " directory=" . implode('|', array_map(static fn($u) => $u->getId(), $t)) . " exit=" . (isset($context->vars_in_scope['$directory']) ? $context->vars_in_scope['$directory']->getId() : '-') . "\n");
        }
        $this->functions[spl_object_id($node) . '@' . strtolower((string) $record->fq_class_name)] = $record;
    }

    public function recordStatement(StatementsAnalyzer $statements_analyzer, Stmt $stmt, Context $context): void
    {
        if (!isset($this->active[$statements_analyzer])) {
            $file_path = $statements_analyzer->getFilePath();
            if (!$this->config->isInProjectDirs($file_path)) {
                return;
            }
            // a placeholder record; the real one is created when the function-like finishes
            $this->active[$statements_analyzer] = new PendingRecord();
        }

        $this->active[$statements_analyzer]->stmt_vars[$stmt] = $context->vars_in_scope;
        $this->active[$statements_analyzer]->addVarTypes($context->vars_in_scope);
    }

    public function recordClassLike(ClassLikeAnalyzer $analyzer, ClassLike $node, ClassLikeStorage $storage): void
    {
        $file_path = $analyzer->getFilePath();

        if (!$this->config->isInProjectDirs($file_path)) {
            return;
        }

        $this->classes[strtolower($storage->name)] = new ClassRecord($node, $storage, $file_path);
    }

    public function getFunctionRecord(Closure|Function_|ClassMethod|ArrowFunction $node, ?string $fq_class_name): ?FunctionRecord
    {
        return $this->functions[spl_object_id($node) . '@' . strtolower((string) $fq_class_name)] ?? null;
    }

    /** @var array<int, array{FunctionLikeStorage, Union}> inferred return types of function-likes without a declared one */
    private array $inferred_return_types = [];

    public function recordInferredReturnType(FunctionLikeStorage $storage, Union $type): void
    {
        if ($type->isVoid() || $type->isNever() || $type->hasMixed() || $type->possibly_undefined) {
            return;
        }
        $this->inferred_return_types[spl_object_id($storage)] = [$storage, $type];
    }

    /**
     * Declare the inferred return types on their storages so a second analysis pass sees typed call sites.
     *
     * @return int number of signatures completed
     */
    public function applyInferredReturnTypes(): int
    {
        $n = 0;
        foreach ($this->inferred_return_types as [$storage, $type]) {
            if ($storage->return_type === null) {
                $storage->return_type = $type;
                $n++;
                if (getenv('DBG_REC')) {
                    fwrite(STDERR, "[inferred-return] " . ($storage->cased_name ?? '?') . ' => ' . $type->getId() . "\n");
                }
            }
        }
        $this->inferred_return_types = [];
        return $n;
    }

    /** Forget everything recorded by the first analysis pass. */
    public function resetRecords(): void
    {
        $this->functions = [];
        $this->classes = [];
        $this->active = new \WeakMap();
    }

    public function emit(Codebase $codebase): void
    {
        fwrite(STDERR, "\nTranspiling to Rust: " . count($this->functions) . " functions, "
            . count($this->classes) . " classes\n");

        $emitter = new CrateEmitter($this, $codebase);
        $emitter->emit();
    }
}
