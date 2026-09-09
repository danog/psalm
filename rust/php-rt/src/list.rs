//! Copy-on-write list (PHP `list<T>`).

use crate::map::Map;
use std::fmt;
use std::ops::{Deref, Index};
use std::rc::Rc;

/// The second field is PHP's internal array pointer (`current()`/`next()`...), copied with the value.
pub struct List<T>(Rc<Vec<T>>, usize);

impl<T> Clone for List<T> {
    #[inline]
    fn clone(&self) -> List<T> {
        List(self.0.clone(), self.1)
    }
}

impl<T> Default for List<T> {
    fn default() -> List<T> {
        List(Rc::new(Vec::new()), 0)
    }
}

impl<T> Deref for List<T> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &[T] {
        self.0.as_slice()
    }
}

impl<T> List<T> {
    #[inline]
    pub fn new() -> List<T> {
        List(Rc::new(Vec::new()), 0)
    }
    pub fn with_capacity(n: usize) -> List<T> {
        List(Rc::new(Vec::with_capacity(n)), 0)
    }
    #[inline]
    pub fn from_vec(v: Vec<T>) -> List<T> {
        List(Rc::new(v), 0)
    }
    /// `current()`: element at the internal pointer.
    pub fn ptr_current(&self) -> Option<&T> {
        self.0.get(self.1)
    }
    /// `key()`: index at the internal pointer.
    pub fn ptr_key(&self) -> Option<i64> {
        if self.1 < self.0.len() { Some(self.1 as i64) } else { None }
    }
    /// `next()`: advance the internal pointer and return the element there.
    pub fn ptr_next(&mut self) -> Option<&T> {
        if self.1 < self.0.len() {
            self.1 += 1;
        }
        self.0.get(self.1)
    }
    /// `prev()`
    pub fn ptr_prev(&mut self) -> Option<&T> {
        if self.1 == 0 || self.1 > self.0.len() {
            self.1 = self.0.len();
            return None;
        }
        self.1 -= 1;
        self.0.get(self.1)
    }
    /// `reset()`
    pub fn ptr_reset(&mut self) -> Option<&T> {
        self.1 = 0;
        self.0.first()
    }
    /// `end()`
    pub fn ptr_end(&mut self) -> Option<&T> {
        self.1 = self.0.len().saturating_sub(1);
        self.0.last()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    #[inline]
    pub fn count(&self) -> i64 {
        self.0.len() as i64
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    #[inline]
    pub fn ptr_eq(&self, other: &List<T>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
    #[inline]
    pub fn get(&self, i: i64) -> Option<&T> {
        if i < 0 {
            None
        } else {
            self.0.get(i as usize)
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
        i >= 0 && (i as usize) < self.0.len()
    }
    pub fn first(&self) -> Option<&T> {
        self.0.first()
    }
    pub fn last(&self) -> Option<&T> {
        self.0.last()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}

impl<T: Clone> List<T> {
    #[inline]
    pub fn make_mut(&mut self) -> &mut Vec<T> {
        Rc::make_mut(&mut self.0)
    }
    #[inline]
    pub fn push(&mut self, v: T) {
        self.make_mut().push(v);
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.0.is_empty() {
            return None;
        }
        self.make_mut().pop()
    }
    pub fn shift(&mut self) -> Option<T> {
        if self.0.is_empty() {
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
        let n = self.0.len();
        if i >= 0 && (i as usize) < n {
            self.make_mut()[i as usize] = v;
        } else if i >= 0 && i as usize == n {
            self.make_mut().push(v);
        } else {
            panic!("List index {} out of range (len {})", i, n);
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
        let n = self.0.len();
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
        match Rc::try_unwrap(self.0) {
            Ok(v) => v,
            Err(rc) => (*rc).clone(),
        }
    }
    pub fn to_vec(&self) -> Vec<T> {
        (*self.0).clone()
    }
    pub fn to_map(&self) -> Map<i64, T> {
        self.0.iter().enumerate().map(|(i, v)| (i as i64, v.clone())).collect()
    }
    pub fn reverse(&mut self) {
        self.make_mut().reverse();
    }
    pub fn sort_by<F: FnMut(&T, &T) -> std::cmp::Ordering>(&mut self, f: F) {
        self.make_mut().sort_by(f);
    }
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, f: F) {
        self.make_mut().retain(f);
    }
    pub fn truncate(&mut self, n: usize) {
        self.make_mut().truncate(n);
    }
    pub fn slice(&self, start: usize, end: usize) -> List<T> {
        let end = end.min(self.0.len());
        let start = start.min(end);
        List::from_vec(self.0[start..end].to_vec())
    }
    pub fn clear(&mut self) {
        *self = List::new();
    }
    pub fn splice_replace(&mut self, start: usize, len: usize, repl: Vec<T>) -> List<T> {
        let v = self.make_mut();
        let start = start.min(v.len());
        let end = (start + len).min(v.len());
        let removed: Vec<T> = v.splice(start..end, repl).collect();
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
        self.0.iter()
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
        &self.0[i]
    }
}
impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}
impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &List<T>) -> bool {
        Rc::ptr_eq(&self.0, &other.0) || *self.0 == *other.0
    }
}
impl<T: Eq> Eq for List<T> {}

#[macro_export]
macro_rules! list {
    () => { $crate::List::new() };
    ($($x:expr),+ $(,)?) => { $crate::List::from_vec(vec![$($x),+]) };
}
