<?php

/**
 * textual.php ROOT: names become interned ids, by plain token rewriting (no analysis):
 *
 *  1. typehints: `string` / `?string` / `string|null` of parameters, properties and returns that are names
 *     become `int`; in docblocks, their string types (and every `lowercase-string`) become `int`;
 *  2. `strtolower(<name>)` becomes `<name>` (names are case-sensitive ids, as in pzoom);
 *  3. string literals and `X::class` compared with a name (===, !==, ==, !=, case, match arms, in_array lists)
 *     become `Sym::` constants (src) / `Interner::intern(...)` (tests).
 *
 * A name is a variable, property or parameter whose name says so (fq_class_name, method_name_lc, parent_class...,
 * plus the fields listed in NAME_FIELDS), or the return of a getter named so (getFQCLN, getMethodName...).
 * Then Interner.php / Sym.php are added. What breaks is found by running Psalm on the result.
 */

declare(strict_types=1);

require_once __DIR__ . '/names.php';
require_once __DIR__ . '/TypeStr.php';

$root = rtrim($argv[1] ?? '', '/') . '/';
if ($root === '/') {
    fwrite(STDERR, "usage: textual.php ROOT\n");
    exit(2);
}

const NAME_WORDS = ['class', 'classes', 'classlike', 'classlikes', 'fqcln', 'fqcn', 'interface', 'interfaces',
    'trait', 'traits', 'enum', 'method', 'methods', 'function', 'functions', 'property', 'properties', 'prop',
    'props', 'constant', 'constants', 'const', 'parent', 'parents', 'mixin', 'self'];
const NOT_NAME_WORDS = ['id', 'ids', 'file', 'files', 'path', 'paths', 'dir', 'message', 'text', 'code', 'doc',
    'docblock', 'description', 'var', 'vars', 'key', 'keys', 'hash', 'snippet', 'content', 'contents', 'string',
    'str', 'prefix', 'suffix', 'pattern', 'regex', 'format', 'type', 'types', 'version', 'storage', 'storages',
    'node', 'nodes', 'stmt', 'stmts', 'analyzer', 'provider', 'location', 'map', 'data', 'info', 'params', 'param',
    'exists', 'count', 'kind', 'visibility', 'flags', 'body', 'template', 'templates', 'signature', 'tokens', 'fake'];

/** Fields whose names do not say they hold a name. */
const NAME_FIELDS = ['value' => ['TNamedObject', 'TGenericObject', 'TClosure', 'TLiteralClassString', 'TIterable'], 'name' => ['ClassLikeStorage'],
    'defining_class' => ['TTemplateParam', 'TTemplateParamClass', 'TTemplateKeyOf', 'TTemplateValueOf'],
    'cased_name' => ['FunctionLikeStorage', 'MethodStorage', 'FunctionStorage']];

function words(string $name): array
{
    $name = (string) preg_replace('/(?<=[a-z0-9])(?=[A-Z])/', '_', ltrim($name, '$'));
    return array_values(array_filter(explode('_', strtolower($name)), static fn(string $w): bool => $w !== ''));
}

function isName(string $name): bool
{
    $w = words($name);
    if (array_intersect($w, NOT_NAME_WORDS) !== [] || array_intersect($w, NAME_WORDS) === []) {
        return false;
    }
    $last = end($w);
    return in_array($last, [...NAME_WORDS, 'name', 'names', 'lc', 'fqcln'], true);
}

/** A getter whose name says it returns a name: getFQCLN, getClassName, getParentClass... */
function isNameGetter(string $fn): bool
{
    return (bool) preg_match('/^get(FQCLN|ParentFQCLN|ClassName|ParentClass|MethodName|FunctionName|PropertyName|ConstantName|CasedName|DeclaringClass)$/i', $fn);
}

final class Rewriter
{
    /** @var array<string, true> variables assigned a stripped `strtolower(<name>)` (the declared spelling now) */
    private array $lowered = [];
    /** literal() canonicalizes the spelling instead of making an id */
    private bool $canon = false;
    /** @var array<string, string> literal => Sym constant */
    public static array $sym = [];
    /** @var array<string, true> properties whose keys became ids */
    public static array $id_keyed = [];

    private array $t;
    private string $cls = '';
    private int $magic_until = -1;
    /** @var array<string, true> variables a docblock types as lowercase names (their native hints follow) */
    private array $forced = [];
    /** @var array<string, true> variables a docblock types as class references (they stay strings) */
    private array $keep = [];
    private string $fn_name = '';
    public int $edits = 0;

    public function __construct(private string $src, private bool $test)
    {
        $this->t = token_get_all($src);
        // class references: variables called on (`$c::m()`) or instantiated (`new $c`) stay strings
        for ($i = 0; $i < count($this->t); $i++) {
            if ($this->id($i) !== T_VARIABLE) {
                continue;
            }
            $n = $this->next($i);
            $p = $this->next($i, -1);
            if (($n >= 0 && $this->text($n) === '::') || ($p >= 0 && $this->id($p) === T_NEW)) {
                $this->keep[substr($this->text($i), 1)] = true;
            }
            // the class argument of a reflection builtin: a class reference (its narrowing types the analysis)
            if ($p >= 0 && $this->text($p) === '(') {
                $f = $this->next($p, -1);
                if ($f >= 0 && in_array(strtolower(ltrim($this->text($f), '\\')), ['is_subclass_of', 'class_exists',
                    'interface_exists', 'trait_exists', 'enum_exists', 'method_exists', 'property_exists', 'is_a',
                    'get_parent_class', 'class_implements', 'reflectionclass'], true)
                ) {
                    $this->keep[substr($this->text($i), 1)] = true;
                }
            }
        }
    }

