//! PHP semantic traits implemented for runtime types (generated code implements them for classes).

use crate::conv::{self, Num};
use crate::key::{ArrayKey, MapKey};
use crate::list::List;
use crate::map::Map;
use crate::mixed::Mixed;
use crate::string::Str;
use std::cmp::Ordering;

// ---------------------------------------------------------------- truthiness

pub trait Truthy {
    fn truthy(&self) -> bool;
}
impl Truthy for bool {
    #[inline]
    fn truthy(&self) -> bool {
        *self
    }
}
impl Truthy for i64 {
    #[inline]
    fn truthy(&self) -> bool {
        *self != 0
    }
}
impl Truthy for f64 {
    #[inline]
    fn truthy(&self) -> bool {
        *self != 0.0
    }
}
impl Truthy for Str {
    #[inline]
    fn truthy(&self) -> bool {
        let b = self.as_bytes();
        !(b.is_empty() || b == b"0")
    }
}
impl Truthy for () {
    fn truthy(&self) -> bool {
        false
    }
}
impl<T: Truthy> Truthy for Option<T> {
    fn truthy(&self) -> bool {
        match self {
            None => false,
            Some(v) => v.truthy(),
        }
    }
}
impl<T> Truthy for List<T> {
    fn truthy(&self) -> bool {
        !self.is_empty()
    }
}
impl<K: MapKey, V> Truthy for Map<K, V> {
    fn truthy(&self) -> bool {
        !self.is_empty()
    }
}
impl Truthy for ArrayKey {
    fn truthy(&self) -> bool {
        match self {
            ArrayKey::Int(i) => *i != 0,
            ArrayKey::Str(s) => s.truthy(),
        }
    }
}
impl Truthy for Mixed {
    fn truthy(&self) -> bool {
        match self {
            Mixed::Null => false,
            Mixed::Bool(b) => *b,
            Mixed::Int(i) => *i != 0,
            Mixed::Float(f) => *f != 0.0,
            Mixed::Str(s) => s.truthy(),
            Mixed::Arr(a) => !a.is_empty(),
            Mixed::Obj(_) | Mixed::Closure(_) => true,
        }
    }
}
impl<T: Truthy + ?Sized> Truthy for &T {
    fn truthy(&self) -> bool {
        (**self).truthy()
    }
}
impl<T: Truthy> Truthy for crate::late::Late<T> {
    fn truthy(&self) -> bool {
        self.get().truthy()
    }
}

#[inline]
pub fn truthy<T: Truthy>(v: T) -> bool {
    v.truthy()
}

// ---------------------------------------------------------------- string conversion

pub trait ToStr {
    fn to_php_str(&self) -> Str;
}
impl ToStr for Str {
    #[inline]
    fn to_php_str(&self) -> Str {
        self.clone()
    }
}
impl ToStr for i64 {
    fn to_php_str(&self) -> Str {
        Str::from_string(self.to_string())
    }
}
impl ToStr for f64 {
    fn to_php_str(&self) -> Str {
        Str::from_string(conv::float_to_string(*self))
    }
}
impl ToStr for bool {
    fn to_php_str(&self) -> Str {
        if *self { Str::from_static("1") } else { Str::empty() }
    }
}
impl ToStr for () {
    fn to_php_str(&self) -> Str {
        Str::empty()
    }
}
impl<T: ToStr> ToStr for Option<T> {
    fn to_php_str(&self) -> Str {
        match self {
            None => Str::empty(),
            Some(v) => v.to_php_str(),
        }
    }
}
impl ToStr for ArrayKey {
    fn to_php_str(&self) -> Str {
        self.to_str()
    }
}
impl ToStr for Mixed {
    fn to_php_str(&self) -> Str {
        match self {
            Mixed::Null => Str::empty(),
            Mixed::Bool(b) => b.to_php_str(),
            Mixed::Int(i) => i.to_php_str(),
            Mixed::Float(f) => f.to_php_str(),
            Mixed::Str(s) => s.clone(),
            Mixed::Arr(_) => Str::from_static("Array"),
            Mixed::Obj(o) => o.php_to_string().unwrap_or_else(|| Str::from_str(o.class_name())),
            Mixed::Closure(_) => Str::from_static("Closure"),
        }
    }
}
impl ToStr for &str {
    fn to_php_str(&self) -> Str {
        Str::from_str(self)
    }
}
impl ToStr for String {
    fn to_php_str(&self) -> Str {
        Str::from_str(self)
    }
}
impl<T: ToStr + ?Sized> ToStr for &T {
    fn to_php_str(&self) -> Str {
        (**self).to_php_str()
    }
}
impl<T: ToStr> ToStr for crate::late::Late<T> {
    fn to_php_str(&self) -> Str {
        self.get().to_php_str()
    }
}
impl<T> ToStr for List<T> {
    fn to_php_str(&self) -> Str {
        Str::from_static("Array")
    }
}
impl<K: MapKey, V> ToStr for Map<K, V> {
    fn to_php_str(&self) -> Str {
        Str::from_static("Array")
    }
}

