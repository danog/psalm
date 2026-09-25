#!/bin/bash
# sync the local hand-fixed tree to ct, run the clean check, pull back the new-vs-pristine issues
S=/tmp/claude-1000/-home-daniil-repos-psalm-port/eaf4415e-6e69-4265-9995-15e39f85484c/scratchpad
rsync -a --no-owner --no-group --delete --exclude vendor --exclude .git $S/conv/ ct:/root/idconv/master/ || exit 1
ssh ct 'cd /root/idconv/master && export PHP_INI_SCAN_DIR=/root/bench/confd && php -d memory_limit=-1 /root/idconv/master-ref/psalm -c psalm-check.xml --no-cache --threads=16 --no-progress --output-format=json --report=/root/idconv/scratch/tx-final.json >/dev/null 2>&1; cd /root/idconv && python3 classify.py scratch > scratch/classify.txt; head -1 scratch/classify.txt'
scp -q ct:/root/idconv/scratch/new.json $S/new.json