    private function text(int $i): string
    {
        return is_array($this->t[$i]) ? $this->t[$i][1] : $this->t[$i];
    }

    private function id(int $i): int
    {
        return is_array($this->t[$i]) ? $this->t[$i][0] : 0;
    }

    private function set(int $i, string $text): void
    {
        $this->t[$i] = [is_array($this->t[$i]) ? $this->t[$i][0] : T_STRING, $text, 0];
        $this->edits++;
    }

    /** the next / previous significant token */
    private function next(int $i, int $dir = 1): int
    {
        for ($j = $i + $dir; $j >= 0 && $j < count($this->t); $j += $dir) {
            if (!in_array($this->id($j), [T_WHITESPACE, T_COMMENT, T_DOC_COMMENT], true)) {
                return $j;
            }
        }
        return -1;
    }

    /** Whether the expression ending at token $i (a variable or a property fetch) is a name. */
    private function nameExprAt(int $i): bool
    {
        $id = $this->id($i);
        $t = $this->text($i);
        if ($id === T_VARIABLE) {
            return isName($t);
        }
        if ($id === T_STRING) {
            $p = $this->next($i, -1);
            if ($p >= 0 && in_array($this->text($p), ['->', '?->', '::'], true)) {
                if (isName($t)) {
                    return true;
                }
                // a field named only by its class (`value` of TNamedObject): `$this->value` inside those classes
                $recv = $this->next($p, -1);
                if (isset(NAME_FIELDS[$t]) && $recv >= 0 && $this->text($recv) === '$this'
                    && in_array($this->cls, NAME_FIELDS[$t], true)
                ) {
                    return true;
                }
            }
        }
        if ($t === ')') {
            // a name getter call: ...->getFQCLN()
            $o = $this->next($i, -1);
            if ($o >= 0 && $this->text($o) === '(') {
                $f = $this->next($o, -1);
                return $f >= 0 && isNameGetter($this->text($f));
            }
        }
        return false;
    }

    /** Whether the expression ending at token $i is a call returning a declared class-like name. */
    private function classNameCallAt(int $i): bool
    {
        if ($this->text($i) !== ')') {
            return false;
        }
        $f = $this->next($this->matchBack($i), -1);
        return $f >= 0 && in_array($this->text($f), ['getUnAliasedName', 'getFQCLNFromNameObject'], true);
    }

    /** The start token of the variable / fetch chain ending at $i. */
    private function exprStart(int $i): int
    {
        $j = $i;
        while (true) {
            $p = $this->next($j, -1);
            if ($p < 0) {
                return $j;
            }
            if (in_array($this->text($p), ['->', '?->', '::'], true)) {
                $q = $this->next($p, -1);
                if ($q < 0) {
                    return $j;
                }
                if ($this->text($q) === ')') {
                    $q = $this->matchBack($q);
                    $f = $this->next($q, -1);
                    $j = $f >= 0 && in_array($this->id($f), [T_STRING, T_VARIABLE], true) ? $f : $q;
                    continue;
                }
                $j = $q;
                continue;
            }
            return $j;
        }
    }

    private function matchBack(int $i): int
    {
        $depth = 0;
        for ($j = $i; $j >= 0; $j--) {
            $t = $this->text($j);
            if ($t === ')' || $t === ']') {
                $depth++;
            } elseif ($t === '(' || $t === '[') {
                $depth--;
                if ($depth === 0) {
                    return $j;
                }
            }
        }
        return $i;
    }

    private function matchForward(int $i): int
    {
        $depth = 0;
        for ($j = $i; $j < count($this->t); $j++) {
            $t = $this->text($j);
            if ($t === '(' || $t === '[' || $t === '{' || $this->id($j) === T_CURLY_OPEN || $this->id($j) === T_DOLLAR_OPEN_CURLY_BRACES) {
                $depth++;
            } elseif ($t === ')' || $t === ']' || $t === '}') {
                $depth--;
                if ($depth === 0) {
                    return $j;
                }
            }
        }
        return $i;
    }

