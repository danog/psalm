<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Stmt\ClassMethod;
use PhpParser\Node\Stmt\Function_;
use Psalm\Codebase;
use Psalm\Internal\MethodIdentifier;
use Psalm\Storage\ClassLikeStorage;
use Psalm\Storage\FunctionLikeStorage;
use Psalm\Storage\MethodStorage;

use function array_keys;
use function explode;
use function max;
use function array_key_exists;
use function array_pop;
use function file_get_contents;
use function glob;
use function implode;
use function is_file;
use function rtrim;
use function str_starts_with;
use function strlen;
use function substr;
use function strtolower;
use function usort;
use function fwrite;

use const STDERR;

/**
 * The whole-program model: classes, hierarchy, functions.
 *
 * @internal
 */
final class Program
{
    /** @var array<lowercase-string, ClassModel> */
    public array $classes = [];

    /** @var array<lowercase-string, FunctionModel> free functions by lowercase fully-qualified name */
    public array $functions = [];

    public TypeMapper $types;

    /** @var array<string, MethodModel> cache keyed by "class::method" */
    private array $method_cache = [];

    /** @var array<lowercase-string, GlobalConstModel> global constants by lowercase name */
    public array $constants = [];


    public function __construct(
        public readonly Codebase $codebase,
        public readonly Transpiler $transpiler,
    ) {
        $this->types = new TypeMapper($codebase, $this);
        $this->build();
    }

    /** @return list<ClassModel> distinct class models (aliases share a model) */
    public function uniqueClasses(): array
    {
        $seen = [];
        $out = [];
        foreach ($this->classes as $model) {
            $id = spl_object_id($model);
            if (!isset($seen[$id])) {
                $seen[$id] = true;
                $out[] = $model;
            }
        }
        return $out;
    }

    private function build(): void
    {
        // 1. project classes
        foreach ($this->transpiler->classes as $lc => $record) {
            $this->classes[$lc] = new ClassModel($record->storage->name, $record->storage, $record->node, true);
            $this->classes[$lc]->crate = $this->transpiler->crateOfFile($record->file_path);
        }

        fwrite(STDERR, "[program] hierarchy\n");
        // 2. hierarchy (also pulls in external ancestors)
        foreach (array_keys($this->classes) as $lc) {
            $model = $this->classes[$lc];
            $this->linkAncestors($model);
        }

        foreach ($this->uniqueClasses() as $model) {
            foreach ($model->ancestors as $ancestor) {
                // dispatch enums only enumerate classes of their own crate (and upstream ones);
                // instances of subclasses from downstream crates travel through the `Other__` variant
                if ($model->isConcrete() && $model->is_project && $model->crate <= $ancestor->crate) {
                    $ancestor->concrete[] = $model;
                }
            }
            if ($model->isConcrete() && $model->is_project) {
                $model->concrete[] = $model;
            }
        }
        foreach ($this->uniqueClasses() as $model) {
            $model->concrete = $this->dedupe($model->concrete);
            $model->children = $this->dedupe($model->children);
            $model->ancestors = $this->dedupe($model->ancestors);
            usort($model->concrete, static fn(ClassModel $a, ClassModel $b) => $a->fqcn <=> $b->fqcn);
            usort($model->children, static fn(ClassModel $a, ClassModel $b) => $a->fqcn <=> $b->fqcn);
        }

        fwrite(STDERR, "[program] functions\n");
        // 3. free functions
        foreach ($this->transpiler->functions as $record) {
            if ($record->node instanceof Function_) {
                $name = (string) ($record->node->getAttribute('resolvedName') ?? $record->node->name->name);
                $this->functions[strtolower($name)] = new FunctionModel($name, $record);
            }
        }

        fwrite(STDERR, "[program] members\n");
        // 4. members
        foreach ($this->uniqueClasses() as $model) {
            $this->types->current_class = $model->fqcn;
            $this->types->current_crate = $model->crate;
            $this->buildFields($model);
            $this->buildConstants($model);
        }
        $this->types->current_class = null;
        $this->types->current_crate = 0;
        fwrite(STDERR, "[program] methods\n");
        foreach ($this->uniqueClasses() as $model) {
            $this->types->current_crate = $model->crate;
            $this->buildMethods($model);
        }
        foreach ($this->functions as $fn) {
            $this->types->current_crate = $this->crateOfRecord($fn->record);
            $this->resolveSignature($fn->record->storage, $fn->param_types, $fn->return_type, $fn->record->node);
        }
        $this->types->current_crate = 0;
        $this->propagateLateStaticBinding();
        fwrite(STDERR, "[program] cross-crate inheritance\n");
        foreach ($this->uniqueClasses() as $model) {
            if ($model->is_project && $model->crateRoot() === $model) {
                $this->importUpstreamMethods($model);
            }
        }
    }

