//! String functions.

use crate::Mixed;
use crate::conv;
use crate::error::RtError;
use crate::key::ArrayKey;
use crate::list::List;
use crate::map::Map;
use crate::string::{find_bytes, rfind_bytes, Str};
use crate::traits::ToStr;

#[inline]
pub fn strlen(s: &Str) -> i64 {
    s.len() as i64
}
pub fn strtolower(s: &Str) -> Str {
    s.to_lowercase()
}
pub fn strtoupper(s: &Str) -> Str {
    s.to_uppercase()
}
pub fn ucfirst(s: &Str) -> Str {
    let b = s.as_bytes();
    if b.is_empty() || !b[0].is_ascii_lowercase() {
        return s.clone();
    }
    let mut v = b.to_vec();
    v[0] = v[0].to_ascii_uppercase();
    Str::from_vec(v)
}
pub fn lcfirst(s: &Str) -> Str {
    let b = s.as_bytes();
    if b.is_empty() || !b[0].is_ascii_uppercase() {
        return s.clone();
    }
    let mut v = b.to_vec();
    v[0] = v[0].to_ascii_lowercase();
    Str::from_vec(v)
}
pub fn ucwords(s: &Str) -> Str {
    let mut v = s.to_vec();
    let mut cap = true;
    for c in v.iter_mut() {
        if cap && c.is_ascii_lowercase() {
            *c = c.to_ascii_uppercase();
        }
        cap = matches!(*c, b' ' | b'\t' | b'\r' | b'\n' | 0x0c | 0x0b);
    }
    Str::from_vec(v)
}
pub fn strrev(s: &Str) -> Str {
    let mut v = s.to_vec();
    v.reverse();
    Str::from_vec(v)
}

fn resolve_range(len: usize, offset: i64, length: Option<i64>) -> Option<(usize, usize)> {
    let len_i = len as i64;
    let mut start = if offset < 0 { (len_i + offset).max(0) } else { offset };
    if start > len_i {
        start = len_i;
    }
    let end = match length {
        None => len_i,
        Some(l) if l < 0 => (len_i + l).max(start),
        Some(l) => (start + l).min(len_i),
    };
    if end < start {
        return Some((start as usize, start as usize));
    }
    Some((start as usize, end as usize))
}
pub fn substr(s: &Str, offset: i64, length: Option<i64>) -> Str {
    match resolve_range(s.len(), offset, length) {
        Some((a, b)) => s.slice(a, b),
        None => Str::empty(),
    }
}
pub fn substr_count(hay: &Str, needle: &Str) -> i64 {
    if needle.is_empty() {
        return 0;
    }
    let mut n = 0;
    let mut pos = 0;
    while let Some(p) = find_bytes(hay, needle, pos) {
        n += 1;
        pos = p + needle.len();
    }
    n
}
fn norm_offset(len: usize, offset: i64) -> Option<usize> {
    if offset < 0 {
        let o = len as i64 + offset;
        if o < 0 { None } else { Some(o as usize) }
    } else if offset as usize > len {
        None
    } else {
        Some(offset as usize)
    }
}
pub fn strpos(hay: &Str, needle: &Str, offset: i64) -> Option<i64> {
    let from = norm_offset(hay.len(), offset)?;
    find_bytes(hay, needle, from).map(|p| p as i64)
}
pub fn stripos(hay: &Str, needle: &Str, offset: i64) -> Option<i64> {
    let from = norm_offset(hay.len(), offset)?;
    let h = hay.as_bytes().to_ascii_lowercase();
    let n = needle.as_bytes().to_ascii_lowercase();
    find_bytes(&h, &n, from).map(|p| p as i64)
}
pub fn strrpos(hay: &Str, needle: &Str, offset: i64) -> Option<i64> {
    let h = hay.as_bytes();
    if offset >= 0 {
        let from = offset as usize;
        if from > h.len() {
            return None;
        }
        rfind_bytes(h, needle, h.len()).filter(|&p| p >= from).map(|p| p as i64)
    } else {
        let end = (h.len() as i64 + offset + needle.len() as i64).max(0) as usize;
        rfind_bytes(h, needle, end.min(h.len())).map(|p| p as i64)
    }
}
pub fn strripos(hay: &Str, needle: &Str, offset: i64) -> Option<i64> {
    let h = Str::from_vec(hay.as_bytes().to_ascii_lowercase());
    let n = Str::from_vec(needle.as_bytes().to_ascii_lowercase());
    strrpos(&h, &n, offset)
}
pub fn strstr(hay: &Str, needle: &Str, before: bool) -> Option<Str> {
    let p = find_bytes(hay, needle, 0)?;
    Some(if before { hay.slice(0, p) } else { hay.slice(p, hay.len()) })
}
pub fn strrchr(hay: &Str, needle: &Str) -> Option<Str> {
    let c = *needle.as_bytes().first()?;
    let p = hay.as_bytes().iter().rposition(|&x| x == c)?;
    Some(hay.slice(p, hay.len()))
}
#[inline]
pub fn str_starts_with(s: &Str, p: &Str) -> bool {
    s.as_bytes().starts_with(p)
}
#[inline]
pub fn str_ends_with(s: &Str, p: &Str) -> bool {
    s.as_bytes().ends_with(p)
}
#[inline]
pub fn str_contains(s: &Str, p: &Str) -> bool {
    find_bytes(s, p, 0).is_some()
}
pub fn str_repeat(s: &Str, n: i64) -> Str {
    if n <= 0 {
        return Str::empty();
    }
    Str::from_vec(s.as_bytes().repeat(n as usize))
}
pub fn replace_bytes(subject: &[u8], search: &[u8], replace: &[u8], count: &mut i64) -> Vec<u8> {
    if search.is_empty() {
        return subject.to_vec();
    }
    let mut out = Vec::with_capacity(subject.len());
    let mut pos = 0;
    while let Some(p) = find_bytes(subject, search, pos) {
        out.extend_from_slice(&subject[pos..p]);
        out.extend_from_slice(replace);
        pos = p + search.len();
        *count += 1;
    }
    out.extend_from_slice(&subject[pos..]);
    out
}
pub fn str_replace(search: &Str, replace: &Str, subject: &Str) -> Str {
    let mut c = 0;
    if find_bytes(subject, search, 0).is_none() {
        return subject.clone();
    }
    Str::from_vec(replace_bytes(subject, search, replace, &mut c))
}
/// str_replace with array search and string/array replace.
/// `str_replace` with dynamically typed arguments (string or array search/replace/subject).
pub fn str_replace_m(search: &Mixed, replace: &Mixed, subject: &Mixed) -> Mixed {
    let one = |s: &Str| -> Str {
        match (search, replace) {
            (Mixed::Arr(sa), Mixed::Arr(ra)) => {
                let sl: List<Str> = sa.values().map(|v| v.to_php_str()).collect();
                let rl: List<Str> = ra.values().map(|v| v.to_php_str()).collect();
                str_replace_arr(&sl, &rl, s)
            }
            (Mixed::Arr(sa), r) => {
                let sl: List<Str> = sa.values().map(|v| v.to_php_str()).collect();
                str_replace_arr_s(&sl, &r.to_php_str(), s)
            }
            (se, r) => str_replace(&se.to_php_str(), &r.to_php_str(), s),
        }
    };
    match subject {
        Mixed::Arr(m) => Mixed::Arr(m.clone().map_values(|v| Mixed::Str(one(&v.to_php_str())))),
        other => Mixed::Str(one(&other.to_php_str())),
    }
}

