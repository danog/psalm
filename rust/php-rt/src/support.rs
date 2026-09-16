//! Glue used by generated code: instanceof, downcasts, Mixed helpers, misc.

use crate::conv::Num;
use crate::error::RtError;
use crate::key::ArrayKey;
use crate::list::List;
use crate::map::Map;
use crate::mixed::{AnyObj, Mixed, PhpObject};
use crate::string::Str;
use crate::traits::*;
use std::cell::RefCell;
use std::sync::Arc as Rc;

// ---------------------------------------------------------------- instanceof / downcasts

pub trait InstanceOf<T> {
    fn is_instance(&self) -> bool;
}
#[inline]
pub fn is_instance<T>(s: &(impl InstanceOf<T> + ?Sized)) -> bool {
    s.is_instance()
}

pub trait TryDowncast: Sized {
    fn try_downcast(o: &AnyObj) -> Option<Self>;
}
#[inline]
pub fn try_downcast<T: TryDowncast>(o: &AnyObj) -> Option<T> {
    T::try_downcast(o)
}

/// Attempt a Mixed narrowing without panicking.
pub fn try_cast_mixed<T>(m: &Mixed) -> Option<T>
where
    Mixed: crate::cast::CastTo<T>,
{
    let m = m.clone();
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || crate::cast::cast::<T>(m))).ok()
}

// ---------------------------------------------------------------- source locations

thread_local! {
    static SRC_ROOT: RefCell<Option<Str>> = RefCell::new(None);
}

/// Configure where `__DIR__`/`__FILE__` resolve (defaults to `$PHP_SRC_ROOT` or the cwd).
pub fn set_src_root(root: &str) {
    SRC_ROOT.with(|r| *r.borrow_mut() = Some(Str::from_str(root)));
}

pub fn src_root() -> Str {
    SRC_ROOT.with(|r| {
        if let Some(s) = &*r.borrow() {
            return s.clone();
        }
        let root = std::env::var("PHP_SRC_ROOT").unwrap_or_else(|_| std::env::current_dir().map(|p| p.to_string_lossy().into_owned()).unwrap_or_else(|_| ".".into()));
        Str::from_string(root)
    })
}

pub fn src_dir(rel: &str) -> Str {
    let root = src_root();
    if rel == "." || rel.is_empty() {
        return root;
    }
    crate::sfmt!("{}/{}", root, rel)
}

pub fn src_file(rel: &str) -> Str {
    crate::sfmt!("{}/{}", src_root(), rel)
}

// ---------------------------------------------------------------- fallback throw type

/// Used when the generated crate has no Throwable class (tests of the runtime itself).
#[derive(Clone, Debug)]
pub struct FallbackThrow(pub RtError);

impl FallbackThrow {
    pub fn error(msg: Str) -> Self {
        FallbackThrow(RtError::error(msg))
    }
    pub fn value_error(msg: Str) -> Self {
        FallbackThrow(RtError::value_error(msg))
    }
    pub fn type_error(msg: Str) -> Self {
        FallbackThrow(RtError::type_error(msg))
    }
    pub fn assertion(msg: Str) -> Self {
        FallbackThrow(RtError::new("AssertionError", msg))
    }
    pub fn unhandled_match<T: std::fmt::Debug>(v: &T) -> Self {
        FallbackThrow(RtError::new("UnhandledMatchError", crate::sfmt!("Unhandled match case {:?}", v)))
    }
    pub fn exit(status: i64) -> Self {
        FallbackThrow(RtError::new("exit", crate::sfmt!("{}", status)))
    }
    pub fn exit_status(&self) -> Option<i64> {
        if self.0.class == "exit" { Some(crate::conv::str_to_int(&self.0.message)) } else { None }
    }
    pub fn message(&self) -> Str {
        self.0.message.clone()
    }
}
impl crate::error::PhpThrowable for FallbackThrow {
    fn class_name(&self) -> &'static str {
        "Exception"
    }
    fn class_ancestors(&self) -> &'static [&'static str] {
        &["exception", "throwable"]
    }
    fn message(&self) -> Str {
        self.0.message.clone()
    }
}
impl From<RtError> for FallbackThrow {
    fn from(e: RtError) -> Self {
        FallbackThrow(e)
    }
}
impl std::fmt::Display for FallbackThrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------- PHP enums

