#!/usr/bin/env python3
"""resolve.py [--prefer ours|theirs]: resolves every conflicted path of the current cherry-pick / merge by a
token-level three-way merge (each token on its own line, then `git merge-file`), so that edits to different tokens
of the same line both apply; where both sides edit the same tokens the preferred side wins.

Add/add and modify/delete conflicts: the preferred side's version (or its absence) wins.
Prints one line per resolved path with the number of token conflicts decided by preference."""
import os, re, subprocess, sys, tempfile

prefer = 'ours'
if '--prefer' in sys.argv:
    prefer = sys.argv[sys.argv.index('--prefer') + 1]

TOK = re.compile(r'\n|[^\S\n]+|\w+|[^\w\s]', re.S)
NL = '\x1e'


def stage(path, n):
    r = subprocess.run(['git', 'show', f':{n}:{path}'], capture_output=True)
    return r.stdout.decode('utf-8', 'surrogateescape') if r.returncode == 0 else None


def tokens(s):
    return ''.join((t.replace('\n', NL)) + '\n' for t in TOK.findall(s))


def untokens(s):
    return ''.join(line.replace(NL, '\n') for line in s.split('\n'))


AVOID = re.compile(os.environ.get('RESOLVE_AVOID', r'(?!x)x'))


def decide(tok):
    """Picks a side per conflict chunk of a token-per-line diff3 output: `--prefer` side, except that a chunk whose
    preferred text matches RESOLVE_AVOID (and whose other text does not) takes the other side."""
    out = []
    lines = tok.split('\n')
    i = 0
    while i < len(lines):
        l = lines[i]
        if l.startswith('<<<<<<< O'):
            ours, base, theirs, part = [], [], [], 'o'
            i += 1
            while i < len(lines) and not lines[i].startswith('>>>>>>> T'):
                if lines[i].startswith('||||||| B'):
                    part = 'b'
                elif lines[i] == '=======':
                    part = 't'
                else:
                    {'o': ours, 'b': base, 't': theirs}[part].append(lines[i])
                i += 1
            i += 1
            pick_ours = prefer == 'ours'
            o_txt, t_txt = untokens('\n'.join(ours)), untokens('\n'.join(theirs))
            pref, other = (o_txt, t_txt) if pick_ours else (t_txt, o_txt)
            if AVOID.search(pref) and not AVOID.search(other):
                pick_ours = not pick_ours
            out.extend(ours if pick_ours else theirs)
            continue
        out.append(l)
        i += 1
    return '\n'.join(out)


def unmerged():
    out = subprocess.run(['git', 'diff', '--name-only', '--diff-filter=U'], capture_output=True, text=True).stdout
    return [p for p in out.split('\n') if p]


for path in unmerged():
    base, ours, theirs = stage(path, 1), stage(path, 2), stage(path, 3)
    if ours is None or theirs is None or base is None:
        keep = ours if prefer == 'ours' else theirs
        if keep is None:
            # the preferred side deleted it (or never had it)
            subprocess.run(['git', 'rm', '-q', '--cached', path], capture_output=True)
            if os.path.exists(path):
                os.remove(path)
            print(f'{path}: removed ({prefer} has no such file)')
        else:
            with open(path, 'w', encoding='utf-8', errors='surrogateescape') as f:
                f.write(keep)
            subprocess.run(['git', 'add', path])
            print(f'{path}: {prefer} version (add/delete conflict)')
        continue
    with tempfile.TemporaryDirectory() as d:
        files = []
        for name, content in (('ours', ours), ('base', base), ('theirs', theirs)):
            p = os.path.join(d, name)
            with open(p, 'w', encoding='utf-8', errors='surrogateescape') as f:
                f.write(tokens(content))
            files.append(p)
        r0 = subprocess.run(['git', 'merge-file', '-p', '--diff3', '-L', 'O', '-L', 'B', '-L', 'T', *files], capture_output=True)
        n = max(r0.returncode, 0)
        merged = untokens(decide(r0.stdout.decode('utf-8', 'surrogateescape')))
    with open(path, 'w', encoding='utf-8', errors='surrogateescape') as f:
        f.write(merged)
    subprocess.run(['git', 'add', path])
    print(f'{path}: token merge, {n} conflict(s) decided for {prefer}')
