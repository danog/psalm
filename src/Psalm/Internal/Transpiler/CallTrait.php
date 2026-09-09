<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Arg;
use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\VariadicPlaceholder;
use Psalm\Storage\FunctionLikeStorage;

use function array_values;
use function count;
use function implode;
use function in_array;
use function is_string;
use function strtolower;

/**
 * Calls: functions, methods, static methods, constructors.
 *
 * @internal
 */
trait CallTrait
{
    /**
     * Emit argument expressions for a callee with known storage.
     *
     * @param list<Arg|VariadicPlaceholder> $args
     * @param list<RustType> $param_types
     * @return list<string>
     */
    public function args(array $args, FunctionLikeStorage $storage, array $param_types, ?ClassModel $callee_class, string $callee_name): array
    {
        $params = array_values($storage->params);
        $out = [];
        $positional = [];
        $named = [];
        foreach ($args as $arg) {
            if ($arg instanceof VariadicPlaceholder) {
                continue;
            }
            if ($arg->name !== null) {
                $named[$arg->name->name] = $arg;
            } else {
                $positional[] = $arg;
            }
        }
        $variadic_index = null;
        foreach ($params as $i => $p) {
            if ($p->is_variadic) {
                $variadic_index = $i;
            }
        }
        foreach ($params as $i => $param) {
            $t = $param_types[$i] ?? RustType::mixed();
            if ($param->is_variadic) {
                $rest = array_slice($positional, $i);
                $elem = $t->kind === RustType::LIST ? $t->inner() : RustType::mixed();
                if (count($rest) === 1 && $rest[0]->unpack) {
                    $sv = $this->expr($rest[0]->value);
                    $out[] = $this->casts->convert($sv->code, $sv->type, RustType::list($elem));
                } else {
                    $parts = [];
                    foreach ($rest as $a) {
                        if ($a->unpack) {
                            $this->warn('argument unpacking mixed with positional variadic args', $a);
                            continue;
                        }
                        $parts[] = $this->exprTo($a->value, $elem);
                    }
                    $out[] = $parts === [] ? 'List::new()' : 'list![' . implode(', ', $parts) . ']';
                }
                break;
            }
            $arg = $positional[$i] ?? $named[$param->name] ?? null;
            if ($arg !== null && $arg->unpack) {
                $this->warn('argument unpacking', $arg);
                $sv = $this->expr($arg->value);
                $out[] = $this->casts->convert($sv->code . '.idx(0).clone()', RustType::mixed(), $t);
                continue;
            }
            if ($arg === null) {
                $out[] = $this->defaultArg($param, $t, $callee_class, $callee_name, $i);
                continue;
            }
            if ($param->by_ref) {
                $out[] = $this->byRefArg($arg->value, $t);
                continue;
            }
            $out[] = $this->exprTo($arg->value, $t);
        }
        return $out;
    }

    /** Pass an lvalue as `&mut T`. */
    private function byRefArg(Expr $e, RustType $t): string
    {
        $place = $this->place($e);
        if ($place->hasMut() && $place->type->toRust() === $t->toRust()) {
            return '&mut ' . $place->mut();
        }
        $this->warn('by-ref argument needing conversion (' . $place->type->toRust() . ' vs ' . $t->toRust() . ')', $e);
        return '&mut ' . $this->casts->defaultOf($t);
    }

    /** The default value expression of a parameter, evaluated in the callee's scope. */
    private function defaultArg(\Psalm\Storage\FunctionLikeParameter $param, RustType $t, ?ClassModel $callee_class, string $callee_name, int $index): string
    {
        $node = $this->paramDefaultNode($callee_class, $callee_name, $index);
        if ($node === null) {
            if ($param->default_type !== null) {
                $dt = $this->types()->map($param->default_type);
                if ($param->default_type->isNull()) {
                    return $this->casts->convert('()', RustType::unit(), $t);
                }
                if ($param->default_type->isSingleIntLiteral()) {
                    return $this->casts->convert($param->default_type->getSingleIntLiteral()->value . 'i64', RustType::int(), $t);
                }
                if ($param->default_type->isSingleStringLiteral()) {
                    return $this->casts->convert(Names::strLit($param->default_type->getSingleStringLiteral()->value), RustType::str(), $t);
                }
                if ($param->default_type->isTrue()) {
                    return $this->casts->convert('true', RustType::bool(), $t);
                }
                if ($param->default_type->isFalse()) {
                    return $this->casts->convert('false', RustType::bool(), $t);
                }
                if ($dt->kind === RustType::LIST || $dt->kind === RustType::MAP || $param->default_type->isEmptyArray()) {
                    return $this->casts->defaultOf($t);
                }
            }
            if ($t->hasDefault()) {
                return $this->casts->defaultOf($t);
            }
            $this->warn('missing default for parameter ' . $param->name, null);
            return $this->casts->defaultOf($t);
        }
        // evaluate the default expression in the callee's class scope
        $saved_class = $this->class;
        $emitter = $callee_class !== null && $callee_class !== $this->class ? $this->scopedEmitter($callee_class) : $this;
        $v = $emitter->constExpr($node, $t);
        return $v;
    }

