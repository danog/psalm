#!/bin/bash
# Closed-world transpile->compile->run loop.
# Every php/<case>.php declares `namespace Cw\<Case>;` and an entry function `run_<case>()` returning a string
# (function names are global in the generated crate: keep helper names unique across cases).
# The expected output of each case is what `php -n` prints for it; the transpiled crate must print the same.
# Per case it reports PASS/FAIL and the number of `Mixed` occurrences in that case's generated module.
# Usage: bash run.sh [--no-closed-world]   (the flag transpiles in trusted mode, for comparison)
set -e
cd "$(dirname "$0")"
HERE="$(pwd)"
OUT="$HERE/rust/generated/tiny_cw"
CW_FLAG="--transpile-closed-world"
if [ "$1" = "--no-closed-world" ]; then CW_FLAG=""; fi
mkdir -p build

cases=()
for f in php/*.php; do cases+=("$(basename "$f" .php)"); done

ns_of() { sed -n 's/^namespace \([A-Za-z0-9_\\]*\);.*/\1/p' "php/$1.php" | head -1; }
mod_of() { # PHP namespace -> generated module dir
  echo "$1" | tr '\\' '\n' | sed -E 's/([a-z0-9])([A-Z])/\1_\2/g' | tr 'A-Z' 'a-z' | paste -sd/ -
}

echo "=== TRANSPILE ${CW_FLAG:-(trusted mode)} ==="
rm -rf "$OUT/src"
php -d memory_limit=4000M ../../psalm -c psalm-cw.xml --no-cache --no-progress --threads=1 $CW_FLAG \
  --transpile-rust="$OUT" > build/transpile.log 2>&1 || true
grep -E "Transpiler crashed|Fatal error|Uncaught|\[closed-world\]|\[transpiler\] [a-z]|e\.g\.|\[mixed-var\]" build/transpile.log \
  | grep -v "runtime-stubs" | sed 's#'"$HERE"'/##' || true
sed -i.bak 's#^php-rt = .*#php-rt = { path = "../../../../rust/php-rt" }#' "$OUT/Cargo.toml" && rm -f "$OUT/Cargo.toml.bak"

# driver: run every case, delimited
{
  echo 'fn main() {'
  echo '    tiny_cw::init();'
  for c in "${cases[@]}"; do
    file=$(grep -rl "^pub fn run_$c(" "$OUT/src" | head -1)
    path=$(echo "${file#$OUT/src/}" | sed -e 's#/mod\.rs$##' -e 's#\.rs$##' -e 's#/#::#g')
    if [ -n "$file" ]; then
      echo "    println!(\"=== $c\"); print!(\"{}\", tiny_cw::$path::run_$c());"
    fi
  done
  echo '}'
} > rust/driver/src/main.rs

echo "=== BUILD + RUN ==="
cd rust
set +e
CARGO_TARGET_DIR="$HERE/../rust/target" CARGO_BUILD_JOBS=4 nice cargo build -q -p driver > "$HERE/build/cargo.log" 2>&1
BUILD_RC=$?
if [ $BUILD_RC -ne 0 ]; then
  grep -E "^error" -A6 "$HERE/build/cargo.log" | head -80
  echo "=== RESULT: BUILD FAILED ($(grep -c '^error' "$HERE/build/cargo.log") errors) ==="
fi
ACTUAL=""
if [ $BUILD_RC -eq 0 ]; then ACTUAL=$("$HERE/../rust/target/debug/driver" 2>&1); fi
cd "$HERE"
fail=0
for c in "${cases[@]}"; do
  ns=$(ns_of "$c")
  expected=$(php -n -d display_errors=0 -r 'require $argv[1]; echo ("\\" . $argv[2] . "\\run_" . $argv[3])();' "php/$c.php" "$ns" "$c" 2>/dev/null)
  actual=$(printf '%s\n' "$ACTUAL" | awk -v c="=== $c" '$0==c{f=1;next} /^=== /{f=0} f')
  mixed=$(python3 count_mixed.py "$OUT/src" "php/$c.php" "$(mod_of "$ns")" 2>/dev/null)
  if [ $BUILD_RC -eq 0 ] && [ "$expected" = "$actual" ]; then
    echo "PASS $c (Mixed: $mixed)"
  else
    echo "FAIL $c (Mixed: $mixed)"
    echo "  expected: $(printf '%s' "$expected" | head -c 300)"
    echo "  actual:   $(printf '%s' "$actual" | head -c 300)"
    fail=1
  fi
done
[ $fail -eq 0 ] && [ $BUILD_RC -eq 0 ] && echo "=== RESULT: all cases PASS ===" || { echo "=== RESULT: FAIL ==="; exit 1; }
