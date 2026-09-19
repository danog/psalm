<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use Psalm\Codebase;

use function array_fill;
use function array_keys;
use function array_sum;
use function arsort;
use function array_shift;
use function array_slice;
use function basename;
use function dirname;
use function explode;
use function file_exists;
use function max;
use function preg_replace_callback;
use function realpath;
use function str_replace;
use function strrpos;
use function substr;
use function trim;
use function unlink;
use function array_map;
use function count;
use function file_get_contents;
use function file_put_contents;
use function fwrite;
use function implode;
use function in_array;
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

    /** @var array<int, array<string, Writer>> crate => module path (a::b::c) => writer */
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
        $n_crates = $this->transpiler->crateCount();

        $class_emitter = new ClassEmitter($this->program, $this->casts, $this->builtins, $this->diag);
        $cast_emitter = new CastEmitter($this->program, $this->casts);
        $data_emitter = new DataEmitter($this->codebase);

        // includable files: the data files listed on the command line (bodies may bind more, see below)
        $this->program->collectFiles();

        // classes
        $project_classes = [];
        foreach ($this->program->uniqueClasses() as $cls) {
            if ($cls->is_project && !$cls->isTrait()) {
                $project_classes[] = $cls;
            }
        }
        foreach ($project_classes as $cls) {
            $w = $this->classModule($cls);
            try {
                $class_emitter->emit($cls, $w);
            } catch (\Throwable $e) {
                $this->diag->warn('class emission error: ' . $e->getMessage() . ' @ ' . basename($e->getFile()) . ':' . $e->getLine(), $cls->node, $cls->fqcn);
            }
        }
        foreach ($project_classes as $cls) {
            $w = $this->classModule($cls);
            $cast_emitter->emitClassImpls($cls, $w);
        }

        fwrite(STDERR, "[transpiler] emitting functions\n");
        // free functions
        foreach ($this->program->functions as $fn) {
            $w = $this->module($this->program->crateOfRecord($fn->record), Names::modulePath($fn->fq_name));
            $this->emitFunction($fn, $w);
        }

        fwrite(STDERR, "[transpiler] emitting tests\n");
        $tests_w = [];
        $n_tests = 0;
        $test_emitter = new TestEmitter($this->program, $this->casts, $this->builtins, $this->diag);
        for ($i = 0; $i < $n_crates; $i++) {
            $tests_w[$i] = new Writer();
            $n_tests += $test_emitter->emit($tests_w[$i], $i);
        }

        // data files bound by `include` expressions with compile-time paths, in the types the sites declare
        $this->emitDataFiles($data_emitter);

        // copies of upstream method bodies requested by `parent::m()` calls (their bodies may request more)
        $done_copies = [];
        do {
            $new = false;
            foreach ($this->program->super_copies as $k => [$root, $m]) {
                if (isset($done_copies[$k])) {
                    continue;
                }
                $done_copies[$k] = true;
                $new = true;
                $name = substr($k, strrpos($k, '::') + 2);
                $class_emitter->emitSuperCopy($root, $m, $name, $this->classModule($root));
            }
            // dispatch accessors for variant-specific fields written/read through a base/interface enum
            // (MutableTypeVisitor pattern) — requested during body emission, may accrue as more bodies emit.
            foreach ($this->program->enum_field_accessors as $k => [$enum, $field, $ftype]) {
                if (isset($done_copies['efa:' . $k])) {
                    continue;
                }
                $done_copies['efa:' . $k] = true;
                $new = true;
                $class_emitter->emitEnumFieldAccessor($enum, $field, $ftype, $this->classModule($enum));
            }
        } while ($new);

        // dispatch methods of closed hierarchies: only those some code calls through the handle
        // (the Throw glue and the harness read exceptions through the Throwable handle)
        $thr = $this->program->getClass('Throwable');
        if ($thr !== null) {
            foreach (['getmessage', 'getcode', 'getprevious', 'getfile', 'getline', 'gettraceasstring', '__tostring'] as $lc) {
                $this->program->noteDispatch($thr, $lc);
            }
        }
        $n_disp = 0;
        foreach ($this->program->pending_dispatch as $k => [$dcls, $dm]) {
            if (isset($this->program->dispatch_demands[$k])) {
                $class_emitter->emitDeferredDispatch($dcls, $dm, $this->classModule($dcls));
                $n_disp++;
            }
        }
        fwrite(STDERR, '[lazy-dispatch] ' . $n_disp . ' of ' . count($this->program->pending_dispatch) . " hierarchy dispatch methods emitted\n");

        // the runtime object protocol, dynamic part only for classes some body erases to Mixed
        $this->program->types->context = '<post-pass: unions, shapes, casts>';
        $dyn = $this->casts->dynReachable();
        $n_dyn = 0;
        foreach ($project_classes as $cls) {
            $dynamic = $dyn['all'] !== null || isset($dyn['classes'][$cls->fqcn]);
            $n_dyn += $dynamic ? 1 : 0;
            $class_emitter->emitObjectProtocol($cls, $this->classModule($cls), $dynamic);
        }
        fwrite(STDERR, '[dyn] object protocol emitted for ' . $n_dyn . ' of ' . count($project_classes) . ' classes'
            . ($dyn['all'] !== null ? ' (everything: ' . $dyn['all'] . ' erased to Mixed)' : '') . "\n");
        // `object` conversions per class, only when some emitted body names AnyObject
        $any_used = false;
        foreach ($this->modules as $mods) {
            foreach ($mods as $mw) {
                $any_used = $any_used || str_contains($mw->get(), 'AnyObject');
            }
        }
        foreach ($tests_w as $tw) {
            $any_used = $any_used || str_contains($tw->get(), 'AnyObject');
        }
        if ($any_used) {
            foreach ($project_classes as $cls) {
                $cast_emitter->emitAnyObjectImpls($cls, $this->classModule($cls));
            }
        }
        $keepers = $dyn['keepers'];
        usort($keepers, fn($a, $b) => strcmp(substr($a, strpos($a, ' @ ')), substr($b, strpos($b, ' @ '))));
        fwrite(STDERR, '[dyn] ' . count($keepers) . " erasures reach objects\n");
        foreach (array_slice($keepers, 0, 400) as $keeper) {
            fwrite(STDERR, "  [dyn-keeper] $keeper\n");
        }

        // generated unions and shapes (emission of casts may add more, so loop); every generated type and
        // impl lives in the crate of the highest-crate class it mentions
        $types_w = [];
        $casts_w = [];
        for ($i = 0; $i < $n_crates; $i++) {
            $types_w[$i] = new Writer();
            $casts_w[$i] = new Writer();
        }
        // only the generated types some emitted code names are declared: a type mapped for a discarded
        // pass, a dead branch or an unused member would otherwise keep its (Mixed-typed) shape alive
        $referenced = [];
        $scan = static function (string $text) use (&$referenced): void {
            // runtime helpers producing a Mixed value (`.to_mixed()`, `mixed_get`) count as naming Mixed
            // (helper CALLS only: a module path such as `mixed_issue::MixedIssue` names a class, not Mixed)
            preg_match_all('/\b(?:U_|Shape_)[A-Za-z0-9_]+|\bMixed\b|\bto_mixed\(|\bmixed_[a-z_]+\(|AnyObject|\bphp_clone_mixed\b/', $text, $m);
            foreach ($m[0] as $id) {
                $referenced[str_starts_with($id, 'to_mixed') || str_starts_with($id, 'mixed_') ? 'Mixed' : $id] = true;
            }
        };
        foreach ($this->modules as $mods) {
            foreach ($mods as $w) {
                $scan($w->get());
            }
        }
        foreach ($tests_w as $w) {
            $scan($w->get());
        }
        // the `object` newtype and the dynamic clone, only when some emitted body names them; init(), Throw
        $this->any_object_used = isset($referenced['AnyObject']);
        $this->clone_mixed_used = isset($referenced['php_clone_mixed']);
        $any = new Writer();
        $this->emitAnyObject($any, $this->any_object_used, $this->clone_mixed_used);
        $scan($any->get());
        // whether Mixed exists in this program is decided by the emitted BODIES alone: the type impls emitted in
        // the rounds below may mention Mixed among themselves (stale conversions recorded by discarded emission
        // passes), which must not resurrect it
        $mixed_wanted = isset($referenced['Mixed']);
        $this->casts->mixed_allowed = $mixed_wanted;
        $drop_mixed = function () use ($mixed_wanted, &$referenced): void {
            if ($mixed_wanted) {
                return;
            }
            unset($referenced['Mixed']);
            $this->casts->casts = array_filter($this->casts->casts, static fn(array $p) => $p[0]->kind !== RustType::MIXED && $p[1]->kind !== RustType::MIXED && !$p[0]->containsMixed() && !$p[1]->containsMixed());
            $this->casts->instance_checks = array_filter($this->casts->instance_checks, static fn(array $p) => $p[0]->kind !== RustType::MIXED && !$p[0]->containsMixed());
        };
        $drop_mixed();
        $emitted = [];
        $select = function (RustType $from, RustType $to) use ($casts_w): ?Writer {
            $home = max($this->program->typeCrate($from), $this->program->typeCrate($to));
            if ($home > 0 && !$this->localTo($from, $home) && !$this->localTo($to, $home)) {
                // orphan rules: neither side is a type of the crate the impl would have to live in
                $this->casts->warnings[] = $from->toRust() . ' => ' . $to->toRust() . ' (no crate may implement it)';
                return null;
            }
            return $casts_w[$home];
        };
        for ($round = 0; $round < 10; $round++) {
            $new = false;
            $drop_mixed();
            foreach ($this->program->types->unions as $name => $u) {
                if (!isset($emitted[$name]) && isset($referenced[$name])) {
                    $emitted[$name] = true;
                    $cast_emitter->emitUnion($u, $types_w[$this->program->typeCrate($u)]);
                    $new = true;
                }
            }
            foreach ($this->program->types->shapes as $name => $s) {
                if (!isset($emitted[$name]) && isset($referenced[$name])) {
                    $emitted[$name] = true;
                    $cast_emitter->emitShape($s, $types_w[$this->program->typeCrate($s)]);
                    $new = true;
                }
            }
            $before = count($this->casts->casts) + count($this->casts->instance_checks);
            $cast_emitter->emitRecordedCasts($select);
            foreach ($types_w as $w) {
                $scan($w->get());
            }
            foreach ($casts_w as $w) {
                $scan($w->get());
            }
            if (!$new && $before === count($this->casts->casts) + count($this->casts->instance_checks)) {
                break;
            }
        }

        fwrite(STDERR, "[transpiler] $n_tests tests, writing " . $n_crates . " crate(s)\n");
        $this->buildPathMap();
        for ($i = 0; $i < $n_crates; $i++) {
            $this->writeCrate($i, $types_w[$i], $casts_w[$i], $i === 0 ? $any : null, $tests_w[$i]);
        }
        $this->diag->report();
        foreach ($this->casts->warnings as $wmsg) {
            fwrite(STDERR, "  [casts] unsupported conversion: $wmsg\n");
        }
        foreach ($this->program->types->unsupported as $atomic => $n) {
            fwrite(STDERR, "  [types] unsupported atomic $atomic: $n\n");
        }
        $mixed_roots = $this->program->types->mixed_roots;
        arsort($mixed_roots);
        $mixed_total = array_sum($mixed_roots);
        fwrite(STDERR, "[transpiler] Mixed roots: $mixed_total total type-mappings to Mixed\n");
        foreach ($mixed_roots as $why => $n) {
            fwrite(STDERR, "  [mixed] $why: $n\n");
        }
        // top contexts per root (which PHP members to type), for the actionable roots
        foreach ($this->program->types->mixed_sites as $why => $sites) {
            arsort($sites);
            $top = array_slice($sites, 0, 25, true);
            fwrite(STDERR, "[transpiler] Mixed root '$why' top contexts (" . count($sites) . " total):\n");
            foreach ($top as $ctx => $n) {
                fwrite(STDERR, "  [mixed:$why] $ctx ($n)\n");
            }
        }
        // map inventories (user directive: arrays are lists, shapes or string-keyed maps; int keys rarely, array-key never)
        foreach (['arraykey' => $this->program->types->array_key_sites, 'intkey' => $this->program->types->int_key_sites, 'shapemap' => $this->program->types->shape_map_sites] as $tag => $sites) {
            ksort($sites);
            $n = 0;
            foreach ($sites as $types) {
                $n += count($types);
            }
            fwrite(STDERR, "[transpiler] $tag maps: $n distinct sites in " . count($sites) . " contexts\n");
            foreach ($sites as $ctx => $types) {
                foreach ($types as $t => $count) {
                    fwrite(STDERR, "  [$tag] $ctx: $t ($count)\n");
                }
            }
        }
    }

    /** Is `$t` a type whose definition is generated in crate `$crate` (so that impls for it may live there)? */
    private function localTo(RustType $t, int $crate): bool
    {
        return Casts::isLocal($t) && $t->kind !== RustType::ANY_OBJECT && $this->program->typeCrate($t) === $crate;
    }

    /** @var array<string, int> `crate::...` path of every generated item => crate it lives in */
    private array $path_map = [];

    private function buildPathMap(): void
    {
        foreach ($this->program->uniqueClasses() as $cls) {
            if (!$cls->is_project || $cls->isTrait()) {
                continue;
            }
            $this->path_map[$cls->path()] = $cls->crate;
            $this->path_map[$cls->ownPath()] = $cls->crate;
            $this->path_map[$cls->objPath()] = $cls->crate;
        }
        foreach ($this->program->functions as $fn) {
            $this->path_map[$fn->path()] = $this->program->crateOfRecord($fn->record);
        }
        foreach ($this->program->constants as $c) {
            $this->path_map['crate::consts::' . Names::constant($c->name)] = $this->program->crateOfRecord($c->record);
        }
        foreach ($this->program->files as $file) {
            if ($file !== null && $file->isData()) {
                $this->path_map[$file->path()] = $file->crate;
            }
        }
    }

    /**
     * Rewrite `crate::...` paths of items that live in another crate to that crate's name (code is emitted
     * crate-agnostically with `crate::` paths).
     */
    private function rewritePaths(string $code, int $crate): string
    {
        if ($crate === 0 && count($this->transpiler->splits) === 0) {
            return $code;
        }
        return preg_replace_callback('/\bcrate::((?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_][A-Za-z0-9_]*)/', function (array $m) use ($crate): string {
            $segs = explode('::', $m[1]);
            for ($n = count($segs); $n >= 1; $n--) {
                $path = 'crate::' . implode('::', array_slice($segs, 0, $n));
                if (isset($this->path_map[$path])) {
                    $home = $this->path_map[$path];
                    return $home === $crate ? $m[0] : '::' . $this->transpiler->crateName($home) . '::' . $m[1];
                }
            }
            return $m[0];
        }, $code) ?? $code;
    }

    private function module(int $crate, string $path): Writer
    {
        return $this->modules[$crate][$path] ??= new Writer();
    }

    /**
     * The module a class's Rust items are written to. Each class gets its OWN submodule (one file per
     * class, mirroring the PHP source layout) so no single generated module becomes a giant compilation
     * unit — one 200k-line tests module made rustc's codegen exhaust memory. Class item PATHS
     * (Names::classPath / ClassModel::ownPath / objPath) resolve to this same module, so references need
     * no re-export.
     */
    private function classModule(ClassModel $cls): Writer
    {
        return $this->module($cls->crate, Names::classModule($cls->fqcn));
    }

    /** @var array<string, true> data files already emitted (by root-relative path) */
    private array $emitted_files = [];

    /** Emit the compiled value of every data file not emitted yet into its crate's `files` module. */
    private function emitDataFiles(DataEmitter $data_emitter): void
    {
        foreach ($this->program->files as $rel => $file) {
            if ($file === null || !$file->isData() || isset($this->emitted_files[$rel])) {
                continue;
            }
            $this->emitted_files[$rel] = true;
            if (!isset($this->modules[$file->crate]['files'])) {
                $this->module($file->crate, 'files')->line('use php_rt::data::{Data, DataKey};');
            }
            $w = $this->module($file->crate, 'files');
            $demands = $this->program->data_demands[$rel] ?? [];
            try {
                $table = $file->data !== null ? $data_emitter->emit($file->data) : $data_emitter->emitFile($file->abs_path);
                $w->line('fn ' . $file->rustName() . '_table() -> &\'static Data { static D: Data = ' . $table . '; &D }');
            } catch (\RuntimeException $e) {
                fwrite(STDERR, '  [transpiler] data file ' . $rel . ' not compiled: ' . $e->getMessage() . "\n");
                $w->line('fn ' . $file->rustName() . '_table() -> &\'static Data { panic!(' . Names::rustStringLiteral('include(' . $rel . '): file could not be compiled: ' . $e->getMessage()) . ') }');
            }
            foreach ($demands as $key => $t) {
                if ($key === 'mixed') {
                    $w->line('pub fn ' . $file->rustName() . '() -> Mixed { ' . $file->rustName() . '_table().to_mixed() }');
                } else {
                    $w->line('pub fn ' . $file->rustName() . '_' . $key . '() -> ' . $t->toRust() . ' { <' . $t->toRust() . ' as php_rt::data::FromData>::from_data(' . $file->rustName() . '_table()) }');
                }
            }
        }
    }

    private function emitFunction(FunctionModel $fn, Writer $w): void
    {
        $record = $fn->record;
        $b = new BodyEmitter($this->program, $record, null, $this->casts, $this->builtins, $this->diag, null);
        $b->this_type = null;
        $b->generics = $fn->generics;
        $params = [];
        $decls = [];
        foreach ($record->storage->params as $i => $p) {
            $t = $fn->param_types[$i] ?? RustType::mixed();
            $params[$p->name] = $t;
            if ($p->by_ref) {
                $b->byref[$p->name] = true;
                $decls[] = 'mut ' . Names::var($p->name) . ': &mut ' . $t->toRust();
            } elseif (isset($fn->borrow_params[$i])) {
                // owned/borrowed (axis 5): non-escaping read-only param -> `&T`, callers borrow (no clone).
                $b->borrow[$p->name] = true;
                $decls[] = Names::var($p->name) . ': &' . $t->toRust();
            } else {
                $decls[] = 'mut ' . Names::var($p->name) . ': ' . $t->toRust();
            }
        }
        try {
            $body = $b->emitBody($params, $record->node->stmts, $fn->return_type);
        } catch (\Throwable $e) {
            $this->diag->warn('transpiler error: ' . $e->getMessage() . ' @ ' . basename($e->getFile()) . ':' . $e->getLine(), $record->node, $record->file_path);
            $body = "    unreachable!(\"transpiler error\")\n";
        }
        $w->line('pub fn ' . $fn->rustName() . ClassEmitter::genericParams($fn->generics) . '(' . implode(', ', $decls) . ') -> ' . $fn->return_type->toRust() . ' {');
        $w->raw($body);
        $w->line('}');
    }

    private bool $any_object_used = true;
    private bool $clone_mixed_used = true;

    private function emitAnyObject(Writer $w, bool $with_any, bool $with_clone_mixed): void
    {
        if ($with_clone_mixed) {
            $w->line('pub fn php_clone_mixed(m: Mixed) -> Mixed { match m { Mixed::Obj(o) => Mixed::Obj(o.php_clone_dyn()), other => other } }');
        }
        if ($with_any) {
        // `object`-typed values: a newtype over the runtime's type-erased handle. Every class converts
        // into it directly (`AnyObject(Rc::new(own))`) and narrows out of it through its `TryDowncast`
        // (a class-id match over the target hierarchy), so no program-wide enum is needed.
        $w->line('#[derive(Clone)]');
        $w->line('pub struct AnyObject(pub AnyObj);');
        $w->open('impl php_rt::PhpObject for AnyObject {');
        $w->line('fn class_name(&self) -> &\'static str { self.0.class_name() }');
        $w->line('fn class_ancestors(&self) -> &\'static [&\'static str] { self.0.class_ancestors() }');
        $w->line('fn class_id(&self) -> u32 { self.0.class_id() }');
        $w->line('fn class_ancestor_ids(&self) -> &\'static [u32] { self.0.class_ancestor_ids() }');
        $w->line('fn obj_id(&self) -> usize { self.0.obj_id() }');
        $w->line('fn as_any(&self) -> &dyn std::any::Any { self.0.as_any() }');
        $w->line('fn props(&self) -> Vec<(Str, Mixed)> { self.0.props() }');
        $w->line('fn set_prop(&self, name: &str, value: Mixed) -> bool { self.0.set_prop(name, value) }');
        $w->line('fn get_prop(&self, name: &str) -> Option<Mixed> { self.0.get_prop(name) }');
        $w->line('fn php_to_string(&self) -> Option<Str> { self.0.php_to_string() }');
        $w->line('fn call_method(&self, name: &str, args: Vec<Mixed>) -> Mixed { self.0.call_method(name, args) }');
        $w->line('fn public_props(&self) -> Vec<(Str, Mixed)> { self.0.public_props() }');
        $w->line('fn php_clone_dyn(&self) -> AnyObj { self.0.php_clone_dyn() }');
        $w->close();
        $w->open('impl AnyObject {');
        $w->line('pub fn from_mixed(m: Mixed) -> AnyObject { if let Mixed::Obj(o) = m { return AnyObject(o); } panic!("not an object: {:?}", m) }');
        $w->line('pub fn to_php_string(&self) -> Str { self.php_to_string().unwrap_or_else(|| panic!("Uncaught exception: Object could not be converted to string")) }');
        $w->close();
        $w->line('impl php_rt::CastTo<Mixed> for AnyObject { fn cast_to(self) -> Mixed { Mixed::Obj(self.0) } }');
        $w->line('impl php_rt::CastTo<AnyObject> for Mixed { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(self) } }');
        $w->line('impl php_rt::TryDowncast for AnyObject { fn try_downcast(o: &AnyObj) -> Option<Self> { Some(AnyObject(o.clone())) } }');
        $w->line('impl php_rt::Truthy for AnyObject { fn truthy(&self) -> bool { true } }');
        $w->line('impl php_rt::PhpKind for AnyObject { fn php_kind(&self) -> php_rt::Kind { php_rt::Kind::Obj } fn php_class_name(&self) -> Option<&\'static str> { Some(self.0.class_name()) } }');
        $w->line('impl php_rt::InstanceOfName for AnyObject { fn php_instance_of(&self, __n: &[u8]) -> bool { self.0.class_ancestors().iter().any(|a| a.as_bytes().eq_ignore_ascii_case(__n)) } }');
        $w->line('impl php_rt::Identical for AnyObject { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }');
        $w->line('impl php_rt::PhpCmp for AnyObject { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { Mixed::Obj(self.0.clone()).php_cmp(&Mixed::Obj(o.0.clone())) } }');
        $w->line('impl php_rt::ToStr for AnyObject { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_str(self.class_name())) } }');
        $w->line('impl std::fmt::Debug for AnyObject { fn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result { write!(f, "object({})", self.class_name()) } }');
        $w->line('impl php_rt::PhpClone for AnyObject { fn php_clone(&self) -> Self { AnyObject(self.0.php_clone_dyn()) } }');
        }

        $this->emitInit($w, 0);

        // Throwable alias: the generated Throw type
        $throwable = $this->program->getClass('Throwable');
        if ($throwable !== null && $throwable->is_project) {
            $w->line('pub type Throw = ' . $throwable->path() . ';');
            $this->emitThrowSupport($throwable, $w);
        } else {
            $w->line('pub type Throw = php_rt::FallbackThrow;');
            $w->line('#[inline] pub fn __throw_rt(e: RtError) -> ! { php_rt::uncaught(e) }');
            $w->line('#[inline] pub fn __unwrap<T>(r: R<T>) -> T { r.unwrap_or_else(|e| php_rt::uncaught(e)) }');
        }
    }

    /** `init()`: runs the upstream crates' initializers (kept as the program's entry hook). */
    private function emitInit(Writer $w, int $crate): void
    {
        $w->line('thread_local! { static __INIT: std::cell::Cell<bool> = std::cell::Cell::new(false); }');
        $w->open('pub fn init() {');
        $w->line('if __INIT.with(|c| c.replace(true)) { return; }');
        // silence Rust panic-hook backtraces for in-flight PHP exceptions (PhpThrow), which are normal control flow
        $w->line('php_rt::install_throw_panic_hook();');
        for ($i = 0; $i < $crate; $i++) {
            $w->line('::' . $this->transpiler->crateName($i) . '::init();');
        }
        $w->close();
        $this->emitNames($w, $crate);
    }

    /**
     * `names`: static tables of the crate's classes, functions and constants for the name-based builtins
     * (`class_exists`, `is_subclass_of` on class names, `defined`, `constant`, `get_declared_classes`, ...).
     * Each crate's tables fall back to the upstream crate's, the main crate to the runtime's builtin tables.
     */
    private function emitNames(Writer $w, int $crate): void
    {
        $up = $crate > 0 ? '::' . $this->transpiler->crateName($crate - 1) . '::names::' : null;
        $w->open('pub mod names {');
        $w->line('use php_rt::prelude::*;');
        for ($i = 0; $i < $crate; $i++) {
            $w->line('use ::' . $this->transpiler->crateName($i) . '::generated::*;');
        }
        $w->line('use crate::generated::*;');
        $w->line('use crate::Throw;');
        $w->line('pub use php_rt::names::ClassInfo;');
        $classes = [];
        $all_functions = [];
        foreach ($this->program->uniqueClasses() as $cls) {
            if (!$cls->is_project || $cls->crate !== $crate) {
                continue;
            }
            $classes[$cls->lc()] = $cls;
        }
        ksort($classes);
        $rows = [];
        foreach ($classes as $lc => $cls) {
            $names = [];
            foreach ($cls->ancestors as $a) {
                $names[] = Names::rustStringLiteral(strtolower($a->fqcn));
            }
            $kind = $cls->isInterface() ? 1 : ($cls->isTrait() ? 2 : ($cls->isEnum() ? 3 : 0));
            // the runtime stubs stand in for classes PHP itself provides: a compiled program must
            // report them as internal (no file), or anything reflecting one -- Psalm scanning
            // `Stringable`, say -- reads the stub's source as if it were the program's own
            $file = $this->transpiler->classes[$lc]->file_path ?? '';
            if ($file !== '' && str_contains(str_replace('\\', '/', $file), '/Transpiler/runtime-stubs/')) {
                $file = '';
            }
            $rows[] = '(' . Names::byteStrLiteral($lc) . ', ClassInfo { name: ' . Names::rustStringLiteral($cls->fqcn) . ', ancestors: &[' . implode(', ', $names) . '], kind: ' . $kind . ', file: ' . Names::rustStringLiteral($file) . ' })';
        }
        $w->line('static CLASSES: &[(&[u8], ClassInfo)] = &[' . implode(', ', $rows) . '];');
        $w->line('pub fn class_info(name: &Str) -> Option<&\'static ClassInfo> { class_info_lc(&php_rt::names::norm(name)) }');
        $w->line('pub fn class_info_lc(lc: &[u8]) -> Option<&\'static ClassInfo> { match CLASSES.binary_search_by(|(k, _)| (*k).cmp(lc)) { Ok(i) => Some(&CLASSES[i].1), Err(_) => ' . ($up !== null ? $up . 'class_info_lc(lc)' : 'None') . ' } }');
        $w->line('pub fn class_exists(name: &Str) -> bool { let lc = php_rt::names::norm(name); match class_info_lc(&lc) { Some(i) => i.kind == 0 || i.kind == 3, None => php_rt::names::builtin_class_exists(&lc) } }');
        $w->line('pub fn interface_exists(name: &Str) -> bool { let lc = php_rt::names::norm(name); match class_info_lc(&lc) { Some(i) => i.kind == 1, None => php_rt::names::builtin_interface_exists(&lc) } }');
        $w->line('pub fn trait_exists(name: &Str) -> bool { class_info(name).map_or(false, |i| i.kind == 2) }');
        $w->line('pub fn enum_exists(name: &Str) -> bool { class_info(name).map_or(false, |i| i.kind == 3) }');
        $w->line('pub fn class_is_trait(name: &Str) -> bool { trait_exists(name) }');
        $w->line('pub fn class_file(name: &Str) -> Option<Str> { class_info(name).and_then(|i| if i.file.is_empty() { None } else { Some(Str::from_static(i.file)) }) }');
        $w->line('/// `is_subclass_of($sub, $parent)` / `is_a($sub, $parent, true)` on two class names.');
        $w->line('pub fn is_subclass(sub: &Str, parent: &Str, allow_same: bool) -> bool { let s = php_rt::names::norm(sub); let p = php_rt::names::norm(parent); if s == p { return allow_same; } class_info_lc(&s).map_or(false, |i| i.ancestors.iter().any(|a| a.as_bytes() == p.as_slice())) }');
        $w->line('pub fn declared_classlikes(interfaces: bool) -> List<Str> { let mut out: List<Str> = ' . ($up !== null ? $up . 'declared_classlikes(interfaces)' : 'php_rt::names::builtin_declared(interfaces)') . '; for (_, i) in CLASSES { if (interfaces && i.kind == 1) || (!interfaces && (i.kind == 0 || i.kind == 3)) { out.push(Str::from_static(i.name)); } } out }');
        // functions
        $fns = [];
        foreach ($this->program->functions as $fn) {
            $fc = $this->program->crateOfRecord($fn->record);
            if ($fc <= $crate) {
                $all_functions[] = Names::rustStringLiteral(strtolower($fn->fq_name));
            }
            if ($fc === $crate) {
                $fns[strtolower($fn->fq_name)] = true;
            }
        }
        ksort($fns);
        $w->line('static FUNCTIONS: &[&[u8]] = &[' . implode(', ', array_map(fn($n) => Names::byteStrLiteral($n), array_keys($fns))) . '];');
        $w->line('pub static USER_FUNCTIONS: &[&str] = &[' . implode(', ', $all_functions) . '];');
        $w->line('pub fn function_exists(name: &Str) -> bool { let lc = php_rt::names::norm(name); FUNCTIONS.binary_search(&lc.as_slice()).is_ok() || ' . ($up !== null ? $up . 'function_exists(name)' : 'php_rt::builtins::misc::builtin_function_exists(&lc)') . ' }');
        // constants: `defined()` checks names; `constant()` (dynamic lookup) only when some body calls it
        $names = [];
        foreach ($this->program->constants as $c) {
            if ($this->program->crateOfRecord($c->record) === $crate) {
                $names[] = Names::byteStrLiteral($c->name);
            }
        }
        foreach ($classes as $lc => $cls) {
            if ($cls->isTrait()) {
                continue;
            }
            foreach ($cls->constants as $c) {
                $names[] = Names::byteStrLiteral($cls->fqcn . '::' . $c->name);
            }
        }
        $w->line('static CONSTANT_NAMES: &[&[u8]] = &[' . implode(', ', array_unique($names)) . '];');
        $w->line('pub fn constant_defined(name: &Str) -> bool { CONSTANT_NAMES.iter().any(|n| *n == name.as_bytes()) || ' . ($up !== null ? $up . 'constant_defined(name)' : 'php_rt::builtins::misc::builtin_constant_defined(name)') . ' }');
        if ($this->program->uses_constant_fn) {
            $arms = [];
            foreach ($this->program->constants as $c) {
                if ($this->program->crateOfRecord($c->record) !== $crate) {
                    continue;
                }
                $arms[] = Names::byteStrLiteral($c->name) . ' => Some(' . $this->casts->convert('crate::consts::' . Names::constant($c->name) . '()', $c->type, RustType::mixed()) . ')';
            }
            $w->line('pub fn constant_value(name: &Str) -> Option<Mixed> { match name.as_bytes() { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => ' . ($up !== null ? $up . 'constant_value(name)' : 'None') . ' } }');
            $w->line('pub fn constant(name: &Str) -> Mixed { constant_value(name).unwrap_or_else(|| panic!("Uncaught exception: Undefined constant {}", name)) }');
        }
        $w->close();
    }

    private function emitThrowSupport(ClassModel $throwable, Writer $w): void
    {
        $tt = RustType::class($throwable->fqcn);
        $mk = function (string $cls, string $msg_code) use ($tt): string {
            $c = $this->program->getClass($cls);
            if ($c === null || !$c->is_project) {
                return 'panic!("missing runtime stub class ' . $cls . '")';
            }
            return $this->casts->convert($c->path() . '::new(' . $msg_code . ', 0i64, None)', RustType::class($c->fqcn), $tt);
        };
        $w->open('impl ' . $throwable->path() . ' {');
        $w->line('pub fn error(msg: Str) -> Self { ' . $mk('Error', 'msg') . ' }');
        $w->line('pub fn type_error(msg: Str) -> Self { ' . $mk('TypeError', 'msg') . ' }');
        $w->line('pub fn value_error(msg: Str) -> Self { ' . $mk('ValueError', 'msg') . ' }');
        $w->line('pub fn assertion(msg: Str) -> Self { ' . $mk('AssertionError', 'msg') . ' }');
        $w->line('pub fn unhandled_match<T: std::fmt::Debug>(v: &T) -> Self { ' . $mk('UnhandledMatchError', 'php_rt::sfmt!("Unhandled match case {:?}", v)') . ' }');
        $exit = $this->program->getClass('PhpExitException');
        if ($exit !== null && $exit->is_project) {
            $w->line('pub fn exit(status: i64) -> Self { ' . $this->casts->convert($exit->path() . '::new(status)', RustType::class($exit->fqcn), $tt) . ' }');
            $w->line('pub fn exit_status(&self) -> Option<i64> { if is_instance::<' . $exit->path() . '>(self) { Some(cast::<' . $exit->path() . '>(self.clone()).p_status_get()) } else { None } }');
            $this->casts->needInstanceOf($tt, RustType::class($exit->fqcn));
            $this->casts->need($tt, RustType::class($exit->fqcn));
        } else {
            $w->line('pub fn exit(status: i64) -> Self { Self::error(cat!(Str::from_static("exit "), to_str(&status))) }');
            $w->line('pub fn exit_status(&self) -> Option<i64> { None }');
        }
        $w->line('pub fn message(&self) -> Str { self.getMessage() }');
        $w->close();

        // the typed exception interface of the test harness (php_rt::testing) and of php-rt's take_thrown
        $w->line('impl php_rt::PhpThrowable for ' . $throwable->path() . ' { fn class_name(&self) -> &\'static str { php_rt::PhpObject::class_name(self) } fn class_ancestors(&self) -> &\'static [&\'static str] { php_rt::PhpObject::class_ancestors(self) } fn message(&self) -> Str { self.getMessage() } }');
        $classes = ['Error', 'TypeError', 'ValueError', 'ArgumentCountError', 'ArithmeticError', 'DivisionByZeroError', 'AssertionError', 'UnhandledMatchError', 'JsonException', 'RuntimeException', 'LogicException', 'InvalidArgumentException', 'UnexpectedValueException', 'OutOfBoundsException'];
        $arms = [];
        foreach ($classes as $cls) {
            $c = $this->program->getClass($cls);
            if ($c !== null && $c->is_project) {
                $arms[] = Names::rustStringLiteral($cls) . ' => ' . $this->casts->convert($c->path() . '::new(e.message, 0i64, None)', RustType::class($c->fqcn), $tt);
            }
        }
        $w->line('impl From<RtError> for ' . $throwable->path() . ' { fn from(e: RtError) -> Self { match e.class { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => Self::error(e.message) } } }');
        $w->line('impl From<DynError> for ' . $throwable->path() . ' { fn from(e: DynError) -> Self { match e { DynError::Rt(r) => Self::from(r) } } }');
        $w->line('impl std::fmt::Display for ' . $throwable->path() . ' { fn fmt(&self, f: &mut std::fmt::Formatter<\'_>) -> std::fmt::Result { write!(f, "{}: {}", self.class_name(), self.message()) } }');
        // Convert a runtime error (RRtError) into a PHP exception object and throw it via the panic-based
        // error model, so a surrounding PHP try/catch can catch runtime failures (division by zero, etc.).
        $w->line('#[inline] pub fn __throw_rt(e: RtError) -> ! { php_rt::do_throw(' . $throwable->path() . '::from(e)) }');
        // Unwrap a fallible runtime call (php-rt `R<T>`) in a non-Result context: on error, throw it.
        $w->line('#[inline] pub fn __unwrap<T>(r: R<T>) -> T { match r { Ok(v) => v, Err(e) => __throw_rt(e) } }');
    }

    private function writeCrate(int $crate, Writer $types, Writer $casts, ?Writer $any, Writer $tests): void
    {
        $out = $this->transpiler->crateDir($crate);
        $name = $this->transpiler->crateName($crate);
        $src = $out . '/src';
        if (!is_dir($src)) {
            mkdir($src, 0777, true);
        }
        $upstream = [];
        for ($i = 0; $i < $crate; $i++) {
            $upstream[] = $this->transpiler->crateName($i);
        }
        $prelude = "#![allow(unused_imports, unused_variables, unused_mut, dead_code, non_snake_case, non_camel_case_types, unreachable_code, unused_parens, unused_braces, unused_assignments, unused_labels, unused_unsafe, clippy::all, irrefutable_let_patterns, unreachable_patterns, unused_must_use, non_upper_case_globals, deprecated, ambiguous_glob_reexports, hidden_glob_reexports)]\n";
        $use = "use php_rt::prelude::*;\n";
        foreach ($upstream as $up) {
            $use .= "use ::$up::generated::*;\n";
        }
        $use .= "use crate::generated::*;\nuse crate::Throw;\n" . ($this->any_object_used ? "use crate::AnyObject;\n" : '') . "use crate::{__throw_rt, __unwrap};\n";

        // module tree
        $tree = [];
        foreach ($this->modules[$crate] ?? [] as $path => $w) {
            $segs = explode('::', $path);
            $node = &$tree;
            foreach ($segs as $s) {
                $node[$s] ??= [];
                $node = &$node[$s];
            }
            unset($node);
        }
        // recursion_limit: rustc's Send/Sync auto-trait proof over the recursive node graph overflows the default
        $lib = "#![recursion_limit = \"4096\"]\n" . $prelude . "pub mod generated;\npub mod consts;\npub use generated::*;\n";
        $lib .= "/// Marker of the leaf classes of this crate (targets of the generic narrowing casts of dispatch enums).\npub trait Leaf__ {}\n";
        $lib .= $this->writeTree($crate, $tree, $src, $use, '') . "\n";
        if ($any !== null) {
            $lib .= "use php_rt::prelude::*;\nuse crate::generated::*;\n" . $any->get();
        } else {
            $base = $this->transpiler->crateName(0);
            $lib .= "pub use ::$base::{Throw" . ($this->any_object_used ? ', AnyObject' : '') . ($this->clone_mixed_used ? ', php_clone_mixed' : '') . "};\nuse php_rt::prelude::*;\n";
            foreach ($upstream as $up) {
                $lib .= "use ::$up::generated::*;\n";
            }
            $lib .= "use crate::generated::*;\n";
            $init = new Writer();
            $this->emitInit($init, $crate);
            $lib .= $init->get();
        }
        $this->writeFile($src . '/lib.rs', $this->rewritePaths($lib, $crate));
        $gen_use = "use php_rt::prelude::*;\n";
        foreach ($upstream as $up) {
            $gen_use .= "use ::$up::generated::*;\n";
        }
        $gen_use .= "use crate::*;\n";
        $this->writeFile($src . '/generated.rs', $this->rewritePaths($prelude . $gen_use . $types->get() . $casts->get(), $crate));
        $this->writeFile($src . '/consts.rs', $this->rewritePaths($prelude . $use . $this->constsModule($crate), $crate));
        // the PHPUnit harness is an integration test crate: it links against the library instead of being
        // compiled into it (a single crate with all tests exhausts memory on large projects)
        if (!is_dir($out . '/tests')) {
            mkdir($out . '/tests', 0777, true);
        }
        $harness = "#![recursion_limit = \"4096\"]\n" . $prelude . str_replace('crate::', '::' . $name . '::', $this->rewritePaths($use, $crate)) . str_replace('crate::', '::' . $name . '::', $this->rewritePaths($tests->get(), $crate));
        $this->writeFile($out . '/tests/harness.rs', $harness);
        if (file_exists($src . '/tests.rs')) {
            unlink($src . '/tests.rs');
        }
        $deps = "php-rt = { path = \"" . $this->relativePath($out, dirname($this->transpiler->out_dir) . '/../php-rt') . "\" }\n";
        for ($i = 0; $i < $crate; $i++) {
            $deps .= $this->transpiler->crateName($i) . " = { path = \"" . $this->relativePath($out, $this->transpiler->crateDir($i)) . "\" }\n";
        }
        $this->writeFile($out . '/Cargo.toml', "[package]\nname = \"" . $name . "\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\npath = \"src/lib.rs\"\ntest = false\n\n[[test]]\nname = \"harness\"\npath = \"tests/harness.rs\"\nharness = false\n\n[dependencies]\n" . $deps . "\n[dev-dependencies]\nlibtest-mimic = \"0.8\"\n");
        fwrite(STDERR, 'wrote ' . count($this->modules[$crate] ?? []) . " modules to $out\n");
    }

    /** Write a file unless its content is unchanged (so cargo does not rebuild untouched crates). */
    private function writeFile(string $path, string $content): void
    {
        if (file_exists($path) && file_get_contents($path) === $content) {
            return;
        }
        file_put_contents($path, $content);
    }

    /** Relative path from directory `$from` to `$to`. */
    private function relativePath(string $from, string $to): string
    {
        $from = realpath($from) ?: $from;
        $to = realpath($to) ?: $to;
        $f = explode('/', trim($from, '/'));
        $t = explode('/', trim($to, '/'));
        while ($f && $t && $f[0] === $t[0]) {
            array_shift($f);
            array_shift($t);
        }
        return implode('/', [...array_fill(0, count($f), '..'), ...$t]) ?: '.';
    }

    /** Write nested module directories; returns the `pub mod` declarations for this level. */
    private function writeTree(int $crate, array $tree, string $dir, string $use, string $prefix): string
    {
        ksort($tree);
        $decls = '';
        foreach ($tree as $seg => $children) {
            $path = $prefix === '' ? $seg : $prefix . '::' . $seg;
            $moddir = $dir . '/' . $seg;
            if (!is_dir($moddir)) {
                mkdir($moddir, 0777, true);
            }
            $content = $use;
            $content .= $this->writeTree($crate, $children, $moddir, $use, $path);
            if (isset($this->modules[$crate][$path])) {
                $content .= $this->modules[$crate][$path]->get();
            }
            $this->writeFile($moddir . '/mod.rs', $this->rewritePaths($content, $crate));
            $decls .= 'pub mod ' . $seg . ";\n";
        }
        return $decls;
    }

    private function constsModule(int $crate): string
    {
        $out = '';
        foreach ($this->program->constants as $c) {
            if ($this->program->crateOfRecord($c->record) !== $crate) {
                continue;
            }
            $b = new BodyEmitter($this->program, $c->record, null, $this->casts, $this->builtins, $this->diag, null);
            $b->this_type = null;
            $code = $b->constExpr($c->expr, $c->type);
            $out .= 'pub fn ' . Names::constant($c->name) . '() -> ' . $c->type->toRust() . ' { ' . $code . " }\n";
        }
        return $out;
    }
}