#[inline]
pub fn to_str<T: ToStr>(v: T) -> Str {
    v.to_php_str()
}

// ---------------------------------------------------------------- casts

pub trait ToInt {
    fn to_php_int(&self) -> i64;
}
impl ToInt for i64 {
    fn to_php_int(&self) -> i64 {
        *self
    }
}
impl ToInt for f64 {
    fn to_php_int(&self) -> i64 {
        conv::float_to_int(*self)
    }
}
impl ToInt for bool {
    fn to_php_int(&self) -> i64 {
        *self as i64
    }
}
impl ToInt for Str {
    fn to_php_int(&self) -> i64 {
        conv::str_to_int(self.as_bytes())
    }
}
impl ToInt for () {
    fn to_php_int(&self) -> i64 {
        0
    }
}
impl<T: ToInt> ToInt for Option<T> {
    fn to_php_int(&self) -> i64 {
        match self {
            None => 0,
            Some(v) => v.to_php_int(),
        }
    }
}
impl ToInt for ArrayKey {
    fn to_php_int(&self) -> i64 {
        match self {
            ArrayKey::Int(i) => *i,
            ArrayKey::Str(s) => s.to_php_int(),
        }
    }
}
impl ToInt for Mixed {
    fn to_php_int(&self) -> i64 {
        match self {
            Mixed::Null => 0,
            Mixed::Bool(b) => *b as i64,
            Mixed::Int(i) => *i,
            Mixed::Float(f) => conv::float_to_int(*f),
            Mixed::Str(s) => s.to_php_int(),
            Mixed::Arr(a) => !a.is_empty() as i64,
            Mixed::Obj(_) | Mixed::Closure(_) => 1,
        }
    }
}
impl<T> ToInt for List<T> {
    fn to_php_int(&self) -> i64 {
        !self.is_empty() as i64
    }
}
impl<K: MapKey, V> ToInt for Map<K, V> {
    fn to_php_int(&self) -> i64 {
        !self.is_empty() as i64
    }
}
impl<T: ToInt + ?Sized> ToInt for &T {
    fn to_php_int(&self) -> i64 {
        (**self).to_php_int()
    }
}
#[inline]
pub fn to_int<T: ToInt>(v: T) -> i64 {
    v.to_php_int()
}

pub trait ToFloat {
    fn to_php_float(&self) -> f64;
}
impl ToFloat for f64 {
    fn to_php_float(&self) -> f64 {
        *self
    }
}
impl ToFloat for i64 {
    fn to_php_float(&self) -> f64 {
        *self as f64
    }
}
impl ToFloat for bool {
    fn to_php_float(&self) -> f64 {
        *self as i64 as f64
    }
}
impl ToFloat for Str {
    fn to_php_float(&self) -> f64 {
        conv::str_to_float(self.as_bytes())
    }
}
impl ToFloat for () {
    fn to_php_float(&self) -> f64 {
        0.0
    }
}
impl<T: ToFloat> ToFloat for Option<T> {
    fn to_php_float(&self) -> f64 {
        match self {
            None => 0.0,
            Some(v) => v.to_php_float(),
        }
    }
}
impl ToFloat for ArrayKey {
    fn to_php_float(&self) -> f64 {
        match self {
            ArrayKey::Int(i) => *i as f64,
            ArrayKey::Str(s) => s.to_php_float(),
        }
    }
}
impl ToFloat for Mixed {
    fn to_php_float(&self) -> f64 {
        match self {
            Mixed::Null => 0.0,
            Mixed::Bool(b) => *b as i64 as f64,
            Mixed::Int(i) => *i as f64,
            Mixed::Float(f) => *f,
            Mixed::Str(s) => s.to_php_float(),
            Mixed::Arr(a) => !a.is_empty() as i64 as f64,
            Mixed::Obj(_) | Mixed::Closure(_) => 1.0,
        }
    }
}
impl<T: ToFloat + ?Sized> ToFloat for &T {
    fn to_php_float(&self) -> f64 {
        (**self).to_php_float()
    }
}
#[inline]
pub fn to_float<T: ToFloat>(v: T) -> f64 {
    v.to_php_float()
}

