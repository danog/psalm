<?php
// Generates compilable class skeletons (no bodies) for vendor classes, via reflection.
require __DIR__ . '/../vendor/autoload.php';
$names = array_filter(array_map('trim', file($argv[1])));
$out = $argv[2];
$done = [];
$queue = array_values($names);
function typeToString(?ReflectionType $t): string {
    if ($t === null) return '';
    if ($t instanceof ReflectionNamedType) {
        $n = $t->getName();
        $builtin = $t->isBuiltin() || in_array($n, ['self','static','parent'], true);
        $s = ($t->allowsNull() && $n !== 'mixed' && $n !== 'null' ? '?' : '') . ($builtin ? $n : '\\' . $n);
        return $s;
    }
    if ($t instanceof ReflectionUnionType) return implode('|', array_map(fn($x) => typeToString($x), $t->getTypes()));
    if ($t instanceof ReflectionIntersectionType) return implode('&', array_map(fn($x) => typeToString($x), $t->getTypes()));
    return '';
}
function defaultToString(ReflectionParameter $p): string {
    if (!$p->isDefaultValueAvailable()) return '';
    if ($p->isDefaultValueConstant()) { $c = $p->getDefaultValueConstantName(); if (preg_match('/^(self|static|parent)::/i', $c)) return ' = ' . $c; return ' = ' . '\\' . ltrim($c, '\\'); }
    return ' = ' . var_export($p->getDefaultValue(), true);
}
function params(ReflectionFunctionAbstract $m): string {
    $ps = [];
    foreach ($m->getParameters() as $p) {
        $ps[] = trim(typeToString($p->getType()) . ' ' . ($p->isPassedByReference() ? '&' : '') . ($p->isVariadic() ? '...' : '') . '$' . $p->getName() . defaultToString($p));
    }
    return implode(', ', $ps);
}
while ($queue) {
    $name = array_shift($queue);
    $name = ltrim($name, '\\');
    if (isset($done[$name]) || !class_exists($name) && !interface_exists($name) && !trait_exists($name)) { $done[$name] = true; continue; }
    $r = new ReflectionClass($name);
    if ($r->isInternal()) { $done[$name] = true; continue; }
    $done[$name] = true;
    foreach (array_filter(array_merge([$r->getParentClass() ? $r->getParentClass()->getName() : null], $r->getInterfaceNames(), $r->getTraitNames())) as $dep) { if (!isset($done[$dep])) $queue[] = $dep; }
    $code = "<?php\n\ndeclare(strict_types=1);\n\nnamespace " . $r->getNamespaceName() . ";\n\n";
    if ($r->isEnum()) {
        $e = new ReflectionEnum($name);
        $code .= 'enum ' . $r->getShortName() . ($e->isBacked() ? ': ' . $e->getBackingType() : '');
        $ifaces = array_diff($r->getInterfaceNames(), ['UnitEnum', 'BackedEnum']);
        if ($ifaces) $code .= ' implements ' . implode(', ', array_map(fn($i) => '\\' . $i, $ifaces));
        $code .= "\n{\n";
        foreach ($e->getCases() as $case) { $code .= '    case ' . $case->getName() . ($e->isBacked() ? ' = ' . var_export($case->getBackingValue(), true) : '') . ";\n"; }
        foreach ($r->getMethods() as $m) {
            if ($m->getDeclaringClass()->getName() !== $name || in_array($m->getName(), ['cases','from','tryFrom'], true)) continue;
            $vis = $m->isPrivate() ? 'private' : ($m->isProtected() ? 'protected' : 'public');
            $ret = $m->hasReturnType() ? ': ' . typeToString($m->getReturnType()) : '';
            $code .= "    $vis " . ($m->isStatic() ? 'static ' : '') . 'function ' . $m->getName() . '(' . params($m) . ')' . $ret . "\n    {\n        throw new \\RuntimeException('vendor stub');\n    }\n";
        }
        $code .= "}\n";
        $file = $out . '/' . str_replace('\\', '/', $name) . '.php'; @mkdir(dirname($file), 0777, true); file_put_contents($file, $code); continue;
    }
    $kind = $r->isInterface() ? 'interface' : ($r->isTrait() ? 'trait' : (($r->isAbstract() ? 'abstract ' : ($r->isFinal() ? 'final ' : '')) . 'class'));
    $code .= "$kind " . $r->getShortName();
    if ($r->getParentClass()) $code .= ' extends \\' . $r->getParentClass()->getName();
    $ifaces = $r->isInterface() ? $r->getInterfaceNames() : array_diff($r->getInterfaceNames(), $r->getParentClass() ? $r->getParentClass()->getInterfaceNames() : []);
    // keep only the directly declared interfaces: PHP rejects re-extending an interface inherited through another one
    $ifaces = array_values(array_filter($ifaces, static function (string $i) use ($ifaces): bool {
        foreach ($ifaces as $j) {
            if ($j !== $i && in_array($i, (new ReflectionClass($j))->getInterfaceNames(), true)) return false;
        }
        return true;
    }));
    if ($ifaces) $code .= ($r->isInterface() ? ' extends ' : ' implements ') . implode(', ', array_map(fn($i) => '\\' . $i, $ifaces));
    $code .= "\n{\n";
    foreach ($r->getTraitNames() as $t) $code .= "    use \\$t;\n";
    foreach ($r->getReflectionConstants() as $c) {
        if ($c->getDeclaringClass()->getName() !== $name) continue;
        $vis = $c->isPrivate() ? 'private' : ($c->isProtected() ? 'protected' : 'public');
        $code .= "    $vis const " . $c->getName() . ' = ' . var_export($c->getValue(), true) . ";\n";
    }
    foreach ($r->getProperties() as $p) {
        if ($p->getDeclaringClass()->getName() !== $name || $r->isInterface()) continue;
        $vis = $p->isPrivate() ? 'private' : ($p->isProtected() ? 'protected' : 'public');
        $t = typeToString($p->getType());
        $def = $p->hasDefaultValue() && !$p->isPromoted() ? ' = ' . var_export($p->getDefaultValue(), true) : ($t === '' ? '' : ($p->hasType() && $p->getType()->allowsNull() && !$p->isReadOnly() ? ' = null' : ''));
        $code .= "    $vis " . ($p->isStatic() ? 'static ' : '') . ($p->isReadOnly() ? 'readonly ' : '') . trim($t . ' $' . $p->getName()) . $def . ";\n";
    }
    foreach ($r->getMethods() as $m) {
        if ($m->getDeclaringClass()->getName() !== $name) continue;
        $vis = $m->isPrivate() ? 'private' : ($m->isProtected() ? 'protected' : 'public');
        $ret = $m->hasReturnType() ? ': ' . typeToString($m->getReturnType()) : '';
        $sig = "    $vis " . ($m->isStatic() ? 'static ' : '') . (($m->isAbstract() && !$r->isInterface()) ? 'abstract ' : '') . 'function ' . ($m->returnsReference() ? '&' : '') . $m->getName() . '(' . params($m) . ')' . $ret;
        if ($r->isInterface() || $m->isAbstract()) { $code .= $sig . ";\n"; continue; }
        $rt = $m->hasReturnType() ? typeToString($m->getReturnType()) : '';
        if ($m->getName() === '__serialize') { $rt = 'array'; $sig = preg_replace('/: [^ ]+$/', ': array', $sig); }
        $defaults = ['array' => '[]', 'int' => '0', 'string' => "''", 'bool' => 'false', 'float' => '0.0', 'mixed' => 'null'];
        if ($rt === 'void' || $rt === 'never' || $rt === '') $body = "        throw new \\RuntimeException('vendor stub');";
        elseif (str_starts_with($rt, '?')) $body = "        return null;";
        elseif (isset($defaults[$rt])) $body = '        return ' . $defaults[$rt] . ';';
        else $body = "        throw new \\RuntimeException('vendor stub');";
        $code .= $sig . "\n    {\n" . $body . "\n    }\n";
    }
    $code .= "}\n";
    $file = $out . '/' . str_replace('\\', '/', $name) . '.php';
    @mkdir(dirname($file), 0777, true);
    file_put_contents($file, $code);
}
echo count(array_filter($done)) . " classes processed\n";
