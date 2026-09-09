//! Copy-on-write insertion-ordered hash map (PHP `array<K, V>`).

use crate::key::{ArrayKey, MapKey};
use crate::list::List;
use hashbrown::HashTable;
use std::borrow::Borrow;
use std::fmt;
use std::hash::{BuildHasher, Hash, Hasher};
use std::rc::Rc;

#[inline]
fn hash_of<Q: Hash + ?Sized>(q: &Q) -> u64 {
    let mut h = foldhash::fast::FixedState::with_seed(0x5eed_1234_abcd_9876).build_hasher();
    q.hash(&mut h);
    h.finish()
}

pub struct OrderedMap<K, V> {
    entries: Vec<Option<(K, V)>>,
    table: HashTable<usize>,
    len: usize,
    next_index: i64,
    /// PHP's internal array pointer (an index into `entries`).
    pos: usize,
}

impl<K: Clone, V: Clone> Clone for OrderedMap<K, V> {
    fn clone(&self) -> Self {
        OrderedMap { entries: self.entries.clone(), table: self.table.clone(), len: self.len, next_index: self.next_index, pos: self.pos }
    }
}

impl<K: MapKey, V> OrderedMap<K, V> {
    fn new() -> Self {
        OrderedMap { entries: Vec::new(), table: HashTable::new(), len: 0, next_index: 0, pos: 0 }
    }
    fn with_capacity(n: usize) -> Self {
        OrderedMap { entries: Vec::with_capacity(n), table: HashTable::with_capacity(n), len: 0, next_index: 0, pos: 0 }
    }
    #[inline]
    fn key_at(&self, idx: usize) -> &K {
        match &self.entries[idx] {
            Some((k, _)) => k,
            None => unreachable!(),
        }
    }
    fn find<Q: ?Sized + Hash + Eq>(&self, q: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
    {
        let h = hash_of(q);
        self.table.find(h, |&idx| self.key_at(idx).borrow() == q).copied()
    }
    fn insert_new(&mut self, key: K, value: V) -> usize {
        if let Some(i) = key.int_value() {
            if i >= self.next_index {
                self.next_index = i.wrapping_add(1);
            }
        }
        let idx = self.entries.len();
        let h = hash_of(&key);
        self.entries.push(Some((key, value)));
        self.len += 1;
        let entries = &self.entries;
        self.table.insert_unique(h, idx, |&i| match &entries[i] {
            Some((k, _)) => hash_of(k),
            None => unreachable!(),
        });
        idx
    }
    fn remove_at(&mut self, idx: usize) -> (K, V) {
        let (k, v) = self.entries[idx].take().unwrap();
        self.len -= 1;
        let h = hash_of(&k);
        if let Ok(e) = self.table.find_entry(h, |&i| i == idx) {
            e.remove();
        }
        if self.entries.len() > 16 && self.len * 2 < self.entries.len() {
            self.compact();
        }
        (k, v)
    }
    fn compact(&mut self) {
        let old = std::mem::take(&mut self.entries);
        self.pos = old.iter().take(self.pos).filter(|e| e.is_some()).count();
        self.entries = old.into_iter().flatten().map(Some).collect();
        self.table.clear();
        let entries = &self.entries;
        for (idx, e) in entries.iter().enumerate() {
            let h = hash_of(&e.as_ref().unwrap().0);
            self.table.insert_unique(h, idx, |&i| hash_of(&entries[i].as_ref().unwrap().0));
        }
    }
}

/// Reference-counted, copy-on-write ordered map.
pub struct Map<K, V>(Rc<OrderedMap<K, V>>);

impl<K, V> Clone for Map<K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Map(self.0.clone())
    }
}

impl<K: MapKey, V> Default for Map<K, V> {
    fn default() -> Self {
        Map(Rc::new(OrderedMap::new()))
    }
}

pub struct MapIter<'a, K, V> {
    entries: &'a [Option<(K, V)>],
    idx: usize,
}
impl<'a, K, V> Iterator for MapIter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.entries.len() {
            let i = self.idx;
            self.idx += 1;
            if let Some((k, v)) = &self.entries[i] {
                return Some((k, v));
            }
        }
        None
    }
}

