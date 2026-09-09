<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node;
use PhpParser\Node\Expr;
use PhpParser\Node\Expr\ArrowFunction;
use PhpParser\Node\Expr\BinaryOp;
use PhpParser\Node\Expr\Closure;
use PhpParser\Node\Identifier;
use PhpParser\Node\InterpolatedStringPart;
use PhpParser\Node\Name;
use PhpParser\Node\Scalar;
use Psalm\Type\Atomic\TLiteralInt;
use Psalm\Type\Atomic\TLiteralString;

use function count;
use function implode;
use function in_array;
use function is_string;
use function sprintf;
use function str_contains;
use function strtolower;

/**
 * Expression emission.
 *
 * @internal
 */
trait ExprTrait
{
    /** Emit an expression; if `$expected` is given, the result is converted to that type. */
    public function expr(Expr $e, ?RustType $expected = null): Val
    {
        $v = $this->exprNatural($e, $expected);
        if ($expected !== null) {
            return new Val($this->casts->convert($v->code, $v->type, $expected), $expected);
        }
        return $v;
    }

    public function exprTo(Expr $e, RustType $to): string
    {
        return $this->expr($e, $to)->code;
    }

    /** Emit an expression yielding an owned value of its natural Rust type. */
    private function exprNatural(Expr $e, ?RustType $expected): Val
    {
        // scalars
        if ($e instanceof Scalar\Int_) {
            return new Val($e->value . 'i64', RustType::int());
        }
        if ($e instanceof Scalar\Float_) {
            return new Val($this->floatLit($e->value), RustType::float());
        }
        if ($e instanceof Scalar\String_) {
            return new Val(Names::strLit($e->value), RustType::str());
        }
        if ($e instanceof Scalar\InterpolatedString) {
            return $this->interpolated($e->parts);
        }
        if ($e instanceof Scalar\MagicConst) {
            return $this->magicConst($e);
        }
        if ($e instanceof Expr\ConstFetch) {
            return $this->constFetch($e);
        }
        if ($e instanceof Expr\Variable) {
            return $this->variable($e);
        }
        if ($e instanceof Expr\Array_) {
            return $this->arrayLiteral($e, $expected);
        }
        if ($e instanceof Expr\Assign) {
            return $this->assignExpr($e);
        }
        if ($e instanceof Expr\AssignOp) {
            return $this->assignOpExpr($e);
        }
        if ($e instanceof Expr\AssignRef) {
            $this->warn('assign by reference', $e);
            return $this->assignExpr(new Expr\Assign($e->var, $e->expr, $e->getAttributes()));
        }
        if ($e instanceof BinaryOp) {
            return $this->binaryOp($e);
        }
        if ($e instanceof Expr\UnaryMinus) {
            $v = $this->expr($e->expr);
            if ($v->type->kind === RustType::FLOAT) {
                return new Val('(-' . $v->code . ')', RustType::float());
            }
            if ($v->type->kind === RustType::INT) {
                return new Val('(' . $v->code . ').wrapping_neg()', RustType::int());
            }
            return new Val('(-to_float(&' . $v->code . '))', RustType::float());
        }
        if ($e instanceof Expr\UnaryPlus) {
            return $this->expr($e->expr);
        }
        if ($e instanceof Expr\BooleanNot) {
            return new Val('(!' . $this->truthy($e->expr) . ')', RustType::bool());
        }
        if ($e instanceof Expr\BitwiseNot) {
            return new Val('(!' . $this->exprTo($e->expr, RustType::int()) . ')', RustType::int());
        }
        if ($e instanceof Expr\PreInc || $e instanceof Expr\PreDec || $e instanceof Expr\PostInc || $e instanceof Expr\PostDec) {
            return $this->incDec($e);
        }
        if ($e instanceof Expr\Cast) {
            return $this->cast($e);
        }
        if ($e instanceof Expr\Ternary) {
            return $this->ternary($e);
        }
        if ($e instanceof Expr\Isset_) {
            $parts = [];
            foreach ($e->vars as $v) {
                $parts[] = $this->issetCode($v);
            }
            return new Val('(' . implode(' && ', $parts) . ')', RustType::bool());
        }
        if ($e instanceof Expr\Empty_) {
            return new Val('(!' . $this->truthyOptional($e->expr) . ')', RustType::bool());
        }
        if ($e instanceof Expr\ArrayDimFetch) {
            return $this->dimFetch($e);
        }
        if ($e instanceof Expr\PropertyFetch) {
            return $this->propertyFetch($e, false);
        }
        if ($e instanceof Expr\NullsafePropertyFetch) {
            return $this->propertyFetch($e, true);
        }
        if ($e instanceof Expr\StaticPropertyFetch) {
            return $this->staticPropertyFetch($e);
        }
        if ($e instanceof Expr\ClassConstFetch) {
            return $this->classConstFetch($e);
        }
        if ($e instanceof Expr\FuncCall) {
            return $this->funcCall($e);
        }
        if ($e instanceof Expr\MethodCall) {
            return $this->methodCall($e, false);
        }
        if ($e instanceof Expr\NullsafeMethodCall) {
            return $this->methodCall($e, true);
        }
        if ($e instanceof Expr\StaticCall) {
            return $this->staticCall($e);
        }
        if ($e instanceof Expr\New_) {
            return $this->newExpr($e, $expected);
        }
        if ($e instanceof Closure || $e instanceof ArrowFunction) {
            return $this->closure($e);
        }
        if ($e instanceof Expr\Instanceof_) {
            return $this->instanceOf($e);
        }
        if ($e instanceof Expr\Match_) {
            return $this->matchExpr($e);
        }
        if ($e instanceof Expr\Throw_) {
            return new Val('return Err(' . $this->throwValue($e->expr) . ')', RustType::never());
        }
        if ($e instanceof Expr\Exit_) {
            $code = $e->expr !== null ? $this->expr($e->expr) : null;
            if ($code === null) {
                return new Val('return Err(Throw::exit(0))', RustType::never());
            }
            if ($code->type->kind === RustType::INT) {
                return new Val('return Err(Throw::exit(' . $code->code . '))', RustType::never());
            }
            return new Val('{ echo(to_str(' . $code->code . ').as_bytes()); return Err(Throw::exit(0)) }', RustType::never());
        }
        if ($e instanceof Expr\Print_) {
            return new Val('{ echo(' . $this->exprTo($e->expr, RustType::str()) . '.as_bytes()); 1i64 }', RustType::int());
        }
        if ($e instanceof Expr\ErrorSuppress) {
            return $this->expr($e->expr, $expected);
        }
        if ($e instanceof Expr\Clone_) {
            $v = $this->expr($e->expr);
            if ($v->type->kind === RustType::MIXED) {
                return new Val('crate::php_clone_mixed(' . $v->code . ')', $v->type);
            }
            return new Val($v->code . '.php_clone()', $v->type);
        }
        if ($e instanceof Expr\Include_) {
            $this->warn('include/require', $e);
            return new Val('{ let _ = ' . $this->expr($e->expr)->code . '; Mixed::Bool(true) }', RustType::mixed());
        }
        if ($e instanceof Expr\Eval_) {
            $this->warn('eval', $e);
            return new Val('{ let _ = ' . $this->expr($e->expr)->code . '; unreachable!("eval") }', RustType::never());
        }
        if ($e instanceof Expr\Yield_) {
            return $this->yieldExpr($e);
        }
        if ($e instanceof Expr\YieldFrom) {
            return $this->yieldFrom($e);
        }
        if ($e instanceof Expr\List_ || $e instanceof Expr\ShellExec) {
            $this->warn('unsupported expression ' . $e->getType(), $e);
            return new Val('unreachable!("unsupported ' . $e->getType() . '")', RustType::never());
        }

        $this->warn('unsupported expression ' . $e->getType(), $e);
        return new Val('unreachable!("unsupported ' . $e->getType() . '")', RustType::never());
    }

    private function floatLit(float $f): string
    {
        if (is_nan($f)) {
            return 'f64::NAN';
        }
        if (is_infinite($f)) {
            return $f > 0 ? 'f64::INFINITY' : 'f64::NEG_INFINITY';
        }
        $s = sprintf('%.17g', $f);
        if (!str_contains($s, '.') && !str_contains($s, 'e') && !str_contains($s, 'E')) {
            $s .= '.0';
        }
        return $s . 'f64';
    }

