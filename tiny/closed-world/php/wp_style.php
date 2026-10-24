<?php
namespace Cw\WpStyle;

class Post { }

function wp_parse_args($args, $defaults = array()) {
    if (is_object($args)) { $r = get_object_vars($args); }
    elseif (is_array($args)) { $r =& $args; }
    else { parse_str($args, $r); }
    if (is_array($defaults) && $defaults) { return array_merge($defaults, $r); }
    return $r;
}

function add_filter($hook, $cb, $prio = 10) {
    global $wp_filter;
    $wp_filter[$hook][$prio][] = $cb;
    return true;
}

function apply_filters($hook, $value) {
    global $wp_filter;
    $args = func_get_args();
    array_shift($args);
    if (!isset($wp_filter[$hook])) return $value;
    ksort($wp_filter[$hook]);
    foreach ($wp_filter[$hook] as $prio => $cbs) {
        foreach ($cbs as $cb) { $args[0] = $value = call_user_func_array($cb, $args); }
    }
    return $value;
}

function upper($s) { return strtoupper($s); }

function run_wp_style() {
    $p = new Post();
    $p->title = 'hello';
    $p->id = '42';
    $out = '';
    $a = wp_parse_args('a=1&b=2', ['c' => 3]);
    foreach ($a as $k => $v) { $out .= "$k=$v,"; }
    add_filter('t', 'Cw\WpStyle\upper');
    add_filter('t', function ($s) { return $s . '!'; }, 5);
    $out .= apply_filters('t', $p->title);
    $x = $p->id + 1;
    $out .= "|$x|";
    $name = 'dyn';
    $$name = 7;
    $out .= $dyn;
    extract(['q' => 'Q']);
    $out .= $q;
    $arr = [1, 2, 3];
    foreach ($arr as &$v) { $v *= 2; }
    unset($v);
    $out .= implode(',', $arr);
    return $out . "\n";
}
