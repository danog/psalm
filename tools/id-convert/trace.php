<?php

/**
 * trace.php ROOT ISSUES.json PSALM CONFIG: class A of the remaining errors (a variable or property written with ids
 * at some sites and strings at others). The writers are found textually, their types are asked of Psalm itself
 * (`@psalm-trace` on each writing statement, one run), then the traces go and each writer of the wrong kind gets
 * `Interner::intern` / `Interner::lookup`.
 *
 * Writers: `$v = RHS;` statements of the variable in the enclosing function; `$this->p[K] = V;` / `self::$p[K] = V;`
 * / `$this->p = V;` writes of the property in the file (their key / value variables are traced).
 */

declare(strict_types=1);

require_once __DIR__ . '/names.php';
require_once __DIR__ . '/TypeStr.php';

[, $root, $issues_file, $psalm, $config] = $argv;
$root = rtrim($root, '/') . '/';
$issues = json_decode((string) file_get_contents($issues_file), true);
// only what the conversion introduced: the issues an unconverted tree has too (env ID_CONVERT_PRISTINE) stay
$pristine_file = (string) getenv('ID_CONVERT_PRISTINE');
if ($pristine_file !== '' && is_file($pristine_file)) {
    $seen_pristine = [];
    foreach (json_decode((string) file_get_contents($pristine_file), true) ?: [] as $pi) {
        $pk = $pi['file_name'] . "\0" . $pi['type'] . "\0" . $pi['message'];
        $seen_pristine[$pk] = ($seen_pristine[$pk] ?? 0) + 1;
    }
    $issues = array_values(array_filter($issues, static function (array $ci) use (&$seen_pristine): bool {
        $ck = $ci['file_name'] . "\0" . $ci['type'] . "\0" . $ci['message'];
        if (($seen_pristine[$ck] ?? 0) > 0) {
            $seen_pristine[$ck]--;
            return false;
        }
        return true;
    }));
}
CanonicalNames::init($root);

$stringish = '(?:[a-z]+-)*string(?:<[^>]*>)?|\'[^\']*\'|""';
$intish = 'int|positive-int|non-negative-int|int<[^>]*>|-?\d+';

/**
 * @var list<array{file: string, at: int, stmt_end: int, var: string, what: string, want: string, rhs: ?array{int, int}}>
 */
$sites = [];
/** @var array<string, array<int, array{int, string}>> literal values to intern (no trace needed) */
$lit_edits = [];
$src = [];
$read = static function (string $f) use (&$src): string {
    return $src[$f] ??= (string) file_get_contents($f);
};

/** The function body enclosing an offset: [start, end). */
function enclosing(string $s, int $at): ?array
{
    $fn = strrpos(substr($s, 0, $at), 'function ');
    if ($fn === false) {
        return null;
    }
    $open = strpos($s, '{', $fn);
    if ($open === false || $open > $at) {
        return null;
    }
    $depth = 0;
    for ($k = $open; $k < strlen($s); $k++) {
        if ($s[$k] === '{') {
            $depth++;
        } elseif ($s[$k] === '}') {
            $depth--;
            if ($depth === 0) {
                return [$open, $k];
            }
        }
    }
    return null;
}

/** Statement-level `$v = RHS;` assignments in [from, to): [stmt start, rhs start, rhs end]. */
function assignments(string $s, int $from, int $to, string $var): array
{
    $out = [];
    $body = substr($s, $from, $to - $from);
    if (!preg_match_all('/(?<=[;{}]|\n)(\s*)(' . preg_quote($var, '/') . '\s*=(?!=)\s*)/', $body, $m, PREG_OFFSET_CAPTURE)) {
        return [];
    }
    foreach ($m[2] as $k => [$whole, $off]) {
        $rhs = $from + $off + strlen($whole);
        // the statement's `;` at depth 0
        $depth = 0;
        for ($e = $rhs; $e < $to; $e++) {
            $c = $s[$e];
            if ($c === '(' || $c === '[' || $c === '{') {
                $depth++;
            } elseif ($c === ')' || $c === ']' || $c === '}') {
                $depth--;
            } elseif ($c === "'" || $c === '"') {
                // skip a string literal
                for ($e++; $e < $to && $s[$e] !== $c; $e++) {
                    if ($s[$e] === '\\') {
                        $e++;
                    }
                }
            } elseif ($c === ';' && $depth === 0) {
                break;
            }
        }
        $out[] = [$from + $off, $rhs, $e];
    }
    return $out;
}

