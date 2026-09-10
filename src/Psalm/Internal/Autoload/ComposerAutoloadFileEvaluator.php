<?php

declare(strict_types=1);

namespace Psalm\Internal\Autoload;

use PhpParser\Error;
use PhpParser\Node;
use PhpParser\Node\Expr;
use PhpParser\Node\Scalar;
use PhpParser\Node\Stmt;
use PhpParser\ParserFactory;
use Psalm\Exception\ConfigException;

use function array_key_exists;
use function dirname;
use function file_get_contents;
use function is_array;
use function is_int;
use function is_scalar;
use function is_string;

/**
 * Evaluates the generated vendor/composer/autoload_*.php map files without executing them.
 *
 * Those files only assign `$vendorDir`/`$baseDir` and return an array made of string
 * literals concatenated with those variables.
 *
 * @internal
 */
final class ComposerAutoloadFileEvaluator
{
    /**
     * @return array<array-key, mixed>
     * @throws ConfigException
     * @psalm-suppress MixedAssignment
     */
    public static function evaluate(string $file): array
    {
        $contents = file_get_contents($file);

        if ($contents === false) {
            throw new ConfigException('Could not read ' . $file);
        }

        try {
            $stmts = (new ParserFactory())->createForNewestSupportedVersion()->parse($contents) ?? [];
        } catch (Error $e) {
            throw new ConfigException('Could not parse ' . $file . ': ' . $e->getMessage(), 0, $e);
        }

        /** @var array<string, mixed> $variables */
        $variables = [];

        foreach ($stmts as $stmt) {
            if ($stmt instanceof Stmt\Expression
                && $stmt->expr instanceof Expr\Assign
                && $stmt->expr->var instanceof Expr\Variable
                && is_string($stmt->expr->var->name)
            ) {
                $variables[$stmt->expr->var->name] = self::evaluateExpression($stmt->expr->expr, $file, $variables);
                continue;
            }

            if ($stmt instanceof Stmt\Return_ && $stmt->expr !== null) {
                $value = self::evaluateExpression($stmt->expr, $file, $variables);

                if (!is_array($value)) {
                    throw new ConfigException($file . ' does not return an array');
                }

                return $value;
            }

            if ($stmt instanceof Stmt\Nop || $stmt instanceof Stmt\Declare_ || $stmt instanceof Stmt\InlineHTML) {
                continue;
            }

            throw new ConfigException('Unsupported statement ' . $stmt->getType() . ' in ' . $file);
        }

        throw new ConfigException($file . ' does not return an array');
    }

    /**
     * @param array<string, mixed> $variables
     * @throws ConfigException
     * @psalm-suppress MixedAssignment
     */
    private static function evaluateExpression(Expr $expr, string $file, array $variables): mixed
    {
        if ($expr instanceof Scalar\String_ || $expr instanceof Scalar\Int_) {
            return $expr->value;
        }

        if ($expr instanceof Scalar\MagicConst\Dir) {
            return dirname($file);
        }

        if ($expr instanceof Expr\Variable && is_string($expr->name)) {
            if (!array_key_exists($expr->name, $variables)) {
                throw new ConfigException('Undefined variable $' . $expr->name . ' in ' . $file);
            }

            return $variables[$expr->name];
        }

        if ($expr instanceof Expr\BinaryOp\Concat) {
            $left = self::evaluateExpression($expr->left, $file, $variables);
            $right = self::evaluateExpression($expr->right, $file, $variables);

            if (!is_scalar($left) || !is_scalar($right)) {
                throw new ConfigException('Unsupported concatenation in ' . $file);
            }

            return (string) $left . (string) $right;
        }

        if ($expr instanceof Expr\Array_) {
            $result = [];

            foreach ($expr->items as $item) {
                if ($item === null || $item->byRef || $item->unpack) {
                    throw new ConfigException('Unsupported array item in ' . $file);
                }

                $value = self::evaluateExpression($item->value, $file, $variables);

                if ($item->key === null) {
                    $result[] = $value;
                    continue;
                }

                $key = self::evaluateExpression($item->key, $file, $variables);

                if (!is_int($key) && !is_string($key)) {
                    throw new ConfigException('Unsupported array key in ' . $file);
                }

                $result[$key] = $value;
            }

            return $result;
        }

        if ($expr instanceof Expr\FuncCall
            && $expr->name instanceof Node\Name
            && $expr->name->toLowerString() === 'dirname'
        ) {
            $path = isset($expr->args[0]) && $expr->args[0] instanceof Node\Arg
                ? self::evaluateExpression($expr->args[0]->value, $file, $variables)
                : null;
            $levels = isset($expr->args[1]) && $expr->args[1] instanceof Node\Arg
                ? self::evaluateExpression($expr->args[1]->value, $file, $variables)
                : 1;

            if (!is_string($path) || !is_int($levels) || $levels < 1) {
                throw new ConfigException('Unsupported dirname() call in ' . $file);
            }

            return dirname($path, $levels);
        }

        if ($expr instanceof Expr\ConstFetch) {
            $name = $expr->name->toLowerString();

            if ($name === 'true' || $name === 'false' || $name === 'null') {
                return $name === 'true' ? true : ($name === 'false' ? false : null);
            }
        }

        throw new ConfigException('Unsupported expression ' . $expr->getType() . ' in ' . $file);
    }
}
