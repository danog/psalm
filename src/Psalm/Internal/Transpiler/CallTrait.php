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
use function preg_match;
use function array_pop;
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
    /** @var list<string> hoisted argument evaluations (`let` statements) pending for the next finishCall() */
    private array $pending_pre = [];

    /**
     * Wraps a call expression with the argument evaluations hoisted by the matching args() call:
     * by-reference arguments borrow their place mutably, so the other arguments (which may read the
     * same variable) are evaluated into temporaries first.
     */
    public function finishCall(string $call): string
    {
        [$pre, $post] = array_pop($this->pending_pre) ?? ['', ''];
        if ($pre === '' && $post === '') {
            return $call;
        }
        if ($post === '') {
            return '{ ' . $pre . ' ' . $call . ' }';
        }
        return '{ ' . $pre . ' let __cr = ' . $call . '; ' . $post . ' __cr }';
    }

    /** @var list<array{string, string}> per-args() frames of (pre, post) code produced by byRefArg() */
    private array $byref_frames = [];

    /** @param array<int, true> $borrow_params param indices the callee receives as `&T` (owned/borrowed axis 5) */
    public function args(array $args, FunctionLikeStorage $storage, array $param_types, ?ClassModel $callee_class, string $callee_name, array $borrow_params = []): array
    {
        $this->byref_frames[] = ['', ''];
        $out = $this->argsInner($args, $storage, $param_types, $callee_class, $callee_name, $borrow_params);
        [$pre, $post] = array_pop($this->byref_frames);
        $has_byref = false;
        foreach (array_values($storage->params) as $i => $p) {
            if ($p->by_ref && isset($out[$i])) {
                $has_byref = true;
            }
        }
        if ($has_byref) {
            foreach (array_values($storage->params) as $i => $p) {
                if ($p->by_ref || !isset($out[$i]) || preg_match('/^[A-Za-z_][A-Za-z0-9_]*$|^-?[0-9]+(?:i64|\.0)?$|^(?:true|false|None)$/', $out[$i])) {
                    continue;
                }
                $tmp = $this->tmp('__h');
                $pre .= 'let ' . $tmp . ' = ' . $out[$i] . '; ';
                $out[$i] = $tmp;
            }
        }
        $this->pending_pre[] = [$pre, $post];
        return $out;
    }

    /** Code evaluated before the call being built (see finishCall). */
    private function addPre(string $code): void
    {
        if ($this->byref_frames === []) {
            $this->byref_frames[] = ['', ''];
        }
        $this->byref_frames[count($this->byref_frames) - 1][0] .= $code . ' ';
    }

    /** @param array<int, true> $borrow_params */
    private function argsInner(array $args, FunctionLikeStorage $storage, array $param_types, ?ClassModel $callee_class, string $callee_name, array $borrow_params = []): array
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
        /** @var array{string, int}|null hoisted list of a spread argument and the parameter index it starts at */
        $unpacked = null;
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
            if ($unpacked !== null) {
                // parameters fed by a spread argument (`f(...$pair)`): element i-k of the hoisted list
                $k = $i - $unpacked[1];
                $default = ($param->by_ref ? '&mut ' : '') . $this->defaultArg($param, $t, $callee_class, $callee_name, $i);
                $out[] = '(match ' . $unpacked[0] . '.get(' . $k . ').cloned() { Some(__ua) => ' . $this->casts->convert('__ua', $unpacked[2], $t) . ', None => ' . $default . ' })';
                continue;
            }
            if ($arg !== null && $arg->unpack) {
                // a spread list keeps its element type (a typed `explode()` result feeds typed parameters)
                $sv = $this->expr($arg->value);
                $et = $sv->type->kind === RustType::LIST ? $sv->type->inner() : RustType::mixed();
                $lt = RustType::list($et);
                $tmp = $this->tmp('__ul');
                $this->addPre('let ' . $tmp . ': ' . $lt->toRust() . ' = ' . $this->casts->convert($sv->code, $sv->type, $lt) . ';');
                $unpacked = [$tmp, $i, $et];
                $out[] = '(match ' . $tmp . '.get(0).cloned() { Some(__ua) => ' . $this->casts->convert('__ua', $et, $t) . ', None => ' . $this->defaultArg($param, $t, $callee_class, $callee_name, $i) . ' })';
                continue;
            }
            if ($arg === null) {
                $out[] = ($param->by_ref ? '&mut ' : '') . $this->defaultArg($param, $t, $callee_class, $callee_name, $i);
                continue;
            }
            if ($param->by_ref) {
                $out[] = $this->byRefArg($arg->value, $t);
                continue;
            }
            if (isset($borrow_params[$i])) {

                $out[] = $this->borrowArg($arg->value, $t);
                continue;
            }
            $out[] = $this->exprTo($arg->value, $t);
        }
        return $out;
    }

    /** Pass an lvalue as `&mut T`. */
    private function byRefArg(Expr $e, RustType $t): string
    {
        if (!($e instanceof Expr\Variable || $e instanceof Expr\PropertyFetch || $e instanceof Expr\StaticPropertyFetch || $e instanceof Expr\ArrayDimFetch)) {
            // not an lvalue: the callee writes into a temporary
            return '&mut ' . $this->exprTo($e, $t);
        }
        $place = $this->place($e);
        if ($place->hasMut() && $place->type->toRust() === $t->toRust()) {
            return '&mut ' . $place->mut();
        }
        // a differently typed (or borrow-less) place: pass a converted temporary and write it back after the call
        $tmp = $this->tmp('__ref');
        $frame = count($this->byref_frames) - 1;
        if ($frame < 0) {
            $this->byref_frames[] = ['', ''];
            $frame = 0;
        }
        $this->byref_frames[$frame][0] .= 'let mut ' . $tmp . ': ' . $t->toRust() . ' = ' . $this->casts->convert($place->read(), $place->type, $t) . '; ';
        $this->byref_frames[$frame][1] .= $place->write($this->casts->convert($tmp, $t, $place->type)) . ' ';
        return '&mut ' . $tmp;
    }

    /**
     * Owned/borrowed (axis 5): pass an argument as `&T` to a borrow-safe param, without cloning where the
     * value is already available as `T` (a local of matching type, or a Late local). Otherwise borrow a
     * converted temporary (`&expr`), which still avoids an extra Rc clone for a computed owned value.
     */
    private function borrowArg(Expr $e, RustType $t): string
    {
        if ($e instanceof Expr\Variable && is_string($e->name) && $e->name !== 'this'
            && isset($this->vars[$e->name])
            && $this->vars[$e->name]->toRust() === $t->toRust()
            && empty($this->cells[$e->name]) && empty($this->refvars[$e->name])
            && empty($this->byref[$e->name]) && empty($this->globals[$e->name])
        ) {
            $name = $e->name;
            $rn = Names::var($name);
            if (!empty($this->borrow[$name])) {
                return $rn; // already `&T`
            }
            if (!empty($this->late[$name])) {
                return $rn . '.get()'; // `&T` into the Late's stored value
            }
            if (!$this->vars[$name]->isCopy()) {
                return '&' . $rn; // plain local: borrow in place, no clone
            }
        }
        return '&' . $this->exprTo($e, $t);
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
            $argc = $this->args($args, $fn->record->storage, $fn->param_types, null, $fn->fq_name, $fn->borrow_params);
            return new Val($this->finishCall($fn->path() . '(' . implode(', ', $argc) . ')' . ($fn->throws ? '?' : '')), $fn->return_type);
        }

        $result = $this->builtins->emit($this, $e, strtolower($short), $args);
        if ($result !== null) {
            return $result;
        }
        $this->warn('unknown function ' . $resolved, $e);
        // dead in the closed world: typed by what the site expects (a declared return, a parameter) when known
        $exp = $this->call_expected;
        $inf = $this->inferredOrMixed($e);
        $dead_t = $exp !== null && !$exp->containsMixed() && !$exp->hasGeneric() ? $exp : ($inf->containsMixed() ? RustType::unit() : $inf);
        return $this->dead('unknown function ' . $resolved . '', $dead_t);
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
            return new Val('(' . $callee->code . ')(' . implode(', ', $argc) . ')', $t->ret);
        }
        if ($t->kind === RustType::DYN_CALLABLE || $t->kind === RustType::MIXED || $t->kind === RustType::STR) {
            if ($t->kind === RustType::STR) {
                $this->warn('string used as a callable', $site);
            }
            $argc = [];
            foreach ($args as $a) {
                $argc[] = $this->exprTo($a->value, RustType::mixed());
            }
            $conv = $t->kind === RustType::DYN_CALLABLE ? $callee->code : 'to_callable(&' . $this->casts->convert($callee->code, $t, RustType::mixed()) . ')';
            return $this->narrow(new Val($conv . '.call(vec![' . implode(', ', $argc) . '])', RustType::mixed()), $site);
        }
        if ($t->kind === RustType::CLASS_) {
            $cls = $this->program->classOf($t);
            $m = $cls !== null ? $this->program->findMethod($cls, '__invoke') : null;
            if ($m !== null) {
                $argc = $this->args($args, $m->storage, $m->param_types, $m->declaring, $m->name, $m->borrow_params);
                return new Val($this->finishCall($callee->code . '.' . $m->rustName() . '(' . implode(', ', $argc) . ')' . ($m->throws ? '?' : '')), $m->return_type);
            }
        }
        $this->warn('call of ' . $t->toRust(), $site);
        return $this->dead('call of ' . $t->toRust() . '', $this->inferredOrMixed($site));
    }

    private function firstClassCallable(Expr\FuncCall|Expr\MethodCall|Expr\StaticCall $e): Val
    {
        $inf = $this->inferred($e);
        if ($inf === null || $inf->kind !== RustType::CLOSURE) {
            $this->warn('first-class callable without signature', $e);
            return $this->dead('first-class callable', $inf ?? RustType::dynCallable());
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
        // locals the receiver / class expression mentions are captured by value
        $receiver = $e instanceof Expr\MethodCall ? $e->var : ($e instanceof Expr\StaticCall ? $e->class : null);
        if ($receiver instanceof \PhpParser\Node) {
            foreach ((new \PhpParser\NodeFinder())->findInstanceOf([$receiver], Expr\Variable::class) as $var) {
                if (is_string($var->name) && $var->name !== 'this' && isset($this->vars[$var->name])) {
                    $rv = Names::var($var->name);
                    $capt .= 'let ' . $rv . ' = ' . $this->readVar($var->name)->code . '; ';
                }
            }
        }
        $decls = [];
        foreach ($inf->params as $i => $p) {
            $decls[] = 'let __fcc' . $i . ' = __p' . $i . ';';
        }
        $code = '{ ' . $capt . 'Rc::new(move |' . implode(', ', $params) . '| -> ' . $inf->ret->toRust() . ' { ' . implode(' ', $decls) . ' ' . str_replace('self.', 'this.', $v->code) . ' }) as ' . $inf->toRust() . ' }';
        return new Val($code, $inf);
    }

    // ------------------------------------------------------------------ methods

    private function methodCall(Expr\MethodCall|Expr\NullsafeMethodCall $e, bool $nullsafe): Val
    {
        if ($e instanceof Expr\MethodCall && $e->isFirstClassCallable()) {
            return $this->firstClassCallable($e);
        }
        if (!$e->name instanceof Identifier) {
            // `$obj->$name()`: methods are never looked up by name (closed world)
            $this->warn('dynamic method name', $e);
            return $this->dead('dynamic method name', $this->inferredOrMixed($e));
        }
        $name = $e->name->name;
        $lc = strtolower($name);
        // the receiver of `?->` is read with its declared type: Psalm narrows it to non-null for the call itself
        $recv = $nullsafe ? $this->rawValue($e->var) : $this->receiver($e->var);
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
                $argc = $this->args($args, $m->storage, $m->param_types, $m->declaring, $m->name, $m->borrow_params);
                if ($m->isStatic()) {
                    if (!$cls->isLeaf() && !$m->isPrivate() && !$m->declaring->isEnum()) {
                        // a static method called on an instance: the runtime class' implementation
                        return new Val($this->finishCall($recv->code . '.' . $m->rustName() . '__static(' . implode(', ', $argc) . ')'), $m->return_type);
                    }
                    return new Val('{ let _ = ' . $recv->code . '; ' . $this->finishCall($m->declaring->path() . '::' . $m->rustName() . '(' . implode(', ', $argc) . ')') . ' }', $m->return_type);
                }
                // Owned/borrowed (axis 5): a `&self` method on a plain local receiver only needs to BORROW it — drop
                // the defensive Rc-clone (`x.clone().m()` -> `x.m()`), saving a refcount bump on the hot call path.
                // Safe: generated methods take &self/&mut self (never `self` by value) and return owned values, so the
                // borrow is confined to the call. Only for &self (not immutable construction methods, which are &mut
                // self and must own a fresh value); &self allows any number of concurrent borrows so args can't conflict.
                $recv_code = $recv->code;
                if ($e instanceof Expr\MethodCall && $e->var instanceof Expr\Variable && is_string($e->var->name)
                    && $e->var->name !== 'this'
                    && !($m->declaring->immutable() && isset($m->declaring->constructionMethods()[$m->lc()]))
                ) {
                    $bind = Names::var($e->var->name);
                    if ($recv_code === $bind . '.clone()') {
                        // plain local: `x.clone().m()` -> `x.m()`
                        $recv_code = $bind;
                    } elseif ($recv_code === $bind . '.get().clone()') {
                        // Late local: `x.get().clone().m()` -> `x.get().m()` (borrow the stored value)
                        $recv_code = $bind . '.get()';
                    }
                }
                return new Val($this->finishCall($recv_code . '.' . $m->rustName() . '(' . implode(', ', $argc) . ')' . ($m->throws ? '?' : '')), $m->return_type);
            }
            if ($cls !== null && $cls->isEnum()) {
                return $this->enumStaticCall($cls, $lc, $args, $e, $recv);
            }
            // an intersection-typed receiver (`$atomic instanceof DependentType` narrows to Atomic&DependentType):
            // the method lives on an intersected interface — downcast the handle to that interface's enum and call
            // it there (closed dispatch), instead of falling back to the dynamic protocol
            $psalm = $e instanceof Expr\MethodCall || $e instanceof Expr\NullsafeMethodCall ? $this->psalmType($e->var) : null;
            if ($psalm !== null) {
                foreach ($psalm->getAtomicTypes() as $atomic) {
                    if (!$atomic instanceof \Psalm\Type\Atomic\TNamedObject) {
                        continue;
                    }
                    foreach ($atomic->extra_types as $extra) {
                        if (!$extra instanceof \Psalm\Type\Atomic\TNamedObject) {
                            continue;
                        }
                        $it = $this->types()->mapAtomic($extra);
                        $icls = $it->kind === RustType::CLASS_ ? $this->program->classOf($it) : null;
                        if ($icls === null || $icls === $cls || $this->program->findMethod($icls, $lc) === null) {
                            continue;
                        }
                        $this->casts->need($rt, $it);
                        return $this->methodCallOn(new Val($this->casts->convert($recv->code, $rt, $it), $it), $name, $e);
                    }
                }
            }
            // magic __call
            $call = $cls !== null ? $this->program->findMethod($cls, '__call') : null;
            if ($call !== null) {
                $argc = [];
                foreach ($args as $a) {
                    $argc[] = $this->exprTo($a->value, RustType::mixed());
                }
                $list = $argc === [] ? 'List::new()' : 'list![' . implode(', ', $argc) . ']';
                $v = new Val($recv->code . '.' . $call->rustName() . '(' . Names::strLit($name) . ', ' . $this->casts->convert($list, RustType::list(RustType::mixed()), $call->param_types[1] ?? RustType::list(RustType::mixed())) . ')', $call->return_type);
                return $this->narrow($v, $e);
            }
            // not declared on the static type (e.g. `hasAttribute` on a `DOMNode`): dispatched by name at runtime
            $this->warn('unknown method ' . $name . ' on ' . $rt->toRust() . ' (dynamic call)', $e);
            \fwrite(\STDERR, "[dyn-used-method] $lc\n");
            if (!isset(ClassEmitter::DYN_DISPATCH_METHODS[$lc])) {
                \fwrite(\STDERR, "[dyn-allowlist-MISS] $lc (call_method) — add to ClassEmitter::DYN_DISPATCH_METHODS or its arm is elided\n");
            }
            $argc = [];
            foreach ($args as $a) {
                $argc[] = $this->exprTo($a->value, RustType::mixed());
            }
            $res = $this->inferredOrMixed($e);
            $call = 'php_rt::other_obj(&' . $this->casts->convert($recv->code, $rt, RustType::mixed()) . ').call_method(' . Names::rustStringLiteral($lc) . ', vec![' . implode(', ', $argc) . '])';
            return $this->narrow(new Val($this->casts->convert($call, RustType::mixed(), $res), $res), $e);
        }
        if ($rt->kind === RustType::UNION) {
            $res = $this->inferredOrMixed($e);
            if ($res->containsMixed()) {
                // Psalm sees `mixed` (the method is declared on some members only): the members that declare
                // it agree on the result type
                $arm_types = [];
                foreach ($rt->params as $member) {
                    $mc = $member->kind === RustType::CLASS_ ? $this->program->classOf($member) : null;
                    $mm = $mc !== null ? $this->program->findMethod($mc, $lc) : null;
                    if ($mm !== null) {
                        $arm_types[] = $mm->return_type;
                    }
                }
                if ($arm_types !== []) {
                    $combined = $this->types()->combine($arm_types);
                    if (!$combined->containsMixed() && !$combined->hasGeneric()) {
                        $res = $combined;
                    }
                }
            }
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
                $argc = $this->args($args, $m->storage, $m->param_types, $m->declaring, $m->name, $m->borrow_params);
                $call = $this->finishCall('__o.' . $m->rustName() . '(' . implode(', ', $argc) . ')' . ($m->throws ? '?' : ''));
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
            \fwrite(\STDERR, "[dyn-used-method] " . strtolower($name) . "\n");
            if (!isset(ClassEmitter::DYN_DISPATCH_METHODS[strtolower($name)])) {
                \fwrite(\STDERR, "[dyn-allowlist-MISS] " . strtolower($name) . " (mixed_call) — add to ClassEmitter::DYN_DISPATCH_METHODS or its arm is elided\n");
            }
            $v = new Val('mixed_call(&' . $this->casts->convert($recv->code, $rt, RustType::mixed()) . ', &' . Names::strLit($name) . ', vec![' . implode(', ', $argc) . '])', RustType::mixed());
            return $this->narrow($v, $e);
        }
        $this->warn('method call on ' . $rt->toRust(), $e);
        return $this->dead('method call on ' . $rt->toRust() . '', $this->inferredOrMixed($e));
    }

    private function staticCall(Expr\StaticCall $e): Val
    {
        if ($e->isFirstClassCallable()) {
            return $this->firstClassCallable($e);
        }
        if (!$e->name instanceof Identifier) {
            $this->warn('dynamic static method name', $e);
            return $this->dead('dynamic static method', $this->inferredOrMixed($e));
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
            if ($cv->type->kind === RustType::UNION || $cv->type->kind === RustType::ANY_OBJECT || $cv->type->kind === RustType::OPTION) {
                return $this->methodCallOn($cv, $name, $e);
            }
            // `$class::method(...)` with a class name: classes are never looked up by name (closed world)
            $this->warn('static call on a class name', $e);
            return $this->dead('static call on a class name', $this->inferredOrMixed($e));
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
            return $this->dead('unknown class ' . $fqcn . '', $this->inferredOrMixed($e));
        }
        if (!$cls->is_project) {
            // no generated code: a defaultable result (false for `X::isEnabled()`) or a dead value
            $this->warn('static call on external class ' . $fqcn, $e);
            $t = $this->inferredOrMixed($e);
            $m = $this->program->findMethod($cls, $lc);
            if ($m !== null && $m->return_type->hasDefault() && $m->return_type->kind !== RustType::MIXED) {
                $t = $m->return_type;
            }
            foreach ($args as $a) {
                if ($a instanceof \PhpParser\Node\Arg) {
                    $this->expr($a->value);
                }
            }
            return $t->hasDefault() ? new Val('<' . $t->toRust() . '>::default()', $t) : $this->dead('external static call ' . $fqcn . '::' . $name, $t);
        }
        if ($cls->isEnum()) {
            return $this->enumStaticCall($cls, $lc, $args, $e, null);
        }
        $m = $this->program->findMethod($cls, $lc);
        if ($m === null) {
            $this->warn('unknown static method ' . $fqcn . '::' . $name, $e);
            return $this->dead('unknown static method ' . $name . '', $this->inferredOrMixed($e));
        }
        $argc = $this->args($args, $m->storage, $m->param_types, $m->declaring, $m->name, $m->borrow_params);
        if ($m->isStatic()) {
            $target = $m->declaring;
            if ($m->uses_lsb && !$m->isPrivate()) {
                $bound = $this->static_class ?? $this->class;
                if ($kind === 'static' && $this->static_class === null && $this->this_type !== null && $this->class !== null && !$this->class->isLeaf()) {
                    return new Val($this->finishCall($this->this_expr . '.' . $m->rustName() . '__static(' . implode(', ', $argc) . ')'), $m->return_type);
                }
                if (in_array($kind, ['static', 'self', 'parent'], true) && $bound !== null) {
                    // forwarded late static binding: `static` stays bound to the calling class
                    $bm = $this->program->findMethod($bound, $lc);
                    if (($kind === 'parent' || $kind === 'self') && $bm !== null && $bm->origin() !== $m->origin()) {
                        // `self::`/`parent::` name a specific body that the bound class overrides: a copy of that
                        // body with `static` bound to the calling class
                        $copy = $this->program->requestSuperCopy($bound, $m);
                        return new Val($this->finishCall($bound->path() . '::' . $copy . '(' . implode(', ', $argc) . ')'), $m->return_type);
                    }
                    $target = $bound;
                } elseif ($kind !== 'self' && $kind !== 'parent') {
                    $target = $cls; // explicitly named class
                }
            }
            return new Val($this->finishCall($target->path() . '::' . $m->rustName() . '(' . implode(', ', $argc) . ')'), $m->return_type);
        }
        // instance method called with self::/parent::/static:: => non-virtual call on $this
        if ($this->this_type === null) {
            $this->warn('instance method called statically', $e);
            $this->finishCall('');
            return $this->dead('instance method called statically', $m->return_type);
        }
        if ($kind === 'static') {
            return new Val($this->finishCall($this->this_expr . '.' . $m->rustName() . '(' . implode(', ', $argc) . ')' . ($m->throws ? '?' : '')), $m->return_type);
        }
        // Immutable Rc<T>: a self::/parent:: call to a construction method (writes $this) must mutate THIS object in
        // place. The generic path casts `self.clone()` to the declaring class and calls `__impl`, but for Rc<T> the
        // clone bumps the refcount so make_mut copies and the writes are lost (and the enum omits __impl for immutable
        // hierarchies). Emit a super-copy of the body on the current (leaf) class and call it on `self` directly
        // (&mut self) — during construction refcount is 1 so make_mut mutates in place. Pairs with emitMethodOnOwn's
        // inherited-ctor forwarding to run the whole inherited ctor chain in place.
        if ($this->class !== null && !$m->isAbstract() && $this->class->isImmutableCtorMethod($m->lc())) {
            $copy = $this->program->requestSuperCopy($this->class, $m);
            return new Val($this->finishCall($this->this_expr . '.' . $copy . '(' . implode(', ', $argc) . ')' . ($m->throws ? '?' : '')), $m->return_type);
        }
        $decl = $m->declaring;
        $this_t = $this->this_type;
        if ($this->class !== null && $decl->is_project && $decl->crate < $this->class->crate && !$m->isAbstract()) {
            // the body lives in another crate whose dispatch enum cannot hold `$this`: call a copy of it
            // emitted on this crate's topmost class
            $root = $this->class->crateRoot();
            $copy = $this->program->requestSuperCopy($root, $m);
            $recv = $this->casts->convert($this->this_expr . '.clone()', $this_t, RustType::class($root->fqcn));
            return new Val($this->finishCall($recv . '.' . $copy . '(' . implode(', ', $argc) . ')' . ($m->throws ? '?' : '')), $m->return_type);
        }
        $recv = $this->casts->convert($this->this_expr . '.clone()', $this_t, RustType::class($decl->fqcn));
        $impl = $decl->isLeaf() ? $m->rustName() : $m->rustName() . '__impl';
        if ($m->isAbstract()) {
            $impl = $m->rustName();
        }
        return new Val($this->finishCall($recv . '.' . $impl . '(' . implode(', ', $argc) . ')' . ($m->throws ? '?' : '')), $m->return_type);
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
                return new Val($path . '::from_value(' . $this->exprTo($args[0]->value, $backing) . ')', $t);
            case 'tryfrom':
                return new Val($path . '::try_from_value(' . $this->exprTo($args[0]->value, $backing) . ')', RustType::option($t));
        }
        $m = $this->program->findMethod($cls, $lc);
        if ($m !== null) {
            $argc = $this->args($args, $m->storage, $m->param_types, $m->declaring, $m->name, $m->borrow_params);
            if ($m->isStatic() || $recv === null) {
                return new Val($this->finishCall($path . '::' . $m->rustName() . '(' . implode(', ', $argc) . ')'), $m->return_type);
            }
            return new Val($this->finishCall($recv->code . '.' . $m->rustName() . '(' . implode(', ', $argc) . ')'), $m->return_type);
        }
        $this->warn('unknown enum method ' . $lc, $e);
        return $this->dead('unknown enum method', $this->inferredOrMixed($e));
    }

    // ------------------------------------------------------------------ new

    private function newExpr(Expr\New_ $e, ?RustType $expected = null): Val
    {
        $args = $e->getArgs();
        if ($e->class instanceof \PhpParser\Node\Stmt\Class_) {
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
                return $this->dead('anonymous class', $this->inferredOrMixed($e));
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
                return $this->dead('new ' . $fqcn . '', $this->inferredOrMixed($e));
            }
            if (strtolower($e->class->toString()) === 'static' && $this->class !== null && !$this->class->isLeaf() && $this->this_type !== null) {
                $argc = $this->constructorArgs($cls, $args);
                return new Val($this->this_expr . '.new_same_class(' . implode(', ', $argc) . ')', RustType::class($cls->fqcn));
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
                return new Val($cv->code . '.new_same_class(' . implode(', ', $argc) . ')', RustType::class($cls->fqcn));
            }
        }
        // `new $class(...)` with a class name: classes are never instantiated by name (closed world)
        $this->warn('new on a class name', $e);
        return $this->dead('new on a class name', $inf ?? RustType::anyObject());
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
        return $this->dead('new ' . $t->name . '', $t);
    }

    private function constructorArgs(ClassModel $cls, array $args): array
    {
        $ctor = $this->program->findMethod($cls, '__construct');
        if ($ctor === null) {
            return [];
        }
        return $this->args($args, $ctor->storage, $ctor->param_types, $ctor->declaring, '__construct', $ctor->borrow_params);
    }

    public function construct(ClassModel $cls, array $args, Expr $e): Val
    {
        if (!$cls->is_project) {
            $this->warn('new on external class ' . $cls->fqcn, $e);
            return $this->dead('new on external class ' . $cls->fqcn, RustType::anyObject());
        }
        $argc = $this->constructorArgs($cls, $args);
        $t = RustType::class($cls->fqcn);
        return new Val($this->finishCall($cls->path() . '::new(' . implode(', ', $argc) . ')'), $t);
    }
}