    private function paramDefaultNode(?ClassModel $callee_class, string $callee_name, int $index): ?Expr
    {
        if ($callee_class !== null) {
            $m = $this->program->findMethod($callee_class, strtolower($callee_name));
            if ($m !== null && $m->node !== null) {
                return $m->node->params[$index]->default ?? null;
            }
            return null;
        }
        $f = $this->program->getFunction($callee_name);
        if ($f !== null) {
            return $f->record->node->params[$index]->default ?? null;
        }
        return null;
    }

    /** A body emitter whose `self::` refers to another class, for constant expressions. */
    private function scopedEmitter(ClassModel $cls): BodyEmitter
    {
        $e = new BodyEmitter($this->program, $this->record, $cls, $this->casts, $this->builtins, $this->diag, null);
        return $e;
    }

    /** Emit a constant expression (defaults, constants) converted to `$t`; no Psalm types are available. */
    public function constExpr(Expr $e, RustType $t): string
    {
        $ce = new ConstExprEmitter($this);
        return $ce->emit($e, $t);
    }

    // ------------------------------------------------------------------ function calls

    private function funcCall(Expr\FuncCall $e): Val
    {
        if ($e->isFirstClassCallable()) {
            return $this->firstClassCallable($e);
        }
        if (!$e->name instanceof Name) {
            return $this->dynamicCall($e);
        }
        $resolved = (string) ($e->name->getAttribute('resolvedName') ?? $e->name->toString());
        $short = $e->name->getLast();
        $args = $e->getArgs();

        // user-defined function?
        $fn = $this->program->getFunction($resolved) ?? $this->program->getFunction($short);
        if ($fn !== null) {
            $argc = $this->args($args, $fn->record->storage, $fn->param_types, null, $fn->fq_name);
            return new Val($fn->path() . '(' . implode(', ', $argc) . ')?', $fn->return_type);
        }

        $result = $this->builtins->emit($this, $e, strtolower($short), $args);
        if ($result !== null) {
            return $result;
        }
        $this->warn('unknown function ' . $resolved, $e);
        return new Val('unreachable!("unknown function ' . $resolved . '")', $this->inferredOrMixed($e));
    }

    /** `$callable(...)` */
    private function dynamicCall(Expr\FuncCall $e): Val
    {
        $callee = $this->expr($e->name);
        return $this->callValue($callee, $e->getArgs(), $e);
    }

    /** Call a closure-typed value. */
    public function callValue(Val $callee, array $args, Expr $site): Val
    {
        $t = $callee->type;
        if ($t->kind === RustType::OPTION) {
            $callee = new Val($callee->code . '.unwrap()', $t->inner());
            $t = $t->inner();
        }
        if ($t->kind === RustType::CLOSURE) {
            $argc = [];
            foreach ($t->params as $i => $pt) {
                if (isset($args[$i])) {
                    $argc[] = $this->exprTo($args[$i]->value, $pt);
                } else {
                    $argc[] = $this->casts->defaultOf($pt);
                }
            }
            return new Val('(' . $callee->code . ')(' . implode(', ', $argc) . ')?', $t->ret);
        }
        if ($t->kind === RustType::DYN_CALLABLE || $t->kind === RustType::MIXED || $t->kind === RustType::STR) {
            $argc = [];
            foreach ($args as $a) {
                $argc[] = $this->exprTo($a->value, RustType::mixed());
            }
            $conv = $t->kind === RustType::DYN_CALLABLE ? $callee->code : 'to_callable(&' . $this->casts->convert($callee->code, $t, RustType::mixed()) . ')';
            return $this->narrow(new Val($conv . '.call(vec![' . implode(', ', $argc) . '])?', RustType::mixed()), $site);
        }
        if ($t->kind === RustType::CLASS_) {
            $cls = $this->program->classOf($t);
            $m = $cls !== null ? $this->program->findMethod($cls, '__invoke') : null;
            if ($m !== null) {
                $argc = $this->args($args, $m->storage, $m->param_types, $m->declaring, $m->name);
                return new Val($callee->code . '.' . $m->rustName() . '(' . implode(', ', $argc) . ')?', $m->return_type);
            }
        }
        $this->warn('call of ' . $t->toRust(), $site);
        return new Val('unreachable!("call of ' . $t->toRust() . '")', $this->inferredOrMixed($site));
    }

