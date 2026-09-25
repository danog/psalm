<?php

declare(strict_types=1);

namespace Psalm\Tools\IdConvert;

use PhpParser\Comment\Doc;
use PhpParser\Node;
use PhpParser\Node\Expr;
use PhpParser\Node\Stmt;
use Psalm\Codebase;
use Psalm\Internal\MethodIdentifier;
use Psalm\Storage\ClassLikeStorage;
use Psalm\Storage\FunctionLikeStorage;
use Psalm\Type\Union;
use Throwable;

/**
 * Declaration facts of one file: the slots its class-likes and functions declare (properties F, class constants C,
 * parameters P, returns R) with their string paths and the source ranges that spell their types, and the ties
 * between a method and the methods it overrides.
 */
final class Decls
{
    private const MAGIC = ['__tostring', '__get', '__set', '__call', '__callstatic', '__isset', '__unset', '__invoke',
        '__serialize', '__unserialize', '__set_state', '__debuginfo', '__sleep', '__wakeup', 'jsonserialize',
        'offsetget', 'offsetset', 'offsetexists', 'offsetunset', 'getiterator', 'current', 'key', 'serialize',
        'unserialize'];

    private string $src;
    private ?string $self = null;

    public function __construct(private Codebase $codebase, private string $file)
    {
        $this->src = (string) file_get_contents($file);
    }

    /** @param array<Node> $stmts */
    public function file(array $stmts, string $ns = ''): void
    {
        foreach ($stmts as $s) {
            if ($s instanceof Stmt\Namespace_) {
                $this->file($s->stmts, $s->name?->toString() ?? '');
            } elseif ($s instanceof Stmt\ClassLike && $s->name !== null) {
                $this->classLike($s, ltrim($ns . '\\' . $s->name->name, '\\'));
            } elseif ($s instanceof Stmt\Function_) {
                $this->function($s, ltrim($ns . '\\' . $s->name->name, '\\'));
            } elseif ($s instanceof Stmt\If_ || $s instanceof Stmt\Block) {
                // conditional declarations (polyfills)
                $this->file($s instanceof Stmt\If_ ? [...$s->stmts, ...array_merge(...array_map(
                    static fn(Stmt\ElseIf_ $e): array => $e->stmts,
                    $s->elseifs,
                )), ...($s->else?->stmts ?? [])] : $s->stmts, $ns);
            }
        }
    }

    private function classLike(Stmt\ClassLike $node, string $fq): void
    {
        $storage = $this->codebase->classlike_storage_provider->get($fq);
        $cls = strtolower($storage->name);
        $this->self = $storage->name;
        foreach ($node->stmts as $s) {
            if ($s instanceof Stmt\Property) {
                foreach ($s->props as $p) {
                    $name = $p->name->name;
                    $ps = $storage->properties[$name] ?? null;
                    $slot = 'F:' . $cls . '::$' . $name;
                    $this->slot($slot, $ps?->type, [
                        ...$this->native($s->type),
                        ...$this->docTags($s->getDocComment(), ['@var', '@psalm-var', '@phpstan-var'], null),
                    ], ['static' => $s->isStatic(), 'vis' => $this->vis($s->flags), 'cls' => $cls, 'member' => $name]);
                    if ($p->default !== null) {
                        $this->defaults[] = [$slot, $p->default, $this->self];
                    }
                }
            } elseif ($s instanceof Stmt\ClassConst) {
                foreach ($s->consts as $c) {
                    $name = $c->name->name;
                    $cs = $storage->constants[$name] ?? null;
                    $slot = 'C:' . $cls . '::' . $name;
                    $this->slot($slot, $cs?->type ?? $cs?->inferredType ?? null, [
                        ...$this->native($s->type),
                        ...$this->docTags($s->getDocComment(), ['@var', '@psalm-var'], null),
                    ], ['cls' => $cls, 'member' => $name, 'vis' => $this->vis($s->flags)]);
                    $this->defaults[] = [$slot, $c->value, $this->self];
                }
            } elseif ($s instanceof Stmt\ClassMethod) {
                $lc = strtolower($s->name->name);
                $ms = $storage->methods[$lc] ?? null;
                if ($ms === null) {
                    continue;
                }
                $this->signature($s, $ms, $cls . '::' . $lc, $cls, $storage, $lc);
            }
        }
        // a class using traits: the trait's method signatures are the class's too (Psalm copies them)
        foreach ($storage->overridden_method_ids as $lc => $by_class) {
            $decl = $storage->declaring_method_ids[$lc] ?? null;
            if ($decl === null) {
                continue;
            }
            foreach ($by_class as $mid) {
                if (strtolower($mid->fq_class_name) !== strtolower($decl->fq_class_name)) {
                    $this->tieMethods($decl, $mid, (string) $lc);
                }
            }
        }
    }

