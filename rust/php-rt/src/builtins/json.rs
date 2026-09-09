//! json_encode / json_decode.

use crate::conv;
use crate::error::RtError;
use crate::key::ArrayKey;
use crate::map::Map;
use crate::mixed::Mixed;
use crate::string::Str;
use std::cell::RefCell;

thread_local! {
    static LAST_ERROR: RefCell<(i64, &'static str)> = RefCell::new((0, "No error"));
}

pub fn json_last_error() -> i64 {
    LAST_ERROR.with(|e| e.borrow().0)
}
pub fn json_last_error_msg() -> Str {
    Str::from_static(LAST_ERROR.with(|e| e.borrow().1))
}
fn set_err(code: i64, msg: &'static str) {
    LAST_ERROR.with(|e| *e.borrow_mut() = (code, msg));
}

const THROW: i64 = 4194304;
const PRETTY: i64 = 128;
const UNESCAPED_SLASHES: i64 = 64;
const UNESCAPED_UNICODE: i64 = 256;
const FORCE_OBJECT: i64 = 16;
const PRESERVE_ZERO_FRACTION: i64 = 1024;
const HEX_TAG: i64 = 1;
const HEX_AMP: i64 = 2;
const HEX_APOS: i64 = 4;
const HEX_QUOT: i64 = 8;

fn encode_str(s: &[u8], flags: i64, out: &mut Vec<u8>) -> Result<(), RtError> {
    let text = match std::str::from_utf8(s) {
        Ok(t) => t,
        Err(_) => {
            if flags & 2097152 != 0 {
                return encode_str(String::from_utf8_lossy(s).as_bytes(), flags, out);
            }
            if flags & 1048576 != 0 {
                let cleaned: Vec<u8> = s.iter().copied().filter(|b| b.is_ascii()).collect();
                return encode_str(&cleaned, flags, out);
            }
            set_err(5, "Malformed UTF-8 characters, possibly incorrectly encoded");
            return Err(RtError::new("JsonException", "Malformed UTF-8 characters, possibly incorrectly encoded"));
        }
    };
    out.push(b'"');
    for ch in text.chars() {
        match ch {
            '"' if flags & HEX_QUOT != 0 => out.extend_from_slice(b"\\u0022"),
            '"' => out.extend_from_slice(b"\\\""),
            '\\' => out.extend_from_slice(b"\\\\"),
            '/' if flags & UNESCAPED_SLASHES == 0 => out.extend_from_slice(b"\\/"),
            '\n' => out.extend_from_slice(b"\\n"),
            '\r' => out.extend_from_slice(b"\\r"),
            '\t' => out.extend_from_slice(b"\\t"),
            '\u{8}' => out.extend_from_slice(b"\\b"),
            '\u{c}' => out.extend_from_slice(b"\\f"),
            '<' if flags & HEX_TAG != 0 => out.extend_from_slice(b"\\u003C"),
            '>' if flags & HEX_TAG != 0 => out.extend_from_slice(b"\\u003E"),
            '&' if flags & HEX_AMP != 0 => out.extend_from_slice(b"\\u0026"),
            '\'' if flags & HEX_APOS != 0 => out.extend_from_slice(b"\\u0027"),
            c if (c as u32) < 0x20 => out.extend_from_slice(format!("\\u{:04x}", c as u32).as_bytes()),
            c if (c as u32) > 0x7f && flags & UNESCAPED_UNICODE == 0 => {
                let mut buf = [0u16; 2];
                for u in c.encode_utf16(&mut buf) {
                    out.extend_from_slice(format!("\\u{:04x}", u).as_bytes());
                }
            }
            c => {
                let mut b = [0u8; 4];
                out.extend_from_slice(c.encode_utf8(&mut b).as_bytes());
            }
        }
    }
    out.push(b'"');
    Ok(())
}

fn indent(out: &mut Vec<u8>, depth: usize, flags: i64) {
    if flags & PRETTY != 0 {
        out.push(b'\n');
        for _ in 0..depth {
            out.extend_from_slice(b"    ");
        }
    }
}

fn encode(m: &Mixed, flags: i64, depth: usize, max_depth: i64, out: &mut Vec<u8>) -> Result<(), RtError> {
    if depth as i64 > max_depth {
        set_err(1, "Maximum stack depth exceeded");
        return Err(RtError::new("JsonException", "Maximum stack depth exceeded"));
    }
    match m {
        Mixed::Null => out.extend_from_slice(b"null"),
        Mixed::Bool(b) => out.extend_from_slice(if *b { b"true" } else { b"false" }),
        Mixed::Int(i) => out.extend_from_slice(i.to_string().as_bytes()),
        Mixed::Float(f) => {
            if !f.is_finite() {
                set_err(7, "Inf and NaN cannot be JSON encoded");
                return Err(RtError::new("JsonException", "Inf and NaN cannot be JSON encoded"));
            }
            let mut s = conv::gcvt(*f, None, 'e');
            if flags & PRESERVE_ZERO_FRACTION != 0 && !s.contains('.') && !s.contains('e') {
                s.push_str(".0");
            }
            out.extend_from_slice(s.as_bytes());
        }
        Mixed::Str(s) => encode_str(s, flags, out)?,
        Mixed::Arr(a) => {
            let as_list = flags & FORCE_OBJECT == 0 && a.is_list();
            if as_list {
                if a.is_empty() {
                    out.extend_from_slice(b"[]");
                    return Ok(());
                }
                out.push(b'[');
                for (i, (_, v)) in a.iter().enumerate() {
                    if i > 0 {
                        out.push(b',');
                    }
                    indent(out, depth + 1, flags);
                    encode(v, flags, depth + 1, max_depth, out)?;
                }
                indent(out, depth, flags);
                out.push(b']');
            } else {
                if a.is_empty() {
                    out.extend_from_slice(b"{}");
                    return Ok(());
                }
                out.push(b'{');
                for (i, (k, v)) in a.iter().enumerate() {
                    if i > 0 {
                        out.push(b',');
                    }
                    indent(out, depth + 1, flags);
                    encode_str(k.to_str().as_bytes(), flags, out)?;
                    out.push(b':');
                    if flags & PRETTY != 0 {
                        out.push(b' ');
                    }
                    encode(v, flags, depth + 1, max_depth, out)?;
                }
                indent(out, depth, flags);
                out.push(b'}');
            }
        }
        Mixed::Obj(o) => {
            // JsonSerializable objects are pre-converted by the transpiler via jsonSerialize();
            // stdClass / plain objects encode their public props
            let props = o.props();
            if props.is_empty() {
                out.extend_from_slice(b"{}");
                return Ok(());
            }
            out.push(b'{');
            for (i, (k, v)) in props.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                indent(out, depth + 1, flags);
                encode_str(k, flags, out)?;
                out.push(b':');
                if flags & PRETTY != 0 {
                    out.push(b' ');
                }
                encode(v, flags, depth + 1, max_depth, out)?;
            }
            indent(out, depth, flags);
            out.push(b'}');
        }
        Mixed::Closure(_) => out.extend_from_slice(b"{}"),
    }
    Ok(())
}

