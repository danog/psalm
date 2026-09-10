<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use Psalm\Codebase;

use function array_keys;
use function array_map;
use function count;
use function file_put_contents;
use function fwrite;
use function implode;
use function is_dir;
use function ksort;
use function mkdir;
use function strtolower;

use const STDERR;

/**
 * Writes the generated Rust crate.
 *
 * @internal
 */
final class CrateEmitter
{
    private Program $program;
    private Casts $casts;
    private Builtins $builtins;
    private Diagnostics $diag;

    /** @var array<string, Writer> module path (a::b::c) => writer */
    private array $modules = [];

    public function __construct(
        private readonly Transpiler $transpiler,
        private readonly Codebase $codebase,
    ) {
    }

    public function emit(): void
    {
        fwrite(STDERR, "[transpiler] building program model\n");
        $this->program = new Program($this->codebase, $this->transpiler);
        fwrite(STDERR, "[transpiler] emitting classes\n");
        $this->casts = new Casts($this->program);
        $this->builtins = new Builtins();
        $this->diag = new Diagnostics();

        $class_emitter = new ClassEmitter($this->program, $this->casts, $this->builtins, $this->diag);
        $cast_emitter = new CastEmitter($this->program, $this->casts);

        // classes
        $project_classes = [];
        foreach ($this->program->uniqueClasses() as $cls) {
            if ($cls->is_project && !$cls->isTrait()) {
                $project_classes[] = $cls;
            }
        }
        foreach ($project_classes as $cls) {
            $w = $this->module(Names::modulePath($cls->fqcn));
            try {
                $class_emitter->emit($cls, $w);
            } catch (\Throwable $e) {
                $this->diag->warn('class emission error: ' . $e->getMessage() . ' @ ' . basename($e->getFile()) . ':' . $e->getLine(), $cls->node, $cls->fqcn);
            }
        }
        // factories are discovered during body emission; emit them now
        foreach ($this->program->factories as [$fc, $arity]) {
            $w = $this->module(Names::modulePath($fc->fqcn));
            $fake = new \ReflectionMethod($class_emitter, 'emitFactory');
            $fake->invoke($class_emitter, $fc, $w);
        }
        foreach ($project_classes as $cls) {
            $w = $this->module(Names::modulePath($cls->fqcn));
            $cast_emitter->emitClassImpls($cls, $w);
        }

        fwrite(STDERR, "[transpiler] emitting functions\n");
        // free functions
        foreach ($this->program->functions as $fn) {
            $w = $this->module(Names::modulePath($fn->fq_name));
            $this->emitFunction($fn, $w);
        }

        // AnyObject enum over all concrete classes
        $any = new Writer();
        $this->emitAnyObject($any);

        fwrite(STDERR, "[transpiler] emitting tests\n");
        $tests_w = new Writer();
        $test_emitter = new TestEmitter($this->program, $this->casts, $this->builtins, $this->diag);
        $n_tests = $test_emitter->emit($tests_w);
        // generated unions and shapes (emission of casts may add more, so loop)
        $types_w = new Writer();
        $emitted = [];
        $casts_w = new Writer();
        for ($round = 0; $round < 10; $round++) {
            $new = false;
            foreach ($this->program->types->unions as $name => $u) {
                if (!isset($emitted[$name])) {
                    $emitted[$name] = true;
                    $cast_emitter->emitUnion($u, $types_w);
                    $new = true;
                }
            }
            foreach ($this->program->types->shapes as $name => $s) {
                if (!isset($emitted[$name])) {
                    $emitted[$name] = true;
                    $cast_emitter->emitShape($s, $types_w);
                    $new = true;
                }
            }
            $before = count($this->casts->casts) + count($this->casts->instance_checks);
            $cast_emitter->emitRecordedCasts($casts_w);
            if (!$new && $before === count($this->casts->casts) + count($this->casts->instance_checks)) {
                break;
            }
        }

        fwrite(STDERR, "[transpiler] $n_tests tests, writing crate\n");
        $this->writeCrate($types_w, $casts_w, $any, $tests_w);
        $this->diag->report();
        foreach ($this->casts->warnings as $wmsg) {
            fwrite(STDERR, "  [casts] unsupported conversion: $wmsg\n");
        }
        foreach ($this->program->types->unsupported as $atomic => $n) {
            fwrite(STDERR, "  [types] unsupported atomic $atomic: $n\n");
        }
    }

