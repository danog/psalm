#!/bin/bash
# regenerate, build and test the psalm crate; writes build/pipeline-psalm.log
cd /home/daniil/repos/psalm-port
php -d memory_limit=5000M psalm -c psalm-transpile-psalm.xml --no-cache --no-progress --transpile-rust=rust/generated/psalm > build/transpile-psalm.log 2>&1
grep -n "\[transpiler\] [0-9]\|crashed\|Uncaught\|Fatal" build/transpile-psalm.log | head -5
cd rust
cargo build -p psalm --tests > build-psalm.log 2>&1
echo "build errors: $(grep -c '^error' build-psalm.log)"
grep "^error" build-psalm.log | sort | uniq -c | sort -rn | head -12
grep -n "^error" -A12 build-psalm.log | grep -E "^[0-9]+-\s+-->" | head -8
if ! grep -q "^error" build-psalm.log; then
  timeout 1800 cargo test -p psalm > test_psalm.log 2>&1
  grep "test result" test_psalm.log
  awk '/^---- tests::/{name=$2; got=0} /^\[failed\]/{ if(!got){got=1; sub(/^\[failed\] data set "[^"]*": /,""); print name " :: " substr($0,1,170)} } /panicked at/{ if(!got){got=1; getline msg; print name " :: " substr(msg,1,170)} }' test_psalm.log > fail_map_psalm.txt
  cut -d: -f4- fail_map_psalm.txt | sed -E 's/[0-9]+/N/g' | sort | uniq -c | sort -rn | head -45
fi
