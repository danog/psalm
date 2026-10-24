<?php
namespace Cw\StrProps;

// untyped properties only ever written with strings are Str fields; a property written with ints is i64
class SpUser {
    public $name;
    public $email = '';
    var $visits = 0;
    public function __construct($name) { $this->name = $name; }
    public function greet() { return 'Hi ' . $this->name . ' <' . $this->email . '>'; }
    public function visit() { $this->visits++; return $this; }
}

function run_str_props() {
    $u = new SpUser('ann');
    $u->email = 'ann@example.org';
    $u->visit()->visit();
    return $u->greet() . ' ' . $u->visits . "\n";
}
