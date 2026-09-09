//! Minimal XML parser backing the `SimpleXMLElement` / `DOMDocument` runtime stubs.
//!
//! `xml_parse` returns the document element as a PHP array tree:
//! `['name' => string, 'attrs' => array<string, string>, 'children' => list<node>, 'text' => string]`
//! where text children appear in `children` as `['name' => '#text', 'text' => ...]` nodes.

use crate::key::ArrayKey;
use crate::map::Map;
use crate::mixed::Mixed;
use crate::string::Str;

struct P<'a> {
    s: &'a [u8],
    i: usize,
}

pub fn xml_parse(src: &Str) -> Option<Mixed> {
    let mut s = src.as_bytes();
    if s.starts_with(&[0xEF, 0xBB, 0xBF]) {
        s = &s[3..];
    }
    let mut p = P { s, i: 0 };
    p.skip_misc();
    let root = p.element()?;
    p.skip_misc();
    Some(root)
}

fn node(name: &[u8], attrs: Map<ArrayKey, Mixed>, children: Map<ArrayKey, Mixed>, text: Vec<u8>) -> Mixed {
    let mut m: Map<ArrayKey, Mixed> = Map::new();
    m.insert(ArrayKey::from_str_val(Str::from_static("name")), Mixed::Str(Str::from_bytes(name)));
    m.insert(ArrayKey::from_str_val(Str::from_static("attrs")), Mixed::Arr(attrs));
    m.insert(ArrayKey::from_str_val(Str::from_static("children")), Mixed::Arr(children));
    m.insert(ArrayKey::from_str_val(Str::from_static("text")), Mixed::Str(Str::from_vec(text)));
    Mixed::Arr(m)
}

fn is_name_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c == b'-' || c == b'.' || c == b':' || c >= 0x80
}

impl<'a> P<'a> {
    fn peek(&self, k: usize) -> u8 {
        self.s.get(self.i + k).copied().unwrap_or(0)
    }
    fn starts(&self, pat: &[u8]) -> bool {
        self.s[self.i..].starts_with(pat)
    }
    fn skip_ws(&mut self) {
        while self.i < self.s.len() && self.s[self.i].is_ascii_whitespace() {
            self.i += 1;
        }
    }
    fn skip_to(&mut self, end: &[u8]) -> bool {
        match crate::string::find_bytes(&self.s[self.i..], end, 0) {
            Some(p) => {
                self.i += p + end.len();
                true
            }
            None => {
                self.i = self.s.len();
                false
            }
        }
    }
    /// whitespace, processing instructions, comments, doctype
    fn skip_misc(&mut self) {
        loop {
            self.skip_ws();
            if self.starts(b"<?") {
                self.skip_to(b"?>");
            } else if self.starts(b"<!--") {
                self.skip_to(b"-->");
            } else if self.starts(b"<!DOCTYPE") || self.starts(b"<!doctype") {
                // optional internal subset [...]
                let mut depth = 0i32;
                while self.i < self.s.len() {
                    match self.s[self.i] {
                        b'[' => depth += 1,
                        b']' => depth -= 1,
                        b'>' if depth <= 0 => {
                            self.i += 1;
                            break;
                        }
                        _ => {}
                    }
                    self.i += 1;
                }
            } else {
                return;
            }
        }
    }
    fn name(&mut self) -> Option<&'a [u8]> {
        let start = self.i;
        while self.i < self.s.len() && is_name_char(self.s[self.i]) {
            self.i += 1;
        }
        if self.i == start { None } else { Some(&self.s[start..self.i]) }
    }
    fn element(&mut self) -> Option<Mixed> {
        if self.peek(0) != b'<' {
            return None;
        }
        self.i += 1;
        let name = self.name()?;
        let mut attrs: Map<ArrayKey, Mixed> = Map::new();
        loop {
            self.skip_ws();
            match self.peek(0) {
                b'/' => {
                    if self.peek(1) != b'>' {
                        return None;
                    }
                    self.i += 2;
                    return Some(node(name, attrs, Map::new(), Vec::new()));
                }
                b'>' => {
                    self.i += 1;
                    break;
                }
                0 => return None,
                _ => {
                    let an = self.name()?;
                    self.skip_ws();
                    if self.peek(0) != b'=' {
                        return None;
                    }
                    self.i += 1;
                    self.skip_ws();
                    let q = self.peek(0);
                    if q != b'"' && q != b'\'' {
                        return None;
                    }
                    self.i += 1;
                    let start = self.i;
                    while self.i < self.s.len() && self.s[self.i] != q {
                        self.i += 1;
                    }
                    if self.i >= self.s.len() {
                        return None;
                    }
                    let raw = &self.s[start..self.i];
                    self.i += 1;
                    attrs.insert(ArrayKey::from_str_val(Str::from_bytes(an)), Mixed::Str(Str::from_vec(decode_entities(raw))));
                }
            }
        }
        // content
        let mut children: Map<ArrayKey, Mixed> = Map::new();
        let mut text: Vec<u8> = Vec::new();
        let mut pending: Vec<u8> = Vec::new();
        let flush = |pending: &mut Vec<u8>, children: &mut Map<ArrayKey, Mixed>, text: &mut Vec<u8>| {
            if !pending.is_empty() {
                text.extend_from_slice(pending);
                children.push(node(b"#text", Map::new(), Map::new(), std::mem::take(pending)));
            }
        };
        loop {
            if self.i >= self.s.len() {
                return None;
            }
            if self.starts(b"</") {
                flush(&mut pending, &mut children, &mut text);
                self.i += 2;
                let end = self.name()?;
                if end != name {
                    return None;
                }
                self.skip_ws();
                if self.peek(0) != b'>' {
                    return None;
                }
                self.i += 1;
                return Some(node(name, attrs, children, text));
            }
            if self.starts(b"<!--") {
                if !self.skip_to(b"-->") {
                    return None;
                }
                continue;
            }
            if self.starts(b"<![CDATA[") {
                self.i += 9;
                let start = self.i;
                if !self.skip_to(b"]]>") {
                    return None;
                }
                pending.extend_from_slice(&self.s[start..self.i - 3]);
                continue;
            }
            if self.starts(b"<?") {
                if !self.skip_to(b"?>") {
                    return None;
                }
                continue;
            }
            if self.peek(0) == b'<' {
                flush(&mut pending, &mut children, &mut text);
                let child = self.element()?;
                children.push(child);
                continue;
            }
            let start = self.i;
            while self.i < self.s.len() && self.s[self.i] != b'<' {
                self.i += 1;
            }
            pending.extend_from_slice(&decode_entities(&self.s[start..self.i]));
        }
    }
}

