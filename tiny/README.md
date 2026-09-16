# Tiny standalone transpile→compile→run loop

Fast (~2-10s) local iteration on the transpiler, without the full psalm build.

## Usage
1. Edit `php/*.php` (small PHP exercising the transpiler feature under test).
2. Expose an entry point: a global function returning a printable value,
   called from `rust/driver/src/main.rs`.
3. `bash run.sh` — transpiles `php/` (+ runtime-stubs) into the `tiny_repro`
   crate and runs the driver.

## One-time setup (php-rt must match the current transpiler)
The generated code targets the php-rt API the transpiler currently emits.
Vendor the matching php-rt into `rust/php-rt/` before first run:

    rsync -a --delete --exclude=target ct:/root/psalm-build/rust/php-rt/ rust/php-rt/

(or from local `../rust/php-rt` once that is up to date). Refresh it whenever
php-rt's API changes.