    public function run(): string
    {
        $n = count($this->t);
        for ($i = 0; $i < $n; $i++) {
            $id = $this->id($i);
            $t = $this->text($i);
            if ($id === T_FUNCTION) {
                // a magic method's signature is PHP's
                $f = $this->next($i);
                if ($f >= 0 && str_starts_with($this->text($f), '__') && strtolower($this->text($f)) !== '__construct') {
                    $open = $this->next($f);
                    $this->magic_until = $open >= 0 ? $this->matchForward($open) : $i;
                    $colon = $this->next($this->magic_until);
                    if ($colon >= 0 && $this->text($colon) === ':') {
                        $this->magic_until = $this->next($this->next($colon));
                    }
                }
            }
            if ($i <= $this->magic_until) {
                continue;
            }
            if (in_array($id, [T_CLASS, T_TRAIT, T_INTERFACE, T_ENUM], true)) {
                $c = $this->next($i);
                if ($c >= 0 && $this->id($c) === T_STRING) {
                    $this->cls = $this->text($c);
                }
            }
            if ($id === T_DOC_COMMENT) {
                $this->docComment($i);
                continue;
            }
            // 1. native hints of name parameters / properties
            if ($id === T_STRING && strtolower($t) === 'string') {
                $this->nativeHint($i);
                continue;
            }
            // 2. strtolower(<name>)
            if ($id === T_STRING && strtolower($t) === 'strtolower') {
                $this->stripLower($i);
                continue;
            }
            // 3. literals compared with names
            if (in_array($id, [T_IS_IDENTICAL, T_IS_NOT_IDENTICAL, T_IS_EQUAL, T_IS_NOT_EQUAL], true)) {
                $l = $this->next($i, -1);
                $r = $this->next($i);
                if ($l >= 0 && $this->nameExprAt($l)) {
                    $this->literal($r);
                } elseif ($l >= 0 && isset($this->lowered[$this->text($l)])) {
                    $this->canon = true;
                    $this->literal($r);
                    $this->canon = false;
                } elseif ($r >= 0) {
                    $end = $this->exprEnd($r);
                    if ($end !== null && $this->nameExprAt($end)) {
                        $this->literal($l);
                    } elseif ($end !== null && isset($this->lowered[$this->text($end)])) {
                        $this->canon = true;
                        $this->literal($l);
                        $this->canon = false;
                    }
                }
                continue;
            }
            // `(string) <name>` (null as ''): the empty name's id is 0
            if ($id === T_STRING_CAST) {
                $first = $this->next($i);
                $end = $first >= 0 ? $this->exprEnd($first) : null;
                $last_word = strtolower((string) preg_replace('/.*_/', '', $this->text($end ?? $i)));
                // a getter or a property (a bare local's type is not known textually: a key, an XML node...)
                if ($end !== null && $this->nameExprAt($end)
                    && ($this->text($end) === ')' || (preg_match('/(?:name|fqcln|class|self)$/i', $this->text($end))
                        && $this->text($this->next($end, -1)) === '->'))
                ) {
                    // the parentheses go into the expression's own tokens (a bare `(` / `)` token would unbalance
                    // the bracket matching of the rest of the file)
                    $this->set($i, '');
                    $this->set($first, '(' . $this->text($first));
                    $this->set($end, $this->text($end) . ' ?? 0)');
                }
                continue;
            }
            // a name concatenated / interpolated: its string
            if ($t === '.') {
                $this->concatOperands($i);
                continue;
            }
            if ($t === '"') {
                $i = $this->interpolation($i);
                continue;
            }
            if ($id === T_SWITCH || $id === T_MATCH) {
                $this->switchOrMatch($i, $id === T_MATCH);
                $this->canon = false;
                continue;
            }
            if ($id === T_STRING && strtolower($t) === 'in_array') {
                $this->inArray($i);
                $this->canon = false;
            }
        }
        $out = '';
        foreach ($this->t as $tok) {
            $out .= is_array($tok) ? $tok[1] : $tok;
        }
        return $out;
    }

    /** The last token of a simple expression (variable / fetch chain / getter call) starting at $i. */
    private function exprEnd(int $i): ?int
    {
        if (!in_array($this->id($i), [T_VARIABLE, T_STRING], true)) {
            return null;
        }
        $j = $i;
        while (true) {
            $n = $this->next($j);
            if ($n < 0) {
                return $j;
            }
            $tx = $this->text($n);
            if (($tx === '(' && $this->id($j) === T_STRING) || $tx === '[') {
                $j = $this->matchForward($n);
                continue;
            }
            if (in_array($tx, ['->', '?->', '::'], true)) {
                $m = $this->next($n);
                if ($m < 0 || !in_array($this->id($m), [T_STRING, T_VARIABLE], true)) {
                    return $j;
                }
                $j = $m;
                continue;
            }
            return $j;
        }
    }

    private function nativeHint(int $i): void
    {
        // `?string $x`, `string|null $x`, `string $x`: a parameter or a typed property named as a name
        $n = $this->next($i);
        $var = $n;
        if ($n >= 0 && $this->text($n) === '|') {
            $m = $this->next($n);
            if ($m >= 0 && strtolower($this->text($m)) === 'null') {
                $var = $this->next($m);
            }
        }
        if ($var >= 0 && $this->id($var) === T_VARIABLE) {
            $name = substr($this->text($var), 1);
            if (isset($this->keep[$name])) {
                return;
            }
            if (isName($name) || isset($this->forced[$name])
                || (isset(NAME_FIELDS[$name]) && in_array($this->cls, NAME_FIELDS[$name], true))
            ) {
                $this->set($i, 'int');
                // its default: the name's id ('' is 0)
                $eq = $this->next($var);
                $dv = $eq >= 0 && $this->text($eq) === '=' ? $this->next($eq) : -1;
                if ($dv >= 0 && $this->id($dv) === T_CONSTANT_ENCAPSED_STRING) {
                    $lit = stripcslashes(substr($this->text($dv), 1, -1));
                    // a default is a constant expression: the Sym constant even in tests
                    $was = $this->test;
                    $this->test = false;
                    $this->set($dv, $lit === '' ? '0' : $this->symText(ltrim($lit, '\\')));
                    $this->test = $was;
                }
            }
            return;
        }
        // a return type: `function getFQCLN(): string`
        $p = $this->next($i, -1);
        if ($p >= 0 && $this->text($p) === '?') {
            $p = $this->next($p, -1);
        }
        if ($p >= 0 && $this->text($p) === ':') {
            $close = $this->next($p, -1);
            if ($close >= 0 && $this->text($close) === ')') {
                $open = $this->matchBack($close);
                $fn = $this->next($open, -1);
                if ($fn >= 0 && isNameGetter($this->text($fn))) {
                    $this->set($i, 'int');
                }
            }
        }
    }

