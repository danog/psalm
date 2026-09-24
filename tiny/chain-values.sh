#!/bin/bash
# On ct (copy to /root/bench/chain-values.sh). Step 1: c112 = id-based lookups (psalm-port ba116edfd), default emission.
# Step 2: scratch transpile + cargo check with VALUE_TYPES=1. Step 3: c113 = VALUE_TYPES=1 full cycle + harness + bench + profile.
# Prereq (from the laptop): rsync psalm-port (excl rust) and psalm-transpiler (excl target, tiny/rust) to /home/daniil/repos/.
set -u
bash /root/bench/full-cycle.sh 112
bash /root/bench/selfcheck.sh f23 > /root/bench/cycle-f23.log 2>&1
bash /root/bench/run-suite.sh tests-f23 >> /root/bench/cycle-f23.log 2>&1
echo CYCLE_DONE >> /root/bench/cycle-f23.log
export PATH=/root/.cargo/bin:$PATH
cd /root/psalm-build/rust && cargo +nightly build --profile bench-fat --target-dir target-fat -p psalm-cli > /root/bench/fat-build.log 2>&1
echo FAT_RC=$? >> /root/bench/fat-build.log
sleep 30
bash /root/bench/rebench2.sh > /root/bench/rebench2-c112.log 2>&1
bash /root/bench/reprofile.sh c112 > /root/bench/reprofile-c112.log 2>&1
echo BENCH_DONE >> /root/bench/rebench2-c112.log
# --- value types: scratch check first
cd /home/daniil/repos/psalm-port
OUT=/root/psalm-build/rust/generated/psalm_mono.diag
VALUE_TYPES=1 TRANSPILE_JOBS=16 php -d memory_limit=12000M ../psalm-transpiler/psalm -c psalm-transpile-psalm-tests.xml --no-cache --no-progress --threads=16 --scan-threads=32 --transpile-rust=$OUT --transpile-rust-data="dictionaries/*.php" > /root/bench/transpile-values.log 2>&1
echo "rc=$?" >> /root/bench/transpile-values.log
grep -rho "^pub struct [A-Za-z0-9_]*(pub [A-Za-z0-9_]*Obj_);" $OUT/src | wc -l >> /root/bench/transpile-values.log
cd $OUT && sed -i 's/^name = "psalm_mono.diag"/name = "psalm_mono_diag"/' Cargo.toml && (grep -q "^\[workspace\]" Cargo.toml || printf "\n[workspace]\n" >> Cargo.toml)
(RUSTFLAGS="-Zthreads=16" cargo +nightly check --lib -j16 --target-dir /root/psalm-build/rust/target-diag) > /root/bench/check-values.log 2>&1
echo "rc=$?" >> /root/bench/check-values.log
if grep -q "^rc=0" /root/bench/check-values.log; then
  export VALUE_TYPES=1
  bash /root/bench/full-cycle.sh 113
  unset VALUE_TYPES
  cd /root/psalm-build/rust && cargo +nightly build --profile bench-fat --target-dir target-fat -p psalm-cli > /root/bench/fat-build.log 2>&1
  echo FAT_RC=$? >> /root/bench/fat-build.log
  sleep 30
  bash /root/bench/rebench2.sh > /root/bench/rebench2-c113.log 2>&1
  bash /root/bench/reprofile.sh c113 > /root/bench/reprofile-c113.log 2>&1
  echo BENCH_DONE >> /root/bench/rebench2-c113.log
fi
