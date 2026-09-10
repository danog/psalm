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
 *   typephp/project.yml        the TypePHP project file (sources + ignore list)
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
$overridden = 0;
foreach ($sources as $source) {
    foreach (phpFiles($source) as $file) {
        $code = (string) file_get_contents($file);
        // fast path: nothing executable at file scope
        if (!preg_match('/^(if|return|require|include|class_alias|\\\\?define|trigger_deprecation|\\\\?defineCompatibilityTokens|[A-Za-z_\\\\]+\()/m', $code)) {
            continue;
        }
        $stmts = $parser->parse($code) ?? [];
        $consts = [];
        [$flat, $changed] = flatten($stmts, '', $consts, substr($file, strlen($vendor) + 1), $report);
        // (the third element, whether the file returned early, only matters inside flatten())
        if (!$changed) {
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
        $ignore[] = $file;
        if (!$hasDecl) {
            continue; // alias/bootstrap/data file with nothing left to compile
        }
        if ($throws) {
            $report[] = substr($file, strlen($vendor) + 1) . ': throws at file scope, excluded';
            continue;
        }
        $target = $overrides . substr($file, strlen($vendor));
        @mkdir(dirname($target), 0777, true);
        file_put_contents($target, $printer->prettyPrintFile($flat) . "\n");
        $overridden++;
    }
}

$yaml = "# Generated by typephp/gen-vendor-build.php - do not edit.\n";
$yaml .= "name: psalm\nbuild-mode: bin\ncxx-std: c++17\nsources:\n  - ./main.php\n  - ./overrides\n  - ./vendor-overrides\n  - ../src\n";
foreach ($sources as $s) {
    $yaml .= '  - ' . str_replace($root . '/', '../', $s) . "\n";
}
$yaml .= "ignore:\n  - ../src/Psalm/Internal/CodeLoader.php\n";
foreach ($ignore as $i) {
    $yaml .= '  - ' . str_replace($root . '/', '../', $i) . "\n";
}
file_put_contents(__DIR__ . '/project.yml', $yaml);
echo count($sources) . " vendor source entries, $overridden overrides, " . count($ignore) . " ignored files\n";
foreach ($report as $line) {
    $reported = explode(':', $line, 2)[0];
    if (in_array($vendor . '/' . $reported, $ignore, true) && !is_file($overrides . '/' . $reported)) {
        continue; // the file is excluded from the build anyway
    }
    echo "  $line\n";
}
