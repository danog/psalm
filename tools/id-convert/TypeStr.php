<?php

declare(strict_types=1);

namespace Psalm\Tools\IdConvert;

/**
 * Docblock type strings with source ranges: finds the string-like atoms at an array path of a type and rewrites
 * them to `int`. Paths: "" (the value itself), "#k" (an array's keys), "#v" (its values), nested ("#v#k").
 *
 * Supported: unions `A|B`, intersections, `?A`, parentheses, generics `array<K, V>` / `list<V>` / `iterable<K, V>`,
 * shapes `array{a: X, b?: Y}` / `list{X, Y}`, `X[]`, callables `callable(A): B` / `Closure(A): B`, literals.
 */
final class TypeStr
{
    /** atoms holding a name as a string */
    public const STRINGISH = [
        'string', 'lowercase-string', 'non-empty-lowercase-string', 'non-empty-string', 'non-falsy-string',
        'truthy-string', 'class-string', 'interface-string', 'enum-string', 'trait-string', 'callable-string',
        'literal-string', 'non-empty-literal-string',
    ];

    private string $s;
    private int $i = 0;

    private function __construct(string $s)
    {
        $this->s = $s;
    }

    /**
     * A parsed union: list of atoms. An atom: ['kind' => name|literal|shape|group, 'name', 'start', 'end',
     * 'params' => list<union>, 'items' => list<union> (shape values), 'array' => depth of `[]`, 'nullable'].
     *
     * @return array{atoms: list<array<string, mixed>>, start: int, end: int}
     */
    public static function parse(string $type): array
    {
        $p = new self($type);
        $u = $p->union();
        return $u;
    }

    private function ws(): void
    {
        while ($this->i < strlen($this->s) && ctype_space($this->s[$this->i])) {
            $this->i++;
        }
    }

    private function peek(): string
    {
        $this->ws();
        return $this->s[$this->i] ?? '';
    }

    /** @return array{atoms: list<array<string, mixed>>, start: int, end: int} */
    private function union(): array
    {
        $this->ws();
        $start = $this->i;
        $atoms = [$this->atom()];
        while (in_array($this->peek(), ['|', '&'], true)) {
            $this->i++;
            $atoms[] = $this->atom();
        }
        $end = $this->i;
        while ($end > $start && ctype_space($this->s[$end - 1])) {
            $end--;
        }
        return ['atoms' => $atoms, 'start' => $start, 'end' => $end];
    }

    /** @return array<string, mixed> */
    private function atom(): array
    {
        $this->ws();
        $start = $this->i;
        $c = $this->s[$this->i] ?? '';
        if ($c === '?') {
            $this->i++;
            $a = $this->atom();
            $a['nullable'] = true;
            $a['start_all'] = $start;
            return $a;
        }
        if ($c === '(') {
            $this->i++;
            $u = $this->union();
            if ($this->peek() === ')') {
                $this->i++;
            }
            $a = ['kind' => 'group', 'union' => $u, 'start' => $start, 'end' => $this->i];
            return $this->suffix($a);
        }
        if ($c === '"' || $c === "'") {
            $q = $c;
            $this->i++;
            while ($this->i < strlen($this->s) && $this->s[$this->i] !== $q) {
                $this->i += $this->s[$this->i] === '\\' ? 2 : 1;
            }
            $this->i++;
            return $this->suffix(['kind' => 'literal', 'name' => substr($this->s, $start, $this->i - $start), 'start' => $start, 'end' => $this->i]);
        }
        // a name (with `\`, `-`, `::`, `$`, digits, `.` for floats, `*` for wildcards)
        while ($this->i < strlen($this->s) && preg_match('/[A-Za-z0-9_\\\\\-\$\.\*:]/', $this->s[$this->i])) {
            // `::` is part of a name; a lone `:` ends it (callable return)
            if ($this->s[$this->i] === ':' && ($this->s[$this->i + 1] ?? '') !== ':' && ($this->s[$this->i - 1] ?? '') !== ':') {
                break;
            }
            $this->i++;
        }
        $name = substr($this->s, $start, $this->i - $start);
        $a = ['kind' => 'name', 'name' => $name, 'start' => $start, 'end' => $this->i, 'params' => [], 'items' => []];
        if (($this->s[$this->i] ?? '') === '<') {
            $this->i++;
            $a['params'][] = $this->union();
            while ($this->peek() === ',') {
                $this->i++;
                $a['params'][] = $this->union();
            }
            if ($this->peek() === '>') {
                $this->i++;
            }
            $a['end'] = $this->i;
        } elseif (($this->s[$this->i] ?? '') === '{') {
            $a['kind'] = 'shape';
            $this->i++;
            while (!in_array($this->peek(), ['}', ''], true)) {
                // `key?: Type` or `Type`
                $save = $this->i;
                if (preg_match('/\G\s*(?:[A-Za-z0-9_\-]+|\'[^\']*\'|"[^"]*")\??\s*:(?!:)/', $this->s, $m, 0, $this->i)) {
                    $this->i += strlen($m[0]);
                } else {
                    $this->i = $save;
                }
                if ($this->peek() === '.') {
                    // `...` (unsealed shape)
                    while ($this->peek() === '.') {
                        $this->i++;
                    }
                    if ($this->peek() === '<') {
                        $this->i++;
                        $a['params'][] = $this->union();
                        while ($this->peek() === ',') {
                            $this->i++;
                            $a['params'][] = $this->union();
                        }
                        if ($this->peek() === '>') {
                            $this->i++;
                        }
                    }
                } else {
                    $a['items'][] = $this->union();
                }
                if ($this->peek() === ',') {
                    $this->i++;
                }
            }
            if ($this->peek() === '}') {
                $this->i++;
            }
            $a['end'] = $this->i;
        } elseif (($this->s[$this->i] ?? '') === '(') {
            // callable(...): R
            $a['kind'] = 'callable';
            $this->i++;
            while (!in_array($this->peek(), [')', ''], true)) {
                $a['items'][] = $this->union();
                while (in_array($this->peek(), ['=', '.'], true) || $this->peek() === '&') {
                    $this->i++;
                }
                if (preg_match('/\G\s*\$[A-Za-z_]\w*/', $this->s, $m, 0, $this->i)) {
                    $this->i += strlen($m[0]);
                }
                if ($this->peek() === ',') {
                    $this->i++;
                }
            }
            if ($this->peek() === ')') {
                $this->i++;
            }
            if ($this->peek() === ':') {
                $this->i++;
                $a['return'] = ['atoms' => [$this->atom()], 'start' => 0, 'end' => 0];
            }
            $a['end'] = $this->i;
        }
        return $this->suffix($a);
    }

