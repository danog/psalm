#!/usr/bin/env python3
"""baseline_refresh.py <fresh-baseline.xml> <file>:<IssueType> ...: replaces those (file, issue type) blocks of
psalm-baseline.xml by the fresh baseline's (a refactor changed the code snippets the baseline matches on)."""
import re, sys, os
root = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
base_path = os.path.join(root, 'psalm-baseline.xml')
fresh = open(sys.argv[1]).read(); base = open(base_path).read()
def block(xml, f, t):
    fm = re.search(r'<file src="' + re.escape(f) + r'">(.*?)</file>', xml, re.S)
    if not fm: return None, None
    tm = re.search(r'\n    <' + t + r'>.*?</' + t + r'>', fm.group(1), re.S)
    return fm, tm
for pair in sys.argv[2:]:
    f, t = pair.rsplit(':', 1)
    ffm, ftm = block(fresh, f, t)
    bfm, btm = block(base, f, t)
    new = ftm.group(0) if ftm else ''
    if bfm is None:
        print('no file block in baseline for', f); continue
    inner = bfm.group(1)
    if btm:
        inner = inner[:btm.start()] + new + inner[btm.end():]
    else:
        # keep the issue types sorted
        types = [(m.start(), m.group(1)) for m in re.finditer(r'\n    <(\w+)>', inner)]
        pos = next((p for p, n in types if n > t), len(inner.rstrip()) - len(inner.rstrip()) + len(inner) - (len(inner) - len(inner.rstrip())))
        inner = inner[:pos] + new + inner[pos:]
    base = base[:bfm.start(1)] + inner + base[bfm.end(1):]
    print('refreshed', f, t, 'present' if ftm else 'removed')
open(base_path, 'w').write(base)
