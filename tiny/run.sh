#!/bin/bash
# Fast standalone transpile->compile->run regression loop.
# Edit tiny/php/Cases.php (add a case to run_all()), then: bash tiny/run.sh
# Transpiles php/ (+ runtime-stubs) into the tiny_repro crate, runs the driver,
# and fails (non-zero exit) if any case reports FAIL.
set -e
cd "$(dirname "$0")"
OUT="$(pwd)/rust/generated/tiny_repro"

echo "=== TRANSPILE ==="
php -d memory_limit=4000M ../psalm -c psalm-tiny.xml --no-cache --no-progress \
  --threads=1 --transpile-rust="$OUT" 2>&1 \
  | grep -E "wrote |Transpiler crashed|Fatal error|Uncaught|\[mixed|Mixed roots|\[dyn" || true
echo "Mixed in generated: $(grep -rho "\bMixed\b" "$OUT/src" | wc -l)"

# Keep php-rt pointed at the vendored copy (transpiler re-emits its path each run).
sed -i 's#^php-rt = .*#php-rt = { path = "../../php-rt" }#' "$OUT/Cargo.toml"

echo "=== BUILD + RUN ==="
cd rust
set +e
OUTPUT=$(cargo run -q -p driver 2>&1)
BUILD_RC=$?
echo "$OUTPUT"
if [ $BUILD_RC -ne 0 ] || echo "$OUTPUT" | grep -q "FAIL\|error\[\|could not compile\|panicked"; then
  echo "=== RESULT: FAIL ==="
  exit 1
fi
echo "=== RESULT: all cases PASS ==="
