#!/bin/bash
# tx-all.sh ROUNDS: pristine master -> textual.php -> rounds of (check, fix.php, check, trace.php) until nothing changes
export PHP_INI_SCAN_DIR=/root/bench/confd
S=/root/idconv/scratch
export ID_CONVERT_PRISTINE=/root/idconv/pristine.json
T=/root/idconv/tools
M=/root/idconv/master
cd $M && git reset -q --hard pristine && git clean -fdq src bin tests examples
php $T/textual.php $M | tail -1
check() { php -d memory_limit=-1 /root/idconv/master-ref/psalm -c psalm-check.xml --no-cache --threads=16 --no-progress --output-format=json --report=$1 > /dev/null 2>&1; }
errs() { python3 -c "import json; print(sum(1 for i in json.load(open('$1')) if i['severity']=='error'))"; }
lint() { for f in $(git diff --name-only); do php -l $f 2>&1 | grep "Parse error\|Fatal error"; done | head -3; }
for n in $(seq 1 $1); do
  check $S/tx-a$n.json
  f=$(php $T/fix.php $M $S/tx-a$n.json)
  bad=$(lint); [ -n "$bad" ] && { echo "round $n fix broke: $bad"; exit 1; }
  check $S/tx-b$n.json
  t=$(php $T/trace.php $M $S/tx-b$n.json /root/idconv/master-ref/psalm psalm-check.xml 2>/dev/null)
  bad=$(lint); [ -n "$bad" ] && { echo "round $n trace broke: $bad"; exit 1; }
  echo "round $n: $(errs $S/tx-a$n.json) -> $(errs $S/tx-b$n.json) errors; $f; $t"
  case "$f $t" in *"fixed 0 sites"*"fixed 0 writers"*) break;; esac
  cur=$(errs $S/tx-b$n.json); if [ -n "${prev:-}" ] && [ "$cur" -gt "$prev" ]; then ups=$(( ${ups:-0} + 1 )); else ups=0; fi
  if [ "${ups:-0}" -ge 2 ]; then echo "diverging at round $n"; break; fi; prev=$cur
done
php $T/prune.php $M
# the automated output, kept as the base the reviewed hand fixes apply to
git add -A && git commit -qm "pipeline output" && git tag -f auto > /dev/null
# the reviewed hand fixes on top of the automated conversion
if [ -s $T/manual.diff ]; then git apply --3way --whitespace=nowarn $T/manual.diff && [ -z "$(git diff --name-only --diff-filter=U)" ] && echo "manual.diff applied" || echo "manual.diff FAILED"; fi
# class names in the dictionaries' types in their declared spelling
php $T/dicts.php $M
check $S/tx-final.json; echo "final: $(errs $S/tx-final.json) errors"
