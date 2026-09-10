//! Runtime class/function registry (populated by the generated crate at startup).

use crate::containers::DynCallable;
use crate::error::RtError;
use crate::string::Str;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct ClassEntry {
    pub name: &'static str,
    pub ancestors: &'static [&'static str],
    pub is_interface: bool,
    pub is_trait: bool,
    /// source file the class was compiled from ("" for runtime stubs)
    pub file: &'static str,
}

type StaticDispatch = Box<dyn Fn(&str, Vec<crate::mixed::Mixed>) -> Result<crate::mixed::Mixed, crate::containers::DynError>>;

thread_local! {
    static STATICS: RefCell<HashMap<Vec<u8>, StaticDispatch>> = RefCell::new(HashMap::new());
    static FACTORIES: RefCell<HashMap<Vec<u8>, Box<dyn Fn() -> crate::mixed::Mixed>>> = RefCell::new(HashMap::new());
    static CLASSES: RefCell<HashMap<Vec<u8>, ClassEntry>> = RefCell::new(HashMap::new());
    static FUNCTIONS: RefCell<HashMap<Vec<u8>, DynCallable>> = RefCell::new(HashMap::new());
    static CONSTANTS: RefCell<HashMap<Vec<u8>, crate::mixed::Mixed>> = RefCell::new(HashMap::new());
    static CLASS_CONSTANTS: RefCell<HashMap<Vec<u8>, crate::mixed::Mixed>> = RefCell::new(HashMap::new());
}

