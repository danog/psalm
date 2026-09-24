#!/usr/bin/env python3
"""scan_report.py <classname-scan.jsonl>: psalm-port's string-name inventory (ClassNameScanPlugin) next to pzoom's
(pzoom_scan.py), plus the psalm-port slots grouped by class and the boundary calls grouped by method."""
import json, sys, collections, subprocess, os
here = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, here)
import pzoom_scan
rows = [json.loads(l) for l in open(sys.argv[1])]
slots = [r for r in rows if r['kind'] == 'slot']
bounds = list({(r['file'], r['pos'], r['what']): r for r in rows if r['kind'] == 'boundary'}.values())
pf, pp, pb, _ = pzoom_scan.scan()
pz = collections.Counter()
for (f, fn, k), n in pb.items(): pz[k] += n
ours = collections.Counter(r['what'].split(':')[0] + ':' + r['what'].split(':')[-1] if r['what'].startswith('interner') else 'casefold' for r in bounds)
print("                         pzoom   psalm-port")
print(f"name params (string)     {sum(1 for p in pp if p['kind']=='str' and p['nameish']):5d}   {sum(1 for s in slots if s['slot']=='param'):5d}")
print(f"name fields (string)     {sum(1 for f in pf if f['kind']=='str' and f['nameish']):5d}   {sum(1 for s in slots if s['slot']=='property'):5d}")
print(f"name returns (string)        -   {sum(1 for s in slots if s['slot']=='return'):5d}")
print(f"casefold call sites      {pz['casefold']:5d}   {ours['casefold']:5d}")
print(f"intern call sites        {pz['intern']:5d}   {ours['interner:intern']:5d}")
print(f"lookup call sites        {pz['lookup']:5d}   {ours['interner:lookup'] + ours['interner:lookuplc']:5d}")
if '--detail' in sys.argv:
    print("\n== psalm-port string-name slots by class")
    byc = collections.Counter(s['class'] for s in slots)
    for c, n in byc.most_common(40): print(f"  {n:4d}  {c}")
    print("\n== casefold + intern sites by method")
    bym = collections.Counter(b['method'] for b in bounds if b['what'].startswith('casefold') or b['what'] == 'interner:intern')
    for m, n in bym.most_common(40): print(f"  {n:4d}  {m}")
