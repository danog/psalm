#!/bin/bash
# regenerate, build and test the php-parser crate; writes build/pipeline.log
cd /home/daniil/repos/psalm-port
php -d memory_limit=3000M ../psalm-transpiler/psalm -c psalm-transpile-phpparser.xml --no-cache --no-progress --threads=1 --scan-threads=3 --transpile-rust=rust/generated/php_parser > build/transpile.log 2>&1
grep -n "\[transpiler\] [0-9]\|crashed\|Uncaught" build/transpile.log | head -3
cd rust
cargo build -p php_parser --tests > build.log 2>&1
echo "build errors: $(grep -c '^error' build.log)"
grep "^error" build.log | sort | uniq -c | sort -rn | head -12
grep -n "^error" -A12 build.log | grep -E "^[0-9]+-\s+-->" | head -8
if ! grep -q "^error" build.log; then
  timeout 900 cargo test -p php_parser > test_full.log 2>&1
  grep "test result" test_full.log
  awk '/^---- .* stdout ----$/{name=$2; got=0} /^\[failed\]/{ if(!got){got=1; sub(/^\[failed\] data set "[^"]*": /,""); print name " :: " substr($0,1,170)} } /panicked at/{ if(!got){got=1; getline msg; print name " :: " substr(msg,1,170)} }' test_full.log > fail_map.txt
  cut -d: -f4- fail_map.txt | sed -E 's/[0-9]+/N/g' | sort | uniq -c | sort -rn | head -45
fi