foreach ($issues as $i) {
    $file = $i['file_path'] ?? '';
    if ($file === '' || substr($read($file), $i['from'], $i['to'] - $i['from']) !== $i['selected_text']) {
        continue;
    }
    $msg = $i['message'];
    $want = null;
    // a variable of both kinds reaching a parameter of one
    if ($i['type'] === 'PossiblyInvalidArgument'
        && preg_match('/expects (?:null\|)?(int|string)(?:\|null)?, but possibly different type (.+) provided/', $msg, $m)
        && preg_match('/(^|\|)(?:' . $intish . ')(\||$)/', $m[2]) && preg_match('/(^|\|)(?:' . $stringish . ')(\||$)/', $m[2])
        && preg_match('/^\$\w+$/', $i['selected_text'])
    ) {
        $want = $m[1];
        $s = $read($file);
        $body = enclosing($s, $i['from']);
        if ($body === null) {
            continue;
        }
        foreach (assignments($s, $body[0], $i['from'], $i['selected_text']) as [$stmt, $rs, $re]) {
            $sites[] = ['file' => $file, 'at' => $stmt, 'var' => $i['selected_text'], 'what' => 'value', 'want' => $want,
                'rhs' => [$rs, $re]];
        }
        // foreach bindings of the variable: the iterated array (its keys or values) is traced
        $v = preg_quote($i['selected_text'], '/');
        $bodytext = substr($s, $body[0], $i['from'] - $body[0]);
        if (preg_match_all('/foreach\s*\(\s*(\$\w+(?:->\w+)*)\s+as\s+(?:(\$\w+)\s*=>\s*)?&?(\$\w+)\s*\)/', $bodytext, $fm, PREG_OFFSET_CAPTURE | PREG_SET_ORDER)) {
            foreach ($fm as $f) {
                $is_key = ($f[2][0] ?? '') === $i['selected_text'];
                $is_val = $f[3][0] === $i['selected_text'];
                if (!$is_key && !$is_val) {
                    continue;
                }
                $ls = strrpos(substr($s, 0, $body[0] + $f[0][1]), "\n");
                $sites[] = ['file' => $file, 'at' => $ls + 1, 'var' => $f[1][0], 'what' => 'array',
                    'want' => ['k' => $is_key ? $want : null, 'v' => $is_val ? $want : null],
                    'rhs' => [$body[0] + $f[1][1], $body[0] + $f[1][1] + strlen($f[1][0])]];
            }
        }
        continue;
    }
    // a whole-array value whose `[K => V]` literals hold strings where the map wants ids
    if (in_array($i['type'], ['InvalidPropertyAssignmentValue', 'PossiblyInvalidPropertyAssignmentValue'], true)
        && preg_match("/with declared type '(.+?)' cannot be assigned type '(.+?)'/", $msg, $m)
        && preg_match_all('/\[\s*(\$\w+)\s*=>\s*(\$\w+)\s*\]/', $i['selected_text'], $lm, PREG_OFFSET_CAPTURE)
    ) {
        $declared = strtolower($m[1]);
        $wk = preg_match('/^(?:non-empty-)?array<(?:' . $intish . '),/', $declared) === 1;
        $wv = preg_match('/, (?:' . $intish . ')>$/', $declared) === 1;
        $ls = strrpos(substr($read($file), 0, $i['from']), "\n");
        foreach ($lm[1] as $k2 => [$kv, $koff]) {
            [$vv, $voff] = $lm[2][$k2];
            if ($wk) {
                $sites[] = ['file' => $file, 'at' => $ls + 1, 'var' => $kv, 'what' => 'key', 'want' => 'int',
                    'rhs' => [$i['from'] + $koff, $i['from'] + $koff + strlen($kv)]];
            }
            if ($wv) {
                $sites[] = ['file' => $file, 'at' => $ls + 1, 'var' => $vv, 'what' => 'value', 'want' => 'int',
                    'rhs' => [$i['from'] + $voff, $i['from'] + $voff + strlen($vv)]];
            }
        }
        continue;
    }
    // a property written with keys / values of both kinds
    if (in_array($i['type'], ['InvalidPropertyAssignmentValue', 'PossiblyInvalidPropertyAssignmentValue'], true)
        && preg_match("/with declared type '(.+?)' cannot be assigned type '(.+?)'/", $msg, $m)
        && (preg_match('/^(\$\w+(?:->\w+)*->|self::\$|static::\$)(\w+)$/', $i['selected_text'], $pm)
            // Psalm may select the whole statement: the property is the message's subject
            || preg_match('/^([\w\\\\]+::\$|\$\w+(?:->\w+)*->)(\w+) with declared type/', $msg, $pm))
    ) {
        $declared = strtolower($m[1]);
        // keys / values wanted as ids or strings
        $wk = preg_match('/^(?:non-empty-)?(?:array|list)<(?:' . $intish . '),/', $declared) ? 'int'
            : (preg_match('/^(?:non-empty-)?array<(?:' . $stringish . '),/', $declared) ? 'string' : null);
        $wv = preg_match('/, (?:' . $intish . ')>$/', $declared) || preg_match('/^(?:non-empty-)?list<(?:' . $intish . ')>$/', $declared) ? 'int'
            : (preg_match('/, (?:' . $stringish . ')>$/', $declared) || preg_match('/^(?:non-empty-)?list<(?:' . $stringish . ')>$/', $declared) ? 'string' : null);
        $s = $read($file);
        $prop = $pm[2];
        // every write of the property in the file: `->prop[K1][K2]... = V;` (a `[]` appends), each key level
        // against its level of the declared type
        $levelKind = static function (string $declared, string $path) use ($stringish, $intish): ?string {
            try {
                $atoms = [];
                foreach (\Psalm\Tools\IdConvert\TypeStr::at(\Psalm\Tools\IdConvert\TypeStr::parse($declared), $path) as $u) {
                    foreach ($u['atoms'] as $at) {
                        $n = strtolower((string) ($at['name'] ?? ''));
                        if ($n !== 'null') {
                            $atoms[] = $n;
                        }
                    }
                }
            } catch (\Throwable) {
                return null;
            }
            if ($atoms === []) {
                return null;
            }
            $ints = count(array_filter($atoms, static fn(string $n): bool => (bool) preg_match('/^(?:' . $intish . ')$/', $n)));
            $strs = count(array_filter($atoms, static fn(string $n): bool => (bool) preg_match('/^(?:' . $stringish . ')$/', $n)));
            return $ints === count($atoms) ? 'int' : ($strs === count($atoms) ? 'string' : null);
        };
        if (preg_match_all('/(?:\$\w+(?:->\w+)*->|self::\$|static::\$)' . $prop . '((?:\[[^\[\]]*\])+)\s*=(?![=>])\s*/', $s, $wm, PREG_OFFSET_CAPTURE)) {
            foreach ($wm[0] as $k => [$whole, $off]) {
                [$dims, $dims_at] = $wm[1][$k];
                $ls = strrpos(substr($s, 0, $off), "\n");
                $stmt = $ls === false ? $off : $ls + 1;
                preg_match_all('/\[([^\[\]]*)\]/', $dims, $dm, PREG_OFFSET_CAPTURE);
                foreach ($dm[1] as $d => [$key, $koff]) {
                    $want = $levelKind($declared, str_repeat('#v', $d) . '#k');
                    if ($want !== null && trim($key) !== '') {
                        $key_at = $dims_at + $koff;
                        $sites[] = ['file' => $file, 'at' => $stmt, 'var' => trim($key), 'what' => 'key', 'want' => $want,
                            'rhs' => [$key_at, $key_at + strlen($key)]];
                    }
                }
                $wv2 = $levelKind($declared, str_repeat('#v', count($dm[1])));
                if ($wv2 !== null) {
                    $vs = $off + strlen($whole);
                    $ve = strpos($s, ';', $vs);
                    $vtext = $ve === false ? '' : trim(substr($s, $vs, $ve - $vs));
                    if ($wv2 === 'int' && preg_match("/^'([A-Za-z_\\\\][A-Za-z0-9_\\\\]*)'$/", $vtext, $lm)) {
                        $lit_edits[$file][$vs] = [$ve, 'Interner::intern(' . var_export(CanonicalNames::of(stripcslashes($lm[1])), true) . ')'];
                    } elseif ($ve !== false && preg_match('/^\$\w+(?:->\w+)*$/', $vtext)) {
                        $sites[] = ['file' => $file, 'at' => $stmt, 'var' => $vtext, 'what' => 'value', 'want' => $wv2,
                            'rhs' => [$vs, $ve]];
                    }
                }
            }
        }
        // whole-array writes: `->prop += $v;` / `->prop = array_replace($a, $b);` / `->prop = $v;` - the arrays traced
        if (preg_match_all('/(?:\$\w+(?:->\w+)*->|self::\$|static::\$)' . $prop . '\s*(\+=|=(?![=>]))\s*([^;]+);/', $s, $am, PREG_OFFSET_CAPTURE)) {
            foreach ($am[2] as $k => [$rhs, $roff]) {
                $ls = strrpos(substr($s, 0, $am[0][$k][1]), "\n");
                $stmt = $ls === false ? $am[0][$k][1] : $ls + 1;
                $args = preg_match('/^array_(?:replace|merge)\((.*)\)$/s', trim($rhs), $rm) ? $rm[1] : trim($rhs);
                if (preg_match_all('/\$\w+(?:->\w+)*(?:\[[\'"]\w+[\'"]\])?/', $args, $vm2, PREG_OFFSET_CAPTURE)) {
                    $base = $roff + (int) strpos($rhs, $args);
                    foreach ($vm2[0] as [$v, $voff]) {
                        if (str_contains($v, $prop)) {
                            continue; // the property itself
                        }
                        $sites[] = ['file' => $file, 'at' => $stmt, 'var' => $v, 'what' => 'array',
                            'want' => ['k' => $levelKind($declared, '#k'), 'v' => $levelKind($declared, '#v')],
                            'rhs' => [$base + $voff, $base + $voff + strlen($v)]];
                    }
                }
            }
        }
    }
}