pub fn decode_entities(raw: &[u8]) -> Vec<u8> {
    if !raw.contains(&b'&') {
        return raw.to_vec();
    }
    let mut out = Vec::with_capacity(raw.len());
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == b'&' {
            if let Some(end) = raw[i..].iter().position(|&c| c == b';') {
                let ent = &raw[i + 1..i + end];
                let decoded: Option<Vec<u8>> = match ent {
                    b"lt" => Some(b"<".to_vec()),
                    b"gt" => Some(b">".to_vec()),
                    b"amp" => Some(b"&".to_vec()),
                    b"quot" => Some(b"\"".to_vec()),
                    b"apos" => Some(b"'".to_vec()),
                    _ if ent.first() == Some(&b'#') => {
                        let (digits, radix) = if ent.get(1) == Some(&b'x') || ent.get(1) == Some(&b'X') { (&ent[2..], 16) } else { (&ent[1..], 10) };
                        u32::from_str_radix(std::str::from_utf8(digits).unwrap_or(""), radix)
                            .ok()
                            .and_then(char::from_u32)
                            .map(|c| c.to_string().into_bytes())
                    }
                    _ => None,
                };
                if let Some(d) = decoded {
                    out.extend_from_slice(&d);
                    i += end + 1;
                    continue;
                }
            }
        }
        out.push(raw[i]);
        i += 1;
    }
    out
}

/// Escapes text/attribute content for serialization.
pub fn xml_escape(s: &Str, attr: bool) -> Str {
    let mut out = Vec::with_capacity(s.as_bytes().len());
    for &c in s.as_bytes() {
        match c {
            b'<' => out.extend_from_slice(b"&lt;"),
            b'>' => out.extend_from_slice(b"&gt;"),
            b'&' => out.extend_from_slice(b"&amp;"),
            b'"' if attr => out.extend_from_slice(b"&quot;"),
            _ => out.push(c),
        }
    }
    Str::from_vec(out)
}
