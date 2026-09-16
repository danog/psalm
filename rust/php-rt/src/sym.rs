//! Interned symbols (`Sym`) — the runtime foundation for the StrId axis.
//!
//! PHP class / interface / method / property / type names recur constantly during analysis; representing
//! them as [`Sym`] (a `u32` index into a global interner) makes comparison and hashing integer ops and
//! deduplicates their storage, exactly like pzoom's `StrId`/`Interner` and Zend's interned strings.
//!
//! The interner is process-global (so a `Sym` is valid on any thread — the analysis runs on a worker
//! pool) and append-only: interned bytes are leaked to `'static`, which is intentional (identifiers live
//! for the whole process and are bounded in number). Because the bytes are `'static`, turning a `Sym`
//! back into a [`Str`] yields a zero-allocation [`Str::Static`].

use crate::string::Str;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// An interned identifier: a small copyable handle whose equality/hash are integer operations.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Sym(pub u32);

struct Interner {
    /// id -> the interned (leaked) bytes.
    ids: Vec<&'static [u8]>,
    /// bytes -> id, using the same fast hash the maps use.
    map: HashMap<&'static [u8], u32, foldhash::fast::FixedState>,
}

fn interner() -> &'static RwLock<Interner> {
    static INTERNER: OnceLock<RwLock<Interner>> = OnceLock::new();
    INTERNER.get_or_init(|| {
        RwLock::new(Interner {
            ids: Vec::new(),
            map: HashMap::with_hasher(foldhash::fast::FixedState::with_seed(0x5eed_1234_abcd_9876)),
        })
    })
}

impl Sym {
    /// Intern `bytes`, returning its stable [`Sym`]. Idempotent: equal bytes always yield the same `Sym`.
    pub fn intern(bytes: &[u8]) -> Sym {
        // fast path: a shared read lock is enough for the common (already-interned) case
        {
            let g = interner().read().expect("sym interner poisoned");
            if let Some(&id) = g.map.get(bytes) {
                return Sym(id);
            }
        }
        let mut g = interner().write().expect("sym interner poisoned");
        // re-check under the write lock (another thread may have inserted it meanwhile)
        if let Some(&id) = g.map.get(bytes) {
            return Sym(id);
        }
        let leaked: &'static [u8] = Box::leak(bytes.to_vec().into_boxed_slice());
        let id = g.ids.len() as u32;
        g.ids.push(leaked);
        g.map.insert(leaked, id);
        Sym(id)
    }

    #[inline]
    pub fn intern_str(s: &str) -> Sym {
        Sym::intern(s.as_bytes())
    }

    /// Intern the bytes of a [`Str`].
    #[inline]
    pub fn from_str(s: &Str) -> Sym {
        Sym::intern(s.as_bytes())
    }

    /// The interned bytes (`'static`, since the interner never frees).
    #[inline]
    pub fn as_bytes(self) -> &'static [u8] {
        let g = interner().read().expect("sym interner poisoned");
        g.ids[self.0 as usize]
    }

    /// A zero-allocation [`Str`] view of this symbol (the bytes are `'static`).
    #[inline]
    pub fn to_str(self) -> Str {
        Str::from_static_bytes(self.as_bytes())
    }
}

impl Default for Sym {
    #[inline]
    fn default() -> Self {
        Sym::intern(b"")
    }
}

impl std::fmt::Debug for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sym({}, {})", self.0, String::from_utf8_lossy(self.as_bytes()))
    }
}

impl std::fmt::Display for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from_utf8_lossy(self.as_bytes()))
    }
}

// ---- interop trait impls so the transpiler can emit `Sym` as a first-class value type ----

use crate::cast::{cast, CastTo};
use crate::key::{ArrayKey, KeyQuery, MapKey};
use crate::mixed::Mixed;
use crate::traits::{Identical, ToStr, Truthy};

impl CastTo<Str> for Sym {
    #[inline]
    fn cast_to(self) -> Str {
        self.to_str()
    }
}
impl CastTo<Mixed> for Sym {
    #[inline]
    fn cast_to(self) -> Mixed {
        Mixed::Str(self.to_str())
    }
}
impl CastTo<Sym> for Str {
    #[inline]
    fn cast_to(self) -> Sym {
        Sym::from_str(&self)
    }
}
impl CastTo<Sym> for Mixed {
    #[inline]
    fn cast_to(self) -> Sym {
        Sym::from_str(&cast::<Str>(self))
    }
}

impl crate::traits::PhpKind for Sym {
    fn php_kind(&self) -> crate::traits::Kind { crate::traits::Kind::Str }
}
impl crate::traits::InstanceOfName for Sym {
    fn php_instance_of(&self, _name: &[u8]) -> bool { false }
}
impl Truthy for Sym {
    #[inline]
    fn truthy(&self) -> bool {
        // PHP string truthiness: "" and "0" are false. Identifiers are neither, but stay correct.
        !matches!(self.as_bytes(), b"" | b"0")
    }
}
impl ToStr for Sym {
    #[inline]
    fn to_php_str(&self) -> Str {
        self.to_str()
    }
}
impl Identical for Sym {
    #[inline]
    fn identical(&self, other: &Self) -> bool {
        // interning dedups, so equal bytes <=> equal id
        self.0 == other.0
    }
}

impl KeyQuery for Sym {
    #[inline]
    fn packed_index(&self) -> Option<i64> {
        None
    }
    #[inline]
    fn map_hash(&self) -> u64 {
        // must match Str's map_hash for the same bytes so Sym/Str keys agree
        crate::map::hash_bytes(self.as_bytes())
    }
}
impl MapKey for Sym {
    #[inline]
    fn to_array_key(&self) -> ArrayKey {
        self.to_str().to_array_key()
    }
    #[inline]
    fn from_array_key(k: ArrayKey) -> Sym {
        Sym::from_str(&Str::from_array_key(k))
    }
    #[inline]
    fn int_value(&self) -> Option<i64> {
        MapKey::int_value(&self.to_str())
    }
    #[inline]
    fn from_index(i: i64) -> Sym {
        Sym::from_str(&<Str as MapKey>::from_index(i))
    }
}

impl crate::traits::PhpCmp for Sym {
    fn php_cmp(&self, o: &Self) -> std::cmp::Ordering {
        crate::traits::ToStr::to_php_str(self).php_cmp(&crate::traits::ToStr::to_php_str(o))
    }
}
