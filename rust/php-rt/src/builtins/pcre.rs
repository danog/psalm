//! preg_* functions on top of PCRE2.

use crate::error::RtError;
use crate::key::ArrayKey;
use crate::list::List;
use crate::map::Map;
use crate::mixed::Mixed;
use crate::string::Str;
use pcre2::bytes::{Regex, RegexBuilder};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc as Rc;

thread_local! {
    static CACHE: RefCell<HashMap<Vec<u8>, Rc<Compiled>>> = RefCell::new(HashMap::new());
    static LAST_ERROR: RefCell<(i64, Str)> = RefCell::new((0, Str::from_static("No error")));
}

pub struct Compiled {
    pub re: Regex,
    pub names: Vec<Option<Str>>,
}

fn set_error(code: i64, msg: &str) {
    LAST_ERROR.with(|e| *e.borrow_mut() = (code, Str::from_str(msg)));
}

pub fn preg_last_error() -> i64 {
    LAST_ERROR.with(|e| e.borrow().0)
}
pub fn preg_last_error_msg() -> Str {
    LAST_ERROR.with(|e| e.borrow().1.clone())
}

/// Parse a PHP regex literal (`/pat/flags`) and compile it.
pub fn compile(pattern: &Str) -> Result<Rc<Compiled>, RtError> {
    if let Some(c) = CACHE.with(|c| c.borrow().get(pattern.as_bytes()).cloned()) {
        return Ok(c);
    }
    let p = pattern.as_bytes();
    let mut i = 0;
    while i < p.len() && crate::conv::is_ws(p[i]) {
        i += 1;
    }
    if i >= p.len() {
        return Err(RtError::value_error("preg: empty regular expression"));
    }
    let delim = p[i];
    if delim.is_ascii_alphanumeric() || delim == b'\\' {
        return Err(RtError::value_error("preg: delimiter must not be alphanumeric, backslash, or NUL"));
    }
    let end_delim = match delim {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        d => d,
    };
    let start = i + 1;
    let mut end = None;
    let mut j = start;
    let mut depth = 0;
    while j < p.len() {
        if p[j] == b'\\' {
            j += 2;
            continue;
        }
        if end_delim != delim && p[j] == delim {
            depth += 1;
        } else if p[j] == end_delim {
            if depth == 0 {
                end = Some(j);
                break;
            }
            depth -= 1;
        }
        j += 1;
    }
    let end = match end {
        Some(e) => e,
        None => return Err(RtError::value_error("preg: no ending delimiter found")),
    };
    let body = &p[start..end];
    let flags = &p[end + 1..];
    let mut b = RegexBuilder::new();
    let mut inline_opts = String::new();
    let mut jit = true;
    for &f in flags {
        match f {
            b'i' => {
                b.caseless(true);
            }
            b'm' => {
                b.multi_line(true);
            }
            b's' => {
                b.dotall(true);
            }
            b'x' => {
                b.extended(true);
            }
            b'u' => {
                b.utf(true);
                b.ucp(true);
            }
            b'U' => {
                inline_opts.push_str("U");
            }
            b'D' => {}
            b'A' => {
                // anchored: emulate by prefixing \G? we use \A wrapping instead
            }
            b'S' | b'X' | b'J' | b'\n' | b' ' | b'\r' | b'\t' => {}
            _ => {
                set_error(1, "Unknown modifier");
                return Err(RtError::value_error(crate::sfmt!("preg: unknown modifier '{}'", f as char)));
            }
        }
    }
    let mut src = body.to_vec();
    if !inline_opts.is_empty() {
        let mut s = format!("(?{})", inline_opts).into_bytes();
        s.extend_from_slice(&src);
        src = s;
    }
    if flags.contains(&b'A') {
        let mut s = b"\\A(?:".to_vec();
        s.extend_from_slice(body);
        s.push(b')');
        src = s;
    }
    b.jit_if_available(jit);
    jit = false;
    let _ = jit;
    let re = match b.build(std::str::from_utf8(&src).map_err(|_| RtError::value_error("preg: pattern is not valid UTF-8"))?) {
        Ok(r) => r,
        Err(e) => {
            set_error(1, &e.to_string());
            return Err(RtError::value_error(crate::sfmt!("preg_match(): Compilation failed: {}", e)));
        }
    };
    let mut names: Vec<Option<Str>> = vec![None; re.captures_len()];
    for (i, n) in re.capture_names().iter().enumerate() {
        if let Some(n) = n {
            names[i] = Some(Str::from_str(n));
        }
    }
    let c = Rc::new(Compiled { re, names });
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if cache.len() > 4096 {
            cache.clear();
        }
        cache.insert(pattern.to_vec(), c.clone());
    });
    Ok(c)
}