    private function firstClassCallable(Expr\FuncCall|Expr\MethodCall|Expr\StaticCall $e): Val
    {
        $inf = $this->inferred($e);
        if ($inf === null || $inf->kind !== RustType::CLOSURE) {
            $this->warn('first-class callable without signature', $e);
            return new Val('unreachable!("first-class callable")', $inf ?? RustType::dynCallable());
        }
        $params = [];
        $args = [];
        foreach ($inf->params as $i => $p) {
            $params[] = '__p' . $i . ': ' . $p->toRust();
            $args[] = new Arg(new Expr\Variable('__fcc' . $i));
        }
        // emit the call with placeholder variables of the right types
        $child_vars = $this->vars;
        foreach ($inf->params as $i => $p) {
            $this->vars['__fcc' . $i] = $p;
            $this->late['__fcc' . $i] = false;
        }
        if ($e instanceof Expr\FuncCall) {
            $call = new Expr\FuncCall($e->name, $args, $e->getAttributes());
        } elseif ($e instanceof Expr\MethodCall) {
            $call = new Expr\MethodCall($e->var, $e->name, $args, $e->getAttributes());
        } else {
            $call = new Expr\StaticCall($e->class, $e->name, $args, $e->getAttributes());
        }
        $v = $this->expr($call, $inf->ret);
        $this->vars = $child_vars;
        $capt = $this->this_type !== null ? 'let this = ' . $this->this_expr . '.clone(); ' : '';
        $decls = [];
        foreach ($inf->params as $i => $p) {
            $decls[] = 'let __fcc' . $i . ' = __p' . $i . ';';
        }
        $code = '{ ' . $capt . 'Rc::new(move |' . implode(', ', $params) . '| -> Result<' . $inf->ret->toRust() . ', Throw> { ' . implode(' ', $decls) . ' Ok(' . str_replace('self.', 'this.', $v->code) . ') }) as ' . $inf->toRust() . ' }';
        return new Val($code, $inf);
    }

    // ------------------------------------------------------------------ methods

    private function methodCall(Expr\MethodCall|Expr\NullsafeMethodCall $e, bool $nullsafe): Val
    {
        if ($e instanceof Expr\MethodCall && $e->isFirstClassCallable()) {
            return $this->firstClassCallable($e);
        }
        if (!$e->name instanceof Identifier) {
            $recv = $this->expr($e->var);
            $name = $this->exprTo($e->name, RustType::str());
            $argc = [];
            foreach ($e->getArgs() as $a) {
                $argc[] = $this->exprTo($a->value, RustType::mixed());
            }
            return $this->narrow(new Val('mixed_call(&' . $this->casts->convert($recv->code, $recv->type, RustType::mixed()) . ', &' . $name . ', vec![' . implode(', ', $argc) . '])?', RustType::mixed()), $e);
        }
        $name = $e->name->name;
        $lc = strtolower($name);
        $recv = $this->receiver($e->var);
        $rt = $recv->type;
        if ($rt->kind === RustType::OPTION) {
            if ($nullsafe) {
                $inner = $rt->inner();
                $tmp = $this->tmp();
                $inner_call = $this->methodCallOn(new Val($tmp, $inner), $name, $e);
                $res = $inner_call->type->kind === RustType::OPTION ? $inner_call->type : RustType::option($inner_call->type);
                $body = $inner_call->type->kind === RustType::OPTION ? $inner_call->code : 'Some(' . $inner_call->code . ')';
                return new Val('(match ' . $recv->code . ' { Some(' . $tmp . ') => ' . $body . ', None => None })', $res);
            }
            $recv = new Val($recv->code . '.unwrap()', $rt->inner());
        }
        return $this->methodCallOn($recv, $name, $e);
    }