pub fn register_class(name: &'static str, ancestors: &'static [&'static str], is_interface: bool, is_trait: bool, file: &'static str) {
    CLASSES.with(|c| {
        c.borrow_mut().insert(name.to_ascii_lowercase().into_bytes(), ClassEntry { name, ancestors, is_interface, is_trait, file });
    });
}

/// Names of the builtin and compiled classes (or interfaces), builtins first.
pub fn declared_classlikes(interfaces: bool) -> crate::list::List<Str> {
    let mut out = crate::list::List::new();
    let builtin = if interfaces { BUILTIN_INTERFACES } else { BUILTIN_CLASSES };
    for b in builtin {
        out.push(Str::from_str(&canonical_builtin_name(b)));
    }
    CLASSES.with(|c| {
        for e in c.borrow().values() {
            if e.is_interface == interfaces && !e.is_trait {
                out.push(Str::from_str(e.name));
            }
        }
    });
    out
}

/// PHP's spelling of a builtin class name known only in lowercase.
fn canonical_builtin_name(lc: &str) -> String {
    match lc {
        "stdclass" => "stdClass".into(),
        "datetime" => "DateTime".into(),
        "datetimeimmutable" => "DateTimeImmutable".into(),
        "datetimeinterface" => "DateTimeInterface".into(),
        "dateinterval" => "DateInterval".into(),
        "dateperiod" => "DatePeriod".into(),
        "datetimezone" => "DateTimeZone".into(),
        "arrayobject" => "ArrayObject".into(),
        "arrayiterator" => "ArrayIterator".into(),
        "arrayaccess" => "ArrayAccess".into(),
        "iteratoraggregate" => "IteratorAggregate".into(),
        "jsonserializable" => "JsonSerializable".into(),
        "splobjectstorage" => "SplObjectStorage".into(),
        "weakmap" => "WeakMap".into(),
        "weakreference" => "WeakReference".into(),
        "unitenum" => "UnitEnum".into(),
        "backedenum" => "BackedEnum".into(),
        other => {
            let mut s = other.to_string();
            if let Some(f) = s.get_mut(0..1) {
                f.make_ascii_uppercase();
            }
            s
        }
    }
}

/// Source file of a compiled class (None for classes the runtime does not know).
pub fn class_file(name: &[u8]) -> Option<Str> {
    let lc = norm(name);
    CLASSES.with(|c| c.borrow().get(&lc).map(|e| Str::from_str(e.file)))
}

pub fn class_is_trait(name: &[u8]) -> bool {
    let lc = norm(name);
    CLASSES.with(|c| c.borrow().get(&lc).map_or(false, |e| e.is_trait))
}

/// All constants of a class (declared or inherited), by name.
pub fn class_constants(name: &[u8]) -> crate::map::Map<crate::key::ArrayKey, crate::mixed::Mixed> {
    let mut prefix = norm(name);
    prefix.extend_from_slice(b"::");
    let mut out = crate::map::Map::new();
    CLASS_CONSTANTS.with(|c| {
        for (k, v) in c.borrow().iter() {
            if k.starts_with(&prefix) {
                out.insert(crate::key::ArrayKey::from_str_val(Str::from_bytes(&k[prefix.len()..])), v.clone());
            }
        }
    });
    out
}

/// Registers how to build an instance of a class without running its constructor.
thread_local! {
    static CTORS: RefCell<HashMap<Vec<u8>, Box<dyn Fn(Vec<crate::mixed::Mixed>) -> Result<crate::mixed::Mixed, crate::containers::DynError>>>> = RefCell::new(HashMap::new());
}

/// Registers the constructor of a class for `new $name(...)` with a runtime class name.
pub fn register_ctor(name: &str, f: Box<dyn Fn(Vec<crate::mixed::Mixed>) -> Result<crate::mixed::Mixed, crate::containers::DynError>>) {
    CTORS.with(|c| {
        c.borrow_mut().insert(name.to_ascii_lowercase().into_bytes(), f);
    });
}

/// `new $name(...args)`.
pub fn construct(name: &Str, args: Vec<crate::mixed::Mixed>) -> Result<crate::mixed::Mixed, crate::containers::DynError> {
    let lc = norm(name.as_bytes());
    let r = CTORS.with(|c| c.borrow().get(&lc).map(|f| f(args)));
    match r {
        Some(r) => r,
        None => Err(crate::containers::DynError::Rt(RtError::error(crate::sfmt!("Class \"{}\" not found", name)))),
    }
}

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

/// Builtin PHP classes that exist in any interpreter even though the runtime has no code for them.
const BUILTIN_CLASSES: &[&str] = &[
    "stdclass", "datetime", "datetimeimmutable", "dateinterval", "dateperiod", "datetimezone", "closure", "generator",
    "arrayobject", "arrayiterator", "splobjectstorage", "splstack", "splqueue", "spldoublylinkedlist", "splfixedarray",
    "splpriorityqueue", "splminheap", "splmaxheap", "weakmap", "weakreference", "exception", "error", "errorexception",
    "typeerror", "valueerror", "arithmeticerror", "divisionbyzeroerror", "argumentcounterror", "runtimeexception",
    "logicexception", "invalidargumentexception", "domainexception", "lengthexception", "outofrangeexception",
    "outofboundsexception", "rangeexception", "overflowexception", "underflowexception", "unexpectedvalueexception",
    "reflectionclass", "reflectionmethod", "reflectionproperty", "reflectionfunction", "reflectionnamedtype",
    "simplexmlelement", "domdocument", "domelement", "domnode", "pdo", "mysqli", "curlhandle", "attribute",
];
const BUILTIN_INTERFACES: &[&str] = &[
    "traversable", "iterator", "iteratoraggregate", "arrayaccess", "countable", "stringable", "throwable",
    "jsonserializable", "serializable", "unitenum", "backedenum", "datetimeinterface", "outeriterator",
    "recursiveiterator", "seekableiterator", "splobserver", "splsubject",
];

pub fn class_exists(name: &[u8]) -> bool {
    let lc = norm(name);
    CLASSES.with(|c| c.borrow().get(&lc).map_or(false, |e| !e.is_interface)) || BUILTIN_CLASSES.iter().any(|b| b.as_bytes() == lc.as_slice())
}

pub fn interface_exists(name: &[u8]) -> bool {
    let lc = norm(name);
    CLASSES.with(|c| c.borrow().get(&lc).map_or(false, |e| e.is_interface)) || BUILTIN_INTERFACES.iter().any(|b| b.as_bytes() == lc.as_slice())
}

pub fn classlike_exists(name: &[u8]) -> bool {
    let lc = norm(name);
    CLASSES.with(|c| c.borrow().contains_key(&lc)) || class_exists(name) || interface_exists(name)
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

/// Lowercased names of the registered user functions.
pub fn user_function_names() -> Vec<Vec<u8>> {
    FUNCTIONS.with(|f| f.borrow().keys().cloned().collect())
}

pub fn function_by_name(name: &Str) -> Option<DynCallable> {
    let lc = norm(name.as_bytes());
    FUNCTIONS.with(|f| f.borrow().get(&lc).cloned()).or_else(|| crate::builtins::misc::builtin_callable(&lc))
}

thread_local! {
    static FILES: RefCell<HashMap<Vec<u8>, std::rc::Rc<dyn Fn() -> Result<crate::mixed::Mixed, crate::containers::DynError>>>> = RefCell::new(HashMap::new());
}

/// Registers a compiled file (path relative to the source root) for `include`/`require` by path.
pub fn register_file(rel_path: &str, f: std::rc::Rc<dyn Fn() -> Result<crate::mixed::Mixed, crate::containers::DynError>>) {
    FILES.with(|c| {
        c.borrow_mut().insert(normalize_path(rel_path.as_bytes()), f);
    });
}

/// `include $path`: the value of the compiled file at `path` (absolute, or relative to the source root
/// or the working directory).
pub fn include_file(path: &Str) -> Result<crate::mixed::Mixed, crate::containers::DynError> {
    let key = normalize_path(path.as_bytes());
    let f = FILES.with(|c| c.borrow().get(&key).cloned());
    match f {
        Some(f) => f(),
        None => Err(crate::containers::DynError::Rt(RtError::error(crate::sfmt!(
            "include({}): file is not part of the compiled program",
            path
        )))),
    }
}

/// Canonical registry key: relative to the source root, `.`/`..` resolved, no leading slash.
fn normalize_path(p: &[u8]) -> Vec<u8> {
    let root = crate::support::src_root();
    let root_b = root.as_bytes();
    let mut rel: Vec<u8> = if !root_b.is_empty() && p.starts_with(root_b) && p.get(root_b.len()) == Some(&b'/') {
        p[root_b.len() + 1..].to_vec()
    } else if p.first() == Some(&b'/') {
        // an absolute path outside the root: resolved as-is
        p.to_vec()
    } else {
        p.to_vec()
    };
    if rel.contains(&b'\\') {
        rel = rel.iter().map(|b| if *b == b'\\' { b'/' } else { *b }).collect();
    }
    let absolute = rel.first() == Some(&b'/');
    let mut parts: Vec<&[u8]> = Vec::new();
    for seg in rel.split(|b| *b == b'/') {
        match seg {
            b"" | b"." => {}
            b".." => {
                parts.pop();
            }
            s => parts.push(s),
        }
    }
    let mut out = Vec::new();
    if absolute {
        out.push(b'/');
    }
    for (i, s) in parts.iter().enumerate() {
        if i > 0 {
            out.push(b'/');
        }
        out.extend_from_slice(s);
    }
    // an absolute path that turned out to be inside the root after resolution
    if absolute && !root_b.is_empty() && out.starts_with(root_b) && out.get(root_b.len()) == Some(&b'/') {
        return out[root_b.len() + 1..].to_vec();
    }
    out
}

pub fn define_constant(name: &Str, v: crate::mixed::Mixed) -> bool {
    if let Some(pos) = name.as_bytes().windows(2).position(|w| w == b"::") {
        // class constant: also indexed by lowercased class name for `$class::NAME` lookups
        let mut key = name.as_bytes()[..pos].to_ascii_lowercase();
        key.extend_from_slice(&name.as_bytes()[pos..]);
        CLASS_CONSTANTS.with(|c| {
            c.borrow_mut().entry(key).or_insert_with(|| v.clone());
        });
    }
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

/// `$class::NAME` / `$object::NAME`: the class constant of a class named by a string or an object.
pub fn class_constant(class: &crate::mixed::Mixed, name: &str) -> crate::mixed::Mixed {
    let cls: Vec<u8> = match class {
        crate::mixed::Mixed::Obj(o) => o.class_name().as_bytes().to_vec(),
        other => crate::traits::ToStr::to_php_str(other).as_bytes().to_vec(),
    };
    let mut key = norm(&cls);
    key.extend_from_slice(b"::");
    key.extend_from_slice(name.as_bytes());
    CLASS_CONSTANTS.with(|c| c.borrow().get(&key).cloned()).unwrap_or_else(|| {
        panic!("Undefined constant {}::{}", String::from_utf8_lossy(&cls), name)
    })
}

pub fn constant_value(name: &Str) -> Option<crate::mixed::Mixed> {
    CONSTANTS.with(|c| c.borrow().get(name.as_bytes()).cloned()).or_else(|| crate::consts::builtin_value(name.as_bytes()))
}