    /**
     * `static` is forwarded through `self::m()`, `static::m()` and `parent::m()` calls: a static method
     * calling one that uses late static binding needs per-class copies too.
     */
    private function propagateLateStaticBinding(): void
    {
        $finder = new \PhpParser\NodeFinder();
        $calls = [];
        foreach ($this->uniqueClasses() as $model) {
            foreach ($model->methods as $m) {
                if (!$m->isStatic() || $m->node === null || $m->node->stmts === null || $m->declaring !== $model || $m->uses_lsb) {
                    continue;
                }
                $targets = [];
                foreach ($finder->findInstanceOf($m->node->stmts, \PhpParser\Node\Expr\StaticCall::class) as $call) {
                    if (!$call->class instanceof \PhpParser\Node\Name || !$call->name instanceof \PhpParser\Node\Identifier) {
                        continue;
                    }
                    $kind = strtolower($call->class->toString());
                    $target = $kind === 'parent' ? $model->parent : (($kind === 'self' || $kind === 'static') ? $model : null);
                    if ($target === null) {
                        continue;
                    }
                    $tm = $this->findMethod($target, strtolower($call->name->name));
                    if ($tm !== null && $tm->isStatic()) {
                        $targets[] = $tm;
                    }
                }
                if ($targets !== []) {
                    $calls[] = [$m, $targets];
                }
            }
        }
        do {
            $changed = false;
            foreach ($calls as [$m, $targets]) {
                if ($m->uses_lsb) {
                    continue;
                }
                foreach ($targets as $tm) {
                    if ($tm->uses_lsb) {
                        $m->uses_lsb = true;
                        $changed = true;
                        break;
                    }
                }
            }
        } while ($changed);
    }

    /**
     * A class extending a class of another (upstream) crate re-declares the inherited instance methods:
     * their bodies are emitted again with `$this` bound to this class, since the upstream dispatch enum
     * has no variant for it.
     */
    private function importUpstreamMethods(ClassModel $root): void
    {
        $imports = [];
        foreach ($root->methods as $lc => $m) {
            if ($m->isStatic() || $m->isAbstract() || $m->declaring->crate >= $root->crate || !$m->declaring->is_project) {
                continue;
            }
            $imports[$lc] = $m->importedInto($root);
        }
        // private methods of upstream ancestors are called by the imported bodies but are not inherited
        for ($anc = $root->parent; $anc !== null && $anc->is_project && $anc->crate < $root->crate; $anc = $anc->parent) {
            foreach ($anc->methods as $lc => $m) {
                if (isset($root->methods[$lc]) || isset($imports[$lc]) || $m->declaring !== $anc || !$m->isPrivate() || $m->isStatic() || $m->isAbstract()) {
                    continue;
                }
                $imports[$lc] = $m->importedInto($root);
                $root->methods[$lc] = $imports[$lc];
            }
        }
        if ($imports === []) {
            return;
        }
        $apply = function (ClassModel $c) use (&$apply, $imports, $root): void {
            foreach ($imports as $lc => $imp) {
                if (isset($c->methods[$lc]) && $c->methods[$lc] === $imp->import_of) {
                    $c->methods[$lc] = $imp;
                }
            }
            foreach ($c->children as $child) {
                if ($child->crate === $root->crate && $child->parent === $c) {
                    $apply($child);
                }
            }
        };
        $apply($root);
    }

    /** @var array<string, array{ClassModel, MethodModel}> bodies of upstream methods needed for `parent::m()` calls, by copy name */
    public array $super_copies = [];

    /**
     * Name of the copy of upstream method `$m` emitted on `$root`'s handle type (requested by a `parent::m()`
     * call whose target lives in another crate).
     */
    public function requestSuperCopy(ClassModel $root, MethodModel $m): string
    {
        $origin = $m->origin();
        $name = $m->rustName() . '__super_' . Names::classMangle($origin->declaring->fqcn);
        $this->super_copies[$root->lc() . '::' . $name] = [$root, $origin];
        return $name;
    }

    /** @var array<string, true> recursive unions being measured by typeCrate() */
    private array $type_crate_visiting = [];