    private function module(string $path): Writer
    {
        return $this->modules[$path] ??= new Writer();
    }

    private function emitFunction(FunctionModel $fn, Writer $w): void
    {
        $record = $fn->record;
        $b = new BodyEmitter($this->program, $record, null, $this->casts, $this->builtins, $this->diag, null);
        $b->this_type = null;
        $params = [];
        $decls = [];
        foreach ($record->storage->params as $i => $p) {
            $t = $fn->param_types[$i] ?? RustType::mixed();
            $params[$p->name] = $t;
            if ($p->by_ref) {
                $b->byref[$p->name] = true;
            }
            $decls[] = 'mut ' . Names::var($p->name) . ': ' . ($p->by_ref ? '&mut ' : '') . $t->toRust();
        }
        try {
            $body = $b->emitBody($params, $record->node->stmts, $fn->return_type);
        } catch (\Throwable $e) {
            $this->diag->warn('transpiler error: ' . $e->getMessage() . ' @ ' . basename($e->getFile()) . ':' . $e->getLine(), $record->node, $record->file_path);
            $body = "    unreachable!(\"transpiler error\")\n";
        }
        $w->line('pub fn ' . $fn->rustName() . '(' . implode(', ', $decls) . ') -> Result<' . $fn->return_type->toRust() . ', Throw> {');
        $w->raw($body);
        $w->line('}');
    }

    private function emitAnyObject(Writer $w): void
    {
        $concrete = [];
        foreach ($this->program->uniqueClasses() as $cls) {
            if ($cls->is_project && $cls->isConcrete() && !$cls->isTrait() && !$cls->isEnum()) {
                $concrete[] = $cls;
            }
        }
        $w->line('#[derive(Clone)]');
        $w->open('pub enum AnyObject {');
        foreach ($concrete as $c) {
            $w->line($c->variant() . '(' . $c->ownPath() . '),');
        }
        $w->line('Other(AnyObj),');
        $w->close();
        $arms = fn(string $call) => implode(', ', array_map(fn(ClassModel $c) => 'AnyObject::' . $c->variant() . '(h) => h.' . $call, $concrete)) . ($concrete ? ', ' : '') . 'AnyObject::Other(o) => o.' . $call;
        $w->open('impl php_rt::PhpObject for AnyObject {');
        $w->line('fn class_name(&self) -> &\'static str { match self { ' . $arms('class_name()') . ' } }');
        $w->line('fn class_ancestors(&self) -> &\'static [&\'static str] { match self { ' . $arms('class_ancestors()') . ' } }');
        $w->line('fn obj_id(&self) -> usize { match self { ' . $arms('obj_id()') . ' } }');
        $w->line('fn as_any(&self) -> &dyn std::any::Any { self }');
        $w->line('fn props(&self) -> Vec<(Str, Mixed)> { match self { ' . $arms('props()') . ' } }');
        $w->line('fn set_prop(&self, name: &str, value: Mixed) -> bool { match self { ' . $arms('set_prop(name, value)') . ' } }');
        $w->line('fn get_prop(&self, name: &str) -> Option<Mixed> { match self { ' . $arms('get_prop(name)') . ' } }');
        $w->line('fn php_to_string(&self) -> Option<Str> { match self { ' . $arms('php_to_string()') . ' } }');
        $w->line('fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match self { ' . $arms('call_method(name, args)') . ' } }');
        $w->line('fn public_props(&self) -> Vec<(Str, Mixed)> { match self { ' . $arms('public_props()') . ' } }');
        $w->close();
        $w->open('impl AnyObject {');
        $downs = [];
        foreach ($concrete as $c) {
            $downs[] = 'if let Some(v) = o.as_any().downcast_ref::<' . $c->ownPath() . '>() { return AnyObject::' . $c->variant() . '(v.clone()); }';
        }
        $w->line('pub fn from_mixed(m: Mixed) -> AnyObject { if let Mixed::Obj(o) = &m { ' . implode(' ', $downs) . ' return AnyObject::Other(o.clone()); } panic!("not an object: {:?}", m) }');
        $w->line('pub fn to_php_string(&self) -> Result<Str, Throw> { self.php_to_string().ok_or_else(|| Throw::error(Str::from_static("Object could not be converted to string"))) }');
        $w->close();
        $w->line('impl php_rt::CastTo<Mixed> for AnyObject { fn cast_to(self) -> Mixed { match self { ' . implode(', ', array_map(fn(ClassModel $c) => 'AnyObject::' . $c->variant() . '(h) => Mixed::Obj(Rc::new(h))', $concrete)) . ($concrete ? ', ' : '') . 'AnyObject::Other(o) => Mixed::Obj(o) } } }');
        $w->line('impl php_rt::CastTo<AnyObject> for Mixed { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(self) } }');
        $w->line('impl php_rt::TryDowncast for AnyObject { fn try_downcast(o: &AnyObj) -> Option<Self> { Some(AnyObject::from_mixed(Mixed::Obj(o.clone()))) } }');
        $w->line('impl php_rt::Truthy for AnyObject { fn truthy(&self) -> bool { true } }');
        $w->line('impl php_rt::Identical for AnyObject { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }');
        $w->line('impl php_rt::PhpCmp for AnyObject { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }');
        $w->line('impl php_rt::ToStr for AnyObject { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_str(self.class_name())) } }');
        $w->line('impl std::fmt::Debug for AnyObject { fn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result { write!(f, "object({})", self.class_name()) } }');
        $w->line('pub fn php_clone_mixed(m: Mixed) -> Mixed { match m { Mixed::Obj(_) => cast::<Mixed>(AnyObject::from_mixed(m).php_clone()), other => other } }');
        $w->line('impl php_rt::PhpClone for AnyObject { fn php_clone(&self) -> Self { match self { ' . implode(', ', array_map(fn(ClassModel $c) => 'AnyObject::' . $c->variant() . '(h) => AnyObject::' . $c->variant() . '(h.php_clone())', $concrete)) . ($concrete ? ', ' : '') . 'AnyObject::Other(o) => AnyObject::Other(o.clone()) } } }');

        $this->emitInit($w);

        // Throwable alias: the generated Throw type
        $throwable = $this->program->getClass('Throwable');
        if ($throwable !== null && $throwable->is_project) {
            $w->line('pub type Throw = ' . $throwable->path() . ';');
            $this->emitThrowSupport($throwable, $w);
        } else {
            $w->line('pub type Throw = php_rt::FallbackThrow;');
        }
    }