    /** Emit a boolean condition from a PHP expression (applies truthiness). */
    public function truthy(Expr $e): string
    {
        $v = $this->expr($e);
        if ($v->type->kind === RustType::BOOL) {
            return $v->code;
        }
        if ($v->type->kind === RustType::NEVER) {
            return '{ ' . $v->code . '; false }';
        }
        return 'truthy(&' . $v->code . ')';
    }

    /** Truthiness of an expression that may be undefined (empty()). */
    private function truthyOptional(Expr $e): string
    {
        $v = $this->optionalValue($e);
        if ($v === null) {
            return $this->truthy($e);
        }
        return 'truthy(&' . $v->code . ')';
    }

    private function interpolated(array $parts): Val
    {
        $codes = [];
        foreach ($parts as $part) {
            if ($part instanceof InterpolatedStringPart) {
                $codes[] = Names::strLit($part->value);
            } else {
                $codes[] = $this->exprTo($part, RustType::str());
            }
        }
        if (count($codes) === 1) {
            return new Val($codes[0], RustType::str());
        }
        return new Val('cat!(' . implode(', ', $codes) . ')', RustType::str());
    }

    private function magicConst(Scalar\MagicConst $e): Val
    {
        $t = $this->psalmType($e);
        if ($t !== null && $t->isSingleStringLiteral()) {
            return new Val(Names::strLit($t->getSingleStringLiteral()->value), RustType::str());
        }
        if ($e instanceof Scalar\MagicConst\Line) {
            return new Val($e->getStartLine() . 'i64', RustType::int());
        }
        if ($e instanceof Scalar\MagicConst\Dir) {
            return new Val('src_dir(' . Names::strLit($this->relativeDir()) . ')', RustType::str());
        }
        if ($e instanceof Scalar\MagicConst\File) {
            return new Val('src_file(' . Names::strLit($this->relativeFile()) . ')', RustType::str());
        }
        if ($e instanceof Scalar\MagicConst\Class_) {
            return new Val(Names::strLit($this->class?->fqcn ?? ''), RustType::str());
        }
        if ($e instanceof Scalar\MagicConst\Function_ || $e instanceof Scalar\MagicConst\Method) {
            return new Val(Names::strLit((string) $this->record->method_name), RustType::str());
        }
        if ($e instanceof Scalar\MagicConst\Namespace_) {
            $fq = $this->class?->fqcn ?? '';
            $pos = strrpos($fq, '\\');
            return new Val(Names::strLit($pos === false ? '' : substr($fq, 0, $pos)), RustType::str());
        }
        return new Val(Names::strLit(''), RustType::str());
    }

    private function relativeFile(): string
    {
        $root = $this->program->transpiler->root_dir;
        $f = $this->record->file_path;
        if (str_starts_with($f, $root . '/')) {
            return substr($f, strlen($root) + 1);
        }
        return $f;
    }

    private function relativeDir(): string
    {
        return dirname($this->relativeFile());
    }

    private function constFetch(Expr\ConstFetch $e): Val
    {
        $name = strtolower($e->name->toString());
        if ($name === 'true') {
            return new Val('true', RustType::bool());
        }
        if ($name === 'false') {
            return new Val('false', RustType::bool());
        }
        if ($name === 'null') {
            return new Val('()', RustType::unit());
        }
        $resolved = $e->name->getAttribute('resolvedName') ?? $e->name->toString();
        $short = $e->name->getLast();
        $t = $this->inferred($e);
        // literal constants known to Psalm can be inlined
        $pt = $this->psalmType($e);
        if ($pt !== null && $pt->isSingle()) {
            $atomic = $pt->getSingleAtomic();
            if ($atomic instanceof TLiteralInt) {
                return new Val($atomic->value . 'i64', RustType::int());
            }
            if ($atomic instanceof TLiteralString && strlen($atomic->value) < 200) {
                return new Val(Names::strLit($atomic->value), RustType::str());
            }
        }
        if ($this->builtins->hasConstant($short)) {
            return new Val('consts::' . $short, $this->builtins->constantType($short));
        }
        $user = $this->program->getConstant($resolved) ?? $this->program->getConstant($short);
        if ($user !== null) {
            return new Val('crate::consts::' . Names::constant($user->name) . '()', $user->type);
        }
        $this->warn('unknown constant ' . $resolved, $e);
        return new Val('unreachable!("unknown constant ' . $resolved . '")', $t ?? RustType::mixed());
    }

    private function variable(Expr\Variable $e): Val
    {
        if (!is_string($e->name)) {
            $this->warn('variable variable', $e);
            return new Val('unreachable!("variable variable")', RustType::mixed());
        }
        $v = $this->readVar($e->name);
        return $this->narrow($v, $e);
    }

    /** Convert a read value to the type Psalm inferred for the expression. */
    public function narrow(Val $v, Expr $e): Val
    {
        $inf = $this->inferred($e);
        if ($inf === null || $inf->kind === RustType::NEVER || $inf->kind === RustType::MIXED && $v->type->kind !== RustType::MIXED) {
            return $v;
        }
        if ($inf->toRust() === $v->type->toRust()) {
            return $v;
        }
        // never widen a concrete value into a union/option just because Psalm says so; only narrow
        if ($this->isWidening($v->type, $inf)) {
            return $v;
        }
        return new Val($this->casts->convert($v->code, $v->type, $inf), $inf);
    }

    /** True when converting from => to only adds information (Option wrapping, union wrapping, upcast). */
    private function isWidening(RustType $from, RustType $to): bool
    {
        if ($to->kind === RustType::OPTION && $from->kind !== RustType::OPTION) {
            return true;
        }
        if ($to->kind === RustType::UNION && $from->kind !== RustType::UNION) {
            return $this->casts->pickMember($to, $from) !== null;
        }
        if ($to->kind === RustType::CLASS_ && $from->kind === RustType::CLASS_) {
            $fc = $this->program->classOf($from);
            $tc = $this->program->classOf($to);
            return $fc !== null && $tc !== null && $fc !== $tc && $fc->isSubclassOf($tc);
        }
        if ($to->kind === RustType::ANY_OBJECT && $from->kind === RustType::CLASS_) {
            return true;
        }
        return false;
    }

    // ------------------------------------------------------------------ arrays

