<?php
/**
 * Prepares the closed-world TypePHP build of Psalm and its vendor packages.
 *
 * Run it with the PHP build that the binary will embed, so that conditions such
 * as function_exists()/extension_loaded() are evaluated for the target runtime:
 *
 *   /path/to/embed/bin/php typephp/gen-vendor-build.php
 *
 * It writes:
 *   typephp/vendor-overrides/  copies of vendor files whose file-scope code was
 *                              flattened (constant conditions evaluated, guards
 *                              removed, aliases/requires dropped)
 *   typephp/project.yml        the TypePHP project file (sources + ignore list);
 *                              project-open-world.yml with --open-world
 */

declare(strict_types=1);

require __DIR__ . '/../vendor/autoload.php';

use PhpParser\BuilderHelpers;
use PhpParser\Node;
use PhpParser\Node\Expr;
use PhpParser\Node\Scalar;
use PhpParser\Node\Stmt;
use PhpParser\ParserFactory;
use PhpParser\PrettyPrinter\Standard;

$root = realpath(__DIR__ . '/..');
$vendor = $root . '/vendor';
$overrides = __DIR__ . '/vendor-overrides';
$report = [];

// packages whose functions/classes the embed PHP already provides natively
$excludedPackages = [
    'symfony/polyfill-ctype',
    'symfony/polyfill-mbstring',
    'symfony/polyfill-php84',
    'symfony/polyfill-intl-grapheme',
    'symfony/polyfill-intl-normalizer',
];

$installed = json_decode((string) file_get_contents($vendor . '/composer/installed.json'), true, 512, JSON_THROW_ON_ERROR)['packages'];
$runtime = array_map('trim', file(__DIR__ . '/runtime-packages.txt'));

$sources = [];
$ignore = [];
foreach ($installed as $package) {
    if (!in_array($package['name'], $runtime, true) || in_array($package['name'], $excludedPackages, true)) {
        continue;
    }
    $base = $vendor . '/' . $package['name'];
    $autoload = $package['autoload'] ?? [];
    $dirs = [];
    foreach (['psr-4', 'psr-0'] as $kind) {
        foreach ($autoload[$kind] ?? [] as $paths) {
            foreach ((array) $paths as $path) {
                $dirs[] = rtrim($base . '/' . $path, '/');
            }
        }
    }
    foreach ($autoload['classmap'] ?? [] as $path) {
        $dirs[] = rtrim($base . '/' . $path, '/');
    }
    foreach ($autoload['files'] ?? [] as $path) {
        $file = $base . '/' . $path;
        $inside = false;
        foreach ($dirs as $dir) {
            if (str_starts_with($file, $dir . '/')) {
                $inside = true;
            }
        }
        if (!$inside) {
            $dirs[] = $file;
        }
    }
    foreach (array_unique($dirs) as $dir) {
        $sources[] = $dir;
    }
}

/** @return list<string> */
function phpFiles(string $path): array
{
    if (is_file($path)) {
        return [$path];
    }
    $out = [];
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path, FilesystemIterator::SKIP_DOTS));
    foreach ($it as $f) {
        if (str_ends_with($f->getPathname(), '.php')) {
            $out[] = $f->getPathname();
        }
    }
    sort($out);
    return $out;
}

$parser = (new ParserFactory())->createForNewestSupportedVersion();
$printer = new Standard();

/**
 * Statically evaluates the constant conditions vendor files use at file scope.
 *
 * @param array<string, mixed> $fileConsts constants declared earlier in the same file
 */