    /** `init()`: registers classes and constants with the runtime registry (for class_exists, is_a, constant(), ...). */
    private function emitInit(Writer $w): void
    {
        $w->line('thread_local! { static __INIT: std::cell::Cell<bool> = std::cell::Cell::new(false); }');
        $w->open('pub fn init() {');
        $w->line('if __INIT.with(|c| c.replace(true)) { return; }');
        foreach ($this->program->uniqueClasses() as $cls) {
            if (!$cls->is_project || $cls->isTrait()) {
                continue;
            }
            $names = [strtolower($cls->fqcn)];
            foreach ($cls->ancestors as $a) {
                $names[] = strtolower($a->fqcn);
            }
            $w->line('php_rt::registry::register_class(' . Names::rustStringLiteral($cls->fqcn) . ', &[' . implode(', ', array_map(fn($n) => Names::rustStringLiteral($n), $names)) . '], ' . ($cls->isInterface() ? 'true' : 'false') . ');');
            if ($cls->isConcrete() && !$cls->isEnum()) {
                $w->line('php_rt::registry::register_factory(' . Names::rustStringLiteral($cls->fqcn) . ', Box::new(|| ' . $this->casts->convert($cls->ownPath() . '::new_uninit()', RustType::class($cls->fqcn), RustType::mixed()) . '));');
            }
            if (!$cls->isInterface() && !$cls->isEnum()) {
                $w->line('php_rt::registry::register_static(' . Names::rustStringLiteral($cls->fqcn) . ', Box::new(|__m, __a| ' . $cls->path() . '::call_static(__m, __a)));');
            }
        }
        foreach ($this->program->uniqueClasses() as $cls) {
            if (!$cls->is_project || $cls->isTrait()) {
                continue;
            }
            $seen = [];
            foreach ([$cls, ...$cls->ancestors] as $src) {
                foreach ($src->constants as $c) {
                    if ($c->expr === null || isset($seen[$c->name])) {
                        continue;
                    }
                    $seen[$c->name] = true;
                    $w->line('php_rt::registry::define_constant(&Str::from_static(' . Names::rustStringLiteral($cls->fqcn . '::' . $c->name) . '), ' . $this->casts->convert($src->path() . '::' . $c->rustName() . '()', $c->type, RustType::mixed()) . ');');
                }
            }
        }
        foreach ($this->program->constants as $c) {
            $w->line('php_rt::registry::define_constant(&Str::from_static(' . Names::rustStringLiteral($c->name) . '), ' . $this->casts->convert('crate::consts::' . Names::constant($c->name) . '()', $c->type, RustType::mixed()) . ');');
        }
        $w->close();
    }

