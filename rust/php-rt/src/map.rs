//! Copy-on-write insertion-ordered hash map (PHP `array<K, V>`).

use crate::key::{ArrayKey, KeyQuery, MapKey};
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
    /// PHP references between elements (`$a[$x] = &$a[$y]`): alias key => key of the entry that holds the
    /// shared value. Aliases are resolved by every keyed access and listed after the entries when iterating.
    aliases: Option<Box<Vec<(K, K)>>>,
    /// PHP's packed arrays: no tombstones and entry `i` has the int key `base + i`, so lookups index the
    /// entries directly and no hash table is built (parser stacks, lists read through `array<int, T>`).
    packed: bool,
    base: i64,
}

impl<K: Clone, V: Clone> Clone for OrderedMap<K, V> {
    fn clone(&self) -> Self {
        OrderedMap { entries: self.entries.clone(), table: self.table.clone(), len: self.len, next_index: self.next_index, pos: self.pos, aliases: self.aliases.clone(), packed: self.packed, base: self.base }
    }
}

/// Maps with at most this many entry slots are searched linearly: no hash table is built for them
/// (most PHP arrays are tiny, and hashing plus a table allocation per array dominated the profile).
const SMALL: usize = 8;

impl<K: MapKey, V> OrderedMap<K, V> {
    fn new() -> Self {
        OrderedMap { entries: Vec::new(), table: HashTable::new(), len: 0, next_index: 0, pos: 0, aliases: None, packed: true, base: 0 }
    }
    fn with_capacity(n: usize) -> Self {
        let table = if n > SMALL { HashTable::with_capacity(n) } else { HashTable::new() };
        OrderedMap { entries: Vec::with_capacity(n), table, len: 0, next_index: 0, pos: 0, aliases: None, packed: true, base: 0 }
    }
    /// Whether the hash table is in use (it indexes every live entry once there are more than `SMALL` slots).
    #[inline]
    fn hashed(&self) -> bool {
        !self.packed && self.entries.len() > SMALL
    }
    fn build_table(&mut self) {
        self.table.clear();
        let entries = &self.entries;
        for (idx, e) in entries.iter().enumerate() {
            if let Some((k, _)) = e {
                let h = hash_of(k);
                self.table.insert_unique(h, idx, |&i| hash_of(&entries[i].as_ref().unwrap().0));
            }
        }
    }
    /// The entry key an alias resolves to, if `q` is an alias.
    fn alias_target<Q: ?Sized + Hash + Eq + KeyQuery>(&self, q: &Q) -> Option<&K>
    where
        K: Borrow<Q>,
    {
        let aliases = self.aliases.as_ref()?;
        aliases.iter().find(|(a, _)| a.borrow() == q).map(|(_, c)| c)
    }
    fn find_direct<Q: ?Sized + Hash + Eq + KeyQuery>(&self, q: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
    {
        if self.packed {
            let i = q.packed_index()?.wrapping_sub(self.base);
            return if i >= 0 && (i as usize) < self.entries.len() { Some(i as usize) } else { None };
        }
        if !self.hashed() {
            return self.entries.iter().position(|e| matches!(e, Some((k, _)) if k.borrow() == q));
        }
        let h = hash_of(q);
        self.table.find(h, |&idx| self.key_at(idx).borrow() == q).copied()
    }
    #[inline]
    fn key_at(&self, idx: usize) -> &K {
        match &self.entries[idx] {
            Some((k, _)) => k,
            None => unreachable!(),
        }
    }
    fn find<Q: ?Sized + Hash + Eq + KeyQuery>(&self, q: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
    {
        if let Some(idx) = self.find_direct(q) {
            return Some(idx);
        }
        if let Some(target) = self.alias_target(q) {
            let target = target.clone();
            return self.find_direct::<K>(&target);
        }
        None
    }
    fn insert_new(&mut self, key: K, value: V) -> usize {
        if let Some(i) = key.int_value() {
            if i >= self.next_index {
                self.next_index = i.wrapping_add(1);
            }
        }
        let idx = self.entries.len();
        let was_packed = self.packed;
        if self.packed {
            match key.packed_index() {
                Some(i) if idx == 0 => self.base = i,
                Some(i) if i == self.base.wrapping_add(idx as i64) => {}
                _ => self.packed = false,
            }
        }
        self.entries.push(Some((key, value)));
        self.len += 1;
        if self.hashed() {
            if was_packed || idx == SMALL {
                // just left the packed mode or crossed the size threshold: index every entry
                self.build_table();
            } else {
                let entries = &self.entries;
                let h = hash_of(&entries[idx].as_ref().unwrap().0);
                self.table.insert_unique(h, idx, |&i| match &entries[i] {
                    Some((k, _)) => hash_of(k),
                    None => unreachable!(),
                });
            }
        }
        idx
    }
    fn remove_at(&mut self, idx: usize) -> (K, V) {
        if self.packed && idx + 1 == self.entries.len() {
            // popping the last entry keeps the array packed
            let (k, v) = self.entries.pop().unwrap().unwrap();
            self.len -= 1;
            return (k, v);
        }
        let was_packed = self.packed;
        self.packed = false;
        let (k, v) = self.entries[idx].take().unwrap();
        self.len -= 1;
        if was_packed {
            if self.hashed() {
                self.build_table();
            }
        } else if self.hashed() {
            let h = hash_of(&k);
            if let Ok(e) = self.table.find_entry(h, |&i| i == idx) {
                e.remove();
            }
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
        self.packed = self.entries.first().map_or(true, |e| e.as_ref().unwrap().0.packed_index().is_some());
        if self.packed {
            if let Some(e) = self.entries.first() {
                self.base = e.as_ref().unwrap().0.packed_index().unwrap();
                let base = self.base;
                self.packed = self.entries.iter().enumerate().all(|(i, e)| e.as_ref().unwrap().0.packed_index() == Some(base.wrapping_add(i as i64)));
            }
        }
        if self.hashed() {
            self.build_table();
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
    map: &'a OrderedMap<K, V>,
    idx: usize,
    alias_idx: usize,
}
impl<'a, K: MapKey, V> Iterator for MapIter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        let entries = &self.map.entries;
        while self.idx < entries.len() {
            let i = self.idx;
            self.idx += 1;
            if let Some((k, v)) = &entries[i] {
                return Some((k, v));
            }
        }
        // aliased keys share the value of the entry they reference
        if let Some(aliases) = &self.map.aliases {
            while self.alias_idx < aliases.len() {
                let (a, c) = &aliases[self.alias_idx];
                self.alias_idx += 1;
                if let Some(idx) = self.map.find_direct(c) {
                    if let Some((_, v)) = &entries[idx] {
                        return Some((a, v));
                    }
                }
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
        self.0.len + self.0.aliases.as_ref().map_or(0, |a| a.len())
    }
    #[inline]
    pub fn count(&self) -> i64 {
        self.len() as i64
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// `$a[$alias] = &$a[$target]`: both keys share one value slot from now on.
    pub fn alias(&mut self, alias: K, target: K)
    where
        V: Clone,
    {
        let d = self.data();
        let target = match d.alias_target(&target) {
            Some(c) => c.clone(),
            None => target,
        };
        if d.find_direct(&target).is_none() {
            // nothing to share yet (PHP would create a null entry); the alias is dropped
            return;
        }
        if alias == target {
            return;
        }
        if let Some(idx) = d.find_direct(&alias) {
            d.remove_at(idx);
        }
        let aliases = d.aliases.get_or_insert_with(|| Box::new(Vec::new()));
        aliases.retain(|(a, _)| *a != alias);
        aliases.push((alias, target));
    }
    #[inline]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
    pub fn next_index(&self) -> i64 {
        self.0.next_index
    }
    #[inline]
    pub fn get<Q: ?Sized + Hash + Eq + KeyQuery>(&self, q: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        let idx = self.0.find(q)?;
        self.0.entries[idx].as_ref().map(|(_, v)| v)
    }
    /// Lookup that panics with a PHP-like message on a missing key.
    #[inline]
    pub fn idx<Q: ?Sized + Hash + Eq + KeyQuery + fmt::Debug>(&self, q: &Q) -> &V
    where
        K: Borrow<Q>,
    {
        match self.get(q) {
            Some(v) => v,
            None => panic!("Undefined array key {:?}", q),
        }
    }
    #[inline]
    pub fn contains_key<Q: ?Sized + Hash + Eq + KeyQuery>(&self, q: &Q) -> bool
    where
        K: Borrow<Q>,
    {
        self.0.find(q).is_some()
    }
    pub fn iter(&self) -> MapIter<'_, K, V> {
        MapIter { map: &self.0, idx: 0, alias_idx: 0 }
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
    pub fn position_of<Q: ?Sized + Hash + Eq + KeyQuery>(&self, q: &Q) -> Option<usize>
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
    pub fn remove<Q: ?Sized + Hash + Eq + KeyQuery>(&mut self, q: &Q) -> Option<V>
    where
        K: Borrow<Q>,
    {
        if self.0.find(q).is_none() {
            return None;
        }
        let d = self.data();
        if d.find_direct(q).is_none() {
            // unsetting an alias only drops that name; the shared value stays under the other names
            let aliases = d.aliases.as_mut()?;
            let pos = aliases.iter().position(|(a, _)| a.borrow() == q)?;
            aliases.remove(pos);
            return None;
        }
        let idx = d.find_direct(q)?;
        let (key, value) = d.remove_at(idx);
        // other names still refer to the value: the first alias becomes the entry
        let promote = d.aliases.as_ref().and_then(|al| al.iter().position(|(_, c)| *c == key));
        if let Some(pos) = promote {
            let aliases = d.aliases.as_mut().unwrap();
            let (new_key, _) = aliases.remove(pos);
            for (_, c) in aliases.iter_mut() {
                if *c == key {
                    *c = new_key.clone();
                }
            }
            let v = value;
            d.insert_new(new_key, v);
            return None;
        }
        Some(value)
    }
    pub fn unset<Q: ?Sized + Hash + Eq + KeyQuery>(&mut self, q: &Q)
    where
        K: Borrow<Q>,
    {
        self.remove(q);
    }
    #[inline]
    pub fn get_mut<Q: ?Sized + Hash + Eq + KeyQuery>(&mut self, q: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
    {
        let idx = self.0.find(q)?;
        self.data().entries[idx].as_mut().map(|(_, v)| v)
    }
    pub fn idx_mut<Q: ?Sized + Hash + Eq + KeyQuery + fmt::Debug>(&mut self, q: &Q) -> &mut V
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
    /// `array_unshift`: prepend a value; integer keys are renumbered.
    pub fn unshift(&mut self, value: V) {
        let old = std::mem::take(self);
        let mut out: Map<K, V> = Map::new();
        out.push(value);
        for (k, v) in old.into_iter() {
            if k.int_value().is_some() {
                out.push(v);
            } else {
                out.insert(k, v);
            }
        }
        *self = out;
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
        if self.0.aliases.is_some() {
            return self.to_pairs().into_iter();
        }
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

#[cfg(test)]
mod small_map_tests {
    use super::Map;
    use crate::key::ArrayKey;

    #[test]
    fn small_and_hashed_modes_agree() {
        let mut m: Map<ArrayKey, i64> = Map::new();
        for i in 0..40 {
            m.insert(ArrayKey::Int(i), i * 10);
            assert_eq!(m.get(&ArrayKey::Int(i)), Some(&(i * 10)));
            assert_eq!(m.len(), (i + 1) as usize);
        }
        // remove most entries: compaction drops back into the linear mode
        for i in 0..36 {
            assert_eq!(m.remove(&ArrayKey::Int(i)), Some(i * 10));
            assert_eq!(m.get(&ArrayKey::Int(i)), None);
        }
        assert_eq!(m.len(), 4);
        assert_eq!(m.keys().map(|k| k.clone()).collect::<Vec<_>>(), (36..40).map(ArrayKey::Int).collect::<Vec<_>>());
        for i in 36..40 {
            assert_eq!(m.get(&ArrayKey::Int(i)), Some(&(i * 10)));
        }
        // grow again across the threshold with string keys
        for i in 0..20 {
            m.insert(ArrayKey::Str(crate::string::Str::from_string(format!("k{i}"))), i);
        }
        assert_eq!(m.get(&ArrayKey::Str(crate::string::Str::from_static("k7"))), Some(&7));
        assert_eq!(m.get(&ArrayKey::Int(38)), Some(&380));
        assert!(!m.contains_key(&ArrayKey::Int(3)));
        // overwrite keeps position and count
        m.insert(ArrayKey::Int(38), 1);
        assert_eq!(m.len(), 24);
        assert_eq!(m.get(&ArrayKey::Int(38)), Some(&1));
        // packed 1-based stack: writes at len+1 stay packed, popping the last keeps it packed
        let mut st: Map<i64, i64> = Map::new();
        for i in 1..=30 { st.insert(i, i * 2); }
        assert!(st.0.packed && st.0.base == 1);
        assert_eq!(st.get(&30), Some(&60));
        assert_eq!(st.get(&0), None);
        assert_eq!(st.remove(&30), Some(60));
        assert!(st.0.packed);
        assert_eq!(st.get(&30), None);
        st.insert(30, 7);
        assert!(st.0.packed);
        assert_eq!(st.get(&30), Some(&7));
        // removing a middle entry leaves the packed mode; lookups stay correct in both modes
        assert_eq!(st.remove(&5), Some(10));
        assert!(!st.0.packed);
        assert_eq!(st.get(&4), Some(&8));
        assert_eq!(st.get(&6), Some(&12));
        assert_eq!(st.get(&5), None);
        st.insert(5, 99);
        assert_eq!(st.get(&5), Some(&99));
        assert_eq!(st.keys().last(), Some(&5));
        assert_eq!(st.len(), 30);
        // a small non-sequential int map
        let mut sp: Map<i64, i64> = Map::new();
        sp.insert(10, 1); sp.insert(3, 2); sp.insert(11, 3);
        assert!(!sp.0.packed);
        assert_eq!(sp.get(&3), Some(&2));
        assert_eq!(sp.get(&11), Some(&3));
        assert_eq!(sp.keys().cloned().collect::<Vec<_>>(), vec![10, 3, 11]);
        // push after removals appends with the next index
        let mut p: Map<ArrayKey, i64> = Map::new();
        for i in 0..3 { p.push(i); }
        p.remove(&ArrayKey::Int(1));
        p.push(9);
        assert_eq!(p.keys().map(|k| k.clone()).collect::<Vec<_>>(), vec![ArrayKey::Int(0), ArrayKey::Int(2), ArrayKey::Int(3)]);
    }
}