pub fn str_replace_arr(search: &List<Str>, replace: &List<Str>, subject: &Str) -> Str {
    let mut cur = subject.to_vec();
    let mut c = 0;
    for (i, s) in search.iter().enumerate() {
        let r: &[u8] = replace.get(i as i64).map(|r| r.as_bytes()).unwrap_or(b"");
        cur = replace_bytes(&cur, s, r, &mut c);
    }
    Str::from_vec(cur)
}
pub fn str_replace_arr_s(search: &List<Str>, replace: &Str, subject: &Str) -> Str {
    let mut cur = subject.to_vec();
    let mut c = 0;
    for s in search.iter() {
        cur = replace_bytes(&cur, s, replace, &mut c);
    }
    Str::from_vec(cur)
}
pub fn str_replace_count(search: &Str, replace: &Str, subject: &Str, count: &mut i64) -> Str {
    Str::from_vec(replace_bytes(subject, search, replace, count))
}
pub fn str_ireplace(search: &Str, replace: &Str, subject: &Str) -> Str {
    let ls = search.as_bytes().to_ascii_lowercase();
    if ls.is_empty() {
        return subject.clone();
    }
    let lsub = subject.as_bytes().to_ascii_lowercase();
    let mut out = Vec::new();
    let mut pos = 0;
    while let Some(p) = find_bytes(&lsub, &ls, pos) {
        out.extend_from_slice(&subject.as_bytes()[pos..p]);
        out.extend_from_slice(replace);
        pos = p + ls.len();
    }
    out.extend_from_slice(&subject.as_bytes()[pos..]);
    Str::from_vec(out)
}
pub fn substr_replace(s: &Str, replace: &Str, offset: i64, length: Option<i64>) -> Str {
    let (a, b) = resolve_range(s.len(), offset, length).unwrap_or((0, 0));
    let mut out = Vec::with_capacity(s.len() + replace.len());
    out.extend_from_slice(&s.as_bytes()[..a]);
    out.extend_from_slice(replace);
    out.extend_from_slice(&s.as_bytes()[b..]);
    Str::from_vec(out)
}