#[macro_export]
macro_rules! impl_enum_handle {
    ($name:ident, $fqcn:expr, $ancestors:expr) => {
        impl $crate::PhpObject for $name {
            fn class_name(&self) -> &'static str {
                $fqcn
            }
            fn class_ancestors(&self) -> &'static [&'static str] {
                $ancestors
            }
            fn obj_id(&self) -> usize {
                // enum cases are singletons: identity is the discriminant
                (*self as usize) + 1
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn props(&self) -> Vec<($crate::Str, $crate::Mixed)> {
                vec![($crate::Str::from_static("name"), $crate::Mixed::Str(self.name()))]
            }
        }
        impl $crate::Truthy for $name {
            fn truthy(&self) -> bool {
                true
            }
        }
        impl $crate::Identical for $name {
            fn identical(&self, o: &Self) -> bool {
                self == o
            }
        }
        impl $crate::PhpCmp for $name {
            fn php_cmp(&self, o: &Self) -> std::cmp::Ordering {
                (*self as usize).cmp(&(*o as usize))
            }
        }
        impl $crate::ToStr for $name {
            fn to_php_str(&self) -> $crate::Str {
                self.name()
            }
        }
        impl $crate::CastTo<$crate::Mixed> for $name {
            fn cast_to(self) -> $crate::Mixed {
                $crate::Mixed::Obj(std::sync::Arc::new(self))
            }
        }
        impl $crate::CastTo<$name> for $crate::Mixed {
            fn cast_to(self) -> $name {
                if let $crate::Mixed::Obj(o) = &self {
                    if let Some(v) = o.as_any().downcast_ref::<$name>() {
                        return *v;
                    }
                }
                panic!("Mixed value is not a {}", $fqcn)
            }
        }
        impl $crate::InstanceOf<$name> for $crate::Mixed {
            fn is_instance(&self) -> bool {
                self.instance_of(&$fqcn.to_ascii_lowercase())
            }
        }
        impl $crate::PhpClone for $name {
            fn php_clone(&self) -> Self {
                *self
            }
        }
    };
}

// ---------------------------------------------------------------- dynamic dispatch arguments

/// Argument `i` of a dynamic call converted to the parameter type (its default when missing).
pub fn dyn_arg<T: Default>(args: &[Mixed], i: usize) -> T
where
    Mixed: crate::cast::CastTo<T>,
{
    match args.get(i) {
        Some(a) => crate::cast::cast::<T>(a.clone()),
        None => T::default(),
    }
}

/// Argument `i` of a dynamic call converted to the parameter type (null when missing).
pub fn dyn_arg_req<T>(args: &[Mixed], i: usize) -> T
where
    Mixed: crate::cast::CastTo<T>,
{
    crate::cast::cast::<T>(args.get(i).cloned().unwrap_or(Mixed::Null))
}

// ---------------------------------------------------------------- superglobals

/// `$_SERVER` and friends: a minimal environment view (argv, REQUEST_TIME, env variables).
thread_local! {
    static SUPERGLOBALS: std::cell::RefCell<crate::FastMap<String, Map<Str, Mixed>>> = std::cell::RefCell::new(crate::fast_map());
    static GLOBALS: std::cell::RefCell<crate::FastMap<String, Mixed>> = std::cell::RefCell::new(crate::fast_map());
}

/// `$_SERVER` and friends: stored per thread, initialized from the environment on first use.
pub fn superglobal(name: &str) -> Map<Str, Mixed> {
    SUPERGLOBALS.with(|s| {
        if let Some(m) = s.borrow().get(name) {
            return m.clone();
        }
        let m = initial_superglobal(name);
        s.borrow_mut().insert(name.to_string(), m.clone());
        m
    })
}

pub fn superglobal_set(name: &str, m: Map<Str, Mixed>) {
    SUPERGLOBALS.with(|s| {
        s.borrow_mut().insert(name.to_string(), m);
    });
}

