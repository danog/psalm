# Compiling Psalm with TypePHP

[TypePHP](https://github.com/swoole/typephp) is an ahead-of-time compiler that
translates PHP into C++ and native binaries. This directory holds the build
configuration and the generator that assembles the closed-world source set.

The build uses the [danog/typephp](https://github.com/danog/typephp) and
[danog/phpx](https://github.com/danog/phpx) forks, which carry the compiler
fixes listed at the end of this file.

## Closed world

Psalm and all of its dependencies are compiled into the binary; no part of
Psalm itself is loaded at runtime:

- The dictionaries (`dictionaries/*.php`) are embedded as generated classes in
  `src/Psalm/Internal/Dictionaries/` and read through
  `Psalm\Internal\Codebase\Dictionaries`. Regenerate them with
  `bin/generate-dictionaries.php` after editing a dictionary
  (`--check` verifies they are up to date).
- Composer autoloaders of the analysed project are not executed. The generated
  `vendor/composer/autoload_*.php` maps are evaluated statically
  (`Psalm\Internal\Autoload\ComposerAutoloadFileEvaluator`) and turned into a
  `ComposerClassLocator`, which resolves class files and, under plain PHP,
  registers an autoloader for plugin classes.
- Forked workers run `Psalm\Internal\Fork\TaskRunner` instead of requiring
  amphp's `task-runner.php`.
- Code of the analysed project that Psalm executes (plugins, the `autoloader`
  config attribute, stubs pulled in through the project's autoloader) is
  loaded through `Psalm\Internal\CodeLoader`, the only class containing
  `require`; the native binary compiles it as a runtime include, exactly like
  plain PHP. Nothing of Psalm's own code or dependencies is loaded that way.
- The plain-PHP launchers (`psalm`, `psalter`, ...) load Psalm's own Composer
  autoloader before calling the CLI class.

## Rules the sources follow

- No executable code at file scope.
- A local variable keeps the native type of its first assignment. The forked
  compiler regenerates a function with a dynamic local when it detects
  conflicting assignments; `Psalm\Internal\TypePhp\Dynamic::any()` remains the
  explicit way to request dynamic storage.
- References to objects are not supported: object by-reference parameters are
  declared `mixed &$param` with `@param`/`@param-out` docblocks.
- No `$GLOBALS` or `global`, no `list()` in loop conditions or `foreach`
  targets, no `~` on strings, no `|=` on typed properties.

## Building

1. Check out the forks and build them:

       git clone https://github.com/danog/typephp /path/to/typephp
       cd /path/to/typephp && composer install
       cmake -S vendor/swoole/phpx -B vendor/swoole/phpx/build && cmake --build vendor/swoole/phpx/build

   Build a PHP 8.5 embed SAPI (`./configure --enable-embed=shared ...` with
   tokenizer, ctype, mbstring, filter, libxml/dom/simplexml/xml*, phar, pcntl,
   posix, iconv, openssl, zlib, sockets, gmp and intl) and point `PHP_HOME` at
   its prefix.

2. Generate the closed-world project file. Run the generator with the embed
   PHP so that `function_exists()`/`extension_loaded()` conditions in vendor
   code are evaluated for the target runtime:

       $PHP_HOME/bin/php typephp/gen-vendor-build.php

   The generator

   - indexes every file of the runtime vendor packages
     (`typephp/runtime-packages.txt`) and keeps only the files reachable by
     name from `src/`, `typephp/main.php` and `typephp/vendor-extra/`;
   - rewrites vendor files with file-scope code (`typephp/vendor-overrides/`,
     generated): constant conditions such as `if (PHP_VERSION_ID >= ...)` or
     `if (!function_exists(...))` are evaluated, `require`, `class_alias()` and
     preload hints are dropped, `define()` becomes `const`;
   - uses the hand-written replacements in `typephp/vendor-manual/` where
     vendor code relies on unsupported features (`Closure::bind()`, private
     property shadowing, ...), and compiles the stubs in `typephp/vendor-extra/`
     for optional dependencies that are not installed;
   - writes `typephp/project.yml`.

3. Generate C++ only (`--dry`) or build the binary, using every core:

       php /path/to/typephp/bin/tpc.php typephp/project.yml --dry --build-dir /tmp/psalm-typephp
       php /path/to/typephp/bin/tpc.php typephp/project.yml -o psalm-native --build-dir /tmp/psalm-typephp \
           -j "$(sysctl -n hw.ncpu 2>/dev/null || nproc)"

   `TYPEPHP_COLLECT_ERRORS=<file>` (fork feature) makes the front end report
   every rejected construct instead of stopping at the first one.

## Running the test suite inside the binary

`psalm-native --typephp-run <script.php> [args]` runs an interpreted PHP
script on top of the compiled runtime. `typephp/run-tests.php` uses this to
run PHPUnit (interpreted, loaded through Composer) against the compiled Psalm:

    $PHP_HOME/bin/php typephp/gen-vendor-build.php --open-world
    php /path/to/typephp/bin/tpc.php typephp/project.yml -o psalm-native --build-dir /tmp/psalm-typephp-open -j 12
    ./psalm-native --typephp-run typephp/run-tests.php tests/ArgTest.php

`--open-world` sets the fork's `open-world` project option: classes are
registered without `final` and overridable methods are never devirtualized,
so test classes can extend (`TestConfig`) and mock the compiled classes the
same way `dg/bypass-finals` allows under plain PHP. Composer "files" entries
whose functions are compiled in are skipped by the runner.

## Status

The native binary (all of `src/` plus 1,122 reachable vendor files, 2,300
translation units) analyses projects with the same results as plain PHP:

- a sample project (taint analysis included): identical 9 issues;
- worker forks (`--threads=N`) work;
- plugins are loaded at runtime (psalm/plugin-phpunit, plugin-mockery);
- `psalm-native /path/to/psalter ...` runs the other launchers.

The PHPUnit suite is run inside the open-world build (see above) and compared
with plain PHP per test; the remaining differences are being worked through.

## Compiler fixes carried by the forks

danog/phpx:

- `phpx.h` includes `<cstdlib>`/`<algorithm>`.
- `typephp_assign_dim()`: PHP dimension-write semantics for dynamic containers
  (arrays in place, `ArrayAccess` objects through `offsetSet()`).

danog/typephp:

- `open-world` project option (see above); promoted constructor properties
  are written in the declaring class scope; first-class callables of
  namespaced functions.
- Files without `declare(strict_types=1)` get PHP's weak typing: scalar
  parameters and return values coerce instead of throwing.
- Array properties without a default are uninitialized (`??=` initializers,
  `isset()`); `$this->prop` of a non-final class uses the declaring class
  slot; nullsafe/isset/empty reads are silent; virtual properties of
  internal classes (DOM) go through the property handlers.
- A local whose scalar type changes (`$x = false; $x = 1;`) and locals passed
  to dynamic by-reference parameters get dynamic storage; call write-backs
  run before the enclosing assignment; runtime-dispatched calls with a scalar
  declared return type are converted.
- Runtime: the compiled module registers with permanent interned strings
  (OPcache), uninitialized property slots follow PHP semantics,
  `typephp_set_server_argv()`, warnings are attributed to the right function.
- Error collection mode (`TYPEPHP_COLLECT_ERRORS`), also during trait
  composition.
- Function generation is retried with a dynamic local when a local receives
  incompatible static types (sibling classes, `int` then `string`, a `foreach`
  key into a typed local, parameters included).
- A namespaced function and a class method with the same name no longer map
  to the same C++ symbol (`Amp\Future\await()` vs `Amp\Future::await()`).
- `switch` cases without a terminating statement fall through like PHP.
- `$x = &$source` binds a Closure-captured local to the source.
- `$this->readonlyObject[$k] = $v` is an `offsetSet()`, not a rebinding;
  property dimension writes follow PHP semantics for objects.
- `__DIR__`/`__FILE__` are resolved in constant expressions; class constant
  references honour `use` imports; the stub generator finds constants declared
  in other files.
- Static calls to methods inherited from internal classes
  (`PhpToken::tokenize()`) fall back to a runtime call.
- `__serialize(): never` is accepted; keyword-named user methods
  (`toDecimal()`) are ordinary calls; by-reference values can be assigned to
  typed properties.
- A local that receives a value of a wider/unknown class, a `null`/dynamic
  value into fixed storage, or that is passed by reference, degrades to
  dynamic storage instead of a silent conversion or a runtime cast.
- Calls whose callee is unknown at compile time pass dynamic locals and
  writable properties by reference (copy-in/copy-out), so by-reference
  parameters resolved at runtime work.
- Class constants of interfaces, `Iterator`/`IteratorAggregate` registration
  order, abstract static methods, methods inherited from internal classes,
  SPL `get_method` forwarding, surplus call arguments and PHP's null-offset
  semantics are handled as in PHP.
- Runtime (danog/phpx): `hash()` on PHP 8.5, `Reference` assignment/comparison
  operators, method-cache invalidation for forwarded methods, quiet list
  destructuring.