type Spans = Vec<Option<(usize, usize)>>;

fn spans_of(caps: &pcre2::bytes::Captures<'_>) -> Spans {
    (0..caps.len()).map(|i| caps.get(i).map(|g| (g.start(), g.end()))).collect()
}

fn spans_of_locs(locs: &pcre2::bytes::CaptureLocations) -> Spans {
    (0..locs.len()).map(|i| locs.get(i)).collect()
}

fn groups_map(c: &Compiled, caps: &Spans, subject: &[u8], offset_capture: bool, unmatched_null: bool, trailing: bool) -> Map<ArrayKey, Mixed> {
    let mut m: Map<ArrayKey, Mixed> = Map::new();
    let n = caps.len();
    // PHP trims trailing unmatched groups unless PREG_UNMATCHED_AS_NULL
    let mut last = n;
    if !unmatched_null && !trailing {
        while last > 1 && caps[last - 1].is_none() {
            last -= 1;
        }
    }
    for i in 0..last {
        let val = match caps[i] {
            Some((gs, ge)) => {
                let s = Mixed::Str(Str::from_bytes(&subject[gs..ge]));
                if offset_capture {
                    let mut pair: Map<ArrayKey, Mixed> = Map::new();
                    pair.push(s);
                    pair.push(Mixed::Int(gs as i64));
                    Mixed::Arr(pair)
                } else {
                    s
                }
            }
            None => {
                if unmatched_null {
                    Mixed::Null
                } else if offset_capture {
                    let mut pair: Map<ArrayKey, Mixed> = Map::new();
                    pair.push(Mixed::Str(Str::empty()));
                    pair.push(Mixed::Int(-1));
                    Mixed::Arr(pair)
                } else {
                    Mixed::Str(Str::empty())
                }
            }
        };
        if let Some(Some(name)) = c.names.get(i) {
            m.insert(ArrayKey::Str(name.clone()), val.clone());
        }
        m.insert(ArrayKey::Int(i as i64), val);
    }
    m
}

pub fn preg_match(pattern: &Str, subject: &Str, offset: i64) -> Result<i64, RtError> {
    let c = compile(pattern)?;
    let off = offset.max(0) as usize;
    if off > subject.len() {
        return Ok(0);
    }
    let mut locs = c.re.capture_locations();
    match c.re.captures_read_at(&mut locs, subject, off) {
        Ok(Some(_)) => Ok(1),
        Ok(None) => Ok(0),
        Err(e) => {
            set_error(2, &e.to_string());
            Ok(0)
        }
    }
}

/// preg_match with $matches: returns (result, groups as string map).
pub fn preg_match_groups(pattern: &Str, subject: &Str, offset: i64) -> Result<(i64, Map<ArrayKey, Str>), RtError> {
    let (r, m) = preg_match_groups_flags(pattern, subject, 0, offset)?;
    Ok((r, m.into_iter().map(|(k, v)| (k, crate::traits::ToStr::to_php_str(&v))).collect()))
}

pub fn preg_match_groups_flags(pattern: &Str, subject: &Str, flags: i64, offset: i64) -> Result<(i64, Map<ArrayKey, Mixed>), RtError> {
    let c = compile(pattern)?;
    let off = offset.max(0) as usize;
    if off > subject.len() {
        return Ok((0, Map::new()));
    }
    let mut locs = c.re.capture_locations();
    match c.re.captures_read_at(&mut locs, subject, off) {
        Ok(Some(_)) => {
            let m = groups_map(&c, &spans_of_locs(&locs), subject, flags & 256 != 0, flags & 512 != 0, false);
            Ok((1, m))
        }
        Ok(None) => Ok((0, Map::new())),
        Err(e) => {
            set_error(2, &e.to_string());
            Ok((0, Map::new()))
        }
    }
}

