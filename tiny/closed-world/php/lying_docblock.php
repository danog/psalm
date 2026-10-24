<?php
namespace Cw\LyingDocblock;

/**
 * @param string $x
 * @return string
 */
function ld_double($x) { return $x * 2; }

class LdBox {
    /** @var string */
    public $v;
    /** @param array<string> $v */
    public function __construct($v) { $this->v = $v; }
    /** @return int */
    public function get() { return $this->v; }
}

function run_lying_docblock() {
    $d = ld_double(21);
    $b = new LdBox(1.5);
    /** @var string $f */
    $f = $b->get() + 1;
    return $d . ' ' . $f . ' ' . var_export(is_int($d), true) . "\n";
}
