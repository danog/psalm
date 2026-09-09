//! Runtime class/function registry (populated by the generated crate at startup).

use crate::containers::DynCallable;
use crate::string::Str;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct ClassEntry {
    pub name: &'static str,
    pub ancestors: &'static [&'static str],
    pub is_interface: bool,
}

type StaticDispatch = Box<dyn Fn(&str, Vec<crate::mixed::Mixed>) -> Result<crate::mixed::Mixed, crate::containers::DynError>>;

thread_local! {
    static STATICS: RefCell<HashMap<Vec<u8>, StaticDispatch>> = RefCell::new(HashMap::new());
    static FACTORIES: RefCell<HashMap<Vec<u8>, Box<dyn Fn() -> crate::mixed::Mixed>>> = RefCell::new(HashMap::new());
    static CLASSES: RefCell<HashMap<Vec<u8>, ClassEntry>> = RefCell::new(HashMap::new());
    static FUNCTIONS: RefCell<HashMap<Vec<u8>, DynCallable>> = RefCell::new(HashMap::new());
    static CONSTANTS: RefCell<HashMap<Vec<u8>, crate::mixed::Mixed>> = RefCell::new(HashMap::new());
}

pub fn register_class(name: &'static str, ancestors: &'static [&'static str], is_interface: bool) {
    CLASSES.with(|c| {
        c.borrow_mut().insert(name.to_ascii_lowercase().into_bytes(), ClassEntry { name, ancestors, is_interface });
    });
}

/// Registers how to build an instance of a class without running its constructor.
pub fn register_factory(name: &str, f: Box<dyn Fn() -> crate::mixed::Mixed>) {
    FACTORIES.with(|c| {
        c.borrow_mut().insert(name.to_ascii_lowercase().into_bytes(), f);
    });
}

/// Registers the static-method dispatcher of a class (`$class::method(...)` with a runtime name).
pub fn register_static(name: &str, f: StaticDispatch) {
    STATICS.with(|c| {
        c.borrow_mut().insert(name.to_ascii_lowercase().into_bytes(), f);
    });
}

/// `$class::method(...)` on a class named at runtime.
pub fn call_static(class: &[u8], method: &str, args: Vec<crate::mixed::Mixed>) -> Result<crate::mixed::Mixed, crate::containers::DynError> {
    let lc = norm(class);
    let lm = method.to_ascii_lowercase();
    let f = STATICS.with(|c| c.borrow().get(&lc).map(|f| f as *const StaticDispatch));
    match f {
        // the dispatcher table is only appended to; the pointer stays valid
        Some(f) => unsafe { (*f)(&lm, args) },
        None => Err(crate::containers::DynError::Rt(crate::error::RtError::error(crate::sfmt!("Class \"{}\" not found", String::from_utf8_lossy(class))))),
    }
}

/// `ReflectionClass::newInstanceWithoutConstructor()`: `None` when the class is unknown.
pub fn new_uninit(name: &[u8]) -> Option<crate::mixed::Mixed> {
    let lc = norm(name);
    FACTORIES.with(|c| c.borrow().get(&lc).map(|f| f()))
}

pub fn register_function(name: &str, f: DynCallable) {
    FUNCTIONS.with(|c| {
        c.borrow_mut().insert(name.to_ascii_lowercase().into_bytes(), f);
    });
}

pub fn class_exists(name: &[u8]) -> bool {
    let lc = norm(name);
    CLASSES.with(|c| c.borrow().get(&lc).map_or(false, |e| !e.is_interface))
}

pub fn interface_exists(name: &[u8]) -> bool {
    let lc = norm(name);
    CLASSES.with(|c| c.borrow().get(&lc).map_or(false, |e| e.is_interface))
}

pub fn classlike_exists(name: &[u8]) -> bool {
    let lc = norm(name);
    CLASSES.with(|c| c.borrow().contains_key(&lc))
}

/// Canonical (declared-case) class name.
pub fn class_name(name: &[u8]) -> Option<&'static str> {
    let lc = norm(name);
    CLASSES.with(|c| c.borrow().get(&lc).map(|e| e.name))
}

fn norm(name: &[u8]) -> Vec<u8> {
    let n = if name.first() == Some(&b'\\') { &name[1..] } else { name };
    n.to_ascii_lowercase()
}

/// `is_subclass_of`/`is_a` on class names.
pub fn is_subclass_name(sub: &[u8], parent: &[u8], allow_same: bool) -> bool {
    let sub = norm(sub);
    let parent = norm(parent);
    if sub == parent {
        return allow_same;
    }
    CLASSES.with(|c| {
        c.borrow().get(&sub).map_or(false, |e| e.ancestors.iter().any(|a| a.as_bytes() == parent.as_slice()))
    })
}

pub fn function_exists(name: &[u8]) -> bool {
    let lc = norm(name);
    FUNCTIONS.with(|f| f.borrow().contains_key(&lc)) || crate::builtins::misc::builtin_function_exists(&lc)
}

pub fn function_by_name(name: &Str) -> Option<DynCallable> {
    let lc = norm(name.as_bytes());
    FUNCTIONS.with(|f| f.borrow().get(&lc).cloned()).or_else(|| crate::builtins::misc::builtin_callable(&lc))
}

pub fn define_constant(name: &Str, v: crate::mixed::Mixed) -> bool {
    CONSTANTS.with(|c| {
        let mut c = c.borrow_mut();
        if c.contains_key(name.as_bytes()) {
            return false;
        }
        c.insert(name.to_vec(), v);
        true
    })
}

pub fn constant_defined(name: &Str) -> bool {
    CONSTANTS.with(|c| c.borrow().contains_key(name.as_bytes())) || crate::consts::builtin_defined(name.as_bytes())
}

pub fn constant_value(name: &Str) -> Option<crate::mixed::Mixed> {
    CONSTANTS.with(|c| c.borrow().get(name.as_bytes()).cloned()).or_else(|| crate::consts::builtin_value(name.as_bytes()))
}