pub fn preg_match_all(pattern: &Str, subject: &Str, flags: i64) -> Result<(i64, Map<ArrayKey, Mixed>), RtError> {
    let c = compile(pattern)?;
    let set_order = flags & 2 != 0;
    let offset_capture = flags & 256 != 0;
    let unmatched_null = flags & 512 != 0;
    let mut count = 0i64;
    let mut sets: Vec<Map<ArrayKey, Mixed>> = Vec::new();
    for caps in c.re.captures_iter(subject) {
        let caps = caps.map_err(|e| RtError::error(crate::sfmt!("preg_match_all: {}", e)))?;
        count += 1;
        sets.push(groups_map(&c, &spans_of(&caps), subject, offset_capture, unmatched_null, !set_order));
    }
    if set_order {
        let mut out: Map<ArrayKey, Mixed> = Map::new();
        for s in sets {
            out.push(Mixed::Arr(s));
        }
        return Ok((count, out));
    }
    // pattern order: one array per group
    let mut out: Map<ArrayKey, Mixed> = Map::new();
    let ngroups = c.re.captures_len();
    for i in 0..ngroups {
        let mut col: Map<ArrayKey, Mixed> = Map::new();
        for s in &sets {
            col.push(s.get(&ArrayKey::Int(i as i64)).cloned().unwrap_or(Mixed::Str(Str::empty())));
        }
        if let Some(Some(name)) = c.names.get(i) {
            out.insert(ArrayKey::Str(name.clone()), Mixed::Arr(col.clone()));
        }
        out.insert(ArrayKey::Int(i as i64), Mixed::Arr(col));
    }
    Ok((count, out))
}