/// `global $x`: the script-level variable `x` (`$argv`/`$argc` come from the process arguments).
pub fn global_get(name: &str) -> Mixed {
    GLOBALS.with(|g| {
        if let Some(v) = g.borrow().get(name) {
            return v.clone();
        }
        let v = match name {
            "argv" => superglobal("_SERVER").get(&Str::from_static("argv")).cloned().unwrap_or(Mixed::Null),
            "argc" => superglobal("_SERVER").get(&Str::from_static("argc")).cloned().unwrap_or(Mixed::Null),
            _ => Mixed::Null,
        };
        g.borrow_mut().insert(name.to_string(), v.clone());
        v
    })
}

pub fn global_set(name: &str, v: Mixed) {
    GLOBALS.with(|g| {
        g.borrow_mut().insert(name.to_string(), v);
    });
}

fn initial_superglobal(name: &str) -> Map<Str, Mixed> {
    let mut m: Map<Str, Mixed> = Map::new();
    match name {
        "_SERVER" => {
            let argv: Map<ArrayKey, Mixed> = std::env::args().map(|a| Mixed::Str(Str::from_string(a))).collect::<Vec<_>>().into_iter().enumerate().map(|(i, v)| (ArrayKey::Int(i as i64), v)).collect();
            m.insert(Str::from_static("argc"), Mixed::Int(argv.len() as i64));
            m.insert(Str::from_static("argv"), Mixed::Arr(argv));
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0);
            m.insert(Str::from_static("REQUEST_TIME"), Mixed::Int(now as i64));
            m.insert(Str::from_static("REQUEST_TIME_FLOAT"), Mixed::Float(now));
            for (k, v) in std::env::vars() {
                m.insert(Str::from_string(k), Mixed::Str(Str::from_string(v)));
            }
        }
        "_ENV" => {
            for (k, v) in std::env::vars() {
                m.insert(Str::from_string(k), Mixed::Str(Str::from_string(v)));
            }
        }
        _ => {}
    }
    m
}

// ---------------------------------------------------------------- escape variants

/// A non-object value stored where a class was declared (`Other__` variants of class enums):
/// answers the object protocol with PHP's type name and no members.
pub struct NonObject(pub Mixed);
impl PhpObject for NonObject {
    fn class_name(&self) -> &'static str {
        match &self.0 {
            Mixed::Null => "null",
            Mixed::Bool(_) => "bool",
            Mixed::Int(_) => "int",
            Mixed::Float(_) => "float",
            Mixed::Str(_) => "string",
            Mixed::Arr(_) => "array",
            Mixed::Closure(_) => "Closure",
            Mixed::Obj(_) => "object",
        }
    }
    fn class_ancestors(&self) -> &'static [&'static str] {
        &[]
    }
    fn obj_id(&self) -> usize {
        0
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn php_to_string(&self) -> Option<Str> {
        Some(crate::traits::to_str(&self.0))
    }
}

/// The object behind a `Mixed` held by an escape variant, or a [`NonObject`] view of a scalar.
pub fn other_obj(m: &Mixed) -> AnyObj {
    match m {
        Mixed::Obj(o) => o.clone(),
        other => Rc::new(NonObject(other.clone())),
    }
}

// ---------------------------------------------------------------- Mixed helpers

impl Mixed {
    /// Null => None, otherwise Some(self).
    pub fn to_option(self) -> Option<Mixed> {
        match self {
            Mixed::Null => None,
            m => Some(m),
        }
    }
}