    private function arrayLiteral(Expr\Array_ $e, ?RustType $expected): Val
    {
        $target = $expected;
        $inf = $this->inferred($e);
        if ($target === null || !in_array($target->kind, [RustType::LIST, RustType::MAP, RustType::TUPLE, RustType::SHAPE], true)) {
            $target = $inf;
        }
        if ($target === null || $target->kind === RustType::MIXED) {
            $target = RustType::map(RustType::arrayKey(), RustType::mixed());
        }
        if ($target->kind === RustType::OPTION) {
            $target = $target->inner();
        }
        $has_spread = false;
        $has_keys = false;
        foreach ($e->items as $item) {
            if ($item->unpack) {
                $has_spread = true;
            }
            if ($item->key !== null) {
                $has_keys = true;
            }
        }

        if ($target->kind === RustType::TUPLE && !$has_spread && !$has_keys && count($e->items) === count($target->params)) {
            $parts = [];
            foreach ($e->items as $i => $item) {
                $parts[] = $this->exprTo($item->value, $target->params[$i]);
            }
            return new Val('(' . implode(', ', $parts) . (count($parts) === 1 ? ',' : '') . ')', $target);
        }
        if ($target->kind === RustType::SHAPE && !$has_spread) {
            $fields = [];
            $seen = [];
            foreach ($e->items as $i => $item) {
                $key = $item->key !== null ? $this->literalKey($item->key) : (string) $i;
                if ($key === null) {
                    return $this->arrayLiteral($e, RustType::map(RustType::arrayKey(), $this->shapeValueType($target)));
                }
                if (!isset($target->fields[$key])) {
                    return $this->arrayLiteral($e, RustType::map(
                        $this->allIntKeys($target) ? RustType::int() : RustType::arrayKey(),
                        $this->shapeValueType($target),
                    ));
                }
                [$ft, $opt] = $target->fields[$key];
                $code = $this->exprTo($item->value, $ft);
                $fields[Names::field($key)] = $opt ? $this->casts->convert($code, $ft, RustType::option($ft)) : $code;
                $seen[$key] = true;
            }
            foreach ($target->fields as $k => [$ft, $opt]) {
                if (!isset($seen[$k])) {
                    $fields[Names::field($k)] = $opt ? 'None' : $this->casts->defaultOf($ft);
                }
            }
            $parts = [];
            foreach ($fields as $k => $v) {
                $parts[] = $k . ': ' . $v;
            }
            return new Val($target->mangle() . ' { ' . implode(', ', $parts) . ' }', $target);
        }
        if ($target->kind === RustType::LIST && !$has_keys) {
            $elem = $target->inner();
            if (!$has_spread) {
                $parts = [];
                foreach ($e->items as $item) {
                    $parts[] = $this->exprTo($item->value, $elem);
                }
                if (count($parts) === 0) {
                    return new Val('List::<' . $elem->toRust() . '>::new()', $target);
                }
                return new Val('list![' . implode(', ', $parts) . ']', $target);
            }
            $tmp = $this->tmp('__l');
            $code = '{ let mut ' . $tmp . ': ' . $target->toRust() . ' = List::new(); ';
            foreach ($e->items as $item) {
                if ($item->unpack) {
                    $sv = $this->expr($item->value);
                    $code .= $tmp . '.extend(' . $this->casts->convert($sv->code, $sv->type, RustType::list($elem)) . '.into_iter()); ';
                } else {
                    $code .= $tmp . '.push(' . $this->exprTo($item->value, $elem) . '); ';
                }
            }
            return new Val($code . $tmp . ' }', $target);
        }
        if ($target->kind === RustType::LIST) {
            $target = RustType::map(RustType::arrayKey(), $target->inner());
        }
        if ($target->kind === RustType::TUPLE || $target->kind === RustType::SHAPE) {
            $target = RustType::map(RustType::arrayKey(), $target->kind === RustType::TUPLE ? $this->types()->combine($target->params) : $this->shapeValueType($target));
        }
        // map
        [$kt, $vt] = $target->params;
        if (count($e->items) === 0) {
            return new Val('Map::<' . $kt->toRust() . ', ' . $vt->toRust() . '>::new()', $target);
        }
        $tmp = $this->tmp('__m');
        $code = '{ let mut ' . $tmp . ': ' . $target->toRust() . ' = Map::new(); ';
        foreach ($e->items as $item) {
            if ($item->unpack) {
                $sv = $this->expr($item->value);
                $code .= 'for (__k, __v) in ' . $this->casts->convert($sv->code, $sv->type, $target) . '.into_iter() { if __k.int_value().is_some() { ' . $tmp . '.push(__v); } else { ' . $tmp . '.insert(__k, __v); } } ';
            } elseif ($item->key === null) {
                $code .= $tmp . '.push(' . $this->exprTo($item->value, $vt) . '); ';
            } else {
                $code .= $tmp . '.insert(' . $this->keyExpr($item->key, $kt) . ', ' . $this->exprTo($item->value, $vt) . '); ';
            }
        }
        return new Val($code . $tmp . ' }', $target);
    }

    private function shapeValueType(RustType $shape): RustType
    {
        $types = [];
        foreach ($shape->fields as [$t, $opt]) {
            $types[] = $t;
        }
        return $this->types()->combine($types);
    }

    private function allIntKeys(RustType $shape): bool
    {
        foreach ($shape->fields as $k => $_) {
            if ((string) (int) $k !== (string) $k) {
                return false;
            }
        }
        return true;
    }

    /** The literal string/int key of an array item, if statically known. */
    public function literalKey(Expr $key): ?string
    {
        if ($key instanceof Scalar\String_) {
            return $key->value;
        }
        if ($key instanceof Scalar\Int_) {
            return (string) $key->value;
        }
        $t = $this->psalmType($key);
        if ($t !== null && $t->isSingleStringLiteral()) {
            return $t->getSingleStringLiteral()->value;
        }
        if ($t !== null && $t->isSingleIntLiteral()) {
            return (string) $t->getSingleIntLiteral()->value;
        }
        return null;
    }

    /** Emit a key expression converted to the map's key type (owned). */
    public function keyExpr(Expr $key, RustType $kt): string
    {
        $v = $this->expr($key);
        return $this->keyFrom($v, $kt);
    }

    public function keyFrom(Val $v, RustType $kt): string
    {
        $ft = $v->type;
        if ($ft->toRust() === $kt->toRust()) {
            return $v->code;
        }
        if ($kt->kind === RustType::ARRAY_KEY) {
            if ($ft->kind === RustType::INT || $ft->kind === RustType::STR || $ft->kind === RustType::BOOL || $ft->kind === RustType::FLOAT) {
                return 'to_key(&' . $v->code . ')';
            }
            if ($ft->kind === RustType::OPTION) {
                return 'to_key(&' . $v->code . ')';
            }
            return 'to_key(&' . $this->casts->convert($v->code, $ft, RustType::mixed()) . ')';
        }
        if ($kt->kind === RustType::INT) {
            if ($ft->kind === RustType::ARRAY_KEY || $ft->kind === RustType::STR || $ft->kind === RustType::FLOAT || $ft->kind === RustType::BOOL) {
                return 'to_int(&' . $v->code . ')';
            }
            return $this->casts->convert($v->code, $ft, $kt);
        }
        if ($kt->kind === RustType::STR) {
            if ($ft->kind === RustType::ARRAY_KEY || $ft->kind === RustType::INT) {
                return 'to_str(&' . $v->code . ')';
            }
            return $this->casts->convert($v->code, $ft, $kt);
        }
        return $this->casts->convert($v->code, $ft, $kt);
    }

    // ------------------------------------------------------------------ operators

