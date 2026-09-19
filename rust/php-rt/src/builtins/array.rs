//! Array functions, generic over List<T> and Map<K, V>. `_l` suffix: list argument; `_m`: map argument.

use crate::error::RtError;
use crate::key::{ArrayKey, MapKey};
use crate::list::List;
use crate::map::Map;
use crate::mixed::Mixed;
use crate::string::Str;
use crate::traits::{Identical, PhpCmp, ToStr, Truthy};
use std::cmp::Ordering;

pub fn array_keys_l<T>(l: &List<T>) -> List<i64> {
    (0..l.len() as i64).collect()
}
pub fn array_keys_m<K: MapKey, V: Clone>(m: &Map<K, V>) -> List<K> {
    m.keys_list()
}
pub fn array_keys_search_m<K: MapKey, V: Clone + Identical>(m: &Map<K, V>, needle: &V) -> List<K> {
    m.iter().filter(|(_, v)| v.identical(needle)).map(|(k, _)| k.clone()).collect()
}
pub fn array_values_l<T: Clone>(l: &List<T>) -> List<T> {
    l.clone()
}
pub fn array_values_m<K: MapKey, V: Clone>(m: &Map<K, V>) -> List<V> {
    m.values_list()
}
pub fn in_array_l<T: Identical>(needle: &T, hay: &List<T>) -> bool {
    hay.iter().any(|v| v.identical(needle))
}
pub fn in_array_m<K: MapKey, V: Identical>(needle: &V, hay: &Map<K, V>) -> bool {
    hay.values().any(|v| v.identical(needle))
}
pub fn in_array_loose_l<T: PhpCmp>(needle: &T, hay: &List<T>) -> bool {
    hay.iter().any(|v| v.loose_eq(needle))
}
pub fn in_array_loose_m<K: MapKey, V: PhpCmp>(needle: &V, hay: &Map<K, V>) -> bool {
    hay.values().any(|v| v.loose_eq(needle))
}
pub fn array_search_l<T: Identical>(needle: &T, hay: &List<T>) -> Option<i64> {
    hay.iter().position(|v| v.identical(needle)).map(|p| p as i64)
}
pub fn array_search_m<K: MapKey, V: Identical>(needle: &V, hay: &Map<K, V>) -> Option<K> {
    hay.iter().find(|(_, v)| v.identical(needle)).map(|(k, _)| k.clone())
}
pub fn array_merge_l<T: Clone>(parts: &[&List<T>]) -> List<T> {
    let mut out = Vec::new();
    for p in parts {
        out.extend(p.iter().cloned());
    }
    List::from_vec(out)
}
/// array_merge for maps: string keys overwrite, int keys renumber.
pub fn array_merge_m<K: MapKey, V: Clone>(parts: &[&Map<K, V>]) -> Map<K, V> {
    let mut out = Map::new();
    for p in parts {
        for (k, v) in p.iter() {
            if k.int_value().is_some() {
                out.push(v.clone());
            } else {
                out.insert(k.clone(), v.clone());
            }
        }
    }
    out
}
/// `$a + $b` on arrays: keep left entries, add missing right entries.
pub fn array_union<K: MapKey, V: Clone>(a: &Map<K, V>, b: &Map<K, V>) -> Map<K, V> {
    let mut out = a.clone();
    for (k, v) in b.iter() {
        if !out.contains_key(k) {
            out.insert(k.clone(), v.clone());
        }
    }
    out
}
/// `array_replace_recursive` over dynamic arrays.
pub fn array_replace_recursive(a: &Mixed, b: &Mixed) -> Mixed {
    match (a, b) {
        (Mixed::Arr(x), Mixed::Arr(y)) => {
            let mut out = x.clone();
            for (k, v) in y.iter() {
                let merged = match out.get(k) {
                    Some(Mixed::Arr(_)) if matches!(v, Mixed::Arr(_)) => array_replace_recursive(out.get(k).unwrap(), v),
                    _ => v.clone(),
                };
                out.insert(k.clone(), merged);
            }
            Mixed::Arr(out)
        }
        _ => b.clone(),
    }
}
pub fn array_replace_m<K: MapKey, V: Clone>(a: &Map<K, V>, b: &Map<K, V>) -> Map<K, V> {
    let mut out = a.clone();
    for (k, v) in b.iter() {
        out.insert(k.clone(), v.clone());
    }
    out
}
pub fn array_unique_l<T: Clone + PhpCmp>(l: &List<T>) -> Map<i64, T> {
    let mut out: Map<i64, T> = Map::new();
    let mut seen: Vec<T> = Vec::new();
    for (i, v) in l.iter().enumerate() {
        if !seen.iter().any(|s| s.loose_eq(v)) {
            seen.push(v.clone());
            out.insert(i as i64, v.clone());
        }
    }
    out
}
pub fn array_unique_m<K: MapKey, V: Clone + PhpCmp>(m: &Map<K, V>) -> Map<K, V> {
    let mut out: Map<K, V> = Map::new();
    let mut seen: Vec<V> = Vec::new();
    for (k, v) in m.iter() {
        if !seen.iter().any(|s| s.loose_eq(v)) {
            seen.push(v.clone());
            out.insert(k.clone(), v.clone());
        }
    }
    out
}
pub fn array_unique_str<K: MapKey>(m: &Map<K, Str>) -> Map<K, Str> {
    let mut out: Map<K, Str> = Map::new();
    let mut seen: hashbrown::HashSet<Str> = hashbrown::HashSet::new();
    for (k, v) in m.iter() {
        if seen.insert(v.clone()) {
            out.insert(k.clone(), v.clone());
        }
    }
    out
}
pub fn array_unique_str_l(l: &List<Str>) -> Map<i64, Str> {
    let mut out: Map<i64, Str> = Map::new();
    let mut seen: hashbrown::HashSet<Str> = hashbrown::HashSet::new();
    for (i, v) in l.iter().enumerate() {
        if seen.insert(v.clone()) {
            out.insert(i as i64, v.clone());
        }
    }
    out
}
pub fn array_reverse_l<T: Clone>(l: &List<T>) -> List<T> {
    let mut v = l.to_vec();
    v.reverse();
    List::from_vec(v)
}
pub fn array_reverse_m<K: MapKey, V: Clone>(m: &Map<K, V>, preserve: bool) -> Map<K, V> {
    let mut pairs = m.to_pairs();
    pairs.reverse();
    let mut out = Map::new();
    for (k, v) in pairs {
        if !preserve && k.int_value().is_some() {
            out.push(v);
        } else {
            out.insert(k, v);
        }
    }
    out
}
fn slice_bounds(len: usize, offset: i64, length: Option<i64>) -> (usize, usize) {
    let len_i = len as i64;
    let start = if offset < 0 { (len_i + offset).max(0) } else { offset.min(len_i) };
    let end = match length {
        None => len_i,
        Some(l) if l < 0 => (len_i + l).max(start),
        Some(l) => (start + l).min(len_i),
    };
    (start as usize, end.max(start) as usize)
}
pub fn array_slice_l<T: Clone>(l: &List<T>, offset: i64, length: Option<i64>) -> List<T> {
    let (a, b) = slice_bounds(l.len(), offset, length);
    l.slice(a, b)
}
pub fn array_slice_m<K: MapKey, V: Clone>(m: &Map<K, V>, offset: i64, length: Option<i64>, preserve: bool) -> Map<K, V> {
    let (a, b) = slice_bounds(m.len(), offset, length);
    let mut out = Map::new();
    for (k, v) in m.iter().skip(a).take(b - a) {
        if !preserve && k.int_value().is_some() {
            out.push(v.clone());
        } else {
            out.insert(k.clone(), v.clone());
        }
    }
    out
}
pub fn array_splice_l<T: Clone>(l: &mut List<T>, offset: i64, length: Option<i64>, repl: Vec<T>) -> List<T> {
    let (a, b) = slice_bounds(l.len(), offset, length);
    l.splice_replace(a, b - a, repl)
}
/// The extracted elements keep their string keys, as PHP's does; only integer keys are renumbered.
pub fn array_splice_m<K: MapKey, V: Clone>(m: &mut Map<K, V>, offset: i64, length: Option<i64>, repl: Vec<V>) -> Map<K, V> {
    let (a, b) = slice_bounds(m.len(), offset, length);
    let pairs = m.to_pairs();
    let mut out = Map::new();
    let mut removed: Map<K, V> = Map::new();
    for (i, (k, v)) in pairs.into_iter().enumerate() {
        if i == a {
            for r in repl.iter() {
                out.push(r.clone());
            }
        }
        if i >= a && i < b {
            if k.int_value().is_some() {
                removed.push(v);
            } else {
                removed.insert(k, v);
            }
        } else if k.int_value().is_some() {
            out.push(v);
        } else {
            out.insert(k, v);
        }
    }
    if a >= m.len() {
        for r in repl {
            out.push(r);
        }
    }
    *m = out;
    removed
}
pub fn array_map_l<T: Clone, U, F: FnMut(T) -> U>(l: &List<T>, mut f: F) -> List<U> {
    let mut out = Vec::with_capacity(l.len());
    for v in l.iter() {
        out.push(f(v.clone()));
    }
    List::from_vec(out)
}
pub fn array_map_m<K: MapKey, V: Clone, U: Clone, F: FnMut(V) -> U>(m: &Map<K, V>, mut f: F) -> Map<K, U> {
    let mut out = Map::with_capacity(m.len());
    for (k, v) in m.iter() {
        out.insert(k.clone(), f(v.clone()));
    }
    out
}
pub fn array_map2_l<A: Clone, B: Clone, U, F: FnMut(A, B) -> U>(a: &List<A>, b: &List<B>, mut f: F) -> List<U> {
    let n = a.len().max(b.len());
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let x = a.get(i as i64).cloned().expect("array_map: arrays of different lengths");
        let y = b.get(i as i64).cloned().expect("array_map: arrays of different lengths");
        out.push(f(x, y));
    }
    List::from_vec(out)
}
pub fn array_filter_l<T: Clone + Truthy>(l: &List<T>) -> Map<i64, T> {
    l.iter().enumerate().filter(|(_, v)| v.truthy()).map(|(i, v)| (i as i64, v.clone())).collect()
}
pub fn array_filter_m<K: MapKey, V: Clone + Truthy>(m: &Map<K, V>) -> Map<K, V> {
    m.iter().filter(|(_, v)| v.truthy()).map(|(k, v)| (k.clone(), v.clone())).collect()
}
pub fn array_filter_cb_l<T: Clone, F: FnMut(T) -> bool>(l: &List<T>, mut f: F) -> Map<i64, T> {
    let mut out = Map::new();
    for (i, v) in l.iter().enumerate() {
        if f(v.clone()) {
            out.insert(i as i64, v.clone());
        }
    }
    out
}
pub fn array_filter_cb_m<K: MapKey, V: Clone, F: FnMut(V) -> bool>(m: &Map<K, V>, mut f: F) -> Map<K, V> {
    let mut out = Map::new();
    for (k, v) in m.iter() {
        if f(v.clone()) {
            out.insert(k.clone(), v.clone());
        }
    }
    out
}
pub fn array_filter_key_m<K: MapKey, V: Clone, F: FnMut(K) -> bool>(m: &Map<K, V>, mut f: F) -> Map<K, V> {
    let mut out = Map::new();
    for (k, v) in m.iter() {
        if f(k.clone()) {
            out.insert(k.clone(), v.clone());
        }
    }
    out
}
pub fn array_filter_both_m<K: MapKey, V: Clone, F: FnMut(V, K) -> bool>(m: &Map<K, V>, mut f: F) -> Map<K, V> {
    let mut out = Map::new();
    for (k, v) in m.iter() {
        if f(v.clone(), k.clone()) {
            out.insert(k.clone(), v.clone());
        }
    }
    out
}
pub fn array_filter_key_l<T: Clone, F: FnMut(i64) -> bool>(l: &List<T>, mut f: F) -> Map<i64, T> {
    let mut out = Map::new();
    for (i, v) in l.iter().enumerate() {
        if f(i as i64) {
            out.insert(i as i64, v.clone());
        }
    }
    out
}
pub fn array_filter_both_l<T: Clone, F: FnMut(T, i64) -> bool>(l: &List<T>, mut f: F) -> Map<i64, T> {
    let mut out = Map::new();
    for (i, v) in l.iter().enumerate() {
        if f(v.clone(), i as i64) {
            out.insert(i as i64, v.clone());
        }
    }
    out
}
pub fn array_reduce_l<T: Clone, A, F: FnMut(A, T) -> A>(l: &List<T>, init: A, mut f: F) -> A {
    let mut acc = init;
    for v in l.iter() {
        acc = f(acc, v.clone());
    }
    acc
}
pub fn array_reduce_m<K: MapKey, V: Clone, A, F: FnMut(A, V) -> A>(m: &Map<K, V>, init: A, mut f: F) -> A {
    let mut acc = init;
    for (_, v) in m.iter() {
        acc = f(acc, v.clone());
    }
    acc
}
pub fn array_walk_l<T: Clone, F: FnMut(&mut T)>(l: &mut List<T>, mut f: F) {
    for v in l.make_mut().iter_mut() {
        f(v);
    }
}
pub fn array_sum_i(l: &[i64]) -> i64 {
    l.iter().fold(0i64, |a, b| a.wrapping_add(*b))
}
pub fn array_sum_f(l: &[f64]) -> f64 {
    l.iter().sum()
}
pub fn array_sum_mi<K: MapKey>(m: &Map<K, i64>) -> i64 {
    m.values().fold(0i64, |a, b| a.wrapping_add(*b))
}
pub fn array_flip_l(l: &List<Str>) -> Map<Str, i64> {
    l.iter().enumerate().map(|(i, v)| (v.clone(), i as i64)).collect()
}
pub fn array_flip_m<K: MapKey + Clone, V: MapKey>(m: &Map<K, V>) -> Map<V, K> {
    m.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
}
pub fn array_fill<T: Clone>(start: i64, n: i64, v: T) -> Map<i64, T> {
    (0..n.max(0)).map(|i| (start + i, v.clone())).collect()
}
pub fn array_fill_l<T: Clone>(n: i64, v: T) -> List<T> {
    List::from_vec(vec![v; n.max(0) as usize])
}
pub fn array_fill_keys<K: MapKey, V: Clone>(keys: &[K], v: V) -> Map<K, V> {
    keys.iter().map(|k| (k.clone(), v.clone())).collect()
}
pub fn array_fill_keys_m<K0: MapKey, K: MapKey, V: Clone>(keys: &Map<K0, K>, v: V) -> Map<K, V> {
    keys.values().map(|k| (k.clone(), v.clone())).collect()
}
pub fn array_combine<K: MapKey, V: Clone>(keys: &[K], values: &[V]) -> Result<Map<K, V>, RtError> {
    if keys.len() != values.len() {
        return Err(RtError::value_error("array_combine(): Argument #1 ($keys) and argument #2 ($values) must have the same number of elements"));
    }
    Ok(keys.iter().cloned().zip(values.iter().cloned()).collect())
}
pub fn array_diff_l<T: Clone + PhpCmp>(a: &List<T>, b: &List<T>) -> Map<i64, T> {
    a.iter().enumerate().filter(|(_, v)| !b.iter().any(|w| w.loose_eq(v))).map(|(i, v)| (i as i64, v.clone())).collect()
}
pub fn array_diff_m<K: MapKey, V: Clone + PhpCmp, K2: MapKey>(a: &Map<K, V>, b: &Map<K2, V>) -> Map<K, V> {
    a.iter().filter(|(_, v)| !b.values().any(|w| w.loose_eq(v))).map(|(k, v)| (k.clone(), v.clone())).collect()
}
pub fn array_diff_str_l(a: &List<Str>, b: &List<Str>) -> Map<i64, Str> {
    let set: hashbrown::HashSet<&Str> = b.iter().collect();
    a.iter().enumerate().filter(|(_, v)| !set.contains(v)).map(|(i, v)| (i as i64, v.clone())).collect()
}
pub fn array_diff_key<K: MapKey, V: Clone, V2>(a: &Map<K, V>, b: &Map<K, V2>) -> Map<K, V> {
    a.iter().filter(|(k, _)| !b.contains_key(k)).map(|(k, v)| (k.clone(), v.clone())).collect()
}
pub fn array_diff_key_l<T: Clone, V2>(a: &List<T>, b: &Map<i64, V2>) -> Map<i64, T> {
    a.iter().enumerate().filter(|(i, _)| !b.contains_key(&(*i as i64))).map(|(i, v)| (i as i64, v.clone())).collect()
}
pub fn array_diff_ukey<K: MapKey, V: Clone, V2, F: FnMut(K, K) -> i64>(a: &Map<K, V>, b: &Map<K, V2>, mut f: F) -> Map<K, V> {
    let mut out = Map::new();
    for (k, v) in a.iter() {
        let mut found = false;
        for (k2, _) in b.iter() {
            if f(k.clone(), k2.clone()) == 0 {
                found = true;
                break;
            }
        }
        if !found {
            out.insert(k.clone(), v.clone());
        }
    }
    out
}
pub fn array_intersect_key<K: MapKey, V: Clone, V2>(a: &Map<K, V>, b: &Map<K, V2>) -> Map<K, V> {
    a.iter().filter(|(k, _)| b.contains_key(k)).map(|(k, v)| (k.clone(), v.clone())).collect()
}
pub fn array_intersect_l<T: Clone + PhpCmp>(a: &List<T>, b: &List<T>) -> Map<i64, T> {
    a.iter().enumerate().filter(|(_, v)| b.iter().any(|w| w.loose_eq(v))).map(|(i, v)| (i as i64, v.clone())).collect()
}
pub fn array_intersect_m<K: MapKey, V: Clone + PhpCmp, K2: MapKey>(a: &Map<K, V>, b: &Map<K2, V>) -> Map<K, V> {
    a.iter().filter(|(_, v)| b.values().any(|w| w.loose_eq(v))).map(|(k, v)| (k.clone(), v.clone())).collect()
}
pub fn array_key_first_m<K: MapKey, V>(m: &Map<K, V>) -> Option<K> {
    m.first_key().cloned()
}
pub fn array_key_last_m<K: MapKey, V>(m: &Map<K, V>) -> Option<K> {
    m.last_key().cloned()
}
pub fn array_key_first_l<T>(l: &List<T>) -> Option<i64> {
    if l.is_empty() { None } else { Some(0) }
}
pub fn array_key_last_l<T>(l: &List<T>) -> Option<i64> {
    if l.is_empty() { None } else { Some(l.len() as i64 - 1) }
}
pub fn array_column<K: MapKey, V: Clone, F: Fn(&V) -> Option<U>, U>(m: &Map<K, V>, f: F) -> List<U> {
    m.values().filter_map(f).collect()
}
pub fn array_chunk_l<T: Clone>(l: &List<T>, size: i64) -> List<List<T>> {
    let size = size.max(1) as usize;
    l.as_slice().chunks(size).map(|c| List::from_vec(c.to_vec())).collect()
}
pub fn array_pad_l<T: Clone>(l: &List<T>, size: i64, v: T) -> List<T> {
    let mut out = l.to_vec();
    let n = size.unsigned_abs() as usize;
    if n <= out.len() {
        return l.clone();
    }
    let extra = n - out.len();
    if size > 0 {
        out.extend(std::iter::repeat(v).take(extra));
    } else {
        let mut front: Vec<T> = std::iter::repeat(v).take(extra).collect();
        front.extend(out);
        out = front;
    }
    List::from_vec(out)
}
pub fn range_i(a: i64, b: i64, step: i64) -> List<i64> {
    let step = step.abs().max(1);
    if a <= b {
        (a..=b).step_by(step as usize).collect()
    } else {
        let mut v = Vec::new();
        let mut x = a;
        while x >= b {
            v.push(x);
            x -= step;
        }
        List::from_vec(v)
    }
}
pub fn range_c(a: u8, b: u8) -> List<Str> {
    if a <= b {
        (a..=b).map(|c| Str::from_vec(vec![c])).collect()
    } else {
        (b..=a).rev().map(|c| Str::from_vec(vec![c])).collect()
    }
}
pub fn array_is_list_m<K: MapKey, V>(m: &Map<K, V>) -> bool {
    m.is_list()
}
pub fn array_key_exists_m<K: MapKey, V, Q: ?Sized + std::hash::Hash + Eq + crate::key::KeyQuery>(k: &Q, m: &Map<K, V>) -> bool
where
    K: std::borrow::Borrow<Q>,
{
    m.contains_key(k)
}
pub fn array_key_exists_l<T>(k: i64, l: &List<T>) -> bool {
    l.has(k)
}
pub fn array_any_l<T: Clone, F: FnMut(T, i64) -> bool>(l: &List<T>, mut f: F) -> bool {
    for (i, v) in l.iter().enumerate() {
        if f(v.clone(), i as i64) {
            return true;
        }
    }
    false
}
pub fn array_any_m<K: MapKey, V: Clone, F: FnMut(V, K) -> bool>(m: &Map<K, V>, mut f: F) -> bool {
    for (k, v) in m.iter() {
        if f(v.clone(), k.clone()) {
            return true;
        }
    }
    false
}
pub fn array_all_l<T: Clone, F: FnMut(T, i64) -> bool>(l: &List<T>, mut f: F) -> bool {
    for (i, v) in l.iter().enumerate() {
        if !f(v.clone(), i as i64) {
            return false;
        }
    }
    true
}
pub fn array_all_m<K: MapKey, V: Clone, F: FnMut(V, K) -> bool>(m: &Map<K, V>, mut f: F) -> bool {
    for (k, v) in m.iter() {
        if !f(v.clone(), k.clone()) {
            return false;
        }
    }
    true
}
pub fn array_find_l<T: Clone, F: FnMut(T, i64) -> bool>(l: &List<T>, mut f: F) -> Option<T> {
    for (i, v) in l.iter().enumerate() {
        if f(v.clone(), i as i64) {
            return Some(v.clone());
        }
    }
    None
}
pub fn array_find_m<K: MapKey, V: Clone, F: FnMut(V, K) -> bool>(m: &Map<K, V>, mut f: F) -> Option<V> {
    for (k, v) in m.iter() {
        if f(v.clone(), k.clone()) {
            return Some(v.clone());
        }
    }
    None
}
pub fn array_find_key_m<K: MapKey, V: Clone, F: FnMut(V, K) -> bool>(m: &Map<K, V>, mut f: F) -> Option<K> {
    for (k, v) in m.iter() {
        if f(v.clone(), k.clone()) {
            return Some(k.clone());
        }
    }
    None
}
pub fn array_change_key_case_lower<V: Clone>(m: &Map<Str, V>) -> Map<Str, V> {
    m.iter().map(|(k, v)| (k.to_lowercase(), v.clone())).collect()
}
pub fn array_count_values<K: MapKey, V: MapKey>(m: &Map<K, V>) -> Map<V, i64> {
    let mut out: Map<V, i64> = Map::new();
    for (_, v) in m.iter() {
        *out.entry_or_default(v.clone()) += 1;
    }
    out
}
pub fn array_count_values_l<V: MapKey>(l: &List<V>) -> Map<V, i64> {
    let mut out: Map<V, i64> = Map::new();
    for v in l.iter() {
        *out.entry_or_default(v.clone()) += 1;
    }
    out
}
pub fn array_product_i(l: &[i64]) -> i64 {
    l.iter().fold(1i64, |a, b| a.wrapping_mul(*b))
}