function evaluate(Expr $expr, string $namespace, array $fileConsts): mixed
{
    if ($expr instanceof Scalar\String_ || $expr instanceof Scalar\Int_ || $expr instanceof Scalar\Float_) {
        return $expr->value;
    }
    if ($expr instanceof Expr\BooleanNot) {
        return !evaluate($expr->expr, $namespace, $fileConsts);
    }
    if ($expr instanceof Expr\BinaryOp\BooleanAnd) {
        return evaluate($expr->left, $namespace, $fileConsts) && evaluate($expr->right, $namespace, $fileConsts);
    }
    if ($expr instanceof Expr\BinaryOp\BooleanOr) {
        return evaluate($expr->left, $namespace, $fileConsts) || evaluate($expr->right, $namespace, $fileConsts);
    }
    if ($expr instanceof Expr\BinaryOp\Concat) {
        return evaluate($expr->left, $namespace, $fileConsts) . evaluate($expr->right, $namespace, $fileConsts);
    }
    if ($expr instanceof Expr\BinaryOp) {
        $l = evaluate($expr->left, $namespace, $fileConsts);
        $r = evaluate($expr->right, $namespace, $fileConsts);
        return match ($expr->getOperatorSigil()) {
            '>=' => $l >= $r, '>' => $l > $r, '<=' => $l <= $r, '<' => $l < $r,
            '===' => $l === $r, '!==' => $l !== $r, '==' => $l == $r, '!=' => $l != $r,
            default => throw new RuntimeException('operator ' . $expr->getOperatorSigil()),
        };
    }
    if ($expr instanceof Expr\Cast\Double) {
        return (float) evaluate($expr->expr, $namespace, $fileConsts);
    }
    if ($expr instanceof Expr\Ternary && $expr->if !== null) {
        return evaluate($expr->cond, $namespace, $fileConsts) ? evaluate($expr->if, $namespace, $fileConsts) : evaluate($expr->else, $namespace, $fileConsts);
    }
    if ($expr instanceof Expr\ConstFetch) {
        $name = $expr->name->toString();
        if (isset($fileConsts[$name])) {
            return $fileConsts[$name];
        }
        return constant(ltrim($name, '\\'));
    }
    if ($expr instanceof Scalar\MagicConst\Namespace_) {
        return $namespace;
    }
    if ($expr instanceof Expr\ClassConstFetch && $expr->name instanceof Node\Identifier && $expr->name->toLowerString() === 'class') {
        return $expr->class->toString();
    }
    if ($expr instanceof Expr\FuncCall && $expr->name instanceof Node\Name) {
        $fn = ltrim($expr->name->toString(), '\\');
        $args = array_map(static fn(Node\Arg $a) => evaluate($a->value, $namespace, $fileConsts), $expr->args);
        return match ($fn) {
            'function_exists' => function_exists((string) $args[0]),
            'class_exists' => class_exists((string) $args[0]),
            'interface_exists' => interface_exists((string) $args[0]),
            'defined' => defined((string) $args[0]),
            'extension_loaded' => extension_loaded((string) $args[0]),
            'strlen' => strlen((string) $args[0]),
            default => throw new RuntimeException('call ' . $fn),
        };
    }
    throw new RuntimeException('expression ' . $expr->getType());
}

/**
 * @param list<Node\Stmt> $stmts
 * @return array{list<Node\Stmt>, bool, bool} flattened statements, whether anything changed, whether the file returned
 */
