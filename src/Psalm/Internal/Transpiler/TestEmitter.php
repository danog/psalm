<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use function array_keys;
use function array_slice;
use function basename;
use function count;
use function implode;
use function in_array;
use function max;
use function preg_match;
use function preg_match_all;
use function str_replace;
use function str_starts_with;
use function strtolower;

/**
 * Generates a Rust `#[test]` function for every PHPUnit test method of the transpiled test classes.
 *
 * @internal
 */
final class TestEmitter
{
    public function __construct(
        private readonly Program $program,
        private readonly Casts $casts,
        private readonly Builtins $builtins,
        private readonly Diagnostics $diag,
    ) {
    }

    /**
     * The harness is a `libtest-mimic` binary: every test method contributes one trial per data set (data
     * providers run once at start-up to name the sets), so the counts match PHPUnit's.
     */
    public function emit(Writer $w, int $crate): int
    {
        $base = $this->program->getClass('PHPUnit\Framework\TestCase');
        $collectors = [];
        if ($base !== null) {
            foreach ($this->program->uniqueClasses() as $cls) {
                if (!$cls->is_project || !$cls->isConcrete() || !$cls->isSubclassOf($base) || $cls === $base || $cls->crate !== $crate) {
                    continue;
                }
                foreach ($cls->methods as $m) {
                    if ($m->isStatic() || $m->node === null || !$this->isTestMethod($m)) {
                        continue;
                    }
                    try {
                        $collectors[] = $this->emitTest($cls, $m, $w);
                    } catch (\Throwable $e) {
                        $this->diag->warn('test harness error: ' . $e->getMessage() . ' @ ' . basename($e->getFile()) . ':' . $e->getLine(), $m->node, $cls->fqcn);
                    }
                }
            }
        }
        $w->open('fn main() {');
        $w->line('let args = libtest_mimic::Arguments::from_args();');
        $w->line('let mut groups: Vec<Vec<libtest_mimic::Trial>> = Vec::new();');
        foreach ($collectors as $c) {
            $w->line('{ let mut trials = Vec::new(); ' . $c . '(&mut trials); groups.push(trials); }');
        }
        $w->line('let trials: Vec<libtest_mimic::Trial> = groups.into_iter().flatten().collect();');
        $w->line('libtest_mimic::run(&args, trials).exit();');
        $w->close();
        return count($collectors);
    }

    private function isTestMethod(MethodModel $m): bool
    {
        if ($m->storage->visibility !== \Psalm\Internal\Analyzer\ClassLikeAnalyzer::VISIBILITY_PUBLIC) {
            return false;
        }
        if (str_starts_with(strtolower($m->name), 'test')) {
            return true;
        }
        foreach ($m->node->attrGroups as $group) {
            foreach ($group->attrs as $attr) {
                if (strtolower($attr->name->getLast()) === 'test') {
                    return true;
                }
            }
        }
        return false;
    }

    /** Why the test cannot run in the compiled suite (PHPUnit mocks, closure rebinding), or null. */
    /** Reflection classes the compiled program does not provide (only ReflectionClass name tables exist). */
    private const RUNTIME_REFLECTION = ['reflectionfunction', 'reflectionmethod', 'reflectionnamedtype', 'reflectionparameter', 'reflectionproperty', 'reflectionobject', 'reflectionuniontype', 'reflectionintersectiontype', 'reflectionenum', 'reflectionfunctionabstract'];

    /** @var array<string, array<string, true>> per class: lowercase names of methods that (transitively) use runtime reflection */
    private array $reflection_users = [];