// ---------------------------------------------------------------- sorting

pub fn sort_l<T: Clone + PhpCmp>(l: &mut List<T>) {
    l.sort_by(|a, b| a.php_cmp(b));
}
pub fn rsort_l<T: Clone + PhpCmp>(l: &mut List<T>) {
    l.sort_by(|a, b| b.php_cmp(a));
}
/// sort() on a map: values sorted, keys renumbered => list.
pub fn sort_m<K: MapKey, V: Clone + PhpCmp>(m: &Map<K, V>) -> List<V> {
    let mut v = m.values_list();
    sort_l(&mut v);
    v
}
pub fn rsort_m<K: MapKey, V: Clone + PhpCmp>(m: &Map<K, V>) -> List<V> {
    let mut v = m.values_list();
    rsort_l(&mut v);
    v
}
pub fn usort_l<T: Clone, F: FnMut(T, T) -> i64>(l: &mut List<T>, mut f: F) {
    l.sort_by(|a, b| f(a.clone(), b.clone()).cmp(&0));
}
pub fn usort_m<K: MapKey, V: Clone, F: FnMut(V, V) -> i64>(m: &Map<K, V>, f: F) -> List<V> {
    let mut v = m.values_list();
    usort_l(&mut v, f);
    v
}
pub fn uasort_m<K: MapKey, V: Clone, F: FnMut(V, V) -> i64>(m: &mut Map<K, V>, mut f: F) {
    m.sort_by(|a, b| f(a.1.clone(), b.1.clone()).cmp(&0), false);
}
pub fn uksort_m<K: MapKey, V: Clone, F: FnMut(K, K) -> i64>(m: &mut Map<K, V>, mut f: F) {
    m.sort_by(|a, b| f(a.0.clone(), b.0.clone()).cmp(&0), false);
}
pub fn ksort_m<K: MapKey + PhpCmp, V: Clone>(m: &mut Map<K, V>) {
    m.sort_by(|a, b| a.0.php_cmp(&b.0), false);
}
pub fn krsort_m<K: MapKey + PhpCmp, V: Clone>(m: &mut Map<K, V>) {
    m.sort_by(|a, b| b.0.php_cmp(&a.0), false);
}
pub fn asort_m<K: MapKey, V: Clone + PhpCmp>(m: &mut Map<K, V>) {
    m.sort_by(|a, b| a.1.php_cmp(&b.1), false);
}
pub fn arsort_m<K: MapKey, V: Clone + PhpCmp>(m: &mut Map<K, V>) {
    m.sort_by(|a, b| b.1.php_cmp(&a.1), false);
}
pub fn ksort_flag_string<K: MapKey, V: Clone>(m: &mut Map<K, V>) {
    m.sort_by(|a, b| a.0.to_str_key().as_bytes().cmp(b.0.to_str_key().as_bytes()), false);
}
pub fn sort_flag_string_l(l: &mut List<Str>) {
    l.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
}
pub fn array_multisort_l<T: Clone + PhpCmp>(l: &mut List<T>) {
    sort_l(l)
}
pub fn shuffle_l<T: Clone>(l: &mut List<T>) {
    let v = l.make_mut();
    let n = v.len();
    for i in (1..n).rev() {
        let j = (crate::builtins::math::mt_rand(0, i as i64)) as usize;
        v.swap(i, j);
    }
}
pub fn max_l<T: Clone + PhpCmp>(l: &List<T>) -> Result<T, RtError> {
    let mut it = l.iter();
    let mut best = match it.next() {
        Some(v) => v.clone(),
        None => return Err(RtError::value_error("max(): Argument #1 ($value) must contain at least one element")),
    };
    for v in it {
        if v.php_cmp(&best) == Ordering::Greater {
            best = v.clone();
        }
    }
    Ok(best)
}
pub fn min_l<T: Clone + PhpCmp>(l: &List<T>) -> Result<T, RtError> {
    let mut it = l.iter();
    let mut best = match it.next() {
        Some(v) => v.clone(),
        None => return Err(RtError::value_error("min(): Argument #1 ($value) must contain at least one element")),
    };
    for v in it {
        if v.php_cmp(&best) == Ordering::Less {
            best = v.clone();
        }
    }
    Ok(best)
}
pub fn max_m<K: MapKey, V: Clone + PhpCmp>(m: &Map<K, V>) -> Result<V, RtError> {
    max_l(&m.values_list())
}
pub fn min_m<K: MapKey, V: Clone + PhpCmp>(m: &Map<K, V>) -> Result<V, RtError> {
    min_l(&m.values_list())
}
pub fn max2<T: PhpCmp>(a: T, b: T) -> T {
    if b.php_cmp(&a) == Ordering::Greater { b } else { a }
}
pub fn min2<T: PhpCmp>(a: T, b: T) -> T {
    if b.php_cmp(&a) == Ordering::Less { b } else { a }
}
pub fn implode_keys<K: MapKey, V>(sep: &Str, m: &Map<K, V>) -> Str {
    let parts: Vec<Str> = m.keys().map(|k| k.to_str_key()).collect();
    crate::builtins::string::implode(sep, &parts)
}
/// Iterate a list with keys (foreach $k => $v).
pub fn enumerate_l<T: Clone>(l: &List<T>) -> Vec<(i64, T)> {
    l.iter().enumerate().map(|(i, v)| (i as i64, v.clone())).collect()
}
pub fn list_to_map_ak<T: Clone>(l: &List<T>) -> Map<ArrayKey, T> {
    l.iter().enumerate().map(|(i, v)| (ArrayKey::Int(i as i64), v.clone())).collect()
}
pub fn array_walk_keys_str<V: ToStr>(m: &Map<Str, V>) -> Vec<(Str, Str)> {
    m.iter().map(|(k, v)| (k.clone(), v.to_php_str())).collect()
}
pub fn compact_ref<T: Clone>(l: &List<T>) -> Vec<T> {
    l.to_vec()
}
