//! `eval()` for the constant expressions test fixtures use (`return "\t";`, `PHP_INT_MAX + 1`,
//! `implode(range("\0", "\37"))`, `chr(27)`): literals, a few constants and functions, `+ - .`
//! and unary minus. Anything else is an Error.

use crate::consts::*;
use crate::error::RtError;
use crate::key::ArrayKey;
use crate::map::Map;
use crate::mixed::Mixed;
use crate::string::Str;
use crate::support::{num_add, num_sub, to_num};
use crate::Num;

struct P {
    toks: Vec<(i64, Vec<u8>)>,
    i: usize,
}

fn err(msg: &str) -> RtError {
    RtError::error(crate::sfmt!("eval(): unsupported code: {}", msg))
}

pub fn php_eval(code: &Str) -> Result<Mixed, RtError> {
    let mut src = Vec::with_capacity(code.as_bytes().len() + 6);
    src.extend_from_slice(b"<?php ");
    src.extend_from_slice(code.as_bytes());
    let toks: Vec<(i64, Vec<u8>)> = crate::tokenizer::tokenize(&Str::from_vec(src))
        .into_iter()
        .filter(|(id, _, _, _)| *id != T_OPEN_TAG && *id != T_WHITESPACE && *id != T_COMMENT && *id != T_DOC_COMMENT)
        .map(|(id, text, _, _)| (id, text.as_bytes().to_vec()))
        .collect();
    let mut p = P { toks, i: 0 };
    if p.peek_id() == T_RETURN {
        p.i += 1;
    }
    let v = p.expr()?;
    if p.peek_text() == b";" {
        p.i += 1;
    }
    if p.i != p.toks.len() {
        return Err(err("trailing tokens"));
    }
    Ok(v)
}

impl P {
    fn peek_id(&self) -> i64 {
        self.toks.get(self.i).map_or(0, |t| t.0)
    }
    fn peek_text(&self) -> &[u8] {
        self.toks.get(self.i).map_or(b"", |t| t.1.as_slice())
    }
    fn eat(&mut self, text: &[u8]) -> Result<(), RtError> {
        if self.peek_text() == text {
            self.i += 1;
            Ok(())
        } else {
            Err(err(&format!("expected {}", String::from_utf8_lossy(text))))
        }
    }
    /// additive level: `a + b`, `a - b`, `a . b`
    fn expr(&mut self) -> Result<Mixed, RtError> {
        let mut v = self.unary()?;
        loop {
            match self.peek_text() {
                b"+" => {
                    self.i += 1;
                    let r = self.unary()?;
                    v = num_add(to_num(&v), to_num(&r)).to_mixed();
                }
                b"-" => {
                    self.i += 1;
                    let r = self.unary()?;
                    v = num_sub(to_num(&v), to_num(&r)).to_mixed();
                }
                b"." => {
                    self.i += 1;
                    let r = self.unary()?;
                    let mut s = crate::traits::to_str(&v);
                    s.push_bytes(crate::traits::to_str(&r).as_bytes());
                    v = Mixed::Str(s);
                }
                _ => return Ok(v),
            }
        }
    }
    fn unary(&mut self) -> Result<Mixed, RtError> {
        if self.peek_text() == b"-" {
            self.i += 1;
            let v = self.unary()?;
            return Ok(match to_num(&v) {
                Num::Int(i) => match i.checked_neg() {
                    Some(n) => Mixed::Int(n),
                    None => Mixed::Float(-(i as f64)),
                },
                Num::Float(f) => Mixed::Float(-f),
            });
        }
        if self.peek_text() == b"+" {
            self.i += 1;
            return self.unary();
        }
        self.primary()
    }
    fn primary(&mut self) -> Result<Mixed, RtError> {
        let (id, text) = match self.toks.get(self.i) {
            Some(t) => (t.0, t.1.clone()),
            None => return Err(err("unexpected end")),
        };
        self.i += 1;
        if id == T_LNUMBER {
            let s = String::from_utf8_lossy(&text).replace('_', "");
            return Ok(match s.parse::<i64>() {
                Ok(i) => Mixed::Int(i),
                Err(_) => Mixed::Float(s.parse::<f64>().unwrap_or(0.0)),
            });
        }
        if id == T_DNUMBER {
            let s = String::from_utf8_lossy(&text).replace('_', "");
            return Ok(Mixed::Float(s.parse::<f64>().unwrap_or(0.0)));
        }
        if id == T_CONSTANT_ENCAPSED_STRING {
            return Ok(Mixed::Str(Str::from_vec(unquote(&text))));
        }
        if text == b"(" {
            let v = self.expr()?;
            self.eat(b")")?;
            return Ok(v);
        }
        if text == b"[" {
            return self.array(b"]");
        }
        if id == T_ARRAY && self.peek_text() == b"(" {
            self.i += 1;
            return self.array(b")");
        }
        if id == T_STRING || id == T_NAME_FULLY_QUALIFIED || id == T_NAME_QUALIFIED {
            let name = text.strip_prefix(b"\\").unwrap_or(&text).to_vec();
            if self.peek_text() == b"(" {
                self.i += 1;
                let mut args = Vec::new();
                while self.peek_text() != b")" {
                    args.push(self.expr()?);
                    if self.peek_text() == b"," {
                        self.i += 1;
                    }
                }
                self.i += 1;
                return call(&name, args);
            }
            return constant(&name);
        }
        Err(err(&String::from_utf8_lossy(&text)))
    }
    fn array(&mut self, close: &[u8]) -> Result<Mixed, RtError> {
        let mut m: Map<ArrayKey, Mixed> = Map::new();
        while self.peek_text() != close {
            let v = self.expr()?;
            if self.peek_id() == T_DOUBLE_ARROW {
                self.i += 1;
                let val = self.expr()?;
                m.insert(crate::traits::to_key(&v), val);
            } else {
                m.push(v);
            }
            if self.peek_text() == b"," {
                self.i += 1;
            }
        }
        self.i += 1;
        Ok(Mixed::Arr(m))
    }
}