    private function binaryOp(BinaryOp $e): Val
    {
        if ($e instanceof BinaryOp\Concat) {
            $l = $this->exprTo($e->left, RustType::str());
            $r = $this->exprTo($e->right, RustType::str());
            return new Val('concat(' . $l . ', ' . $r . ')', RustType::str());
        }
        if ($e instanceof BinaryOp\BooleanAnd || $e instanceof BinaryOp\LogicalAnd) {
            return new Val('(' . $this->truthy($e->left) . ' && ' . $this->truthy($e->right) . ')', RustType::bool());
        }
        if ($e instanceof BinaryOp\BooleanOr || $e instanceof BinaryOp\LogicalOr) {
            return new Val('(' . $this->truthy($e->left) . ' || ' . $this->truthy($e->right) . ')', RustType::bool());
        }
        if ($e instanceof BinaryOp\LogicalXor) {
            return new Val('(' . $this->truthy($e->left) . ' ^ ' . $this->truthy($e->right) . ')', RustType::bool());
        }
        if ($e instanceof BinaryOp\Coalesce) {
            return $this->coalesce($e);
        }
        if ($e instanceof BinaryOp\Identical || $e instanceof BinaryOp\NotIdentical) {
            $code = $this->identicalCode($e->left, $e->right);
            return new Val($e instanceof BinaryOp\Identical ? $code : '(!' . $code . ')', RustType::bool());
        }
        if ($e instanceof BinaryOp\Equal || $e instanceof BinaryOp\NotEqual) {
            $code = $this->looseEqCode($e->left, $e->right);
            return new Val($e instanceof BinaryOp\Equal ? $code : '(!' . $code . ')', RustType::bool());
        }
        if ($e instanceof BinaryOp\Smaller || $e instanceof BinaryOp\SmallerOrEqual || $e instanceof BinaryOp\Greater
            || $e instanceof BinaryOp\GreaterOrEqual || $e instanceof BinaryOp\Spaceship
        ) {
            [$l, $r, $t] = $this->commonOperands($e->left, $e->right, true);
            $fn = match (true) {
                $e instanceof BinaryOp\Smaller => 'php_lt',
                $e instanceof BinaryOp\SmallerOrEqual => 'php_le',
                $e instanceof BinaryOp\Greater => 'php_gt',
                $e instanceof BinaryOp\GreaterOrEqual => 'php_ge',
                default => 'spaceship',
            };
            if ($t->kind === RustType::INT || $t->kind === RustType::FLOAT) {
                $op = match ($fn) {
                    'php_lt' => '<',
                    'php_le' => '<=',
                    'php_gt' => '>',
                    'php_ge' => '>=',
                    default => null,
                };
                if ($op !== null) {
                    return new Val('(' . $l . ' ' . $op . ' ' . $r . ')', RustType::bool());
                }
            }
            $ret = $fn === 'spaceship' ? RustType::int() : RustType::bool();
            return new Val($fn . '(&' . $l . ', &' . $r . ')', $ret);
        }
        if ($e instanceof BinaryOp\Plus || $e instanceof BinaryOp\Minus || $e instanceof BinaryOp\Mul
            || $e instanceof BinaryOp\Div || $e instanceof BinaryOp\Mod || $e instanceof BinaryOp\Pow
        ) {
            return $this->arith($e);
        }
        if ($e instanceof BinaryOp\BitwiseAnd || $e instanceof BinaryOp\BitwiseOr || $e instanceof BinaryOp\BitwiseXor
            || $e instanceof BinaryOp\ShiftLeft || $e instanceof BinaryOp\ShiftRight
        ) {
            $lt = $this->inferredOrMixed($e->left);
            if (($e instanceof BinaryOp\BitwiseAnd || $e instanceof BinaryOp\BitwiseOr || $e instanceof BinaryOp\BitwiseXor) && $lt->kind === RustType::STR) {
                $fn = $e instanceof BinaryOp\BitwiseAnd ? 'str_bit_and' : ($e instanceof BinaryOp\BitwiseOr ? 'str_bit_or' : 'str_bit_xor');
                return new Val($fn . '(&' . $this->exprTo($e->left, RustType::str()) . ', &' . $this->exprTo($e->right, RustType::str()) . ')', RustType::str());
            }
            $l = $this->exprTo($e->left, RustType::int());
            $r = $this->exprTo($e->right, RustType::int());
            $code = match (true) {
                $e instanceof BinaryOp\BitwiseAnd => '(' . $l . ' & ' . $r . ')',
                $e instanceof BinaryOp\BitwiseOr => '(' . $l . ' | ' . $r . ')',
                $e instanceof BinaryOp\BitwiseXor => '(' . $l . ' ^ ' . $r . ')',
                $e instanceof BinaryOp\ShiftLeft => '(' . $l . ').wrapping_shl((' . $r . ') as u32)',
                default => '(' . $l . ').wrapping_shr((' . $r . ') as u32)',
            };
            return new Val($code, RustType::int());
        }
        $this->warn('unsupported binary op ' . $e->getOperatorSigil(), $e);
        return new Val('unreachable!()', RustType::never());
    }

    private function arith(BinaryOp $e): Val
    {
        $lt = $this->inferredOrMixed($e->left);
        $rt = $this->inferredOrMixed($e->right);
        $res = $this->inferredOrMixed($e);
        $sig = $e->getOperatorSigil();

        // array union
        if ($sig === '+' && ($lt->kind === RustType::MAP || $lt->kind === RustType::LIST || $lt->kind === RustType::SHAPE || $lt->kind === RustType::TUPLE)) {
            $target = $res->kind === RustType::MAP ? $res : RustType::map(RustType::arrayKey(), RustType::mixed());
            if ($res->kind !== RustType::MAP) {
                $target = $lt->kind === RustType::MAP ? $lt : RustType::map(RustType::arrayKey(), RustType::mixed());
            }
            $l = $this->exprTo($e->left, $target);
            $r = $this->exprTo($e->right, $target);
            return new Val('array_union(&' . $l . ', &' . $r . ')', $target);
        }

        $int_ok = fn(RustType $t) => $t->kind === RustType::INT || $t->kind === RustType::BOOL;
        if ($sig === '/') {
            if ($res->kind === RustType::INT && $int_ok($lt) && $int_ok($rt)) {
                return new Val('div_i(' . $this->exprTo($e->left, RustType::int()) . ', ' . $this->exprTo($e->right, RustType::int()) . ')?', RustType::int());
            }
            if ($res->kind === RustType::FLOAT) {
                return new Val('div_f(' . $this->exprTo($e->left, RustType::float()) . ', ' . $this->exprTo($e->right, RustType::float()) . ')?', RustType::float());
            }
            $l = $this->numOperand($e->left);
            $r = $this->numOperand($e->right);
            $num = 'div(' . $l . ', ' . $r . ')?';
            return $this->numResult($num, $res);
        }
        if ($sig === '%') {
            return new Val('imod(' . $this->exprTo($e->left, RustType::int()) . ', ' . $this->exprTo($e->right, RustType::int()) . ')?', RustType::int());
        }
        if ($sig === '**') {
            if ($res->kind === RustType::INT && $int_ok($lt) && $int_ok($rt)) {
                return new Val('pow_i(' . $this->exprTo($e->left, RustType::int()) . ', ' . $this->exprTo($e->right, RustType::int()) . ').to_i64()', RustType::int());
            }
            if ($res->kind === RustType::FLOAT) {
                return new Val('pow_f(' . $this->exprTo($e->left, RustType::float()) . ', ' . $this->exprTo($e->right, RustType::float()) . ')', RustType::float());
            }
            return $this->numResult('pow_i(' . $this->exprTo($e->left, RustType::int()) . ', ' . $this->exprTo($e->right, RustType::int()) . ')', $res);
        }
        // + - *
        if ($int_ok($lt) && $int_ok($rt) && $res->kind !== RustType::FLOAT) {
            $fn = match ($sig) {
                '+' => 'wrapping_add',
                '-' => 'wrapping_sub',
                default => 'wrapping_mul',
            };
            return new Val('(' . $this->exprTo($e->left, RustType::int()) . ').' . $fn . '(' . $this->exprTo($e->right, RustType::int()) . ')', RustType::int());
        }
        if (($lt->kind === RustType::FLOAT || $lt->kind === RustType::INT || $lt->kind === RustType::BOOL)
            && ($rt->kind === RustType::FLOAT || $rt->kind === RustType::INT || $rt->kind === RustType::BOOL)
        ) {
            $op = $sig;
            return new Val('(' . $this->exprTo($e->left, RustType::float()) . ' ' . $op . ' ' . $this->exprTo($e->right, RustType::float()) . ')', RustType::float());
        }
        // numeric strings / mixed / unions
        $l = $this->numOperand($e->left);
        $r = $this->numOperand($e->right);
        $fn = match ($sig) {
            '+' => 'num_add',
            '-' => 'num_sub',
            default => 'num_mul',
        };
        return $this->numResult($fn . '(' . $l . ', ' . $r . ')', $res);
    }

    /** Emit an operand as a runtime `Num`. */
    private function numOperand(Expr $e): string
    {
        $v = $this->expr($e);
        return match ($v->type->kind) {
            RustType::INT => 'Num::Int(' . $v->code . ')',
            RustType::FLOAT => 'Num::Float(' . $v->code . ')',
            RustType::BOOL => 'Num::Int(' . $v->code . ' as i64)',
            default => 'to_num(&' . $this->casts->convert($v->code, $v->type, RustType::mixed()) . ')',
        };
    }

    private function numResult(string $num_code, RustType $res): Val
    {
        if ($res->kind === RustType::INT) {
            return new Val($num_code . '.to_i64()', RustType::int());
        }
        if ($res->kind === RustType::FLOAT) {
            return new Val($num_code . '.to_f64()', RustType::float());
        }
        if ($res->kind === RustType::UNION) {
            $this->casts->need(RustType::rtGeneric('Num', []), $res);
            return new Val('cast::<' . $res->toRust() . '>(' . $num_code . ')', $res);
        }
        return new Val($num_code . '.to_mixed()', RustType::mixed());
    }

    /**
     * Emit both operands converted to a common type for comparison.
     *
     * @return array{string, string, RustType}
     */
    private function commonOperands(Expr $left, Expr $right, bool $numeric): array
    {
        $l = $this->expr($left);
        $r = $this->expr($right);
        $t = $this->commonType($l->type, $r->type, $numeric);
        return [$this->casts->convert($l->code, $l->type, $t), $this->casts->convert($r->code, $r->type, $t), $t];
    }