    /** `X[]` suffixes. @param array<string, mixed> $a @return array<string, mixed> */
    private function suffix(array $a): array
    {
        $a['array'] = 0;
        while (substr($this->s, $this->i, 2) === '[]') {
            $this->i += 2;
            $a['array']++;
        }
        $a['end_all'] = $this->i;
        return $a;
    }

    public static function isStringish(array $atom): bool
    {
        if (($atom['array'] ?? 0) > 0) {
            return false;
        }
        if ($atom['kind'] === 'literal') {
            return $atom['name'][0] === '"' || $atom['name'][0] === "'";
        }
        if ($atom['kind'] !== 'name') {
            return false;
        }
        return in_array(strtolower($atom['name']), self::STRINGISH, true);
    }

    /**
     * The unions reachable at a path (a union is the value type at that position).
     *
     * @param array{atoms: list<array<string, mixed>>, start: int, end: int} $u
     * @return list<array{atoms: list<array<string, mixed>>, start: int, end: int}>
     */
    public static function at(array $u, string $path): array
    {
        if ($path === '') {
            return [$u];
        }
        $step = substr($path, 0, 2);
        $rest = substr($path, 2);
        $out = [];
        foreach ($u['atoms'] as $a) {
            foreach (self::stepAtom($a, $step) as $sub) {
                foreach (self::at($sub, $rest) as $r) {
                    $out[] = $r;
                }
            }
        }
        return $out;
    }

    /** @return list<array{atoms: list<array<string, mixed>>, start: int, end: int}> */
    private static function stepAtom(array $a, string $step): array
    {
        if (($a['array'] ?? 0) > 0) {
            // X[] (X[][]: one level peeled)
            if ($step === '#k') {
                return [];
            }
            $inner = $a;
            $inner['array']--;
            $inner['end_all'] -= 2;
            return [['atoms' => [$inner], 'start' => $a['start'], 'end' => $a['end']]];
        }
        if ($a['kind'] === 'group') {
            $out = [];
            foreach ($a['union']['atoms'] as $x) {
                $out = [...$out, ...self::stepAtom($x, $step)];
            }
            return $out;
        }
        $n = strtolower($a['name'] ?? '');
        if ($a['kind'] === 'shape') {
            return $step === '#v' ? [...$a['items'], ...array_slice($a['params'], -1)] : (count($a['params']) === 2 ? [$a['params'][0]] : []);
        }
        if ($a['kind'] !== 'name' || $a['params'] === []) {
            return [];
        }
        if (in_array($n, ['list', 'non-empty-list'], true)) {
            return $step === '#v' ? [$a['params'][0]] : [];
        }
        if (in_array($n, ['array', 'non-empty-array', 'iterable', 'traversable', 'arrayaccess', 'arrayobject', 'arrayiterator', 'splobjectstorage', 'generator', 'iterator', 'iteratoraggregate'], true)) {
            if (count($a['params']) === 1) {
                return $step === '#v' ? [$a['params'][0]] : [];
            }
            return $step === '#k' ? [$a['params'][0]] : [$a['params'][1]];
        }
        return [];
    }

    /**
     * Edits turning the string-like atoms at `$path` into `int` (a union whose every non-null atom is string-like
     * becomes `int` / `int|null` / `?int` as a whole).
     *
     * @return list<array{int, int, string}> [start, end, text] relative to the type string
     */
    public static function toIntAt(string $type, string $path): array
    {
        $u = self::parse($type);
        $edits = [];
        foreach (self::at($u, $path) as $target) {
            $atoms = $target['atoms'];
            $str = array_filter($atoms, self::isStringish(...));
            if ($str === []) {
                continue;
            }
            $nonNull = array_filter($atoms, static fn(array $a): bool => !($a['kind'] === 'name' && strtolower($a['name']) === 'null'));
            if (count($str) === count($nonNull)) {
                // the whole union: int, keeping a null member / `?`
                $hasNull = count($nonNull) !== count($atoms);
                $first = $atoms[0];
                $lead = isset($first['nullable']) && count($atoms) === 1 ? '?' : '';
                $s = $first['start_all'] ?? $first['start'];
                $e = end($atoms)['end_all'];
                $edits[] = [$s, $e, $lead . 'int' . ($hasNull ? '|null' : '')];
                continue;
            }
            foreach ($str as $a) {
                $edits[] = [$a['start'], $a['end_all'], 'int'];
            }
        }
        return $edits;
    }

    public static function apply(string $type, array $edits): string
    {
        usort($edits, static fn($a, $b) => $b[0] <=> $a[0]);
        foreach ($edits as [$s, $e, $t]) {
            $type = substr($type, 0, $s) . $t . substr($type, $e);
        }
        return $type;
    }
}