function flatten(array $stmts, string $namespace, array &$fileConsts, string $file, array &$report): array
{
    $out = [];
    $changed = false;
    foreach ($stmts as $stmt) {
        if ($stmt instanceof Stmt\Namespace_) {
            $ns = $stmt->name?->toString() ?? '';
            [$inner, $c, $returned] = flatten($stmt->stmts, $ns, $fileConsts, $file, $report);
            $stmt->stmts = $inner;
            $out[] = $stmt;
            $changed = $changed || $c;
            if ($returned) {
                return [$out, true, true];
            }
            continue;
        }
        if ($stmt instanceof Stmt\Const_) {
            foreach ($stmt->consts as $const) {
                try {
                    $fileConsts[$const->name->toString()] = evaluate($const->value, $namespace, $fileConsts);
                } catch (RuntimeException) {
                }
            }
            $out[] = $stmt;
            continue;
        }
        if ($stmt instanceof Stmt\ClassLike || $stmt instanceof Stmt\Function_ || $stmt instanceof Stmt\Use_
            || $stmt instanceof Stmt\GroupUse || $stmt instanceof Stmt\Declare_ || $stmt instanceof Stmt\Nop
        ) {
            $out[] = $stmt;
            continue;
        }
        if ($stmt instanceof Stmt\If_ && $stmt->elseifs === []) {
            try {
                $cond = evaluate($stmt->cond, $namespace, $fileConsts);
            } catch (RuntimeException $e) {
                $report[] = "$file:{$stmt->getStartLine()}: cannot evaluate condition ({$e->getMessage()})";
                $out[] = $stmt;
                continue;
            }
            $branch = $cond ? $stmt->stmts : ($stmt->else?->stmts ?? []);
            [$inner, $_, $returned] = flatten($branch, $namespace, $fileConsts, $file, $report);
            foreach ($inner as $s) {
                $out[] = $s;
            }
            if ($returned) {
                return [$out, true, true];
            }
            $changed = true;
            continue;
        }
        if ($stmt instanceof Stmt\Return_) {
            // a file-scope `return` ends the executable part of the file
            return [$out, true, true];
        }
        if ($stmt instanceof Stmt\Expression) {
            $expr = $stmt->expr;
            if ($expr instanceof Expr\Include_) {
                $changed = true;
                continue;
            }
            if ($expr instanceof Expr\FuncCall && $expr->name instanceof Node\Name) {
                $fn = ltrim($expr->name->toString(), '\\');
                if ($fn === 'class_alias') {
                    // registered at startup by the generated aliases file
                    global $classAliases;
                    $args = array_map(static fn(Node\Arg $a) => evaluate($a->value, $namespace, $fileConsts), $expr->args);
                    if (isset($args[0], $args[1]) && is_string($args[0]) && is_string($args[1])) {
                        $classAliases[$args[1]] = $args[0];
                    } else {
                        $report[] = "$file:{$stmt->getStartLine()}: class_alias() with non-constant arguments dropped";
                    }
                    $changed = true;
                    continue;
                }
                if ($fn === 'define' && isset($expr->args[0], $expr->args[1])) {
                    // define('NS\\NAME', <constant expression>) becomes `const NAME = ...;`
                    // when the constant belongs to the file's namespace
                    try {
                        $name = (string) evaluate($expr->args[0]->value, $namespace, $fileConsts);
                        $value = evaluate($expr->args[1]->value, $namespace, $fileConsts);
                    } catch (RuntimeException $e) {
                        $report[] = "$file:{$stmt->getStartLine()}: define() kept at file scope ({$e->getMessage()})";
                        $out[] = $stmt;
                        continue;
                    }
                    $pos = strrpos($name, '\\');
                    $constNs = $pos === false ? '' : substr($name, 0, $pos);
                    $short = $pos === false ? $name : substr($name, $pos + 1);
                    if ($constNs !== $namespace) {
                        $report[] = "$file:{$stmt->getStartLine()}: define('$name') outside the file namespace dropped";
                    } else {
                        $out[] = new Stmt\Const_([new Node\Const_($short, BuilderHelpers::normalizeValue($value))]);
                        $fileConsts[$short] = $value;
                    }
                    $changed = true;
                    continue;
                }
                // opcache preload hints, deprecations and php-parser's token setup have no compile-time meaning
                if (in_array($fn, ['trigger_deprecation', 'class_exists', 'interface_exists', 'trait_exists', 'enum_exists', 'function_exists'], true)
                    || str_ends_with($fn, 'defineCompatibilityTokens')
                ) {
                    $changed = true;
                    continue;
                }
            }
            $report[] = "$file:{$stmt->getStartLine()}: file-scope statement kept ({$expr->getType()})";
            $out[] = $stmt;
            continue;
        }
        $report[] = "$file:{$stmt->getStartLine()}: file-scope statement kept ({$stmt->getType()})";
        $out[] = $stmt;
    }
    return [$out, $changed, false];
}

if (is_dir($overrides)) {
    exec('rm -rf ' . escapeshellarg($overrides));
}

/*
 * Closed world: only the vendor files reachable (by name) from Psalm's sources
 * are compiled. Every candidate file is parsed once to index the classes,
 * functions and constants it declares and the names it references.
 */
use PhpParser\NodeTraverser;
use PhpParser\NodeVisitor\NameResolver;
use PhpParser\NodeVisitorAbstract;

final class SymbolCollector extends NodeVisitorAbstract
{
    /** @var list<string> */
    public array $classes = [];
    /** @var list<string> */
    public array $functions = [];
    /** @var list<string> */
    public array $constants = [];
    /** @var list<string> */
    public array $classRefs = [];
    /** @var list<string> */
    public array $functionRefs = [];
    /** @var list<string> */
    public array $constantRefs = [];