pub trait ToArrayKey {
    fn to_php_key(&self) -> ArrayKey;
}
impl ToArrayKey for i64 {
    fn to_php_key(&self) -> ArrayKey {
        ArrayKey::Int(*self)
    }
}
impl ToArrayKey for Str {
    fn to_php_key(&self) -> ArrayKey {
        ArrayKey::from_str_val(self.clone())
    }
}
impl ToArrayKey for bool {
    fn to_php_key(&self) -> ArrayKey {
        ArrayKey::Int(*self as i64)
    }
}
impl ToArrayKey for f64 {
    fn to_php_key(&self) -> ArrayKey {
        ArrayKey::Int(conv::float_to_int(*self))
    }
}
impl ToArrayKey for ArrayKey {
    fn to_php_key(&self) -> ArrayKey {
        self.clone()
    }
}
impl<T: ToArrayKey> ToArrayKey for Option<T> {
    fn to_php_key(&self) -> ArrayKey {
        match self {
            None => ArrayKey::Str(Str::empty()),
            Some(v) => v.to_php_key(),
        }
    }
}
impl ToArrayKey for Mixed {
    fn to_php_key(&self) -> ArrayKey {
        match self {
            Mixed::Null => ArrayKey::Str(Str::empty()),
            Mixed::Bool(b) => ArrayKey::Int(*b as i64),
            Mixed::Int(i) => ArrayKey::Int(*i),
            Mixed::Float(f) => ArrayKey::Int(conv::float_to_int(*f)),
            Mixed::Str(s) => ArrayKey::from_str_val(s.clone()),
            _ => panic!("Illegal offset type"),
        }
    }
}
impl<T: ToArrayKey + ?Sized> ToArrayKey for &T {
    fn to_php_key(&self) -> ArrayKey {
        (**self).to_php_key()
    }
}
#[inline]
pub fn to_key<T: ToArrayKey>(v: T) -> ArrayKey {
    v.to_php_key()
}

// ---------------------------------------------------------------- identity (===)

pub trait Identical {
    fn identical(&self, other: &Self) -> bool;
}
macro_rules! identical_prim {
    ($($t:ty),*) => { $(impl Identical for $t { #[inline] fn identical(&self, o: &Self) -> bool { self == o } })* };
}
identical_prim!(bool, i64, Str, ArrayKey, ());
impl Identical for f64 {
    fn identical(&self, o: &Self) -> bool {
        self == o
    }
}
impl<T: Identical> Identical for Option<T> {
    fn identical(&self, o: &Self) -> bool {
        match (self, o) {
            (None, None) => true,
            (Some(a), Some(b)) => a.identical(b),
            _ => false,
        }
    }
}
impl<T: Identical> Identical for List<T> {
    fn identical(&self, o: &Self) -> bool {
        self.ptr_eq(o) || (self.len() == o.len() && self.iter().zip(o.iter()).all(|(a, b)| a.identical(b)))
    }
}
impl<K: MapKey, V: Identical> Identical for Map<K, V> {
    fn identical(&self, o: &Self) -> bool {
        self.ptr_eq(o)
            || (self.len() == o.len() && self.iter().zip(o.iter()).all(|((k1, v1), (k2, v2))| k1 == k2 && v1.identical(v2)))
    }
}
impl Identical for Mixed {
    fn identical(&self, o: &Self) -> bool {
        match (self, o) {
            (Mixed::Null, Mixed::Null) => true,
            (Mixed::Bool(a), Mixed::Bool(b)) => a == b,
            (Mixed::Int(a), Mixed::Int(b)) => a == b,
            (Mixed::Float(a), Mixed::Float(b)) => a == b,
            (Mixed::Str(a), Mixed::Str(b)) => a == b,
            (Mixed::Arr(a), Mixed::Arr(b)) => a.identical(b),
            (Mixed::Obj(a), Mixed::Obj(b)) => a.obj_id() == b.obj_id(),
            (Mixed::Closure(a), Mixed::Closure(b)) => std::sync::Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}
impl<T: Identical> Identical for crate::late::Late<T> {
    fn identical(&self, o: &Self) -> bool {
        self.get().identical(o.get())
    }
}
macro_rules! identical_tuple {
    ($($n:tt $T:ident),+) => {
        impl<$($T: Identical),+> Identical for ($($T,)+) {
            fn identical(&self, o: &Self) -> bool { $(self.$n.identical(&o.$n))&&+ }
        }
    };
}
identical_tuple!(0 A);
identical_tuple!(0 A, 1 B);
identical_tuple!(0 A, 1 B, 2 C);
identical_tuple!(0 A, 1 B, 2 C, 3 D);
identical_tuple!(0 A, 1 B, 2 C, 3 D, 4 E);
identical_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F);
identical_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G);
identical_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H);
identical_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I);
identical_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J);
identical_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K);
identical_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K, 11 L);

