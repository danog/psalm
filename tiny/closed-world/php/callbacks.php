<?php
namespace Cw\Callbacks;

// callables resolved from their values: literal names, `[$obj, 'method']`, closures by identity; the targets'
// untyped parameters receive what the dynamic calls (and the builtins that call them) pass
function cb_double($x) { return $x * 2; }
function cb_greet($name) { return 'hi ' . $name; }
class CbSorter { public function cmp($a, $b) { return $a <=> $b; } }

function run_callbacks() {
    $nums = [3, 1, 2];
    $d = array_map('Cw\Callbacks\cb_double', $nums);
    $s = new CbSorter();
    usort($d, [$s, 'cmp']);
    $f = function ($v) { return $v + 1; };
    $e = array_map($f, $d);
    $g = 'Cw\Callbacks\cb_greet';
    $out = implode(',', $d) . ' ' . implode(',', $e) . ' ' . $g('bob') . ' ' . call_user_func_array($f, [41]);
    $fs = [$f, fn($v) => $v * 10];
    foreach ($fs as $h) {
        $out .= ' ' . $h(5);
    }
    return $out . "\n";
}