    /** @var list<array{string, Expr, ?string}> [slot, default value, class]: walked by the caller with the slot as demand */
    public array $defaults = [];

    private function function(Stmt\Function_ $node, string $fq): void
    {
        $lc = strtolower($fq);
        $this->self = null;
        try {
            $fs = $this->codebase->functions->getStorage(null, $lc);
        } catch (Throwable) {
            return;
        }
        $this->signature($node, $fs, $lc, null, null, $lc);
    }

    private function signature(
        Stmt\ClassMethod|Stmt\Function_ $node,
        FunctionLikeStorage $fs,
        string $fn,
        ?string $cls,
        ?ClassLikeStorage $storage,
        string $lc,
    ): void {
        $doc = $node->getDocComment();
        $stop = null;
        if ($cls !== null && in_array($lc, self::MAGIC, true)) {
            $stop = 'magic';
        }
        foreach ($node->params as $i => $p) {
            if (!$p->var instanceof Expr\Variable || !is_string($p->var->name)) {
                continue;
            }
            $name = $p->var->name;
            $param = $fs->params[$i] ?? null;
            $slot = 'P:' . $fn . '|' . $name;
            $extra = ['fn' => $fn, 'idx' => $i, 'member' => $name];
            if ($p->byRef) {
                $extra['byref'] = true;
            }
            if ($p->variadic) {
                $extra['variadic'] = true;
            }
            $this->slot($slot, $this->inheritedParamType($param, $storage, $lc, $i), [
                ...$this->native($p->type),
                ...$this->docTags($doc, ['@param', '@psalm-param', '@phpstan-param'], $name),
                ...$this->docTags($p->getDocComment(), ['@var'], null),
            ], $extra + ($stop !== null ? ['stop' => $stop] : []));
            if ($p->default !== null) {
                $this->defaults[] = [$slot, $p->default, $this->self];
            }
            if ($p->flags !== 0 && $cls !== null && $lc === '__construct') {
                // a promoted parameter is its property
                $prop = 'F:' . $cls . '::$' . $name;
                $ps = $storage?->properties[$name] ?? null;
                $this->slot($prop, $ps?->type, [], ['cls' => $cls, 'member' => $name, 'vis' => $this->vis($p->flags)]);
                ConvertPlugin::out(['k' => 'tie', 'a' => $slot, 'b' => $prop, 'why' => 'promoted']);
            }
        }
        $rtype = $fs->return_type;
        if ($storage !== null && $fs instanceof \Psalm\Storage\MethodStorage && !$fs->has_docblock_return_type) {
            foreach ($storage->overridden_method_ids[$lc] ?? [] as $mid) {
                try {
                    $ps = $this->codebase->methods->getStorage($mid);
                } catch (Throwable) {
                    continue;
                }
                if ($ps->has_docblock_return_type && $ps->return_type !== null) {
                    // the documented type of the overridden method holds here too
                    $rtype = $ps->return_type;
                    break;
                }
            }
        }
        $this->slot('R:' . $fn, $rtype, [
            ...$this->native($node->returnType),
            ...$this->docTags($doc, ['@return', '@psalm-return', '@phpstan-return'], null),
        ], ['fn' => $fn] + ($stop !== null ? ['stop' => $stop] : []));
    }

    /** A parameter without a docblock type has the documented type of the parameter it overrides. */
    private function inheritedParamType(
        ?\Psalm\Storage\FunctionLikeParameter $param,
        ?ClassLikeStorage $storage,
        string $lc,
        int $i,
    ): ?Union {
        if ($param === null || $param->has_docblock_type || $storage === null) {
            return $param?->type;
        }
        foreach ($storage->overridden_method_ids[$lc] ?? [] as $mid) {
            try {
                $pp = $this->codebase->methods->getStorage($mid)->params[$i] ?? null;
            } catch (Throwable) {
                continue;
            }
            if ($pp !== null && $pp->has_docblock_type && $pp->type !== null) {
                return $pp->type;
            }
        }
        return $param->type;
    }