    private function commonType(RustType $a, RustType $b, bool $numeric): RustType
    {
        if ($a->toRust() === $b->toRust()) {
            return $a;
        }
        $num = fn(RustType $t) => in_array($t->kind, [RustType::INT, RustType::FLOAT, RustType::BOOL], true);
        if ($num($a) && $num($b)) {
            return ($a->kind === RustType::FLOAT || $b->kind === RustType::FLOAT) ? RustType::float() : RustType::int();
        }
        if ($a->kind === RustType::OPTION && $a->inner()->toRust() === $b->toRust()) {
            return $a;
        }
        if ($b->kind === RustType::OPTION && $b->inner()->toRust() === $a->toRust()) {
            return $b;
        }
        if ($a->kind === RustType::UNIT && $b->kind !== RustType::MIXED) {
            return RustType::option($b);
        }
        if ($b->kind === RustType::UNIT && $a->kind !== RustType::MIXED) {
            return RustType::option($a);
        }
        if ($a->kind === RustType::UNION && $this->casts->pickMember($a, $b) !== null) {
            return $a;
        }
        if ($b->kind === RustType::UNION && $this->casts->pickMember($b, $a) !== null) {
            return $b;
        }
        if ($a->kind === RustType::OPTION && $a->inner()->kind === RustType::UNION && $this->casts->pickMember($a->inner(), $b) !== null) {
            return $a;
        }
        if ($b->kind === RustType::OPTION && $b->inner()->kind === RustType::UNION && $this->casts->pickMember($b->inner(), $a) !== null) {
            return $b;
        }
        if ($a->kind === RustType::CLASS_ && $b->kind === RustType::CLASS_) {
            $ca = $this->program->classOf($a);
            $cb = $this->program->classOf($b);
            if ($ca !== null && $cb !== null) {
                if ($ca->isSubclassOf($cb)) {
                    return $b;
                }
                if ($cb->isSubclassOf($ca)) {
                    return $a;
                }
            }
            return RustType::anyObject();
        }
        if (($a->kind === RustType::ARRAY_KEY && ($b->kind === RustType::INT || $b->kind === RustType::STR))
            || ($b->kind === RustType::ARRAY_KEY && ($a->kind === RustType::INT || $a->kind === RustType::STR))
        ) {
            return RustType::arrayKey();
        }
        if (($a->kind === RustType::STR && $num($b)) || ($b->kind === RustType::STR && $num($a))) {
            return RustType::mixed();
        }
        if ($a->kind === RustType::LIST && $b->kind === RustType::MAP) {
            return $b;
        }
        if ($b->kind === RustType::LIST && $a->kind === RustType::MAP) {
            return $a;
        }
        return RustType::mixed();
    }

    /** `$a === $b` */
    private function identicalCode(Expr $left, Expr $right): string
    {
        // comparisons with null literal
        if ($this->isNullLiteral($right) || $this->isNullLiteral($left)) {
            $other = $this->isNullLiteral($right) ? $left : $right;
            $v = $this->expr($other);
            $ov = $this->optionalValue($other);
            // a null stored where a callable is declared (docblock `callable[]` holding nulls)
            if ($ov !== null && $ov->type->inner()->kind === RustType::DYN_CALLABLE) {
                return '(match &' . $ov->code . ' { None => true, Some(__c) => __c.is_null() })';
            }
            if ($ov === null && $v->type->kind === RustType::DYN_CALLABLE) {
                return $v->code . '.is_null()';
            }
            if ($ov === null && $v->type->kind === RustType::OPTION && $v->type->inner()->kind === RustType::DYN_CALLABLE) {
                return '(match &' . $v->code . ' { None => true, Some(__c) => __c.is_null() })';
            }
            if ($ov !== null) {
                return $ov->code . '.is_none()';
            }
            if ($v->type->kind === RustType::OPTION) {
                return $v->code . '.is_none()';
            }
            if ($v->type->kind === RustType::UNIT) {
                return '{ let _ = ' . $v->code . '; true }';
            }
            if ($v->type->kind === RustType::MIXED) {
                return $v->code . '.is_null()';
            }
            return '{ let _ = ' . $v->code . '; false }';
        }
        // comparisons with bool literals against unions holding True/False variants
        foreach ([[$left, $right], [$right, $left]] as [$a, $b]) {
            if ($b instanceof Expr\ConstFetch && in_array(strtolower($b->name->toString()), ['true', 'false'], true)) {
                $v = $this->expr($a);
                $lit = strtolower($b->name->toString()) === 'true';
                $t = $v->type;
                $inner = $t->kind === RustType::OPTION ? $t->inner() : $t;
                if ($inner->kind === RustType::UNION && $this->casts->hasUnit($inner, $lit ? 'True' : 'False') && !$this->unionHasBool($inner)) {
                    $pat = $inner->mangle() . '::' . ($lit ? 'True' : 'False');
                    if ($t->kind === RustType::OPTION) {
                        return 'matches!(' . $v->code . ', Some(' . $pat . '))';
                    }
                    return 'matches!(' . $v->code . ', ' . $pat . ')';
                }
                if ($t->kind === RustType::BOOL) {
                    return '(' . $v->code . ' == ' . ($lit ? 'true' : 'false') . ')';
                }
                if ($t->kind === RustType::OPTION && $inner->kind === RustType::BOOL) {
                    return '(' . $v->code . ' == Some(' . ($lit ? 'true' : 'false') . '))';
                }
                if ($t->kind === RustType::OPTION && $inner->kind !== RustType::MIXED && $inner->kind !== RustType::UNION) {
                    // `T|false` results (e.g. strpos) are represented as Option<T>: `None` is the `false`
                    return $lit ? '{ let _ = ' . $v->code . '; false }' : $v->code . '.is_none()';
                }
                if ($t->kind === RustType::MIXED) {
                    return 'matches!(' . $v->code . ', Mixed::Bool(' . ($lit ? 'true' : 'false') . '))';
                }
            }
        }
        [$l, $r, $t] = $this->commonOperands($left, $right, false);
        if ($t->isCopy() && $t->kind !== RustType::OPTION) {
            return '(' . $l . ' == ' . $r . ')';
        }
        return 'identical(&' . $l . ', &' . $r . ')';
    }

    private function unionHasBool(RustType $u): bool
    {
        foreach ($u->params as $m) {
            if ($m->kind === RustType::BOOL) {
                return true;
            }
        }
        return false;
    }

    private function looseEqCode(Expr $left, Expr $right): string
    {
        if ($this->isNullLiteral($right) || $this->isNullLiteral($left)) {
            $other = $this->isNullLiteral($right) ? $left : $right;
            return '(!' . $this->truthy($other) . ')';
        }
        [$l, $r, $t] = $this->commonOperands($left, $right, true);
        if ($t->kind === RustType::INT || $t->kind === RustType::FLOAT || $t->kind === RustType::BOOL) {
            return '(' . $l . ' == ' . $r . ')';
        }
        return 'loose_eq(&' . $l . ', &' . $r . ')';
    }

    public function isNullLiteral(Expr $e): bool
    {
        return $e instanceof Expr\ConstFetch && strtolower($e->name->toString()) === 'null';
    }

    private function coalesce(BinaryOp\Coalesce $e): Val
    {
        $res = $this->inferredOrMixed($e);
        $left = $this->optionalValue($e->left);
        if ($left === null) {
            $lv = $this->expr($e->left);
            if ($lv->type->kind === RustType::OPTION) {
                $left = $lv;
            } elseif ($lv->type->kind === RustType::MIXED) {
                $left = new Val($lv->code . '.to_option()', RustType::option(RustType::mixed()));
            } else {
                // never null: the right side is dead
                return $lv;
            }
        }
        $inner = $left->type->inner();
        $target = $res->kind === RustType::MIXED && $inner->kind !== RustType::MIXED ? $this->commonType($inner, $this->inferredOrMixed($e->right), false) : $res;
        if ($target->kind === RustType::MIXED && $inner->kind !== RustType::MIXED) {
            $target = $inner;
        }
        if ($e->right instanceof Expr\Throw_) {
            return new Val('(match ' . $left->code . ' { Some(__v) => ' . $this->casts->convert('__v', $inner, $target) . ', None => return Err(' . $this->throwValue($e->right->expr) . ') })', $target);
        }
        $right = $this->exprTo($e->right, $target);
        return new Val('(match ' . $left->code . ' { Some(__v) => ' . $this->casts->convert('__v', $inner, $target) . ', None => ' . $right . ' })', $target);
    }

