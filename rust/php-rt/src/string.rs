//! PHP byte string: cheap static literals, copy-on-write heap storage.
//!
//! Heap strings use a single allocation (like Zend's `zend_string`): one thin, non-atomically
//! reference-counted block holding the refcount, a cached hash, the length/capacity, and the bytes
//! inline. This keeps [`Str`] at 16 bytes (so [`crate::Mixed`] stays 24) via the null-pointer niche
//! of the `Static` variant, halves the allocations of the old `Rc<Vec<u8>>` layout, and lets a
//! string's hash be computed once and reused across map lookups.

use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::ptr::{self, NonNull};

/// Header of a heap string's single allocation; the bytes follow inline immediately after it.
#[repr(C)]
struct Hdr {
    /// Atomic strong reference count (axis-7: Str is Send+Sync, like `Arc`).
    strong: AtomicUsize,
    /// Cached hash of the bytes; `0` means "not computed yet" (a real hash of 0 is bumped to 1).
    hash: AtomicU64,
    /// Number of live bytes (mutated in place only while uniquely owned -- COW).
    len: AtomicUsize,
    /// Bytes of inline capacity available after the header (fixed per allocation).
    cap: usize,
}

const HDR_SIZE: usize = std::mem::size_of::<Hdr>();
const HDR_ALIGN: usize = std::mem::align_of::<Hdr>();

/// A thin (8-byte), single-allocation, reference-counted, copy-on-write byte string.
pub struct HeapStr {
    ptr: NonNull<Hdr>,
}

impl HeapStr {
    fn layout(cap: usize) -> Layout {
        Layout::from_size_align(HDR_SIZE + cap, HDR_ALIGN).expect("string layout overflow")
    }
    /// Allocate a block with `cap` bytes of inline capacity, refcount 1, length 0, uncached hash.
    fn alloc_with(cap: usize) -> NonNull<Hdr> {
        let l = Self::layout(cap);
        unsafe {
            let p = alloc(l) as *mut Hdr;
            if p.is_null() {
                handle_alloc_error(l);
            }
            ptr::write(p, Hdr { strong: AtomicUsize::new(1), hash: AtomicU64::new(0), len: AtomicUsize::new(0), cap });
            NonNull::new_unchecked(p)
        }
    }
    #[inline]
    fn hdr(&self) -> &Hdr {
        unsafe { self.ptr.as_ref() }
    }
    #[inline]
    fn data_ptr(&self) -> *mut u8 {
        unsafe { (self.ptr.as_ptr() as *mut u8).add(HDR_SIZE) }
    }
    fn with_capacity(cap: usize) -> HeapStr {
        HeapStr { ptr: Self::alloc_with(cap) }
    }
    fn from_slice(b: &[u8]) -> HeapStr {
        let hs = Self::with_capacity(b.len());
        unsafe {
            ptr::copy_nonoverlapping(b.as_ptr(), hs.data_ptr(), b.len());
            hs.hdr().len.store(b.len(), Ordering::Relaxed);
        }
        hs
    }
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data_ptr() as *const u8, self.hdr().len.load(Ordering::Relaxed)) }
    }
    #[inline]
    fn is_unique(&self) -> bool {
        self.hdr().strong.load(Ordering::Relaxed) == 1
    }
    /// Cached byte-hash: compute once with `f`, then reuse (`0` is reserved for "uncomputed").
    #[inline]
    pub fn cached_hash(&self, f: impl FnOnce(&[u8]) -> u64) -> u64 {
        let cur = self.hdr().hash.load(Ordering::Relaxed);
        if cur != 0 {
            return cur;
        }
        let mut v = f(self.as_bytes());
        if v == 0 {
            v = 1;
        }
        self.hdr().hash.store(v, Ordering::Relaxed);
        v
    }
    /// Ensure this handle uniquely owns a buffer with room for `needed` bytes; copies on share/grow.
    /// After this call the current bytes are preserved, the buffer is unique, and the hash is cleared.
    fn ensure_unique_cap(&mut self, needed: usize) {
        let len = self.hdr().len.load(Ordering::Relaxed);
        if self.is_unique() && needed <= self.hdr().cap {
            self.hdr().hash.store(0, Ordering::Relaxed);
            return;
        }
        let new_cap = needed.max(self.hdr().cap.saturating_mul(2)).max(needed);
        let np = Self::alloc_with(new_cap);
        unsafe {
            ptr::copy_nonoverlapping(self.data_ptr() as *const u8, (np.as_ptr() as *mut u8).add(HDR_SIZE), len);
            np.as_ref().len.store(len, Ordering::Relaxed);
        }
        let old = std::mem::replace(&mut self.ptr, np);
        drop(HeapStr { ptr: old });
    }
    fn push_slice(&mut self, b: &[u8]) {
        if b.is_empty() {
            return;
        }
        let len = self.hdr().len.load(Ordering::Relaxed);
        self.ensure_unique_cap(len + b.len());
        unsafe {
            ptr::copy_nonoverlapping(b.as_ptr(), self.data_ptr().add(len), b.len());
            self.hdr().len.store(len + b.len(), Ordering::Relaxed);
        }
    }
    /// Set the byte at an in-range index (copies first if shared).
    fn set_byte(&mut self, idx: usize, c: u8) {
        let len = self.hdr().len.load(Ordering::Relaxed);
        debug_assert!(idx < len);
        self.ensure_unique_cap(len);
        unsafe {
            *self.data_ptr().add(idx) = c;
        }
    }
    fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

