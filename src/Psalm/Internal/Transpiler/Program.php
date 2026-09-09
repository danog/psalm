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

    /** @var array<string, array{ClassModel, int}> `new $name(...)` factories needed, keyed by class and arity */
    public array $factories = [];

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
        }

        fwrite(STDERR, "[program] hierarchy\n");
        // 2. hierarchy (also pulls in external ancestors)
        foreach (array_keys($this->classes) as $lc) {
            $model = $this->classes[$lc];
            $this->linkAncestors($model);
        }

        foreach ($this->uniqueClasses() as $model) {
            foreach ($model->ancestors as $ancestor) {
                if ($model->isConcrete() && $model->is_project) {
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
            $this->buildFields($model);
            $this->buildConstants($model);
        }
        $this->types->current_class = null;
        fwrite(STDERR, "[program] methods\n");
        foreach ($this->uniqueClasses() as $model) {
            $this->buildMethods($model);
        }
        foreach ($this->functions as $fn) {
            $this->resolveSignature($fn->record->storage, $fn->param_types, $fn->return_type, $fn->record->node);
        }
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
        $storage = $model->storage;
        foreach ($storage->properties as $name => $prop_storage) {
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
                foreach ($node_class->node->getProperties() as $pnode) {
                    foreach ($pnode->props as $item) {
                        if ($item->name->name === $name) {
                            $default = $item->default;
                            $has_default = $has_default || $item->default !== null;
                        }
                    }
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
            // trait methods are emitted into every using class
            $body_owner = $model;
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
        $this->types->current_class = $body_owner->fqcn;
        $this->resolveSignature($storage, $method->param_types, $method->return_type, $method->node);
        $this->types->current_class = $saved;
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
        $shape = $t->kind === RustType::OPTION ? $t->inner() : $t;
        if ($shape->kind !== RustType::SHAPE) {
            return $t;
        }
        $stmts = $node->getStmts();
        if ($stmts === null) {
            return $t;
        }
        $extra = [];
        foreach ((new \PhpParser\NodeFinder())->findInstanceOf($stmts, \PhpParser\Node\Stmt\Return_::class) as $ret) {
            if (!$ret->expr instanceof \PhpParser\Node\Expr\Array_) {
                continue;
            }
            foreach ($ret->expr->items as $item) {
                $key = null;
                if ($item->key instanceof \PhpParser\Node\Scalar\String_) {
                    $key = $item->key->value;
                } elseif ($item->key instanceof \PhpParser\Node\Scalar\Int_) {
                    $key = (string) $item->key->value;
                }
                if ($key === null || isset($shape->fields[$key]) || isset($extra[$key])) {
                    continue;
                }
                $extra[$key] = [RustType::mixed(), true];
            }
        }
        if ($extra === []) {
            return $t;
        }
        $shape = RustType::shape($shape->fields + $extra);
        $this->types->shapes[$shape->mangle()] = $shape;
        return $t->kind === RustType::OPTION ? RustType::option($shape) : $shape;
    }

    /**
     * @param list<RustType> $param_types
     */
    private function resolveSignature(FunctionLikeStorage $storage, array &$param_types, RustType &$return_type, ?\PhpParser\Node $node = null): void
    {
        $param_types = [];
        foreach ($storage->params as $param) {
            $t = $this->types->map($param->type);
            if ($param->is_variadic) {
                $t = RustType::list($t);
            }
            if ($node instanceof \PhpParser\Node\FunctionLike) {
                $t = $this->extendShapeWithBodyReads($t, $param->name, $node);
            }
            $param_types[] = $t;
        }
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

    public function needFactory(ClassModel $cls, int $arity): void
    {
        $this->factories[$cls->lc() . '/' . $arity] = [$cls, $arity];
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
