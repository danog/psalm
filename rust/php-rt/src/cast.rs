//! Type conversions between Rust representations of PHP types.
//! Generated code adds concrete impls for its own types; the runtime provides
//! impls among runtime types.

use crate::key::{ArrayKey, MapKey};
use crate::list::List;
use crate::map::Map;
use crate::mixed::{AnyObj, Mixed};
use crate::string::Str;

pub trait CastTo<T> {
    fn cast_to(self) -> T;
}

#[inline]
pub fn cast<T>(f: impl CastTo<T>) -> T {
    f.cast_to()
}

macro_rules! identity_cast {
    ($($t:ty),*) => { $(impl CastTo<$t> for $t { #[inline] fn cast_to(self) -> $t { self } })* };
}
identity_cast!(Mixed, i64, f64, bool, Str, ArrayKey, (), AnyObj);

impl CastTo<f64> for i64 {
    fn cast_to(self) -> f64 {
        self as f64
    }
}
impl CastTo<i64> for f64 {
    fn cast_to(self) -> i64 {
        crate::conv::float_to_int(self)
    }
}
impl CastTo<ArrayKey> for i64 {
    fn cast_to(self) -> ArrayKey {
        ArrayKey::Int(self)
    }
}
impl CastTo<ArrayKey> for f64 {
    fn cast_to(self) -> ArrayKey {
        ArrayKey::Int(crate::conv::float_to_int(self))
    }
}
impl CastTo<ArrayKey> for bool {
    fn cast_to(self) -> ArrayKey {
        ArrayKey::Int(if self { 1 } else { 0 })
    }
}
impl CastTo<ArrayKey> for () {
    fn cast_to(self) -> ArrayKey {
        ArrayKey::from_str_val(Str::from_static(""))
    }
}
impl CastTo<ArrayKey> for Str {
    fn cast_to(self) -> ArrayKey {
        ArrayKey::from_str_val(self)
    }
}
impl CastTo<Str> for ArrayKey {
    fn cast_to(self) -> Str {
        self.to_str()
    }
}
impl CastTo<i64> for ArrayKey {
    fn cast_to(self) -> i64 {
        crate::traits::ToInt::to_php_int(&self)
    }
}
impl CastTo<Str> for i64 {
    fn cast_to(self) -> Str {
        Str::from_string(self.to_string())
    }
}
impl CastTo<Str> for f64 {
    fn cast_to(self) -> Str {
        Str::from_string(crate::conv::float_to_string(self))
    }
}
impl CastTo<i64> for Str {
    fn cast_to(self) -> i64 {
        crate::conv::str_to_int(self.as_bytes())
    }
}
impl CastTo<f64> for Str {
    fn cast_to(self) -> f64 {
        crate::conv::str_to_float(self.as_bytes())
    }
}
impl CastTo<bool> for Str {
    fn cast_to(self) -> bool {
        crate::traits::Truthy::truthy(&self)
    }
}
impl CastTo<f64> for bool {
    fn cast_to(self) -> f64 {
        if self { 1.0 } else { 0.0 }
    }
}
impl CastTo<i64> for bool {
    fn cast_to(self) -> i64 {
        self as i64
    }
}
impl CastTo<bool> for i64 {
    fn cast_to(self) -> bool {
        self != 0
    }
}
impl CastTo<Str> for bool {
    fn cast_to(self) -> Str {
        crate::traits::ToStr::to_php_str(&self)
    }
}

// --- into Mixed
macro_rules! into_mixed {
    ($($t:ty),*) => { $(impl CastTo<Mixed> for $t { fn cast_to(self) -> Mixed { Mixed::from(self) } })* };
}
into_mixed!(i64, f64, bool, Str, ArrayKey, (), AnyObj);
impl<T: CastTo<Mixed>> CastTo<Mixed> for Option<T> {
    fn cast_to(self) -> Mixed {
        match self {
            None => Mixed::Null,
            Some(v) => v.cast_to(),
        }
    }
}
impl<T: CastTo<Mixed> + Clone> CastTo<Mixed> for List<T> {
    fn cast_to(self) -> Mixed {
        Mixed::Arr(self.into_iter().enumerate().map(|(i, v)| (ArrayKey::Int(i as i64), v.cast_to())).collect())
    }
}
impl<K: MapKey, V: CastTo<Mixed> + Clone> CastTo<Mixed> for Map<K, V> {
    fn cast_to(self) -> Mixed {
        Mixed::Arr(self.into_iter().map(|(k, v)| (k.to_array_key(), v.cast_to())).collect())
    }
}

// --- out of Mixed: PHP's scalar juggling where a scalar was declared (docblocks may be imprecise);
// panics only when a scalar was declared and a container or object shows up
fn mismatch(expected: &str, got: &Mixed) -> ! {
    panic!("Mixed value narrowed to {} but was {:?}", expected, got)
}
impl CastTo<i64> for Mixed {
    fn cast_to(self) -> i64 {
        match self {
            Mixed::Int(i) => i,
            Mixed::Bool(b) => b as i64,
            Mixed::Float(f) => crate::conv::float_to_int(f),
            Mixed::Null => 0,
            Mixed::Str(s) => crate::conv::str_to_int(s.as_bytes()),
            m => mismatch("int", &m),
        }
    }
}
impl CastTo<f64> for Mixed {
    fn cast_to(self) -> f64 {
        match self {
            Mixed::Float(f) => f,
            Mixed::Int(i) => i as f64,
            Mixed::Bool(b) => if b { 1.0 } else { 0.0 },
            Mixed::Null => 0.0,
            Mixed::Str(s) => crate::conv::str_to_float(s.as_bytes()),
            m => mismatch("float", &m),
        }
    }
}
impl CastTo<bool> for Mixed {
    fn cast_to(self) -> bool {
        match self {
            Mixed::Bool(b) => b,
            Mixed::Obj(_) => true,
            m => crate::traits::Truthy::truthy(&m),
        }
    }
}
impl CastTo<Str> for Mixed {
    fn cast_to(self) -> Str {
        match self {
            Mixed::Str(s) => s,
            Mixed::Null => Str::from_static(""),
            Mixed::Bool(b) => Str::from_static(if b { "1" } else { "" }),
            Mixed::Int(i) => Str::from_string(i.to_string()),
            Mixed::Float(f) => Str::from_string(crate::conv::float_to_string(f)),
            Mixed::Obj(o) => match o.php_to_string() {
                Some(s) => s,
                None => panic!("Object of class {} could not be converted to string", o.class_name()),
            },
            m => mismatch("string", &m),
        }
    }
}
impl CastTo<()> for Mixed {
    fn cast_to(self) {
        match self {
            Mixed::Null => (),
            m => mismatch("null", &m),
        }
    }
}
impl CastTo<ArrayKey> for Mixed {
    fn cast_to(self) -> ArrayKey {
        match self {
            Mixed::Int(i) => ArrayKey::Int(i),
            Mixed::Str(s) => ArrayKey::from_str_val(s),
            Mixed::Bool(b) => ArrayKey::Int(b as i64),
            Mixed::Float(f) => ArrayKey::Int(crate::conv::float_to_int(f)),
            Mixed::Null => ArrayKey::from_str_val(Str::from_static("")),
            m => mismatch("array-key", &m),
        }
    }
}
impl CastTo<AnyObj> for Mixed {
    fn cast_to(self) -> AnyObj {
        match self {
            Mixed::Obj(o) => o,
            m => mismatch("object", &m),
        }
    }
}
impl<T> CastTo<Option<T>> for Mixed
where
    Mixed: CastTo<T>,
{
    fn cast_to(self) -> Option<T> {
        match self {
            Mixed::Null => None,
            m => Some(m.cast_to()),
        }
    }
}
impl<T> CastTo<List<T>> for Mixed
where
    Mixed: CastTo<T>,
{
    fn cast_to(self) -> List<T> {
        match self {
            Mixed::Arr(a) => a.into_iter().map(|(_, v)| v.cast_to()).collect(),
            m => mismatch("list", &m),
        }
    }
}
impl<K: MapKey, V> CastTo<Map<K, V>> for Mixed
where
    Mixed: CastTo<V>,
    V: Clone,
{
    fn cast_to(self) -> Map<K, V> {
        match self {
            Mixed::Arr(a) => a.into_iter().map(|(k, v)| (K::from_array_key(k), v.cast_to())).collect(),
            m => mismatch("array", &m),
        }
    }
}

// --- containers
impl<T: Clone> CastTo<Map<i64, T>> for List<T> {
    fn cast_to(self) -> Map<i64, T> {
        self.into_iter().enumerate().map(|(i, v)| (i as i64, v)).collect()
    }
}
impl<T: Clone> CastTo<Map<ArrayKey, T>> for List<T> {
    fn cast_to(self) -> Map<ArrayKey, T> {
        self.into_iter().enumerate().map(|(i, v)| (ArrayKey::Int(i as i64), v)).collect()
    }
}
impl<T: Clone> CastTo<List<T>> for Map<i64, T> {
    fn cast_to(self) -> List<T> {
        self.into_iter().map(|(_, v)| v).collect()
    }
}
impl<T: Clone> CastTo<List<T>> for Map<ArrayKey, T> {
    fn cast_to(self) -> List<T> {
        self.into_iter().map(|(_, v)| v).collect()
    }
}
impl<T: Clone> CastTo<List<T>> for Map<Str, T> {
    fn cast_to(self) -> List<T> {
        self.into_iter().map(|(_, v)| v).collect()
    }
}
impl<T: Clone> CastTo<Map<ArrayKey, T>> for Map<i64, T> {
    fn cast_to(self) -> Map<ArrayKey, T> {
        self.into_iter().map(|(k, v)| (ArrayKey::Int(k), v)).collect()
    }
}
impl<T: Clone> CastTo<Map<ArrayKey, T>> for Map<Str, T> {
    fn cast_to(self) -> Map<ArrayKey, T> {
        self.into_iter().map(|(k, v)| (ArrayKey::from_str_val(k), v)).collect()
    }
}
impl<T: Clone> CastTo<Map<i64, T>> for Map<ArrayKey, T> {
    fn cast_to(self) -> Map<i64, T> {
        self.into_iter().map(|(k, v)| (i64::from_array_key(k), v)).collect()
    }
}
impl<T: Clone> CastTo<Map<Str, T>> for Map<ArrayKey, T> {
    fn cast_to(self) -> Map<Str, T> {
        self.into_iter().map(|(k, v)| (k.to_str(), v)).collect()
    }
}
impl<T: Clone> CastTo<Map<Str, T>> for Map<i64, T> {
    fn cast_to(self) -> Map<Str, T> {
        self.into_iter().map(|(k, v)| (Str::from_string(k.to_string()), v)).collect()
    }
}
impl<T: Clone> CastTo<Map<i64, T>> for Map<Str, T> {
    fn cast_to(self) -> Map<i64, T> {
        self.into_iter().map(|(k, v)| (i64::from_array_key(ArrayKey::from_str_val(k)), v)).collect()
    }
}

// ---------------------------------------------------------------- typed closures <-> Mixed
//
// A typed closure (`Rc<dyn Fn(A..) -> Result<R, E>>`, the Rust form of a PHP closure with a known
// signature) stored in a container that is converted to/from `Mixed` (dynamic arguments, property
// tables) is wrapped into / unwrapped from a `DynCallable`.
macro_rules! closure_casts {
    ($n:expr $(, $a:ident)*) => {
        impl<R, E $(, $a)*> CastTo<Mixed> for std::sync::Arc<dyn Fn($($a),*) -> Result<R, E> + Send + Sync>
        where
            R: CastTo<Mixed> + 'static,
            E: CastTo<Mixed> + 'static,
            $($a: 'static, Mixed: CastTo<$a>,)*
        {
            fn cast_to(self) -> Mixed {
                let f = self.clone();
                crate::containers::DynCallable::new($n, move |a: Vec<Mixed>| -> Mixed {
                    let mut it = a.into_iter();
                    $(let $a: $a = cast::<$a>(it.next().unwrap_or(Mixed::Null));)*
                    cast::<Mixed>(f($($a),*).unwrap_or_else(|_| panic!("Uncaught exception in callable")))
                })
                .cast_to()
            }
        }
        impl<R, E $(, $a)*> CastTo<std::sync::Arc<dyn Fn($($a),*) -> Result<R, E> + Send + Sync>> for Mixed
        where
            R: 'static,
            Mixed: CastTo<R>,
            E: From<crate::containers::DynError> + 'static,
            $($a: CastTo<Mixed> + 'static,)*
        {
            fn cast_to(self) -> std::sync::Arc<dyn Fn($($a),*) -> Result<R, E> + Send + Sync> {
                let c = crate::containers::to_callable(&self);
                std::sync::Arc::new(move |$($a: $a),*| -> Result<R, E> {
                    Ok(cast::<R>(c.call(vec![$(cast::<Mixed>($a)),*])))
                })
            }
        }
    };
}
closure_casts!(0);
closure_casts!(1, A1);
closure_casts!(2, A1, A2);
closure_casts!(3, A1, A2, A3);
closure_casts!(4, A1, A2, A3, A4);
closure_casts!(5, A1, A2, A3, A4, A5);
closure_casts!(6, A1, A2, A3, A4, A5, A6);


/// The uniform bound of a Rust generic parameter standing for a PHP `@template T`: every PHP value type
/// (scalars, containers, generated class handles/unions/shapes) implements these, so a generic function can
/// compare, print, test and clone its arguments without a dynamically typed representation.
pub trait PhpValue:
    Clone
    + std::fmt::Debug
    + crate::traits::Identical
    + crate::traits::PhpCmp
    + crate::traits::Truthy
    + crate::traits::ToStr
    + crate::traits::PhpKind
    + crate::traits::InstanceOfName
    + Send
    + Sync
    + 'static
{
}
impl<T> PhpValue for T where
    T: Clone
        + std::fmt::Debug
        + crate::traits::Identical
        + crate::traits::PhpCmp
        + crate::traits::Truthy
        + crate::traits::ToStr
        + crate::traits::PhpKind
        + crate::traits::InstanceOfName
        + Send
        + Sync
        + 'static
{
}

/// A generic value narrowed by the analysis to a concrete type (`is_string($x)` on a `T`): the static type is
/// recovered by downcasting the generic (the analysis proved the branch).
pub fn gcast<To: 'static>(v: impl std::any::Any) -> To {
    let b: Box<dyn std::any::Any> = Box::new(v);
    match b.downcast::<To>() {
        Ok(x) => *x,
        Err(_) => panic!("generic value is not of the narrowed type {}", std::any::type_name::<To>()),
    }
}