fn constant(name: &[u8]) -> Result<Mixed, RtError> {
    let lower = name.to_ascii_lowercase();
    Ok(match lower.as_slice() {
        b"true" => Mixed::Bool(true),
        b"false" => Mixed::Bool(false),
        b"null" => Mixed::Null,
        _ => match name {
            b"PHP_INT_MAX" => Mixed::Int(i64::MAX),
            b"PHP_INT_MIN" => Mixed::Int(i64::MIN),
            b"PHP_INT_SIZE" => Mixed::Int(8),
            b"PHP_EOL" => Mixed::Str(Str::from_static("\n")),
            b"PHP_VERSION_ID" => Mixed::Int(PHP_VERSION_ID),
            _ => {
                let n = Str::from_bytes(name);
                match crate::registry::constant_value(&n) {
                    Some(v) => v,
                    None => match crate::consts::token_value(name) {
                        Some(v) => Mixed::Int(v),
                        None => return Err(err(&format!("constant {}", String::from_utf8_lossy(name)))),
                    },
                }
            }
        },
    })
}

fn call(name: &[u8], args: Vec<Mixed>) -> Result<Mixed, RtError> {
    let lower = name.to_ascii_lowercase();
    match lower.as_slice() {
        b"chr" => Ok(Mixed::Str(crate::builtins::string::chr(to_num(&args[0]).to_i64()))),
        b"ord" => Ok(Mixed::Int(crate::traits::to_str(&args[0]).as_bytes().first().map_or(0, |b| *b as i64))),
        b"implode" => {
            let (sep, arr) = if args.len() >= 2 { (crate::traits::to_str(&args[0]), args[1].clone()) } else { (Str::empty(), args[0].clone()) };
            let mut out = Str::empty();
            if let Mixed::Arr(a) = arr {
                for (i, (_, v)) in a.iter().enumerate() {
                    if i > 0 {
                        out.push_bytes(sep.as_bytes());
                    }
                    out.push_bytes(crate::traits::to_str(v).as_bytes());
                }
            }
            Ok(Mixed::Str(out))
        }
        b"range" => {
            let mut m: Map<ArrayKey, Mixed> = Map::new();
            match (&args[0], &args[1]) {
                (Mixed::Str(a), Mixed::Str(b)) if a.as_bytes().len() == 1 && b.as_bytes().len() == 1 => {
                    let (lo, hi) = (a.as_bytes()[0], b.as_bytes()[0]);
                    if lo <= hi {
                        for c in lo..=hi {
                            m.push(Mixed::Str(Str::from_bytes(&[c])));
                        }
                    } else {
                        for c in (hi..=lo).rev() {
                            m.push(Mixed::Str(Str::from_bytes(&[c])));
                        }
                    }
                }
                (a, b) => {
                    let (lo, hi) = (to_num(a).to_i64(), to_num(b).to_i64());
                    if lo <= hi {
                        for i in lo..=hi {
                            m.push(Mixed::Int(i));
                        }
                    } else {
                        for i in (hi..=lo).rev() {
                            m.push(Mixed::Int(i));
                        }
                    }
                }
            }
            Ok(Mixed::Arr(m))
        }
        b"str_repeat" => {
            let s = crate::traits::to_str(&args[0]);
            let n = to_num(&args[1]).to_i64().max(0) as usize;
            Ok(Mixed::Str(Str::from_vec(s.as_bytes().repeat(n))))
        }
        _ => Err(err(&format!("function {}", String::from_utf8_lossy(name)))),
    }
}

