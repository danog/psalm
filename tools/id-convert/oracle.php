<?php

/**
 * oracle.php --root DIR --facts FILE --repo DIR --ref REF > seeds.txt
 *
 * Seeds from an earlier conversion: every declared slot path (property, parameter, return) whose type in REF's
 * version of the same declaration is `int` where the facts' tree has a string.
 */

declare(strict_types=1);

namespace Psalm\Tools\IdConvert;

use PhpParser\Node;
use PhpParser\Node\Stmt;
use PhpParser\NodeFinder;
use PhpParser\ParserFactory;

require_once __DIR__ . '/TypeStr.php';

$opts = getopt('', ['root:', 'facts:', 'repo:', 'ref:']);
$root = rtrim((string) $opts['root'], '/') . '/';
require $root . 'vendor/autoload.php';
$repo = (string) $opts['repo'];
$ref = (string) $opts['ref'];

$slots = [];
foreach (file((string) $opts['facts']) as $line) {
    $r = json_decode($line, true);
    if (($r['k'] ?? '') === 'slot' && preg_match('/^[FPR]:/', $r['s']) && ($r['paths'] ?? []) !== []) {
        $slots[$r['s']] = $r;
    }
}

$parser = (new ParserFactory())->createForHostVersion();
$finder = new NodeFinder();
/** @var array<string, array<string, list<string>>> rel file => slot => the type texts there */
$fork = [];

$docType = static function (?string $doc, string $tag, ?string $var): ?string {
    if ($doc === null) {
        return null;
    }
    $best = null;
    foreach (['@psalm-' . $tag, '@' . $tag] as $t) {
        if (!preg_match_all('/' . preg_quote($t, '/') . '\s+/', $doc, $m, PREG_OFFSET_CAPTURE)) {
            continue;
        }
        foreach ($m[0] as [$whole, $at]) {
            $rest = substr($doc, $at + strlen($whole), 2000);
            $flat = (string) preg_replace_callback('/\n\s*\*/', static fn(array $x): string => str_repeat(' ', strlen($x[0])), $rest);
            try {
                $u = TypeStr::parse($flat);
            } catch (\Throwable) {
                continue;
            }
            $after = ltrim(substr($flat, $u['end'], 200));
            if ($var !== null && (!preg_match('/^(?:&\s*)?(?:\.\.\.\s*)?\$(\w+)/', $after, $vm) || $vm[1] !== $var)) {
                continue;
            }
            return substr($flat, 0, $u['end']);
        }
    }
    return $best;
};

$typeText = static function (?Node $t) use (&$typeText): ?string {
    if ($t === null) {
        return null;
    }
    if ($t instanceof Node\NullableType) {
        return '?' . $typeText($t->type);
    }
    if ($t instanceof Node\UnionType || $t instanceof Node\IntersectionType) {
        return implode($t instanceof Node\UnionType ? '|' : '&', array_map($typeText, $t->types));
    }
    return $t instanceof Node\Identifier || $t instanceof Node\Name ? $t->toString() : null;
};

$load = function (string $rel) use (&$fork, $repo, $ref, $parser, $finder, $docType, $typeText): array {
    if (isset($fork[$rel])) {
        return $fork[$rel];
    }
    $code = shell_exec('git -C ' . escapeshellarg($repo) . ' show ' . escapeshellarg($ref . ':' . $rel) . ' 2>/dev/null');
    $out = [];
    if (!is_string($code) || $code === '') {
        return $fork[$rel] = $out;
    }
    try {
        $stmts = $parser->parse($code) ?? [];
    } catch (\Throwable) {
        return $fork[$rel] = $out;
    }
    $ns = '';
    foreach ($finder->find($stmts, static fn(Node $n): bool => $n instanceof Stmt\Namespace_ || $n instanceof Stmt\ClassLike) as $n) {
        if ($n instanceof Stmt\Namespace_) {
            $ns = $n->name?->toString() ?? '';
            foreach ($n->stmts as $c) {
                if ($c instanceof Stmt\ClassLike && $c->name !== null) {
                    $cls = strtolower(ltrim($ns . '\\' . $c->name->name, '\\'));
                    foreach ($c->stmts as $m) {
                        if ($m instanceof Stmt\Property) {
                            foreach ($m->props as $p) {
                                $out['F:' . $cls . '::$' . $p->name->name] = array_filter([
                                    $docType($m->getDocComment()?->getText(), 'var', null), $typeText($m->type)]);
                            }
                        } elseif ($m instanceof Stmt\ClassMethod) {
                            $fn = $cls . '::' . strtolower($m->name->name);
                            $doc = $m->getDocComment()?->getText();
                            $out['R:' . $fn] = array_filter([$docType($doc, 'return', null), $typeText($m->returnType)]);
                            foreach ($m->params as $p) {
                                if ($p->var instanceof Node\Expr\Variable && is_string($p->var->name)) {
                                    $out['P:' . $fn . '|' . $p->var->name] = array_filter([
                                        $docType($doc, 'param', $p->var->name), $typeText($p->type)]);
                                    if ($p->flags !== 0 && strtolower($m->name->name) === '__construct') {
                                        $out['F:' . $cls . '::$' . $p->var->name] = $out['P:' . $fn . '|' . $p->var->name];
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return $fork[$rel] = $out;
};

$isInt = static function (array $u, string $path): bool {
    try {
        $targets = TypeStr::at($u, $path);
    } catch (\Throwable) {
        return false;
    }
    if ($targets === []) {
        return false;
    }
    foreach ($targets as $t) {
        $int = false;
        foreach ($t['atoms'] as $a) {
            $name = strtolower((string) ($a['name'] ?? ''));
            if ($name === 'null') {
                continue;
            }
            if ($a['kind'] !== 'name' || !in_array($name, ['int', 'positive-int', 'non-negative-int'], true)) {
                return false;
            }
            $int = true;
        }
        if (!$int) {
            return false;
        }
    }
    return true;
};

foreach ($slots as $slot => $r) {
    $rel = substr((string) $r['file'], strlen($root));
    $texts = $load($rel)[$slot] ?? [];
    foreach ($r['paths'] as $path => $_) {
        foreach ($texts as $text) {
            try {
                $u = TypeStr::parse($text);
            } catch (\Throwable) {
                continue;
            }
            if ($isInt($u, $path)) {
                echo $slot . $path, "\n";
                break;
            }
        }
    }
}
