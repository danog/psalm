<?php
namespace Cw\Soundness;

// writes the analysis must see for the inferred types to hold: by-reference out values, writes through
// untyped parameters, static properties, references to array items, `$o->$name = v`
class SBox { public $v; public static $count = 0; }
class SOther { public $v = 0; }

function s_set(&$out, $x) { $out = $x . '!'; }
function s_fill($o, $val) { $o->v = $val; }
function s_dyn($o, $name, $val) { $o->$name = $val; }

function run_soundness() {
    $n = 5;
    s_set($n, 3);
    $out = $n . '|';
    $b = new SBox();
    $b->v = 1;
    s_fill($b, 'str');
    $out .= $b->v . '|';
    SBox::$count = 2;
    SBox::$count .= 'x';
    $out .= SBox::$count . '|';
    $arr = [1, 2];
    $r = &$arr[0];
    $r = 'z';
    $out .= $arr[0] . '|';
    $o = new SOther();
    s_dyn($o, 'v', 3.5);
    $out .= $o->v . "\n";
    return $out;
}
