//! PHP array keys (int|string) with numeric-string normalization.

use crate::string::Str;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArrayKey {
    Int(i64),
    Str(Str),
}

impl Hash for ArrayKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ArrayKey::Int(i) => {
                0u8.hash(state);
                i.hash(state)
            }
            ArrayKey::Str(s) => {
                1u8.hash(state);
                s.hash(state)
            }
        }
    }
}

/// PHP's rule: a string is an integer key if it is a canonical decimal integer in i64 range.
pub fn parse_int_key(b: &[u8]) -> Option<i64> {
    if b.is_empty() || b.len() > 20 {
        return None;
    }
    let (neg, digits) = if b[0] == b'-' { (true, &b[1..]) } else { (false, b) };
    if digits.is_empty() || !digits.iter().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if digits[0] == b'0' && (digits.len() > 1 || neg) {
        return None;
    }
    let mut v: i64 = 0;
    for &c in digits {
        v = v.checked_mul(10)?;
        let d = (c - b'0') as i64;
        v = if neg { v.checked_sub(d)? } else { v.checked_add(d)? };
    }
    Some(v)
}

impl ArrayKey {
    pub fn from_bytes(b: &[u8]) -> ArrayKey {
        match parse_int_key(b) {
            Some(i) => ArrayKey::Int(i),
            None => ArrayKey::Str(Str::from_bytes(b)),
        }
    }
    pub fn from_str_val(s: Str) -> ArrayKey {
        match parse_int_key(s.as_bytes()) {
            Some(i) => ArrayKey::Int(i),
            None => ArrayKey::Str(s),
        }
    }
    pub const fn from_static(s: &'static str) -> ArrayKey {
        ArrayKey::Str(Str::from_static(s))
    }
    pub fn to_str(&self) -> Str {
        match self {
            ArrayKey::Int(i) => Str::from_string(i.to_string()),
            ArrayKey::Str(s) => s.clone(),
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            ArrayKey::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&Str> {
        match self {
            ArrayKey::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn is_int(&self) -> bool {
        matches!(self, ArrayKey::Int(_))
    }
    pub fn is_str(&self) -> bool {
        matches!(self, ArrayKey::Str(_))
    }
}

impl fmt::Debug for ArrayKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArrayKey::Int(i) => write!(f, "{}", i),
            ArrayKey::Str(s) => write!(f, "{:?}", s),
        }
    }
}
impl fmt::Display for ArrayKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArrayKey::Int(i) => write!(f, "{}", i),
            ArrayKey::Str(s) => write!(f, "{}", s),
        }
    }
}

impl From<i64> for ArrayKey {
    fn from(i: i64) -> ArrayKey {
        ArrayKey::Int(i)
    }
}
impl From<Str> for ArrayKey {
    fn from(s: Str) -> ArrayKey {
        ArrayKey::from_str_val(s)
    }
}
impl From<&Str> for ArrayKey {
    fn from(s: &Str) -> ArrayKey {
        ArrayKey::from_str_val(s.clone())
    }
}
impl From<&str> for ArrayKey {
    fn from(s: &str) -> ArrayKey {
        ArrayKey::from_bytes(s.as_bytes())
    }
}
impl From<bool> for ArrayKey {
    fn from(b: bool) -> ArrayKey {
        ArrayKey::Int(b as i64)
    }
}
impl Default for ArrayKey {
    fn default() -> ArrayKey {
        ArrayKey::Int(0)
    }
}

/// Types usable as map keys: conversions from PHP scalars.
pub trait MapKey: Clone + Eq + Hash + fmt::Debug {
    fn to_array_key(&self) -> ArrayKey;
    fn from_array_key(k: ArrayKey) -> Self;
    /// Next auto-index after this key (only meaningful for int-like keys).
    fn int_value(&self) -> Option<i64>;
    fn from_index(i: i64) -> Self;
    fn to_str_key(&self) -> Str {
        self.to_array_key().to_str()
    }
}

impl MapKey for i64 {
    fn to_array_key(&self) -> ArrayKey {
        ArrayKey::Int(*self)
    }
    fn from_array_key(k: ArrayKey) -> i64 {
        match k {
            ArrayKey::Int(i) => i,
            ArrayKey::Str(s) => crate::conv::str_to_int(s.as_bytes()),
        }
    }
    fn int_value(&self) -> Option<i64> {
        Some(*self)
    }
    fn from_index(i: i64) -> i64 {
        i
    }
}
impl MapKey for Str {
    fn to_array_key(&self) -> ArrayKey {
        ArrayKey::from_str_val(self.clone())
    }
    fn from_array_key(k: ArrayKey) -> Str {
        k.to_str()
    }
    fn int_value(&self) -> Option<i64> {
        parse_int_key(self.as_bytes())
    }
    fn from_index(i: i64) -> Str {
        Str::from_string(i.to_string())
    }
}
impl MapKey for ArrayKey {
    fn to_array_key(&self) -> ArrayKey {
        self.clone()
    }
    fn from_array_key(k: ArrayKey) -> ArrayKey {
        k
    }
    fn int_value(&self) -> Option<i64> {
        self.as_int()
    }
    fn from_index(i: i64) -> ArrayKey {
        ArrayKey::Int(i)
    }
}
