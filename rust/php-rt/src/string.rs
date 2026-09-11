//! PHP byte string: cheap static literals, copy-on-write heap storage.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub enum Str {
    Static(&'static [u8]),
    Heap(Rc<Vec<u8>>),
}

impl Str {
    #[inline]
    pub const fn from_static(s: &'static str) -> Str {
        Str::Static(s.as_bytes())
    }
    #[inline]
    pub const fn from_static_bytes(s: &'static [u8]) -> Str {
        Str::Static(s)
    }
    #[inline]
    pub fn empty() -> Str {
        Str::Static(b"")
    }
    #[inline]
    pub fn from_vec(v: Vec<u8>) -> Str {
        Str::Heap(Rc::new(v))
    }
    #[inline]
    pub fn from_bytes(v: &[u8]) -> Str {
        Str::Heap(Rc::new(v.to_vec()))
    }
    #[inline]
    pub fn from_string(s: String) -> Str {
        Str::Heap(Rc::new(s.into_bytes()))
    }
    #[inline]
    pub fn from_str(s: &str) -> Str {
        Str::Heap(Rc::new(s.as_bytes().to_vec()))
    }
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Str::Static(s) => s,
            Str::Heap(v) => v.as_slice(),
        }
    }
    /// Lossy UTF-8 view.
    pub fn to_string_lossy(&self) -> std::borrow::Cow<'_, str> {
        String::from_utf8_lossy(self.as_bytes())
    }
    /// Returns a &str if valid UTF-8, else lossy conversion.
    pub fn as_str(&self) -> std::borrow::Cow<'_, str> {
        match std::str::from_utf8(self.as_bytes()) {
            Ok(s) => std::borrow::Cow::Borrowed(s),
            Err(_) => String::from_utf8_lossy(self.as_bytes()),
        }
    }
    pub fn to_std_string(&self) -> String {
        self.as_str().into_owned()
    }
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
    pub fn into_vec(self) -> Vec<u8> {
        match self {
            Str::Static(s) => s.to_vec(),
            Str::Heap(v) => match Rc::try_unwrap(v) {
                Ok(v) => v,
                Err(rc) => (*rc).clone(),
            },
        }
    }
    /// Mutable access (copy-on-write).
    pub fn make_mut(&mut self) -> &mut Vec<u8> {
        if let Str::Static(s) = self {
            *self = Str::Heap(Rc::new(s.to_vec()));
        }
        match self {
            Str::Heap(v) => Rc::make_mut(v),
            _ => unreachable!(),
        }
    }
    pub fn push_bytes(&mut self, b: &[u8]) {
        if b.is_empty() {
            return;
        }
        if self.as_bytes().is_empty() {
            *self = Str::from_bytes(b);
            return;
        }
        self.make_mut().extend_from_slice(b);
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.as_bytes().is_empty()
    }
    pub fn to_lowercase(&self) -> Str {
        let b = self.as_bytes();
        if !b.iter().any(|c| c.is_ascii_uppercase()) {
            return self.clone();
        }
        Str::from_vec(b.to_ascii_lowercase())
    }
    pub fn to_uppercase(&self) -> Str {
        let b = self.as_bytes();
        if !b.iter().any(|c| c.is_ascii_lowercase()) {
            return self.clone();
        }
        Str::from_vec(b.to_ascii_uppercase())
    }
}

impl Deref for Str {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl PartialEq for Str {
    #[inline]
    fn eq(&self, other: &Str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}
impl Eq for Str {}
impl PartialEq<[u8]> for Str {
    fn eq(&self, other: &[u8]) -> bool {
        self.as_bytes() == other
    }
}
impl PartialEq<str> for Str {
    fn eq(&self, other: &str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}
impl PartialOrd for Str {
    fn partial_cmp(&self, other: &Str) -> Option<std::cmp::Ordering> {
        Some(self.as_bytes().cmp(other.as_bytes()))
    }
}
impl Ord for Str {
    fn cmp(&self, other: &Str) -> std::cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}
impl Hash for Str {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state)
    }
}
impl fmt::Debug for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_string_lossy())
    }
}
impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}
impl From<String> for Str {
    fn from(s: String) -> Str {
        Str::from_string(s)
    }
}
impl From<Vec<u8>> for Str {
    fn from(s: Vec<u8>) -> Str {
        Str::from_vec(s)
    }
}
impl Default for Str {
    fn default() -> Str {
        Str::empty()
    }
}

impl Str {
    /// Concatenate, reusing the left buffer when uniquely owned.
    pub fn concat(mut self, other: &[u8]) -> Str {
        self.push_bytes(other);
        self
    }
    pub fn push_str_slice(&mut self, s: &str) {
        self.push_bytes(s.as_bytes());
    }
    pub fn push_char(&mut self, c: u8) {
        self.make_mut().push(c);
    }
    pub fn starts_with(&self, p: &[u8]) -> bool {
        self.as_bytes().starts_with(p)
    }
    pub fn ends_with(&self, p: &[u8]) -> bool {
        self.as_bytes().ends_with(p)
    }
    pub fn slice(&self, start: usize, end: usize) -> Str {
        let b = self.as_bytes();
        let end = end.min(b.len());
        let start = start.min(end);
        if end - start == 1 {
            return crate::ops::single_byte_str(b[start]);
        }
        match self {
            Str::Static(s) => Str::Static(&s[start..end]),
            _ => Str::from_bytes(&b[start..end]),
        }
    }
    pub fn find(&self, needle: &[u8]) -> Option<usize> {
        find_bytes(self.as_bytes(), needle, 0)
    }
    pub fn eq_ignore_ascii_case(&self, other: &[u8]) -> bool {
        self.as_bytes().eq_ignore_ascii_case(other)
    }
}

/// Byte substring search starting at `from`.
pub fn find_bytes(hay: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    if from > hay.len() {
        return None;
    }
    if needle.is_empty() {
        return Some(from);
    }
    if needle.len() == 1 {
        return hay[from..].iter().position(|&c| c == needle[0]).map(|p| p + from);
    }
    memchr::memmem::find(&hay[from..], needle).map(|p| p + from)
}

pub fn rfind_bytes(hay: &[u8], needle: &[u8], end: usize) -> Option<usize> {
    let end = end.min(hay.len());
    if needle.is_empty() {
        return Some(end);
    }
    if needle.len() > end {
        return None;
    }
    let mut i = end - needle.len() + 1;
    while i > 0 {
        i -= 1;
        if &hay[i..i + needle.len()] == needle {
            return Some(i);
        }
    }
    None
}

impl From<&str> for Str {
    fn from(s: &str) -> Str {
        Str::from_str(s)
    }
}
impl From<&[u8]> for Str {
    fn from(s: &[u8]) -> Str {
        Str::from_bytes(s)
    }
}
impl std::borrow::Borrow<[u8]> for Str {
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}
impl AsRef<[u8]> for Str {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
/// Build a Str from a `format!`-like invocation.
#[macro_export]
macro_rules! sfmt {
    ($($arg:tt)*) => { $crate::Str::from_string(format!($($arg)*)) };
}