    /** @return array<string, true> */
    private function reflectionUsers(ClassModel $cls): array
    {
        if (isset($this->reflection_users[$cls->fqcn])) {
            return $this->reflection_users[$cls->fqcn];
        }
        $finder = new \PhpParser\NodeFinder();
        $direct = [];
        $calls = [];
        foreach ($cls->methods as $method) {
            if ($method->node === null || $method->node->stmts === null) {
                continue;
            }
            $lc = strtolower($method->name);
            foreach ($finder->findInstanceOf($method->node->stmts, \PhpParser\Node\Expr\New_::class) as $new) {
                if ($new->class instanceof \PhpParser\Node\Name && in_array(strtolower($new->class->getLast()), self::RUNTIME_REFLECTION, true)) {
                    $direct[$lc] = true;
                }
            }
            foreach ($finder->findInstanceOf($method->node->stmts, \PhpParser\Node\Expr\StaticCall::class) as $call) {
                if ($call->class instanceof \PhpParser\Node\Name && in_array(strtolower($call->class->getLast()), self::RUNTIME_REFLECTION, true)) {
                    $direct[$lc] = true;
                }
            }
            foreach ($finder->findInstanceOf($method->node->stmts, \PhpParser\Node\Expr\MethodCall::class) as $call) {
                if ($call->var instanceof \PhpParser\Node\Expr\Variable && $call->var->name === 'this' && $call->name instanceof \PhpParser\Node\Identifier) {
                    $calls[$lc][] = strtolower($call->name->name);
                }
            }
        }
        // transitive closure over `$this->helper()` calls inside the class
        $users = $direct;
        do {
            $changed = false;
            foreach ($calls as $caller => $callees) {
                if (isset($users[$caller])) {
                    continue;
                }
                foreach ($callees as $callee) {
                    if (isset($users[$callee])) {
                        $users[$caller] = true;
                        $changed = true;
                        break;
                    }
                }
            }
        } while ($changed);
        return $this->reflection_users[$cls->fqcn] = $users;
    }

    private function unsupportedMechanism(MethodModel $m): ?string
    {
        if ($m->node->stmts === null) {
            return null;
        }
        if (isset($this->reflectionUsers($m->declaring)[strtolower($m->name)])) {
            return 'PHP reflection is not available in the compiled test suite';
        }
        $finder = new \PhpParser\NodeFinder();
        foreach ($finder->findInstanceOf($m->node->stmts, \PhpParser\Node\Expr\MethodCall::class) as $call) {
            if ($call->name instanceof \PhpParser\Node\Identifier) {
                $name = strtolower($call->name->name);
                if (in_array($name, ['createmock', 'getmockbuilder', 'createstub', 'createconfiguredmock', 'createpartialmock'], true)) {
                    return 'PHPUnit mocks are not available in the compiled test suite';
                }
                if ($name === 'bindto') {
                    return 'Closure rebinding is not available in the compiled test suite';
                }
            }
        }
        return null;
    }

    private function dataProvider(MethodModel $m): ?string
    {
        foreach ($m->node->attrGroups as $group) {
            foreach ($group->attrs as $attr) {
                if (strtolower($attr->name->getLast()) === 'dataprovider' && isset($attr->args[0])) {
                    $v = $attr->args[0]->value;
                    if ($v instanceof \PhpParser\Node\Scalar\String_) {
                        return $v->value;
                    }
                }
            }
        }
        $doc = $m->node->getDocComment();
        if ($doc !== null && preg_match('/@dataProvider\s+([A-Za-z_][A-Za-z0-9_]*)/', $doc->getText(), $mm)) {
            return $mm[1];
        }
        return null;
    }

    /** @return list<string> names of the test methods this test depends on (their return values become trailing args) */
    private function depends(MethodModel $m): array
    {
        $out = [];
        foreach ($m->node->attrGroups as $group) {
            foreach ($group->attrs as $attr) {
                if (strtolower($attr->name->getLast()) === 'depends' && isset($attr->args[0])) {
                    $v = $attr->args[0]->value;
                    if ($v instanceof \PhpParser\Node\Scalar\String_) {
                        $out[] = $v->value;
                    }
                }
            }
        }
        $doc = $m->node->getDocComment();
        if ($doc !== null && preg_match_all('/@depends\s+(?:[A-Za-z_\\\\]+::)?([A-Za-z_][A-Za-z0-9_]*)/', $doc->getText(), $mm)) {
            foreach ($mm[1] as $name) {
                $out[] = $name;
            }
        }
        return $out;
    }

