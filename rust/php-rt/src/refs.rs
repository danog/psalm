//! PHP references (`$a = &$b`, `use (&$x)`).
//!
//! A local that is captured by reference lives in an `Rc<RefCell<Late<T>>>` cell shared by every
//! closure using it. A local that is bound by reference to varying targets (`$t = &$a; ... $t = &$o->p`)
//! holds a `PhpRef<T>`: a pair of getter/setter closures onto the current target.

use crate::late::Late;
use crate::support::RwCell as RefCell;
use std::sync::Arc as Rc;

pub type Cell<T> = Rc<RefCell<Late<T>>>;

pub struct PhpRef<T> {
    get: Rc<dyn Fn() -> T>,
    set: Rc<dyn Fn(T)>,
    with: Rc<dyn Fn(&mut dyn FnMut(&mut T))>,
}

impl<T> Clone for PhpRef<T> {
    fn clone(&self) -> Self {
        PhpRef { get: self.get.clone(), set: self.set.clone(), with: self.with.clone() }
    }
}

impl<T: Clone + 'static> PhpRef<T> {
    /// A reference onto a getter/setter pair (a property): in-place mutation goes through both.
    pub fn new(get: impl Fn() -> T + 'static, set: impl Fn(T) + 'static) -> Self {
        let get: Rc<dyn Fn() -> T> = Rc::new(get);
        let set: Rc<dyn Fn(T)> = Rc::new(set);
        let (g, s) = (get.clone(), set.clone());
        PhpRef {
            get,
            set,
            with: Rc::new(move |f: &mut dyn FnMut(&mut T)| {
                let mut v = g();
                f(&mut v);
                s(v);
            }),
        }
    }
    /// A reference onto a shared cell: mutation happens in place.
    pub fn from_cell(cell: Cell<T>) -> Self {
        let c1 = cell.clone();
        let c2 = cell.clone();
        let c3 = cell;
        PhpRef {
            get: Rc::new(move || c1.borrow().get().clone()),
            set: Rc::new(move |v| c2.borrow_mut().set(v)),
            with: Rc::new(move |f: &mut dyn FnMut(&mut T)| f(c3.borrow_mut().get_mut())),
        }
    }
    /// Run `f` on the referenced value in place.
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut f = Some(f);
        let mut out = None;
        (self.with)(&mut |t: &mut T| {
            if let Some(f) = f.take() {
                out = Some(f(t));
            }
        });
        out.expect("with_mut closure not invoked")
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