fn trim_mask(chars: Option<&Str>) -> Vec<u8> {
    match chars {
        None => b" \t\n\r\0\x0B".to_vec(),
        Some(c) => {
            let b = c.as_bytes();
            let mut out = Vec::new();
            let mut i = 0;
            while i < b.len() {
                if i + 3 < b.len() && b[i + 1] == b'.' && b[i + 2] == b'.' {
                    for x in b[i]..=b[i + 3] {
                        out.push(x);
                    }
                    i += 4;
                } else {
                    out.push(b[i]);
                    i += 1;
                }
            }
            out
        }
    }
}
pub fn trim(s: &Str, chars: Option<&Str>) -> Str {
    let m = trim_mask(chars);
    let b = s.as_bytes();
    let mut a = 0;
    while a < b.len() && m.contains(&b[a]) {
        a += 1;
    }
    let mut e = b.len();
    while e > a && m.contains(&b[e - 1]) {
        e -= 1;
    }
    if a == 0 && e == b.len() { s.clone() } else { s.slice(a, e) }
}
pub fn ltrim(s: &Str, chars: Option<&Str>) -> Str {
    let m = trim_mask(chars);
    let b = s.as_bytes();
    let mut a = 0;
    while a < b.len() && m.contains(&b[a]) {
        a += 1;
    }
    if a == 0 { s.clone() } else { s.slice(a, b.len()) }
}
pub fn rtrim(s: &Str, chars: Option<&Str>) -> Str {
    let m = trim_mask(chars);
    let b = s.as_bytes();
    let mut e = b.len();
    while e > 0 && m.contains(&b[e - 1]) {
        e -= 1;
    }
    if e == b.len() { s.clone() } else { s.slice(0, e) }
}
pub fn explode(sep: &Str, s: &Str, limit: i64) -> Result<List<Str>, RtError> {
    if sep.is_empty() {
        return Err(RtError::value_error("explode(): Argument #1 ($separator) cannot be empty"));
    }
    let b = s.as_bytes();
    let mut parts: Vec<Str> = Vec::new();
    let mut pos = 0;
    if limit > 0 {
        while parts.len() + 1 < limit as usize {
            match find_bytes(b, sep, pos) {
                Some(p) => {
                    parts.push(s.slice(pos, p));
                    pos = p + sep.len();
                }
                None => break,
            }
        }
        parts.push(s.slice(pos, b.len()));
    } else {
        while let Some(p) = find_bytes(b, sep, pos) {
            parts.push(s.slice(pos, p));
            pos = p + sep.len();
        }
        parts.push(s.slice(pos, b.len()));
        if limit < 0 {
            let drop = (-limit) as usize;
            if drop >= parts.len() {
                parts.clear();
            } else {
                parts.truncate(parts.len() - drop);
            }
        }
    }
    Ok(List::from_vec(parts))
}
pub fn implode<T: ToStr>(sep: &Str, parts: &[T]) -> Str {
    let mut out = Vec::new();
    for (i, p) in parts.iter().enumerate() {
        if i > 0 {
            out.extend_from_slice(sep);
        }
        out.extend_from_slice(p.to_php_str().as_bytes());
    }
    Str::from_vec(out)
}
pub fn implode_m<K: crate::key::MapKey, T: ToStr>(sep: &Str, parts: &Map<K, T>) -> Str {
    let mut out = Vec::new();
    for (i, (_, p)) in parts.iter().enumerate() {
        if i > 0 {
            out.extend_from_slice(sep);
        }
        out.extend_from_slice(p.to_php_str().as_bytes());
    }
    Str::from_vec(out)
}
pub fn str_split(s: &Str, len: i64) -> List<Str> {
    let n = len.max(1) as usize;
    if s.is_empty() {
        return List::from_vec(vec![Str::empty()]);
    }
    s.as_bytes().chunks(n).map(Str::from_bytes).collect()
}
pub fn str_pad(s: &Str, len: i64, pad: &Str, pad_type: i64) -> Str {
    let cur = s.len() as i64;
    if len <= cur || pad.is_empty() {
        return s.clone();
    }
    let total = (len - cur) as usize;
    let mk = |n: usize| -> Vec<u8> {
        let mut v = Vec::with_capacity(n);
        while v.len() < n {
            let take = (n - v.len()).min(pad.len());
            v.extend_from_slice(&pad.as_bytes()[..take]);
        }
        v
    };
    let mut out = Vec::with_capacity(len as usize);
    match pad_type {
        0 => {
            // STR_PAD_LEFT
            out.extend(mk(total));
            out.extend_from_slice(s);
        }
        2 => {
            // STR_PAD_BOTH
            let left = total / 2;
            out.extend(mk(left));
            out.extend_from_slice(s);
            out.extend(mk(total - left));
        }
        _ => {
            out.extend_from_slice(s);
            out.extend(mk(total));
        }
    }
    Str::from_vec(out)
}
pub fn ord(s: &Str) -> i64 {
    s.as_bytes().first().copied().unwrap_or(0) as i64
}
pub fn chr(c: i64) -> Str {
    let b = (c.rem_euclid(256)) as u8;
    Str::from_vec(vec![b])
}
pub fn strcmp(a: &Str, b: &Str) -> i64 {
    match a.as_bytes().cmp(b.as_bytes()) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}
pub fn strcasecmp(a: &Str, b: &Str) -> i64 {
    let x = a.as_bytes().to_ascii_lowercase();
    let y = b.as_bytes().to_ascii_lowercase();
    match x.cmp(&y) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}
pub fn strncmp(a: &Str, b: &Str, n: i64) -> i64 {
    let n = n.max(0) as usize;
    let x = &a.as_bytes()[..n.min(a.len())];
    let y = &b.as_bytes()[..n.min(b.len())];
    match x.cmp(y) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}