    /** Index of the crate a generated type (union, shape, ...) is emitted into: the highest crate of any class it mentions. */
    public function typeCrate(RustType $t): int
    {
        $crate = 0;
        if ($t->kind === RustType::CLASS_) {
            $c = $this->classOf($t);
            return $c !== null ? $c->crate : 0;
        }
        if ($t->isRecursive()) {
            // the enum mentions itself through its list member
            if (isset($this->type_crate_visiting[$t->mangle()])) {
                return 0;
            }
            $this->type_crate_visiting[$t->mangle()] = true;
            try {
                foreach ($t->params as $p) {
                    $crate = max($crate, $this->typeCrate($p));
                }
            } finally {
                unset($this->type_crate_visiting[$t->mangle()]);
            }
            return $crate;
        }
        foreach ($t->params as $p) {
            $crate = max($crate, $this->typeCrate($p));
        }
        foreach ($t->fields as [$ft, $_]) {
            $crate = max($crate, $this->typeCrate($ft));
        }
        if ($t->ret !== null) {
            $crate = max($crate, $this->typeCrate($t->ret));
        }
        return $crate;
    }

    /** @var array<string, FileModel|null> includable files by root-relative path (null: known not to be compilable) */
    public array $files = [];

    /** Root-relative form of an absolute path (paths outside the root stay absolute). */
    public function relativePath(string $abs): string
    {
        $root = rtrim($this->transpiler->root_dir, '/') . '/';
        return str_starts_with($abs, $root) ? substr($abs, strlen($root)) : $abs;
    }

    /** Resolve `.`/`..` segments of a path. */
    public static function normalizePath(string $path): string
    {
        $absolute = str_starts_with($path, '/');
        $parts = [];
        foreach (explode('/', $path) as $seg) {
            if ($seg === '' || $seg === '.') {
                continue;
            }
            if ($seg === '..') {
                array_pop($parts);
                continue;
            }
            $parts[] = $seg;
        }
        return ($absolute ? '/' : '') . implode('/', $parts);
    }

    /** Compile the data files listed on the command line and register every project source file as a unit. */
    public function collectFiles(): void
    {
        $root = rtrim($this->transpiler->root_dir, '/');
        foreach ($this->transpiler->data_globs as $glob) {
            foreach (glob(str_starts_with($glob, '/') ? $glob : $root . '/' . $glob) ?: [] as $path) {
                $this->fileForPath($path);
            }
        }
        foreach ($this->transpiler->classes as $record) {
            $this->fileForPath($record->file_path);
        }
        foreach ($this->transpiler->functions as $record) {
            $this->fileForPath($record->file_path);
        }
    }

    /**
     * The includable file at an absolute path: a data file (compiled `return` value) or a unit of
     * declarations; null when the file does not exist or has top-level code that cannot be compiled.
     */
    public function fileForPath(string $abs_path): ?FileModel
    {
        $abs_path = self::normalizePath($abs_path);
        $rel = $this->relativePath($abs_path);
        if (array_key_exists($rel, $this->files)) {
            return $this->files[$rel];
        }
        $this->files[$rel] = null;
        if (!is_file($abs_path)) {
            return null;
        }
        if ($this->transpiler->isDataFile($abs_path)) {
            // a dictionary: compiled mechanically from the value the file returns (see DataEmitter::emitFile)
            $model = new FileModel($rel, $abs_path, $this->transpiler->crateOfFile($abs_path), null, true);
            $this->files[$rel] = $model;
            return $model;
        }
        $stmts = $this->transpiler->file_stmts[$abs_path] ?? null;
        if ($stmts === null) {
            $code = file_get_contents($abs_path);
            if ($code === false) {
                return null;
            }
            try {
                $parsed = (new \PhpParser\ParserFactory())->createForNewestSupportedVersion()->parse($code) ?? [];
            } catch (\PhpParser\Error $e) {
                return null;
            }
            // resolve `use` imports so class constants in data files name their class fully
            $traverser = new \PhpParser\NodeTraverser();
            $traverser->addVisitor(new \PhpParser\NodeVisitor\NameResolver(null, ['preserveOriginalNames' => true, 'replaceNodes' => false]));
            $parsed = $traverser->traverse($parsed);
            $stmts = self::leftoverStatements($parsed);
        }
        $data = null;
        foreach ($stmts as $stmt) {
            if ($stmt instanceof \PhpParser\Node\Stmt\Return_) {
                if ($data !== null || $stmt->expr === null) {
                    return null;
                }
                $data = $stmt->expr;
            } elseif ($stmt instanceof \PhpParser\Node\Stmt\Nop || $stmt instanceof \PhpParser\Node\Stmt\Declare_
                || $stmt instanceof \PhpParser\Node\Stmt\Use_ || $stmt instanceof \PhpParser\Node\Stmt\GroupUse
                || $stmt instanceof \PhpParser\Node\Stmt\InlineHTML
            ) {
                continue;
            } else {
                // top-level code: not compiled
                return null;
            }
        }
        $model = new FileModel($rel, $abs_path, $this->transpiler->crateOfFile($abs_path), $data);
        $this->files[$rel] = $model;
        return $model;
    }