/// Expand a PHP replacement string (`$1`, `\1`, `${1}`) for one match.
fn expand(repl: &[u8], caps: &pcre2::bytes::Captures<'_>, subject: &[u8], out: &mut Vec<u8>) {
    let mut i = 0;
    while i < repl.len() {
        let c = repl[i];
        if (c == b'$' || c == b'\\') && i + 1 < repl.len() {
            let mut j = i + 1;
            let braced = c == b'$' && repl[j] == b'{';
            if braced {
                j += 1;
            }
            let start = j;
            while j < repl.len() && repl[j].is_ascii_digit() && j - start < 2 {
                j += 1;
            }
            if j > start && (!braced || (j < repl.len() && repl[j] == b'}')) {
                let n: usize = std::str::from_utf8(&repl[start..j]).unwrap().parse().unwrap();
                if let Some(g) = caps.get(n) {
                    out.extend_from_slice(&subject[g.start()..g.end()]);
                }
                i = if braced { j + 1 } else { j };
                continue;
            }
            if c == b'\\' && repl[i + 1] == b'\\' {
                out.push(b'\\');
                i += 2;
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
}

fn replace_impl<F: FnMut(&pcre2::bytes::Captures<'_>, &mut Vec<u8>) -> Result<(), RtError>>(c: &Compiled, subject: &[u8], limit: i64, mut f: F) -> Result<(Vec<u8>, i64), RtError> {
    let mut out = Vec::with_capacity(subject.len());
    let mut last = 0;
    let mut n = 0i64;
    for caps in c.re.captures_iter(subject) {
        if limit >= 0 && n >= limit {
            break;
        }
        let caps = caps.map_err(|e| RtError::error(crate::sfmt!("preg_replace: {}", e)))?;
        let m = caps.get(0).unwrap();
        out.extend_from_slice(&subject[last..m.start()]);
        f(&caps, &mut out)?;
        last = m.end();
        n += 1;
    }
    out.extend_from_slice(&subject[last..]);
    Ok((out, n))
}

pub fn preg_replace(pattern: &Str, replacement: &Str, subject: &Str, limit: i64) -> Result<Str, RtError> {
    let c = compile(pattern)?;
    let (out, _) = replace_impl(&c, subject, limit, |caps, out| {
        expand(replacement, caps, subject, out);
        Ok(())
    })?;
    Ok(Str::from_vec(out))
}

pub fn preg_replace_arr(patterns: &List<Str>, replacements: &List<Str>, subject: &Str, limit: i64) -> Result<Str, RtError> {
    let mut cur = subject.clone();
    for (i, p) in patterns.iter().enumerate() {
        let r = replacements.get(i as i64).cloned().unwrap_or_default();
        cur = preg_replace(p, &r, &cur, limit)?;
    }
    Ok(cur)
}

pub fn preg_replace_arr_s(patterns: &List<Str>, replacement: &Str, subject: &Str, limit: i64) -> Result<Str, RtError> {
    let mut cur = subject.clone();
    for p in patterns.iter() {
        cur = preg_replace(p, replacement, &cur, limit)?;
    }
    Ok(cur)
}

pub fn preg_replace_callback<E, F: FnMut(Map<ArrayKey, Str>) -> Result<Str, E>>(pattern: &Str, subject: &Str, limit: i64, mut f: F) -> Result<Str, E>
where
    E: From<RtError>,
{
    let c = compile(pattern)?;
    let mut err: Option<E> = None;
    let res = replace_impl(&c, subject, limit, |caps, out| {
        let m = groups_map(&c, &spans_of(caps), subject, false, false, false);
        let m: Map<ArrayKey, Str> = m.into_iter().map(|(k, v)| (k, crate::traits::ToStr::to_php_str(&v))).collect();
        match f(m) {
            Ok(s) => {
                out.extend_from_slice(&s);
                Ok(())
            }
            Err(e) => {
                err = Some(e);
                Err(RtError::error("callback failed"))
            }
        }
    });
    if let Some(e) = err {
        return Err(e);
    }
    let (out, _) = res?;
    Ok(Str::from_vec(out))
}

pub fn preg_split(pattern: &Str, subject: &Str, limit: i64, flags: i64) -> Result<List<Str>, RtError> {
    let c = compile(pattern)?;
    let no_empty = flags & 1 != 0;
    let delim_capture = flags & 2 != 0;
    let limit = if limit == 0 { -1 } else { limit };
    let mut out: Vec<Str> = Vec::new();
    let mut last = 0usize;
    let mut pieces = 0i64;
    let mut it = c.re.captures_iter(subject);
    loop {
        if limit > 0 && pieces + 1 >= limit {
            break;
        }
        let caps = match it.next() {
            Some(Ok(c)) => c,
            Some(Err(e)) => return Err(RtError::error(crate::sfmt!("preg_split: {}", e))),
            None => break,
        };
        let m = caps.get(0).unwrap();
        if m.start() == m.end() && m.start() == last && (last == 0 || last == subject.len()) && no_empty {
            // empty match at boundary yields empty piece: skip
        }
        let piece = &subject[last..m.start()];
        if !(no_empty && piece.is_empty()) {
            out.push(Str::from_bytes(piece));
            pieces += 1;
        }
        if delim_capture {
            for i in 1..caps.len() {
                if let Some(g) = caps.get(i) {
                    let s = &subject[g.start()..g.end()];
                    if !(no_empty && s.is_empty()) {
                        out.push(Str::from_bytes(s));
                    }
                }
            }
        }
        last = m.end();
        if m.start() == m.end() {
            // avoid infinite loop on empty matches: pcre2 iterator advances itself
        }
    }
    let rest = &subject[last..];
    if !(no_empty && rest.is_empty()) {
        out.push(Str::from_bytes(rest));
    }
    Ok(List::from_vec(out))
}

pub fn preg_split_offsets(pattern: &Str, subject: &Str, limit: i64, flags: i64) -> Result<List<(Str, i64)>, RtError> {
    let c = compile(pattern)?;
    let no_empty = flags & 1 != 0;
    let mut out: Vec<(Str, i64)> = Vec::new();
    let mut last = 0usize;
    let mut pieces = 0i64;
    for caps in c.re.captures_iter(subject) {
        if limit > 0 && pieces + 1 >= limit {
            break;
        }
        let caps = caps.map_err(|e| RtError::error(crate::sfmt!("preg_split: {}", e)))?;
        let m = caps.get(0).unwrap();
        let piece = &subject[last..m.start()];
        if !(no_empty && piece.is_empty()) {
            out.push((Str::from_bytes(piece), last as i64));
            pieces += 1;
        }
        last = m.end();
    }
    let rest = &subject[last..];
    if !(no_empty && rest.is_empty()) {
        out.push((Str::from_bytes(rest), last as i64));
    }
    Ok(List::from_vec(out))
}

pub fn preg_grep<K: crate::key::MapKey>(pattern: &Str, input: &Map<K, Str>) -> Result<Map<K, Str>, RtError> {
    let c = compile(pattern)?;
    let mut out = Map::new();
    for (k, v) in input.iter() {
        if c.re.is_match(v).unwrap_or(false) {
            out.insert(k.clone(), v.clone());
        }
    }
    Ok(out)
}

pub fn preg_quote(s: &Str, delim: Option<&Str>) -> Str {
    let mut out = Vec::with_capacity(s.len() + 8);
    for &c in s.as_bytes() {
        if b".\\+*?[^]$(){}=!<>|:-#/".contains(&c) || delim.map_or(false, |d| d.as_bytes().contains(&c)) {
            if c == b'/' && !delim.map_or(false, |d| d.as_bytes().contains(&b'/')) {
                out.push(c);
                continue;
            }
            out.push(b'\\');
            out.push(c);
        } else if c == 0 {
            out.extend_from_slice(b"\\000");
        } else {
            out.push(c);
        }
    }
    Str::from_vec(out)
}