    private function concatOperands(int $i): void
    {
        $l = $this->next($i, -1);
        if ($l >= 0 && $this->nameExprAt($l) && !$this->wrapped($l)) {
            $start = $this->exprStart($l);
            $this->set($start, 'Interner::lookup(' . $this->text($start));
            $this->set($l, $this->text($l) . ')');
        }
        $r = $this->next($i);
        $end = $r >= 0 ? $this->exprEnd($r) : null;
        if ($end !== null && $this->nameExprAt($end) && !$this->wrapped($end)) {
            $this->set($r, 'Interner::lookup(' . $this->text($r));
            $this->set($end, $this->text($end) . ')');
        }
    }

    /** Whether the token already carries a wrap (another operator got to it first). */
    private function wrapped(int $i): bool
    {
        return str_ends_with($this->text($i), ')') && $this->id($i) !== 0 && $this->text($i) !== ')'
            || str_starts_with($this->text($this->exprStart($i)), 'Interner::');
    }

    /** `"...$name..."` / `"...{$x->name}..."`: the name leaves the string as a looked-up concatenation. */
    private function interpolation(int $i): int
    {
        for ($j = $i + 1; $j < count($this->t); $j++) {
            if ($this->text($j) === '"' && !is_array($this->t[$j])) {
                return $j;
            }
            if ($this->id($j) === T_VARIABLE) {
                // a simple `$x`, `$x->y`
                $end = $j;
                $n = $j + 1;
                if ($n < count($this->t) && $this->id($n) === T_OBJECT_OPERATOR && $this->id($n + 1) === T_STRING) {
                    $end = $n + 1;
                }
                if ($this->nameExprAt($end)) {
                    if ($end === $j) {
                        $this->set($j, '" . Interner::lookup(' . $this->text($j) . ') . "');
                    } else {
                        $this->set($j, '" . Interner::lookup(' . $this->text($j));
                        $this->set($end, $this->text($end) . ') . "');
                    }
                }
                $j = $end;
                continue;
            }
            if ($this->id($j) === T_CURLY_OPEN) {
                $close = $this->matchForward($j);
                $last = $close - 1;
                if ($this->nameExprAt($last)) {
                    $this->set($j, '" . Interner::lookup(');
                    $this->set($close, ') . "');
                }
                $j = $close;
            }
        }
        return $i;
    }

    private function stripLower(int $i): void
    {
        $open = $this->next($i);
        if ($open < 0 || $this->text($open) !== '(') {
            return;
        }
        $close = $this->matchForward($open);
        $last = $this->next($close, -1);
        $comma = null;
        if ($last >= 0 && $this->text($last) === ',') {
            // a multi-line call's trailing comma
            $comma = $last;
            $last = $this->next($last, -1);
        }
        $first = $this->next($open);
        if ($last < 0 || $first < 0 || $this->exprEnd($first) !== $last
            || !($this->nameExprAt($last) || $this->classNameCallAt($last))
        ) {
            return;
        }
        // only class-like names are case-sensitive ids (member and function maps stay keyed by lowercase names)
        $w = words($this->text($last));
        if (array_intersect($w, ['method', 'methods', 'function', 'functions', 'property', 'properties', 'prop',
            'props', 'constant', 'constants', 'const']) !== []
        ) {
            return;
        }
        // compared with a literal: a keyword (`self`, `resource`...) keeps its case-insensitive match, a class-like
        // literal becomes the name's id
        $compared = null;
        $after = $this->next($comma ?? $close);
        if ($after >= 0 && in_array($this->id($after), [T_IS_IDENTICAL, T_IS_NOT_IDENTICAL, T_IS_EQUAL, T_IS_NOT_EQUAL], true)) {
            $r = $this->next($after);
            if ($r >= 0 && $this->id($r) === T_CONSTANT_ENCAPSED_STRING) {
                if (!CanonicalNames::isClass(stripcslashes(substr($this->text($r), 1, -1)))) {
                    return;
                }
                $compared = $r;
            }
        }
        // `in_array(strtolower(x), ['self', 'static', 'parent'])`
        $callee = $this->next($i, -1) >= 0 ? $this->next($this->next($i, -1), -1) : -1;
        if ($callee >= 0 && $this->text($this->next($i, -1)) === '(' && strtolower($this->text($callee)) === 'in_array'
            && $after >= 0 && $this->text($after) === ','
        ) {
            $list = $this->next($after);
            if ($list >= 0 && $this->text($list) === '[') {
                for ($k = $this->next($list); $k >= 0 && $this->text($k) !== ']'; $k = $this->next($k)) {
                    if ($this->id($k) === T_CONSTANT_ENCAPSED_STRING
                        && !CanonicalNames::isClass(stripcslashes(substr($this->text($k), 1, -1)))
                    ) {
                        return;
                    }
                }
            }
        }
        $this->set($i, '');
        $this->set($open, '');
        $this->set($close, '');
        if ($comma !== null) {
            $this->set($comma, '');
        }
        if ($compared !== null) {
            $this->literal($compared);
        }
        // `$x_lower = strtolower(<name>)`: $x_lower now holds the declared spelling
        $eq = $this->next($i, -1);
        if ($eq >= 0 && $this->text($eq) === '\\') {
            $eq = $this->next($eq, -1);
        }
        if ($eq >= 0 && $this->text($eq) === '=') {
            $v = $this->next($eq, -1);
            if ($v >= 0 && $this->id($v) === T_VARIABLE && !isName($this->text($v))) {
                $this->lowered[$this->text($v)] = true;
            }
        }
        // `\strtolower`
        $p = $i - 1;
        if ($p >= 0 && $this->text($p) === '\\') {
            $this->set($p, '');
        }
    }