    /**
     * Statements of a parsed file that are not declarations (namespaces are flattened).
     *
     * @param list<\PhpParser\Node\Stmt> $stmts
     * @return list<\PhpParser\Node\Stmt>
     */
    private static function leftoverStatements(array $stmts): array
    {
        $out = [];
        foreach ($stmts as $stmt) {
            if ($stmt instanceof \PhpParser\Node\Stmt\Namespace_) {
                foreach (self::leftoverStatements($stmt->stmts ?? []) as $inner) {
                    $out[] = $inner;
                }
                continue;
            }
            if ($stmt instanceof \PhpParser\Node\Stmt\ClassLike || $stmt instanceof \PhpParser\Node\Stmt\Function_) {
                continue;
            }
            $out[] = $stmt;
        }
        return $out;
    }

    /** Index of the crate a function or constant is emitted into. */
    public function crateOfRecord(FunctionRecord $record): int
    {
        return $this->transpiler->crateOfFile($record->file_path);
    }

    /** @param list<ClassModel> $models */
    private function dedupe(array $models): array
    {
        $seen = [];
        $out = [];
        foreach ($models as $m) {
            $id = spl_object_id($m);
            if (!isset($seen[$id])) {
                $seen[$id] = true;
                $out[] = $m;
            }
        }
        return $out;
    }

    private function linkAncestors(ClassModel $model): void
    {
        if ($model->linked) {
            return;
        }
        $model->linked = true;
        $storage = $model->storage;
        $seen = [];
        if ($storage->parent_class !== null) {
            $parent = $this->getClass($storage->parent_class);
            if ($parent !== null) {
                $model->parent = $parent;
                $parent->children[] = $model;
                $this->linkAncestors($parent);
                $seen[$parent->lc()] = $parent;
                foreach ($parent->ancestors as $a) {
                    $seen[$a->lc()] = $a;
                }
            }
        }
        $interfaces = $storage->is_interface ? $storage->parent_interfaces : $storage->class_implements;
        foreach ($interfaces as $lc_iface => $_) {
            $iface = $this->getClass($lc_iface);
            if ($iface === null || $iface === $model || isset($seen[$iface->lc()])) {
                continue;
            }
            $this->linkAncestors($iface);
            $seen[$iface->lc()] = $iface;
            foreach ($iface->ancestors as $a) {
                $seen[$a->lc()] = $a;
            }
        }
        $model->ancestors = array_values($seen);
        // direct interface parents count as children links for dispatch enums
        $direct = $storage->is_interface ? $storage->direct_interface_parents : $storage->direct_class_interfaces;
        foreach ($direct as $lc_iface => $_) {
            $iface = $this->getClass($lc_iface);
            if ($iface !== null && $iface !== $model) {
                $iface->children[] = $model;
            }
        }
    }

    /** Get or lazily create a class model (external classes are created on demand). */
    public function getClass(string $fqcn): ?ClassModel
    {
        $lc = strtolower($this->codebase->classlikes->getUnAliasedName(ltrim($fqcn, '\\')));
        if (isset($this->classes[$lc])) {
            return $this->classes[$lc];
        }
        if (!$this->codebase->classlike_storage_provider->has($lc)) {
            return null;
        }
        $storage = $this->codebase->classlike_storage_provider->get($lc);
        $alias_of = $this->aliasTarget($storage);
        if ($alias_of !== null) {
            $target = $this->getClass($alias_of);
            if ($target !== null) {
                $this->classes[$lc] = $target;
                return $target;
            }
        }
        $model = new ClassModel($storage->name, $storage, null, false);
        $this->classes[$lc] = $model;
        $this->linkAncestors($model);
        return $model;
    }

    /**
     * Classes declared in project files that were never analyzed (e.g. `if (false) { class X extends Y {} }`
     * compatibility shims) are treated as aliases of their parent.
     */
    private function aliasTarget(ClassLikeStorage $storage): ?string
    {
        if ($storage->parent_class === null || $storage->location === null) {
            return null;
        }
        if (!$this->transpiler->config->isInProjectDirs($storage->location->file_path)) {
            return null;
        }
        if (isset($this->transpiler->classes[strtolower($storage->name)])) {
            return null;
        }
        foreach ($storage->methods as $m) {
            if ($m->defining_fqcln === null || strtolower($m->defining_fqcln) === strtolower($storage->name)) {
                return null;
            }
        }
        if ($storage->properties !== []) {
            return null;
        }
        return $storage->parent_class;
    }