#[inline]
pub fn identical<T: Identical>(a: &T, b: &T) -> bool {
    a.identical(b)
}

// ---------------------------------------------------------------- loose equality (==) and ordering

/// PHP 8 `<=>` / `==` between two PHP scalar values.
pub fn cmp_num(a: Num, b: Num) -> Ordering {
    match (a, b) {
        (Num::Int(x), Num::Int(y)) => x.cmp(&y),
        _ => a.to_f64().partial_cmp(&b.to_f64()).unwrap_or(Ordering::Equal),
    }
}

/// PHP string comparison (`"10" == "1e1"` is true; non-numeric strings compare bytewise).
pub fn cmp_str(a: &[u8], b: &[u8]) -> Ordering {
    if let (Some(x), Some(y)) = (conv::parse_numeric(a), conv::parse_numeric(b)) {
        return cmp_num(x, y);
    }
    a.cmp(b)
}
pub fn str_loose_eq(a: &[u8], b: &[u8]) -> bool {
    cmp_str(a, b) == Ordering::Equal
}
/// int/float vs string comparison (PHP 8 semantics).
pub fn cmp_num_str(n: Num, s: &[u8]) -> Ordering {
    match conv::parse_numeric(s) {
        Some(m) => cmp_num(n, m),
        None => {
            let ns = match n {
                Num::Int(i) => i.to_string(),
                Num::Float(f) => conv::float_to_string(f),
            };
            ns.as_bytes().cmp(s)
        }
    }
}

pub trait PhpCmp {
    fn php_cmp(&self, other: &Self) -> Ordering;
    fn loose_eq(&self, other: &Self) -> bool {
        self.php_cmp(other) == Ordering::Equal
    }
}
impl PhpCmp for i64 {
    fn php_cmp(&self, o: &Self) -> Ordering {
        self.cmp(o)
    }
}
impl PhpCmp for f64 {
    fn php_cmp(&self, o: &Self) -> Ordering {
        self.partial_cmp(o).unwrap_or(Ordering::Equal)
    }
}
impl PhpCmp for bool {
    fn php_cmp(&self, o: &Self) -> Ordering {
        self.cmp(o)
    }
}
impl PhpCmp for Str {
    fn php_cmp(&self, o: &Self) -> Ordering {
        cmp_str(self.as_bytes(), o.as_bytes())
    }
    fn loose_eq(&self, o: &Self) -> bool {
        let (a, b) = (self.as_bytes(), o.as_bytes());
        if a == b {
            return true;
        }
        // PHP 8: two strings compare numerically only when both are numeric strings
        let numeric_start = |s: &[u8]| s.first().is_some_and(|c| c.is_ascii_digit() || matches!(c, b' ' | b'\t' | b'\n' | b'\r' | b'+' | b'-' | b'.'));
        if !numeric_start(a) || !numeric_start(b) {
            return false;
        }
        cmp_str(a, b) == Ordering::Equal
    }
}
impl PhpCmp for () {
    fn php_cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}