    private function methodCallOn(Val $recv, string $name, Expr $e): Val
    {
        $lc = strtolower($name);
        $rt = $recv->type;
        $args = $e->getArgs();
        if ($rt->kind === RustType::CLASS_) {
            $cls = $this->program->classOf($rt);
            $m = $cls !== null ? $this->program->findMethod($cls, $lc) : null;
            if ($m !== null) {
                $argc = $this->args($args, $m->storage, $m->param_types, $m->declaring, $m->name);
                if ($m->isStatic()) {
                    return new Val('{ let _ = ' . $recv->code . '; ' . $m->declaring->path() . '::' . $m->rustName() . '(' . implode(', ', $argc) . ')? }', $m->return_type);
                }
                return new Val($recv->code . '.' . $m->rustName() . '(' . implode(', ', $argc) . ')?', $m->return_type);
            }
            if ($cls !== null && $cls->isEnum()) {
                return $this->enumStaticCall($cls, $lc, $args, $e, $recv);
            }
            // magic __call
            $call = $cls !== null ? $this->program->findMethod($cls, '__call') : null;
            if ($call !== null) {
                $argc = [];
                foreach ($args as $a) {
                    $argc[] = $this->exprTo($a->value, RustType::mixed());
                }
                $list = $argc === [] ? 'List::new()' : 'list![' . implode(', ', $argc) . ']';
                $v = new Val($recv->code . '.' . $call->rustName() . '(' . Names::strLit($name) . ', ' . $this->casts->convert($list, RustType::list(RustType::mixed()), $call->param_types[1] ?? RustType::list(RustType::mixed())) . ')?', $call->return_type);
                return $this->narrow($v, $e);
            }
            $this->warn('unknown method ' . $name . ' on ' . $rt->toRust(), $e);
            return new Val('unreachable!("unknown method ' . $name . '")', $this->inferredOrMixed($e));
        }
        if ($rt->kind === RustType::UNION) {
            $res = $this->inferredOrMixed($e);
            $arms = [];
            $tmp = $this->tmp('__r');
            foreach ($rt->params as $member) {
                if ($member->kind !== RustType::CLASS_) {
                    continue;
                }
                $cls = $this->program->classOf($member);
                $m = $cls !== null ? $this->program->findMethod($cls, $lc) : null;
                if ($m === null) {
                    continue;
                }
                $argc = $this->args($args, $m->storage, $m->param_types, $m->declaring, $m->name);
                $call = '__o.' . $m->rustName() . '(' . implode(', ', $argc) . ')?';
                $arms[] = $rt->mangle() . '::' . $member->variantName() . '(__o) => ' . $this->casts->convert($call, $m->return_type, $res);
            }
            if ($arms !== []) {
                return new Val('(match ' . $recv->code . ' { ' . implode(', ', $arms) . ', _ => unreachable!() })', $res);
            }
        }
        if ($rt->kind === RustType::RT_GENERIC || $rt->kind === RustType::DYN_CALLABLE) {
            return $this->builtins->emitRuntimeMethod($this, $recv, $name, $args, $e);
        }
        if ($rt->kind === RustType::CLOSURE && ($lc === '__invoke' || $lc === 'call')) {
            return $this->callValue($recv, $args, $e);
        }
        if ($rt->kind === RustType::MIXED || $rt->kind === RustType::ANY_OBJECT) {
            $argc = [];
            foreach ($args as $a) {
                $argc[] = $this->exprTo($a->value, RustType::mixed());
            }
            $v = new Val('mixed_call(&' . $this->casts->convert($recv->code, $rt, RustType::mixed()) . ', &' . Names::strLit($name) . ', vec![' . implode(', ', $argc) . '])?', RustType::mixed());
            return $this->narrow($v, $e);
        }
        $this->warn('method call on ' . $rt->toRust(), $e);
        return new Val('unreachable!("method call on ' . $rt->toRust() . '")', $this->inferredOrMixed($e));
    }

