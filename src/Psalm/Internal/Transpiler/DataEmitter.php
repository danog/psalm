<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Expr;
use PhpParser\Node\Name;
use PhpParser\Node\Scalar;
use Psalm\Codebase;
use RuntimeException;

use function constant;
use function defined;
use function get_debug_type;
use function is_array;
use function implode;
use function is_bool;
use function is_float;
use function is_int;
use function is_string;
use function preg_match;
use function sprintf;
use function str_contains;
use function strtolower;

/**
 * Emits the constant value returned by a data file as a static `php_rt::data::Data` table.
 *
 * @internal
 */
final class DataEmitter
{
    public function __construct(
        private readonly Codebase $codebase,
    ) {
    }

    /**
     * The value returned by a dictionary file, obtained by running it (a `return [...]` of literals): the
     * file is never parsed or analyzed, its literals are dumped mechanically.
     *
     * @throws RuntimeException when the file returns something that is not plain data
     */
    public function emitFile(string $abs_path): string
    {
        /** @var mixed $value */
        $value = (static function (string $__path): mixed {
            /** @psalm-suppress UnresolvableInclude */
            return require $__path;
        })($abs_path);
        return $this->fromRuntime($value);
    }

    private function fromRuntime(mixed $v): string
    {
        if (is_array($v)) {
            $items = [];
            /** @var mixed $item */
            foreach ($v as $k => $item) {
                $key = is_int($k) ? 'DataKey::Int(' . $k . 'i64)' : 'DataKey::Str(' . Names::rustStringLiteral($k) . ')';
                $items[] = '(' . $key . ', ' . $this->fromRuntime($item) . ')';
            }
            return 'Data::Arr(&[' . implode(', ', $items) . '])';
        }
        if ($v === null || is_bool($v) || is_int($v) || is_float($v) || is_string($v)) {
            return $this->fromValue($v);
        }
        throw new RuntimeException('a dictionary may only contain arrays and scalars, found ' . get_debug_type($v));
    }

    /** @throws RuntimeException for expressions that are not constant data */
    public function emit(Expr $e): string
    {
        if ($e instanceof Scalar\Int_) {
            return 'Data::Int(' . $e->value . 'i64)';
        }
        if ($e instanceof Scalar\Float_) {
            return 'Data::Float(' . $this->float($e->value) . ')';
        }
        if ($e instanceof Scalar\String_) {
            return $this->str($e->value);
        }
        if ($e instanceof Scalar\InterpolatedString) {
            $parts = [];
            foreach ($e->parts as $part) {
                if (!$part instanceof \PhpParser\Node\InterpolatedStringPart) {
                    throw new RuntimeException('interpolated string with expressions');
                }
                $parts[] = $part->value;
            }
            return $this->str(implode('', $parts));
        }
        if ($e instanceof Expr\UnaryMinus || $e instanceof Expr\UnaryPlus) {
            $v = $this->value($e->expr);
            $v = $e instanceof Expr\UnaryMinus ? -$v : +$v;
            return is_int($v) ? 'Data::Int(' . $v . 'i64)' : 'Data::Float(' . $this->float((float) $v) . ')';
        }
        if ($e instanceof Expr\ConstFetch) {
            return $this->fromValue($this->value($e));
        }
        if ($e instanceof Expr\ClassConstFetch) {
            return $this->fromValue($this->value($e));
        }
        if ($e instanceof Expr\BinaryOp\Concat) {
            return $this->str((string) $this->value($e->left) . (string) $this->value($e->right));
        }
        if ($e instanceof Expr\BinaryOp) {
            return $this->fromValue($this->value($e));
        }
        if ($e instanceof Expr\Array_) {
            $items = [];
            $next = 0;
            foreach ($e->items as $item) {
                if ($item->unpack) {
                    throw new RuntimeException('spread in data array');
                }
                if ($item->key === null) {
                    $key = 'DataKey::Int(' . $next . 'i64)';
                    $next++;
                } else {
                    $k = $this->value($item->key);
                    if (is_int($k) || (is_string($k) && preg_match('/^(0|-?[1-9][0-9]*)$/', $k) && (string) (int) $k === $k)) {
                        $key = 'DataKey::Int(' . (int) $k . 'i64)';
                        $next = max($next, (int) $k + 1);
                    } elseif (is_bool($k)) {
                        $key = 'DataKey::Int(' . ($k ? 1 : 0) . 'i64)';
                    } elseif ($k === null) {
                        $key = 'DataKey::Str("")';
                    } else {
                        $key = 'DataKey::' . substr($this->str((string) $k), 6);
                    }
                }
                $items[] = '(' . $key . ', ' . $this->emit($item->value) . ')';
            }
            return 'Data::Arr(&[' . implode(', ', $items) . '])';
        }
        throw new RuntimeException('unsupported data expression ' . $e->getType());
    }