    public function canonicalClassName(string $name): string
    {
        $name = $this->codebase->classlikes->getUnAliasedName(ltrim($name, '\\'));
        $lc = strtolower($name);
        if (isset($this->classes[$lc])) {
            return $this->classes[$lc]->fqcn;
        }
        $model = $this->getClass($name);
        if ($model !== null) {
            return $model->fqcn;
        }
        return ltrim($name, '\\');
    }

    private function buildFields(ClassModel $model): void
    {
        // inherited first (parent chain), preserving declaration order
        if ($model->parent !== null) {
            $this->buildFields($model->parent);
            foreach ($model->parent->fields as $name => $field) {
                $model->fields[$name] = $field;
            }
        }
        // types are mapped in the class' own crate context (this may run for a parent from a subclass' build)
        $this->types->current_class = $model->fqcn;
        $this->types->current_crate = $model->crate;
        $storage = $model->storage;
        $properties = $storage->properties;
        // properties brought in by used traits are not copied into the class' own property storage
        foreach ($storage->declaring_property_ids as $name => $declaring_id) {
            if (isset($properties[$name]) || isset($model->fields[$name])) {
                continue;
            }
            $trait = $this->getClass($declaring_id);
            if ($trait !== null && $trait->isTrait() && isset($trait->storage->properties[$name])) {
                $properties[$name] = $trait->storage->properties[$name];
            }
        }
        foreach ($properties as $name => $prop_storage) {
            $declaring_id = $storage->declaring_property_ids[$name] ?? $model->fqcn;
            $declaring = $this->getClass($declaring_id) ?? $model;
            if ($declaring !== $model && !$storage->is_trait) {
                // trait property: treat the using class as declaring for emission purposes
                $declaring_storage = $declaring->storage;
                if (!$declaring_storage->is_trait) {
                    continue;
                }
            }
            $default = null;
            $has_default = $prop_storage->has_default;
            $node_class = $declaring->is_project ? $declaring : $model;
            if ($node_class->node !== null) {
                $declared = (bool) $prop_storage->is_promoted;
                foreach ($node_class->node->getProperties() as $pnode) {
                    foreach ($pnode->props as $item) {
                        if ($item->name->name === $name) {
                            $declared = true;
                            $default = $item->default;
                            $has_default = $has_default || $item->default !== null;
                        }
                    }
                }
                if (!$declared && $declaring === $model && !$storage->is_trait) {
                    // declared only by another definition of the class (Psalm's own stub of a builtin class
                    // that a runtime stub replaces): not part of the generated struct
                    continue;
                }
            }
            if (isset($model->fields[$name]) && !$prop_storage->is_static) {
                // redeclared inherited property: keep the root declaration's type
                $root = $model->fields[$name];
                $model->fields[$name] = new FieldModel(
                    $name,
                    $root->type,
                    $root->declaring,
                    $root->storage,
                    $default ?? $root->default,
                    $has_default || $root->has_default,
                    false,
                );
                continue;
            }
            $this->types->context = $model->fqcn . '::$' . $name;
            $field = new FieldModel(
                $name,
                $this->types->map($prop_storage->type),
                $model,
                $prop_storage,
                $default,
                $has_default,
                (bool) $prop_storage->is_static,
            );
            if ($prop_storage->is_static) {
                $model->static_fields[$name] = $field;
            } else {
                $model->fields[$name] = $field;
            }
        }
        $this->widenFieldsByAssignments($model);
    }

    /**
     * A property whose docblock omits `null` but which the class itself assigns a nullable value
     * (`/** @var Stmt[] *\/ public $stmts;` set from a `?array` parameter) becomes nullable, so that
     * `null` survives instead of turning into a default value.
     */
    private function widenFieldsByAssignments(ClassModel $model): void
    {
        if ($model->node === null) {
            return;
        }
        $finder = new \PhpParser\NodeFinder();
        foreach ($model->node->getMethods() as $mnode) {
            $record = $this->transpiler->getFunctionRecord($mnode, $model->fqcn);
            if ($record === null || $mnode->stmts === null) {
                continue;
            }
            foreach ($finder->findInstanceOf($mnode->stmts, \PhpParser\Node\Expr\Assign::class) as $assign) {
                $target = $assign->var;
                if (!$target instanceof \PhpParser\Node\Expr\PropertyFetch
                    || !$target->var instanceof \PhpParser\Node\Expr\Variable
                    || $target->var->name !== 'this'
                    || !$target->name instanceof \PhpParser\Node\Identifier
                ) {
                    continue;
                }
                $name = $target->name->name;
                $field = $model->fields[$name] ?? null;
                if ($field === null || $field->declaring !== $model || $field->type->kind === RustType::OPTION || $field->type->kind === RustType::MIXED) {
                    continue;
                }
                $assigned = $record->node_data->getType($assign->expr);
                if ($assigned === null || !$assigned->isNullable()) {
                    continue;
                }
                $model->fields[$name] = new FieldModel(
                    $field->name,
                    RustType::option($field->type),
                    $field->declaring,
                    $field->storage,
                    $field->default,
                    $field->has_default,
                    $field->is_static,
                );
            }
        }
    }

