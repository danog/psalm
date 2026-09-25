#!/bin/bash
# regenerate, build and test the psalm crate; writes build/pipeline-psalm-tests.log
cd /home/daniil/repos/psalm-port
php -d memory_limit=5000M ../psalm-transpiler/psalm -c psalm-transpile-psalm-tests.xml --no-cache --no-progress --threads=1 --scan-threads=4 --transpile-rust=rust/generated/psalm_base --transpile-rust-split=rust/generated/psalm_data:dictionaries --transpile-rust-split=rust/generated/psalm_src:src --transpile-rust-split=rust/generated/psalm_tests:tests --transpile-rust-data='dictionaries/*.php' > build/transpile-psalm-tests.log 2>&1
grep -n "\[transpiler\] [0-9]\|crashed\|Uncaught\|Fatal" build/transpile-psalm-tests.log | head -5
cd rust
# a debug build first (fast to compile: catches codegen errors), then the fast profile for the suite
cargo build -j1 -p psalm_tests --tests > build-psalm.log 2>&1
echo "build errors: $(grep -c '^error' build-psalm.log)"
grep "^error" build-psalm.log | sort | uniq -c | sort -rn | head -12
grep -n "^error" -A12 build-psalm.log | grep -E "^[0-9]+-\s+-->" | head -8
if ! grep -q "^error" build-psalm.log; then
  cargo build -j1 --profile fast -p psalm_tests --tests > build-psalm-fast.log 2>&1
  echo "fast build errors: $(grep -c '^error' build-psalm-fast.log)"
  ../build/run-harness-by-class.sh fast test_psalm.log
fi