    /** Constructors for runtime-raised errors, and conversion from php_rt::RtError. */
    private function emitThrowSupport(ClassModel $throwable, Writer $w): void
    {
        $tt = RustType::class($throwable->fqcn);
        $mk = function (string $cls, string $msg_code) use ($tt): string {
            $c = $this->program->getClass($cls);
            if ($c === null || !$c->is_project) {
                return 'panic!("missing runtime stub class ' . $cls . '")';
            }
            return $this->casts->convert($c->path() . '::new(' . $msg_code . ', 0i64, None).unwrap()', RustType::class($c->fqcn), $tt);
        };
        $w->open('impl ' . $throwable->path() . ' {');
        $w->line('pub fn error(msg: Str) -> Self { ' . $mk('Error', 'msg') . ' }');
        $w->line('pub fn type_error(msg: Str) -> Self { ' . $mk('TypeError', 'msg') . ' }');
        $w->line('pub fn value_error(msg: Str) -> Self { ' . $mk('ValueError', 'msg') . ' }');
        $w->line('pub fn assertion(msg: Str) -> Self { ' . $mk('AssertionError', 'msg') . ' }');
        $w->line('pub fn unhandled_match(v: &Mixed) -> Self { ' . $mk('UnhandledMatchError', 'cat!(Str::from_static("Unhandled match case "), json_encode_simple(v))') . ' }');
        $exit = $this->program->getClass('PhpExitException');
        if ($exit !== null && $exit->is_project) {
            $w->line('pub fn exit(status: i64) -> Self { ' . $this->casts->convert($exit->path() . '::new(status).unwrap()', RustType::class($exit->fqcn), $tt) . ' }');
            $w->line('pub fn exit_status(&self) -> Option<i64> { if is_instance::<' . $exit->path() . '>(self) { Some(cast::<' . $exit->path() . '>(self.clone()).p_status_get()) } else { None } }');
            $this->casts->needInstanceOf($tt, RustType::class($exit->fqcn));
            $this->casts->need($tt, RustType::class($exit->fqcn));
        } else {
            $w->line('pub fn exit(status: i64) -> Self { Self::error(cat!(Str::from_static("exit "), to_str(&status))) }');
            $w->line('pub fn exit_status(&self) -> Option<i64> { None }');
        }
        $w->line('pub fn message(&self) -> Str { self.getMessage().unwrap_or_default() }');
        $w->close();
        $classes = ['Error', 'TypeError', 'ValueError', 'ArgumentCountError', 'ArithmeticError', 'DivisionByZeroError', 'AssertionError', 'UnhandledMatchError', 'JsonException', 'RuntimeException', 'LogicException', 'InvalidArgumentException', 'UnexpectedValueException', 'OutOfBoundsException'];
        $arms = [];
        foreach ($classes as $cls) {
            $c = $this->program->getClass($cls);
            if ($c !== null && $c->is_project) {
                $arms[] = Names::rustStringLiteral($cls) . ' => ' . $this->casts->convert($c->path() . '::new(e.message, 0i64, None).unwrap()', RustType::class($c->fqcn), $tt);
            }
        }
        $w->line('impl From<RtError> for ' . $throwable->path() . ' { fn from(e: RtError) -> Self { match e.class { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => Self::error(e.message) } } }');
        $w->line('impl From<DynError> for ' . $throwable->path() . ' { fn from(e: DynError) -> Self { match e { DynError::Rt(r) => Self::from(r), DynError::Obj(m) => cast::<Self>(m) } } }');
        $w->line('impl std::fmt::Display for ' . $throwable->path() . ' { fn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result { write!(f, "{}: {}", self.class_name(), self.message()) } }');
    }