    private function buildConstants(ClassModel $model): void
    {
        $exprs = [];
        if ($model->node !== null) {
            foreach ($model->node->getConstants() as $cnode) {
                foreach ($cnode->consts as $c) {
                    $exprs[$c->name->name] = $c->value;
                }
            }
        }
        foreach ($model->storage->constants as $name => $cstorage) {
            if (!isset($exprs[$name]) && $model->is_project) {
                // inherited from an interface/parent; skip, the declaring class emits it
                continue;
            }
            $type = $cstorage->type ?? $cstorage->inferred_type;
            $this->types->context = $model->fqcn . '::' . $name;
            $model->constants[$name] = new ConstModel(
                $name,
                $this->types->map($type),
                $model,
                $cstorage,
                $exprs[$name] ?? null,
            );
        }
    }

    private function buildMethods(ClassModel $model): void
    {
        foreach ($model->storage->declaring_method_ids as $lc_name => $declaring_id) {
            $method = $this->methodFor($model, $lc_name, $declaring_id);
            if ($method !== null) {
                $model->methods[$lc_name] = $method;
            }
        }
        // abstract classes: methods of implemented interfaces are dispatched too
        if (!$model->isConcrete() && !$model->isInterface()) {
            foreach ($model->ancestors as $iface) {
                if (!$iface->isInterface()) {
                    continue;
                }
                foreach ($iface->storage->methods as $lc_name => $_) {
                    if (isset($model->methods[$lc_name])) {
                        continue;
                    }
                    $method = $this->methodFor($model, $lc_name, new MethodIdentifier($iface->fqcn, $lc_name));
                    if ($method !== null) {
                        $model->methods[$lc_name] = $method;
                    }
                }
            }
        }
    }

    private function methodFor(ClassModel $model, string $lc_name, MethodIdentifier $declaring_id): ?MethodModel
    {
        $declaring = $this->getClass($declaring_id->fq_class_name);
        if ($declaring === null) {
            return null;
        }
        $body_owner = $declaring;
        if ($declaring->isTrait()) {
            // trait methods are emitted into the class that uses the trait; its subclasses inherit them
            $body_owner = $model;
            $trait_lc = strtolower($declaring->fqcn);
            for ($c = $model; $c !== null; $c = $c->parent) {
                if (isset($c->storage->used_traits[$trait_lc])) {
                    $body_owner = $c;
                    break;
                }
            }
        }
        $key = $body_owner->lc() . '::' . $lc_name;
        if (isset($this->method_cache[$key])) {
            return $this->method_cache[$key];
        }
        $storage = $declaring->storage->methods[$lc_name] ?? null;
        if ($storage === null) {
            return null;
        }
        $node = null;
        if ($declaring->node !== null) {
            foreach ($declaring->node->getMethods() as $mnode) {
                if (strtolower($mnode->name->name) === $lc_name) {
                    $node = $mnode;
                }
            }
        }
        if ($node === null && $declaring->is_project && !$declaring->isInterface()) {
            // leftover from Psalm's own stub of an overridden builtin class
            return null;
        }
        $record = $node !== null ? $this->transpiler->getFunctionRecord($node, $body_owner->fqcn) : null;
        $method = new MethodModel($storage->cased_name ?? $lc_name, $body_owner, $storage, $node, $record);
        if ($storage->is_static && $node !== null && $node->stmts !== null) {
            $method->uses_lsb = (new \PhpParser\NodeFinder())->findFirst($node->stmts, static function (\PhpParser\Node $n): bool {
                $class = null;
                if ($n instanceof \PhpParser\Node\Expr\New_ || $n instanceof \PhpParser\Node\Expr\StaticCall
                    || $n instanceof \PhpParser\Node\Expr\ClassConstFetch || $n instanceof \PhpParser\Node\Expr\StaticPropertyFetch
                    || $n instanceof \PhpParser\Node\Expr\Instanceof_
                ) {
                    $class = $n->class;
                }
                return $class instanceof \PhpParser\Node\Name && strtolower($class->toString()) === 'static';
            }) !== null;
        }
        $this->method_cache[$key] = $method;
        $saved = $this->types->current_class;
        $saved_crate = $this->types->current_crate;
        $this->types->current_class = $body_owner->fqcn;
        $this->types->current_crate = $body_owner->crate;
        $this->resolveSignature($storage, $method->param_types, $method->return_type, $method->node);
        $this->types->current_class = $saved;
        $this->types->current_crate = $saved_crate;
        // by-reference parameters must keep the type of the root declaration so dispatch signatures agree
        foreach ($storage->params as $i => $p) {
            if (!$p->by_ref) {
                continue;
            }
            foreach ($model->storage->overridden_method_ids[$lc_name] ?? $declaring->storage->overridden_method_ids[$lc_name] ?? [] as $overridden) {
                $parent_cls = $this->getClass($overridden->fq_class_name);
                $pm = $parent_cls !== null ? $this->findMethod($parent_cls, $lc_name) : null;
                if ($pm !== null && isset($pm->param_types[$i])) {
                    $method->param_types[$i] = $pm->param_types[$i];
                    break;
                }
            }
        }
        $this->method_cache[$key] = $method;
        return $method;
    }

