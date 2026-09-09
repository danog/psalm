//! PHP references (`$a = &$b`, `use (&$x)`).
//!
//! A local that is captured by reference lives in an `Rc<RefCell<Late<T>>>` cell shared by every
//! closure using it. A local that is bound by reference to varying targets (`$t = &$a; ... $t = &$o->p`)
//! holds a `PhpRef<T>`: a pair of getter/setter closures onto the current target.

use crate::late::Late;
use std::cell::RefCell;
use std::rc::Rc;

pub type Cell<T> = Rc<RefCell<Late<T>>>;

pub struct PhpRef<T> {
    get: Rc<dyn Fn() -> T>,
    set: Rc<dyn Fn(T)>,
}

impl<T> Clone for PhpRef<T> {
    fn clone(&self) -> Self {
        PhpRef { get: self.get.clone(), set: self.set.clone() }
    }
}

impl<T: Clone + 'static> PhpRef<T> {
    pub fn new(get: impl Fn() -> T + 'static, set: impl Fn(T) + 'static) -> Self {
        PhpRef { get: Rc::new(get), set: Rc::new(set) }
    }
    /// A reference onto a shared cell.
    pub fn from_cell(cell: Cell<T>) -> Self {
        let c1 = cell.clone();
        let c2 = cell;
        PhpRef::new(move || c1.borrow().get().clone(), move |v| c2.borrow_mut().set(v))
    }
    /// A reference variable that is not bound to anything yet: it behaves like a plain local.
    pub fn detached() -> Self {
        PhpRef::from_cell(Rc::new(RefCell::new(Late::uninit())))
    }
    /// A detached reference holding an initial value (a by-value parameter later rebound).
    pub fn of(v: T) -> Self {
        PhpRef::from_cell(Rc::new(RefCell::new(Late::new(v))))
    }
    pub fn get(&self) -> T {
        (self.get)()
    }
    pub fn set(&self, v: T) {
        (self.set)(v)
    }
}

pub fn new_cell<T>() -> Cell<T> {
    Rc::new(RefCell::new(Late::uninit()))
}

pub fn cell_of<T>(v: T) -> Cell<T> {
    Rc::new(RefCell::new(Late::new(v)))
}
