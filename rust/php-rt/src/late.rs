//! Late-initialized field: a typed property without a default value.

use std::cell::UnsafeCell;
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

/// A field written only while its object is being constructed (the transpiler's construction-only analysis:
/// no assignment, by-reference use or external write after the constructor and its init helpers), stored
/// outside the object's RefCell so that a read through a shared handle is a plain `&T`: no borrow flag, no
/// clone, no refcount. The writes (constructor, `__unserialize`, init helpers) go through `&self`; the
/// analysis guarantees no read borrow is live across them.
pub struct Init<T>(UnsafeCell<Late<T>>);

impl<T> Init<T> {
    #[inline]
    pub const fn uninit() -> Init<T> {
        Init(UnsafeCell::new(Late::uninit()))
    }
    #[inline]
    pub fn new(v: T) -> Init<T> {
        Init(UnsafeCell::new(Late::new(v)))
    }
    #[inline]
    fn inner(&self) -> &Late<T> {
        unsafe { &*self.0.get() }
    }
    #[inline]
    #[allow(clippy::mut_from_ref)]
    fn inner_mut(&self) -> &mut Late<T> {
        unsafe { &mut *self.0.get() }
    }
    #[inline]
    pub fn set(&self, v: T) {
        self.inner_mut().set(v);
    }
    #[inline]
    pub fn is_init(&self) -> bool {
        self.inner().is_init()
    }
    #[inline]
    pub fn get(&self) -> &T {
        self.inner().get()
    }
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut T {
        self.inner_mut().get_mut()
    }
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub fn get_or_default_mut(&self) -> &mut T
    where
        T: Default,
    {
        self.inner_mut().get_or_default_mut()
    }
    #[inline]
    pub fn as_option(&self) -> Option<&T> {
        self.inner().as_option()
    }
}
impl<T: Clone> Clone for Init<T> {
    fn clone(&self) -> Init<T> {
        Init(UnsafeCell::new(self.inner().clone()))
    }
}
impl<T> Default for Init<T> {
    fn default() -> Init<T> {
        Init::uninit()
    }
}
impl<T: fmt::Debug> fmt::Debug for Init<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.inner(), f)
    }
}
