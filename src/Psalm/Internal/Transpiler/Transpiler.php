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
use Psalm\Storage\FunctionLikeStorage;
use RuntimeException;

use function fwrite;
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

    public function emit(Codebase $codebase): void
    {
        fwrite(STDERR, "\nTranspiling to Rust: " . count($this->functions) . " functions, "
            . count($this->classes) . " classes\n");

        $emitter = new CrateEmitter($this, $codebase);
        $emitter->emit();
    }
}
