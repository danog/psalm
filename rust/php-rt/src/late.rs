//! Late-initialized field: a typed property without a default value.

use std::fmt;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct Late<T>(pub Option<T>);

impl<T> Late<T> {
    #[inline]
    pub const fn uninit() -> Late<T> {
        Late(None)
    }
    #[inline]
    pub fn new(v: T) -> Late<T> {
        Late(Some(v))
    }
    #[inline]
    pub fn set(&mut self, v: T) {
        self.0 = Some(v);
    }
    #[inline]
    pub fn is_init(&self) -> bool {
        self.0.is_some()
    }
    #[inline]
    pub fn get(&self) -> &T {
        match &self.0 {
            Some(v) => v,
            None => panic!("Typed property accessed before initialization"),
        }
    }
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        match &mut self.0 {
            Some(v) => v,
            None => panic!("Typed property accessed before initialization"),
        }
    }
    /// `&mut` access that initializes an uninitialized property with its type's default first
    /// (PHP auto-vivifies `$this->arr[] = ...` on an uninitialized array property).
    #[inline]
    pub fn get_or_default_mut(&mut self) -> &mut T
    where
        T: Default,
    {
        self.0.get_or_insert_with(T::default)
    }
    #[inline]
    pub fn take(self) -> T {
        self.0.expect("Typed property accessed before initialization")
    }
    pub fn as_option(&self) -> Option<&T> {
        self.0.as_ref()
    }
}

impl<T> Deref for Late<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        self.get()
    }
}
impl<T> DerefMut for Late<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}
impl<T> Default for Late<T> {
    fn default() -> Late<T> {
        Late(None)
    }
}
impl<T> From<T> for Late<T> {
    fn from(v: T) -> Late<T> {
        Late(Some(v))
    }
}
impl<T: fmt::Debug> fmt::Debug for Late<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(v) => v.fmt(f),
            None => write!(f, "<uninit>"),
        }
    }
}