// Axis-7: the payload is immutable while shared (COW: in-place mutation only when strong==1), and the
// refcount is atomic, so HeapStr is safe to Send/Sync between threads (same invariant as Arc<[u8]>).
unsafe impl Send for HeapStr {}
unsafe impl Sync for HeapStr {}

impl Clone for HeapStr {
    #[inline]
    fn clone(&self) -> Self {
        self.hdr().strong.fetch_add(1, Ordering::Relaxed);
        HeapStr { ptr: self.ptr }
    }
}

impl Drop for HeapStr {
    fn drop(&mut self) {
        if self.hdr().strong.fetch_sub(1, Ordering::Release) == 1 {
            std::sync::atomic::fence(Ordering::Acquire);
            let cap = self.hdr().cap;
            unsafe {
                ptr::drop_in_place(self.ptr.as_ptr());
                dealloc(self.ptr.as_ptr() as *mut u8, Self::layout(cap));
            }
        }
    }
}

#[derive(Clone)]
pub enum Str {
    Static(&'static [u8]),
    Heap(HeapStr),
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
        if v.is_empty() {
            return Str::empty();
        }
        Str::Heap(HeapStr::from_slice(&v))
    }
    #[inline]
    pub fn from_bytes(v: &[u8]) -> Str {
        if v.is_empty() {
            return Str::empty();
        }
        Str::Heap(HeapStr::from_slice(v))
    }
    #[inline]
    pub fn from_string(s: String) -> Str {
        Str::from_vec(s.into_bytes())
    }
    #[inline]
    pub fn from_str(s: &str) -> Str {
        Str::from_bytes(s.as_bytes())
    }
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Str::Static(s) => s,
            Str::Heap(v) => v.as_bytes(),
        }
    }
    /// Hash of the bytes, cached in the heap allocation (recomputed only after a mutation). Static
    /// strings recompute each call (they have nowhere to cache). Used by map lookups keyed on strings.
    #[inline]
    pub fn hash_cached(&self, f: impl FnOnce(&[u8]) -> u64) -> u64 {
        match self {
            Str::Heap(h) => h.cached_hash(f),
            Str::Static(s) => f(s),
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
            Str::Heap(v) => v.to_vec(),
        }
    }
    /// Copy-on-write append.
    pub fn push_bytes(&mut self, b: &[u8]) {
        if b.is_empty() {
            return;
        }
        match self {
            Str::Heap(h) => h.push_slice(b),
            Str::Static(s) => {
                if s.is_empty() {
                    *self = Str::Heap(HeapStr::from_slice(b));
                } else {
                    let mut h = HeapStr::with_capacity(s.len() + b.len());
                    h.push_slice(s);
                    h.push_slice(b);
                    *self = Str::Heap(h);
                }
            }
        }
    }
    /// `$s[$i] = $c`: set a byte, growing with spaces (copy-on-write).
    pub fn set_index(&mut self, idx: usize, c: u8) {
        let cur = self.len();
        if idx >= cur {
            let mut pad = Vec::with_capacity(idx + 1 - cur);
            pad.resize(idx - cur, b' ');
            pad.push(c);
            self.push_bytes(&pad);
            return;
        }
        if let Str::Static(s) = self {
            *self = Str::Heap(HeapStr::from_slice(s));
        }
        if let Str::Heap(h) = self {
            h.set_byte(idx, c);
        }
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
        self.push_bytes(&[c]);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_invariant() {
        // Str must stay 16 bytes (niche-packed) so Mixed stays 24.
        assert_eq!(std::mem::size_of::<Str>(), 16);
        assert_eq!(std::mem::size_of::<HeapStr>(), 8);
    }

    #[test]
    fn roundtrip_and_empty() {
        assert_eq!(Str::from_bytes(b"").as_bytes(), b"");
        assert!(matches!(Str::from_bytes(b""), Str::Static(_)));
        let s = Str::from_bytes(b"hello");
        assert_eq!(s.as_bytes(), b"hello");
        assert_eq!(s.len(), 5);
        assert_eq!(s.clone().into_vec(), b"hello".to_vec());
    }

    #[test]
    fn cow_clone_is_independent() {
        let a = Str::from_bytes(b"abc");
        let mut b = a.clone();
        b.push_bytes(b"def"); // must copy-on-write, not mutate `a`
        assert_eq!(a.as_bytes(), b"abc");
        assert_eq!(b.as_bytes(), b"abcdef");
    }

    #[test]
    fn append_grows_amortized() {
        let mut s = Str::from_bytes(b"x");
        for _ in 0..1000 {
            s.push_bytes(b"ab");
        }
        assert_eq!(s.len(), 1 + 2000);
        assert!(s.as_bytes().ends_with(b"ab"));
        assert!(s.as_bytes().starts_with(b"xab"));
    }

    #[test]
    fn push_onto_static() {
        let mut s = Str::from_static("pre-");
        s.push_bytes(b"post");
        assert_eq!(s.as_bytes(), b"pre-post");
        let mut e = Str::empty();
        e.push_bytes(b"z");
        assert_eq!(e.as_bytes(), b"z");
    }

    #[test]
    fn set_index_in_range_and_padding() {
        let mut s = Str::from_bytes(b"cat");
        s.set_index(0, b'b');
        assert_eq!(s.as_bytes(), b"bat");
        // padding past the end fills with spaces
        let mut p = Str::from_bytes(b"ab");
        p.set_index(5, b'Z');
        assert_eq!(p.as_bytes(), b"ab   Z");
        // set_index on a shared value copies on write
        let orig = Str::from_bytes(b"cat");
        let mut d = orig.clone();
        d.set_index(0, b'h');
        assert_eq!(orig.as_bytes(), b"cat");
        assert_eq!(d.as_bytes(), b"hat");
    }

    #[test]
    fn cached_hash_is_stable_and_shared() {
        let s = Str::from_bytes(b"typename");
        if let Str::Heap(h) = &s {
            let a = h.cached_hash(|b| super::super::string::tests::fnv(b));
            let b = h.cached_hash(|_| 0xdead_beef); // ignored: already cached
            assert_eq!(a, b);
            // the cache travels with clones (same allocation)
            let c = s.clone();
            if let Str::Heap(hc) = &c {
                assert_eq!(hc.cached_hash(|_| 0), a);
            }
        } else {
            panic!("expected heap");
        }
    }

    // small helper so the cached_hash test has a real hash fn
    pub fn fnv(b: &[u8]) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for &c in b {
            h ^= c as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h
    }

    #[test]
    fn mutation_clears_cached_hash() {
        let mut s = Str::from_bytes(b"aaa");
        if let Str::Heap(h) = &s {
            let _ = h.cached_hash(|b| fnv(b));
        }
        s.push_bytes(b"bbb");
        // after mutation the hash must be recomputed from the new bytes
        if let Str::Heap(h) = &s {
            assert_eq!(h.cached_hash(|b| fnv(b)), fnv(b"aaabbb"));
        }
    }
}