pub fn json_encode(m: &Mixed, flags: i64, depth: i64) -> Result<Option<Str>, RtError> {
    set_err(0, "No error");
    let mut out = Vec::new();
    match encode(m, flags, 0, depth, &mut out) {
        Ok(()) => Ok(Some(Str::from_vec(out))),
        Err(e) => {
            if flags & THROW != 0 {
                Err(e)
            } else {
                Ok(None)
            }
        }
    }
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
    assoc: bool,
    depth: i64,
    max_depth: i64,
}

impl<'a> Parser<'a> {
    fn ws(&mut self) {
        while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }
    fn err<T>(&self) -> Result<T, ()> {
        Err(())
    }
    fn value(&mut self) -> Result<Mixed, ()> {
        self.ws();
        if self.i >= self.s.len() {
            return self.err();
        }
        match self.s[self.i] {
            b'{' => self.object(),
            b'[' => self.array(),
            b'"' => Ok(Mixed::Str(Str::from_vec(self.string()?))),
            b't' => self.lit(b"true", Mixed::Bool(true)),
            b'f' => self.lit(b"false", Mixed::Bool(false)),
            b'n' => self.lit(b"null", Mixed::Null),
            b'-' | b'0'..=b'9' => self.number(),
            _ => self.err(),
        }
    }
    fn lit(&mut self, word: &[u8], v: Mixed) -> Result<Mixed, ()> {
        if self.s[self.i..].starts_with(word) {
            self.i += word.len();
            Ok(v)
        } else {
            self.err()
        }
    }
    fn number(&mut self) -> Result<Mixed, ()> {
        let start = self.i;
        if self.s[self.i] == b'-' {
            self.i += 1;
        }
        let mut is_float = false;
        while self.i < self.s.len() {
            match self.s[self.i] {
                b'0'..=b'9' => self.i += 1,
                b'.' | b'e' | b'E' | b'+' | b'-' => {
                    is_float = true;
                    self.i += 1;
                }
                _ => break,
            }
        }
        let text = std::str::from_utf8(&self.s[start..self.i]).map_err(|_| ())?;
        if !is_float {
            if let Ok(i) = text.parse::<i64>() {
                return Ok(Mixed::Int(i));
            }
        }
        text.parse::<f64>().map(Mixed::Float).map_err(|_| ())
    }
    fn string(&mut self) -> Result<Vec<u8>, ()> {
        self.i += 1; // opening quote
        let mut out = Vec::new();
        while self.i < self.s.len() {
            let c = self.s[self.i];
            match c {
                b'"' => {
                    self.i += 1;
                    return Ok(out);
                }
                b'\\' => {
                    self.i += 1;
                    if self.i >= self.s.len() {
                        return self.err();
                    }
                    let e = self.s[self.i];
                    self.i += 1;
                    match e {
                        b'"' => out.push(b'"'),
                        b'\\' => out.push(b'\\'),
                        b'/' => out.push(b'/'),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'n' => out.push(b'\n'),
                        b'r' => out.push(b'\r'),
                        b't' => out.push(b'\t'),
                        b'u' => {
                            let mut cp = self.hex4()?;
                            if (0xD800..0xDC00).contains(&cp) {
                                if self.s[self.i..].starts_with(b"\\u") {
                                    self.i += 2;
                                    let lo = self.hex4()?;
                                    if (0xDC00..0xE000).contains(&lo) {
                                        cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                                    } else {
                                        return self.err();
                                    }
                                } else {
                                    return self.err();
                                }
                            }
                            let ch = char::from_u32(cp).ok_or(())?;
                            let mut b = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut b).as_bytes());
                        }
                        _ => return self.err(),
                    }
                }
                c if c < 0x20 => return self.err(),
                c => {
                    out.push(c);
                    self.i += 1;
                }
            }
        }
        self.err()
    }
    fn hex4(&mut self) -> Result<u32, ()> {
        if self.i + 4 > self.s.len() {
            return self.err();
        }
        let h = std::str::from_utf8(&self.s[self.i..self.i + 4]).map_err(|_| ())?;
        self.i += 4;
        u32::from_str_radix(h, 16).map_err(|_| ())
    }
    fn array(&mut self) -> Result<Mixed, ()> {
        self.depth += 1;
        if self.depth > self.max_depth {
            return self.err();
        }
        self.i += 1;
        let mut m: Map<ArrayKey, Mixed> = Map::new();
        self.ws();
        if self.i < self.s.len() && self.s[self.i] == b']' {
            self.i += 1;
            self.depth -= 1;
            return Ok(Mixed::Arr(m));
        }
        loop {
            let v = self.value()?;
            m.push(v);
            self.ws();
            if self.i >= self.s.len() {
                return self.err();
            }
            match self.s[self.i] {
                b',' => self.i += 1,
                b']' => {
                    self.i += 1;
                    self.depth -= 1;
                    return Ok(Mixed::Arr(m));
                }
                _ => return self.err(),
            }
        }
    }
    fn object(&mut self) -> Result<Mixed, ()> {
        self.depth += 1;
        if self.depth > self.max_depth {
            return self.err();
        }
        self.i += 1;
        let mut m: Map<ArrayKey, Mixed> = Map::new();
        self.ws();
        if self.i < self.s.len() && self.s[self.i] == b'}' {
            self.i += 1;
            self.depth -= 1;
            return Ok(self.finish_object(m));
        }
        loop {
            self.ws();
            if self.i >= self.s.len() || self.s[self.i] != b'"' {
                return self.err();
            }
            let k = self.string()?;
            self.ws();
            if self.i >= self.s.len() || self.s[self.i] != b':' {
                return self.err();
            }
            self.i += 1;
            let v = self.value()?;
            let key = if self.assoc { ArrayKey::from_bytes(&k) } else { ArrayKey::Str(Str::from_vec(k)) };
            m.insert(key, v);
            self.ws();
            if self.i >= self.s.len() {
                return self.err();
            }
            match self.s[self.i] {
                b',' => self.i += 1,
                b'}' => {
                    self.i += 1;
                    self.depth -= 1;
                    return Ok(self.finish_object(m));
                }
                _ => return self.err(),
            }
        }
    }
    fn finish_object(&self, m: Map<ArrayKey, Mixed>) -> Mixed {
        if self.assoc {
            Mixed::Arr(m)
        } else {
            let sm: Map<Str, Mixed> = m.into_iter().map(|(k, v)| (k.to_str(), v)).collect();
            Mixed::Obj(std::rc::Rc::new(crate::containers::StdClass::from_map(sm)))
        }
    }
}

pub fn json_decode(s: &Str, assoc: bool, depth: i64, flags: i64) -> Result<Mixed, RtError> {
    set_err(0, "No error");
    let assoc = assoc || flags & 1 != 0;
    let mut p = Parser { s, i: 0, assoc, depth: 0, max_depth: depth };
    let result = p.value().and_then(|v| {
        p.ws();
        if p.i == p.s.len() { Ok(v) } else { Err(()) }
    });
    match result {
        Ok(v) => Ok(v),
        Err(()) => {
            set_err(4, "Syntax error");
            if flags & THROW != 0 {
                Err(RtError::new("JsonException", "Syntax error"))
            } else {
                Ok(Mixed::Null)
            }
        }
    }
}