    /**
     * Emit an expression as an Option (None when unset/null); null when the expression cannot be unset.
     */
    public function optionalValue(Expr $e): ?Val
    {
        if ($e instanceof Expr\Variable && is_string($e->name)) {
            $v = $this->readVar($e->name);
            if ($v->type->kind === RustType::OPTION) {
                return $v;
            }
            if ($v->type->kind === RustType::MIXED) {
                return new Val($v->code . '.to_option()', RustType::option(RustType::mixed()));
            }
            return null;
        }
        if ($e instanceof Expr\ArrayDimFetch && $e->dim !== null) {
            // the base is read with its declared (not isset-narrowed) type: inside `??`/isset Psalm
            // narrows optional shape fields to present ones, which must not become unwraps
            $base = $this->optionalValue($e->var)
                ?? $this->asOption($e->var instanceof Expr\Variable && is_string($e->var->name) ? $this->readVar($e->var->name) : $this->expr($e->var));
            $bt = $base->type->inner();
            $dim = $e->dim;
            if ($bt->kind === RustType::LIST) {
                $inner = $bt->inner();
                $code = '{ let __k = ' . $this->exprTo($dim, RustType::int()) . '; ' . $base->code . '.and_then(|__b| __b.get(__k).cloned()) }';
                return $this->flattenOption($code, $inner);
            }
            if ($bt->kind === RustType::MAP) {
                [$kt, $vt] = $bt->params;
                $code = '{ let __k = ' . $this->keyExpr($dim, $kt) . '; ' . $base->code . '.and_then(|__b| __b.get(&__k).cloned()) }';
                return $this->flattenOption($code, $vt);
            }
            if ($bt->kind === RustType::SHAPE) {
                $key = $this->literalKey($dim);
                if ($key !== null && isset($bt->fields[$key])) {
                    [$ft, $opt] = $bt->fields[$key];
                    $code = $base->code . '.and_then(|__b| ' . ($opt ? '__b.' . Names::field($key) : 'Some(__b.' . Names::field($key) . ')') . ')';
                    if ($opt && $ft->kind === RustType::OPTION) {
                        // optional nullable field: stored as a single (collapsed) Option
                        return new Val($code, $ft);
                    }
                    return $this->flattenOption($code, $ft);
                }
                if ($key !== null) {
                    return new Val('{ let _ = ' . $base->code . '; None::<Mixed> }', RustType::option(RustType::mixed()));
                }
                $mt = RustType::map(RustType::arrayKey(), $this->shapeValueType($bt));
                $code = '{ let __k = ' . $this->keyExpr($dim, RustType::arrayKey()) . '; ' . $base->code . '.and_then(|__b| ' . $this->casts->convert('__b', $bt, $mt) . '.get(&__k).cloned()) }';
                return $this->flattenOption($code, $mt->params[1]);
            }
            if ($bt->kind === RustType::TUPLE) {
                $key = $this->literalKey($dim);
                if ($key !== null && isset($bt->params[(int) $key])) {
                    $ft = $bt->params[(int) $key];
                    return $this->flattenOption($base->code . '.map(|__b| __b.' . (int) $key . ')', $ft);
                }
                return new Val('{ let _ = ' . $base->code . '; None::<Mixed> }', RustType::option(RustType::mixed()));
            }
            if ($bt->kind === RustType::MIXED) {
                $k = $this->expr($dim);
                return new Val('{ let __k = ' . $this->keyFrom($k, RustType::arrayKey()) . '; ' . $base->code . '.and_then(|__b| mixed_get(&__b, &__k)) }', RustType::option(RustType::mixed()));
            }
            if ($bt->kind === RustType::STR) {
                return new Val('{ let __k = ' . $this->exprTo($dim, RustType::int()) . '; ' . $base->code . '.and_then(|__b| str_index_opt(&__b, __k)) }', RustType::option(RustType::str()));
            }
            if ($bt->kind === RustType::CLASS_) {
                // ArrayAccess object
                $k = $this->expr($dim);
                $cls = $this->program->classOf($bt);
                $m = $cls !== null ? $this->program->findMethod($cls, 'offsetget') : null;
                if ($m !== null) {
                    $code = '(match ' . $base->code . ' { Some(__b) => { if __b.offsetExists(' . $this->casts->convert($k->code, $k->type, $this->program->findMethod($cls, 'offsetexists')->param_types[0] ?? RustType::mixed()) . ')? { Some(__b.offsetGet(' . $this->casts->convert($k->code, $k->type, $m->param_types[0] ?? RustType::mixed()) . ')?) } else { None } } None => None })';
                    return $this->flattenOption($code, $m->return_type);
                }
            }
            $this->warn('isset on unsupported base type ' . $bt->toRust(), $e);
            return new Val('{ let _ = ' . $base->code . '; None::<Mixed> }', RustType::option(RustType::mixed()));
        }
        if ($e instanceof Expr\PropertyFetch || $e instanceof Expr\NullsafePropertyFetch) {
            if (!$e->name instanceof Identifier) {
                return null;
            }
            $base = $this->optionalValue($e->var) ?? $this->asOption($this->expr($e->var));
            $bt = $base->type->inner();
            $name = $e->name->name;
            if ($bt->kind === RustType::CLASS_) {
                $cls = $this->program->classOf($bt);
                $field = $cls?->fields[$name] ?? null;
                if ($field !== null) {
                    $getter = $field->isLate()
                        ? '__b.' . $field->acc() . '_opt()'
                        : 'Some(__b.' . $field->acc() . '_get())';
                    return $this->flattenOption($base->code . '.and_then(|__b| ' . $getter . ')', $field->type);
                }
            }
            if ($bt->kind === RustType::MIXED) {
                return new Val($base->code . '.and_then(|__b| mixed_prop(&__b, &' . Names::strLit($name) . '))', RustType::option(RustType::mixed()));
            }
            if ($bt->kind === RustType::RT_GENERIC && $bt->name === 'StdClass') {
                return new Val($base->code . '.and_then(|__b| __b.get(' . Names::strLit($name) . '))', RustType::option(RustType::mixed()));
            }
            return null;
        }
        if ($e instanceof Expr\StaticPropertyFetch) {
            $v = $this->expr($e);
            if ($v->type->kind === RustType::OPTION) {
                return $v;
            }
            return null;
        }
        if ($e instanceof Expr\ClassConstFetch) {
            return null;
        }
        if ($e instanceof Expr\MethodCall || $e instanceof Expr\NullsafeMethodCall || $e instanceof Expr\FuncCall || $e instanceof Expr\StaticCall) {
            $v = $this->expr($e);
            if ($v->type->kind === RustType::OPTION) {
                return $v;
            }
            if ($v->type->kind === RustType::MIXED) {
                return new Val($v->code . '.to_option()', RustType::option(RustType::mixed()));
            }
            return null;
        }
        return null;
    }

    private function asOption(Val $v): Val
    {
        if ($v->type->kind === RustType::OPTION) {
            return $v;
        }
        if ($v->type->kind === RustType::MIXED) {
            return new Val($v->code . '.to_option()', RustType::option(RustType::mixed()));
        }
        return new Val('Some(' . $v->code . ')', RustType::option($v->type));
    }

    /** `Option<Option<T>>` code => `Option<T>` */
    private function flattenOption(string $code, RustType $inner): Val
    {
        if ($inner->kind === RustType::OPTION) {
            return new Val($code . '.flatten()', $inner);
        }
        if ($inner->kind === RustType::MIXED) {
            return new Val($code . '.and_then(|__m| __m.to_option())', RustType::option(RustType::mixed()));
        }
        return new Val($code, RustType::option($inner));
    }

    /** `isset($x)` */
    private function issetCode(Expr $e): string
    {
        $ov = $this->optionalValue($e);
        if ($ov !== null) {
            return $ov->code . '.is_some()';
        }
        $v = $this->expr($e);
        if ($v->type->kind === RustType::OPTION) {
            return $v->code . '.is_some()';
        }
        if ($v->type->kind === RustType::MIXED) {
            return '(!' . $v->code . '.is_null())';
        }
        return '{ let _ = ' . $v->code . '; true }';
    }

