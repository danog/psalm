<?php
// migrate.php facts.jsonl out-edits.jsonl: chooses the string slots to turn into interned ids (IdMigratePlugin facts)
// and writes apply.php edits. A slot is kept when converting it removes more string work than it adds:
//   + each `Interner::intern(<read>)` of it (the call disappears), + each flow into another converted slot
//   - each flow into it from a non-converted source (an intern at the boundary), - 0.3 per other read
//     (it reads Interner::lookup(id)) and per flow out into a non-converted slot.
declare(strict_types=1);
[$_, $in, $out] = $argv + [null, null, null];
if ($in === null || $out === null) { exit("usage: migrate.php facts.jsonl edits.jsonl\n"); }
$decl = []; $flows = []; $uses = []; $block = []; $syms = [];
foreach (file($in, FILE_IGNORE_NEW_LINES) as $line) {
    $r = json_decode($line, true);
    switch ($r['kind']) {
        case 'decl': $decl[$r['slot']] = $r; break;
        case 'flow': $flows[$r['slot'] . json_encode($r['src'])] = $r; break;
        case 'use': $uses[$r['slot'] . json_encode($r['r']) . $r['ctx']] = $r; break;
        case 'block': $block[$r['slot']] = $r['why']; break;
        case 'sym': $syms[$r['name']] = $r['value']; break;
    }
}
$X = []; $why = [];
foreach ($decl as $slot => $d) {
    if (!$d['fixed']) { $why['signature: ' . $d['why']] = ($why['signature: ' . $d['why']] ?? 0) + 1; continue; }
    if (isset($block[$slot])) { $why['blocked: ' . $block[$slot]] = ($why['blocked: ' . $block[$slot]] ?? 0) + 1; continue; }
    $X[$slot] = true;
}
$in_flows = []; $out_flows = []; $slot_uses = [];
foreach ($flows as $f) {
    $in_flows[$f['slot']][] = $f;
    if ($f['src']['k'] === 'slot') { $out_flows[$f['src']['slot']][] = $f; }
}
foreach ($uses as $u) { $slot_uses[$u['slot']][] = $u; }
// objective: total string work after the change (0 = today). Local search by toggling slots.
$cand = $X; $X = [];
$delta = static function (string $s) use (&$X, $in_flows, $out_flows, $slot_uses): float {
    // change of the total when $s flips membership
    $was = isset($X[$s]);
    $cost = static function (bool $in) use ($s, &$X, $in_flows, $out_flows, $slot_uses): float {
        $c = 0.0;
        if ($in) {
            foreach ($slot_uses[$s] ?? [] as $u) { $c += $u['ctx'] === 'intern' ? -1.0 : 0.3; }
        }
        foreach ($in_flows[$s] ?? [] as $f) {
            $k = $f['src']['k'];
            $src_in = $k === 'slot' && ($f['src']['slot'] === $s ? $in : isset($X[$f['src']['slot']]));
            if ($in) { if ($k === 'str' || ($k === 'slot' && !$src_in)) { $c += 1.0; } }
            elseif ($src_in) { $c += 0.3; }
        }
        foreach ($out_flows[$s] ?? [] as $f) {
            if ($f['slot'] === $s) { continue; }
            $t_in = isset($X[$f['slot']]);
            if ($t_in && !$in) { $c += 1.0; } elseif (!$t_in && $in) { $c += 0.3; }
        }
        return $c;
    };
    return $cost(!$was) - $cost($was);
};
$names_mode = in_array('--names', $argv, true);
// pzoom's target state: every class-like / member name slot is an id (strings only at resolution boundaries)
$nameish = '/(^|_)(fq|fqn|fqcn|fqcln|class|classes|classlike|classlikes|interface|trait|enum|parent|self|static|declaring|appearing|'
    . 'implementing|extended|mixin|method|methods|function|functions|property|prop|const|case)(_|$)|_name$|^name$|_name_lc$|name$/i';