    /** The PHP value of a constant expression (scalars and constants only). */
    private function value(Expr $e): int|float|string|bool|null
    {
        if ($e instanceof Scalar\Int_) {
            return $e->value;
        }
        if ($e instanceof Scalar\Float_) {
            return $e->value;
        }
        if ($e instanceof Scalar\String_) {
            return $e->value;
        }
        if ($e instanceof Expr\UnaryMinus) {
            return -$this->value($e->expr);
        }
        if ($e instanceof Expr\UnaryPlus) {
            return +$this->value($e->expr);
        }
        if ($e instanceof Expr\ConstFetch) {
            $name = $e->name->toString();
            $lc = strtolower($name);
            if ($lc === 'true') {
                return true;
            }
            if ($lc === 'false') {
                return false;
            }
            if ($lc === 'null') {
                return null;
            }
            $resolved = (string) ($e->name->getAttribute('resolvedName') ?? $name);
            foreach ([$resolved, $name] as $candidate) {
                if (defined($candidate)) {
                    $v = constant($candidate);
                    if (is_int($v) || is_float($v) || is_string($v) || is_bool($v) || $v === null) {
                        return $v;
                    }
                }
            }
            throw new RuntimeException('unknown constant ' . $name);
        }
        if ($e instanceof Expr\ClassConstFetch && $e->class instanceof Name && $e->name instanceof \PhpParser\Node\Identifier) {
            $fqcn = (string) ($e->class->getAttribute('resolvedName') ?? $e->class->toString());
            // the transpiler runs inside PHP with the project's classes loaded: the actual constant value
            if (defined($fqcn . '::' . $e->name->name)) {
                $v = constant($fqcn . '::' . $e->name->name);
                if (is_int($v) || is_float($v) || is_string($v) || is_bool($v) || $v === null) {
                    return $v;
                }
            }
            $type = $this->codebase->classlikes->getClassConstantType($fqcn, $e->name->name, \ReflectionProperty::IS_PRIVATE);
            if ($type !== null) {
                if ($type->isSingleIntLiteral()) {
                    return $type->getSingleIntLiteral()->value;
                }
                if ($type->isSingleStringLiteral()) {
                    return $type->getSingleStringLiteral()->value;
                }
                if ($type->isSingleFloatLiteral()) {
                    return $type->getSingleFloatLiteral()->value;
                }
                if ($type->isTrue()) {
                    return true;
                }
                if ($type->isFalse()) {
                    return false;
                }
                if ($type->isNull()) {
                    return null;
                }
            }
            throw new RuntimeException('class constant without literal value ' . $fqcn . '::' . $e->name->name);
        }
        if ($e instanceof Expr\BinaryOp\Concat) {
            return (string) $this->value($e->left) . (string) $this->value($e->right);
        }
        if ($e instanceof Expr\BinaryOp) {
            $l = $this->value($e->left);
            $r = $this->value($e->right);
            return match ($e->getOperatorSigil()) {
                '+' => $l + $r,
                '-' => $l - $r,
                '*' => $l * $r,
                '|' => $l | $r,
                '&' => $l & $r,
                '<<' => $l << $r,
                '>>' => $l >> $r,
                '.' => $l . $r,
                default => throw new RuntimeException('unsupported operator ' . $e->getOperatorSigil()),
            };
        }
        throw new RuntimeException('not a constant value: ' . $e->getType());
    }

    private function fromValue(int|float|string|bool|null $v): string
    {
        if ($v === null) {
            return 'Data::Null';
        }
        if (is_bool($v)) {
            return 'Data::Bool(' . ($v ? 'true' : 'false') . ')';
        }
        if (is_int($v)) {
            return 'Data::Int(' . $v . 'i64)';
        }
        if (is_float($v)) {
            return 'Data::Float(' . $this->float($v) . ')';
        }
        return $this->str($v);
    }

    private function str(string $s): string
    {
        if (preg_match('//u', $s) && !str_contains($s, "\0")) {
            return 'Data::Str(' . Names::rustStringLiteral($s) . ')';
        }
        return 'Data::Bytes(' . Names::byteStrLiteral($s) . ')';
    }

    private function float(float $f): string
    {
        if (is_nan($f)) {
            return 'f64::NAN';
        }
        if (is_infinite($f)) {
            return $f > 0 ? 'f64::INFINITY' : 'f64::NEG_INFINITY';
        }
        $s = sprintf('%.17g', $f);
        if (!str_contains($s, '.') && !str_contains($s, 'e')) {
            $s .= '.0';
        }
        return $s;
    }
}
