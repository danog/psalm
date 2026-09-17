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

use function array_map;
use function array_chunk;
use function count;
use function dirname;
use function str_starts_with;
use function preg_match;
use function stripcslashes;
use function strtoupper;
use function array_keys;
use function implode;
use function max;
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
        if ($to->hasGeneric()) {
            if ($to->kind === RustType::GENERIC && $e instanceof Expr\Array_ && $e->items === []) {
                // an empty literal for a generic slot: an empty list (no element type to infer)
                return $this->expr($e, RustType::list(RustType::unit()))->code;
            }
            // an argument for a generic parameter: emitted in its own natural type (rustc infers/checks T), with
            // the parameter's container structure kept (`list<T>` takes a List of the literal's element type,
            // not the tuple/shape the literal would naturally be)
            $unified = $this->unifyGeneric($to, $this->inferred($e));
            return $this->expr($e, $unified !== null && !$unified->hasGeneric() ? $unified : null)->code;
        }
        return $this->expr($e, $to)->code;
    }

    /** The generic-containing target type with its generics bound from a concrete type of the same structure. */
    private function unifyGeneric(RustType $target, ?RustType $actual): ?RustType
    {
        if ($actual === null) {
            return null;
        }
        if ($target->kind === RustType::GENERIC) {
            return $actual;
        }
        if (!$target->hasGeneric()) {
            return $target;
        }
        $types = $this->types();
        switch ($target->kind) {
            case RustType::OPTION:
                $inner = $this->unifyGeneric($target->inner(), $actual->kind === RustType::OPTION ? $actual->inner() : $actual);
                return $inner === null ? null : RustType::option($inner);
            case RustType::LIST:
                if ($actual->kind === RustType::LIST) {
                    $elem = $this->unifyGeneric($target->inner(), $actual->inner());
                } elseif ($actual->kind === RustType::TUPLE && $actual->params !== []) {
                    $elem = $this->unifyGeneric($target->inner(), $types->combine($actual->params));
                } else {
                    return null;
                }
                return $elem === null ? null : RustType::list($elem);
            case RustType::MAP:
                if ($actual->kind === RustType::MAP) {
                    $k = $this->unifyGeneric($target->params[0], $actual->params[0]);
                    $v = $this->unifyGeneric($target->params[1], $actual->params[1]);
                } elseif ($actual->kind === RustType::SHAPE && $actual->fields !== []) {
                    $k = $this->unifyGeneric($target->params[0], $this->allIntKeys($actual) ? RustType::int() : RustType::str());
                    $v = $this->unifyGeneric($target->params[1], $types->combine(array_map(static fn(array $f) => $f[0], array_values($actual->fields))));
                } elseif ($actual->kind === RustType::LIST) {
                    $k = $this->unifyGeneric($target->params[0], RustType::int());
                    $v = $this->unifyGeneric($target->params[1], $actual->inner());
                } elseif ($actual->kind === RustType::TUPLE && $actual->params !== []) {
                    // a positional literal (`['p', 'q']` inferred as a tuple) for an `array<K, T>` parameter
                    $k = $this->unifyGeneric($target->params[0], RustType::int());
                    $v = $this->unifyGeneric($target->params[1], $types->combine($actual->params));
                } else {
                    return null;
                }
                return $k === null || $v === null ? null : RustType::map($k, $v);
            default:
                return null;
        }
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
        if ($e instanceof BinaryOp\Coalesce) {
            return $this->coalesce($e, $expected);
        }
        if ($e instanceof Expr\FuncCall) {
            // builtins reading a declared type (`json_decode` into a shape) see what the site expects
            $saved = $this->call_expected;
            $this->call_expected = $expected;
            try {
                return $this->funcCall($e);
            } finally {
                $this->call_expected = $saved;
            }
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
            if ($e->var instanceof Expr\Variable && is_string($e->var->name) && !empty($this->refvars[$e->var->name])) {
                return new Val('{ ' . $this->assignRefStmt($e) . ' ' . $this->readVar($e->var->name)->code . ' }', $this->varType($e->var->name));
            }
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
            return $this->numResult('num_mul(' . $this->numOf($v->code, $v->type) . ', Num::Int(-1))', $this->inferredOrMixed($e));
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
            return new Val($this->throwCode($e->expr), RustType::never());
        }
        if ($e instanceof Expr\Exit_) {
            $code = $e->expr !== null ? $this->expr($e->expr) : null;
            if ($code === null) {
                return new Val('php_rt::do_throw(Throw::exit(0))', RustType::never());
            }
            if ($code->type->kind === RustType::INT) {
                return new Val('php_rt::do_throw(Throw::exit(' . $code->code . '))', RustType::never());
            }
            return new Val('{ echo(to_str(' . $code->code . ').as_bytes()); php_rt::do_throw(Throw::exit(0)) }', RustType::never());
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
            // closed world: every includable file is compiled and bound by its compile-time path; nothing
            // is ever loaded at runtime
            $static = $this->staticPath($e->expr);
            if ($static !== null) {
                if (!str_starts_with($static, '/')) {
                    $static = dirname($this->record->file_path) . '/' . $static;
                }
                $file = $this->program->fileForPath($static);
                if ($file !== null) {
                    // the value of a data file has ONE type: the type declared at the include site (the
                    // conversion from Mixed happens there), never the literal's own shape
                    if ($file->isData()) {
                        // the table read in the type the site expects (a parameter, a return type), else in the
                        // type of its own contents (a uniform table of scalars/lists/maps)
                        $t = $expected !== null && $expected->kind !== RustType::MIXED && !$expected->hasGeneric() && !$expected->containsMixed()
                            ? $expected
                            : $this->program->dataType($file);
                        if ($t !== null) {
                            $key = substr(md5($t->toRust()), 0, 8);
                            $this->program->data_demands[$file->rel_path][$key] = $t;
                            $this->casts->needFromData($t);
                            return new Val($file->path() . '_' . $key . '()', $t);
                        }
                        $this->program->data_demands[$file->rel_path]['mixed'] = RustType::mixed();
                        return new Val($file->path() . '()', RustType::mixed());
                    }
                    // a code file: its top-level code is not run again; `include` yields 1
                    return new Val('1i64', RustType::int());
                }
                $this->warn('include of a file that is not compiled: ' . $static, $e);
                return $this->dead('include of a file that is not compiled: ' . $static, $this->inferredOrMixed($e));
            }
            $this->warn('include with a runtime path', $e);
            return $this->dead('include with a runtime path', $this->inferredOrMixed($e));
        }
        if ($e instanceof Expr\Eval_) {
            // only constant expressions (`return "\t";`) are supported by the runtime evaluator
            return $this->narrow(new Val('php_eval(&' . $this->exprTo($e->expr, RustType::str()) . ').unwrap_or_else(|__e| __throw_rt(__e))', RustType::mixed()), $e);
        }
        if ($e instanceof Expr\Yield_) {
            return $this->yieldExpr($e);
        }
        if ($e instanceof Expr\YieldFrom) {
            return $this->yieldFrom($e);
        }
        if ($e instanceof Expr\List_ || $e instanceof Expr\ShellExec) {
            $this->warn('unsupported expression ' . $e->getType(), $e);
            return $this->dead('unsupported ' . $e->getType() . '', RustType::never());
        }

        $this->warn('unsupported expression ' . $e->getType(), $e);
        return $this->dead('unsupported ' . $e->getType() . '', RustType::never());
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
        return 'truthy(' . Names::refOf($v->code) . ')';
    }

    /** Truthiness of an expression that may be undefined (empty()). */
    private function truthyOptional(Expr $e): string
    {
        $v = $this->optionalValue($e);
        if ($v === null) {
            return $this->truthy($e);
        }
        return 'truthy(' . Names::refOf($v->code) . ')';
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
            return new Val(Names::strLit(($this->self_class ?? $this->class)?->fqcn ?? ''), RustType::str());
        }
        if ($e instanceof Scalar\MagicConst\Function_ || $e instanceof Scalar\MagicConst\Method) {
            return new Val(Names::strLit((string) $this->record->method_name), RustType::str());
        }
        if ($e instanceof Scalar\MagicConst\Namespace_) {
            $fq = ($this->self_class ?? $this->class)?->fqcn ?? '';
            $pos = strrpos($fq, '\\');
            return new Val(Names::strLit($pos === false ? '' : substr($fq, 0, $pos)), RustType::str());
        }
        return new Val(Names::strLit(''), RustType::str());
    }

    /** The value of a path expression built from literals, `__DIR__`/`__FILE__`, constants and dirname(); null if dynamic. */
    private function staticPath(Expr $e): ?string
    {
        if ($e instanceof Scalar\String_) {
            return $e->value;
        }
        if ($e instanceof Scalar\MagicConst\Dir) {
            return dirname($this->record->file_path);
        }
        if ($e instanceof Scalar\MagicConst\File) {
            return $this->record->file_path;
        }
        if ($e instanceof Expr\BinaryOp\Concat) {
            $l = $this->staticPath($e->left);
            $r = $this->staticPath($e->right);
            return $l !== null && $r !== null ? $l . $r : null;
        }
        if ($e instanceof Scalar\InterpolatedString) {
            $out = '';
            foreach ($e->parts as $part) {
                if ($part instanceof \PhpParser\Node\InterpolatedStringPart) {
                    $out .= $part->value;
                } else {
                    $p = $this->staticPath($part);
                    if ($p === null) {
                        return null;
                    }
                    $out .= $p;
                }
            }
            return $out;
        }
        if ($e instanceof Expr\ConstFetch) {
            $name = strtoupper($e->name->toString());
            return match ($name) {
                'DIRECTORY_SEPARATOR' => '/',
                'PATH_SEPARATOR' => ':',
                default => null,
            };
        }
        if ($e instanceof Expr\FuncCall && $e->name instanceof Name && strtolower($e->name->toString()) === 'dirname') {
            $args = $e->getArgs();
            $base = isset($args[0]) ? $this->staticPath($args[0]->value) : null;
            if ($base === null) {
                return null;
            }
            $levels = 1;
            if (isset($args[1])) {
                if (!$args[1]->value instanceof Scalar\Int_) {
                    return null;
                }
                $levels = $args[1]->value->value;
            }
            return dirname($base, $levels);
        }
        return null;
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
        // Tokenizer constants (T_*) MUST resolve to php-rt's `consts::` values, NOT Psalm's host PHP
        // values: php-rt's tokenizer emits its own scheme and the parser's createTokenMap is keyed by
        // it, but the host PHP 8.4 T_* values differ (e.g. T_BAD_CHARACTER host 409 == php-rt
        // T_AMPERSAND_FOLLOWED 409). Inlining the host literal (below) made every `$token->id === \T_*`
        // comparison mismatch the tokenizer -> the parser produced empty ASTs. Route to consts:: (before
        // the host-literal inline) so the whole parser agrees on php-rt's token scheme.
        if (str_starts_with($short, 'T_')) {
            return new Val('consts::' . $short, RustType::int());
        }
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
            $ct = $this->builtins->constantType($short);
            return new Val('consts::' . $short . ($ct->kind === RustType::RESOURCE ? '()' : ''), $ct);
        }
        $user = $this->program->getConstant($resolved) ?? $this->program->getConstant($short);
        if ($user !== null) {
            return new Val('crate::consts::' . Names::constant($user->name) . '()', $user->type);
        }
        $this->warn('unknown constant ' . $resolved, $e);
        return $this->dead('unknown constant ' . $resolved . '', $t ?? RustType::mixed());
    }

    private function variable(Expr\Variable $e): Val
    {
        if (!is_string($e->name)) {
            $this->warn('variable variable', $e);
            return $this->dead('variable variable', RustType::mixed());
        }
        $v = $this->narrow($this->readVar($e->name), $e);
        // apply an active `instanceof` narrowing (from `$v instanceof X && ...`) as a downcast, so subclass-only
        // members resolve statically instead of via the dynamic protocol. Only a strict class->subclass narrowing.
        if (isset($this->narrowings[$e->name])) {
            $nt = $this->narrowings[$e->name];
            if ($nt->kind === RustType::CLASS_ && $v->type->kind === RustType::CLASS_ && $nt->toRust() !== $v->type->toRust()) {
                $sub = $this->program->classOf($nt);
                $cur = $this->program->classOf($v->type);
                if ($sub !== null && $cur !== null && $sub->isSubclassOf($cur)) {
                    return new Val($this->casts->convert($v->code, $v->type, $nt), $nt);
                }
            }
        }
        return $v;
    }

    /**
     * Register `instanceof` narrowings found in a boolean sub-expression (recursing through `&&`) into
     * `$this->narrowings`, so a later read of the variable in a guarded position downcasts to the tested class.
     * Only project classes (which become closed enums/leaves) are narrowed; external classes stay dynamic.
     */
    private function collectInstanceofNarrowings(Expr $e): void
    {
        if ($e instanceof BinaryOp\BooleanAnd || $e instanceof BinaryOp\LogicalAnd) {
            $this->collectInstanceofNarrowings($e->left);
            $this->collectInstanceofNarrowings($e->right);
            return;
        }
        if ($e instanceof Expr\Instanceof_ && $e->expr instanceof Expr\Variable && is_string($e->expr->name)
            && $e->class instanceof Name
        ) {
            $fqcn = $this->resolveClassName($e->class);
            if ($fqcn !== null) {
                $tc = $this->program->getClass($fqcn);
                if ($tc !== null && $tc->is_project) {
                    $this->narrowings[$e->expr->name] = RustType::class($fqcn);
                }
            }
        }
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
        // a generic value narrowed to `object`/mixed stays generic (its class name is reachable through PhpKind)
        if ($v->type->kind === RustType::GENERIC && ($inf->kind === RustType::ANY_OBJECT || $inf->containsMixed())) {
            return $v;
        }
        // a container read keeps its stored element types: a refinement Psalm made (`array<int, int>` for a
        // stored `array<array-key, int>`, `list<TLiteralInt>` for a stored `list<Atomic>`) would otherwise be
        // an element-wise conversion of the whole container on every read; elements narrow when they are read
        $containers = [RustType::LIST, RustType::MAP];
        if (in_array($v->type->kind, $containers, true) && in_array($inf->kind, $containers, true)) {
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
        // a shape whose optional fields would become required (values invented for absent keys) is not
        // more precise than the stored one; the same holds inside containers
        if ($from->kind === RustType::SHAPE && $to->kind === RustType::SHAPE && array_keys($from->fields) == array_keys($to->fields)) {
            foreach ($from->fields as $k => [$ft, $opt]) {
                [$tt, $topt] = $to->fields[$k];
                if (($opt && !$topt) || $this->isWidening($ft, $tt)) {
                    return true;
                }
            }
            return false;
        }
        if ($from->kind === RustType::MAP && $to->kind === RustType::MAP && $from->params[0]->toRust() === $to->params[0]->toRust()) {
            return $this->isWidening($from->params[1], $to->params[1]);
        }
        // a map is never read as a list: renumbering it would change the keys a later `$a[$k]` reads
        // (Psalm keeps calling an array a list after `unset()`s inside a loop)
        if ($from->kind === RustType::MAP && $to->kind === RustType::LIST) {
            return true;
        }
        if (($from->kind === RustType::LIST && $to->kind === RustType::LIST) || ($from->kind === RustType::OPTION && $to->kind === RustType::OPTION)) {
            return $this->isWidening($from->inner(), $to->inner());
        }
        return false;
    }

    // ------------------------------------------------------------------ arrays

    private function arrayLiteral(Expr\Array_ $e, ?RustType $expected): Val
    {
        $v = $this->arrayLiteralValue($e, $expected);
        if (($v->type->kind === RustType::LIST || $v->type->kind === RustType::MAP) && count($e->items) >= 2 && count($e->items) <= 24 && self::isConstArray($e)) {
            // a constant table: built once per thread and shared (containers are copy-on-write)
            $name = '__CONST_ARR' . $this->tmp('');
            return new Val('{ thread_local! { static ' . $name . ': ' . $v->type->toRust() . ' = ' . $v->code . '; } ' . $name . '.with(|__c| __c.clone()) }', $v->type);
        }
        return $v;
    }

    /** Whether an array literal consists only of scalar literals (and nested such arrays). */
    private static function isConstArray(Expr\Array_ $e): bool
    {
        foreach ($e->items as $item) {
            if ($item->unpack || $item->byRef) {
                return false;
            }
            if ($item->key !== null && !self::isConstScalar($item->key)) {
                return false;
            }
            if ($item->value instanceof Expr\Array_ ? !self::isConstArray($item->value) : !self::isConstScalar($item->value)) {
                return false;
            }
        }
        return true;
    }

    private static function isConstScalar(Expr $e): bool
    {
        if ($e instanceof Node\Scalar\String_ || $e instanceof Node\Scalar\Int_ || $e instanceof Node\Scalar\Float_) {
            return true;
        }
        if ($e instanceof Expr\UnaryMinus || $e instanceof Expr\UnaryPlus) {
            return $e->expr instanceof Node\Scalar\Int_ || $e->expr instanceof Node\Scalar\Float_;
        }
        if ($e instanceof Expr\ConstFetch) {
            return in_array(strtolower($e->name->toString()), ['true', 'false', 'null'], true);
        }
        return false;
    }

    private function arrayLiteralValue(Expr\Array_ $e, ?RustType $expected): Val
    {
        $target = $expected;
        if ($target !== null && $target->kind === RustType::OPTION && in_array($target->inner()->kind, [RustType::LIST, RustType::MAP, RustType::TUPLE, RustType::SHAPE], true)) {
            $target = $target->inner();
        }
        if ($target !== null && $target->kind === RustType::OPTION) {
            $target = $target->inner();
        }
        if ($target !== null && $target->kind === RustType::UNION) {
            // `$x = []` into a `list<T>|Foo` union: build the literal as the union's collection member (the
            // caller wraps it into the variant) instead of an untyped Map<ArrayKey, Mixed>
            $pick = null;
            foreach ($target->params as $member) {
                if (in_array($member->kind, [RustType::LIST, RustType::MAP, RustType::TUPLE, RustType::SHAPE], true)) {
                    $pick = $member;
                    break;
                }
            }
            $target = $pick;
        }
        $inf = $target === null || !in_array($target->kind, [RustType::LIST, RustType::MAP, RustType::TUPLE, RustType::SHAPE], true) ? $this->inferred($e) : null;
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
            $next_int = 0;
            foreach ($e->items as $i => $item) {
                // items without a key get the next integer key (0 for the first one, whatever precedes it)
                $key = $item->key !== null ? $this->literalKey($item->key) : (string) $next_int;
                if ($key === null) {
                    return $this->arrayLiteral($e, RustType::map(RustType::arrayKey(), $this->shapeValueType($target)));
                }
                if ((string) (int) $key === $key) {
                    $next_int = max($next_int, (int) $key + 1);
                }
                if (!isset($target->fields[$key])) {
                    return $this->arrayLiteral($e, RustType::map(
                        $this->allIntKeys($target) ? RustType::int() : RustType::arrayKey(),
                        $this->shapeValueType($target),
                    ));
                }
                [$ft, $opt] = $target->fields[$key];
                if ($opt) {
                    $v = $this->expr($item->value, $ft);
                    $fields[Names::field($key)] = $this->casts->convert($v->code, $v->type, RustType::shapeField($ft, true));
                } else {
                    $fields[Names::field($key)] = $this->exprTo($item->value, $ft);
                }
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
                if (count($parts) > 24) {
                    $tmp = $this->tmp('__l');
                    $stmts = array_map(fn(string $p) => $tmp . '.push(' . $p . '); ', $parts);
                    return new Val('{ let mut ' . $tmp . ': ' . $target->toRust() . ' = List::new(); ' . $this->chunked($stmts, $tmp, $target) . $tmp . ' }', $target);
                }
                return new Val('list![' . implode(', ', $parts) . ']', $target);
            }
            $tmp = $this->tmp('__l');
            $stmts = [];
            foreach ($e->items as $item) {
                if ($item->unpack) {
                    $sv = $this->expr($item->value);
                    $stmts[] = $tmp . '.extend(' . $this->casts->convert($sv->code, $sv->type, RustType::list($elem)) . '.into_iter()); ';
                } else {
                    $stmts[] = $tmp . '.push(' . $this->exprTo($item->value, $elem) . '); ';
                }
            }
            return new Val('{ let mut ' . $tmp . ': ' . $target->toRust() . ' = List::new(); ' . $this->chunked($stmts, $tmp, $target) . $tmp . ' }', $target);
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
        $stmts = [];
        foreach ($e->items as $item) {
            if ($item->unpack) {
                $sv = $this->expr($item->value);
                $stmts[] = 'for (__k, __v) in ' . $this->casts->convert($sv->code, $sv->type, $target) . '.into_iter() { if __k.int_value().is_some() { ' . $tmp . '.push(__v); } else { ' . $tmp . '.insert(__k, __v); } } ';
            } elseif ($item->key === null) {
                $stmts[] = $tmp . '.push(' . $this->exprTo($item->value, $vt) . '); ';
            } else {
                $stmts[] = $tmp . '.insert(' . $this->keyExpr($item->key, $kt) . ', ' . $this->exprTo($item->value, $vt) . '); ';
            }
        }
        return new Val('{ let mut ' . $tmp . ': ' . $target->toRust() . ' = Map::new(); ' . $this->chunked($stmts, $tmp, $target) . $tmp . ' }', $target);
    }

    /**
     * Large literals are built in chunks by immediately-invoked closures, each a separate function body
     * for the Rust compiler (one huge body makes borrow checking very memory hungry).
     *
     * @param list<string> $stmts statements filling `$tmp`
     */
    private function chunked(array $stmts, string $tmp, RustType $t): string
    {
        if (count($stmts) <= 24) {
            return implode('', $stmts);
        }
        $out = '';
        foreach (array_chunk($stmts, 24) as $chunk) {
            $body = implode('', $chunk);
            // panic-based error model: no emitted `?`; the chunk closure is always infallible.
            $out .= '(|' . $tmp . ': &mut ' . $t->toRust() . '| { ' . $body . ' })(&mut ' . $tmp . '); ';
        }
        return $out;
    }

    /**
     * Does the emitted code contain a Rust `?` (try) operator? Double-quoted string literals are blanked first so a
     * literal `?` inside data (e.g. an embedded PHP snippet) is not mistaken for the operator. Char literals `'?'`
     * are left as-is (rare) and would conservatively read as an operator — safe (only over-reports fallibility).
     */
    private function hasTryOp(string $code): bool
    {
        $stripped = preg_replace('/"(?:\\\\.|[^"\\\\])*"/', '""', $code);
        return strpos($stripped ?? $code, '?') !== false;
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
            // literal keys need no runtime numeric-string check
            if ($ft->kind === RustType::INT && preg_match('/^-?[0-9]+i64$/', $v->code)) {
                return 'ArrayKey::Int(' . $v->code . ')';
            }
            if ($ft->kind === RustType::STR && preg_match('/^Str::from_static\("((?:[^"\\\\]|\\\\.)*)"\)$/', $v->code, $m)
                && !preg_match('/^(0|-?[1-9][0-9]{0,18})$/', stripcslashes($m[1]))
            ) {
                return 'ArrayKey::from_static("' . $m[1] . '")';
            }
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
            // `$v instanceof X && $v->method()`: narrow $v to X while emitting the RHS so the call resolves
            // statically (Psalm narrows here but doesn't record it on the receiver node -> would go dynamic).
            $left = $this->truthy($e->left);
            $saved = $this->narrowings;
            $this->collectInstanceofNarrowings($e->left);
            $right = $this->truthy($e->right);
            $this->narrowings = $saved;
            return new Val('(' . $left . ' && ' . $right . ')', RustType::bool());
        }
        if ($e instanceof BinaryOp\BooleanOr || $e instanceof BinaryOp\LogicalOr) {
            // the right side runs when the left is false: narrowings made inside the left do not hold there
            $saved = $this->narrowings;
            $left = $this->truthy($e->left);
            $this->narrowings = $saved;
            $right = $this->truthy($e->right);
            $this->narrowings = $saved;
            return new Val('(' . $left . ' || ' . $right . ')', RustType::bool());
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

        // shape + shape: the left fields, then the right-only ones (typed field by field)
        if ($sig === '+' && $lt->kind === RustType::SHAPE && $rt->kind === RustType::SHAPE) {
            $fields = [];
            foreach ($lt->fields as $k => $f) {
                $fields[$k] = $f;
            }
            foreach ($rt->fields as $k => $f) {
                if (!isset($fields[$k])) {
                    $fields[$k] = $f;
                }
            }
            $target = $res->kind === RustType::SHAPE && array_keys($res->fields) == array_keys($fields) ? $res : $this->types()->registerShape(RustType::shape($fields));
            $inits = [];
            foreach ($target->fields as $k => [$ft, $opt]) {
                $src = isset($lt->fields[$k]) ? '__l' : '__r';
                [$st, $sopt] = isset($lt->fields[$k]) ? $lt->fields[$k] : $rt->fields[$k];
                $inits[] = Names::field($k) . ': ' . $this->casts->convert($src . '.' . Names::field($k), RustType::shapeField($st, $sopt), RustType::shapeField($ft, $opt));
            }
            return new Val('{ let __l = ' . $this->exprTo($e->left, $lt) . '; let __r = ' . $this->exprTo($e->right, $rt) . '; ' . $target->toRust() . ' { ' . implode(', ', $inits) . ' } }', $target);
        }
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
            // div ops panic on division-by-zero (php-rt ops.rs) rather than returning Result — DivisionByZeroError
            // is never caught, so no `?`.
            if ($res->kind === RustType::INT && $int_ok($lt) && $int_ok($rt)) {
                return new Val('div_i(' . $this->exprTo($e->left, RustType::int()) . ', ' . $this->exprTo($e->right, RustType::int()) . ')', RustType::int());
            }
            if ($res->kind === RustType::FLOAT) {
                return new Val('div_f(' . $this->exprTo($e->left, RustType::float()) . ', ' . $this->exprTo($e->right, RustType::float()) . ')', RustType::float());
            }
            $l = $this->numOperand($e->left);
            $r = $this->numOperand($e->right);
            $num = 'div(' . $l . ', ' . $r . ')';
            return $this->numResult($num, $res);
        }
        if ($sig === '%') {
            return new Val('imod(' . $this->exprTo($e->left, RustType::int()) . ', ' . $this->exprTo($e->right, RustType::int()) . ')', RustType::int());
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
        return $this->numOf($v->code, $v->type);
    }

    /** A typed value as a runtime `Num` (through Mixed only for values without a typed numeric view). */
    public function numOf(string $code, RustType $t): string
    {
        if ($t->kind === RustType::RT_GENERIC && $t->name === 'Num') {
            return $code;
        }
        return match ($t->kind) {
            RustType::INT => 'Num::Int(' . $code . ')',
            RustType::FLOAT => 'Num::Float(' . $code . ')',
            RustType::BOOL => 'Num::Int(' . $code . ' as i64)',
            RustType::STR, RustType::UNIT, RustType::ARRAY_KEY, RustType::OPTION, RustType::UNION => 'php_rt::ToNum::to_php_num(&' . $code . ')',
            default => 'to_num(&' . $this->casts->convert($code, $t, RustType::mixed()) . ')',
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
        if ($res->kind === RustType::OPTION && in_array($res->inner()->kind, [RustType::INT, RustType::FLOAT, RustType::UNION], true)) {
            // a nullable result type (`?int`, `ConstValue|null`): a number is never null
            $inner = $this->numResult($num_code, $res->inner());
            return new Val('Some(' . $inner->code . ')', $res);
        }
        // Psalm sees `mixed` (array operands possible): the number stays a typed Num for the consuming conversion
        return new Val($num_code, RustType::rtGeneric('Num', []));
    }

    /**
     * Emit both operands converted to a common type for comparison.
     *
     * @return array{string, string, RustType}
     */
    private function commonOperands(Expr $left, Expr $right, bool $numeric): array
    {
        // declared (not narrowed) types: a value contradicting Psalm's narrowing must still compare, not unwrap
        $l = $this->rawValue($left);
        $r = $this->rawValue($right);
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
        // Option-stripped views: a member of the other side's union (or a class of its hierarchy) compares in that type
        $sa = $a->kind === RustType::OPTION ? $a->inner() : $a;
        $sb = $b->kind === RustType::OPTION ? $b->inner() : $b;
        $opt = $a->kind === RustType::OPTION || $b->kind === RustType::OPTION;
        $wrap = static fn(RustType $t) => $opt ? RustType::option($t) : $t;
        if ($sa->kind === RustType::UNION && $sb->kind !== RustType::UNION && $this->casts->pickMember($sa, $sb) !== null) {
            return $wrap($sa);
        }
        if ($sb->kind === RustType::UNION && $sa->kind !== RustType::UNION && $this->casts->pickMember($sb, $sa) !== null) {
            return $wrap($sb);
        }
        if (($sa->kind === RustType::ARRAY_KEY && in_array($sb->kind, [RustType::INT, RustType::STR, RustType::SYM], true))
            || ($sb->kind === RustType::ARRAY_KEY && in_array($sa->kind, [RustType::INT, RustType::STR, RustType::SYM], true))
        ) {
            return $wrap(RustType::arrayKey());
        }
        if ($sa->kind === RustType::UNION && $sb->kind === RustType::UNION) {
            $joined = $this->program->unionOfRust([$sa, $sb]);
            if ($joined !== null) {
                return $wrap($joined);
            }
        }
        if ($sa->kind === RustType::UNION && $sb->kind === RustType::CLASS_ && $this->unionFitsClass($sa, $sb)) {
            return $wrap($sb);
        }
        if ($sb->kind === RustType::UNION && $sa->kind === RustType::CLASS_ && $this->unionFitsClass($sb, $sa)) {
            return $wrap($sa);
        }
        if ($sa->kind === RustType::CLASS_ && $sb->kind === RustType::CLASS_) {
            $ca = $this->program->classOf($sa);
            $cb = $this->program->classOf($sb);
            if ($ca !== null && $cb !== null) {
                if ($ca->isSubclassOf($cb)) {
                    return $wrap($sb);
                }
                if ($cb->isSubclassOf($ca)) {
                    return $wrap($sa);
                }
            }
        }
        if ($sa->kind === RustType::MAP && $sb->kind === RustType::MAP && $sa->params[1]->toRust() === $sb->params[1]->toRust()
            && in_array($sa->params[0]->kind, [RustType::STR, RustType::INT, RustType::ARRAY_KEY], true)
            && in_array($sb->params[0]->kind, [RustType::STR, RustType::INT, RustType::ARRAY_KEY], true)
        ) {
            return $wrap(RustType::map(RustType::arrayKey(), $sa->params[1]));
        }
        foreach ([[$sa, $sb], [$sb, $sa]] as [$tt, $mt]) {
            if ($tt->kind === RustType::TUPLE && $mt->kind === RustType::MAP && in_array($mt->params[0]->kind, [RustType::INT, RustType::ARRAY_KEY], true)
                && $this->tupleFitsList($tt, RustType::list($mt->params[1]))
            ) {
                return $wrap($mt);
            }
        }
        if ($sa->kind === RustType::TUPLE && $sb->kind === RustType::LIST && $this->tupleFitsList($sa, $sb)) {
            return $wrap($sb);
        }
        if ($sb->kind === RustType::TUPLE && $sa->kind === RustType::LIST && $this->tupleFitsList($sb, $sa)) {
            return $wrap($sa);
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
            $escaped = function (RustType $t): ?string {
                // a non-leaf class value may carry a null through its escape variant
                if ($t->kind !== RustType::CLASS_) {
                    return null;
                }
                $c = $this->program->classOf($t);
                // only an OPEN hierarchy can carry a null through its `Other__(Mixed::Null)` escape; a
                // closed handle has no such variant, so null-checks use `Option`/`false` semantics instead
                return $c !== null && !$c->isLeaf() && $c->has_downstream ? $t->toRust() : null;
            };
            if ($ov !== null) {
                $h = $escaped($ov->type->inner());
                return $h !== null ? 'matches!(&' . $ov->code . ', None | Some(' . $h . '::Other__(Mixed::Null)))' : $ov->code . '.is_none()';
            }
            if ($v->type->kind === RustType::OPTION) {
                $h = $escaped($v->type->inner());
                return $h !== null ? 'matches!(&' . $v->code . ', None | Some(' . $h . '::Other__(Mixed::Null)))' : $v->code . '.is_none()';
            }
            if ($v->type->kind === RustType::UNIT) {
                return '{ let _ = ' . $v->code . '; true }';
            }
            if ($v->type->kind === RustType::MIXED) {
                return $v->code . '.is_null()';
            }
            $h = $escaped($v->type);
            if ($h !== null) {
                return 'matches!(&' . $v->code . ', ' . $h . '::Other__(Mixed::Null))';
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
        // `substr($s, $i, $n) === 'lit'` without materializing the substring
        foreach ([[$left, $right], [$right, $left]] as [$a, $b]) {
            if ($b instanceof Node\Scalar\String_ && $a instanceof Expr\FuncCall && $a->name instanceof Node\Name
                && strtolower($a->name->toString()) === 'substr' && !$a->isFirstClassCallable() && count($a->getArgs()) === 3
            ) {
                $sa = $a->getArgs();
                return 'substr_eq(' . Names::refOf($this->exprTo($sa[0]->value, RustType::str())) . ', ' . $this->exprTo($sa[1]->value, RustType::int())
                    . ', Some(' . $this->exprTo($sa[2]->value, RustType::int()) . '), ' . Names::rustStringLiteral($b->value) . ')';
            }
        }
        // a scalar literal against a union that has no member of its kind: never identical (side effects kept)
        foreach ([[$left, $right], [$right, $left]] as [$a, $b]) {
            $lit_kind = null;
            if ($b instanceof Expr\ConstFetch && in_array(strtolower($b->name->toString()), ['true', 'false'], true)) {
                $lit_kind = 'bool';
            } elseif ($b instanceof Node\Scalar\Int_) {
                $lit_kind = 'int';
            } elseif ($b instanceof Node\Scalar\String_) {
                $lit_kind = 'str';
            } elseif ($b instanceof Node\Scalar\Float_) {
                $lit_kind = 'float';
            }
            if ($lit_kind === null) {
                continue;
            }
            $av = $this->rawValue($a);
            $at = $av->type;
            if ($at->kind === RustType::GENERIC) {
                // a generic against a literal: same runtime kind and same value
                $lit_code = match ($lit_kind) {
                    'bool' => 'php_rt::Truthy::truthy(&__g) == ' . strtolower($b->name->toString()),
                    'str' => 'php_rt::ToStr::to_php_str(&__g).as_bytes() == ' . Names::rustStringLiteral($b->value) . '.as_bytes()',
                    'int' => 'php_rt::ToStr::to_php_str(&__g).as_bytes() == ' . Names::rustStringLiteral((string) $b->value) . '.as_bytes()',
                    default => null,
                };
                $kind = match ($lit_kind) { 'bool' => 'Bool', 'str' => 'Str', 'int' => 'Int', default => 'Float' };
                if ($lit_code !== null) {
                    return '{ let __g = &' . $av->code . '; php_rt::PhpKind::php_kind(__g) == php_rt::Kind::' . $kind . ' && ' . $lit_code . ' }';
                }
            }
            $u = $at->kind === RustType::OPTION ? $at->inner() : $at;
            if ($u->kind !== RustType::UNION) {
                continue;
            }
            $has = false;
            foreach ($u->params as $m) {
                $has = $has || match ($lit_kind) {
                    'bool' => $m->kind === RustType::BOOL || ($m->kind === RustType::RT_GENERIC && in_array($m->name, ['__unit_True', '__unit_False'], true)),
                    'int' => in_array($m->kind, [RustType::INT, RustType::ARRAY_KEY, RustType::FLOAT], true),
                    'str' => in_array($m->kind, [RustType::STR, RustType::SYM, RustType::ARRAY_KEY], true),
                    default => $m->kind === RustType::FLOAT,
                };
            }
            if (!$has) {
                return '{ let _ = ' . $av->code . '; false }';
            }
        }
        // `$list === ['a', 'b']` / `$map === []`: the literal takes the container's type (no Mixed array)
        foreach ([[$left, $right], [$right, $left]] as [$a, $b]) {
            if ($b instanceof Expr\Array_ && !$a instanceof Expr\Array_) {
                $va = $this->rawValue($a);
                $containers = [RustType::LIST, RustType::MAP, RustType::SHAPE];
                if (in_array($va->type->kind, $containers, true)) {
                    return 'identical(' . Names::refOf($va->code) . ', ' . Names::refOf($this->exprTo($b, $va->type)) . ')';
                }
                if ($va->type->kind === RustType::OPTION && in_array($va->type->inner()->kind, $containers, true)) {
                    return 'identical(' . Names::refOf($va->code) . ', &Some(' . $this->exprTo($b, $va->type->inner()) . '))';
                }
                $u = $va->type->kind === RustType::OPTION ? $va->type->inner() : $va->type;
                if ($u->kind === RustType::UNION) {
                    foreach ($u->params as $m) {
                        if (in_array($m->kind, $containers, true)) {
                            $lit = $this->casts->convert($this->exprTo($b, $m), $m, $u);
                            if ($va->type->kind === RustType::OPTION) {
                                $lit = 'Some(' . $lit . ')';
                            }
                            return 'identical(' . Names::refOf($va->code) . ', &' . $lit . ')';
                        }
                    }
                }
            }
        }
        // an object is never identical to a scalar, an array or null: a dead comparison (no Mixed)
        $ta = $this->inferred($left);
        $tb = $this->inferred($right);
        if ($ta !== null && $tb !== null && $this->isPlainRead($left) && $this->isPlainRead($right)) {
            foreach ([[$ta, $tb], [$tb, $ta]] as [$obj, $other]) {
                if ($obj->kind === RustType::CLASS_ && $this->hasNoObject($other)) {
                    return 'false';
                }
            }
        }
        [$l, $r, $t] = $this->commonOperands($left, $right, false);
        if ($t->isCopy() && $t->kind !== RustType::OPTION) {
            return '(' . $l . ' == ' . $r . ')';
        }
        return 'identical(' . Names::refOf($l) . ', ' . Names::refOf($r) . ')';
    }

    /** A variable or property read: evaluating it has no side effect, so a dead comparison may skip it. */
    private function isPlainRead(Expr $e): bool
    {
        while ($e instanceof Expr\PropertyFetch) {
            $e = $e->var;
        }
        return $e instanceof Expr\Variable && is_string($e->name);
    }

    /** True when values of the type are never objects (scalars, null, arrays, unions of those). */
    private function hasNoObject(RustType $t): bool
    {
        if ($t->kind === RustType::OPTION) {
            return $this->hasNoObject($t->inner());
        }
        return match ($t->kind) {
            RustType::STR, RustType::INT, RustType::FLOAT, RustType::BOOL, RustType::UNIT, RustType::ARRAY_KEY, RustType::SYM,
            RustType::LIST, RustType::MAP, RustType::TUPLE, RustType::SHAPE, RustType::NEVER => true,
            RustType::UNION => !Casts::unionHasObject($t) && !$t->containsMixed(),
            default => false,
        };
    }

    /** Every member of `$u` is `$cls` or a subclass of it. */
    private function unionFitsClass(RustType $u, RustType $cls): bool
    {
        $c = $this->program->classOf($cls);
        if ($c === null) {
            return false;
        }
        foreach ($u->params as $m) {
            if ($m->kind !== RustType::CLASS_) {
                return false;
            }
            $mc = $this->program->classOf($m);
            if ($mc === null || !($mc === $c || $mc->isSubclassOf($c))) {
                return false;
            }
        }
        return true;
    }

    private function tupleFitsList(RustType $tuple, RustType $list): bool
    {
        foreach ($tuple->params as $p) {
            if ($p->toRust() !== $list->inner()->toRust()) {
                return false;
            }
        }
        return $tuple->params !== [];
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
        $lt = $this->inferredOrMixed($left);
        $rt = $this->inferredOrMixed($right);
        foreach ([[$left, $lt, $right, $rt], [$right, $rt, $left, $lt]] as [$oe, $ot, $se, $st]) {
            $oi = $ot->kind === RustType::OPTION ? $ot->inner() : $ot;
            if ($st->kind === RustType::STR && ($oi->kind === RustType::CLASS_ || ($oi->kind === RustType::UNION && Casts::unionHasObject($oi)))) {
                // object == string: the object's string form (Stringable) compared, false otherwise
                $ov = $this->rawValue($oe);
                $sv = $this->exprTo($se, RustType::str());
                $to_s = $oi->kind === RustType::CLASS_ ? 'php_rt::PhpObject::php_to_string(__o)' : '__o.php_to_string()';
                $subject = $ov->type->kind === RustType::OPTION ? $ov->code . '.as_ref().and_then(|__o| ' . $to_s . ')' : '{ let __o = &' . $ov->code . '; ' . $to_s . ' }';
                return '(match ' . $subject . ' { Some(__s) => loose_eq(&__s, &' . $sv . '), None => false })';
            }
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

    private function coalesce(BinaryOp\Coalesce $e, ?RustType $expected = null): Val
    {
        // the target shapes both sides (`$this->cb = $cb ?? function () {...}` takes the property's callable type)
        $res = $expected !== null && $expected->kind !== RustType::MIXED && !$expected->hasGeneric() ? $expected : $this->inferredOrMixed($e);
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
            return new Val('(match ' . $left->code . ' { Some(__v) => ' . $this->casts->convert('__v', $inner, $target) . ', None => ' . $this->throwCode($e->right->expr) . ' })', $target);
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
            if ($bt->kind === RustType::UNION && ($ui = $this->unionIndex('__b', $bt, $this->keyExpr($dim, RustType::arrayKey()), $this->literalKey($dim))) !== null) {
                return $this->flattenOption($base->code . '.and_then(|__b| ' . $ui[0] . ')', $ui[1]);
            }
            if ($bt->kind === RustType::STR) {
                return new Val('{ let __k = ' . $this->exprTo($dim, RustType::int()) . '; ' . $base->code . '.and_then(|__b| str_index_opt(&__b, __k)) }', RustType::option(RustType::str()));
            }
            if ($bt->kind === RustType::RT_GENERIC && in_array($bt->name, ['ArrayObject', 'ArrayIterator', 'SplObjectStorage', 'WeakMap'], true)) {
                [$kt, $vt] = $bt->params;
                $code = '{ let __k = ' . $this->exprTo($dim, $kt) . '; ' . $base->code . '.and_then(|__b| __b.get(&__k)) }';
                return $this->flattenOption($code, $vt);
            }
            if ($bt->kind === RustType::CLASS_) {
                // ArrayAccess object
                $k = $this->expr($dim);
                $cls = $this->program->classOf($bt);
                $m = $cls !== null ? $this->program->findMethod($cls, 'offsetget') : null;
                if ($m !== null) {
                    $code = '(match ' . $base->code . ' { Some(__b) => { if __b.offsetExists(' . $this->casts->convert($k->code, $k->type, $this->program->findMethod($cls, 'offsetexists')->param_types[0] ?? RustType::mixed()) . ') { Some(__b.offsetGet(' . $this->casts->convert($k->code, $k->type, $m->param_types[0] ?? RustType::mixed()) . ')) } else { None } } None => None })';
                    return $this->flattenOption($code, $m->return_type);
                }
            }
            if ($bt->kind === RustType::UNIT || $bt->kind === RustType::NEVER) {
                return new Val('{ let _ = ' . $base->code . '; None::<Mixed> }', RustType::option(RustType::mixed()));
            }
            if ($bt->kind === RustType::UNION || $bt->kind === RustType::ANY_OBJECT || $bt->kind === RustType::CLASS_ || $bt->kind === RustType::MAP || $bt->kind === RustType::LIST) {
                // resolved dynamically (array forms and ArrayAccess objects alike)
                $k = $this->expr($dim);
                return new Val('{ let __k = ' . $this->keyFrom($k, RustType::arrayKey()) . '; ' . $base->code . '.and_then(|__b| mixed_get(&' . $this->casts->convert('__b', $bt, RustType::mixed()) . ', &__k)) }', RustType::option(RustType::mixed()));
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
                $magic_get = $cls !== null ? $this->program->findMethod($cls, '__get') : null;
                $magic_isset = $cls !== null ? $this->program->findMethod($cls, '__isset') : null;
                if ($magic_get !== null && $magic_get->node !== null) {
                    $lit = Names::strLit($name);
                    $present = $magic_isset !== null && $magic_isset->node !== null
                        ? '__b.' . $magic_isset->rustName() . '(' . $lit . ')'
                        : 'true';
                    $code = $base->code . '.and_then(|__b| if ' . $present . ' { Some(__b.' . $magic_get->rustName() . '(' . $lit . ')) } else { None })';
                    return $this->flattenOption($code, $magic_get->return_type);
                }
            }
            if ($bt->kind === RustType::MIXED) {
                return new Val($base->code . '.and_then(|__b| mixed_prop(&__b, &' . $this->dynPropName($name) . '))', RustType::option(RustType::mixed()));
            }
            if ($bt->kind === RustType::RT_GENERIC && $bt->name === 'StdClass') {
                return new Val($base->code . '.and_then(|__b| __b.get(' . Names::strLit($name) . '))', RustType::option(RustType::mixed()));
            }
            return null;
        }
        if ($e instanceof Expr\StaticPropertyFetch) {
            if ($e->name instanceof Node\VarLikeIdentifier && $e->class instanceof Name) {
                // `isset(self::$instance)` on a static without initializer: unset until assigned
                $fqcn = $this->resolveClassName($e->class);
                $cls = $fqcn !== null ? $this->program->getClass($fqcn) : null;
                $field = $this->findStaticField($cls, $e->name->name);
                if ($field !== null && $field->type->kind !== RustType::OPTION) {
                    return new Val($field->declaring->path() . '::st_' . $field->rustName() . '_opt()', RustType::option($field->type));
                }
            }
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
        $ut = $t->kind === RustType::OPTION ? $t->inner() : $t;
        if ($ut->kind === RustType::UNION && $this->casts->pickMember($ut, RustType::int()) !== null && $this->casts->pickMember($ut, RustType::int())->kind === RustType::INT) {
            // a union holding an int: the int member steps, anything else is a type error
            $op = $is_inc ? 'wrapping_add(1)' : 'wrapping_sub(1)';
            $step = '(match ' . $tmp . '.clone() { ' . $ut->mangle() . '::Int(__i) => ' . $ut->mangle() . '::Int(__i.' . $op . '), _ => panic!("cannot increment/decrement a non-numeric value") })';
            if ($t->kind === RustType::OPTION) {
                return new Val('{ let ' . $tmp . ' = ' . $place->read() . '.expect("null where a number expected"); let __n = ' . $step . '; ' . $place->write('Some(__n.clone())') . ' ' . ($is_pre ? 'Some(__n)' : 'Some(' . $tmp . ')') . ' }', $t);
            }
            return new Val('{ let ' . $tmp . ' = ' . $place->read() . '; let __n = ' . $step . '; ' . $place->write('__n.clone()') . ' ' . ($is_pre ? '__n' : $tmp) . ' }', $t);
        }
        if ($t->kind !== RustType::NEVER && $t->kind !== RustType::UNIT) {
            // anything else (optional unions, ...): PHP's increment semantics on the Mixed form
            $fn = $is_inc ? 'mixed_inc' : 'mixed_dec';
            return new Val('{ let ' . $tmp . ' = ' . $this->casts->convert($place->read(), $t, RustType::mixed()) . '; let __n = ' . $fn . '(&' . $tmp . '); ' . $place->write($this->casts->convert('__n.clone()', RustType::mixed(), $t)) . ' ' . ($is_pre ? '__n' : $tmp) . ' }', RustType::mixed());
        }
        $this->warn('inc/dec on ' . $t->toRust(), $e);
        return $this->dead('inc/dec on ' . $t->toRust() . '', RustType::never());
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
                return new Val($v->code . '.to_php_string()', RustType::str());
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
            if ($t->kind === RustType::CLASS_ && ($shape = Builtins::objectVarsShape($this, $v, $e->expr instanceof Expr\Variable && $e->expr->name === 'this')) !== null) {
                return $shape;
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
            // Psalm lost the type: join the branches' static types
            $iv = $this->expr($e->if);
            $ev = $this->expr($e->else);
            $t = $this->commonType($iv->type, $ev->type, false);
            if ($t->kind === RustType::MIXED) {
                $t = $this->program->unionOfRust([$iv->type, $ev->type]) ?? $t;
            }
            return new Val('(if ' . $this->truthy($e->cond) . ' { ' . $this->casts->convert($iv->code, $iv->type, $t) . ' } else { ' . $this->casts->convert($ev->code, $ev->type, $t) . ' })', $t);
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
            $code .= ($first ? '{ ' : 'else { ') . 'php_rt::do_throw(Throw::unhandled_match(&' . $tmp . ')) } ';
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
            if ($tc !== null && !$tc->is_project) {
                return new Val($this->casts->instanceOfName($v->code, $t, 'Str::from_static(' . Names::rustStringLiteral($fqcn) . ')'), RustType::bool());
            }
            $dyn_kinds = [RustType::RT_GENERIC, RustType::DYN_CALLABLE, RustType::CLOSURE, RustType::RESOURCE];
            if (in_array($t->kind, $dyn_kinds, true) || ($t->kind === RustType::OPTION && in_array($t->inner()->kind, $dyn_kinds, true))) {
                // runtime containers (generators, callables): checked by class name on the Mixed form
                return new Val($this->casts->instanceOfName($v->code, $t, 'Str::from_static(' . Names::rustStringLiteral($fqcn) . ')'), RustType::bool());
            }
            if ($t->kind === RustType::OPTION) {
                $inner = $t->inner();
                $this->casts->needInstanceOf($inner, $target);
                return new Val($v->code . '.map_or(false, |__v| is_instance::<' . $target->toRust() . '>(&__v))', RustType::bool());
            }
            if ($t->kind === RustType::CLASS_ && $tc !== null) {
                $vc = $this->program->classOf($t);
                if ($vc !== null && $vc->isLeaf() && $vc->isSubclassOf($tc)) {
                    return new Val('{ let _ = ' . $v->code . '; true }', RustType::bool());
                }
                if ($vc !== null && $vc->isLeaf() && !$tc->isSubclassOf($vc) && !$tc->isInterface() && !$vc->isInterface()) {
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
        $name = match (true) {
            $cls->type->kind === RustType::STR => $cls->code,
            $cls->type->kind === RustType::SYM => 'php_rt::ToStr::to_php_str(&' . $cls->code . ')',
            $cls->type->kind === RustType::OPTION && in_array($cls->type->inner()->kind, [RustType::STR, RustType::SYM], true) => 'php_rt::ToStr::to_php_str(&' . $cls->code . '.expect("null class name"))',
            default => $this->casts->classNameOf($cls->code, $cls->type) ?? 'class_name_of(&' . $this->casts->convert($cls->code, $cls->type, RustType::mixed()) . ')',
        };
        return new Val($this->casts->instanceOfName($v->code, $v->type, $name), RustType::bool());
    }

    /** Resolve a class name node (self/static/parent/FQCN) to a canonical FQCN. */
    public function resolveClassName(Name $name): ?string
    {
        $s = $name->toString();
        $lc = strtolower($s);
        if ($lc === 'static' && $this->static_class !== null) {
            return $this->static_class->fqcn;
        }
        if ($lc === 'self') {
            return ($this->self_class ?? $this->class)?->fqcn;
        }
        if ($lc === 'static') {
            return $this->class?->fqcn;
        }
        if ($lc === 'parent') {
            return ($this->self_class ?? $this->class)?->parent?->fqcn;
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
        // a yield produces nothing (generators of the port receive no sent values)
        return new Val('{ __gen.push((' . $key . ', ' . $val . ')); }', RustType::unit());
    }

    private function yieldFrom(Expr\YieldFrom $e): Val
    {
        if (!$this->is_generator) {
            $this->warn('yield from outside generator', $e);
            return new Val('()', RustType::unit());
        }
        $target = RustType::rtGeneric('Generator', [$this->gen_key, $this->gen_val]);
        $src = $this->exprTo($e->expr, $target);
        return new Val('{ __gen.extend(' . $src . '.into_pairs()); }', RustType::unit());
    }

    // ------------------------------------------------------------------ closures

    private function closure(Closure|ArrowFunction $e): Val
    {
        $record = $this->program->transpiler->getFunctionRecord($e, $this->record->fq_class_name);
        $inf = $this->inferred($e);
        if ($record === null) {
            $this->warn('closure without analysis record', $e);
            return $this->dead('closure without record', $inf ?? RustType::dynCallable());
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
                $capture_names[] = $name;
            }
        } else {
            // arrow functions capture the entire enclosing scope by value
            $mentioned = [];
            foreach ($record->var_types as $var_id => $_) {
                $mentioned[substr($var_id, 1)] = true;
            }
            foreach ((new \PhpParser\NodeFinder())->findInstanceOf([$e->expr], Expr\Variable::class) as $v) {
                if (is_string($v->name)) {
                    $mentioned[$v->name] = true;
                }
            }
            foreach ($mentioned as $name => $_) {
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
            if (!empty($this->cells[$name])) {
                // shared by reference: the closure gets the same cell
                $captures[] = 'let ' . Names::var($name) . ' = ' . Names::var($name) . '.clone();';
                $child->vars[$name] = $this->vars[$name];
                $child->cells[$name] = true;
                $child->late[$name] = false;
                $child->predeclared[$name] = true;
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
        $code = '{ ' . implode(' ', $captures) . ' Rc::new(move |' . implode(', ', $param_decls) . '| -> ' . $ret->toRust() . ' { ' . $decl . "\n" . $body . '}) as ' . $type->toRust() . ' }';
        return new Val($code, $type);
    }
}
