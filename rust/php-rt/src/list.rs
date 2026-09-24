//! Copy-on-write list (PHP `list<T>`).

use crate::map::Map;
use std::fmt;
use std::ops::{Deref, Index};
use std::rc::Rc;
use smallvec::SmallVec;

/// Up to two elements live inside the Rc box (one allocation): the census showed len 1-2 lists are 86% of
/// all non-empty lists.
pub type Inner<T> = SmallVec<[T; 2]>;

/// `None` is the empty list (the census showed 37% of all allocations were `[]` boxes); the second field is
/// PHP's internal array pointer (`current()`/`next()`...), copied with the value.
pub struct List<T>(Option<Rc<Inner<T>>>, usize);


impl<T> Clone for List<T> {
    #[inline]
    fn clone(&self) -> List<T> {
        List(self.0.clone(), self.1)
    }
}

impl<T> Default for List<T> {
    fn default() -> List<T> {
        List::new()
    }
}


impl<T> Deref for List<T> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &[T] {
        self.v()
    }
}

impl<T> List<T> {
    /// The elements (an empty slice for the unallocated list).
    #[inline]
    fn v(&self) -> &[T] {
        match &self.0 {
            Some(rc) => rc.as_slice(),
            None => &[],
        }
    }
    #[inline]
    pub const fn new() -> List<T> {
        List(None, 0)
    }
    pub fn with_capacity(n: usize) -> List<T> {
        crate::stats::bump(crate::stats::LIST_WITH_CAP);
        List(Some(Rc::new(Inner::with_capacity(n))), 0)
    }
    #[inline]
    pub fn from_vec(v: Vec<T>) -> List<T> {
        #[cfg(feature = "stats")]
        crate::stats::bump(match v.len() { 0 => crate::stats::LIST_FROM_VEC_0, 1 => crate::stats::LIST_FROM_VEC_1, 2 => crate::stats::LIST_FROM_VEC_2, 3 | 4 => crate::stats::LIST_FROM_VEC_3_4, 5..=8 => crate::stats::LIST_FROM_VEC_5_8, _ => crate::stats::LIST_FROM_VEC_MORE });
        if v.is_empty() {
            return List(None, 0);
        }
        List(Some(Rc::new(Inner::from_vec(v))), 0)
    }
    /// A list from an array literal: one allocation, no intermediate Vec.
    #[inline]
    pub fn from_array<const N: usize>(a: [T; N]) -> List<T> {
        if N == 0 {
            return List(None, 0);
        }
        List(Some(Rc::new(Inner::from_iter(a))), 0)
    }
    /// `current()`: element at the internal pointer.
    pub fn ptr_current(&self) -> Option<&T> {
        self.v().get(self.1)
    }
    /// `key()`: index at the internal pointer.
    pub fn ptr_key(&self) -> Option<i64> {
        if self.1 < self.v().len() { Some(self.1 as i64) } else { None }
    }
    /// `next()`: advance the internal pointer and return the element there.
    pub fn ptr_next(&mut self) -> Option<&T> {
        if self.1 < self.v().len() {
            self.1 += 1;
        }
        self.v().get(self.1)
    }
    /// `prev()`
    pub fn ptr_prev(&mut self) -> Option<&T> {
        if self.1 == 0 || self.1 > self.v().len() {
            self.1 = self.v().len();
            return None;
        }
        self.1 -= 1;
        self.v().get(self.1)
    }
    /// `reset()`
    pub fn ptr_reset(&mut self) -> Option<&T> {
        self.1 = 0;
        self.v().first()
    }
    /// `end()`
    pub fn ptr_end(&mut self) -> Option<&T> {
        self.1 = self.v().len().saturating_sub(1);
        self.v().last()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.v().len()
    }
    #[inline]
    pub fn count(&self) -> i64 {
        self.v().len() as i64
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.v().is_empty()
    }
    #[inline]
    pub fn ptr_eq(&self, other: &List<T>) -> bool {
        match (&self.0, &other.0) {
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        }
    }
    #[inline]
    pub fn get(&self, i: i64) -> Option<&T> {
        if i < 0 {
            None
        } else {
            self.v().get(i as usize)
        }
    }
    /// Indexing that panics with a PHP-like message on a missing offset.
    #[inline]
    pub fn idx(&self, i: i64) -> &T {
        match self.get(i) {
            Some(v) => v,
            None => panic!("Undefined array key {}", i),
        }
    }
    pub fn has(&self, i: i64) -> bool {
        i >= 0 && (i as usize) < self.v().len()
    }
    pub fn first(&self) -> Option<&T> {
        self.v().first()
    }
    pub fn last(&self) -> Option<&T> {
        self.v().last()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.v().iter()
    }
    pub fn as_slice(&self) -> &[T] {
        self.v()
    }
}