    private function incDec(Expr $e): Val
    {
        $is_inc = $e instanceof Expr\PreInc || $e instanceof Expr\PostInc;
        $is_pre = $e instanceof Expr\PreInc || $e instanceof Expr\PreDec;
        $place = $this->place($e->var);
        $t = $place->type;
        $tmp = $this->tmp();
        if ($t->kind === RustType::INT) {
            $op = $is_inc ? 'wrapping_add(1)' : 'wrapping_sub(1)';
            return new Val('{ let ' . $tmp . ' = ' . $place->read() . '; ' . $place->write($tmp . '.' . $op) . ' ' . ($is_pre ? $tmp . '.' . $op : $tmp) . ' }', RustType::int());
        }
        if ($t->kind === RustType::FLOAT) {
            $op = $is_inc ? ' + 1.0' : ' - 1.0';
            return new Val('{ let ' . $tmp . ' = ' . $place->read() . '; ' . $place->write($tmp . $op) . ' ' . ($is_pre ? $tmp . $op : $tmp) . ' }', RustType::float());
        }
        if ($t->kind === RustType::STR && $is_inc) {
            return new Val('{ let ' . $tmp . ' = ' . $place->read() . '; let __n = str_increment(&' . $tmp . '); ' . $place->write('__n.clone()') . ' ' . ($is_pre ? '__n' : $tmp) . ' }', RustType::str());
        }
        if ($t->kind === RustType::OPTION && $t->inner()->kind === RustType::INT) {
            $op = $is_inc ? 'wrapping_add(1)' : 'wrapping_sub(1)';
            return new Val('{ let ' . $tmp . ' = ' . $place->read() . '.unwrap_or(0); ' . $place->write('Some(' . $tmp . '.' . $op . ')') . ' ' . ($is_pre ? $tmp . '.' . $op : $tmp) . ' }', RustType::int());
        }
        if ($t->kind === RustType::MIXED || $t->kind === RustType::UNION || $t->kind === RustType::ARRAY_KEY) {
            $fn = $is_inc ? 'mixed_inc' : 'mixed_dec';
            return new Val('{ let ' . $tmp . ' = ' . $this->casts->convert($place->read(), $t, RustType::mixed()) . '; let __n = ' . $fn . '(&' . $tmp . '); ' . $place->write($this->casts->convert('__n.clone()', RustType::mixed(), $t)) . ' ' . ($is_pre ? '__n' : $tmp) . ' }', RustType::mixed());
        }
        $this->warn('inc/dec on ' . $t->toRust(), $e);
        return new Val('unreachable!("inc/dec on ' . $t->toRust() . '")', RustType::never());
    }

    private function cast(Expr\Cast $e): Val
    {
        $v = $this->expr($e->expr);
        $t = $v->type;
        if ($e instanceof Expr\Cast\Int_) {
            if ($t->kind === RustType::INT) {
                return $v;
            }
            return new Val('to_int(&' . $v->code . ')', RustType::int());
        }
        if ($e instanceof Expr\Cast\Double) {
            if ($t->kind === RustType::FLOAT) {
                return $v;
            }
            return new Val('to_float(&' . $v->code . ')', RustType::float());
        }
        if ($e instanceof Expr\Cast\Bool_) {
            if ($t->kind === RustType::BOOL) {
                return $v;
            }
            return new Val('truthy(&' . $v->code . ')', RustType::bool());
        }
        if ($e instanceof Expr\Cast\String_) {
            if ($t->kind === RustType::STR) {
                return $v;
            }
            if ($t->kind === RustType::CLASS_ || $t->kind === RustType::UNION || $t->kind === RustType::ANY_OBJECT) {
                return new Val($v->code . '.to_php_string()?', RustType::str());
            }
            return new Val('to_str(&' . $v->code . ')', RustType::str());
        }
        if ($e instanceof Expr\Cast\Array_) {
            $res = $this->inferredOrMixed($e);
            if ($t->kind === RustType::LIST || $t->kind === RustType::MAP || $t->kind === RustType::TUPLE || $t->kind === RustType::SHAPE) {
                return $v;
            }
            if ($t->kind === RustType::OPTION) {
                $inner = $t->inner();
                if (in_array($inner->kind, [RustType::LIST, RustType::MAP], true)) {
                    return new Val($v->code . '.unwrap_or_default()', $inner);
                }
                return new Val('(match ' . $v->code . ' { Some(__v) => list![__v], None => List::new() })', RustType::list($inner));
            }
            if ($t->kind === RustType::UNIT) {
                return new Val('List::<Mixed>::new()', RustType::list(RustType::mixed()));
            }
            if ($t->kind === RustType::MIXED) {
                return new Val('mixed_to_array(' . $v->code . ')', RustType::map(RustType::arrayKey(), RustType::mixed()));
            }
            if ($t->kind === RustType::CLASS_ || $t->kind === RustType::ANY_OBJECT) {
                return new Val('object_to_array(&' . $this->casts->convert($v->code, $t, RustType::mixed()) . ')', RustType::map(RustType::arrayKey(), RustType::mixed()));
            }
            return new Val('list![' . $v->code . ']', RustType::list($t));
        }
        if ($e instanceof Expr\Cast\Object_) {
            return new Val('StdClass::from_mixed(' . $this->casts->convert($v->code, $t, RustType::mixed()) . ')', RustType::rtGeneric('StdClass', []));
        }
        if ($e instanceof Expr\Cast\Unset_) {
            return new Val('{ let _ = ' . $v->code . '; () }', RustType::unit());
        }
        $this->warn('unsupported cast', $e);
        return $v;
    }

    private function ternary(Expr\Ternary $e): Val
    {
        $res = $this->inferredOrMixed($e);
        if ($e->if === null) {
            // $a ?: $b
            $cond = $this->expr($e->cond);
            $t = $this->commonType($cond->type, $this->inferredOrMixed($e->else), false);
            if ($res->kind !== RustType::MIXED) {
                $t = $res;
            }
            $tmp = $this->tmp();
            return new Val('{ let ' . $tmp . ' = ' . $cond->code . '; if truthy(&' . $tmp . ') { ' . $this->casts->convert($tmp, $cond->type, $t) . ' } else { ' . $this->exprTo($e->else, $t) . ' } }', $t);
        }
        $t = $res;
        if ($t->kind === RustType::MIXED) {
            $t = $this->commonType($this->inferredOrMixed($e->if), $this->inferredOrMixed($e->else), false);
        }
        return new Val('(if ' . $this->truthy($e->cond) . ' { ' . $this->exprTo($e->if, $t) . ' } else { ' . $this->exprTo($e->else, $t) . ' })', $t);
    }

    private function matchExpr(Expr\Match_ $e): Val
    {
        $res = $this->inferredOrMixed($e);
        $subj = $this->expr($e->cond);
        $tmp = $this->tmp('__subj');
        $code = '{ let ' . $tmp . ' = ' . $subj->code . '; ';
        $first = true;
        $default = null;
        foreach ($e->arms as $arm) {
            if ($arm->conds === null) {
                $default = $arm;
                continue;
            }
            $conds = [];
            foreach ($arm->conds as $c) {
                $cv = $this->expr($c);
                $ct = $this->commonType($subj->type, $cv->type, false);
                $conds[] = 'identical(&' . $this->casts->convert($tmp . '.clone()', $subj->type, $ct) . ', &' . $this->casts->convert($cv->code, $cv->type, $ct) . ')';
            }
            $code .= ($first ? 'if ' : 'else if ') . implode(' || ', $conds) . ' { ' . $this->exprTo($arm->body, $res) . ' } ';
            $first = false;
        }
        if ($default !== null) {
            $code .= ($first ? '{ ' : 'else { ') . $this->exprTo($default->body, $res) . ' } ';
        } else {
            $code .= ($first ? '{ ' : 'else { ') . 'return Err(Throw::unhandled_match(&' . $this->casts->convert($tmp . '.clone()', $subj->type, RustType::mixed()) . ')) } ';
        }
        return new Val($code . '}', $res);
    }