    public function enterNode(Node $node): null
    {
        if ($node instanceof Stmt\ClassLike && isset($node->namespacedName)) {
            $this->classes[] = $node->namespacedName->toString();
        } elseif ($node instanceof Stmt\Function_ && isset($node->namespacedName)) {
            $this->functions[] = $node->namespacedName->toString();
        } elseif ($node instanceof Stmt\Const_) {
            foreach ($node->consts as $const) {
                if (isset($const->namespacedName)) {
                    $this->constants[] = $const->namespacedName->toString();
                }
            }
        }
        if ($node instanceof Stmt\Class_) {
            foreach ([$node->extends, ...$node->implements] as $name) {
                if ($name !== null) {
                    $this->classRefs[] = $name->toString();
                }
            }
        } elseif ($node instanceof Stmt\Interface_) {
            foreach ($node->extends as $name) {
                $this->classRefs[] = $name->toString();
            }
        } elseif ($node instanceof Stmt\Enum_) {
            foreach ($node->implements as $name) {
                $this->classRefs[] = $name->toString();
            }
        } elseif ($node instanceof Stmt\TraitUse) {
            foreach ($node->traits as $name) {
                $this->classRefs[] = $name->toString();
            }
        } elseif ($node instanceof Stmt\Catch_) {
            foreach ($node->types as $name) {
                $this->classRefs[] = $name->toString();
            }
        } elseif ($node instanceof Expr\New_ || $node instanceof Expr\StaticCall || $node instanceof Expr\StaticPropertyFetch
            || $node instanceof Expr\Instanceof_ || $node instanceof Expr\ClassConstFetch
        ) {
            $class = $node->class;
            if ($class instanceof Node\Name) {
                $lower = $class->toLowerString();
                if (!in_array($lower, ['self', 'static', 'parent'], true)) {
                    // X::class is only a string; it does not need the class
                    if (!($node instanceof Expr\ClassConstFetch && $node->name instanceof Node\Identifier && $node->name->toLowerString() === 'class')) {
                        $this->classRefs[] = $class->toString();
                    }
                }
            }
        } elseif ($node instanceof Expr\FuncCall && $node->name instanceof Node\Name) {
            $this->functionRefs[] = $node->name->toString();
            if ($node->name->hasAttribute('namespacedName')) {
                $this->functionRefs[] = $node->name->getAttribute('namespacedName')->toString();
            }
        } elseif ($node instanceof Expr\ConstFetch) {
            $this->constantRefs[] = $node->name->toString();
            if ($node->name->hasAttribute('namespacedName')) {
                $this->constantRefs[] = $node->name->getAttribute('namespacedName')->toString();
            }
        } elseif ($node instanceof Node\Name && !$node instanceof Node\Name\FullyQualified) {
            // leftover (type hints etc. are resolved to FullyQualified by the NameResolver)
        } elseif ($node instanceof Node\Name\FullyQualified) {
            $this->classRefs[] = $node->toString();
        } elseif ($node instanceof Node\Attribute) {
            $this->classRefs[] = $node->name->toString();
        }
        return null;
    }
}

/** @return array{SymbolCollector, list<Node\Stmt>} */
function collectSymbols(string $file, PhpParser\Parser $parser): array
{
    $stmts = $parser->parse((string) file_get_contents($file)) ?? [];
    $traverser = new NodeTraverser();
    $traverser->addVisitor(new NameResolver(null, ['preserveOriginalNames' => false, 'replaceNodes' => true]));
    $collector = new SymbolCollector();
    $traverser->addVisitor($collector);
    $traverser->traverse($stmts);
    return [$collector, $stmts];
}

$index = ['class' => [], 'function' => [], 'constant' => []];
/** @var array<string, SymbolCollector> $symbols */
$symbols = [];
$candidates = [];
foreach ($sources as $source) {
    foreach (phpFiles($source) as $file) {
        $candidates[] = $file;
    }
}
$candidates = array_values(array_unique($candidates));
foreach ($candidates as $file) {
    try {
        [$collector] = collectSymbols($file, $parser);
    } catch (PhpParser\Error $e) {
        $report[] = substr($file, strlen($vendor) + 1) . ': parse error ' . $e->getMessage();
        continue;
    }
    $symbols[$file] = $collector;
    foreach ($collector->classes as $name) {
        $index['class'][strtolower($name)] ??= $file;
    }
    foreach ($collector->functions as $name) {
        $index['function'][strtolower($name)] ??= $file;
    }
    foreach ($collector->constants as $name) {
        $index['constant'][$name] ??= $file;
    }
}

// roots: Psalm's sources, the native entry point and the native overrides
$reachable = [];
$queue = [];
foreach ([$root . '/src', __DIR__ . '/main.php', __DIR__ . '/vendor-extra'] as $rootSource) {
    foreach (phpFiles($rootSource) as $file) {
        [$collector] = collectSymbols($file, $parser);
        $queue[] = $collector;
    }
}
$vendorRefs = ['class' => [], 'function' => [], 'constant' => []];
while ($queue) {
    $collector = array_pop($queue);
    $targets = [];
    foreach ($collector->classRefs as $name) {
        $targets[] = $index['class'][strtolower(ltrim($name, '\\'))] ?? null;
    }
    foreach ($collector->functionRefs as $name) {
        $targets[] = $index['function'][strtolower(ltrim($name, '\\'))] ?? null;
    }
    foreach ($collector->constantRefs as $name) {
        $targets[] = $index['constant'][ltrim($name, '\\')] ?? null;
    }
    foreach ($targets as $target) {
        if ($target === null || isset($reachable[$target])) {
            continue;
        }
        $reachable[$target] = true;
        $queue[] = $symbols[$target];
    }
}
// composer "files" entries (functions) of reachable packages are always loaded
foreach ($candidates as $file) {
    if (!isset($reachable[$file]) && str_contains($file, '/functions.php')) {
        $reachable[$file] = true;
    }
}
$reachableFiles = array_keys($reachable);
sort($reachableFiles);