    private function literal(int $i): void
    {
        if ($i < 0) {
            return;
        }
        if ($this->canon) {
            // compared with a string that used to be lowercased: the literal's declared spelling
            if ($this->id($i) === T_CONSTANT_ENCAPSED_STRING) {
                $v = stripcslashes(substr($this->text($i), 1, -1));
                if (CanonicalNames::isClass($v)) {
                    $this->set($i, var_export(CanonicalNames::of($v), true));
                }
            }
            return;
        }
        if ($this->id($i) === T_CONSTANT_ENCAPSED_STRING) {
            $v = stripcslashes(substr($this->text($i), 1, -1));
            if (preg_match('/^\\\\?[A-Za-z_][A-Za-z0-9_\\\\]*$/', $v)) {
                $this->set($i, $this->symText(ltrim($v, '\\')));
            }
            return;
        }
        // `X::class`
        if (in_array($this->id($i), [T_STRING, T_NAME_QUALIFIED, T_NAME_FULLY_QUALIFIED], true)) {
            $dc = $this->next($i);
            $c = $dc >= 0 ? $this->next($dc) : -1;
            if ($dc >= 0 && $this->text($dc) === '::' && $c >= 0 && strtolower($this->text($c)) === 'class'
                && !in_array(strtolower($this->text($i)), ['self', 'static', 'parent'], true)
            ) {
                // the resolved name is not known textually: keep `X::class` as the interned string
                $this->set($i, ($this->test ? 'Interner::intern(' : 'Interner::intern(') . $this->text($i));
                $this->set($c, 'class)');
            }
        }
    }

    private function symText(string $v): string
    {
        $v = CanonicalNames::of($v);
        if ($this->test) {
            return 'Interner::intern(' . var_export($v, true) . ')';
        }
        if (!isset(self::$sym[$v])) {
            $words = [];
            foreach (explode('\\', $v) as $p) {
                $words[] = strtoupper((string) preg_replace('/(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])/', '_', $p));
            }
            $name = 'C_' . preg_replace('/[^A-Z0-9_]/', '_', implode('__', $words));
            $taken = array_flip(self::$sym);
            $base = $name;
            for ($k = 2; isset($taken[$name]); $k++) {
                $name = $base . '_' . $k;
            }
            self::$sym[$v] = $name;
        }
        return 'Sym::' . self::$sym[$v];
    }

    private function switchOrMatch(int $i, bool $match): void
    {
        $open = $this->next($i);
        if ($open < 0 || $this->text($open) !== '(') {
            return;
        }
        $close = $this->matchForward($open);
        $last = $this->next($close, -1);
        if ($last < 0) {
            return;
        }
        if (!$this->nameExprAt($last)) {
            if (!isset($this->lowered[$this->text($last)])) {
                return;
            }
            $this->canon = true;
        }
        $body = $this->next($close);
        if ($body < 0 || $this->text($body) !== '{') {
            return;
        }
        $end = $this->matchForward($body);
        $depth = 0;
        $arm_start = true;
        for ($j = $body + 1; $j < $end; $j++) {
            $tx = $this->text($j);
            if (in_array($tx, ['(', '[', '{'], true)) {
                $depth++;
            } elseif (in_array($tx, [')', ']', '}'], true)) {
                $depth--;
            }
            if ($depth !== 0) {
                continue;
            }
            if (!$match && $this->id($j) === T_CASE) {
                $this->literal($this->next($j));
            } elseif ($match && $arm_start && in_array($this->id($j), [T_CONSTANT_ENCAPSED_STRING, T_STRING, T_NAME_QUALIFIED, T_NAME_FULLY_QUALIFIED], true)) {
                $this->literal($j);
            }
            if ($match) {
                // match conditions: after `{` or `,` (outside arm bodies), up to `=>`
                if ($tx === '=>') {
                    $arm_start = false;
                } elseif ($tx === ',') {
                    $p = $this->next($j, -1);
                    $arm_start = $this->armHasArrow($body, $j);
                } elseif ($this->id($j) !== T_WHITESPACE && !in_array($this->id($j), [T_CONSTANT_ENCAPSED_STRING, T_STRING, T_DOUBLE_COLON, T_CLASS], true) && $tx !== '::') {
                    // anything else in a condition: not a literal list
                }
            }
        }
    }