// dedupe; only plain variables can be traced (other keys are judged by their form)
$uniq = [];
foreach ($sites as $st) {
    $uniq[$st['file'] . ':' . $st['at'] . ':' . $st['var'] . ':' . $st['rhs'][0]] = $st;
}
$sites = array_values($uniq);
// a fetch chain (`$param->name`) is traced through a temporary copied just before the statement
$tmp = 0;
foreach ($sites as &$st) {
    if (!preg_match('/^\$\w+$/', $st['var']) && preg_match('/^\$\w+(?:->\w+)*(?:\[[\'"]\w+[\'"]\])?$/', $st['var'])) {
        $st['tmp'] = '$__idtrace' . ++$tmp;
    }
}
unset($st);
$traced = array_values(array_filter($sites, static fn(array $st): bool => isset($st['tmp']) || (bool) preg_match('/^\$\w+$/', $st['var'])));
fwrite(STDERR, count($sites) . " writer sites, " . count($traced) . " traced\n");

// instrument: `/** @psalm-trace $v */` before each writing statement (one per statement and variable)
$by_file = [];
$tmps = [];
foreach ($traced as $st) {
    $by_file[$st['file']][$st['at']][$st['tmp'] ?? $st['var']] = true;
    if (isset($st['tmp'])) {
        $tmps[$st['file']][$st['at']][$st['tmp']] = $st['var'];
    }
}
$orig = [];
foreach ($by_file as $file => $stmts) {
    $s = $orig[$file] = $read($file);
    krsort($stmts);
    foreach ($stmts as $at => $vars) {
        $indent = (string) preg_replace('/\S.*/s', '', substr($s, $at, 200));
        $copies = '';
        foreach ($tmps[$file][$at] ?? [] as $t => $expr) {
            $copies .= $indent . $t . ' = ' . $expr . ";\n";
        }
        // the copies come first; the trace docblock sits on the statement itself (one line each way)
        $s = substr($s, 0, $at) . $copies . $indent . '/** @psalm-trace ' . implode(' ', array_keys($vars)) . " */\n"
            . substr($s, $at);
    }
    file_put_contents($file, $s);
}
$report = sys_get_temp_dir() . '/trace-' . getmypid() . '.json';
passthru('cd ' . escapeshellarg($root) . ' && php -d memory_limit=-1 ' . escapeshellarg($psalm) . ' -c ' . escapeshellarg($config)
    . ' --no-cache --threads=4 --no-progress --output-format=json --report=' . escapeshellarg($report) . ' > /dev/null 2>&1');
