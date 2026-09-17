<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Expr;
use PhpParser\Node\Expr\BinaryOp;
use PhpParser\Node\Name;
use PhpParser\Node\Scalar;

use function array_map;
use function count;
use function substr;
use function implode;
use function max;
use function strtolower;

/**
 * Emits constant expressions (parameter defaults, property defaults, class constants)
 * for which no Psalm expression types are recorded. The target type drives the shape.
 *
 * @internal
 */
final class ConstExprEmitter
{
    public function __construct(
        private readonly BodyEmitter $body,
    ) {
    }

    public function emit(Expr $e, RustType $t): string
    {
        $v = $this->natural($e, $t);
        return $this->body->casts->convert($v->code, $v->type, $t);
    }

    private function natural(Expr $e, RustType $t): Val
    {
        $casts = $this->body->casts;
        if ($t->kind === RustType::OPTION && !($e instanceof Expr\ConstFetch && strtolower($e->name->toString()) === 'null')) {
            $inner = $this->natural($e, $t->inner());
            return new Val('Some(' . $casts->convert($inner->code, $inner->type, $t->inner()) . ')', $t);
        }
        if ($e instanceof Scalar\Int_) {
            return new Val($e->value . 'i64', RustType::int());
        }
        if ($e instanceof Scalar\Float_) {
            $s = sprintf('%.17g', $e->value);
            if (!str_contains($s, '.') && !str_contains($s, 'e')) {
                $s .= '.0';
            }
            return new Val($s . 'f64', RustType::float());
        }
        if ($e instanceof Scalar\String_) {
            return new Val(Names::strLit($e->value), RustType::str());
        }
        if ($e instanceof Scalar\MagicConst\Dir) {
            return new Val('src_dir(' . Names::strLit(dirname($this->relativeFile())) . ')', RustType::str());
        }
        if ($e instanceof Scalar\MagicConst\File) {
            return new Val('src_file(' . Names::strLit($this->relativeFile()) . ')', RustType::str());
        }
        if ($e instanceof Scalar\MagicConst\Class_) {
            return new Val(Names::strLit($this->body->class?->fqcn ?? ''), RustType::str());
        }
        if ($e instanceof Scalar\MagicConst) {
            return new Val(Names::strLit(''), RustType::str());
        }
        if ($e instanceof Expr\UnaryMinus) {
            $v = $this->natural($e->expr, $t);
            return new Val('(-' . $v->code . ')', $v->type);
        }
        if ($e instanceof Expr\UnaryPlus) {
            return $this->natural($e->expr, $t);
        }
        if ($e instanceof Expr\ConstFetch) {
            $lc = strtolower($e->name->toString());
            if ($lc === 'true' || $lc === 'false') {
                return new Val($lc, RustType::bool());
            }
            if ($lc === 'null') {
                return new Val('()', RustType::unit());
            }
            $short = $e->name->getLast();
            if ($this->body->builtins->hasConstant($short)) {
                return new Val('consts::' . $short, $this->body->builtins->constantType($short));
            }
            $resolved = (string) ($e->name->getAttribute('resolvedName') ?? $e->name->toString());
            $user = $this->body->program->getConstant($resolved) ?? $this->body->program->getConstant($short);
            if ($user !== null) {
                return new Val('crate::consts::' . Names::constant($user->name) . '()', $user->type);
            }
            $this->body->warn('unknown constant ' . $resolved . ' in constant expression', $e);
            return new Val('unreachable!("unknown constant")', $t);
        }
        if ($e instanceof Expr\ClassConstFetch && $e->class instanceof Name && $e->name instanceof \PhpParser\Node\Identifier) {
            $fqcn = $this->body->resolveClassName($e->class);
            if ($e->name->name === 'class') {
                return new Val(Names::strLit((string) $fqcn), RustType::str());
            }
            $cls = $fqcn !== null ? $this->body->program->getClass($fqcn) : null;
            if ($cls !== null && $cls->isEnum() && isset($cls->storage->enum_cases[$e->name->name])) {
                return new Val($cls->path() . '::' . Names::typeIdent($e->name->name), RustType::class($cls->fqcn));
            }
            $const = $cls !== null ? $this->body->findConstant($cls, $e->name->name) : null;
            if ($const !== null) {
                return new Val($const->declaring->path() . '::' . $const->rustName() . '()', $const->type);
            }
            if ($cls !== null && !$cls->is_project && isset($cls->storage->constants[$e->name->name])) {
                $cs = $cls->storage->constants[$e->name->name];
                $ct = $cs->type ?? $cs->inferred_type;
                if ($ct !== null && $ct->isSingleIntLiteral()) {
                    return new Val($ct->getSingleIntLiteral()->value . 'i64', RustType::int());
                }
                if ($ct !== null && $ct->isSingleStringLiteral()) {
                    return new Val(Names::strLit($ct->getSingleStringLiteral()->value), RustType::str());
                }
            }
            $this->body->warn('unknown class constant in constant expression ' . $fqcn . '::' . $e->name->name, $e);
            return new Val('unreachable!("unknown class constant")', $t);
        }
        if ($e instanceof Expr\Array_) {
            return $this->array($e, $t);
        }
        if ($e instanceof BinaryOp\Concat) {
            $l = $this->emit($e->left, RustType::str());
            $r = $this->emit($e->right, RustType::str());
            return new Val('concat(' . $l . ', ' . $r . ')', RustType::str());
        }
        if ($e instanceof BinaryOp\BitwiseOr || $e instanceof BinaryOp\BitwiseAnd || $e instanceof BinaryOp\Plus
            || $e instanceof BinaryOp\Minus || $e instanceof BinaryOp\Mul || $e instanceof BinaryOp\ShiftLeft
        ) {
            $l = $this->emit($e->left, RustType::int());
            $r = $this->emit($e->right, RustType::int());
            $op = match (true) {
                $e instanceof BinaryOp\BitwiseOr => '|',
                $e instanceof BinaryOp\BitwiseAnd => '&',
                $e instanceof BinaryOp\Plus => '+',
                $e instanceof BinaryOp\Minus => '-',
                $e instanceof BinaryOp\Mul => '*',
                default => '<<',
            };
            return new Val('(' . $l . ' ' . $op . ' ' . $r . ')', RustType::int());
        }
        if ($e instanceof Expr\BitwiseNot) {
            return new Val('(!' . $this->emit($e->expr, RustType::int()) . ')', RustType::int());
        }
        if ($e instanceof Expr\UnaryMinus) {
            if ($t->kind === RustType::FLOAT) {
                return new Val('(-' . $this->emit($e->expr, RustType::float()) . ')', RustType::float());
            }
            return new Val('(' . $this->emit($e->expr, RustType::int()) . ').wrapping_neg()', RustType::int());
        }
        if ($e instanceof Expr\UnaryPlus) {
            return $this->natural($e->expr, $t);
        }
        if ($e instanceof Expr\BooleanNot) {
            return new Val('(!' . $this->emit($e->expr, RustType::bool()) . ')', RustType::bool());
        }
        if ($e instanceof Expr\New_ && $e->class instanceof Name) {
            $fqcn = $this->body->resolveClassName($e->class);
            $cls = $fqcn !== null ? $this->body->program->getClass($fqcn) : null;
            if ($cls !== null) {
                $ctor = $this->body->program->findMethod($cls, '__construct');
                $args = [];
                if ($ctor !== null) {
                    foreach ($ctor->storage->params as $i => $p) {
                        $pt = $ctor->param_types[$i] ?? RustType::mixed();
                        $arg = $e->getArgs()[$i] ?? null;
                        $args[] = $arg !== null ? $this->emit($arg->value, $pt) : $casts->defaultOf($pt);
                    }
                }
                return new Val($cls->path() . '::new(' . implode(', ', $args) . ').unwrap()', RustType::class($cls->fqcn));
            }
        }
        if ($e instanceof Expr\Ternary && $e->if !== null) {
            $c = $this->emit($e->cond, RustType::bool());
            return new Val('(if ' . $c . ' { ' . $this->emit($e->if, $t) . ' } else { ' . $this->emit($e->else, $t) . ' })', $t);
        }
        if ($e instanceof BinaryOp\Identical || $e instanceof BinaryOp\Equal) {
            $l = $this->natural($e->left, RustType::mixed());
            $r = $this->natural($e->right, RustType::mixed());
            return new Val('identical(&' . $casts->convert($l->code, $l->type, RustType::mixed()) . ', &' . $casts->convert($r->code, $r->type, RustType::mixed()) . ')', RustType::bool());
        }
        if ($e instanceof Expr\FuncCall && $e->name instanceof Name) {
            // constant-ish builtin calls in defaults (e.g. sys_get_temp_dir())
            $v = $this->body->builtins->emit($this->body, $e, strtolower($e->name->getLast()), $e->getArgs());
            if ($v !== null) {
                return $v;
            }
        }
        $this->body->warn('unsupported constant expression ' . $e->getType(), $e);
        return new Val('unreachable!("unsupported constant expression ' . $e->getType() . '")', $t);
    }

