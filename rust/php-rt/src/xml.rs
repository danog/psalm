//! Minimal XML parser backing the `SimpleXMLElement` / `DOMDocument` runtime stubs.
//!
//! `xml_parse` returns the document as a flat, typed node list in document order:
//! `(name, text, is_text, attrs, parent index)` — the root at index 0 with parent -1, text nodes named
//! `#text`; an element's `text` is its concatenated direct text content.

use crate::list::List;
use crate::map::Map;
use crate::string::Str;

/// One node of the flat document: name, text, is_text, attributes, parent index (-1 for the root).
pub type XmlFlatNode = (Str, Str, bool, Map<Str, Str>, i64);

struct P<'a> {
    s: &'a [u8],
    i: usize,
}

pub fn xml_parse(src: &Str) -> Option<List<XmlFlatNode>> {
    let mut s = src.as_bytes();
    if s.starts_with(&[0xEF, 0xBB, 0xBF]) {
        s = &s[3..];
    }
    let mut p = P { s, i: 0 };
    p.skip_misc();
    let mut out: Vec<XmlFlatNode> = Vec::new();
    p.element(&mut out, -1)?;
    p.skip_misc();
    Some(List::from_vec(out))
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
    fn element(&mut self, out: &mut Vec<XmlFlatNode>, parent: i64) -> Option<()> {
        if self.peek(0) != b'<' {
            return None;
        }
        self.i += 1;
        let name = self.name()?.to_vec();
        let idx = out.len() as i64;
        out.push((Str::from_bytes(&name), Str::empty(), false, Map::new(), parent));
        let mut attrs: Map<Str, Str> = Map::new();
        loop {
            self.skip_ws();
            match self.peek(0) {
                b'/' => {
                    if self.peek(1) != b'>' {
                        return None;
                    }
                    self.i += 2;
                    out[idx as usize].3 = attrs;
                    return Some(());
                }
                b'>' => {
                    self.i += 1;
                    break;
                }
                0 => return None,
                _ => {
                    let an = self.name()?.to_vec();
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
                    attrs.insert(Str::from_bytes(&an), Str::from_vec(decode_entities(raw)));
                }
            }
        }
        out[idx as usize].3 = attrs;
        // content
        let mut text: Vec<u8> = Vec::new();
        let mut pending: Vec<u8> = Vec::new();
        let flush = |pending: &mut Vec<u8>, out: &mut Vec<XmlFlatNode>, text: &mut Vec<u8>| {
            if !pending.is_empty() {
                text.extend_from_slice(pending);
                out.push((Str::from_static("#text"), Str::from_vec(std::mem::take(pending)), true, Map::new(), idx));
            }
        };
        loop {
            if self.i >= self.s.len() {
                return None;
            }
            if self.starts(b"</") {
                flush(&mut pending, out, &mut text);
                self.i += 2;
                let end = self.name()?;
                if end != &name[..] {
                    return None;
                }
                self.skip_ws();
                if self.peek(0) != b'>' {
                    return None;
                }
                self.i += 1;
                out[idx as usize].1 = Str::from_vec(text);
                return Some(());
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
                flush(&mut pending, out, &mut text);
                self.element(out, idx)?;
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