impl<K: MapKey, V> Map<K, V> {
    #[inline]
    pub fn new() -> Self {
        Map(Rc::new(OrderedMap::new()))
    }
    pub fn with_capacity(n: usize) -> Self {
        Map(Rc::new(OrderedMap::with_capacity(n)))
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len
    }
    #[inline]
    pub fn count(&self) -> i64 {
        self.0.len as i64
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.len == 0
    }
    #[inline]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
    pub fn next_index(&self) -> i64 {
        self.0.next_index
    }
    #[inline]
    pub fn get<Q: ?Sized + Hash + Eq>(&self, q: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        let idx = self.0.find(q)?;
        self.0.entries[idx].as_ref().map(|(_, v)| v)
    }
    /// Lookup that panics with a PHP-like message on a missing key.
    #[inline]
    pub fn idx<Q: ?Sized + Hash + Eq + fmt::Debug>(&self, q: &Q) -> &V
    where
        K: Borrow<Q>,
    {
        match self.get(q) {
            Some(v) => v,
            None => panic!("Undefined array key {:?}", q),
        }
    }
    #[inline]
    pub fn contains_key<Q: ?Sized + Hash + Eq>(&self, q: &Q) -> bool
    where
        K: Borrow<Q>,
    {
        self.0.find(q).is_some()
    }
    pub fn iter(&self) -> MapIter<'_, K, V> {
        MapIter { entries: &self.0.entries, idx: 0 }
    }
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.iter().map(|(k, _)| k)
    }
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.iter().map(|(_, v)| v)
    }
    /// index of the first live entry at or after `i`
    fn live_from(&self, mut i: usize) -> Option<usize> {
        while i < self.0.entries.len() {
            if self.0.entries[i].is_some() {
                return Some(i);
            }
            i += 1;
        }
        None
    }
    fn entry_at(&self, i: Option<usize>) -> Option<(&K, &V)> {
        i.and_then(|i| self.0.entries[i].as_ref()).map(|(k, v)| (k, v))
    }
    /// `current()`: value at the internal pointer.
    pub fn ptr_current(&self) -> Option<&V> {
        self.entry_at(self.live_from(self.0.pos)).map(|(_, v)| v)
    }
    /// `key()`: key at the internal pointer.
    pub fn ptr_key(&self) -> Option<&K> {
        self.entry_at(self.live_from(self.0.pos)).map(|(k, _)| k)
    }
    pub fn first(&self) -> Option<(&K, &V)> {
        self.iter().next()
    }
    pub fn last(&self) -> Option<(&K, &V)> {
        self.0.entries.iter().rev().flatten().next().map(|(k, v)| (k, v))
    }
    pub fn first_key(&self) -> Option<&K> {
        self.first().map(|(k, _)| k)
    }
    pub fn last_key(&self) -> Option<&K> {
        self.last().map(|(k, _)| k)
    }
    pub fn position_of<Q: ?Sized + Hash + Eq>(&self, q: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
    {
        let idx = self.0.find(q)?;
        Some(self.0.entries[..idx].iter().filter(|e| e.is_some()).count())
    }
    /// True when keys are exactly 0..n-1 in order.
    pub fn is_list(&self) -> bool {
        let mut expect = 0i64;
        for (k, _) in self.iter() {
            match k.int_value() {
                Some(i) if i == expect => expect += 1,
                _ => return false,
            }
        }
        true
    }
}