    /** Ties a method's signature slots to an overridden method's (by parameter position), or stops them. */
    private function tieMethods(MethodIdentifier $decl, MethodIdentifier $parent, string $lc): void
    {
        $fn = strtolower($decl->fq_class_name) . '::' . $lc;
        try {
            $own = $this->codebase->methods->getStorage($decl);
            $ps = $this->codebase->methods->getStorage($parent);
            $pcls = $this->codebase->classlike_storage_provider->get($parent->fq_class_name);
        } catch (Throwable) {
            ConvertPlugin::out(['k' => 'stop', 's' => 'R:' . $fn, 'why' => 'unknown-parent', 'fnall' => $fn]);
            return;
        }
        $pfn = strtolower($pcls->name) . '::' . $lc;
        $inproj = $pcls->location !== null && ConvertPlugin::inProject($pcls->location->file_path)
            && !$pcls->stubbed;
        if (!$inproj) {
            ConvertPlugin::out(['k' => 'stop', 's' => 'R:' . $fn, 'why' => 'overrides-external', 'fnall' => $fn]);
            return;
        }
        ConvertPlugin::out(['k' => 'tie', 'a' => 'R:' . $fn, 'b' => 'R:' . $pfn, 'why' => 'override', 'paths' => true]);
        foreach ($own->params as $i => $p) {
            $pp = $ps->params[$i] ?? null;
            if ($pp === null) {
                continue;
            }
            ConvertPlugin::out(['k' => 'tie', 'a' => 'P:' . $fn . '|' . $p->name, 'b' => 'P:' . $pfn . '|' . $pp->name,
                'why' => 'override', 'paths' => true]);
        }
    }

    /**
     * @param list<array<string, mixed>> $decl
     * @param array<string, mixed> $extra
     */
    private function slot(string $slot, ?Union $type, array $decl, array $extra = []): void
    {
        $paths = Types::paths($type);
        if ($paths === [] && !isset($extra['stop'])) {
            return;
        }
        ConvertPlugin::out(['k' => 'slot', 's' => $slot, 'file' => $this->file, 'paths' => $paths,
            'classy' => Types::classy($type), 'lc' => Types::lc($type), 'tpl' => Types::templated($type),
            'decl' => $decl] + $extra);
    }

    /** @return list<array<string, mixed>> */
    private function native(?Node $type): array
    {
        if ($type === null) {
            return [];
        }
        return [['t' => 'type', 'r' => [$type->getStartFilePos(), $type->getEndFilePos() + 1],
            'text' => substr($this->src, $type->getStartFilePos(), $type->getEndFilePos() + 1 - $type->getStartFilePos())]];
    }

    /**
     * The type strings of a docblock's tags (for `@param`, the one naming `$var`).
     *
     * @param list<string> $tags
     * @return list<array<string, mixed>>
     */
    private function docTags(?Doc $doc, array $tags, ?string $var): array
    {
        if ($doc === null) {
            return [];
        }
        return self::docTypes($doc->getText(), $doc->getStartFilePos(), $tags, $var);
    }

    /**
     * @param list<string> $tags
     * @return list<array<string, mixed>>
     */
    public static function docTypes(string $text, int $base, array $tags, ?string $var): array
    {
        $out = [];
        $re = '/(' . implode('|', array_map(static fn(string $t): string => preg_quote($t, '/'), $tags)) . ')(?![\w-])[ \t]+/';
        if (!preg_match_all($re, $text, $m, PREG_OFFSET_CAPTURE)) {
            return [];
        }
        foreach ($m[0] as [$whole, $at]) {
            $start = $at + strlen($whole);
            // a type spans lines only inside brackets; join `*` continuation lines for the parser
            $rest = substr($text, $start, 2000);
            $flat = preg_replace_callback('/\n\s*\*/', static fn(array $x): string => str_repeat(' ', strlen($x[0])), $rest);
            if ($flat === null || $flat === '' || $flat[0] === '$') {
                continue;
            }
            try {
                $u = TypeStr::parse($flat);
            } catch (Throwable) {
                continue;
            }
            $end = $u['end'];
            $after = ltrim(substr($flat, $end, 200));
            if ($var !== null) {
                if (!preg_match('/^(?:&\s*)?(?:\.\.\.\s*)?\$(\w+)/', $after, $vm) || $vm[1] !== $var) {
                    continue;
                }
            } elseif ($tags[0] === '@var' && preg_match('/^\$\w+/', $after)) {
                // an inline `@var T $x` names a local: only the property / constant form (no variable) here
                continue;
            }
            $out[] = ['t' => 'doc', 'r' => [$base + $start, $base + $start + $end], 'text' => substr($rest, 0, $end)];
        }
        return $out;
    }

    private function vis(int $flags): string
    {
        return ($flags & Stmt\Class_::MODIFIER_PRIVATE) ? 'private'
            : (($flags & Stmt\Class_::MODIFIER_PROTECTED) ? 'protected' : 'public');
    }
}
