<?php
namespace Cw\Methods;

// a hierarchy: parameters of overriding methods receive what calls on the base type pass
abstract class MShape {
    abstract public function area($scale);
    public function describe($prefix) { return $prefix . ':' . $this->area(2); }
}
class MSquare extends MShape {
    private $side;
    public function __construct($side) { $this->side = $side; }
    public function area($scale) { return $this->side * $this->side * $scale; }
}
class MCircle extends MShape {
    private $r;
    public function __construct($r) { $this->r = $r; }
    public function area($scale) { return (int) (3 * $this->r * $this->r) * $scale; }
}

function m_total($shapes) {
    $t = 0;
    foreach ($shapes as $s) { $t += $s->area(1); }
    return $t;
}

function run_methods() {
    $shapes = [new MSquare(3), new MCircle(2)];
    $out = m_total($shapes) . ' ';
    foreach ($shapes as $s) { $out .= $s->describe('s') . ' '; }
    return $out . "\n";
}