pub fn mixed_get(m: &Mixed, k: &ArrayKey) -> Option<Mixed> {
    match m {
        Mixed::Arr(a) => a.get(k).cloned().and_then(|v| v.to_option()),
        Mixed::Str(s) => match k {
            ArrayKey::Int(i) => crate::ops::str_index_byte(s, *i).map(|b| Mixed::Str(Str::from_vec(vec![b]))),
            _ => None,
        },
        Mixed::Obj(o) => {
            // ArrayAccess objects: offsetExists() then offsetGet()
            let key = match k { ArrayKey::Int(i) => Mixed::Int(*i), ArrayKey::Str(s) => Mixed::Str(s.clone()) };
            if crate::traits::Truthy::truthy(&o.call_method("offsetexists", vec![key.clone()])) {
                o.call_method("offsetget", vec![key]).to_option()
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn mixed_set(m: &mut Mixed, k: Option<ArrayKey>, v: Mixed) {
    if let Mixed::Null = m {
        *m = Mixed::Arr(Map::new());
    }
    if let Mixed::Arr(a) = m {
        match k {
            Some(k) => a.insert(k, v),
            None => {
                a.push(v);
                None
            }
        };
    }
}

pub fn mixed_entry(m: &mut Mixed, k: Option<ArrayKey>) -> &mut Mixed {
    if let Mixed::Null = m {
        *m = Mixed::Arr(Map::new());
    }
    match m {
        Mixed::Arr(a) => match k {
            Some(k) => a.entry_or_default(k),
            None => {
                a.push(Mixed::Null);
                let key = a.last_key().cloned().unwrap();
                a.get_mut(&key).unwrap()
            }
        },
        _ => panic!("Cannot use a scalar value as an array"),
    }
}

pub fn mixed_pop(m: &mut Mixed) -> Option<Mixed> {
    match m {
        Mixed::Arr(a) => a.pop(),
        _ => None,
    }
}

pub fn mixed_shift(m: &mut Mixed) -> Option<Mixed> {
    match m {
        Mixed::Arr(a) => a.shift(),
        _ => None,
    }
}

pub fn mixed_unset(m: &mut Mixed, k: &ArrayKey) {
    if let Mixed::Arr(a) = m {
        a.remove(k);
    }
}

pub fn mixed_prop(m: &Mixed, name: &Str) -> Option<Mixed> {
    match m {
        Mixed::Obj(o) => o.get_prop(&name.to_string_lossy()).and_then(|v| v.to_option()),
        _ => None,
    }
}

pub fn mixed_set_prop(m: &Mixed, name: &Str, v: Mixed) {
    match m {
        Mixed::Obj(o) => {
            if !o.set_prop(&name.to_string_lossy(), v) {
                panic!("Cannot write undefined property ${} on {}", name, o.class_name());
            }
        }
        _ => panic!("Attempt to assign property ${} on non-object", name),
    }
}

pub fn mixed_inc(m: &Mixed) -> Mixed {
    match m {
        Mixed::Int(i) => match i.checked_add(1) {
            Some(r) => Mixed::Int(r),
            None => Mixed::Float(*i as f64 + 1.0),
        },
        Mixed::Float(f) => Mixed::Float(f + 1.0),
        Mixed::Null => Mixed::Int(1),
        Mixed::Str(s) => match crate::conv::parse_numeric(s) {
            Some(Num::Int(i)) => Mixed::Int(i + 1),
            Some(Num::Float(f)) => Mixed::Float(f + 1.0),
            None => Mixed::Str(crate::ops::str_increment(s)),
        },
        other => other.clone(),
    }
}

pub fn mixed_dec(m: &Mixed) -> Mixed {
    match m {
        Mixed::Int(i) => match i.checked_sub(1) {
            Some(r) => Mixed::Int(r),
            None => Mixed::Float(*i as f64 - 1.0),
        },
        Mixed::Float(f) => Mixed::Float(f - 1.0),
        Mixed::Str(s) => match crate::conv::parse_numeric(s) {
            Some(Num::Int(i)) => Mixed::Int(i - 1),
            Some(Num::Float(f)) => Mixed::Float(f - 1.0),
            None => Mixed::Str(s.clone()),
        },
        other => other.clone(),
    }
}

pub fn mixed_call(m: &Mixed, name: &Str, args: Vec<Mixed>) -> Mixed {
    use crate::containers::to_callable;
    match m {
        Mixed::Obj(o) => o.call_method(&name.to_string_lossy().to_ascii_lowercase(), args),
        Mixed::Closure(c) if name.as_bytes().eq_ignore_ascii_case(b"__invoke") || name.as_bytes().eq_ignore_ascii_case(b"call") => {
            let _ = c;
            to_callable(m).call(args)
        }
        _ => panic!("Uncaught exception: Call to a member function {}() on {}", name, m.type_name()),
    }
}

pub fn mixed_iter(m: Mixed) -> Result<std::vec::IntoIter<(ArrayKey, Mixed)>, RtError> {
    match m {
        Mixed::Arr(a) => Ok(a.into_iter()),
        Mixed::Null => Ok(Vec::new().into_iter()),
        Mixed::Obj(o) => Ok(o.props().into_iter().map(|(k, v)| (ArrayKey::Str(k), v)).collect::<Vec<_>>().into_iter()),
        _ => Err(RtError::type_error("foreach() argument must be of type array|object")),
    }
}

pub fn mixed_to_array(m: Mixed) -> Map<ArrayKey, Mixed> {
    match m {
        Mixed::Arr(a) => a,
        Mixed::Null => Map::new(),
        Mixed::Obj(o) => o.props().into_iter().map(|(k, v)| (ArrayKey::Str(k), v)).collect(),
        other => {
            let mut m = Map::new();
            m.push(other);
            m
        }
    }
}

pub fn object_to_array(m: &Mixed) -> Map<ArrayKey, Mixed> {
    match m {
        Mixed::Obj(o) => o.props().into_iter().map(|(k, v)| (ArrayKey::Str(k), v)).collect(),
        other => mixed_to_array(other.clone()),
    }
}

pub fn class_name_of(m: &Mixed) -> Str {
    match m {
        Mixed::Obj(o) => Str::from_str(o.class_name()),
        Mixed::Closure(_) => Str::from_static("Closure"),
        _ => Str::empty(),
    }
}

pub fn get_class(m: &Mixed) -> Str {
    class_name_of(m)
}

fn norm_class(name: &Str) -> Vec<u8> {
    let b = name.as_bytes();
    let b = if b.first() == Some(&b'\\') { &b[1..] } else { b };
    b.to_ascii_lowercase()
}

pub fn instance_of_name(m: &Mixed, name: &Str) -> bool {
    let n = norm_class(name);
    m.instance_of(std::str::from_utf8(&n).unwrap_or(""))
}

/// `is_a($object, $class)`: class names as subjects are resolved at compile time (closed world).
pub fn is_a_name(m: &Mixed, class: &Str, _allow_string: bool) -> bool {
    match m {
        Mixed::Obj(_) | Mixed::Closure(_) => instance_of_name(m, class),
        _ => false,
    }
}

/// `is_subclass_of($object, $class)`: class names as subjects are resolved at compile time (closed world).
pub fn is_subclass_of_name(m: &Mixed, class: &Str) -> bool {
    match m {
        Mixed::Obj(o) => {
            let n = norm_class(class);
            let n = std::str::from_utf8(&n).unwrap_or("");
            o.class_name().to_ascii_lowercase() != n && o.instance_of_name(n)
        }
        _ => false,
    }
}

pub fn spl_object_id(m: &Mixed) -> i64 {
    match m {
        Mixed::Obj(o) => o.obj_id() as i64,
        Mixed::Closure(c) => Rc::as_ptr(c) as *const u8 as usize as i64,
        _ => 0,
    }
}

pub fn spl_object_hash(m: &Mixed) -> Str {
    Str::from_string(format!("{:032x}", spl_object_id(m)))
}

pub fn array_is_list_val<T: crate::traits::Len + ?Sized>(_v: &T) -> bool {
    true
}

pub fn is_numeric_val<T: ToStr + ?Sized>(v: &T) -> bool {
    crate::conv::parse_numeric(v.to_php_str().as_bytes()).is_some()
}

pub fn is_iterable_val(m: &Mixed) -> bool {
    matches!(m, Mixed::Arr(_)) || m.instance_of("traversable")
}
pub fn is_countable_val(m: &Mixed) -> bool {
    matches!(m, Mixed::Arr(_)) || m.instance_of("countable")
}
pub fn is_scalar_val(m: &Mixed) -> bool {
    m.is_scalar()
}

pub fn is_callable(m: &Mixed) -> bool {
    match m {
        Mixed::Closure(_) => true,
        Mixed::Obj(o) => o.instance_of_name("closure"),
        Mixed::Arr(a) => a.len() == 2,
        _ => false,
    }
}

// ---------------------------------------------------------------- numbers

pub fn to_num(m: &Mixed) -> Num {
    match m {
        Mixed::Int(i) => Num::Int(*i),
        Mixed::Float(f) => Num::Float(*f),
        Mixed::Bool(b) => Num::Int(*b as i64),
        Mixed::Null => Num::Int(0),
        Mixed::Str(s) => match crate::conv::parse_numeric_prefix(s) {
            Some((n, _)) => n,
            None => Num::Int(0),
        },
        _ => Num::Int(1),
    }
}

impl crate::cast::CastTo<Mixed> for Num {
    fn cast_to(self) -> Mixed {
        self.to_mixed()
    }
}

impl Num {
    pub fn to_mixed(self) -> Mixed {
        match self {
            Num::Int(i) => Mixed::Int(i),
            Num::Float(f) => Mixed::Float(f),
        }
    }
}

pub fn num_add(a: Num, b: Num) -> Num {
    match (a, b) {
        (Num::Int(x), Num::Int(y)) => match x.checked_add(y) {
            Some(r) => Num::Int(r),
            None => Num::Float(x as f64 + y as f64),
        },
        _ => Num::Float(a.to_f64() + b.to_f64()),
    }
}
pub fn num_sub(a: Num, b: Num) -> Num {
    match (a, b) {
        (Num::Int(x), Num::Int(y)) => match x.checked_sub(y) {
            Some(r) => Num::Int(r),
            None => Num::Float(x as f64 - y as f64),
        },
        _ => Num::Float(a.to_f64() - b.to_f64()),
    }
}
pub fn num_mul(a: Num, b: Num) -> Num {
    match (a, b) {
        (Num::Int(x), Num::Int(y)) => match x.checked_mul(y) {
            Some(r) => Num::Int(r),
            None => Num::Float(x as f64 * y as f64),
        },
        _ => Num::Float(a.to_f64() * b.to_f64()),
    }
}
pub fn abs_num(a: Num) -> Num {
    match a {
        Num::Int(i) => Num::Int(i.wrapping_abs()),
        Num::Float(f) => Num::Float(f.abs()),
    }
}

// ---------------------------------------------------------------- strings

pub fn str_bit_and(a: &Str, b: &Str) -> Str {
    Str::from_vec(a.iter().zip(b.iter()).map(|(x, y)| x & y).collect())
}
pub fn str_bit_or(a: &Str, b: &Str) -> Str {
    let n = a.len().max(b.len());
    Str::from_vec((0..n).map(|i| a.get(i).copied().unwrap_or(0) | b.get(i).copied().unwrap_or(0)).collect())
}
pub fn str_bit_xor(a: &Str, b: &Str) -> Str {
    Str::from_vec(a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect())
}
pub fn str_index_opt(s: &Str, i: i64) -> Option<Str> {
    crate::ops::str_index_byte(s, i).map(|b| Str::from_vec(vec![b]))
}
pub fn list_unset<T: Clone>(l: &mut List<T>, i: i64) {
    if i >= 0 && (i as usize) < l.len() {
        l.remove(i as usize);
    }
}

pub fn json_encode_simple(m: &Mixed) -> Str {
    crate::builtins::json::json_encode(m, 0, 512).ok().flatten().unwrap_or_default()
}

/// Drive a PHP `Iterator` object through its methods, collecting all (key, value) pairs.
pub fn iterate_php_iterator<K, V>(
    mut rewind: impl FnMut(),
    mut valid: impl FnMut() -> bool,
    mut next: impl FnMut(),
    mut key: impl FnMut() -> K,
    mut current: impl FnMut() -> V,
) -> std::vec::IntoIter<(K, V)> {
    let mut out = Vec::new();
    rewind();
    while valid() {
        out.push((key(), current()));
        next();
    }
    out.into_iter()
}

/// Iterate a PHP Iterator object by calling its methods through Mixed (dynamic fallback).
pub fn iterate_object<K, V>(_o: impl PhpObject) -> Result<std::vec::IntoIter<(K, V)>, RtError> {
    Err(RtError::error("iterate_object: dynamic iteration is not supported for this object"))
}

// ---------------------------------------------------------------- dynamic property access

/// Axis-7: a Sync interior-mutability cell (Arc<RwCell<T>> is Send+Sync so shared object storage can be
/// shared across scan/analyze threads). Drop-in for RefCell: `.borrow()`/`.borrow_mut()` keep RefCell's
/// fail-fast semantics via try_read/try_write (panic on contention, not deadlock), and the guards are
/// parking_lot MAPPED guards so `Ref::map`/`RefMut::map` in generated accessors keep working. The prelude
/// aliases RefCell->RwCell, Ref->CellRef, RefMut->CellRefMut, so generated code converts transparently.
pub type CellRef<'a, T> = parking_lot::MappedRwLockReadGuard<'a, T>;
pub type CellRefMut<'a, T> = parking_lot::MappedRwLockWriteGuard<'a, T>;
pub struct RwCell<T>(parking_lot::RwLock<T>);
impl<T> RwCell<T> {
    pub fn new(v: T) -> Self {
        RwCell(parking_lot::RwLock::new(v))
    }
    pub fn borrow(&self) -> CellRef<'_, T> {
        // Blocking read: concurrent readers proceed; blocks only while a writer holds the lock. (Generated
        // code is structured to avoid same-thread reentrant borrow-across-borrow_mut, so no self-deadlock.)
        parking_lot::RwLockReadGuard::map(self.0.read(), |x| x)
    }
    pub fn borrow_mut(&self) -> CellRefMut<'_, T> {
        // Blocking write: supports real concurrent mutation across threads (waits for contention instead of
        // panicking, which is what the multithreaded scan/analyze model needs).
        parking_lot::RwLockWriteGuard::map(self.0.write(), |x| x)
    }
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }
    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }
}
impl<T: Clone> Clone for RwCell<T> {
    fn clone(&self) -> Self {
        RwCell::new(self.borrow().clone())
    }
}
impl<T: Default> Default for RwCell<T> {
    fn default() -> Self {
        RwCell::new(T::default())
    }
}
impl<T: std::fmt::Debug> std::fmt::Debug for RwCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.borrow().fmt(f)
    }
}

