#!/bin/bash
# build-branches.sh <base> <compat-src> <perf-src> <ids-src> <full>: the fork as atomic, single-commit branches.
#
#   split/runtime        base + rust/ (the Rust runtime and CLI)                                  from <full>
#   split/tooling        base + transpiler pipeline configs, build scripts, the PHP test runner   from <full>
#   split/compat         base + Psalm sources as the transpiler needs them (typing, closed world,
#                        compiled-program guards, test harness)                                    from <compat-src>
#   split/pzoom-perf     split/compat + the pzoom-faithful analysis program without names as ids  from <perf-src>
#   split/interner-ids   split/pzoom-perf + the Interner, every string -> id conversion and its
#                        tooling (tools/id-refactor, tools/id-convert, bin/generate-sym.php)       from <ids-src>
#   split/stripped       merge of runtime + tooling + pzoom-perf: the fork without the id work
#   split/fork           merge of stripped + interner-ids: the whole fork
#
# Each class branch is one commit whose tree is its parent's with the class's paths taken from the source commit
# (paths the source lacks are removed). The subjects of the original commits of each class go in the body.
set -euo pipefail
BASE=$1; COMPAT=$2; PERF=$3; IDS=$4; FULL=$5
cd "$(git rev-parse --show-toplevel)"

RUNTIME_PATHS=(rust)
TOOLING_PATHS=(bin/transpile build run-tests-under-php.php .gitignore psalm-transpile-psalm.xml
    psalm-transpile-psalm-tests.xml psalm-transpile-phpparser.xml psalm-transpile-phpparser-lib.xml psalm-tiny.xml
    psalm-mixed-probe.xml psalm-mixed-inventory.xml psalm-list-probe.xml tools/split)
SOURCE_PATHS=(src tests examples stubs docs dictionaries bin/generate-constant-map.php psalm.xml.dist psalm-baseline.xml)
IDS_EXTRA_PATHS=(bin/generate-sym.php tools/id-refactor tools/id-convert)

TMPIDX=$(mktemp)
trap 'rm -f "$TMPIDX"' EXIT

# tree of <parent commit> with <paths> replaced by <source commit>'s
overlay() {
    local parent=$1 src=$2; shift 2
    GIT_INDEX_FILE=$TMPIDX git read-tree "$parent"
    GIT_INDEX_FILE=$TMPIDX git rm -r -q --cached --ignore-unmatch -- "$@" > /dev/null
    git ls-tree -r "$src" -- "$@" | GIT_INDEX_FILE=$TMPIDX git update-index --index-info
    GIT_INDEX_FILE=$TMPIDX git write-tree
}

commit() {
    local tree=$1 msg=$2; shift 2
    local parents=()
    for p in "$@"; do parents+=(-p "$p"); done
    git commit-tree "$tree" "${parents[@]}" -F <(printf '%s\n' "$msg")
}

subjects() {
    # first-parent subjects of a range touching some paths
    local range=$1; shift
    git log --first-parent --reverse --format='- %s' "$range" -- "$@" | cut -c1-160
}

TRAILER=$'\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\nClaude-Session: https://claude.ai/code/session_01DqSwqQgMeVmE9DNQyXdMt7'

t=$(overlay "$BASE" "$FULL" "${RUNTIME_PATHS[@]}")
RUNTIME=$(commit "$t" "Rust runtime (php-rt), CLI and generated-crate workspace

$(subjects "$BASE..$FULL" "${RUNTIME_PATHS[@]}")
$TRAILER" "$BASE")
git branch -f split/runtime "$RUNTIME"

t=$(overlay "$BASE" "$FULL" "${TOOLING_PATHS[@]}")
TOOLING=$(commit "$t" "Transpiler pipeline: transpile configs, build scripts, probes, the PHP test runner, branch-split tools

$(subjects "$BASE..$FULL" "${TOOLING_PATHS[@]}")
$TRAILER" "$BASE")
git branch -f split/tooling "$TOOLING"

t=$(overlay "$BASE" "$COMPAT" "${SOURCE_PATHS[@]}")
C=$(commit "$t" "Psalm sources for the transpiled analyzer: typing without mixed, closed world, compiled-program guards, test harness

$(subjects "$BASE..$COMPAT" "${SOURCE_PATHS[@]}")
$TRAILER" "$BASE")
git branch -f split/compat "$C"

t=$(overlay "$C" "$PERF" "${SOURCE_PATHS[@]}")
P=$(commit "$t" "pzoom-faithful analysis program without names as ids: memos, clause hashing, union lists, immutable types,
flattened member maps, case-sensitive resolution

$(subjects "$COMPAT..$PERF" "${SOURCE_PATHS[@]}")
$TRAILER" "$C")
git branch -f split/pzoom-perf "$P"

t=$(overlay "$P" "$IDS" "${SOURCE_PATHS[@]}" "${IDS_EXTRA_PATHS[@]}")
I=$(commit "$t" "Interner: names as interned ids (pzoom's StrId) and every string -> id conversion, with the id-refactor tooling

$(for h in $(grep -v '^#' "$(dirname "$0")/ids.txt"); do git log -1 --format='- %s' "$h" | cut -c1-160; done)
$TRAILER" "$P")
git branch -f split/interner-ids "$I"

# the merges: trees assembled from the class branches' paths
t=$(overlay "$P" "$RUNTIME" "${RUNTIME_PATHS[@]}")
t2=$(overlay "$(commit "$t" tmp "$P")" "$TOOLING" "${TOOLING_PATHS[@]}")
S=$(commit "$t2" "The fork without the id work: runtime + tooling + pzoom-perf$TRAILER" "$P" "$RUNTIME" "$TOOLING")
git branch -f split/stripped "$S"

t=$(overlay "$S" "$I" "${SOURCE_PATHS[@]}" "${IDS_EXTRA_PATHS[@]}")
F=$(commit "$t" "The whole fork: stripped + interner-ids$TRAILER" "$S" "$I")
git branch -f split/fork "$F"

for b in runtime tooling compat pzoom-perf interner-ids stripped fork; do
    printf '%-20s %s  %s\n' "split/$b" "$(git rev-parse --short split/$b)" "$(git log -1 --format=%s split/$b | cut -c1-90)"
done
