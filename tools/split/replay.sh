#!/bin/bash
# replay.sh <worktree> <from> <to>: cherry-picks every first-parent commit of from..to that is not in ids.txt onto the
# worktree's branch, resolving conflicts with resolve.py (token merge, the branch's side on overlap); logs per commit.
set -u
WT=$1; FROM=$2; TO=$3
HERE=$(cd "$(dirname "$0")" && pwd)
IDS=$(grep -v '^#' "$HERE/ids.txt" | tr ' ' '\n' | grep -v '^$')
cd "$WT"
for c in $(git rev-list --first-parent --reverse "$FROM..$TO"); do
    short=$(git rev-parse --short=9 "$c")
    if echo "$IDS" | grep -q "^$short"; then echo "SKIP $short $(git log -1 --format=%s $c | cut -c1-70)"; continue; fi
    if git cherry-pick --no-commit "$c" > /dev/null 2>&1; then
        n=0
    else
        out=$(python3 "$HERE/resolve.py" --prefer "${PREFER:-ours}")
        n=$(echo "$out" | grep -o '[0-9]* conflict' | awk '{s+=$1} END {print s+0}')
        echo "$out" | sed "s/^/    [$short] /" | grep -v ": token merge, 0 conflict" 
    fi
    if git diff --cached --quiet; then echo "EMPTY $short"; continue; fi
    git commit -q -C "$c" --no-verify
    echo "PICK $short conflicts=$n $(git log -1 --format=%s $c | cut -c1-70)"
done
