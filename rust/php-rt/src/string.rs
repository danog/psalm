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
    /// The interned lowercase copy of these bytes (`to_lowercase` of a name: a class name held in storage is
    /// lowercased on every lookup), or 0 until computed; cleared with the hash on any in-place mutation.
    lower: Cell<usize>,
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
        #[cfg(feature = "stats")]
        {
            crate::stats::bump(crate::stats::STR_HEAP_ALLOC);
            if cap <= 23 { crate::stats::bump(crate::stats::STR_HEAP_ALLOC_LE_23); }
            if cap <= 32 { crate::stats::bump(crate::stats::STR_HEAP_ALLOC_LE_32); } else if cap <= 128 { crate::stats::bump(crate::stats::STR_HEAP_ALLOC_LE_128); }
        }
        let l = Self::layout(cap);
        unsafe {
            let p = alloc(l) as *mut Hdr;
            if p.is_null() {
                handle_alloc_error(l);
            }
            ptr::write(p, Hdr { strong: Cell::new(1), hash: Cell::new(0), len: Cell::new(0), lower: Cell::new(0), cap });
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
        crate::stats::bump(crate::stats::STR_HASH_COMPUTE);
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
            self.hdr().lower.set(0);
            return;
        }
        crate::stats::bump(crate::stats::STR_COW_GROW);
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
/// Longest text `to_lowercase` interns (longer results are ordinary heap strings).
pub const INTERN_CAP: usize = 64;
const TAG_HEAP: u8 = 0xFE;
const TAG_STATIC: u8 = 0xFF;
/// An interned string: the same pointer/length layout as `TAG_STATIC`, into a never-freed arena record
/// whose 8 bytes before the text hold the map hash of the text (see [`Str::intern`]).
const TAG_INTERNED: u8 = 0xFD;

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
    pub fn is_interned(&self) -> bool {
        self.tag() == TAG_INTERNED
    }
    /// A static literal or an interned string: borrowed bytes that live forever (copied before any mutation).
    #[inline]
    fn is_borrowed(&self) -> bool {
        matches!(self.tag(), TAG_STATIC | TAG_INTERNED)
    }
    /// The interned copy of `b`: one arena record per distinct text on this thread, with the map hash stored
    /// in front of the bytes. Cloning and dropping an interned string is a copy of 16 bytes, its map hash is
    /// read instead of computed, and two interned handles to the same record compare by pointer. Lowercased
    /// names (class, method, property and variable ids, the keys of the hottest maps) are interned by
    /// `to_lowercase`; anything else stays as it is. Never freed: the vocabulary of a run is bounded.
    pub fn intern(b: &[u8]) -> Str {
        thread_local! {
            static INTERNED: std::cell::RefCell<crate::FastMap<&'static [u8], usize>> = std::cell::RefCell::new(crate::fast_map());
        }
        INTERNED.with(|t| {
            let mut t = t.borrow_mut();
            if let Some(&p) = t.get(b) {
                return Str { r: Repr { stat: StaticRef { ptr: p as *const u8, len: b.len() as u32, _pad: [0; 3], tag: TAG_INTERNED } } };
            }
            crate::stats::bump(crate::stats::STR_INTERN_NEW);
            let mut h = crate::map::hash_bytes(b);
            if h == 0 {
                h = 1;
            }
            // record: [map hash u64][lowercase twin ptr, 0 until computed][bytes]
            let layout = Layout::from_size_align(16 + b.len(), 8).unwrap();
            let base = unsafe { alloc(layout) };
            if base.is_null() {
                handle_alloc_error(layout);
            }
            let text = unsafe {
                (base as *mut u64).write(h);
                (base as *mut usize).add(1).write(0);
                std::ptr::copy_nonoverlapping(b.as_ptr(), base.add(16), b.len());
                std::slice::from_raw_parts(base.add(16) as *const u8, b.len())
            };
            t.insert(text, text.as_ptr() as usize);
            Str { r: Repr { stat: StaticRef { ptr: text.as_ptr(), len: b.len() as u32, _pad: [0; 3], tag: TAG_INTERNED } } }
        })
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
    /// The decimal text of an integer, inline (no heap: an i64 has at most 20 characters, and the common
    /// small keys of `(string)$int` fit the inline buffer).
    pub fn from_int(i: i64) -> Str {
        let mut buf = [0u8; 20];
        let mut n = buf.len();
        let neg = i < 0;
        let mut u = i.unsigned_abs();
        loop {
            n -= 1;
            buf[n] = b'0' + (u % 10) as u8;
            u /= 10;
            if u == 0 {
                break;
            }
        }
        if neg {
            n -= 1;
            buf[n] = b'-';
        }
        Str::from_bytes(&buf[n..])
    }

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
            } else if tag == TAG_STATIC || tag == TAG_INTERNED {
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
        if self.tag() == TAG_INTERNED {
            // the map hash stored in front of the interned text (`f` is that hash function)
            return unsafe { (self.r.stat.ptr as *const u64).sub(2).read() };
        }
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
        if tag == TAG_STATIC || tag == TAG_INTERNED {
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
        if tag == TAG_STATIC || tag == TAG_INTERNED {
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
        crate::stats::bump(crate::stats::STR_LOWER);
        let b = self.as_bytes();
        if self.tag() == TAG_INTERNED {
            // an interned name remembers its lowercase twin (itself when already lowercase): no hashing again
            let slot = unsafe { (self.r.stat.ptr as *const usize).sub(1) as *mut usize };
            let twin = unsafe { slot.read() };
            if twin != 0 {
                return Str { r: Repr { stat: StaticRef { ptr: twin as *const u8, len: b.len() as u32, _pad: [0; 3], tag: TAG_INTERNED } } };
            }
            let lower = if b.iter().any(|c| c.is_ascii_uppercase()) {
                let mut buf = [0u8; INTERN_CAP];
                buf[..b.len()].copy_from_slice(b);
                buf[..b.len()].make_ascii_lowercase();
                Str::intern(&buf[..b.len()])
            } else {
                self.clone()
            };
            unsafe { slot.write(lower.r.stat.ptr as usize) };
            return lower;
        }
        // a lowercased name of a class, method, property or variable is a map key somewhere: interned (see intern)
        if b.len() > INLINE_CAP && b.len() <= INTERN_CAP {
            let heap = self.heap();
            if let Some(h) = heap {
                let twin = h.hdr().lower.get();
                if twin != 0 {
                    return Str { r: Repr { stat: StaticRef { ptr: twin as *const u8, len: b.len() as u32, _pad: [0; 3], tag: TAG_INTERNED } } };
                }
            }
            let mut buf = [0u8; INTERN_CAP];
            buf[..b.len()].copy_from_slice(b);
            buf[..b.len()].make_ascii_lowercase();
            let lower = Str::intern(&buf[..b.len()]);
            if let Some(h) = heap {
                h.hdr().lower.set(unsafe { lower.r.stat.ptr } as usize);
            }
            return lower;
        }
        match b.iter().position(|c| c.is_ascii_uppercase()) {
            None => self.clone(),
            Some(first) => Str::mapped_from(b, first, |c| c.to_ascii_lowercase()),
        }
    }
    /// `b` with every byte from `from` on passed through `f`, in one allocation (the prefix is copied as is).
    fn mapped_from(b: &[u8], from: usize, f: impl Fn(u8) -> u8) -> Str {
        if b.len() <= INLINE_CAP {
            let mut data = [0u8; INLINE_CAP];
            data[..b.len()].copy_from_slice(b);
            for c in &mut data[from..b.len()] {
                *c = f(*c);
            }
            return Str::inline(&data[..b.len()]);
        }
        let h = HeapStr::with_capacity(b.len());
        unsafe {
            let dst = std::slice::from_raw_parts_mut(h.data_ptr(), b.len());
            dst[..from].copy_from_slice(&b[..from]);
            for (d, s) in dst[from..].iter_mut().zip(&b[from..]) {
                *d = f(*s);
            }
            h.hdr().len.set(b.len());
        }
        Str::from_heap(h)
    }
    pub fn to_uppercase(&self) -> Str {
        let b = self.as_bytes();
        match b.iter().position(|c| c.is_ascii_lowercase()) {
            None => self.clone(),
            Some(first) => Str::mapped_from(b, first, |c| c.to_ascii_uppercase()),
        }
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
        let (ta, tb) = (self.tag(), other.tag());
        // two inline strings: the 16 bytes are the text, its length and zero padding (every inline constructor
        // zero-fills and writes stay within the length), so equal texts are equal words: no memcmp call
        if ta as usize <= INLINE_CAP && tb as usize <= INLINE_CAP {
            let (a, b) = unsafe {
                let pa = self as *const Str as *const [u64; 2];
                let pb = other as *const Str as *const [u64; 2];
                (pa.read(), pb.read())
            };
            return a == b;
        }
        // two handles to the same interned record (the usual case for map keys) need no byte compare;
        // different records may still hold the same text (interning is per thread), so no fast inequality
        if ta == TAG_INTERNED && tb == TAG_INTERNED && unsafe { self.r.stat.ptr == other.r.stat.ptr } {
            return true;
        }
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
        if self.is_borrowed() && end - start > INLINE_CAP {
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
        return memchr::memchr(needle[0], &hay[from..]).map(|p| p + from);
    }
    let h = &hay[from..];
    if h.len() < needle.len() {
        return None;
    }
    if h.len() <= 64 {
        // short haystacks (variable ids, class names): scan for the first byte and compare, without building
        // memmem's searcher (its setup cost more than the search itself: 200 instructions per call)
        let (first, rest) = (needle[0], &needle[1..]);
        let last_start = h.len() - needle.len();
        let mut i = 0;
        while i <= last_start {
            match memchr::memchr(first, &h[i..=last_start]) {
                None => return None,
                Some(p) => {
                    let at = i + p;
                    if &h[at + 1..at + needle.len()] == rest {
                        return Some(at + from);
                    }
                    i = at + 1;
                }
            }
        }
        return None;
    }
    memchr::memmem::find(h, needle).map(|p| p + from)
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
    #[test]
    fn interned_strings_share_a_record_and_compare() {
        let a = Str::intern(b"a lowercased class name here");
        let b = Str::intern(b"a lowercased class name here");
        assert!(a.is_interned() && b.is_interned());
        assert_eq!(a, b);
        assert_eq!(a.as_bytes(), b"a lowercased class name here");
        assert_eq!(a.hash_cached(crate::map::hash_bytes), crate::map::hash_bytes(b"a lowercased class name here"));
        let heap = Str::from_bytes(b"a lowercased class name here");
        assert_eq!(a, heap);
        assert_eq!(heap, a);
        assert_ne!(a, Str::intern(b"a lowercased class name herf"));
        let lower = Str::from_bytes(b"Some\\Namespace\\ClassName").to_lowercase();
        assert!(lower.is_interned());
        assert_eq!(lower.as_bytes(), b"some\\namespace\\classname");
        let mut m = lower.clone();
        m.push_bytes(b"!");
        assert_eq!(m.as_bytes(), b"some\\namespace\\classname!");
        assert!(lower.is_interned());
    }

    #[test]
    fn from_int_matches_display() {
        for i in [0i64, 1, -1, 7, 10, 12345, -12345, i64::MAX, i64::MIN, 999_999_999_999_999, -999_999_999_999_99] {
            assert_eq!(super::Str::from_int(i).as_bytes(), i.to_string().as_bytes(), "{i}");
        }
        assert!(super::Str::from_int(-999_999_999_999_99).is_inline());
    }
    use super::*;

    #[test]
    fn size_invariant() {
        // Str must stay 16 bytes (niche-packed) so Mixed stays 24.
        assert_eq!(std::mem::size_of::<Str>(), 16);
        let mut a = Str::from("ab");
        a.push_bytes(b"cd");
        assert!(a == Str::from("abcd"));
        assert!(Str::from("abcd") != Str::from("abce"));
        assert!(Str::from("abc") != Str::from("abc\0"));
        assert_eq!(super::find_bytes(b"a->b->c", b"->", 0), Some(1));
        assert_eq!(super::find_bytes(b"a->b->c", b"->", 2), Some(4));
        assert_eq!(super::find_bytes(b"abc", b"bc", 0), Some(1));
        assert_eq!(super::find_bytes(b"abc", b"bcd", 0), None);
        assert_eq!(super::find_bytes(b"aab", b"ab", 0), Some(1));
        assert_eq!(super::find_bytes(b"ab", b"ab", 1), None);
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

#[cfg(test)]
mod case_tests {
    use super::*;
    #[test]
    fn lower_upper() {
        let long = Str::from_bytes(b"Psalm\\Internal\\Codebase\\ClassLikes");
        assert_eq!(long.to_lowercase().as_bytes(), b"psalm\\internal\\codebase\\classlikes");
        assert_eq!(Str::from_bytes(b"abcD").to_lowercase().as_bytes(), b"abcd");
        assert_eq!(Str::from_bytes(b"abc").to_uppercase().as_bytes(), b"ABC");
        let already = Str::from_bytes(b"already lowercase and long enough");
        assert_eq!(already.to_lowercase().as_bytes(), already.as_bytes());
        assert_eq!(long.to_uppercase().as_bytes(), b"PSALM\\INTERNAL\\CODEBASE\\CLASSLIKES");
    }
}
