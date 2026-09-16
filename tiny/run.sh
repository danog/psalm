#!/bin/bash
# Fast standalone transpile->compile->run loop.
# Edit tiny/php/*.php, then: bash tiny/run.sh
# Transpiles php/ (+ runtime-stubs) into a small crate and runs the driver.
set -e
cd "$(dirname "$0")"
OUT="$(pwd)/rust/generated/tiny_repro"

echo "=== TRANSPILE ==="
php -d memory_limit=4000M ../psalm -c psalm-tiny.xml --no-cache --no-progress \
  --threads=1 --transpile-rust="$OUT" 2>&1 \
  | grep -E "wrote |Transpiler crashed|Fatal error|Uncaught" || true

# Keep php-rt path pointed at the vendored copy (transpiler re-emits an absolute-ish path each run).
sed -i 's#^php-rt = .*#php-rt = { path = "../../php-rt" }#' "$OUT/Cargo.toml"

echo "=== BUILD + RUN ==="
cd rust
cargo run -p driver 2>&1 | grep -E "^error|error\[|run\(\) =|panicked|could not compile|Finished" | head -30