/// A property read through a dispatch enum: borrowed from a known object, or a copy fetched
/// dynamically (`get_prop`) from an object of a class defined in another crate.
pub enum PropRef<'a, T> {
    Borrowed(CellRef<'a, T>),
    Owned(T),
}
impl<T> std::ops::Deref for PropRef<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            PropRef::Borrowed(r) => r,
            PropRef::Owned(v) => v,
        }
    }
}

/// A mutable property access through a dispatch enum; the owned form hands the value to a
/// write-back closure (`set_prop`) when dropped.
pub enum PropMut<'a, T> {
    Borrowed(CellRefMut<'a, T>),
    Owned { value: Option<T>, write: Option<Box<dyn FnOnce(T) + 'a>> },
}
impl<'a, T> PropMut<'a, T> {
    pub fn owned(value: T, write: Box<dyn FnOnce(T) + 'a>) -> Self {
        PropMut::Owned { value: Some(value), write: Some(write) }
    }
}
impl<T> std::ops::Deref for PropMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            PropMut::Borrowed(r) => r,
            PropMut::Owned { value, .. } => value.as_ref().unwrap(),
        }
    }
}
impl<T> std::ops::DerefMut for PropMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            PropMut::Borrowed(r) => r,
            PropMut::Owned { value, .. } => value.as_mut().unwrap(),
        }
    }
}
impl<T> Drop for PropMut<'_, T> {
    fn drop(&mut self) {
        if let PropMut::Owned { value, write } = self {
            if let (Some(v), Some(w)) = (value.take(), write.take()) {
                w(v);
            }
        }
    }
}

/// Property `name` of an object held in a `Mixed` (an instance of a class from another crate).
pub fn dyn_prop(m: &Mixed, name: &str) -> Mixed {
    other_obj(m).get_prop(name).unwrap_or(Mixed::Null)
}
