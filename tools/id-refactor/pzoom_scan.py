#!/usr/bin/env python3
"""Inventory of where pzoom keeps names as strings vs interned ids (StrId), and its string<->id boundaries.

For every Rust source under ~/repos/pzoom/crates (tests excluded):
  - struct fields / fn params / map keys typed String, &str, Arc<str>, CompactString vs StrId, with names;
  - per function: calls that cross the boundary: intern (string -> id), lookup / get_str (id -> string),
    to_ascii_lowercase / to_lowercase / eq_ignore_ascii_case (case folding of names).
Output: JSON (--json) or a summary grouped by crate/file/function.
"""
import json, os, re, sys, collections

ROOT = os.path.expanduser('~/repos/pzoom/crates')
STR_T = r'(?:String|&\s*(?:\'[a-z]+\s+)?str|Arc<str>|Rc<str>|CompactString|Box<str>)'
field_re = re.compile(r'^\s*(?:pub(?:\([a-z]+\))?\s+)?([a-z_][a-z0-9_]*)\s*:\s*([^,{;=]+?)\s*,\s*(?://.*)?$')
fn_re = re.compile(r'\bfn\s+([a-z_][a-z0-9_]*)\s*(?:<[^>]*>)?\s*\(')
param_re = re.compile(r'([a-z_][a-z0-9_]*)\s*:\s*([^,()]+(?:\([^)]*\))?[^,()]*)')
BOUNDARY = {
    'intern': re.compile(r'\.intern\(|\binterner\.intern\b|StrId::from_str|intern_str\(|interner\(\)\.find\(|interner\.find\('),
    'lookup': re.compile(r'\.lookup\(|\.get_str\(|interner\.lookup\b|\.resolve_str\('),
    'casefold': re.compile(r'to_ascii_lowercase\(|to_lowercase\(|eq_ignore_ascii_case\(|make_ascii_lowercase\('),
}
NAMEISH = re.compile(r'(^|_)(fq|fqn|fqcn|fqcln|class|classlike|interface|trait|enum|parent|self|static|declaring|'
                     r'appearing|implementing|extended|mixin|name|names|type_name|method|function|property|const)(_|$|s$)')

def scan():
    fields = []; params = []; bounds = collections.Counter(); bound_sites = []
    for dp, dn, fn in os.walk(ROOT):
        if '/tests' in dp or '/benches' in dp or '/target' in dp:
            continue
        for f in fn:
            if not f.endswith('.rs'):
                continue
            path = os.path.join(dp, f)
            rel = os.path.relpath(path, ROOT)
            src = open(path, encoding='utf-8', errors='replace').read()
            # drop #[cfg(test)] modules crudely
            src = re.sub(r'#\[cfg\(test\)\]\s*mod\s+tests\s*\{.*', '', src, flags=re.S)
            cur_fn = None
            depth = 0; fn_depth = None
            in_struct = False; struct_depth = None
            for ln, line in enumerate(src.split('\n'), 1):
                code = line.split('//')[0]
                m = fn_re.search(code)
                if m:
                    cur_fn = m.group(1); fn_depth = depth
                    sig = code[m.end():]
                    for pm in param_re.finditer(sig):
                        pname, ptype = pm.group(1), pm.group(2).strip()
                        kind = 'id' if 'StrId' in ptype else ('str' if re.search(STR_T, ptype) else None)
                        if kind and pname != 'self':
                            params.append({'file': rel, 'line': ln, 'fn': cur_fn, 'name': pname, 'type': ptype, 'kind': kind, 'nameish': bool(NAMEISH.search(pname))})
                if re.match(r'\s*(pub(\([a-z]+\))?\s+)?struct\s+\w+.*\{\s*$', code):
                    in_struct = True; struct_depth = depth
                if in_struct and depth == struct_depth + 1:
                    fm = field_re.match(code)
                    if fm:
                        fname, ftype = fm.group(1), fm.group(2)
                        kind = 'id' if 'StrId' in ftype else ('str' if re.search(STR_T, ftype) else None)
                        if kind:
                            fields.append({'file': rel, 'line': ln, 'name': fname, 'type': ftype, 'kind': kind, 'nameish': bool(NAMEISH.search(fname))})
                for k, rx in BOUNDARY.items():
                    n = len(rx.findall(code))
                    if n:
                        bounds[(rel, cur_fn or '<top>', k)] += n
                        bound_sites.append({'file': rel, 'line': ln, 'fn': cur_fn, 'kind': k})
                depth += code.count('{') - code.count('}')
                if in_struct and depth <= struct_depth:
                    in_struct = False
                if fn_depth is not None and depth <= fn_depth and '}' in code:
                    cur_fn = None; fn_depth = None
    return fields, params, bounds, bound_sites

if __name__ == '__main__':
    fields, params, bounds, sites = scan()
    if '--json' in sys.argv:
        json.dump({'fields': fields, 'params': params, 'boundary_sites': sites}, sys.stdout, indent=1); sys.exit()
    fk = collections.Counter(f['kind'] for f in fields); pk = collections.Counter(p['kind'] for p in params)
    print(f"struct fields: StrId {fk['id']}, string {fk['str']};  fn params: StrId {pk['id']}, string {pk['str']}")
    bk = collections.Counter(); 
    for (f, fn, k), n in bounds.items(): bk[k] += n
    print(f"boundary calls: intern {bk['intern']}, lookup {bk['lookup']}, casefold {bk['casefold']}")
    print("\n== string-typed name-ish fields")
    for f in fields:
        if f['kind'] == 'str' and f['nameish']:
            print(f"  {f['file']}:{f['line']}  {f['name']}: {f['type']}")
    print("\n== string-typed name-ish params (by crate)")
    byc = collections.Counter(p['file'].split('/')[0] for p in params if p['kind'] == 'str' and p['nameish'])
    for c, n in byc.most_common(): print(f"  {n:4d}  {c}")
    print("\n== casefold sites by crate/file")
    byf = collections.Counter()
    for (f, fn, k), n in bounds.items():
        if k == 'casefold': byf[f] += n
    for f, n in byf.most_common(25): print(f"  {n:4d}  {f}")