foreach ($orig as $file => $s) {
    file_put_contents($file, $s);
}
$traces = [];
foreach (json_decode((string) @file_get_contents($report), true) ?: [] as $i) {
    if ($i['type'] === 'Trace' && preg_match('/^(\$\w+): (.+)$/', $i['message'], $m)) {
        // the trace line is the statement's first line in the instrumented file = the line of `at` in the original
        $traces[$i['file_path'] . ':' . $i['line_from'] . ':' . $m[1]] = strtolower($m[2]);
    }
}
@unlink($report);

// the traced statement's line in the instrumented file: its original line plus the traces inserted before it
$edits = [];
foreach ($traced as $st) {
    $s = $orig[$st['file']];
    $line = substr_count($s, "\n", 0, $st['at']) + 1;
    $before = 0;
    foreach (array_keys($by_file[$st['file']]) as $at) {
        if ($at < $st['at']) {
            $before += 1 + count($tmps[$st['file']][$at] ?? []);
        } elseif ($at === $st['at']) {
            $before += count($tmps[$st['file']][$at] ?? []);
        }
    }
    // Psalm reports the trace at the statement (its first line after the inserted docblock)
    $tv = $st['tmp'] ?? $st['var'];
    $type = $traces[$st['file'] . ':' . ($line + $before + 1) . ':' . $tv]
        ?? $traces[$st['file'] . ':' . ($line + $before) . ':' . $tv] ?? null;
    if ($type === null) {
        continue;
    }
    $isInt = (bool) preg_match('/^(?:null\|)?(?:' . $intish . ')(?:\|(?:null|' . $intish . '))*$/', $type);
    $isStr = (bool) preg_match('/^(?:null\|)?(?:' . $stringish . ')(?:\|(?:null|' . $stringish . '))*$/', $type);
    $nl = str_contains($type, 'null') ? 'OrNull' : '';
    [$rs, $re] = $st['rhs'];
    $text = substr($s, $rs, $re - $rs);
    if ($st['what'] === 'array') {
        // a traced array: its keys / values wrapped to the declared kinds
        if (preg_match('/^(?:non-empty-)?array<\s*([^,<>]+?)\s*,\s*(.+)>$/', $type, $am2) || preg_match('/^(?:non-empty-)?list<(.+)>$/', $type, $lm2)) {
            $tk = isset($am2[1]) ? $am2[1] : 'int';
            $tv = isset($am2[2]) ? $am2[2] : $lm2[1];
            $wrap = trim($text);
            if ($st['want']['k'] === 'int' && preg_match('/^(?:' . $stringish . ')$/', $tk)) {
                $wrap = "Interner::internKeys($wrap)";
            } elseif ($st['want']['k'] === 'string' && preg_match('/^(?:' . $intish . ')$/', $tk)) {
                $wrap = "Interner::lookupKeys($wrap)";
            }
            if ($st['want']['v'] === 'int' && preg_match('/^(?:' . $stringish . ')$/', trim($tv))) {
                $wrap = "Interner::internList($wrap)";
            } elseif ($st['want']['v'] === 'string' && preg_match('/^(?:' . $intish . ')$/', trim($tv))) {
                $wrap = "Interner::lookupList($wrap)";
            }
            if ($wrap !== trim($text)) {
                $edits[$st['file']][$rs] = [$re, $wrap];
            }
        }
        continue;
    }
    if ($st['want'] === 'int' && $isStr) {
        $edits[$st['file']][$rs] = [$re, "Interner::intern$nl(" . trim($text) . ')'];
    } elseif ($st['want'] === 'string' && $isInt) {
        $edits[$st['file']][$rs] = [$re, "Interner::lookup$nl(" . trim($text) . ')'];
    }
}
// untraceable keys (expressions): judged by form (string functions / concatenations are strings)
foreach ($sites as $st) {
    if (preg_match('/^\$\w+$/', $st['var']) || $st['want'] !== 'int') {
        continue;
    }
    // an id looked up into a key where ids are wanted: the id itself
    if (preg_match('/^Interner::lookup\((.*)\)$/s', trim($st['var']), $um3)) {
        [$rs, $re] = $st['rhs'];
        $edits[$st['file']][$rs] = [$re, $um3[1]];
        continue;
    }
    // a literal key: the name's id (its declared spelling)
    if (preg_match("/^'([A-Za-z_\\\\][A-Za-z0-9_\\\\]*)'$/", trim($st['var']), $lm)) {
        [$rs, $re] = $st['rhs'];
        $edits[$st['file']][$rs] = [$re, 'Interner::intern(' . var_export(CanonicalNames::of(stripcslashes($lm[1])), true) . ')'];
        continue;
    }
    // string forms: string functions, concatenation, php-parser names (`$node->name->name`, `->toString()`), casts
    if (preg_match('/^(?:strtolower|substr|str_\w+|trim|ltrim|rtrim|sprintf|implode|preg_replace)\(|\s\.\s|->name->name$|->toString\(\)$|->toLowerString\(\)$|^\(string\)|\[\'\w+\'\]$/', $st['var'])) {
        [$rs, $re] = $st['rhs'];
        $edits[$st['file']][$rs] = [$re, 'Interner::intern(' . trim($st['var']) . ')'];
    }
}
foreach ($lit_edits as $file => $list) {
    foreach ($list as $rs => $e) {
        $edits[$file][$rs] ??= $e;
    }
}
$n = 0;
foreach ($edits as $file => $list) {
    $s = $orig[$file] ?? $read($file);
    krsort($list);
    foreach ($list as $rs => [$re, $text]) {
        $s = substr($s, 0, $rs) . $text . substr($s, $re);
        $n++;
    }
    if (!str_contains($s, 'use Psalm\\Internal\\Interner;') && preg_match('/^namespace (?!Psalm\\\\Internal;)[^;]+;\n/m', $s)) {
        $s = (string) preg_replace('/^(namespace [^;]+;\n)/m', "$1\nuse Psalm\\\\Internal\\\\Interner;\n", $s, 1);
    }
    file_put_contents($file, $s);
}
echo "traced " . count($traces) . " types, fixed $n writers\n";