impl PhpCmp for ArrayKey {
    fn php_cmp(&self, o: &Self) -> Ordering {
        match (self, o) {
            (ArrayKey::Int(a), ArrayKey::Int(b)) => a.cmp(b),
            (ArrayKey::Str(a), ArrayKey::Str(b)) => cmp_str(a, b),
            (ArrayKey::Int(a), ArrayKey::Str(b)) => cmp_num_str(Num::Int(*a), b),
            (ArrayKey::Str(a), ArrayKey::Int(b)) => cmp_num_str(Num::Int(*b), a).reverse(),
        }
    }
}
impl<T: PhpCmp + Truthy> PhpCmp for Option<T> {
    fn php_cmp(&self, o: &Self) -> Ordering {
        match (self, o) {
            (None, None) => Ordering::Equal,
            (None, Some(b)) => false.cmp(&b.truthy()),
            (Some(a), None) => a.truthy().cmp(&false),
            (Some(a), Some(b)) => a.php_cmp(b),
        }
    }
}
impl<T: PhpCmp> PhpCmp for List<T> {
    fn php_cmp(&self, o: &Self) -> Ordering {
        match self.len().cmp(&o.len()) {
            Ordering::Equal => {}
            c => return c,
        }
        for (a, b) in self.iter().zip(o.iter()) {
            match a.php_cmp(b) {
                Ordering::Equal => {}
                c => return c,
            }
        }
        Ordering::Equal
    }
}
impl<K: MapKey, V: PhpCmp> PhpCmp for Map<K, V> {
    fn php_cmp(&self, o: &Self) -> Ordering {
        match self.len().cmp(&o.len()) {
            Ordering::Equal => {}
            c => return c,
        }
        for (k, a) in self.iter() {
            match o.get(k) {
                None => return Ordering::Greater,
                Some(b) => match a.php_cmp(b) {
                    Ordering::Equal => {}
                    c => return c,
                },
            }
        }
        Ordering::Equal
    }
}
impl PhpCmp for Mixed {
    fn php_cmp(&self, o: &Self) -> Ordering {
        use Mixed::*;
        match (self, o) {
            (Null, Null) => Ordering::Equal,
            (Null, Str(b)) => Ordering::Equal.then(if b.is_empty() { Ordering::Equal } else { Ordering::Less }),
            (Str(a), Null) => if a.is_empty() { Ordering::Equal } else { Ordering::Greater },
            (Null, b) => false.cmp(&b.truthy()),
            (a, Null) => a.truthy().cmp(&false),
            (Bool(a), b) => a.cmp(&b.truthy()),
            (a, Bool(b)) => a.truthy().cmp(b),
            (Int(a), Int(b)) => a.cmp(b),
            (Int(a), Float(b)) => (*a as f64).partial_cmp(b).unwrap_or(Ordering::Equal),
            (Float(a), Int(b)) => a.partial_cmp(&(*b as f64)).unwrap_or(Ordering::Equal),
            (Float(a), Float(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
            (Int(a), Str(b)) => cmp_num_str(Num::Int(*a), b),
            (Float(a), Str(b)) => cmp_num_str(Num::Float(*a), b),
            (Str(a), Int(b)) => cmp_num_str(Num::Int(*b), a).reverse(),
            (Str(a), Float(b)) => cmp_num_str(Num::Float(*b), a).reverse(),
            (Str(a), Str(b)) => cmp_str(a, b),
            (Arr(a), Arr(b)) => a.php_cmp(b),
            (Arr(_), _) => Ordering::Greater,
            (_, Arr(_)) => Ordering::Less,
            (Obj(a), Obj(b)) => {
                if a.obj_id() == b.obj_id() {
                    Ordering::Equal
                } else if a.class_name() == b.class_name() {
                    let pa = a.props();
                    let pb = b.props();
                    let ma: Map<ArrayKey, Mixed> = pa.into_iter().map(|(k, v)| (ArrayKey::Str(k), v)).collect();
                    let mb: Map<ArrayKey, Mixed> = pb.into_iter().map(|(k, v)| (ArrayKey::Str(k), v)).collect();
                    ma.php_cmp(&mb)
                } else {
                    Ordering::Greater
                }
            }
            (Obj(a), Str(b)) => match a.php_to_string() {
                Some(s) => cmp_str(&s, b),
                None => Ordering::Greater,
            },
            (Str(a), Obj(b)) => match b.php_to_string() {
                Some(s) => cmp_str(a, &s),
                None => Ordering::Less,
            },
            (Obj(_), _) | (Closure(_), _) => Ordering::Greater,
            (_, Obj(_)) | (_, Closure(_)) => Ordering::Less,
        }
    }
}
impl<T: PhpCmp + ?Sized> PhpCmp for &T {
    fn php_cmp(&self, o: &Self) -> Ordering {
        (**self).php_cmp(*o)
    }
}
#[inline]
pub fn loose_eq<T: PhpCmp>(a: &T, b: &T) -> bool {
    a.loose_eq(b)
}
#[inline]
pub fn php_cmp<T: PhpCmp>(a: &T, b: &T) -> Ordering {
    a.php_cmp(b)
}
#[inline]
pub fn spaceship<T: PhpCmp>(a: &T, b: &T) -> i64 {
    match a.php_cmp(b) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}
#[inline]
pub fn php_lt<T: PhpCmp>(a: &T, b: &T) -> bool {
    a.php_cmp(b) == Ordering::Less
}
#[inline]
pub fn php_le<T: PhpCmp>(a: &T, b: &T) -> bool {
    a.php_cmp(b) != Ordering::Greater
}
#[inline]
pub fn php_gt<T: PhpCmp>(a: &T, b: &T) -> bool {
    a.php_cmp(b) == Ordering::Greater
}
#[inline]
pub fn php_ge<T: PhpCmp>(a: &T, b: &T) -> bool {
    a.php_cmp(b) != Ordering::Less
}

// ---------------------------------------------------------------- count()

pub trait Len {
    fn php_count(&self) -> i64;
}
impl<T> Len for List<T> {
    fn php_count(&self) -> i64 {
        self.len() as i64
    }
}
impl<K: MapKey, V> Len for Map<K, V> {
    fn php_count(&self) -> i64 {
        self.len() as i64
    }
}
impl<T: Len + ?Sized> Len for &T {
    fn php_count(&self) -> i64 {
        (**self).php_count()
    }
}
impl Len for Mixed {
    fn php_count(&self) -> i64 {
        match self {
            Mixed::Arr(a) => a.len() as i64,
            _ => 1,
        }
    }
}
#[inline]
pub fn count<T: Len>(v: T) -> i64 {
    v.php_count()
}

/// `clone $x` (generated for classes).
pub trait PhpClone {
    fn php_clone(&self) -> Self;
}

// ---------------------------------------------------------------- tuples

macro_rules! tuple_impls {
    ($($n:tt $T:ident),+) => {
        impl<$($T),+> Truthy for ($($T,)+) {
            fn truthy(&self) -> bool { true }
        }
        impl<$($T: ToStr),+> ToStr for ($($T,)+) {
            fn to_php_str(&self) -> Str { Str::from_static("Array") }
        }
        impl<$($T: crate::cast::CastTo<Mixed>),+> crate::cast::CastTo<Mixed> for ($($T,)+) {
            fn cast_to(self) -> Mixed {
                let mut m: Map<ArrayKey, Mixed> = Map::new();
                $( m.push(self.$n.cast_to()); )+
                Mixed::Arr(m)
            }
        }
        impl<$($T),+> crate::cast::CastTo<($($T,)+)> for Mixed where $(Mixed: crate::cast::CastTo<$T>),+ {
            fn cast_to(self) -> ($($T,)+) {
                let l: List<Mixed> = crate::cast::cast::<List<Mixed>>(self);
                ($( crate::cast::cast::<$T>(l.get($n).cloned().unwrap_or_default()), )+)
            }
        }
        impl<$($T: crate::cast::CastTo<Mixed> + Clone),+> PhpCmp for ($($T,)+) {
            fn php_cmp(&self, o: &Self) -> Ordering {
                crate::cast::cast::<Mixed>(self.clone()).php_cmp(&crate::cast::cast::<Mixed>(o.clone()))
            }
        }
        impl<$($T),+> Len for ($($T,)+) {
            fn php_count(&self) -> i64 { [$($n),+].len() as i64 }
        }
    };
}
tuple_impls!(0 A);
tuple_impls!(0 A, 1 B);
tuple_impls!(0 A, 1 B, 2 C);
tuple_impls!(0 A, 1 B, 2 C, 3 D);
tuple_impls!(0 A, 1 B, 2 C, 3 D, 4 E);
tuple_impls!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F);
tuple_impls!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G);
tuple_impls!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H);
tuple_impls!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I);
tuple_impls!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J);
tuple_impls!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K);
tuple_impls!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K, 11 L);