/// Value of a `'...'` / `"..."` literal.
fn unquote(text: &[u8]) -> Vec<u8> {
    let (mut t, double) = match text.first() {
        Some(b'b') | Some(b'B') => (&text[1..], text.get(1) == Some(&b'"')),
        _ => (text, text.first() == Some(&b'"')),
    };
    if t.len() >= 2 {
        t = &t[1..t.len() - 1];
    }
    let mut out = Vec::with_capacity(t.len());
    let mut i = 0;
    while i < t.len() {
        let c = t[i];
        if c != b'\\' || i + 1 >= t.len() {
            out.push(c);
            i += 1;
            continue;
        }
        let n = t[i + 1];
        if !double {
            if n == b'\'' || n == b'\\' {
                out.push(n);
                i += 2;
            } else {
                out.push(c);
                i += 1;
            }
            continue;
        }
        i += 2;
        match n {
            b'n' => out.push(b'\n'),
            b't' => out.push(b'\t'),
            b'r' => out.push(b'\r'),
            b'v' => out.push(0x0b),
            b'f' => out.push(0x0c),
            b'e' => out.push(0x1b),
            b'\\' => out.push(b'\\'),
            b'$' => out.push(b'$'),
            b'"' => out.push(b'"'),
            b'0'..=b'7' => {
                let mut v = (n - b'0') as u32;
                let mut k = 0;
                while k < 2 && i < t.len() && (b'0'..=b'7').contains(&t[i]) {
                    v = v * 8 + (t[i] - b'0') as u32;
                    i += 1;
                    k += 1;
                }
                out.push((v & 0xff) as u8);
            }
            b'x' => {
                let mut v = 0u32;
                let mut k = 0;
                while k < 2 && i < t.len() && t[i].is_ascii_hexdigit() {
                    v = v * 16 + (t[i] as char).to_digit(16).unwrap();
                    i += 1;
                    k += 1;
                }
                if k == 0 {
                    out.extend_from_slice(b"\\x");
                } else {
                    out.push(v as u8);
                }
            }
            b'u' if t.get(i) == Some(&b'{') => {
                let mut j = i + 1;
                let mut v = 0u32;
                while j < t.len() && t[j] != b'}' {
                    v = v * 16 + (t[j] as char).to_digit(16).unwrap_or(0);
                    j += 1;
                }
                i = j + 1;
                if let Some(ch) = char::from_u32(v) {
                    let mut buf = [0u8; 4];
                    out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                }
            }
            other => {
                out.push(b'\\');
                out.push(other);
            }
        }
    }
    out
}