    private function staticCall(Expr\StaticCall $e): Val
    {
        if ($e->isFirstClassCallable()) {
            return $this->firstClassCallable($e);
        }
        if (!$e->name instanceof Identifier) {
            $this->warn('dynamic static method name', $e);
            return new Val('unreachable!("dynamic static method")', $this->inferredOrMixed($e));
        }
        $name = $e->name->name;
        $lc = strtolower($name);
        $args = $e->getArgs();
        if (!$e->class instanceof Name) {
            // $obj::method() / $class::method()
            $cv = $this->expr($e->class);
            if ($cv->type->kind === RustType::CLASS_) {
                return $this->methodCallOn($cv, $name, $e);
            }
            $this->warn('static call on expression', $e);
            return new Val('unreachable!("static call on expression")', $this->inferredOrMixed($e));
        }
        $kind = strtolower($e->class->toString());
        $fqcn = $this->resolveClassName($e->class);
        $rt = $this->builtins->emitRuntimeStatic($this, strtolower((string) $fqcn), $lc, $args, $e);
        if ($rt !== null) {
            return $rt;
        }
        $cls = $fqcn !== null ? $this->program->getClass($fqcn) : null;
        if ($cls === null) {
            $this->warn('static call on unknown class ' . $fqcn, $e);
            return new Val('unreachable!("unknown class ' . $fqcn . '")', $this->inferredOrMixed($e));
        }
        if ($cls->isEnum()) {
            return $this->enumStaticCall($cls, $lc, $args, $e, null);
        }
        $m = $this->program->findMethod($cls, $lc);
        if ($m === null) {
            $this->warn('unknown static method ' . $fqcn . '::' . $name, $e);
            return new Val('unreachable!("unknown static method ' . $name . '")', $this->inferredOrMixed($e));
        }
        $argc = $this->args($args, $m->storage, $m->param_types, $m->declaring, $m->name);
        if ($m->isStatic()) {
            $target = $m->declaring;
            if ($m->uses_lsb) {
                if (in_array($kind, ['static', 'self', 'parent'], true) && $this->static_class !== null) {
                    $target = $this->static_class; // forwarded late static binding
                } elseif ($kind === 'static' && $this->this_type !== null && $this->class !== null) {
                    if (!$this->class->isLeaf()) {
                        return new Val($this->this_expr . '.' . $m->rustName() . '__static(' . implode(', ', $argc) . ')?', $m->return_type);
                    }
                    $target = $this->class;
                } elseif ($kind !== 'self' && $kind !== 'parent') {
                    $target = $cls; // explicitly named class
                }
            }
            return new Val($target->path() . '::' . $m->rustName() . '(' . implode(', ', $argc) . ')?', $m->return_type);
        }
        // instance method called with self::/parent::/static:: => non-virtual call on $this
        if ($this->this_type === null) {
            $this->warn('instance method called statically', $e);
            return new Val('unreachable!("instance method called statically")', $m->return_type);
        }
        if ($kind === 'static') {
            return new Val($this->this_expr . '.' . $m->rustName() . '(' . implode(', ', $argc) . ')?', $m->return_type);
        }
        $decl = $m->declaring;
        $this_t = $this->this_type;
        $recv = $this->casts->convert($this->this_expr . '.clone()', $this_t, RustType::class($decl->fqcn));
        $impl = $decl->isLeaf() ? $m->rustName() : $m->rustName() . '__impl';
        if ($m->isAbstract()) {
            $impl = $m->rustName();
        }
        return new Val($recv . '.' . $impl . '(' . implode(', ', $argc) . ')?', $m->return_type);
    }

    private function enumStaticCall(ClassModel $cls, string $lc, array $args, Expr $e, ?Val $recv): Val
    {
        $path = $cls->path();
        $t = RustType::class($cls->fqcn);
        $backing = $cls->storage->enum_type === 'int' ? RustType::int() : RustType::str();
        switch ($lc) {
            case 'cases':
                return new Val($path . '::cases()', RustType::list($t));
            case 'from':
                return new Val($path . '::from_value(' . $this->exprTo($args[0]->value, $backing) . ')?', $t);
            case 'tryfrom':
                return new Val($path . '::try_from_value(' . $this->exprTo($args[0]->value, $backing) . ')', RustType::option($t));
        }
        $m = $this->program->findMethod($cls, $lc);
        if ($m !== null) {
            $argc = $this->args($args, $m->storage, $m->param_types, $m->declaring, $m->name);
            if ($m->isStatic() || $recv === null) {
                return new Val($path . '::' . $m->rustName() . '(' . implode(', ', $argc) . ')?', $m->return_type);
            }
            return new Val($recv->code . '.' . $m->rustName() . '(' . implode(', ', $argc) . ')?', $m->return_type);
        }
        $this->warn('unknown enum method ' . $lc, $e);
        return new Val('unreachable!("unknown enum method")', $this->inferredOrMixed($e));
    }

    // ------------------------------------------------------------------ new