    /** Whether the arm the `,` at $j ends has already seen its `=>` (then the next token starts conditions). */
    private function armHasArrow(int $from, int $j): bool
    {
        $depth = 0;
        for ($k = $j - 1; $k > $from; $k--) {
            $tx = $this->text($k);
            if (in_array($tx, [')', ']', '}'], true)) {
                $depth++;
            } elseif (in_array($tx, ['(', '[', '{'], true)) {
                $depth--;
            }
            if ($depth !== 0) {
                continue;
            }
            if ($tx === '=>') {
                return true;
            }
            if ($tx === ',' || $tx === '{') {
                return false;
            }
        }
        return false;
    }

    private function inArray(int $i): void
    {
        $open = $this->next($i);
        if ($open < 0 || $this->text($open) !== '(') {
            return;
        }
        $first = $this->next($open);
        $end = $first >= 0 ? $this->exprEnd($first) : null;
        if ($end === null) {
            return;
        }
        if (!$this->nameExprAt($end)) {
            if (!isset($this->lowered[$this->text($end)])) {
                return;
            }
            $this->canon = true;
        }
        $comma = $this->next($end);
        $arr = $comma >= 0 && $this->text($comma) === ',' ? $this->next($comma) : -1;
        if ($arr < 0 || $this->text($arr) !== '[') {
            return;
        }
        $close = $this->matchForward($arr);
        for ($j = $arr + 1; $j < $close; $j++) {
            $p = $this->next($j, -1);
            if (($p === $arr || $this->text($p) === ',') && $this->id($j) !== T_WHITESPACE) {
                $this->literal($j);
            }
        }
    }

    private function docComment(int $i): void
    {
        $doc = $this->text($i);
        // the declaration the docblock belongs to: its variable / function name
        $decl = $this->next($i);
        $decl_names = [];
        $getter = false;
        $modifier = false;
        $this->fn_name = '';
        for ($j = $decl, $k = 0; $j >= 0 && $k < 30; $j = $this->next($j), $k++) {
            $tx = $this->text($j);
            if (in_array($this->id($j), [T_PUBLIC, T_PROTECTED, T_PRIVATE, T_STATIC, T_VAR, T_READONLY], true)) {
                $modifier = true;
            }
            if ($this->id($j) === T_VARIABLE) {
                $decl_names[] = substr($tx, 1);
                break;
            }
            if ($this->id($j) === T_FUNCTION) {
                $f = $this->next($j);
                $getter = $f >= 0 && isNameGetter($this->text($f));
                $this->fn_name = $f >= 0 ? $this->text($f) : '';
                break;
            }
            if ($tx === ';' || $tx === '{') {
                break;
            }
        }
        $cb = function (array $m) use ($decl_names, $getter, $modifier): string {
                $var = $m[5] ?? null;
                $tag = $m[2];
                if (($var !== null && isset($this->keep[$var])) || ($var === null && $decl_names !== [] && isset($this->keep[$decl_names[0]]))) {
                    return $m[0];
                }
                $isName = match ($tag) {
                    'param' => $var !== null && (isName($var)
                        || (isset(NAME_FIELDS[$var]) && in_array($this->cls, NAME_FIELDS[$var], true))),
                    'var' => $var !== null ? isName($var) : ($decl_names !== [] && $modifier && (isName($decl_names[0])
                        || (isset(NAME_FIELDS[$decl_names[0]]) && in_array($this->cls, NAME_FIELDS[$decl_names[0]], true)))),
                    'return' => $getter,
                };
                $type = $m[3];
                if (str_contains($type, 'class-string<')) {
                    // a class reference for dynamic calls / instantiation (its type drives the analysis): a string
                    if ($var !== null) {
                        $this->keep[$var] = true;
                    } elseif ($decl_names !== []) {
                        $this->keep[$decl_names[0]] = true;
                    }
                    return $m[0];
                }
                // an inline `@var T $x` (a local, often over foreign data) changes only when named as a name
                $inline = $tag === 'var' && ($var !== null || !$modifier);
                // a lowercase string is a name unless the slot says it is something else (a path, a key, an id...)
                $slot = $var ?? ($decl_names[0] ?? $this->fn_name);
                $other = $slot !== '' && array_intersect(words($slot), NOT_NAME_WORDS) !== [];
                if ($inline || $other) {
                    // nothing forced
                } elseif (!$isName && preg_match('/^(?:\??)(?:non-empty-)?lowercase-string(?:\|null)?$/', $type) && $var !== null) {
                    // a lowercase name: its native hint follows
                    $this->forced[$var] = true;
                } elseif (!$isName && preg_match('/^(?:\??)(?:non-empty-)?lowercase-string(?:\|null)?$/', $type) && $decl_names !== []) {
                    $this->forced[$decl_names[0]] = true;
                }
                $newType = $isName
                    ? flipNames($type)
                    : ($inline || $other || !preg_match('/^\s*(?:\?|null\|)?(?:non-empty-)?lowercase-string(?:\|null)?\s*$/', $type)
                        ? $type : (string) preg_replace('/\b(non-empty-)?lowercase-string\b/', 'int', $type));
                return $m[1] . $newType . ($m[4] ?? '');
        };
        // each tag's type: its exact extent (a type spans lines and spaces inside brackets)
        $new = $doc;
        if (preg_match_all('/@(?:psalm-|phpstan-)?(param|var|return)\s+/', $doc, $tm, PREG_OFFSET_CAPTURE)) {
            for ($k = count($tm[0]) - 1; $k >= 0; $k--) {
                [$prefix, $at] = $tm[0][$k];
                $start = $at + strlen($prefix);
                $flat = (string) preg_replace_callback('/\n\s*\*/', static fn(array $x): string => str_repeat(' ', strlen($x[0])), substr($doc, $start));
                try {
                    $end = \Psalm\Tools\IdConvert\TypeStr::parse($flat)['end'];
                } catch (\Throwable) {
                    continue;
                }
                if ($end === 0) {
                    continue;
                }
                $type = substr($doc, $start, $end);
                $var_part = '';
                $var = null;
                if (preg_match('/^(\s+(?:&\s*)?(?:\.\.\.)?\$(\w+))/', substr($flat, $end, 200), $vm)) {
                    $var_part = $vm[1];
                    $var = $vm[2];
                }
                $m = [0 => substr($doc, $at, $start + $end + strlen($var_part) - $at), 1 => $prefix, 2 => $tm[1][$k][0],
                    3 => $type, 4 => $var_part];
                if ($var !== null) {
                    $m[5] = $var;
                }
                $rep = $cb($m);
                if ($modifier && $decl_names !== [] && $tm[1][$k][0] === 'var'
                    && preg_match('/^(?:non-empty-)?array<\s*(?:non-empty-)?(?:lowercase-)?string\s*,/i', ltrim($type))
                    && preg_match('/^@\S+\s+(?:non-empty-)?array<\s*int\s*,/i', ltrim($rep))
                ) {
                    self::$id_keyed[$decl_names[0]] = true;
                }
                $new = substr($new, 0, $at) . $rep . substr($new, $start + $end + strlen($var_part));
            }
        }
        if ($new !== null && $new !== $doc) {
            $this->set($i, $new);
        }
    }
}