    /**
     * A shape-typed parameter whose body reads keys the docblock doesn't declare (`$subNodes['type']`
     * next to `array{flags?: int, ...}`) gets those keys as optional `Mixed` fields, so that call sites
     * passing them keep the values instead of dropping them as unknown.
     */
    private function extendShapeWithBodyReads(RustType $t, string $param_name, \PhpParser\Node\FunctionLike $node): RustType
    {
        $shape = $t->kind === RustType::OPTION ? $t->inner() : $t;
        if ($shape->kind !== RustType::SHAPE) {
            return $t;
        }
        $stmts = $node->getStmts();
        if ($stmts === null) {
            return $t;
        }
        $extra = [];
        foreach ((new \PhpParser\NodeFinder())->findInstanceOf($stmts, \PhpParser\Node\Expr\ArrayDimFetch::class) as $fetch) {
            if (!$fetch->var instanceof \PhpParser\Node\Expr\Variable || $fetch->var->name !== $param_name) {
                continue;
            }
            $key = null;
            if ($fetch->dim instanceof \PhpParser\Node\Scalar\String_) {
                $key = $fetch->dim->value;
            } elseif ($fetch->dim instanceof \PhpParser\Node\Scalar\Int_) {
                $key = (string) $fetch->dim->value;
            }
            if ($key === null || isset($shape->fields[$key]) || isset($extra[$key])) {
                continue;
            }
            $extra[$key] = [RustType::mixed(), true];
        }
        if ($extra === []) {
            return $t;
        }
        $shape = RustType::shape($shape->fields + $extra);
        $this->types->shapes[$shape->mangle()] = $shape;
        return $t->kind === RustType::OPTION ? RustType::option($shape) : $shape;
    }

    /**
     * A shape-typed return whose body returns array literals with keys the docblock doesn't list
     * (`@return array{nodeType: string, ...}` next to `return [..., 'endLine' => ...]`) gets those keys
     * as optional `Mixed` fields so the values survive.
     */
    private function extendShapeWithReturnedKeys(RustType $t, \PhpParser\Node\FunctionLike $node): RustType
    {
        $stmts = $node->getStmts();
        if ($stmts === null) {
            return $t;
        }
        $literals = [];
        foreach ((new \PhpParser\NodeFinder())->findInstanceOf($stmts, \PhpParser\Node\Stmt\Return_::class) as $ret) {
            if ($ret->expr instanceof \PhpParser\Node\Expr\Array_) {
                $literals[] = $ret->expr;
            }
        }
        return $this->extendShapeWithLiterals($t, $literals);
    }

