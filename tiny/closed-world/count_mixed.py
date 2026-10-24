#!/usr/bin/env python3
"""Count `Mixed` occurrences in the generated code of one closed-world case:
the free functions the case file declares (emitted into a shared module) plus its namespace's class modules."""
import os, re, sys
src, php_file, ns_dir = sys.argv[1], sys.argv[2], sys.argv[3]
php = open(php_file).read()
names = set(re.findall(r'^\s*function\s+&?\s*(\w+)\s*\(', php, re.M))
count = 0
word = re.compile(r'\bMixed\b')
for root, _, files in os.walk(src):
    for f in files:
        if not f.endswith('.rs'):
            continue
        path = os.path.join(root, f)
        text = open(path).read()
        if os.path.join(src, ns_dir) in path:
            count += len(word.findall(text))
            continue
        # top-level free functions: from `pub fn name(` to the next top-level item
        for m in re.finditer(r'^pub fn (\w+)\b.*?(?=^\S|\Z)', text, re.M | re.S):
            if m.group(1) in names:
                count += len(word.findall(m.group(0)))
print(count)
