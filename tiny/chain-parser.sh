#!/bin/bash
# On ct (copy to /root/bench/chain-parser.sh). c114 = php-parser descent parser as the default (php-parser 95c8fe8f),
# psalm-port/transpiler as in c112/c113. Export VALUE_TYPES=1 before launching to combine with value-type emission.
# Prereq (from the laptop): rsync php-parser (excl .git) to /home/daniil/repos/php-parser, plus psalm-port/transpiler.
set -u
export PATH=/root/.cargo/bin:$PATH
N=${1:-114}
bash /root/bench/full-cycle.sh $N
bash /root/bench/selfcheck.sh f$N > /root/bench/cycle-f$N.log 2>&1
bash /root/bench/run-suite.sh tests-f$N >> /root/bench/cycle-f$N.log 2>&1
echo CYCLE_DONE >> /root/bench/cycle-f$N.log
cd /root/psalm-build/rust && cargo +nightly build --profile bench-fat --target-dir target-fat -p psalm-cli > /root/bench/fat-build.log 2>&1
echo FAT_RC=$? >> /root/bench/fat-build.log
sleep 30
bash /root/bench/rebench2.sh > /root/bench/rebench2-c$N.log 2>&1
bash /root/bench/reprofile.sh c$N > /root/bench/reprofile-c$N.log 2>&1
echo BENCH_DONE >> /root/bench/rebench2-c$N.log