impl<K: MapKey, V: Clone> Map<K, V> {
    #[inline]
    fn data(&mut self) -> &mut OrderedMap<K, V> {
        Rc::make_mut(&mut self.0)
    }
    pub fn make_mut(&mut self) -> &mut OrderedMap<K, V> {
        self.data()
    }
    /// `next()`: advance the internal pointer and return the value there.
    pub fn ptr_next(&mut self) -> Option<&V> {
        let cur = self.live_from(self.0.pos);
        let next = match cur {
            Some(i) => self.live_from(i + 1),
            None => None,
        };
        self.data().pos = next.unwrap_or(usize::MAX);
        self.ptr_current()
    }
    /// `prev()`
    pub fn ptr_prev(&mut self) -> Option<&V> {
        let mut i = self.live_from(self.0.pos).unwrap_or(0);
        loop {
            if i == 0 {
                self.data().pos = usize::MAX;
                return None;
            }
            i -= 1;
            if self.0.entries[i].is_some() {
                self.data().pos = i;
                return self.ptr_current();
            }
        }
    }
    /// `reset()`
    pub fn ptr_reset(&mut self) -> Option<&V> {
        self.data().pos = 0;
        self.ptr_current()
    }
    /// `end()`
    pub fn ptr_end(&mut self) -> Option<&V> {
        let last = self.0.entries.iter().rposition(|e| e.is_some()).unwrap_or(usize::MAX);
        self.data().pos = last;
        self.ptr_current()
    }
    /// Set a key, keeping its position if it already exists.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let d = self.data();
        if let Some(idx) = d.find(&key) {
            let slot = d.entries[idx].as_mut().unwrap();
            Some(std::mem::replace(&mut slot.1, value))
        } else {
            d.insert_new(key, value);
            None
        }
    }
    pub fn set(&mut self, key: K, value: V) {
        self.insert(key, value);
    }
    /// `$a[] = $v`
    pub fn push(&mut self, value: V) {
        let d = self.data();
        let k = K::from_index(d.next_index);
        d.insert_new(k, value);
    }
    pub fn remove<Q: ?Sized + Hash + Eq>(&mut self, q: &Q) -> Option<V>
    where
        K: Borrow<Q>,
    {
        if self.0.find(q).is_none() {
            return None;
        }
        let d = self.data();
        let idx = d.find(q)?;
        Some(d.remove_at(idx).1)
    }
    pub fn unset<Q: ?Sized + Hash + Eq>(&mut self, q: &Q)
    where
        K: Borrow<Q>,
    {
        self.remove(q);
    }
    #[inline]
    pub fn get_mut<Q: ?Sized + Hash + Eq>(&mut self, q: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
    {
        let idx = self.0.find(q)?;
        self.data().entries[idx].as_mut().map(|(_, v)| v)
    }
    pub fn idx_mut<Q: ?Sized + Hash + Eq + fmt::Debug>(&mut self, q: &Q) -> &mut V
    where
        K: Borrow<Q>,
    {
        match self.0.find(q) {
            Some(idx) => self.data().entries[idx].as_mut().map(|(_, v)| v).unwrap(),
            None => panic!("Undefined array key {:?}", q),
        }
    }
    pub fn entry_or_insert_with<F: FnOnce() -> V>(&mut self, key: K, f: F) -> &mut V {
        let d = self.data();
        let idx = match d.find(&key) {
            Some(i) => i,
            None => d.insert_new(key, f()),
        };
        d.entries[idx].as_mut().map(|(_, v)| v).unwrap()
    }
    pub fn entry_or_default(&mut self, key: K) -> &mut V
    where
        V: Default,
    {
        self.entry_or_insert_with(key, V::default)
    }
    pub fn pop(&mut self) -> Option<V> {
        if self.0.len == 0 {
            return None;
        }
        let d = self.data();
        let idx = d.entries.iter().rposition(|e| e.is_some())?;
        let (_, v) = d.remove_at(idx);
        // array_pop resets the next index
        let mut max = -1i64;
        for e in d.entries.iter().flatten() {
            if let Some(i) = e.0.int_value() {
                if i > max {
                    max = i;
                }
            }
        }
        d.next_index = max + 1;
        Some(v)
    }
    pub fn shift(&mut self) -> Option<V> {
        if self.0.len == 0 {
            return None;
        }
        let old = std::mem::take(self);
        let mut it = old.into_iter();
        let first = it.next().map(|(_, v)| v);
        for (k, v) in it {
            if k.int_value().is_some() {
                self.push(v);
            } else {
                self.insert(k, v);
            }
        }
        first
    }
    pub fn clear(&mut self) {
        *self = Map::new();
    }
    pub fn retain<F: FnMut(&K, &V) -> bool>(&mut self, mut f: F) {
        let d = self.data();
        let mut removed = false;
        for e in d.entries.iter_mut() {
            if let Some((k, v)) = e {
                if !f(k, v) {
                    *e = None;
                    d.len -= 1;
                    removed = true;
                }
            }
        }
        if removed {
            d.compact();
        }
    }
    pub fn into_iter(self) -> std::vec::IntoIter<(K, V)> {
        match Rc::try_unwrap(self.0) {
            Ok(d) => d.entries.into_iter().flatten().collect::<Vec<_>>().into_iter(),
            Err(rc) => rc.entries.iter().flatten().cloned().collect::<Vec<_>>().into_iter(),
        }
    }
    pub fn to_pairs(&self) -> Vec<(K, V)> {
        self.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
    pub fn keys_list(&self) -> List<K> {
        self.keys().cloned().collect()
    }
    pub fn values_list(&self) -> List<V> {
        self.values().cloned().collect()
    }
    pub fn from_pairs<I: IntoIterator<Item = (K, V)>>(it: I) -> Self {
        let mut m = Map::new();
        for (k, v) in it {
            m.insert(k, v);
        }
        m
    }
    /// Sort entries with a comparator; optionally renumber integer keys.
    pub fn sort_by<F: FnMut(&(K, V), &(K, V)) -> std::cmp::Ordering>(&mut self, f: F, renumber: bool) {
        let mut pairs = self.to_pairs();
        pairs.sort_by(f);
        let mut out = Map::with_capacity(pairs.len());
        for (k, v) in pairs {
            if renumber {
                out.push(v);
            } else {
                out.insert(k, v);
            }
        }
        *self = out;
    }
    /// Renumber integer keys from 0, keeping string keys.
    pub fn renumbered(&self) -> Self {
        let mut out = Map::with_capacity(self.len());
        for (k, v) in self.iter() {
            if k.int_value().is_some() {
                out.push(v.clone());
            } else {
                out.insert(k.clone(), v.clone());
            }
        }
        out
    }
    pub fn map_values<U: Clone, F: FnMut(V) -> U>(self, mut f: F) -> Map<K, U> {
        let mut out = Map::with_capacity(self.len());
        for (k, v) in self.into_iter() {
            out.data().insert_new(k, f(v));
        }
        out
    }
    pub fn map_entries<K2: MapKey, U: Clone, F: FnMut(K, V) -> (K2, U)>(self, mut f: F) -> Map<K2, U> {
        let mut out = Map::with_capacity(self.len());
        for (k, v) in self.into_iter() {
            let (k2, v2) = f(k, v);
            out.insert(k2, v2);
        }
        out
    }
    pub fn to_generic(&self) -> Map<ArrayKey, V> {
        let mut out = Map::with_capacity(self.len());
        for (k, v) in self.iter() {
            out.insert(k.to_array_key(), v.clone());
        }
        out
    }
    pub fn from_generic(m: Map<ArrayKey, V>) -> Self {
        let mut out = Map::with_capacity(m.len());
        for (k, v) in m.into_iter() {
            out.insert(K::from_array_key(k), v);
        }
        out
    }
    pub fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, it: I) {
        for (k, v) in it {
            self.insert(k, v);
        }
    }
}

