"""Helpers to delete/rename whole method declarations (4-space indented class members) with their docblocks."""
import re

def method_span(s, name, first_param_type=None):
    """(start, end) of `function name(` whose first parameter has the given type, docblock and attributes included."""
    for m in re.finditer(r"\n    (?:(?:public|private|protected|static|final)\s+)*function " + re.escape(name) + r"\(", s):
        head = s[m.end():m.end() + 200]
        if first_param_type is not None and not re.match(r"\s*" + re.escape(first_param_type) + r"\b", head):
            continue
        start = m.start() + 1
        # docblock / attributes / comments directly above
        before = s[:start]
        k = len(before)
        while True:
            prev_nl = before.rfind("\n", 0, k - 1)
            line = before[prev_nl + 1:k - 1] if k > 0 else ''
            st = line.strip()
            if st.startswith(('/**', '*', '*/', '#[', '//')) and st != '':
                k = prev_nl + 1
                continue
            break
        start = k
        end = s.index("\n    }\n", m.end()) + len("\n    }\n")
        return start, end
    return None

def delete_method(s, name, first_param_type=None):
    sp = method_span(s, name, first_param_type)
    assert sp, (name, first_param_type)
    a, b = sp
    # also drop one blank line left behind
    if s[b:b + 1] == "\n":
        b += 1
    return s[:a] + s[b:]
