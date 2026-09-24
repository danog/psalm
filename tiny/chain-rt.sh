#!/bin/bash
# On ct (copy to /root/bench/chain-rt.sh). chain-rt.sh <n>: a runtime-only cycle (no transpile): rebuild tests
# + harness, bench + fat builds, rebench2, then exact allocation count via callgrind and the php-rt census.
set -u
export PATH=/root/.cargo/bin:$PATH PHP_INI_SCAN_DIR=/root/bench/confd
N=$1; L=/root/bench/cycle-c$N.log
cd /root/psalm-build
echo "START $(date)" > $L
if bash /tmp/build_tests.sh >> $L 2>&1; then
  echo "BUILD_DONE $(date)" >> $L
  bash /tmp/run_harness.sh >> $L 2>&1
  cp /root/psalm-build/run.log /root/psalm-build/run$N.log
  grep "test result" /root/psalm-build/run.log >> $L
  echo "HARNESS_DONE $(date)" >> $L
else
  echo "BUILD_FAILED" >> $L; echo "CYCLE_DONE $(date)" >> $L; exit 1
fi
cd /root/psalm-build/rust
cargo +nightly build --profile bench --target-dir target-bench -p psalm-cli > /root/bench/bench-build.log 2>&1; echo "BENCH_BUILD_RC=$?" >> $L
cargo +nightly build --profile bench-fat --target-dir target-fat -p psalm-cli > /root/bench/fat-build.log 2>&1; echo "FAT_RC=$?" >> /root/bench/fat-build.log
cargo +nightly build --profile bench --features php-rt/stats --target-dir target-stats -p psalm-cli > /root/bench/census-build.log 2>&1
echo "CYCLE_DONE $(date)" >> $L
sleep 20
bash /root/bench/rebench2.sh > /root/bench/rebench2-c$N.log 2>&1
echo BENCH_DONE >> /root/bench/rebench2-c$N.log
cd /root/bench/psalm-master
/root/psalm-build/rust/target-stats/release/psalm-rs -c psalm-notaint.xml --threads=1 --no-cache --no-progress --monochrome --php-version=8.5 > /root/bench/census-c$N.out 2> /root/bench/census-c$N.txt
valgrind --tool=callgrind --callgrind-out-file=/root/bench/cg-c$N.out --cache-sim=no --branch-sim=no /root/psalm-build/rust/target-fat/bench-fat/psalm-rs -c psalm-notaint.xml --threads=1 --no-cache --no-progress --monochrome --php-version=8.5 > /root/bench/cg-c$N.log 2>&1
cd /root/bench && python3 cgsum.py cg-c$N.out "^(mi_malloc_aligned|mi_realloc_aligned)$" > cgsum-c$N.txt 2>&1
echo ALL_DONE >> /root/bench/census-c$N.txt
