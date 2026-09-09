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

use function file_get_contents;
use function fwrite;
use function glob;
use function preg_match;
use function preg_split;
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
    private array $active = [];

    private function __construct(
        public string $out_dir,
        public Config $config,
        public string $root_dir,
    ) {
    }

    public static function enable(string $out_dir, Config $config, string $root_dir): void
    {
        if (!is_dir($out_dir) && !mkdir($out_dir, 0777, true)) {
            throw new RuntimeException("Could not create transpiler output directory $out_dir");
        }

        self::$instance = new self($out_dir, $config, $root_dir);
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

        $record = new FunctionRecord(
            $node,
            $storage,
            $statements_analyzer->node_data,
            $file_path,
            $analyzer->getFQCLN(),
            $node instanceof ClassMethod ? $node->name->name : null,
        );

        $record->exit_vars = $context->vars_in_scope;
        $record->addVarTypes($context->vars_in_scope);

        // statements were recorded while the body was being analyzed, keyed by analyzer
        $key = spl_object_id($statements_analyzer);
        if (isset($this->active[$key])) {
            $pending = $this->active[$key];
            $record->stmt_vars = $pending->stmt_vars;
            foreach ($pending->var_types as $var_id => $types) {
                foreach ($types as $type) {
                    $record->var_types[$var_id][] = $type;
                }
            }
            unset($this->active[$key]);
        }

        $this->functions[spl_object_id($node) . '@' . strtolower((string) $record->fq_class_name)] = $record;
    }

    public function recordStatement(StatementsAnalyzer $statements_analyzer, Stmt $stmt, Context $context): void
    {
        $key = spl_object_id($statements_analyzer);

        if (!isset($this->active[$key])) {
            $file_path = $statements_analyzer->getFilePath();
            if (!$this->config->isInProjectDirs($file_path)) {
                return;
            }
            // a placeholder record; the real one is created when the function-like finishes
            $this->active[$key] = new PendingRecord();
        }

        $this->active[$key]->stmt_vars[$stmt] = $context->vars_in_scope;
        $this->active[$key]->addVarTypes($context->vars_in_scope);
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
        $this->active = [];
    }

    public function emit(Codebase $codebase): void
    {
        fwrite(STDERR, "\nTranspiling to Rust: " . count($this->functions) . " functions, "
            . count($this->classes) . " classes\n");

        $emitter = new CrateEmitter($this, $codebase);
        $emitter->emit();
    }
}
