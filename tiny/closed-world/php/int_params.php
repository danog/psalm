<?php
namespace Cw\IntParams;

// untyped functions only ever called with ints: the parameters (and the returns) are i64
function ip_add($a, $b) { return $a + $b; }
function ip_twice($n) { return ip_add($n, $n); }
function ip_sum_to($n) { return $n <= 0 ? 0 : $n + ip_sum_to($n - 1); }
function ip_pass($x) { return $x; }
function ip_count_down($n, $acc) {
    if ($n === 0) { return $acc; }
    return ip_count_down($n - 1, $acc . $n);
}

function run_int_params() {
    $t = ip_twice(21);
    $s = ip_sum_to(10);
    $p = ip_pass(ip_pass(5));
    return ip_add($t, 1) . ',' . $s . ',' . $p . ',' . ip_count_down(3, '') . "\n";
}
