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
        case 'decl': if (!str_contains($r['slot'], 'psalm\\internal\\interner')) { $decl[$r['slot']] = $r; } break;
        case 'flow':
            // the interner is the boundary itself, not a slot (its uses are the `intern` facts)
            if (str_starts_with($r['slot'], 'P:psalm\\internal\\interner::')) { break; }
            $flows[$r['file'] . $r['slot'] . json_encode($r['src'])] = $r; break;
        case 'use': $uses[$r['file'] . $r['slot'] . json_encode($r['r']) . $r['ctx']] = $r; break;
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
$pure = static fn(string $t): bool => (bool) preg_match('/^\$[A-Za-z_]\w*(->\w+|\[\$?[\w\']+\])*$/', trim($t));
$src_cache0 = [];
$text0 = static function (string $file, array $r) use (&$src_cache0): string {
    $src_cache0[$file] ??= file_get_contents($file);
    return substr($src_cache0[$file], $r[0], $r[1] - $r[0]);
};
$in_flows = []; $out_flows = []; $slot_uses = [];
foreach ($flows as $f) {
    $in_flows[$f['slot']][] = $f;
    if ($f['src']['k'] === 'slot') { $out_flows[$f['src']['slot']][] = $f; }
}
foreach ($uses as $u) { $slot_uses[$u['slot']][] = $u; }
// a slot adjacent (by a flow) to a converted one
$neighbour = static function (string $s) use (&$X, $in_flows, $out_flows): bool {
    foreach ($in_flows[$s] ?? [] as $f) { if ($f['src']['k'] === 'slot' && isset($X[$f['src']['slot']])) { return true; } }
    foreach ($out_flows[$s] ?? [] as $f) { if (isset($X[$f['slot']])) { return true; } }
    return false;
};
// objective: total string work after the change (0 = today). Local search by toggling slots.
$cand = $X; $X = [];
$delta = static function (string $s) use (&$X, $in_flows, $out_flows, $slot_uses): float {
    // change of the total when $s flips membership
    $was = isset($X[$s]);
    $cost = static function (bool $in) use ($s, &$X, $in_flows, $out_flows, $slot_uses): float {
        $c = 0.0;
        if ($in) {
            foreach ($slot_uses[$s] ?? [] as $u) { $c += $u['ctx'] === 'intern' ? -1.0 : (in_array($u['ctx'], ['truthy', 'nullcmp'], true) ? 0.0 : 0.3); }
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
// --seeds=<file>: convert exactly these slots (one per line; a trailing * converts every slot with that prefix), then
// grow along flows where the neighbour's conversion removes more interns than it adds
$seed_file = null;
foreach ($argv as $a) { if (str_starts_with($a, '--seeds=')) { $seed_file = substr($a, 8); } }
// pzoom's target state: every class-like / member name slot is an id (strings only at resolution boundaries)
$nameish = '/(^|_)(fq|fqn|fqcn|fqcln|class|classes|classlike|classlikes|interface|trait|enum|parent|self|static|declaring|appearing|'
    . 'implementing|extended|mixin|method|methods|function|functions|property|prop|const|case)(_|$)|_name$|^name$|_name_lc$|name$/i';
if ($seed_file !== null) {
    $seeds = array_filter(array_map('trim', file($seed_file)));
    foreach ($cand as $s => $_) {
        foreach ($seeds as $sd) {
            if ($s === $sd || (str_ends_with($sd, '*') && str_starts_with($s, substr($sd, 0, -1)))) { $X[$s] = true; }
        }
    }
    foreach ($seeds as $sd) {
        if (!str_ends_with($sd, '*') && !isset($X[$sd])) {
            fwrite(STDERR, "seed not a candidate: $sd" . (isset($decl[$sd]) ? ' (' . ($decl[$sd]['why'] ?? '') . ')' : ' (no decl)') . (isset($block[$sd]) ? ' blocked: ' . $block[$sd] : '') . "\n");
        }
    }
    $seeded = $X;
}
foreach ($cand as $s => $_) {
    if ($seed_file !== null) { continue; }
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
        if ($seed_file !== null && (isset($seeded[$s]) || (!isset($X[$s]) && !$neighbour($s)))) { continue; }
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
$wrap = static function (string $file, array $r, string $fn, bool $nullable = false) use ($add, $text0): void {
    if ($nullable) {
        $add($file, $r[0], $r[0], 'Interner::' . $fn . 'OrNull('); $add($file, $r[1], $r[1], ')');
        return;
    }
    $add($file, $r[0], $r[0], "Interner::$fn("); $add($file, $r[1], $r[1], ')');
};
$src_cache = [];
$text = static function (string $file, array $r) use (&$src_cache): string {
    $src_cache[$file] ??= file_get_contents($file);
    return substr($src_cache[$file], $r[0], $r[1] - $r[0]);
};
$n_intern_removed = 0; $n_intern_added = 0; $n_lookup = 0; $used_syms = []; $fallbacks = [];
foreach (array_keys($X) as $s) {
    $d = $decl[$s];
    $nl = !empty($d['nullable']);
    if ($d['type'] !== null) { $add($d['file'], $d['type'][0], $d['type'][1], $nl ? '?int' : 'int'); }
    if ($d['doc'] !== null) { $add($d['file'], $d['doc'][0], $d['doc'][1], $nl ? '?int' : 'int'); }
    foreach ($slot_uses[$s] ?? [] as $u) {
        if ($u['ctx'] === 'intern') { $add($u['file'], $u['call'][0], $u['call'][1], $text($u['file'], $u['r'])); $n_intern_removed++; }
        elseif ($u['ctx'] === 'truthy' && $nl) { $add($u['file'], $u['r'][0], $u['r'][1], '(' . $text($u['file'], $u['r']) . ' !== null)'); }
        elseif ($u['ctx'] === 'nullcmp') { /* unchanged: null exactly when the string was */ }
        elseif ($u['ctx'] === 'fallback' && $nl && ($u['nullable'] ?? true)) {
            $fallbacks[] = $u;
            $n_lookup++;
        }
        else { $wrap($u['file'], $u['r'], 'lookup', $nl && ($u['nullable'] ?? true)); $n_lookup++; }
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
        } elseif (!$sin) { $wrap($file, $src['r'], 'intern', !empty($src['nullable'])); $n_intern_added++; }
    } elseif ($sin) {
        $wrap($file, $src['r'], 'lookup', !empty($decl[$src['slot']]['nullable']) && ($src['nullable'] ?? true)); $n_lookup++;
    }
}
// fallback rewrites copy their fallback's text: only where no other edit lands inside the expression
foreach ($fallbacks as $u) {
    $inside = false;
    foreach ($edits[$u['file']] ?? [] as [$es, $ee]) {
        if ($es >= $u['expr'][0] && $ee <= $u['expr'][1] && !($es >= $u['r'][0] && $ee <= $u['r'][1])) { $inside = true; }
    }
    $rt = $text($u['file'], $u['r']);
    if ($inside) {
        $wrap($u['file'], $u['r'], 'lookup', true);
    } else {
        $add($u['file'], $u['expr'][0], $u['expr'][1], "(Interner::lookupOrNull($rt) ?? " . $text($u['file'], $u['alt']) . ')');
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