$compiled = [];
$overridden = 0;
$excluded = [];
/** @var array<string, string> alias => class */
$classAliases = [];
$manual = __DIR__ . '/vendor-manual';
foreach ($reachableFiles as $file) {
    $relative = substr($file, strlen($vendor) + 1);
    if (is_file($manual . '/' . $relative)) {
        // hand-written replacement (see typephp/vendor-manual/README.md)
        $compiled[] = $manual . '/' . $relative;
        continue;
    }
    $code = (string) file_get_contents($file);
    // fast path: nothing executable at file scope
    if (!preg_match('/^(if|return|require|include|class_alias|\\\\?define|trigger_deprecation|\\\\?defineCompatibilityTokens|[A-Za-z_\\\\]+\()/m', $code)) {
        $compiled[] = $file;
        continue;
    }
    $stmts = $parser->parse($code) ?? [];
    // fully qualify names so that X::class and friends evaluate correctly
    $resolver = new NodeTraverser();
    $resolver->addVisitor(new NameResolver(null, ['preserveOriginalNames' => false, 'replaceNodes' => true]));
    $stmts = $resolver->traverse($stmts);
    $consts = [];
    [$flat, $changed] = flatten($stmts, '', $consts, $relative, $report);
    if (!$changed) {
        $compiled[] = $file;
        continue;
    }
    $hasDecl = false;
    $throws = false;
    foreach ($flat as $s) {
        $inner = $s instanceof Stmt\Namespace_ ? $s->stmts : [$s];
        foreach ($inner as $i) {
            if ($i instanceof Stmt\ClassLike || $i instanceof Stmt\Function_ || $i instanceof Stmt\Const_) {
                $hasDecl = true;
            }
            if ($i instanceof Stmt\Expression && $i->expr instanceof Expr\Throw_) {
                $throws = true;
            }
        }
    }
    if (!$hasDecl) {
        $excluded[] = $relative; // alias/bootstrap/data file with nothing left to compile
        continue;
    }
    if ($throws) {
        $report[] = "$relative: throws at file scope, excluded";
        $excluded[] = $relative;
        continue;
    }
    $target = $overrides . '/' . $relative;
    @mkdir(dirname($target), 0777, true);
    file_put_contents($target, $printer->prettyPrintFile($flat) . "\n");
    $compiled[] = $target;
    $overridden++;
}

// class_alias() calls of vendor files run at startup (see main.php)
$aliasCode = "<?php\n\n// Generated by typephp/gen-vendor-build.php - do not edit.\n\ndeclare(strict_types=1);\n\nnamespace Psalm\\Internal\\TypePhp;\n\n"
    . "function registerVendorClassAliases(): void\n{\n";
ksort($classAliases);
foreach ($classAliases as $alias => $class) {
    $aliasCode .= '    \\class_alias(' . var_export(ltrim($class, '\\'), true) . ', ' . var_export(ltrim($alias, '\\'), true) . ");\n";
}
$aliasCode .= "}\n";
@mkdir($overrides, 0777, true);
file_put_contents($overrides . '/class-aliases.php', $aliasCode);
$compiled[] = $overrides . '/class-aliases.php';

$yaml = "# Generated by typephp/gen-vendor-build.php - do not edit.\n";
$yaml .= "name: psalm\nbuild-mode: bin\ncxx-std: c++17\n";
// --open-world: compiled classes may be extended/mocked by runtime-loaded code
// (used for running the test suite inside the binary, see run-tests.php);
// written to project-open-world.yml so that project.yml stays the
// closed-world production configuration
$openWorld = in_array('--open-world', $argv, true);
if ($openWorld) {
    $yaml .= "open-world: true\n";
}
$yaml .= "sources:\n  - ./main.php\n  - ./vendor-extra\n  - ../src\n";
foreach ($compiled as $file) {
    $yaml .= '  - ' . str_replace([__DIR__ . '/', $root . '/'], ['./', '../'], $file) . "\n";
}
$yaml .= "ignore: []\n";
file_put_contents(__DIR__ . ($openWorld ? '/project-open-world.yml' : '/project.yml'), $yaml);
echo count($candidates) . " candidate vendor files, " . count($compiled) . " reachable and compiled ($overridden rewritten), " . count($excluded) . " excluded\n";
foreach ($report as $line) {
    echo "  $line\n";
}