    /** Emits the collector of a test method's trials and returns its name. */
    private function emitTest(ClassModel $cls, MethodModel $m, Writer $w): string
    {
        $path = $cls->path();
        $fn_name = 'collect__' . Names::ident(str_replace('\\', '_', $cls->fqcn) . '__' . $m->name);
        $provider_name = $this->dataProvider($m);
        $provider = $provider_name !== null ? $this->program->findMethod($cls, strtolower($provider_name)) : null;
        $root = Names::rustStringLiteral($this->program->transpiler->root_dir);
        $name = Names::rustStringLiteral($cls->fqcn . '::' . $m->name);
        $ctor = $this->program->findMethod($cls, '__construct');
        $new = $path . '::new(' . ($ctor !== null && count($ctor->storage->params) > 0 ? 'Str::from_static(' . Names::rustStringLiteral($m->name) . ')' : '') . ')?';
        $has_setup_class = $this->program->findMethod($cls, 'setupbeforeclass') !== null;
        $has_teardown_class = $this->program->findMethod($cls, 'teardownafterclass') !== null;

        $w->open('fn ' . $fn_name . '(trials: &mut Vec<libtest_mimic::Trial>) {');
        $unsupported = $this->unsupportedMechanism($m);
        if ($unsupported !== null) {
            // mocks and closure rebinding need runtime code generation: the test is reported as skipped
            $w->line('trials.push(libtest_mimic::Trial::test(' . $name . '.to_string(), move || { eprintln!("[skipped] {}: {}", ' . $name . ', ' . Names::rustStringLiteral($unsupported) . '); Ok(()) }));');
            $w->close();
            return $fn_name;
        }
        $rows = null;
        if ($provider !== null) {
            $rows_expr = $provider->isStatic()
                ? $path . '::' . $provider->rustName() . '()?'
                : '{ let __p = ' . $new . '; __p.' . $provider->rustName() . '()? }';
            $rows = $this->rowsIterator($rows_expr, $provider->return_type);
        }
        // the body of one trial: the whole test method, or the data set at position `__i`
        $body = new Writer();
        $body->line('crate::init();');
        if ($has_setup_class) {
            $body->line($path . '::' . Names::method('setUpBeforeClass') . '()?;');
        }
        $dep_args = [];
        $params = $m->storage->params;
        $deps = $this->depends($m);
        $first_dep_param = count($params) - count($deps);
        foreach ($deps as $i => $dep_name) {
            $dep = $this->program->findMethod($cls, strtolower($dep_name));
            $pi = $first_dep_param + $i;
            $pt = $m->param_types[$pi] ?? RustType::mixed();
            if ($pi < 0 || $pi >= count($params)) {
                continue; // the test does not take the depended-on value
            }
            if ($dep === null) {
                $dep_args[] = $this->casts->defaultOf($pt);
                continue;
            }
            $body->line('let __dep' . $i . ' = { let __d = ' . $new . '; __d.' . Names::method('runSetUp') . '()?; let __r = __d.' . $dep->rustName() . '()?; __d.' . Names::method('runTearDown') . '()?; __r };');
            $dep_args[] = $this->casts->convert('__dep' . $i . '.clone()', $dep->return_type, $pt);
        }
        if ($rows === null) {
            $body->line('let __t = ' . $new . ';');
            $this->emitInvocation($cls, $m, $body, $dep_args, 'Str::from_static("")');
        } else {
            [$iter, $kt, $vt] = $rows;
            $body->line('let __t = ' . $new . ';');
            $body->line('__t.' . Names::method('setDataName') . '(to_str(&__key))?;');
            $args = [...$this->rowArgs($m, $vt, $first_dep_param), ...$dep_args];
            $this->emitInvocation($cls, $m, $body, $args, 'to_str(&__key)');
        }
        if ($has_teardown_class) {
            $body->line($path . '::' . Names::method('tearDownAfterClass') . '()?;');
        }
        $body->line('Ok(())');

        if ($rows === null) {
            $w->open('trials.push(libtest_mimic::Trial::test(' . $name . '.to_string(), move || php_rt::testing::run_on_pool(' . $root . ', Box::new(move || php_rt::testing::run_row(' . $name . ', || -> Result<(), Throw> {');
            $w->raw($body->get());
            $w->close('}))).map_err(libtest_mimic::Failed::from)));');
        } else {
            [$iter] = $rows;
            // the data sets are named by running the provider once; every trial then runs on the warm worker
            // pool, where each worker evaluates the provider once and keeps its rows
            $w->line('let __keys: Result<Vec<String>, String> = php_rt::testing::in_thread(' . $root . ', || -> Result<Vec<String>, Throw> { crate::init(); Ok((' . $iter . ').into_iter().map(|(__k, _)| to_str(&__k).to_string()).collect()) });');
            $w->open('match __keys {');
            $w->line('Err(__msg) => trials.push(libtest_mimic::Trial::test(' . $name . '.to_string(), move || Err(libtest_mimic::Failed::from(format!("data provider failed: {}", __msg))))),');
            $w->open('Ok(__keys) => for (__i, __key) in __keys.into_iter().enumerate() {');
            $w->open('trials.push(libtest_mimic::Trial::test(format!("{} [{}]", ' . $name . ', __key), move || php_rt::testing::run_on_pool(' . $root . ', Box::new(move || php_rt::testing::run_row(' . $name . ', || -> Result<(), Throw> {');
            $w->line('crate::init();');
            $w->line('let (__key, __row) = php_rt::testing::cached_rows(' . $name . ', || -> Result<Vec<_>, Throw> { Ok((' . $iter . ').into_iter().collect()) }, |__rows| __rows[__i].clone())?;');
            $w->raw($body->get());
            $w->close('}))).map_err(libtest_mimic::Failed::from)));');
            $w->close('},');
            $w->close();
        }
        $w->close();
        return $fn_name;
    }