    /**
     * Extend the shape(s) in `$t` with the keys of the array literals `$literals` (data-provider style
     * `array<string, array{...}>` returns are extended one level down, from the nested literals).
     *
     * @param list<\PhpParser\Node\Expr\Array_> $literals
     */
    private function extendShapeWithLiterals(RustType $t, array $literals): RustType
    {
        if ($literals === []) {
            return $t;
        }
        if ($t->kind === RustType::OPTION) {
            $inner = $this->extendShapeWithLiterals($t->inner(), $literals);
            return $inner === $t->inner() ? $t : RustType::option($inner);
        }
        if ($t->kind === RustType::LIST || $t->kind === RustType::MAP) {
            $vt = $t->kind === RustType::LIST ? $t->inner() : $t->params[1];
            $nested = [];
            foreach ($literals as $lit) {
                foreach ($lit->items as $item) {
                    if ($item->value instanceof \PhpParser\Node\Expr\Array_) {
                        $nested[] = $item->value;
                    }
                }
            }
            $new = $this->extendShapeWithLiterals($vt, $nested);
            if ($new === $vt) {
                return $t;
            }
            return $t->kind === RustType::LIST ? RustType::list($new) : RustType::map($t->params[0], $new);
        }
        if ($t->kind !== RustType::SHAPE) {
            return $t;
        }
        $extra = [];
        foreach ($literals as $lit) {
            foreach ($lit->items as $item) {
                $key = null;
                if ($item->key instanceof \PhpParser\Node\Scalar\String_) {
                    $key = $item->key->value;
                } elseif ($item->key instanceof \PhpParser\Node\Scalar\Int_) {
                    $key = (string) $item->key->value;
                }
                if ($key === null || isset($t->fields[$key]) || isset($extra[$key])) {
                    continue;
                }
                $extra[$key] = [RustType::mixed(), true];
            }
        }
        if ($extra === []) {
            return $t;
        }
        $shape = RustType::shape($t->fields + $extra);
        $this->types->shapes[$shape->mangle()] = $shape;
        return $shape;
    }

    /**
     * @param list<RustType> $param_types
     */
    private function resolveSignature(FunctionLikeStorage $storage, array &$param_types, RustType &$return_type, ?\PhpParser\Node $node = null): void
    {
        $param_types = [];
        $fn = ($storage instanceof \Psalm\Storage\MethodStorage && $storage->defining_fqcln !== null ? $storage->defining_fqcln . '::' : '') . ($storage->cased_name ?? '{closure}');
        foreach ($storage->params as $param) {
            $this->types->context = $fn . '() param $' . $param->name;
            $t = $this->types->map($param->type);
            if ($param->is_variadic) {
                $t = RustType::list($t);
            }
            if ($node instanceof \PhpParser\Node\FunctionLike) {
                $t = $this->extendShapeWithBodyReads($t, $param->name, $node);
            }
            $param_types[] = $t;
        }
        $this->types->context = $fn . '() return';
        $return_type = $this->types->map($storage->return_type);
        if ($node instanceof \PhpParser\Node\FunctionLike) {
            $return_type = $this->extendShapeWithReturnedKeys($return_type, $node);
        }
        if ($storage->return_type !== null && $storage->return_type->isVoid()) {
            $return_type = RustType::unit();
        }
        if ($storage->return_type !== null && $storage->return_type->isNever()) {
            $return_type = RustType::never();
        }
        if ($storage->has_yield) {
            $ret = $storage->return_type;
            $key = RustType::mixed();
            $val = RustType::mixed();
            if ($ret !== null) {
                foreach ($ret->getAtomicTypes() as $atomic) {
                    if ($atomic instanceof \Psalm\Type\Atomic\TGenericObject || $atomic instanceof \Psalm\Type\Atomic\TIterable) {
                        $key = $this->types->map($atomic->type_params[0] ?? null);
                        $val = $this->types->map($atomic->type_params[1] ?? null);
                    } elseif ($atomic instanceof \Psalm\Type\Atomic\TArray) {
                        $key = $this->types->map($atomic->type_params[0]);
                        $val = $this->types->map($atomic->type_params[1]);
                    }
                }
            }
            $return_type = RustType::rtGeneric('Generator', [$key, $val]);
        }
    }

    /** Resolve the method reachable as `$lc_name` on `$class`. */
    public function findMethod(ClassModel $class, string $lc_name): ?MethodModel
    {
        if (isset($class->methods[$lc_name])) {
            return $class->methods[$lc_name];
        }
        if (isset($class->storage->declaring_method_ids[$lc_name])) {
            $m = $this->methodFor($class, $lc_name, $class->storage->declaring_method_ids[$lc_name]);
            if ($m !== null) {
                $class->methods[$lc_name] = $m;
            }
            return $m;
        }
        // interfaces: look at parent interfaces
        foreach ($class->ancestors as $ancestor) {
            if (isset($ancestor->methods[$lc_name])) {
                return $ancestor->methods[$lc_name];
            }
        }
        return null;
    }

    public function getConstant(string $name): ?GlobalConstModel
    {
        return $this->constants[strtolower(ltrim($name, '\\'))] ?? null;
    }

    public function getFunction(string $fq_name): ?FunctionModel
    {
        return $this->functions[strtolower(ltrim($fq_name, '\\'))] ?? null;
    }

    /** The class model for a Rust class type. */
    public function classOf(RustType $t): ?ClassModel
    {
        if ($t->kind !== RustType::CLASS_) {
            return null;
        }
        return $this->getClass($t->name);
    }
}
