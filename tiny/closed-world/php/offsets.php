<?php
namespace Cw\Offsets;

// reading a key the array may not have gives null: the inferred types say so (and isset() guards narrow it away)
function of_lookup($map, $key) {
    $v = $map[$key];
    return $v === null ? 'none' : 'v' . $v;
}

function of_guarded($map, $key) {
    if (isset($map[$key])) {
        return $map[$key] * 2;
    }
    return -1;
}

function run_offsets() {
    $m = ['a' => 1, 'b' => 2];
    $out = of_lookup($m, 'a') . ' ' . of_lookup($m, 'zz') . ' ' . of_guarded($m, 'b') . ' ' . of_guarded($m, 'q');
    $sum = 0;
    foreach ($m as $k => $v) {
        $sum += $m[$k];
    }
    $list = [10, 20, 30];
    for ($i = 0; $i < count($list); $i++) {
        $sum += $list[$i];
    }
    return $out . ' ' . $sum . "\n";
}
