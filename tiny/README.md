# Tiny standalone transpile→compile→run loop + regression suite

Fast (~2-10s) local iteration on the transpiler, without the full psalm build.

## Usage
- Edit `php/Cases.php`: add a `case_<feature>()` returning a string, then add a
  `check('<name>', case_<feature>(), '<expected>')` line to `run_all()`.
- `bash run.sh` — transpiles `php/` (+ runtime-stubs) into the `tiny_repro`
  crate, runs the driver, prints one PASS/FAIL line per case, and exits
  non-zero if any case fails or the crate doesn't compile.

`run_all()` is the dataprovider: every transpiler feature has a case there so
the loop catches regressions in one build.

## One-time setup (php-rt must match the current transpiler)
The generated code targets the php-rt API the transpiler currently emits.
Vendor the matching php-rt into `rust/php-rt/` before first run:

    rsync -a --delete --exclude=target ct:/root/psalm-build/rust/php-rt/ rust/php-rt/

(or from local `../rust/php-rt` once that is up to date). Refresh it whenever
php-rt's API changes.

## Layout
- `php/Cases.php` — the regression suite (fixtures + `case_*()` + `run_all()`).
- `psalm-tiny.xml` — minimal transpile config (runtime-stubs + `php/`).
- `rust/driver/` — hand-written `main` that calls `tiny_repro::g::run_all()`.
- `rust/php-rt/`, `rust/generated/` — gitignored reproducible artifacts.