impl<T: Clone> List<T> {
    #[inline]
    pub fn make_mut(&mut self) -> &mut Inner<T> {
        #[cfg(feature = "stats")]
        if let Some(rc) = &self.0 {
            if Rc::strong_count(rc) > 1 {
                crate::stats::bump(crate::stats::LIST_COW_CLONE);
            }
        }
        Rc::make_mut(self.0.get_or_insert_with(|| Rc::new(Inner::new())))
    }
    #[inline]
    pub fn push(&mut self, v: T) {
        let vec = self.make_mut();
        #[cfg(feature = "stats")]
        if vec.len() == vec.capacity() {
            crate::stats::bump(crate::stats::LIST_GROW);
        }
        vec.push(v);
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.v().is_empty() {
            return None;
        }
        self.make_mut().pop()
    }
    pub fn shift(&mut self) -> Option<T> {
        if self.v().is_empty() {
            return None;
        }
        Some(self.make_mut().remove(0))
    }
    pub fn unshift(&mut self, v: T) {
        self.make_mut().insert(0, v);
    }
    pub fn insert(&mut self, i: usize, v: T) {
        self.make_mut().insert(i, v);
    }
    pub fn remove(&mut self, i: usize) -> T {
        self.make_mut().remove(i)
    }
    /// `$list[$i] = $v`; appending at len() is allowed (PHP keeps it a list).
    pub fn set(&mut self, i: i64, v: T) {
        let n = self.v().len();
        if i >= 0 && (i as usize) < n {
            self.make_mut()[i as usize] = v;
        } else if i >= 0 && i as usize == n {
            self.make_mut().push(v);
        } else {
            panic!("List index {} out of range (len {})", i, n);
        }
    }
    /// Write through a by-reference element: a reference to an element that has since been removed
    /// writes nowhere, as PHP's does.
    pub fn replace(&mut self, i: i64, v: T) {
        if i >= 0 && (i as usize) < self.v().len() {
            self.make_mut()[i as usize] = v;
        }
    }
    #[inline]
    pub fn get_mut(&mut self, i: i64) -> Option<&mut T> {
        if i < 0 {
            return None;
        }
        self.make_mut().get_mut(i as usize)
    }
    pub fn idx_mut(&mut self, i: i64) -> &mut T {
        let n = self.v().len();
        if i >= 0 && (i as usize) < n {
            &mut self.make_mut()[i as usize]
        } else {
            panic!("Undefined array key {}", i)
        }
    }
    pub fn extend<I: IntoIterator<Item = T>>(&mut self, it: I) {
        self.make_mut().extend(it);
    }
    pub fn into_vec(self) -> Vec<T> {
        match self.0 {
            None => Vec::new(),
            Some(rc) => match Rc::try_unwrap(rc) {
                Ok(v) => v.into_vec(),
                Err(rc) => rc.to_vec(),
            },
        }
    }
    pub fn to_vec(&self) -> Vec<T> {
        self.v().to_vec()
    }
    pub fn to_map(&self) -> Map<i64, T> {
        self.v().iter().enumerate().map(|(i, v)| (i as i64, v.clone())).collect()
    }
    pub fn reverse(&mut self) {
        self.make_mut().reverse();
    }
    pub fn sort_by<F: FnMut(&T, &T) -> std::cmp::Ordering>(&mut self, f: F) {
        self.make_mut().sort_by(f);
    }
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        self.make_mut().retain(|v| f(v));
    }
    pub fn truncate(&mut self, n: usize) {
        self.make_mut().truncate(n);
    }
    pub fn slice(&self, start: usize, end: usize) -> List<T> {
        let end = end.min(self.v().len());
        let start = start.min(end);
        List::from_vec(self.v()[start..end].to_vec())
    }
    pub fn clear(&mut self) {
        *self = List::new();
    }
    pub fn splice_replace(&mut self, start: usize, len: usize, repl: Vec<T>) -> List<T> {
        let v = self.make_mut();
        let start = start.min(v.len());
        let end = (start + len).min(v.len());
        let removed: Vec<T> = v.drain(start..end).collect();
        v.insert_many(start, repl);
        List::from_vec(removed)
    }
    /// Map elements into a new list (used for element casts).
    pub fn map_elems<U, F: FnMut(T) -> U>(self, f: F) -> List<U> {
        List::from_vec(self.into_vec().into_iter().map(f).collect())
    }
}

impl<T: Clone> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}
impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.v().iter()
    }
}
impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> List<T> {
        List::from_vec(it.into_iter().collect())
    }
}
impl<T> From<Vec<T>> for List<T> {
    fn from(v: Vec<T>) -> List<T> {
        List::from_vec(v)
    }
}
impl<T> Index<usize> for List<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.v()[i]
    }
}
impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.v().iter()).finish()
    }
}
impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &List<T>) -> bool {
        self.ptr_eq(other) || self.v() == other.v()
    }
}
impl<T: Eq> Eq for List<T> {}

#[macro_export]
macro_rules! list {
    () => { $crate::List::new() };
    ($($x:expr),+ $(,)?) => { $crate::List::from_array([$($x),+]) };
}