    private function writeCrate(Writer $types, Writer $casts, Writer $any, Writer $tests): void
    {
        $out = $this->transpiler->out_dir;
        $src = $out . '/src';
        if (!is_dir($src)) {
            mkdir($src, 0777, true);
        }
        $prelude = "#![allow(unused_imports, unused_variables, unused_mut, dead_code, non_snake_case, non_camel_case_types, unreachable_code, unused_parens, unused_braces, unused_assignments, unused_labels, unused_unsafe, clippy::all, irrefutable_let_patterns, unreachable_patterns, unused_must_use, non_upper_case_globals, deprecated, ambiguous_glob_reexports, hidden_glob_reexports)]\n";
        $use = "use php_rt::prelude::*;\nuse crate::generated::*;\nuse crate::Throw;\nuse crate::AnyObject;\n";

        // module tree
        $tree = [];
        foreach ($this->modules as $path => $w) {
            $segs = explode('::', $path);
            $node = &$tree;
            foreach ($segs as $s) {
                $node[$s] ??= [];
                $node = &$node[$s];
            }
            unset($node);
        }
        $lib = $prelude . "pub mod generated;\npub mod consts;\npub use generated::*;\n" . $this->writeTree($tree, $src, $use, '') . "\n";
        $lib .= "use php_rt::prelude::*;\nuse crate::generated::*;\n" . $any->get();
        file_put_contents($src . '/lib.rs', $lib);
        file_put_contents($src . '/generated.rs', $prelude . "use php_rt::prelude::*;\nuse crate::*;\n" . $types->get() . $casts->get());
        file_put_contents($src . '/consts.rs', $prelude . $use . $this->constsModule());
        // the PHPUnit harness is an integration test crate: it links against the library instead of being
        // compiled into it (a single crate with all tests exhausts memory on large projects)
        $name = basename($out);
        $crate = str_replace('-', '_', $name);
        if (!is_dir($out . '/tests')) {
            mkdir($out . '/tests', 0777, true);
        }
        $harness = $prelude . str_replace('crate::', $crate . '::', $use) . str_replace('crate::', $crate . '::', $tests->get());
        file_put_contents($out . '/tests/harness.rs', $harness);
        if (file_exists($src . '/tests.rs')) {
            unlink($src . '/tests.rs');
        }
        file_put_contents($out . '/Cargo.toml', "[package]\nname = \"" . str_replace('-', '_', $name) . "\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\npath = \"src/lib.rs\"\n\n[dependencies]\nphp-rt = { path = \"../../php-rt\" }\n");
        fwrite(STDERR, 'wrote ' . count($this->modules) . " modules to $out\n");
    }

    /** Write nested module directories; returns the `pub mod` declarations for this level. */
    private function writeTree(array $tree, string $dir, string $use, string $prefix): string
    {
        ksort($tree);
        $decls = '';
        foreach ($tree as $seg => $children) {
            $path = $prefix === '' ? $seg : $prefix . '::' . $seg;
            $moddir = $dir . '/' . $seg;
            if (!is_dir($moddir)) {
                mkdir($moddir, 0777, true);
            }
            $content = "use php_rt::prelude::*;\nuse crate::generated::*;\nuse crate::Throw;\nuse crate::AnyObject;\n";
            $content .= $this->writeTree($children, $moddir, $use, $path);
            if (isset($this->modules[$path])) {
                $content .= $this->modules[$path]->get();
            }
            file_put_contents($moddir . '/mod.rs', $content);
            $decls .= 'pub mod ' . $seg . ";\n";
        }
        return $decls;
    }

    private function constsModule(): string
    {
        $out = '';
        foreach ($this->program->constants as $c) {
            $b = new BodyEmitter($this->program, $c->record, null, $this->casts, $this->builtins, $this->diag, null);
            $b->this_type = null;
            $code = $b->constExpr($c->expr, $c->type);
            $out .= 'pub fn ' . Names::constant($c->name) . '() -> ' . $c->type->toRust() . ' { ' . $code . " }\n";
        }
        return $out;
    }
}