    /**
     * @return array{string, RustType, RustType} iterator expression over (key, row), key type, row type
     */
    private function rowsIterator(string $rows, RustType $rt): array
    {
        if ($rt->kind === RustType::OPTION) {
            $rows .= '.unwrap_or_default()';
            $rt = $rt->inner();
        }
        switch ($rt->kind) {
            case RustType::LIST:
                return [$rows . '.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v))', RustType::int(), $rt->inner()];
            case RustType::MAP:
                return [$rows . '.into_iter()', $rt->params[0], $rt->params[1]];
            case RustType::RT_GENERIC:
                return [$rows . '.into_pairs()', $rt->params[0] ?? RustType::mixed(), $rt->params[1] ?? RustType::mixed()];
            case RustType::SHAPE:
                $mt = RustType::map(RustType::arrayKey(), $this->shapeValueType($rt));
                return [$this->casts->convert($rows, $rt, $mt) . '.into_iter()', RustType::arrayKey(), $mt->params[1]];
            case RustType::TUPLE:
                $lt = RustType::list($this->program->types->combine($rt->params));
                return [$this->casts->convert($rows, $rt, $lt) . '.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v))', RustType::int(), $lt->inner()];
            default:
                $mt = RustType::map(RustType::arrayKey(), RustType::mixed());
                return [$this->casts->convert($rows, $rt, $mt) . '.into_iter()', RustType::arrayKey(), RustType::mixed()];
        }
    }

    private function shapeValueType(RustType $shape): RustType
    {
        $types = [];
        foreach ($shape->fields as [$t, $opt]) {
            $types[] = $t;
        }
        return $this->program->types->combine($types);
    }