    /** A literal key, or a class constant with a literal value (`self::PUBLIC => 'public'`). */
    private function constKey(Expr $key): ?string
    {
        $lit = $this->body->literalKey($key);
        if ($lit !== null) {
            return $lit;
        }
        if ($key instanceof Expr\ClassConstFetch && $key->class instanceof Name && $key->name instanceof \PhpParser\Node\Identifier) {
            $fqcn = $this->body->resolveClassName($key->class);
            $cls = $fqcn !== null ? $this->body->program->getClass($fqcn) : null;
            $cs = $cls !== null ? ($cls->storage->constants[$key->name->name] ?? null) : null;
            $ct = $cs !== null ? ($cs->type ?? $cs->inferred_type) : null;
            if ($ct !== null && $ct->isSingleIntLiteral()) {
                return (string) $ct->getSingleIntLiteral()->value;
            }
            if ($ct !== null && $ct->isSingleStringLiteral()) {
                return $ct->getSingleStringLiteral()->value;
            }
        }
        return null;
    }

    private function array(Expr\Array_ $e, RustType $t): Val
    {
        $casts = $this->body->casts;
        $has_spread = false;
        foreach ($e->items as $item) {
            if ($item->unpack) {
                $has_spread = true;
            }
        }
        if ($t->kind === RustType::OPTION || $t->kind === RustType::UNION) {
            // `NodeAttributes|array $attributes = []`: the literal takes the array member of the target
            foreach ($t->kind === RustType::OPTION ? [$t->inner()] : $t->params as $m) {
                if (in_array($m->kind, [RustType::SHAPE, RustType::LIST, RustType::MAP, RustType::TUPLE], true)) {
                    $v = $this->array($e, $m);
                    return new Val($casts->convert($v->code, $m, $t), $t);
                }
            }
        }
        if ($has_spread && ($t->kind === RustType::TUPLE || $t->kind === RustType::SHAPE)) {
            // `[...self::A, 'b']`: built as a list/map and converted
            return $this->array($e, $t->kind === RustType::TUPLE
                ? RustType::list($this->body->types()->combine($t->params))
                : RustType::map(RustType::arrayKey(), RustType::mixed()));
        }
        if ($t->kind === RustType::TUPLE && count($t->params) === count($e->items)) {
            $parts = [];
            foreach ($e->items as $i => $item) {
                $parts[] = $this->emit($item->value, $t->params[$i]);
            }
            return new Val('(' . implode(', ', $parts) . (count($parts) === 1 ? ',' : '') . ')', $t);
        }
        if ($t->kind === RustType::SHAPE) {
            $fields = [];
            $seen = [];
            $next_int = 0;
            foreach ($e->items as $i => $item) {
                $key = $item->key !== null ? $this->constKey($item->key) : (string) $next_int;
                if ($key === null || !isset($t->fields[$key])) {
                    return $this->array($e, RustType::map(RustType::arrayKey(), RustType::mixed()));
                }
                if ((string) (int) $key === $key) {
                    $next_int = max($next_int, (int) $key + 1);
                }
                [$ft, $opt] = $t->fields[$key];
                $code = $this->emit($item->value, $ft);
                $fields[] = Names::field($key) . ': ' . ($opt ? $casts->convert($code, $ft, RustType::option($ft)) : $code);
                $seen[$key] = true;
            }
            foreach ($t->fields as $k => [$ft, $opt]) {
                if (!isset($seen[$k])) {
                    $fields[] = Names::field($k) . ': ' . ($opt ? 'None' : $casts->defaultOf($ft));
                }
            }
            return new Val($t->mangle() . ' { ' . implode(', ', $fields) . ' }', $t);
        }
        if ($t->kind === RustType::LIST) {
            $parts = [];
            foreach ($e->items as $item) {
                if ($item->key !== null) {
                    return $this->array($e, RustType::map(RustType::arrayKey(), $t->inner()));
                }
                if ($item->unpack) {
                    $sv = $this->natural($item->value, $t);
                    $parts[] = '__l.extend(' . $casts->convert($sv->code, $sv->type, $t) . '.into_iter());';
                } else {
                    $parts[] = '__l.push(' . $this->emit($item->value, $t->inner()) . ');';
                }
            }
            if (!$has_spread) {
                return new Val($parts === [] ? 'List::<' . $t->inner()->toRust() . '>::new()' : 'list![' . implode(', ', array_map(static fn(string $p) => substr($p, 9, -2), $parts)) . ']', $t);
            }
            return new Val('{ let mut __l: ' . $t->toRust() . ' = List::new(); ' . implode(' ', $parts) . ' __l }', $t);
        }
        if ($t->kind !== RustType::MAP) {
            $t = RustType::map(RustType::arrayKey(), RustType::mixed());
        }
        [$kt, $vt] = $t->params;
        if ($e->items === []) {
            return new Val('Map::<' . $kt->toRust() . ', ' . $vt->toRust() . '>::new()', $t);
        }
        $parts = [];
        foreach ($e->items as $item) {
            if ($item->unpack) {
                $sv = $this->natural($item->value, $t);
                $parts[] = 'for (__k, __v) in ' . $casts->convert($sv->code, $sv->type, $t) . '.into_iter() { if __k.int_value().is_some() { __m.push(__v); } else { __m.insert(__k, __v); } }';
                continue;
            }
            $val = $this->emit($item->value, $vt);
            if ($item->key === null) {
                $parts[] = '__m.push(' . $val . ');';
            } else {
                $key = $this->natural($item->key, $kt);
                $parts[] = '__m.insert(' . $this->body->keyFrom($key, $kt) . ', ' . $val . ');';
            }
        }
        return new Val('{ let mut __m: ' . $t->toRust() . ' = Map::new(); ' . implode(' ', $parts) . ' __m }', $t);
    }

    private function relativeFile(): string
    {
        $root = $this->body->program->transpiler->root_dir;
        $f = $this->body->record->file_path;
        if (str_starts_with($f, $root . '/')) {
            return substr($f, strlen($root) + 1);
        }
        return $f;
    }
}