pub fn strncasecmp(a: &Str, b: &Str, n: i64) -> i64 {
    strncmp(&a.to_lowercase(), &b.to_lowercase(), n)
}
pub fn substr_compare(a: &Str, b: &Str, offset: i64, length: Option<i64>, ci: bool) -> i64 {
    let sub = substr(a, offset, length);
    let bb = match length {
        Some(l) => substr(b, 0, Some(l)),
        None => b.clone(),
    };
    if ci { strcasecmp(&sub, &bb) } else { strcmp(&sub, &bb) }
}
pub fn strtr(s: &Str, from: &Str, to: &Str) -> Str {
    let n = from.len().min(to.len());
    if n == 0 {
        return s.clone();
    }
    let mut table = [None::<u8>; 256];
    for i in 0..n {
        table[from.as_bytes()[i] as usize] = Some(to.as_bytes()[i]);
    }
    let v: Vec<u8> = s.as_bytes().iter().map(|&c| table[c as usize].unwrap_or(c)).collect();
    Str::from_vec(v)
}
pub fn strtr_pairs(s: &Str, pairs: &Map<Str, Str>) -> Str {
    let mut keys: Vec<(&Str, &Str)> = pairs.iter().filter(|(k, _)| !k.is_empty()).collect();
    keys.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    'outer: while i < b.len() {
        for (k, v) in &keys {
            if b[i..].starts_with(k) {
                out.extend_from_slice(v);
                i += k.len();
                continue 'outer;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    Str::from_vec(out)
}
pub fn strspn(s: &Str, mask: &Str) -> i64 {
    s.as_bytes().iter().take_while(|c| mask.as_bytes().contains(c)).count() as i64
}
pub fn strcspn(s: &Str, mask: &Str) -> i64 {
    s.as_bytes().iter().take_while(|c| !mask.as_bytes().contains(c)).count() as i64
}
pub fn strpbrk(s: &Str, chars: &Str) -> Option<Str> {
    let p = s.as_bytes().iter().position(|c| chars.as_bytes().contains(c))?;
    Some(s.slice(p, s.len()))
}
pub fn wordwrap(s: &Str, width: i64, brk: &Str, cut: bool) -> Str {
    // Port of php_wordwrap (general case).
    let text = s.as_bytes();
    let width = width.max(0) as usize;
    let breakchar = brk.as_bytes();
    if text.is_empty() {
        return Str::empty();
    }
    if breakchar.is_empty() {
        return s.clone();
    }
    let mut out = Vec::with_capacity(text.len());
    let (mut laststart, mut lastspace) = (0usize, 0usize);
    let mut current = 0usize;
    while current < text.len() {
        let c = text[current];
        if c == breakchar[0] && current + breakchar.len() <= text.len() && &text[current..current + breakchar.len()] == breakchar {
            out.extend_from_slice(&text[laststart..current + breakchar.len()]);
            current += breakchar.len() - 1;
            laststart = current + 1;
            lastspace = laststart;
        } else if c == b' ' {
            if current - laststart >= width {
                out.extend_from_slice(&text[laststart..current]);
                out.extend_from_slice(breakchar);
                laststart = current + 1;
            }
            lastspace = current;
        } else if current - laststart >= width && cut && laststart >= lastspace {
            out.extend_from_slice(&text[laststart..current]);
            out.extend_from_slice(breakchar);
            laststart = current;
            lastspace = current;
        } else if current - laststart >= width && laststart < lastspace {
            out.extend_from_slice(&text[laststart..lastspace]);
            out.extend_from_slice(breakchar);
            laststart = lastspace + 1;
            lastspace = laststart;
        }
        current += 1;
    }
    if laststart != current {
        out.extend_from_slice(&text[laststart..current]);
    }
    Str::from_vec(out)
}
pub fn addslashes(s: &Str) -> Str {
    let mut out = Vec::with_capacity(s.len());
    for &c in s.as_bytes() {
        match c {
            b'\'' | b'"' | b'\\' => {
                out.push(b'\\');
                out.push(c);
            }
            0 => out.extend_from_slice(b"\\0"),
            _ => out.push(c),
        }
    }
    Str::from_vec(out)
}
pub fn stripslashes(s: &Str) -> Str {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' && i + 1 < b.len() {
            i += 1;
            if b[i] == b'0' {
                out.push(0);
            } else {
                out.push(b[i]);
            }
        } else if b[i] != b'\\' {
            out.push(b[i]);
        }
        i += 1;
    }
    Str::from_vec(out)
}
pub fn addcslashes(s: &Str, chars: &Str) -> Str {
    let mask = trim_mask(Some(chars));
    let mut out = Vec::with_capacity(s.len());
    for &c in s.as_bytes() {
        if mask.contains(&c) {
            match c {
                b'\n' => out.extend_from_slice(b"\\n"),
                b'\t' => out.extend_from_slice(b"\\t"),
                b'\r' => out.extend_from_slice(b"\\r"),
                0x07 => out.extend_from_slice(b"\\a"),
                0x0b => out.extend_from_slice(b"\\v"),
                0x08 => out.extend_from_slice(b"\\b"),
                0x0c => out.extend_from_slice(b"\\f"),
                _ if c < 32 || c > 126 => out.extend_from_slice(format!("\\{:03o}", c).as_bytes()),
                _ => {
                    out.push(b'\\');
                    out.push(c);
                }
            }
        } else {
            out.push(c);
        }
    }
    Str::from_vec(out)
}
pub fn htmlspecialchars(s: &Str) -> Str {
    let mut out = Vec::with_capacity(s.len());
    for &c in s.as_bytes() {
        match c {
            b'&' => out.extend_from_slice(b"&amp;"),
            b'<' => out.extend_from_slice(b"&lt;"),
            b'>' => out.extend_from_slice(b"&gt;"),
            b'"' => out.extend_from_slice(b"&quot;"),
            b'\'' => out.extend_from_slice(b"&#039;"),
            _ => out.push(c),
        }
    }
    Str::from_vec(out)
}
pub fn strip_tags(s: &Str) -> Str {
    let mut out = Vec::with_capacity(s.len());
    let mut depth = 0;
    for &c in s.as_bytes() {
        match c {
            b'<' => depth += 1,
            b'>' if depth > 0 => depth -= 1,
            _ if depth == 0 => out.push(c),
            _ => {}
        }
    }
    Str::from_vec(out)
}
pub fn nl2br(s: &Str) -> Str {
    str_replace(&Str::from_static("\n"), &Str::from_static("<br />\n"), s)
}
pub fn bin2hex(s: &Str) -> Str {
    let mut out = String::with_capacity(s.len() * 2);
    for c in s.as_bytes() {
        out.push_str(&format!("{:02x}", c));
    }
    Str::from_string(out)
}
pub fn hex2bin(s: &Str) -> Option<Str> {
    let b = s.as_bytes();
    if b.len() % 2 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(b.len() / 2);
    for ch in b.chunks(2) {
        let h = std::str::from_utf8(ch).ok()?;
        out.push(u8::from_str_radix(h, 16).ok()?);
    }
    Some(Str::from_vec(out))
}
pub fn dechex(n: i64) -> Str {
    Str::from_string(format!("{:x}", n as u64))
}
pub fn decbin(n: i64) -> Str {
    Str::from_string(format!("{:b}", n as u64))
}
pub fn decoct(n: i64) -> Str {
    Str::from_string(format!("{:o}", n as u64))
}
fn parse_radix(s: &[u8], radix: u32) -> f64 {
    let mut v: f64 = 0.0;
    for &c in s {
        let d = (c as char).to_digit(radix);
        if let Some(d) = d {
            v = v * radix as f64 + d as f64;
        }
    }
    v
}
pub fn hexdec(s: &Str) -> i64 {
    parse_radix(s, 16) as i64
}
pub fn octdec(s: &Str) -> i64 {
    parse_radix(s, 8) as i64
}
pub fn bindec(s: &Str) -> i64 {
    parse_radix(s, 2) as i64
}
pub fn base_convert(s: &Str, from: i64, to: i64) -> Str {
    let v = parse_radix(&s.as_bytes().to_ascii_lowercase(), from as u32) as u64;
    let to = to as u64;
    if v == 0 {
        return Str::from_static("0");
    }
    let mut digits = Vec::new();
    let mut n = v;
    while n > 0 {
        let d = (n % to) as u8;
        digits.push(if d < 10 { b'0' + d } else { b'a' + d - 10 });
        n /= to;
    }
    digits.reverse();
    Str::from_vec(digits)
}
pub fn number_format(n: f64, decimals: i64, dec_point: &Str, thousands: &Str) -> Str {
    let decimals = decimals.max(0) as usize;
    let rounded = crate::builtins::math::round(n, decimals as i64);
    let s = format!("{:.*}", decimals, rounded.abs());
    let (int_part, frac) = match s.split_once('.') {
        Some((a, b)) => (a.to_string(), Some(b.to_string())),
        None => (s, None),
    };
    let mut out = Vec::new();
    if rounded < 0.0 && rounded.abs() != 0.0 {
        out.push(b'-');
    }
    let ib = int_part.as_bytes();
    for (i, c) in ib.iter().enumerate() {
        if i > 0 && (ib.len() - i) % 3 == 0 {
            out.extend_from_slice(thousands);
        }
        out.push(*c);
    }
    if let Some(f) = frac {
        if decimals > 0 {
            out.extend_from_slice(dec_point);
            out.extend_from_slice(f.as_bytes());
        }
    }
    Str::from_vec(out)
}
pub fn strval<T: ToStr>(v: T) -> Str {
    v.to_php_str()
}
pub fn nl_count(s: &Str) -> i64 {
    s.as_bytes().iter().filter(|&&c| c == b'\n').count() as i64
}
pub fn str_word_count(s: &Str) -> i64 {
    s.as_bytes().split(|c| !(c.is_ascii_alphabetic() || *c == b'\'' || *c == b'-')).filter(|w| !w.is_empty()).count()
        as i64
}
pub fn ctype_digit(s: &Str) -> bool {
    !s.is_empty() && s.as_bytes().iter().all(|c| c.is_ascii_digit())
}
pub fn ctype_alpha(s: &Str) -> bool {
    !s.is_empty() && s.as_bytes().iter().all(|c| c.is_ascii_alphabetic())
}
pub fn ctype_alnum(s: &Str) -> bool {
    !s.is_empty() && s.as_bytes().iter().all(|c| c.is_ascii_alphanumeric())
}
pub fn ctype_upper(s: &Str) -> bool {
    !s.is_empty() && s.as_bytes().iter().all(|c| c.is_ascii_uppercase())
}
pub fn ctype_lower(s: &Str) -> bool {
    !s.is_empty() && s.as_bytes().iter().all(|c| c.is_ascii_lowercase())
}
pub fn ctype_space(s: &Str) -> bool {
    !s.is_empty() && s.as_bytes().iter().all(|c| conv::is_ws(*c))
}
pub fn ctype_punct(s: &Str) -> bool {
    !s.is_empty() && s.as_bytes().iter().all(|c| c.is_ascii_punctuation())
}
pub fn ctype_xdigit(s: &Str) -> bool {
    !s.is_empty() && s.as_bytes().iter().all(|c| c.is_ascii_hexdigit())
}
pub fn is_numeric(s: &Str) -> bool {
    conv::parse_numeric(s.as_bytes()).is_some()
}
pub fn uniqid() -> Str {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    Str::from_string(format!("{:08x}{:05x}", t.as_secs(), t.subsec_micros()))
}
pub fn strtok(s: &Str, token: &Str) -> Option<Str> {
    let b = s.as_bytes();
    let start = b.iter().position(|c| !token.as_bytes().contains(c))?;
    let end = b[start..].iter().position(|c| token.as_bytes().contains(c)).map(|p| p + start).unwrap_or(b.len());
    Some(s.slice(start, end))
}
pub fn levenshtein(a: &Str, b: &Str) -> i64 {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0; b.len() + 1];
    for i in 1..=a.len() {
        cur[0] = i;
        for j in 1..=b.len() {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            cur[j] = (prev[j] + 1).min(cur[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()] as i64
}
pub fn similar_text(a: &Str, b: &Str) -> i64 {
    fn sim(a: &[u8], b: &[u8]) -> usize {
        if a.is_empty() || b.is_empty() {
            return 0;
        }
        let (mut max, mut pa, mut pb) = (0, 0, 0);
        for i in 0..a.len() {
            for j in 0..b.len() {
                let mut k = 0;
                while i + k < a.len() && j + k < b.len() && a[i + k] == b[j + k] {
                    k += 1;
                }
                if k > max {
                    max = k;
                    pa = i;
                    pb = j;
                }
            }
        }
        if max == 0 {
            return 0;
        }
        max + sim(&a[..pa], &b[..pb]) + sim(&a[pa + max..], &b[pb + max..])
    }
    sim(a, b) as i64
}
pub fn mb_strlen(s: &Str) -> i64 {
    s.to_string_lossy().chars().count() as i64
}
pub fn mb_substr(s: &Str, start: i64, length: Option<i64>) -> Str {
    let chars: Vec<char> = s.to_string_lossy().chars().collect();
    match resolve_range(chars.len(), start, length) {
        Some((a, b)) => Str::from_string(chars[a..b].iter().collect()),
        None => Str::empty(),
    }
}
pub fn mb_strpos(hay: &Str, needle: &Str, offset: i64) -> Option<i64> {
    let h: Vec<char> = hay.to_string_lossy().chars().collect();
    let n: Vec<char> = needle.to_string_lossy().chars().collect();
    let from = norm_offset(h.len(), offset)?;
    if n.is_empty() {
        return Some(from as i64);
    }
    (from..h.len()).find(|&i| h[i..].starts_with(&n)).map(|i| i as i64)
}
pub fn mb_strtolower(s: &Str) -> Str {
    Str::from_string(s.to_string_lossy().to_lowercase())
}
pub fn mb_strtoupper(s: &Str) -> Str {
    Str::from_string(s.to_string_lossy().to_uppercase())
}
pub fn mb_strcut(s: &Str, start: i64, length: Option<i64>) -> Str {
    let b = s.as_bytes();
    let (mut a, mut e) = match resolve_range(b.len(), start, length) {
        Some(r) => r,
        None => return Str::empty(),
    };
    while a > 0 && a < b.len() && (b[a] & 0xC0) == 0x80 {
        a -= 1;
    }
    while e > a && e < b.len() && (b[e] & 0xC0) == 0x80 {
        e -= 1;
    }
    s.slice(a, e)
}
pub fn mb_str_split(s: &Str) -> List<Str> {
    s.to_string_lossy().chars().map(|c| Str::from_string(c.to_string())).collect()
}
pub fn md5(s: &Str) -> Str {
    Str::from_string(format!("{:x}", crate::builtins::hash::md5_digest(s.as_bytes())))
}
pub fn sha1(s: &Str) -> Str {
    Str::from_string(crate::builtins::hash::sha1_hex(s.as_bytes()))
}
pub fn crc32(s: &Str) -> i64 {
    crate::builtins::hash::crc32(s.as_bytes()) as i64
}
pub fn hash(algo: &Str, data: &Str) -> Result<Str, RtError> {
    match algo.as_bytes() {
        b"md5" => Ok(md5(data)),
        b"sha1" => Ok(sha1(data)),
        b"crc32b" => Ok(Str::from_string(format!("{:08x}", crate::builtins::hash::crc32(data.as_bytes())))),
        b"sha256" => Ok(Str::from_string(crate::builtins::hash::sha256_hex(data.as_bytes()))),
        b"xxh128" | b"xxh3" | b"xxh64" | b"xxh32" | b"murmur3a" => Ok(Str::from_string(format!("{:016x}", crate::builtins::hash::fnv64(data.as_bytes())))),
        _ => Err(RtError::value_error(crate::sfmt!("hash(): Argument #1 ($algo) must be a valid hashing algorithm, got {}", algo))),
    }
}
pub fn urlencode(s: &Str) -> Str {
    let mut out = Vec::new();
    for &c in s.as_bytes() {
        match c {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' => out.push(c),
            b' ' => out.push(b'+'),
            _ => out.extend_from_slice(format!("%{:02X}", c).as_bytes()),
        }
    }
    Str::from_vec(out)
}
pub fn rawurlencode(s: &Str) -> Str {
    let mut out = Vec::new();
    for &c in s.as_bytes() {
        match c {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(c),
            _ => out.extend_from_slice(format!("%{:02X}", c).as_bytes()),
        }
    }
    Str::from_vec(out)
}
pub fn urldecode(s: &Str) -> Str {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'+' => out.push(b' '),
            b'%' if i + 2 < b.len() + 0 && i + 2 <= b.len() - 1 + 0 => {
                if let Ok(v) = u8::from_str_radix(&String::from_utf8_lossy(&b[i + 1..i + 3]), 16) {
                    out.push(v);
                    i += 2;
                } else {
                    out.push(b'%');
                }
            }
            c => out.push(c),
        }
        i += 1;
    }
    Str::from_vec(out)
}
pub fn escapeshellarg(s: &Str) -> Str {
    let mut out = vec![b'\''];
    for &c in s.as_bytes() {
        if c == b'\'' {
            out.extend_from_slice(b"'\\''");
        } else {
            out.push(c);
        }
    }
    out.push(b'\'');
    Str::from_vec(out)
}
pub fn nl_langinfo_eol() -> Str {
    Str::from_static("\n")
}
pub fn str_word_split(s: &Str) -> List<Str> {
    s.as_bytes().split(|c| conv::is_ws(*c)).filter(|w| !w.is_empty()).map(Str::from_bytes).collect()
}
pub fn version_compare(a: &Str, b: &Str) -> i64 {
    fn canon(v: &[u8]) -> Vec<Vec<u8>> {
        let mut out: Vec<Vec<u8>> = Vec::new();
        let mut cur: Vec<u8> = Vec::new();
        let mut prev_kind = 0u8;
        for &c in v {
            let kind = if c.is_ascii_digit() { 1 } else if c == b'.' || c == b'-' || c == b'_' || c == b'+' { 0 } else { 2 };
            if kind == 0 || (prev_kind != 0 && kind != prev_kind) {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            if kind != 0 {
                cur.push(c);
            }
            prev_kind = kind;
        }
        if !cur.is_empty() {
            out.push(cur);
        }
        out
    }
    fn order(p: &[u8]) -> i64 {
        if p.is_empty() {
            return 0;
        }
        if p[0].is_ascii_digit() {
            return 10;
        }
        let l = p.to_ascii_lowercase();
        if l.starts_with(b"dev") {
            0
        } else if l.starts_with(b"alpha") || l.starts_with(b"a") {
            1
        } else if l.starts_with(b"beta") || l.starts_with(b"b") {
            2
        } else if l.starts_with(b"rc") || l.starts_with(b"c") {
            3
        } else if l == b"#" {
            4
        } else if l.starts_with(b"pl") || l.starts_with(b"p") {
            5
        } else {
            -1
        }
    }
    let x = canon(a);
    let y = canon(b);
    let n = x.len().max(y.len());
    for i in 0..n {
        let px: &[u8] = x.get(i).map(|v| v.as_slice()).unwrap_or(b"");
        let py: &[u8] = y.get(i).map(|v| v.as_slice()).unwrap_or(b"");
        if px.is_empty() && py.is_empty() {
            continue;
        }
        let r = if px.is_empty() {
            if py[0].is_ascii_digit() { -1 } else { -order(py).signum().max(-1) }
        } else if py.is_empty() {
            if px[0].is_ascii_digit() { 1 } else { order(px).signum() }
        } else if px[0].is_ascii_digit() && py[0].is_ascii_digit() {
            let a: u64 = String::from_utf8_lossy(px).parse().unwrap_or(0);
            let b: u64 = String::from_utf8_lossy(py).parse().unwrap_or(0);
            (a as i128 - b as i128).signum() as i64
        } else {
            (order(px) - order(py)).signum()
        };
        if r != 0 {
            return r;
        }
    }
    0
}
pub fn version_compare_op(a: &Str, b: &Str, op: &Str) -> bool {
    let c = version_compare(a, b);
    match op.as_bytes() {
        b"<" | b"lt" => c < 0,
        b"<=" | b"le" => c <= 0,
        b">" | b"gt" => c > 0,
        b">=" | b"ge" => c >= 0,
        b"==" | b"eq" => c == 0,
        b"!=" | b"<>" | b"ne" => c != 0,
        _ => false,
    }
}
/// Deterministic string hash used by the transpiled code where PHP used spl_object_hash-like keys.
pub fn php_dirname(path: &Str, levels: i64) -> Str {
    let mut p = path.as_bytes().to_vec();
    for _ in 0..levels.max(1) {
        while p.len() > 1 && p.last() == Some(&b'/') {
            p.pop();
        }
        match p.iter().rposition(|&c| c == b'/') {
            Some(0) => {
                p.truncate(1);
            }
            Some(i) => {
                p.truncate(i);
                while p.len() > 1 && p.last() == Some(&b'/') {
                    p.pop();
                }
            }
            None => {
                p = b".".to_vec();
            }
        }
    }
    Str::from_vec(p)
}
pub fn php_basename(path: &Str, suffix: Option<&Str>) -> Str {
    let mut b = path.as_bytes();
    while b.len() > 1 && b.last() == Some(&b'/') {
        b = &b[..b.len() - 1];
    }
    let start = b.iter().rposition(|&c| c == b'/').map(|i| i + 1).unwrap_or(0);
    let mut name = &b[start..];
    if let Some(s) = suffix {
        if name.len() > s.len() && name.ends_with(s) {
            name = &name[..name.len() - s.len()];
        }
    }
    Str::from_bytes(name)
}
pub fn pathinfo(path: &Str) -> Map<Str, Str> {
    let mut m = Map::new();
    let dir = php_dirname(path, 1);
    let base = php_basename(path, None);
    if path.as_bytes().contains(&b'/') {
        m.insert(Str::from_static("dirname"), dir);
    } else {
        m.insert(Str::from_static("dirname"), Str::from_static("."));
    }
    m.insert(Str::from_static("basename"), base.clone());
    if let Some(p) = base.as_bytes().iter().rposition(|&c| c == b'.') {
        m.insert(Str::from_static("extension"), base.slice(p + 1, base.len()));
        m.insert(Str::from_static("filename"), base.slice(0, p));
    } else {
        m.insert(Str::from_static("filename"), base);
    }
    m
}
pub fn key_str(k: &ArrayKey) -> Str {
    k.to_str()
}

// ---------------------------------------------------------------- sprintf

/// A value usable as a `sprintf` argument.
#[derive(Clone, Debug)]
pub enum FmtArg {
    Int(i64),
    Float(f64),
    Str(Str),
}
impl From<i64> for FmtArg {
    fn from(i: i64) -> FmtArg {
        FmtArg::Int(i)
    }
}
impl From<f64> for FmtArg {
    fn from(f: f64) -> FmtArg {
        FmtArg::Float(f)
    }
}
impl From<Str> for FmtArg {
    fn from(s: Str) -> FmtArg {
        FmtArg::Str(s)
    }
}
impl From<&Str> for FmtArg {
    fn from(s: &Str) -> FmtArg {
        FmtArg::Str(s.clone())
    }
}
impl From<bool> for FmtArg {
    fn from(b: bool) -> FmtArg {
        FmtArg::Int(b as i64)
    }
}
impl From<&str> for FmtArg {
    fn from(s: &str) -> FmtArg {
        FmtArg::Str(Str::from_str(s))
    }
}
impl From<ArrayKey> for FmtArg {
    fn from(k: ArrayKey) -> FmtArg {
        match k {
            ArrayKey::Int(i) => FmtArg::Int(i),
            ArrayKey::Str(s) => FmtArg::Str(s),
        }
    }
}
impl<T: Into<FmtArg>> From<Option<T>> for FmtArg {
    fn from(o: Option<T>) -> FmtArg {
        match o {
            None => FmtArg::Str(Str::empty()),
            Some(v) => v.into(),
        }
    }
}
impl From<crate::mixed::Mixed> for FmtArg {
    fn from(m: crate::mixed::Mixed) -> FmtArg {
        use crate::mixed::Mixed;
        match m {
            Mixed::Int(i) => FmtArg::Int(i),
            Mixed::Float(f) => FmtArg::Float(f),
            Mixed::Bool(b) => FmtArg::Int(b as i64),
            other => FmtArg::Str(other.to_php_str()),
        }
    }
}
impl FmtArg {
    fn as_int(&self) -> i64 {
        match self {
            FmtArg::Int(i) => *i,
            FmtArg::Float(f) => conv::float_to_int(*f),
            FmtArg::Str(s) => conv::str_to_int(s),
        }
    }
    fn as_float(&self) -> f64 {
        match self {
            FmtArg::Int(i) => *i as f64,
            FmtArg::Float(f) => *f,
            FmtArg::Str(s) => conv::str_to_float(s),
        }
    }
    fn as_str(&self) -> Str {
        match self {
            FmtArg::Int(i) => i.to_php_str(),
            FmtArg::Float(f) => f.to_php_str(),
            FmtArg::Str(s) => s.clone(),
        }
    }
}

pub fn sprintf(format: &Str, args: &[FmtArg]) -> Result<Str, RtError> {
    let f = format.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(f.len() + 16);
    let mut i = 0;
    let mut argi = 0usize;
    while i < f.len() {
        let c = f[i];
        if c != b'%' {
            out.push(c);
            i += 1;
            continue;
        }
        i += 1;
        if i >= f.len() {
            return Err(RtError::value_error("Missing format specifier at end of string"));
        }
        if f[i] == b'%' {
            out.push(b'%');
            i += 1;
            continue;
        }
        // argnum$
        let mut argnum: Option<usize> = None;
        {
            let mut j = i;
            let mut n = 0usize;
            while j < f.len() && f[j].is_ascii_digit() {
                n = n * 10 + (f[j] - b'0') as usize;
                j += 1;
            }
            if j > i && j < f.len() && f[j] == b'$' {
                if n == 0 {
                    return Err(RtError::value_error("Argument number specifier must be greater than zero and less than 2147483647"));
                }
                argnum = Some(n - 1);
                i = j + 1;
            }
        }
        // flags
        let mut left = false;
        let mut plus = false;
        let mut pad = b' ';
        loop {
            if i >= f.len() {
                break;
            }
            match f[i] {
                b'-' => {
                    left = true;
                    i += 1;
                }
                b'+' => {
                    plus = true;
                    i += 1;
                }
                b'0' => {
                    pad = b'0';
                    i += 1;
                }
                b' ' => {
                    pad = b' ';
                    i += 1;
                }
                b'\'' if i + 1 < f.len() => {
                    pad = f[i + 1];
                    i += 2;
                }
                _ => break,
            }
        }
        let mut width = 0usize;
        while i < f.len() && f[i].is_ascii_digit() {
            width = width * 10 + (f[i] - b'0') as usize;
            i += 1;
        }
        let mut precision: Option<usize> = None;
        if i < f.len() && f[i] == b'.' {
            i += 1;
            let mut p = 0usize;
            while i < f.len() && f[i].is_ascii_digit() {
                p = p * 10 + (f[i] - b'0') as usize;
                i += 1;
            }
            precision = Some(p);
        }
        // length modifier (ignored)
        while i < f.len() && matches!(f[i], b'l' | b'h') {
            i += 1;
        }
        if i >= f.len() {
            return Err(RtError::value_error("Missing format specifier at end of string"));
        }
        let spec = f[i];
        i += 1;
        let idx = match argnum {
            Some(n) => n,
            None => {
                let n = argi;
                argi += 1;
                n
            }
        };
        let arg = match args.get(idx) {
            Some(a) => a,
            None => {
                return Err(RtError::new(
                    "ArgumentCountError",
                    crate::sfmt!("{} arguments are required, {} given", idx + 2, args.len() + 1),
                ))
            }
        };
        let body: Vec<u8> = match spec {
            b'd' | b'i' => {
                let v = arg.as_int();
                let mut s = v.abs().to_string();
                if v < 0 {
                    s.insert(0, '-');
                } else if plus {
                    s.insert(0, '+');
                }
                s.into_bytes()
            }
            b'u' => (arg.as_int() as u64).to_string().into_bytes(),
            b's' => {
                let s = arg.as_str();
                match precision {
                    Some(p) if p < s.len() => s.as_bytes()[..p].to_vec(),
                    _ => s.to_vec(),
                }
            }
            b'f' | b'F' => {
                let v = arg.as_float();
                let p = precision.unwrap_or(6);
                let mut s = format!("{:.*}", p, crate::builtins::math::round(v, p as i64).abs());
                if v.is_infinite() {
                    s = if v < 0.0 { "-inf".into() } else { "inf".into() };
                } else if v.is_nan() {
                    s = "nan".into();
                }
                if v < 0.0 && !v.is_nan() && v.is_finite() {
                    s.insert(0, '-');
                } else if plus {
                    s.insert(0, '+');
                }
                s.into_bytes()
            }
            b'e' | b'E' => {
                let v = arg.as_float();
                let p = precision.unwrap_or(6);
                let s = format!("{:.*e}", p, v);
                // PHP prints exponent with at least one digit and sign: 1.0e+3
                let (m, e) = s.split_once('e').unwrap();
                let e: i32 = e.parse().unwrap();
                let mut r = format!("{}{}{}{}", m, if spec == b'e' { 'e' } else { 'E' }, if e < 0 { '-' } else { '+' }, e.abs());
                if plus && v >= 0.0 {
                    r.insert(0, '+');
                }
                r.into_bytes()
            }
            b'g' | b'G' => {
                let v = arg.as_float();
                let p = precision.unwrap_or(6).max(1);
                conv::gcvt(v, Some(p), if spec == b'g' { 'e' } else { 'E' }).into_bytes()
            }
            b'x' => format!("{:x}", arg.as_int() as u64).into_bytes(),
            b'X' => format!("{:X}", arg.as_int() as u64).into_bytes(),
            b'o' => format!("{:o}", arg.as_int() as u64).into_bytes(),
            b'b' => format!("{:b}", arg.as_int() as u64).into_bytes(),
            b'c' => vec![(arg.as_int() & 0xff) as u8],
            other => {
                return Err(RtError::value_error(crate::sfmt!("Unknown format specifier \"{}\"", other as char)));
            }
        };
        if body.len() < width {
            let padn = width - body.len();
            if left {
                out.extend_from_slice(&body);
                for _ in 0..padn {
                    out.push(if pad == b'0' { b' ' } else { pad });
                }
            } else if pad == b'0' && matches!(spec, b'd' | b'i' | b'f' | b'F' | b'e' | b'E' | b'u') && (body.first() == Some(&b'-') || body.first() == Some(&b'+')) {
                out.push(body[0]);
                for _ in 0..padn {
                    out.push(b'0');
                }
                out.extend_from_slice(&body[1..]);
            } else {
                for _ in 0..padn {
                    out.push(pad);
                }
                out.extend_from_slice(&body);
            }
        } else {
            out.extend_from_slice(&body);
        }
    }
    Ok(Str::from_vec(out))
}
pub fn vsprintf<T: Clone + Into<FmtArg>>(format: &Str, args: &List<T>) -> Result<Str, RtError> {
    let v: Vec<FmtArg> = args.iter().map(|a| a.clone().into()).collect();
    sprintf(format, &v)
}
#[macro_export]
macro_rules! sprintf {
    ($fmt:expr $(, $arg:expr)* $(,)?) => { $crate::builtins::string::sprintf(&$fmt, &[$($crate::builtins::string::FmtArg::from($arg)),*]) };
}