impl<K: MapKey, V: Clone> FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(it: I) -> Self {
        Map::from_pairs(it)
    }
}
impl<K: MapKey, V: Clone, const N: usize> From<[(K, V); N]> for Map<K, V> {
    fn from(arr: [(K, V); N]) -> Self {
        Map::from_pairs(arr)
    }
}
impl<'a, K: MapKey, V> IntoIterator for &'a Map<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = MapIter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<K: MapKey, V: Clone> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;
    fn into_iter(self) -> Self::IntoIter {
        Map::into_iter(self)
    }
}
impl<K: MapKey + fmt::Debug, V: fmt::Debug> fmt::Debug for Map<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
impl<K: MapKey, V: PartialEq> PartialEq for Map<K, V> {
    fn eq(&self, other: &Self) -> bool {
        if Rc::ptr_eq(&self.0, &other.0) {
            return true;
        }
        if self.len() != other.len() {
            return false;
        }
        self.iter().zip(other.iter()).all(|((k1, v1), (k2, v2))| k1 == k2 && v1 == v2)
    }
}

#[macro_export]
macro_rules! map {
    () => { $crate::Map::new() };
    ($($k:expr => $v:expr),+ $(,)?) => { $crate::Map::from_pairs([$(($k, $v)),+]) };
}
