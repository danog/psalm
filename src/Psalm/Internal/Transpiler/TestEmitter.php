<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use function array_slice;
use function basename;
use function count;
use function implode;
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

    public function emit(Writer $w): int
    {
        $base = $this->program->getClass('PHPUnit\Framework\TestCase');
        if ($base === null) {
            return 0;
        }
        $n = 0;
        foreach ($this->program->uniqueClasses() as $cls) {
            if (!$cls->is_project || !$cls->isConcrete() || !$cls->isSubclassOf($base) || $cls === $base) {
                continue;
            }
            foreach ($cls->methods as $m) {
                if ($m->isStatic() || $m->node === null || !$this->isTestMethod($m)) {
                    continue;
                }
                try {
                    $this->emitTest($cls, $m, $w);
                    $n++;
                } catch (\Throwable $e) {
                    $this->diag->warn('test harness error: ' . $e->getMessage() . ' @ ' . basename($e->getFile()) . ':' . $e->getLine(), $m->node, $cls->fqcn);
                }
            }
        }
        return $n;
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

    private function emitTest(ClassModel $cls, MethodModel $m, Writer $w): void
    {
        $path = $cls->path();
        $fn_name = Names::ident(str_replace('\\', '_', $cls->fqcn) . '__' . $m->name);
        $provider_name = $this->dataProvider($m);
        $provider = $provider_name !== null ? $this->program->findMethod($cls, strtolower($provider_name)) : null;
        $root = $this->program->transpiler->root_dir;
        $ctor = $this->program->findMethod($cls, '__construct');
        $new = $path . '::new(' . ($ctor !== null && count($ctor->storage->params) > 0 ? 'Str::from_static(' . Names::rustStringLiteral($m->name) . ')' : '') . ')?';
        $has_setup_class = $this->program->findMethod($cls, 'setupbeforeclass') !== null;
        $has_teardown_class = $this->program->findMethod($cls, 'teardownafterclass') !== null;

        $w->line('#[test]');
        $w->open('fn ' . $fn_name . '() {');
        $w->open('php_rt::testing::run(' . Names::rustStringLiteral($cls->fqcn . '::' . $m->name) . ', ' . Names::rustStringLiteral($root) . ', || -> Result<(), Throw> {');
        $w->line('crate::init();');
        if ($has_setup_class) {
            $w->line($path . '::' . Names::method('setUpBeforeClass') . '()?;');
        }
        // values produced by depended-on tests
        $dep_args = [];
        $params = $m->storage->params;
        $deps = $this->depends($m);
        $first_dep_param = count($params) - count($deps);
        foreach ($deps as $i => $dep_name) {
            $dep = $this->program->findMethod($cls, strtolower($dep_name));
            $pi = $first_dep_param + $i;
            $pt = $m->param_types[$pi] ?? RustType::mixed();
            if ($dep === null || $pi < 0) {
                $dep_args[] = $this->casts->defaultOf($pt);
                continue;
            }
            $w->line('let __dep' . $i . ' = { let __d = ' . $new . '; __d.' . Names::method('runSetUp') . '()?; let __r = __d.' . $dep->rustName() . '()?; __d.' . Names::method('runTearDown') . '()?; __r };');
            $dep_args[] = $this->casts->convert('__dep' . $i . '.clone()', $dep->return_type, $pt);
        }
        if ($provider === null) {
            $w->line('let __t = ' . $new . ';');
            $this->emitInvocation($cls, $m, $w, $dep_args, 'Str::from_static("")');
        } else {
            $rows = $provider->isStatic()
                ? $path . '::' . $provider->rustName() . '()?'
                : '{ let __p = ' . $new . '; __p.' . $provider->rustName() . '()? }';
            $rt = $provider->return_type;
            [$iter, $kt, $vt] = $this->rowsIterator($rows, $rt);
            $w->open('for (__key, __row) in ' . $iter . ' {');
            $w->line('let __t = ' . $new . ';');
            $args = [...$this->rowArgs($m, $vt, $first_dep_param), ...$dep_args];
            $this->emitInvocation($cls, $m, $w, $args, 'to_str(&__key)');
            $w->close();
        }
        if ($has_teardown_class) {
            $w->line($path . '::' . Names::method('tearDownAfterClass') . '()?;');
        }
        $w->line('Ok(())');
        $w->close('});');
        $w->close();
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
            $by_name = true;
            foreach ($vt->fields as $k => $_) {
                if ((string) (int) $k === (string) $k) {
                    $by_name = false;
                }
            }
            foreach ($params as $i => $p) {
                $key = $by_name ? $p->name : (string) $i;
                $pt = $m->param_types[$i] ?? RustType::mixed();
                if (isset($vt->fields[$key])) {
                    [$ft, $opt] = $vt->fields[$key];
                    $src = '__row.' . Names::field($key) . '.clone()';
                    $args[] = $opt
                        ? $this->casts->convert($src, RustType::option($ft), RustType::option($pt)) . '.unwrap_or_default()'
                        : $this->casts->convert($src, $ft, $pt);
                } else {
                    $args[] = $this->casts->defaultOf($pt);
                }
            }
            return $args;
        }
        if ($vt->kind === RustType::TUPLE) {
            foreach ($params as $i => $p) {
                $pt = $m->param_types[$i] ?? RustType::mixed();
                $args[] = isset($vt->params[$i]) ? $this->casts->convert('__row.' . $i . '.clone()', $vt->params[$i], $pt) : $this->casts->defaultOf($pt);
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
            $args[] = '(match ' . $list . '.get(' . $i . ').cloned() { Some(__a) => ' . $this->casts->convert('__a', $elem, $pt) . ', None => ' . $this->casts->defaultOf($pt) . ' })';
        }
        return $args;
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