/**
 * Structural changes made before the rewrite (pzoom's shapes): a class-name literal is its own atom holding the
 * class id, not a literal string (pzoom's TLiteralClassname { name: StrId }).
 */
function structural(string $file, string $src): string
{
    if (str_ends_with($file, '/Type/Atomic/TLiteralClassString.php')) {
        $src = str_replace('final class TLiteralClassString extends TLiteralString', 'final class TLiteralClassString extends TString', $src);
        $src = preg_replace('/(final class TLiteralClassString extends TString\n\{\n)/', "$1    public string \$value;\n\n", $src, 1);
        $src = str_replace("        parent::__construct(\$value, \$from_docblock);\n    }",
            "        \$this->value = \$value;\n        parent::__construct(\$from_docblock);\n    }\n\n"
            . "    /**\n     * @return static\n     */\n    public function setValue(string \$value): self\n    {\n"
            . "        if (\$value === \$this->value) {\n            return \$this;\n        }\n        \$cloned = clone \$this;\n"
            . "        \$cloned->value = \$value;\n        return \$cloned;\n    }", $src);
    }
    return (string) $src;
}

/**
 * A name slot's type with its names as ids: every union (the value, an array's keys / values, nested) whose
 * non-null atoms are all strings becomes `int` (a union mixing strings with ints, like shape keys, is no name).
 */
function flipNames(string $type): string
{
    $flat = (string) preg_replace_callback('/\n\s*\*/', static fn(array $x): string => str_repeat(' ', strlen($x[0])), $type);
    $edits = [];
    // a shape's entries are fields, not name slots: a type with shapes flips only at the top
    $paths = str_contains($flat, '{') ? [''] : ['', '#k', '#v', '#k#v', '#v#k', '#v#v', '#v#v#k', '#v#v#v'];
    foreach ($paths as $path) {
        try {
            $u = \Psalm\Tools\IdConvert\TypeStr::parse($flat);
            foreach (\Psalm\Tools\IdConvert\TypeStr::at($u, $path) as $target) {
                $atoms = array_values(array_filter($target['atoms'], static fn(array $a): bool => !($a['kind'] === 'name' && strtolower((string) $a['name']) === 'null')));
                if ($atoms === [] || count(array_filter($atoms, \Psalm\Tools\IdConvert\TypeStr::isStringish(...))) !== count($atoms)) {
                    continue;
                }
                foreach (\Psalm\Tools\IdConvert\TypeStr::toIntAt($flat, $path) as $e) {
                    $edits[$e[0] . ':' . $e[1]] = $e;
                }
            }
        } catch (\Throwable) {
            return $type;
        }
    }
    return $edits === [] ? $type : \Psalm\Tools\IdConvert\TypeStr::apply($type, array_values($edits));
}