foreach ($cand as $s => $_) {
    if ($names_mode) {
        $leaf = str_contains($s, '|') ? substr($s, strrpos($s, '|') + 1) : substr($s, strrpos($s, '::') + 2);
        if (preg_match($nameish, $leaf)) { $X[$s] = true; }
        continue;
    }
    foreach ($slot_uses[$s] ?? [] as $u) { if ($u['ctx'] === 'intern') { $X[$s] = true; break; } }
}
for ($pass = 0; $pass < ($names_mode ? 0 : 50); $pass++) {
    $changed = 0;
    foreach ($cand as $s => $_) {
        if ($delta($s) < -1e-9) {
            if (isset($X[$s])) { unset($X[$s]); } else { $X[$s] = true; }
            $changed++;
        }
    }
    if ($changed === 0) { break; }
}
foreach ($cand as $s => $_) { if (!isset($X[$s])) { $why['no gain'] = ($why['no gain'] ?? 0) + 1; } }

// edits
$edits = []; // file => list of [s, e, text]
$add = static function (string $file, int $s, int $e, string $t) use (&$edits): void { $edits[$file][] = [$s, $e, $t]; };
$wrap = static function (string $file, array $r, string $fn) use ($add): void { $add($file, $r[0], $r[0], "Interner::$fn("); $add($file, $r[1], $r[1], ')'); };
$src_cache = [];
$text = static function (string $file, array $r) use (&$src_cache): string {
    $src_cache[$file] ??= file_get_contents($file);
    return substr($src_cache[$file], $r[0], $r[1] - $r[0]);
};
$n_intern_removed = 0; $n_intern_added = 0; $n_lookup = 0; $used_syms = [];
foreach (array_keys($X) as $s) {
    $d = $decl[$s];
    if ($d['type'] !== null) { $add($d['file'], $d['type'][0], $d['type'][1], 'int'); }
    if ($d['doc'] !== null) { $add($d['file'], $d['doc'][0], $d['doc'][1], 'int'); }
    foreach ($slot_uses[$s] ?? [] as $u) {
        if ($u['ctx'] === 'intern') { $add($u['file'], $u['call'][0], $u['call'][1], $text($u['file'], $u['r'])); $n_intern_removed++; }
        else { $wrap($u['file'], $u['r'], 'lookup'); $n_lookup++; }
    }
}
foreach ($flows as $f) {
    $t = $f['slot']; $src = $f['src']; $file = $f['file'];
    $tin = isset($X[$t]);
    if ($src['k'] === 'null') { continue; }
    $sin = $src['k'] === 'slot' && isset($X[$src['slot']]);
    if ($tin) {
        if ($src['k'] === 'edit') {
            $add($file, ...$src['e']);
            if (preg_match('/^Sym::(\w+)$/', $src['e'][2], $m)) { $used_syms[$m[1]] = true; }
        } elseif (!$sin) { $wrap($file, $src['r'], 'intern'); $n_intern_added++; }
    } elseif ($sin) {
        $wrap($file, $src['r'], 'lookup'); $n_lookup++;
    }
}
$fh = fopen($out, 'w');
foreach ($edits as $file => $list) {
    fwrite($fh, json_encode(['kind' => 'edit', 'file' => $file, 'site' => $file, 'edits' => $list], JSON_UNESCAPED_SLASHES) . "\n");
}
foreach ($used_syms as $name => $_) {
    if (isset($syms[$name])) { fwrite($fh, json_encode(['kind' => 'sym', 'name' => $name, 'value' => $syms[$name]]) . "\n"); }
}
fclose($fh);
$byk = []; foreach (array_keys($X) as $s) { $byk[$s[0]] = ($byk[$s[0]] ?? 0) + 1; }
arsort($why);
printf("slots: %d candidates, %d converted (%s); intern calls removed %d, added %d; lookups added %d\n",
    count($decl), count($X), json_encode($byk), $n_intern_removed, $n_intern_added, $n_lookup);
foreach ($why as $k => $v) { printf("  not converted %5d  %s\n", $v, $k); }