    /**
     * Argument expressions for the test method from a data-provider row of type `$vt` (bound to `__row`).
     *
     * @return list<string>
     */
    private function rowArgs(MethodModel $m, RustType $vt, int $n_params): array
    {
        $params = array_slice($m->storage->params, 0, max(0, $n_params));
        $args = [];
        if ($vt->kind === RustType::SHAPE) {
            // PHPUnit passes `array_values($row)`: the i-th field goes to the i-th parameter. Rows keyed by
            // parameter names are matched by name (the struct keeps the docblock's field order, not the row's).
            $keys = array_keys($vt->fields);
            $by_name = $params !== [];
            foreach ($params as $p) {
                if (!isset($vt->fields[$p->name])) {
                    $by_name = false;
                }
            }
            foreach ($params as $i => $p) {
                $key = $by_name ? $p->name : ($keys[$i] ?? null);
                $pt = $m->param_types[$i] ?? RustType::mixed();
                if ($key !== null && isset($vt->fields[$key])) {
                    [$ft, $opt] = $vt->fields[$key];
                    $src = '__row.' . Names::field($key) . '.clone()';
                    if ($opt && $pt->kind === RustType::OPTION) {
                        $args[] = $this->casts->convert($src, RustType::shapeField($ft, true), $pt);
                    } else {
                        $args[] = $opt
                            ? $this->casts->convert($src, RustType::shapeField($ft, true), RustType::option($pt)) . '.unwrap_or_default()'
                            : $this->casts->convert($src, $ft, $pt);
                    }
                } else {
                    $args[] = $this->paramDefault($m, $i, $pt);
                }
            }
            return $args;
        }
        if ($vt->kind === RustType::TUPLE) {
            foreach ($params as $i => $p) {
                $pt = $m->param_types[$i] ?? RustType::mixed();
                $args[] = isset($vt->params[$i]) ? $this->casts->convert('__row.' . $i . '.clone()', $vt->params[$i], $pt) : $this->paramDefault($m, $i, $pt);
            }
            return $args;
        }
        $elem = match ($vt->kind) {
            RustType::LIST => $vt->inner(),
            RustType::MAP => $vt->params[1],
            default => RustType::mixed(),
        };
        $list = $vt->kind === RustType::LIST ? '__row' : $this->casts->convert('__row.clone()', $vt, RustType::list($elem));
        foreach ($params as $i => $p) {
            $pt = $m->param_types[$i] ?? RustType::mixed();
            $args[] = '(match ' . $list . '.get(' . $i . ').cloned() { Some(__a) => ' . $this->casts->convert('__a', $elem, $pt) . ', None => ' . $this->paramDefault($m, $i, $pt) . ' })';
        }
        return $args;
    }

    /** The declared default of parameter `$i` (`string $php_version = '7.4'`), or the type's default value. */
    private function paramDefault(MethodModel $m, int $i, RustType $pt): string
    {
        $param = $m->node->params[$i] ?? null;
        if ($param !== null && $param->default !== null) {
            $record = new FunctionRecord(new \PhpParser\Node\Stmt\ClassMethod('__dummy'), new \Psalm\Storage\MethodStorage(), new \Psalm\Internal\Provider\NodeDataProvider(), $m->record?->file_path ?? '', $m->declaring->fqcn, null);
            $b = new BodyEmitter($this->program, $record, $m->declaring, $this->casts, $this->builtins, $this->diag, null);
            $b->this_type = null;
            return $b->constExpr($param->default, $pt);
        }
        return $this->casts->defaultOf($pt);
    }

    /** @param list<string> $args */
    private function emitInvocation(ClassModel $cls, MethodModel $m, Writer $w, array $args, string $dataset): void
    {
        $w->open('php_rt::testing::case(' . $dataset . ', || -> Result<(), Throw> {');
        $mm = fn(string $n) => Names::method($n);
        $w->line('__t.' . $mm('runSetUp') . '()?;');
        $w->line('let __outcome = __t.' . $m->rustName() . '(' . implode(', ', $args) . ');');
        $w->line('let __td = __t.' . $mm('runTearDown') . '();');
        $w->open('match __outcome {');
        $w->line('Ok(_) => { if __t.' . $mm('expectsException') . '()? { return Err(Throw::assertion(cat!(Str::from_static("Failed asserting that exception of type \\""), __t.' . $mm('expectedExceptionDescription') . '()?, Str::from_static("\\" is thrown")))); } }');
        $w->line('Err(__e) => { if __t.' . $mm('expectsException') . '()? && !php_rt::testing::is_skip(&__e) { __t.' . $mm('verifyExpectedException') . '(__e)?; } else { return Err(__e); } }');
        $w->close();
        $w->line('__td?;');
        $w->line('Ok(())');
        $w->close('})?;');
    }
}
