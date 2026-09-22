//! PHP byte string: inline when short, cheap static literals, copy-on-write heap storage.
//!
//! Heap strings use a single allocation (like Zend's `zend_string`): one thin, non-atomically
//! reference-counted block holding the refcount, a cached hash, the length/capacity, and the bytes
//! inline. [`Str`] itself is 16 bytes (so [`crate::Mixed`] stays 24): a tag byte tells an inline
//! string of up to 15 bytes (no allocation, cloned by copy) from a static literal and from a heap
//! buffer, whose hash is computed once and reused across map lookups.

use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::cell::Cell;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::ptr::{self, NonNull};

/// Header of a heap string's single allocation; the bytes follow inline immediately after it.
#[repr(C)]
struct Hdr {
    /// Strong reference count (the program is single-threaded: a plain cell, like `Rc`).
    strong: Cell<usize>,
    /// Cached hash of the bytes; `0` means "not computed yet" (a real hash of 0 is bumped to 1).
    hash: Cell<u64>,
    /// Number of live bytes (mutated in place only while uniquely owned -- COW).
    len: Cell<usize>,
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
            ptr::write(p, Hdr { strong: Cell::new(1), hash: Cell::new(0), len: Cell::new(0), cap });
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
            hs.hdr().len.set(b.len());
        }
        hs
    }
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data_ptr() as *const u8, self.hdr().len.get()) }
    }
    #[inline]
    fn is_unique(&self) -> bool {
        self.hdr().strong.get() == 1
    }
    /// Cached byte-hash: compute once with `f`, then reuse (`0` is reserved for "uncomputed").
    #[inline]
    pub fn cached_hash(&self, f: impl FnOnce(&[u8]) -> u64) -> u64 {
        let cur = self.hdr().hash.get();
        if cur != 0 {
            return cur;
        }
        let mut v = f(self.as_bytes());
        if v == 0 {
            v = 1;
        }
        self.hdr().hash.set(v);
        v
    }
    /// Ensure this handle uniquely owns a buffer with room for `needed` bytes; copies on share/grow.
    /// After this call the current bytes are preserved, the buffer is unique, and the hash is cleared.
    fn ensure_unique_cap(&mut self, needed: usize) {
        let len = self.hdr().len.get();
        if self.is_unique() && needed <= self.hdr().cap {
            self.hdr().hash.set(0);
            return;
        }
        let new_cap = needed.max(self.hdr().cap.saturating_mul(2)).max(needed);
        let np = Self::alloc_with(new_cap);
        unsafe {
            ptr::copy_nonoverlapping(self.data_ptr() as *const u8, (np.as_ptr() as *mut u8).add(HDR_SIZE), len);
            np.as_ref().len.set(len);
        }
        let old = std::mem::replace(&mut self.ptr, np);
        drop(HeapStr { ptr: old });
    }
    fn push_slice(&mut self, b: &[u8]) {
        if b.is_empty() {
            return;
        }
        let len = self.hdr().len.get();
        self.ensure_unique_cap(len + b.len());
        unsafe {
            ptr::copy_nonoverlapping(b.as_ptr(), self.data_ptr().add(len), b.len());
            self.hdr().len.set(len + b.len());
        }
    }
    /// Set the byte at an in-range index (copies first if shared).
    fn set_byte(&mut self, idx: usize, c: u8) {
        let len = self.hdr().len.get();
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


impl Clone for HeapStr {
    #[inline]
    fn clone(&self) -> Self {
        let h = self.hdr();
        h.strong.set(h.strong.get() + 1);
        HeapStr { ptr: self.ptr }
    }
}

impl Drop for HeapStr {
    fn drop(&mut self) {
        let h = self.hdr();
        let strong = h.strong.get();
        h.strong.set(strong - 1);
        if strong == 1 {
            let cap = self.hdr().cap;
            unsafe {
                ptr::drop_in_place(self.ptr.as_ptr());
                dealloc(self.ptr.as_ptr() as *mut u8, Self::layout(cap));
            }
        }
    }
}

/// Bytes an inline string holds without an allocation (the 16-byte value minus the tag byte).
pub const INLINE_CAP: usize = 15;
const TAG_HEAP: u8 = 0xFE;
const TAG_STATIC: u8 = 0xFF;

/// The three layouts share the last byte as a tag: `0..=INLINE_CAP` is an inline string of that
/// length (bytes at the front), `TAG_STATIC` a `&'static [u8]` (pointer, then a u32 length),
/// `TAG_HEAP` a `HeapStr` pointer. Static and inline values are plain bytes: cloning them is a copy.
#[repr(C, align(8))]
#[derive(Clone, Copy)]
struct Inline {
    data: [u8; INLINE_CAP],
    tag: u8,
}
#[repr(C, align(8))]
#[derive(Clone, Copy)]
struct StaticRef {
    ptr: *const u8,
    len: u32,
    _pad: [u8; 3],
    tag: u8,
}
#[repr(C, align(8))]
struct HeapRef {
    ptr: std::mem::ManuallyDrop<HeapStr>,
    _pad: [u8; 7],
    tag: u8,
}
#[repr(C)]
union Repr {
    inline: Inline,
    stat: StaticRef,
    heap: std::mem::ManuallyDrop<HeapRef>,
}

/// PHP byte string: an inline value up to [`INLINE_CAP`] bytes, a static literal, or a
/// reference-counted heap buffer -- 16 bytes in every case.
pub struct Str {
    r: Repr,
}

impl Str {
    #[inline]
    fn tag(&self) -> u8 {
        // every layout keeps its tag in the last byte, so the inline view reads it
        unsafe { self.r.inline.tag }
    }
    #[inline]
    fn heap(&self) -> Option<&HeapStr> {
        if self.tag() == TAG_HEAP { Some(unsafe { &self.r.heap.ptr }) } else { None }
    }
    #[inline]
    fn heap_mut(&mut self) -> &mut HeapStr {
        debug_assert!(self.tag() == TAG_HEAP);
        // an explicit deref: the compiler will not reach through a ManuallyDrop union field itself
        unsafe { &mut *std::ops::DerefMut::deref_mut(&mut self.r.heap).ptr }
    }
    #[inline]
    fn from_heap(h: HeapStr) -> Str {
        Str { r: Repr { heap: std::mem::ManuallyDrop::new(HeapRef { ptr: std::mem::ManuallyDrop::new(h), _pad: [0; 7], tag: TAG_HEAP }) } }
    }
    #[inline]
    fn inline(b: &[u8]) -> Str {
        debug_assert!(b.len() <= INLINE_CAP);
        let mut data = [0u8; INLINE_CAP];
        data[..b.len()].copy_from_slice(b);
        Str { r: Repr { inline: Inline { data, tag: b.len() as u8 } } }
    }
    pub fn is_inline(&self) -> bool {
        self.tag() as usize <= INLINE_CAP
    }
    pub fn is_static(&self) -> bool {
        self.tag() == TAG_STATIC
    }
    pub fn is_heap(&self) -> bool {
        self.tag() == TAG_HEAP
    }
    #[inline]
    pub const fn from_static(s: &'static str) -> Str {
        Str::from_static_bytes(s.as_bytes())
    }
    #[inline]
    pub const fn from_static_bytes(s: &'static [u8]) -> Str {
        Str { r: Repr { stat: StaticRef { ptr: s.as_ptr(), len: s.len() as u32, _pad: [0; 3], tag: TAG_STATIC } } }
    }
    #[inline]
    pub fn empty() -> Str {
        Str::inline(b"")
    }
    #[inline]
    /// An empty string with room for `cap` bytes (a heap buffer only when they do not fit inline).
    #[inline]
    pub fn with_capacity(cap: usize) -> Str {
        if cap <= INLINE_CAP { Str::empty() } else { Str::from_heap(HeapStr::with_capacity(cap)) }
    }
    /// Make room for `extra` more bytes, so the pushes that follow do not reallocate.
    pub fn reserve(&mut self, extra: usize) {
        let len = self.len();
        if len + extra <= INLINE_CAP && !self.is_heap() {
            return;
        }
        if self.is_heap() {
            self.heap_mut().ensure_unique_cap(len + extra);
            return;
        }
        let mut h = HeapStr::with_capacity(len + extra);
        h.push_slice(self.as_bytes());
        *self = Str::from_heap(h);
    }
    /// `a . b` in one exact-size buffer.
    pub fn from_two(a: &[u8], b: &[u8]) -> Str {
        let n = a.len() + b.len();
        if n <= INLINE_CAP {
            let mut data = [0u8; INLINE_CAP];
            data[..a.len()].copy_from_slice(a);
            data[a.len()..n].copy_from_slice(b);
            return Str::inline(&data[..n]);
        }
        let mut h = HeapStr::with_capacity(n);
        h.push_slice(a);
        h.push_slice(b);
        Str::from_heap(h)
    }
    pub fn from_vec(v: Vec<u8>) -> Str {
        Str::from_bytes(&v)
    }
    #[inline]
    pub fn from_bytes(v: &[u8]) -> Str {
        if v.len() <= INLINE_CAP {
            return Str::inline(v);
        }
        Str::from_heap(HeapStr::from_slice(v))
    }
    #[inline]
    pub fn from_string(s: String) -> Str {
        Str::from_bytes(s.as_bytes())
    }
    #[inline]
    pub fn from_str(s: &str) -> Str {
        Str::from_bytes(s.as_bytes())
    }
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        let tag = self.tag();
        unsafe {
            if tag as usize <= INLINE_CAP {
                &self.r.inline.data[..tag as usize]
            } else if tag == TAG_STATIC {
                std::slice::from_raw_parts(self.r.stat.ptr, self.r.stat.len as usize)
            } else {
                self.r.heap.ptr.as_bytes()
            }
        }
    }
    /// Hash of the bytes, cached in a heap allocation (recomputed only after a mutation); inline
    /// and static strings recompute each call (they have nowhere to cache, and inline ones are
    /// short). Used by map lookups keyed on strings.
    #[inline]
    pub fn hash_cached(&self, f: impl FnOnce(&[u8]) -> u64) -> u64 {
        match self.heap() {
            Some(h) => h.cached_hash(f),
            None => f(self.as_bytes()),
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
        self.as_bytes().to_vec()
    }
    /// Copy-on-write append.
    pub fn push_bytes(&mut self, b: &[u8]) {
        if b.is_empty() {
            return;
        }
        let tag = self.tag();
        if tag as usize <= INLINE_CAP {
            let len = tag as usize;
            if len + b.len() <= INLINE_CAP {
                unsafe {
                    self.r.inline.data[len..len + b.len()].copy_from_slice(b);
                    self.r.inline.tag = (len + b.len()) as u8;
                }
                return;
            }
            let mut h = HeapStr::with_capacity(len + b.len());
            h.push_slice(self.as_bytes());
            h.push_slice(b);
            *self = Str::from_heap(h);
            return;
        }
        if tag == TAG_STATIC {
            let s = self.as_bytes();
            let mut h = HeapStr::with_capacity(s.len() + b.len());
            h.push_slice(s);
            h.push_slice(b);
            *self = Str::from_heap(h);
            return;
        }
        self.heap_mut().push_slice(b);
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
        let tag = self.tag();
        if tag as usize <= INLINE_CAP {
            unsafe {
                self.r.inline.data[idx] = c;
            }
            return;
        }
        if tag == TAG_STATIC {
            *self = Str::from_heap(HeapStr::from_slice(self.as_bytes()));
        }
        self.heap_mut().set_byte(idx, c);
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

impl Clone for Str {
    #[inline]
    fn clone(&self) -> Str {
        if self.tag() == TAG_HEAP {
            // a shared buffer: bump the count; the 16 bytes are then the same
            let h: HeapStr = unsafe { (*self.r.heap.ptr).clone() };
            std::mem::forget(h);
        }
        Str { r: Repr { inline: unsafe { self.r.inline } } }
    }
}

impl Drop for Str {
    #[inline]
    fn drop(&mut self) {
        if self.tag() == TAG_HEAP {
            unsafe {
                let heap = std::ops::DerefMut::deref_mut(&mut self.r.heap);
                std::mem::ManuallyDrop::drop(&mut heap.ptr);
            }
        }
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
        if self.is_static() && end - start > INLINE_CAP {
            // a long slice of a static literal stays a static reference
            return unsafe { Str::from_static_bytes(std::slice::from_raw_parts(b.as_ptr().add(start), end - start)) };
        }
        Str::from_bytes(&b[start..end])
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
        assert!(Str::from_bytes(b"short").is_inline());
        assert!(Str::from_bytes(b"exactly fifteen").is_inline());
        assert!(Str::from_bytes(b"sixteen bytes!!!").is_heap());
        assert!(Str::from_static("literal").is_static());
    }

    #[test]
    fn roundtrip_and_empty() {
        assert_eq!(Str::from_bytes(b"").as_bytes(), b"");
        assert!(Str::from_bytes(b"").is_inline());
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
        let s = Str::from_bytes(b"a type name longer than fifteen");
        if let Some(h) = s.heap() {
            let a = h.cached_hash(|b| super::super::string::tests::fnv(b));
            let b = h.cached_hash(|_| 0xdead_beef); // ignored: already cached
            assert_eq!(a, b);
            // the cache travels with clones (same allocation)
            let c = s.clone();
            if let Some(hc) = c.heap() {
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
        let mut s = Str::from_bytes(b"aaaaaaaaaaaaaaaaaaaa");
        if let Some(h) = s.heap() {
            let _ = h.cached_hash(|b| fnv(b));
        }
        s.push_bytes(b"bbb");
        // after mutation the hash must be recomputed from the new bytes
        if let Some(h) = s.heap() {
            assert_eq!(h.cached_hash(|b| fnv(b)), fnv(b"aaaaaaaaaaaaaaaaaaaabbb"));
        }
        // an inline string grows in place until it spills to the heap, keeping its bytes
        let mut i = Str::from_bytes(b"abc");
        i.push_bytes(b"defghijklmnop");
        assert!(i.is_heap());
        assert_eq!(i.as_bytes(), b"abcdefghijklmnop");
    }
}
