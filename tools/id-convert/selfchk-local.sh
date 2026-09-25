#!/bin/bash
# sync the local tree to ct and run the converted Psalm on itself; print the crash or the issue counts
S=/tmp/claude-1000/-home-daniil-repos-psalm-port/eaf4415e-6e69-4265-9995-15e39f85484c/scratchpad
T=${1:-16}
rsync -a --no-owner --no-group --delete --exclude vendor --exclude .git $S/conv/ ct:/root/idconv/master/ || exit 1
ssh ct "cd /root/idconv/master && cp /root/idconv/psalm-self.xml . && export PHP_INI_SCAN_DIR=/root/bench/confd && timeout 900 php -d memory_limit=-1 ./psalm -c psalm-self.xml --no-cache --threads=$T --no-progress --output-format=json --report=/root/idconv/scratch/self.json > /root/idconv/scratch/self.out 2>&1; echo exit \$?; rm -f psalm-self.xml; grep -m1 -B2 -A12 'Uncaught\|Fatal\|Warning:\|Notice:\|Deprecated:' /root/idconv/scratch/self.out | cut -c1-260; python3 -c \"
import json,collections
try:
    d=json.load(open('/root/idconv/scratch/self.json'))
    c=collections.Counter(i['type'] for i in d if i['severity']=='error'); print('self errors',sum(c.values()), c.most_common(12))
except Exception as e: print('no report', e)
\""