    private function newExpr(Expr\New_ $e, ?RustType $expected = null): Val
    {
        $args = $e->getArgs();
        if ($e->class instanceof Expr\ClassLike) {
            // anonymous class
            $fqcn = $e->class->getAttribute('anonymous_fqcn');
            $cls = null;
            foreach ($this->program->classes as $c) {
                if ($c->node === $e->class) {
                    $cls = $c;
                }
            }
            if ($cls === null) {
                $this->warn('anonymous class without model', $e);
                return new Val('unreachable!("anonymous class")', $this->inferredOrMixed($e));
            }
            return $this->construct($cls, $args, $e);
        }
        if ($e->class instanceof Name) {
            $fqcn = $this->resolveClassName($e->class);
            $inf = $this->inferred($e);
            if ($inf !== null && $inf->kind === RustType::RT_GENERIC) {
                $exp = $expected !== null && $expected->kind === RustType::OPTION ? $expected->inner() : $expected;
                if ($exp !== null && $exp->kind === RustType::RT_GENERIC && $exp->name === $inf->name) {
                    $inf = $exp;
                }
                return $this->newRuntimeGeneric($inf, $args, $e);
            }
            $cls = $fqcn !== null ? $this->program->getClass($fqcn) : null;
            if ($cls === null) {
                $this->warn('new on unknown class ' . $fqcn, $e);
                return new Val('unreachable!("new ' . $fqcn . '")', $this->inferredOrMixed($e));
            }
            if (strtolower($e->class->toString()) === 'static' && $this->class !== null && !$this->class->isLeaf() && $this->this_type !== null) {
                $argc = $this->constructorArgs($cls, $args);
                return new Val($this->this_expr . '.new_same_class(' . implode(', ', $argc) . ')?', RustType::class($cls->fqcn));
            }
            return $this->construct($cls, $args, $e);
        }
        // new $className(...)
        $cv = $this->expr($e->class);
        $inf = $this->inferred($e);
        $target = $inf !== null && $inf->kind === RustType::CLASS_ ? $this->program->classOf($inf) : null;
        if ($cv->type->kind === RustType::CLASS_) {
            // new $object => same class as the object
            $cls = $this->program->classOf($cv->type);
            if ($cls !== null) {
                $argc = $this->constructorArgs($cls, $args);
                return new Val($cv->code . '.new_same_class(' . implode(', ', $argc) . ')?', RustType::class($cls->fqcn));
            }
        }
        if ($target !== null) {
            $argc = $this->constructorArgs($target, $args);
            $this->program->needFactory($target, count($argc));
            return new Val($target->path() . '::new_by_name(&' . $this->casts->convert($cv->code, $cv->type, RustType::str()) . ', ' . implode(', ', $argc) . ')?', RustType::class($target->fqcn));
        }
        $this->warn('dynamic new without known class', $e);
        return new Val('unreachable!("dynamic new")', $inf ?? RustType::anyObject());
    }

    private function newRuntimeGeneric(RustType $t, array $args, Expr $e): Val
    {
        $params = implode(', ', array_map(fn(RustType $p) => $p->toRust(), $t->params));
        $generic = $params === '' ? $t->name : $t->name . '::<' . $params . '>';
        switch ($t->name) {
            case 'SplObjectStorage':
            case 'WeakMap':
                return new Val($generic . '::new()', $t);
            case 'ArrayObject':
            case 'ArrayIterator':
                $mt = RustType::map($t->params[0]->kind === RustType::INT ? RustType::int() : ($t->params[0]->kind === RustType::STR ? RustType::str() : RustType::arrayKey()), $t->params[1]);
                $init = isset($args[0]) ? $this->exprTo($args[0]->value, $mt) : 'Map::new()';
                return new Val($generic . '::new(' . $init . ')', $t);
            case 'StdClass':
                return new Val('StdClass::new()', $t);
        }
        $this->warn('new on runtime class ' . $t->name, $e);
        return new Val('unreachable!("new ' . $t->name . '")', $t);
    }

    private function constructorArgs(ClassModel $cls, array $args): array
    {
        $ctor = $this->program->findMethod($cls, '__construct');
        if ($ctor === null) {
            return [];
        }
        return $this->args($args, $ctor->storage, $ctor->param_types, $ctor->declaring, '__construct');
    }

    public function construct(ClassModel $cls, array $args, Expr $e): Val
    {
        $argc = $this->constructorArgs($cls, $args);
        $t = RustType::class($cls->fqcn);
        if (!$cls->is_project) {
            $this->warn('new on external class ' . $cls->fqcn, $e);
        }
        return new Val($cls->path() . '::new(' . implode(', ', $argc) . ')?', $t);
    }
}