    private function instanceOf(Expr\Instanceof_ $e): Val
    {
        $v = $this->rawValue($e->expr);
        if ($e->class instanceof Name) {
            $fqcn = $this->resolveClassName($e->class);
            if ($fqcn === null) {
                return new Val('{ let _ = ' . $v->code . '; false }', RustType::bool());
            }
            $target = RustType::class($fqcn);
            $tc = $this->program->getClass($fqcn);
            $t = $v->type;
            if ($t->kind === RustType::OPTION) {
                $inner = $t->inner();
                $this->casts->needInstanceOf($inner, $target);
                return new Val($v->code . '.map_or(false, |__v| is_instance::<' . $target->toRust() . '>(&__v))', RustType::bool());
            }
            if ($t->kind === RustType::CLASS_ && $tc !== null) {
                $vc = $this->program->classOf($t);
                if ($vc !== null && $vc->isSubclassOf($tc)) {
                    return new Val('{ let _ = ' . $v->code . '; true }', RustType::bool());
                }
                if ($vc !== null && !$tc->isSubclassOf($vc) && !$tc->isInterface() && !$vc->isInterface()) {
                    return new Val('{ let _ = ' . $v->code . '; false }', RustType::bool());
                }
            }
            if (in_array($t->kind, [RustType::INT, RustType::FLOAT, RustType::STR, RustType::BOOL, RustType::LIST, RustType::MAP, RustType::UNIT, RustType::TUPLE, RustType::SHAPE, RustType::ARRAY_KEY], true)) {
                return new Val('{ let _ = ' . $v->code . '; false }', RustType::bool());
            }
            $this->casts->needInstanceOf($t, $target);
            return new Val('is_instance::<' . $target->toRust() . '>(&' . $v->code . ')', RustType::bool());
        }
        // dynamic class name
        $cls = $this->expr($e->class);
        $name = $cls->type->kind === RustType::STR ? $cls->code : 'class_name_of(&' . $this->casts->convert($cls->code, $cls->type, RustType::mixed()) . ')';
        return new Val('instance_of_name(&' . $this->casts->convert($v->code, $v->type, RustType::mixed()) . ', &' . $name . ')', RustType::bool());
    }

    /** Resolve a class name node (self/static/parent/FQCN) to a canonical FQCN. */
    public function resolveClassName(Name $name): ?string
    {
        $s = $name->toString();
        $lc = strtolower($s);
        if ($lc === 'static' && $this->static_class !== null) {
            return $this->static_class->fqcn;
        }
        if ($lc === 'self' || $lc === 'static') {
            return $this->class?->fqcn;
        }
        if ($lc === 'parent') {
            return $this->class?->parent?->fqcn;
        }
        $resolved = $name->getAttribute('resolvedName') ?? $s;
        return $this->program->canonicalClassName((string) $resolved);
    }

    private function throwValue(Expr $e): string
    {
        $v = $this->expr($e);
        $throwable = RustType::class('Throwable');
        return $this->casts->convert($v->code, $v->type, $throwable);
    }

    private function yieldExpr(Expr\Yield_ $e): Val
    {
        if (!$this->is_generator) {
            $this->warn('yield outside generator', $e);
            return new Val('()', RustType::unit());
        }
        $val = $e->value !== null ? $this->exprTo($e->value, $this->gen_val) : $this->casts->defaultOf($this->gen_val);
        if ($e->key !== null) {
            $key = $this->exprTo($e->key, $this->gen_key);
        } elseif ($this->gen_key->kind === RustType::INT) {
            $key = '(__gen.len() as i64)';
        } elseif ($this->gen_key->kind === RustType::ARRAY_KEY) {
            $key = 'ArrayKey::Int(__gen.len() as i64)';
        } else {
            $key = $this->casts->defaultOf($this->gen_key);
        }
        return new Val('{ __gen.push((' . $key . ', ' . $val . ')); Mixed::Null }', RustType::mixed());
    }

    private function yieldFrom(Expr\YieldFrom $e): Val
    {
        if (!$this->is_generator) {
            $this->warn('yield from outside generator', $e);
            return new Val('()', RustType::unit());
        }
        $target = RustType::rtGeneric('Generator', [$this->gen_key, $this->gen_val]);
        $src = $this->exprTo($e->expr, $target);
        return new Val('{ __gen.extend(' . $src . '.into_pairs()); Mixed::Null }', RustType::mixed());
    }

    // ------------------------------------------------------------------ closures

    private function closure(Closure|ArrowFunction $e): Val
    {
        $record = $this->program->transpiler->getFunctionRecord($e, $this->record->fq_class_name);
        $inf = $this->inferred($e);
        if ($record === null) {
            $this->warn('closure without analysis record', $e);
            return new Val('unreachable!("closure without record")', $inf ?? RustType::dynCallable());
        }
        $child = new BodyEmitter($this->program, $record, $this->class, $this->casts, $this->builtins, $this->diag, $this);
        $storage = $record->storage;
        $param_types = [];
        $param_decls = [];
        foreach ($storage->params as $i => $p) {
            $t = $this->types()->map($p->type);
            if ($p->is_variadic) {
                $t = RustType::list($t);
            }
            $param_types[$p->name] = $t;
            $param_decls[] = 'mut ' . Names::var($p->name) . ': ' . $t->toRust();
        }
        $ret = $this->types()->map($storage->return_type);
        if ($storage->return_type !== null && $storage->return_type->isVoid()) {
            $ret = RustType::unit();
        }
        if ($storage->has_yield) {
            $ret = RustType::rtGeneric('Generator', [RustType::mixed(), RustType::mixed()]);
        }
        if ($e instanceof ArrowFunction && $ret->kind === RustType::UNIT && $storage->return_type === null) {
            $ret = $this->inferredOrMixed($e->expr);
        }

        // captured variables
        $captures = [];
        $capture_names = [];
        if ($e instanceof Closure) {
            foreach ($e->uses as $use) {
                if (!is_string($use->var->name)) {
                    continue;
                }
                $name = $use->var->name;
                if ($use->byRef) {
                    $this->warn('closure use by reference', $e);
                }
                $capture_names[] = $name;
            }
        } else {
            // arrow functions capture the entire enclosing scope by value
            foreach ($record->var_types as $var_id => $_) {
                $name = substr($var_id, 1);
                if ($name !== 'this' && !isset($param_types[$name]) && isset($this->vars[$name])) {
                    $capture_names[] = $name;
                }
            }
        }
        // the closure's own view of the captured variables (joined over its statements) decides their type
        $child->declareLocals($param_types);
        foreach ($capture_names as $name) {
            if (!isset($this->vars[$name])) {
                continue;
            }
            $v = $this->readVar($name);
            $inner_t = $child->vars[$name] ?? $this->vars[$name];
            $captures[] = 'let ' . Names::var($name) . ' = ' . $this->casts->convert($v->code, $v->type, $inner_t) . ';';
            $child->vars[$name] = $inner_t;
            $child->late[$name] = false;
            $child->predeclared[$name] = true;
        }
        $uses_this = !($e instanceof Closure && $e->static) && $this->this_type !== null;
        if ($uses_this) {
            $captures[] = 'let this = ' . $this->this_expr . '.clone();';
        }

        // body
        $stmts = $e instanceof ArrowFunction ? [new Node\Stmt\Return_($e->expr, $e->getAttributes())] : $e->stmts;
        $body = $child->emitBody($param_types, $stmts, $ret);
        // captured locals must be mutable inside the closure (they're re-declared there)
        $decl = '';
        foreach ($capture_names as $name) {
            if (isset($child->vars[$name]) && !isset($param_types[$name])) {
                $rn = Names::var($name);
                $decl .= 'let mut ' . $rn . ' = ' . $rn . '.clone(); ';
            }
        }
        $type = RustType::closure(array_values($param_types), $ret);
        $this->types()->closures[$type->mangle()] = $type;
        $code = '{ ' . implode(' ', $captures) . ' Rc::new(move |' . implode(', ', $param_decls) . '| -> Result<' . $ret->toRust() . ', Throw> { ' . $decl . "\n" . $body . '}) as ' . $type->toRust() . ' }';
        return new Val($code, $type);
    }
}