// one file, printed (debugging)
if (isset($argv[2])) {
    CanonicalNames::init($root);
    echo (new Rewriter((string) file_get_contents($argv[2]), false))->run();
    exit(0);
}

CanonicalNames::init($root);

// the files
$it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root . 'src', FilesystemIterator::SKIP_DOTS));
$files = [];
foreach ($it as $f) {
    if (str_ends_with((string) $f, '.php')) {
        $files[] = (string) $f;
    }
}
foreach (['tests', 'examples'] as $d) {
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root . $d, FilesystemIterator::SKIP_DOTS));
    foreach ($it as $f) {
        if (str_ends_with((string) $f, '.php') && !str_contains((string) $f, '/fixtures/') && !str_contains((string) $f, '/stubs/')) {
            $files[] = (string) $f;
        }
    }
}
$changed = 0;
foreach ($files as $file) {
    $orig_src = (string) file_get_contents($file);
    $src = structural($file, $orig_src);
    $rw = new Rewriter($src, !str_starts_with($file, $root . 'src/'));
    $out = $rw->run();
    if ($out === $orig_src) {
        continue;
    }
    // imports
    $ns = preg_match('/^namespace\s+([^;{\s]+)/m', $out, $m) ? $m[1] : '';
    foreach (['Interner', 'Sym'] as $cls) {
        if (!preg_match('/(?<![\w\\\\$>])' . $cls . '::/', $out) || $ns === 'Psalm\\Internal'
            || preg_match('/^use\s+Psalm\\\\Internal\\\\' . $cls . ';/m', $out)
        ) {
            continue;
        }
        $line = "use Psalm\\Internal\\$cls;\n";
        if (preg_match('/^namespace\s+[^;]+;\n/m', $out, $nm, PREG_OFFSET_CAPTURE)) {
            $at = $nm[0][1] + strlen($nm[0][0]);
            $out = substr($out, 0, $at) . "\n" . $line . substr($out, $at);
        } else {
            $out = (string) preg_replace('/(?<![\w\\\\$>])' . $cls . '::/', '\\\\Psalm\\\\Internal\\\\' . $cls . '::', $out);
        }
    }
    file_put_contents($file, $out);
    $changed++;
}

file_put_contents($root . '.id-keyed.json', json_encode(array_keys(Rewriter::$id_keyed)));

// the interner: the preloaded names merged by the Codebase, the table kept beside the cache, forked workers
// handing their new strings back to the parent
copy(__DIR__ . '/templates/Interner.php', $root . 'src/Psalm/Internal/Interner.php');
$patch = static function (string $rel, string $anchor, string $insert) use ($root): void {
    $file = $root . $rel;
    $src = (string) file_get_contents($file);
    if (substr_count($src, $anchor) !== 1) {
        fwrite(STDERR, "bootstrap anchor not unique in $rel\n");
        exit(1);
    }
    $src = str_replace($anchor, $anchor . $insert, $src);
    foreach (['Interner', 'Sym'] as $cls) {
        if (str_contains($insert, "$cls::") && !preg_match('/^use Psalm\\\\Internal\\\\' . $cls . ';/m', $src)
            && !preg_match('/^namespace Psalm\\\\Internal;/m', $src)
        ) {
            $src = (string) preg_replace('/^(namespace [^;]+;\n)/m', "$1\nuse Psalm\\\\Internal\\\\$cls;\n", $src, 1);
        }
    }
    file_put_contents($file, $src);
};
$patch('src/Psalm/Codebase.php', "        if (\$progress === null) {\n            \$progress = new VoidProgress();\n        }\n",
    "        Interner::merge(Sym::PRELOADED);\n        \$cache_directory = \$config->getCacheDirectory();\n"
    . "        if (\$cache_directory !== null) {\n            Interner::persistTo(\$cache_directory);\n        }\n");
foreach (['InitScannerTask', 'InitAnalyzerTask'] as $t) {
    $patch("src/Psalm/Internal/Fork/$t.php", "Cancellation \$cancellation): mixed\n    {\n", "        Interner::mark();\n\n");
}
foreach (['ShutdownScannerTask', 'ShutdownAnalyzerTask'] as $t) {
    $patch("src/Psalm/Internal/Fork/$t.php", "        return [\n", "            'interner' => Interner::delta(),\n");
}
foreach (['Scanner' => 'PoolData', 'Analyzer' => 'WorkerData'] as $c => $type) {
    $patch("src/Psalm/Internal/Codebase/$c.php", "                \$pool_data = \$pool_data->await();\n",
        "\n                Interner::merge(\$pool_data['interner']);\n");
    $patch("src/Psalm/Internal/Codebase/$c.php", " * @psalm-type  $type = array{\n", " *     interner: list<string>,\n");
}

// the name constants
$names = Rewriter::$sym;
ksort($names);
$list = '';
foreach ($names as $v => $c) {
    $list .= '    ' . var_export($c, true) . ' => ' . var_export((string) $v, true) . ",\n";
}
file_put_contents($root . 'bin/generate-sym.php', str_replace("    // NAMES\n", $list, (string) file_get_contents(__DIR__ . '/generate-sym.php.tpl')));
passthru('php ' . escapeshellarg($root . 'bin/generate-sym.php'));
echo "rewrote $changed files, " . count($names) . " Sym constants\n";
