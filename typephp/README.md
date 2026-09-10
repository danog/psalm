# Compiling Psalm with TypePHP

[TypePHP](https://github.com/swoole/typephp) is an ahead-of-time compiler that
translates PHP into C++ and native binaries. Psalm's own sources (`src/`) are
kept free of the PHP features TypePHP rejects; this directory holds the build
configuration and the helper used to satisfy vendor dependencies.

## Rules the sources follow

- No executable code at file scope: the `require_once` bootstrap lines live in
  the launcher scripts (`psalm`, `psalter`, ...), not in the CLI classes.
- A local variable keeps the native type of its first assignment. Locals that
  are later assigned a sibling class, or that are passed by reference, take
  their first value through `Psalm\Internal\TypePhp\Dynamic::any()`.
- References to objects are not supported: object by-reference parameters are
  declared `mixed &$param` with `@param`/`@param-out` docblocks.
- Every `switch` case ends in `break`/`return`/`continue`/`throw`; intentional
  fallthrough is written out explicitly.
- No `$GLOBALS`, no `list()` in loop conditions or `foreach` targets, no `~` on
  strings (`BitwiseNotAnalyzer::bitwiseNotString()`), no `|=` on typed
  properties.

## Building

1. Install TypePHP (`composer install` in its checkout, then build phpx with
   `cmake -S vendor/swoole/phpx -B vendor/swoole/phpx/build && cmake --build vendor/swoole/phpx/build`)
   and a PHP 8.4/8.5 embed SAPI (`./configure --enable-embed=shared ...`);
   point `PHP_HOME` at that PHP prefix.
2. Generate compilable skeletons of the vendor classes Psalm extends or
   implements (the compiler needs their declarations; the real vendor code is
   not compiled yet):

       php typephp/gen-vendor-stubs.php typephp/vendor-classes.txt typephp/vendor-stubs

3. Generate C++ (`--dry`) or build the binary:

       php /path/to/typephp/bin/tpc.php typephp/project.yml --dry --build-dir /tmp/psalm-typephp
       php /path/to/typephp/bin/tpc.php typephp/project.yml -o psalm-native --build-dir /tmp/psalm-typephp -j 8

## Status and known compiler issues (TypePHP v0.8.1)

All of `src/` passes TypePHP's front end, and the full build compiles and links
a native `psalm` binary (1,356 translation units on macOS/arm64). Running it
is blocked on the vendor dependencies: the skeletons declare the classes but
their methods throw, so the real vendor code (php-parser, amphp, symfony
console, ...) has to be made compilable next.

Compiler-side issues found while developing this, worked around locally:

- `phpx.h` uses `std::abort`/`std::fill` without including `<cstdlib>` and
  `<algorithm>`, which fails with recent libc++.
- `gen_stub.php` resolves class names in constant expressions relative to the
  current namespace and ignores `use` imports (`Mutations::LEVEL_ALL` in
  parameter defaults), fixed by consulting the resolved name attribute.
- Reads of typed `int` properties are emitted as `Variant` and cannot be
  assigned to native `php::Int` locals or properties; `CodeLocation` and
  `ProtocolStreamReader` carry small workarounds for this.
- The property read cache (`_object_prop_*`) is declared inside a lowered
  ternary lambda and used outside it; the affected reads were hoisted into
  locals in `ClassAnalyzer` and `ReturnTypeAnalyzer`.
