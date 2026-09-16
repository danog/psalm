//! Runtime implementations of builtin generic classes and callable/resource values.

use crate::error::RtError;
use crate::key::{ArrayKey, MapKey};
use crate::list::List;
use crate::map::Map;
use crate::mixed::{AnyObj, Mixed, PhpObject};
use crate::string::Str;
use crate::traits::*;
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::support::RwCell;
use std::sync::{Arc as Rc, Weak};

// ---------------------------------------------------------------- Generator (eagerly evaluated)

pub struct GeneratorData<K, V> {
    pairs: Vec<(K, V)>,
    pos: AtomicUsize,
}

/// An eagerly-evaluated PHP generator / iterator over key-value pairs.
pub struct Generator<K, V>(Rc<GeneratorData<K, V>>);

impl<K, V> Clone for Generator<K, V> {
    fn clone(&self) -> Self {
        Generator(self.0.clone())
    }
}

impl<K: Clone, V: Clone> Generator<K, V> {
    pub fn from_pairs(pairs: Vec<(K, V)>) -> Self {
        Generator(Rc::new(GeneratorData { pairs, pos: AtomicUsize::new(0) }))
    }
    pub fn into_pairs(&self) -> Vec<(K, V)> {
        self.0.pairs.clone()
    }
    pub fn into_list(&self) -> List<V> {
        self.0.pairs.iter().map(|(_, v)| v.clone()).collect()
    }
    pub fn count(&self) -> i64 {
        self.0.pairs.len() as i64
    }
    pub fn current(&self) -> Option<V> {
        self.0.pairs.get(self.0.pos.load(Ordering::Relaxed)).map(|(_, v)| v.clone())
    }
    pub fn key(&self) -> Option<K> {
        self.0.pairs.get(self.0.pos.load(Ordering::Relaxed)).map(|(k, _)| k.clone())
    }
    pub fn next(&self) {
        self.0.pos.store(self.0.pos.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
    }
    pub fn rewind(&self) {
        self.0.pos.store(0, Ordering::Relaxed);
    }
    pub fn valid(&self) -> bool {
        self.0.pos.load(Ordering::Relaxed) < self.0.pairs.len()
    }
}
impl<K: MapKey, V: Clone> Generator<K, V> {
    pub fn into_map(&self) -> Map<K, V> {
        self.0.pairs.iter().cloned().collect()
    }
    pub fn from_map(m: Map<K, V>) -> Self {
        Self::from_pairs(m.into_iter().collect())
    }
}
impl<V: Clone> Generator<i64, V> {
    pub fn from_list(l: List<V>) -> Self {
        Self::from_pairs(l.into_iter().enumerate().map(|(i, v)| (i as i64, v)).collect())
    }
}
impl<V: Clone> Generator<ArrayKey, V> {
    pub fn from_list(l: List<V>) -> Self {
        Self::from_pairs(l.into_iter().enumerate().map(|(i, v)| (ArrayKey::Int(i as i64), v)).collect())
    }
}
impl<V: Clone> Generator<Mixed, V> {
    pub fn from_list(l: List<V>) -> Self {
        Self::from_pairs(l.into_iter().enumerate().map(|(i, v)| (Mixed::Int(i as i64), v)).collect())
    }
}
impl<K: Clone, V: Clone> crate::traits::PhpKind for Generator<K, V> { fn php_kind(&self) -> crate::traits::Kind { crate::traits::Kind::Obj } }
impl<K: Clone, V: Clone> crate::traits::InstanceOfName for Generator<K, V> { fn php_instance_of(&self, name: &[u8]) -> bool { name.eq_ignore_ascii_case(b"generator") || name.eq_ignore_ascii_case(b"traversable") || name.eq_ignore_ascii_case(b"iterator") } }
impl<K: Clone, V: Clone> Truthy for Generator<K, V> {
    fn truthy(&self) -> bool {
        true
    }
}
impl<K: Clone, V: Clone> Identical for Generator<K, V> {
    fn identical(&self, o: &Self) -> bool {
        Rc::ptr_eq(&self.0, &o.0)
    }
}
impl<K: Clone, V: Clone> Len for Generator<K, V> {
    fn php_count(&self) -> i64 {
        self.count()
    }
}
impl<K: Clone + crate::cast::CastTo<Mixed> + Send + Sync + 'static, V: Clone + crate::cast::CastTo<Mixed> + Send + Sync + 'static> PhpObject for Generator<K, V> {
    fn class_name(&self) -> &'static str {
        "Generator"
    }
    fn class_ancestors(&self) -> &'static [&'static str] {
        &["generator", "iterator", "traversable"]
    }
    fn obj_id(&self) -> usize {
        Rc::as_ptr(&self.0) as *const u8 as usize
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl<K: Clone + crate::cast::CastTo<Mixed> + Send + Sync + 'static, V: Clone + crate::cast::CastTo<Mixed> + Send + Sync + 'static> crate::cast::CastTo<Mixed> for Generator<K, V> {
    fn cast_to(self) -> Mixed {
        Mixed::Obj(Rc::new(self))
    }
}
impl<K: Clone + 'static, V: Clone + 'static> crate::cast::CastTo<Generator<K, V>> for Mixed
where
    Generator<K, V>: PhpObject,
    Mixed: crate::cast::CastTo<K> + crate::cast::CastTo<V>,
{
    fn cast_to(self) -> Generator<K, V> {
        if let Mixed::Obj(o) = &self {
            if let Some(g) = o.as_any().downcast_ref::<Generator<K, V>>() {
                return g.clone();
            }
            // any other Iterator object: drive it through its methods
            let call = |name: &str| -> Mixed {
                o.call_method(name, Vec::new())
            };
            let mut pairs = Vec::new();
            call("rewind");
            while call("valid").truthy() {
                pairs.push((crate::cast::cast::<K>(call("key")), crate::cast::cast::<V>(call("current"))));
                call("next");
            }
            return Generator::from_pairs(pairs);
        }
        if let Mixed::Arr(a) = self {
            return Generator::from_pairs(a.into_iter().map(|(k, v)| (crate::cast::cast::<K>(Mixed::from(k)), crate::cast::cast::<V>(v))).collect());
        }
        panic!("Mixed value is not a Generator")
    }
}
impl<K: Clone, V: Clone> std::fmt::Debug for Generator<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Generator({} items)", self.0.pairs.len())
    }
}

pub type PhpIterator<K, V> = Generator<K, V>;
pub type Traversable<K, V> = Generator<K, V>;
pub type IteratorAggregate<K, V> = Generator<K, V>;

// ---------------------------------------------------------------- ArrayObject / ArrayIterator

pub struct ArrayObject<K, V>(Rc<RwCell<Map<K, V>>>, AtomicUsize);

impl<K, V> Clone for ArrayObject<K, V> {
    fn clone(&self) -> Self {
        ArrayObject(self.0.clone(), AtomicUsize::new(self.1.load(Ordering::Relaxed)))
    }
}
impl<K: MapKey, V: Clone> ArrayObject<K, V> {
    pub fn new(m: Map<K, V>) -> Self {
        ArrayObject(Rc::new(RwCell::new(m)), AtomicUsize::new(0))
    }
    pub fn get_array_copy(&self) -> Map<K, V> {
        self.0.borrow().clone()
    }
    pub fn count(&self) -> i64 {
        self.0.borrow().len() as i64
    }
    pub fn get(&self, k: &K) -> Option<V> {
        self.0.borrow().get(k).cloned()
    }
    pub fn idx(&self, k: &K) -> V {
        self.0.borrow().get(k).cloned().unwrap_or_else(|| panic!("Undefined array key {:?}", k))
    }
    pub fn contains(&self, k: &K) -> bool {
        self.0.borrow().contains_key(k)
    }
    pub fn set(&self, k: K, v: V) {
        self.0.borrow_mut().insert(k, v);
    }
    pub fn append(&self, v: V) {
        self.0.borrow_mut().push(v);
    }
    pub fn remove(&self, k: &K) {
        self.0.borrow_mut().remove(k);
    }
    pub fn get_iterator(&self) -> ArrayIterator<K, V> {
        ArrayObject(self.0.clone(), AtomicUsize::new(0))
    }
    pub fn into_pairs(&self) -> Vec<(K, V)> {
        self.0.borrow().to_pairs()
    }
    pub fn current(&self) -> Option<V> {
        self.0.borrow().iter().nth(self.1.load(Ordering::Relaxed)).map(|(_, v)| v.clone())
    }
    pub fn key(&self) -> Option<K> {
        self.0.borrow().iter().nth(self.1.load(Ordering::Relaxed)).map(|(k, _)| k.clone())
    }
    pub fn next(&self) {
        self.1.store(self.1.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
    }
    pub fn rewind(&self) {
        self.1.store(0, Ordering::Relaxed);
    }
    pub fn valid(&self) -> bool {
        self.1.load(Ordering::Relaxed) < self.0.borrow().len()
    }
}
pub type ArrayIterator<K, V> = ArrayObject<K, V>;
impl<K: MapKey, V: Clone> crate::traits::PhpKind for ArrayObject<K, V> { fn php_kind(&self) -> crate::traits::Kind { crate::traits::Kind::Obj } }
impl<K: MapKey, V: Clone> crate::traits::InstanceOfName for ArrayObject<K, V> { fn php_instance_of(&self, name: &[u8]) -> bool { name.eq_ignore_ascii_case(b"arrayobject") || name.eq_ignore_ascii_case(b"traversable") || name.eq_ignore_ascii_case(b"countable") || name.eq_ignore_ascii_case(b"arrayaccess") || name.eq_ignore_ascii_case(b"iteratoraggregate") } }
impl<K: MapKey, V: Clone> Truthy for ArrayObject<K, V> {
    fn truthy(&self) -> bool {
        true
    }
}
impl<K: MapKey, V: Clone> Identical for ArrayObject<K, V> {
    fn identical(&self, o: &Self) -> bool {
        Rc::ptr_eq(&self.0, &o.0)
    }
}
impl<K: MapKey, V: Clone> Len for ArrayObject<K, V> {
    fn php_count(&self) -> i64 {
        self.count()
    }
}
impl<K: MapKey + Send + Sync + 'static, V: Clone + Send + Sync + 'static> PhpObject for ArrayObject<K, V> {
    fn class_name(&self) -> &'static str {
        "ArrayObject"
    }
    fn class_ancestors(&self) -> &'static [&'static str] {
        &["arrayobject", "arrayiterator", "iteratoraggregate", "traversable", "arrayaccess", "countable", "iterator"]
    }
    fn obj_id(&self) -> usize {
        Rc::as_ptr(&self.0) as *const u8 as usize
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl<K: MapKey + Send + Sync + 'static, V: Clone + Send + Sync + 'static> crate::cast::CastTo<Mixed> for ArrayObject<K, V> {
    fn cast_to(self) -> Mixed {
        Mixed::Obj(Rc::new(self))
    }
}
impl<K: MapKey + Send + Sync + 'static, V: Clone + Send + Sync + 'static> crate::cast::CastTo<ArrayObject<K, V>> for Mixed {
    fn cast_to(self) -> ArrayObject<K, V> {
        if let Mixed::Obj(o) = &self {
            if let Some(g) = o.as_any().downcast_ref::<ArrayObject<K, V>>() {
                return g.clone();
            }
        }
        panic!("Mixed value is not an ArrayObject")
    }
}
impl<K: MapKey, V> std::fmt::Debug for ArrayObject<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArrayObject")
    }
}

// ---------------------------------------------------------------- SplObjectStorage / WeakMap

pub struct SplObjectStorage<K, V>(Rc<RwCell<Vec<(K, V)>>>);

impl<K, V> Clone for SplObjectStorage<K, V> {
    fn clone(&self) -> Self {
        SplObjectStorage(self.0.clone())
    }
}
impl<K: PhpObject + Clone, V: Clone> SplObjectStorage<K, V> {
    pub fn new() -> Self {
        SplObjectStorage(Rc::new(RwCell::new(Vec::new())))
    }
    fn find(&self, k: &K) -> Option<usize> {
        let id = k.obj_id();
        self.0.borrow().iter().position(|(o, _)| o.obj_id() == id)
    }
    pub fn attach(&self, k: K, v: V) {
        match self.find(&k) {
            Some(i) => self.0.borrow_mut()[i].1 = v,
            None => self.0.borrow_mut().push((k, v)),
        }
    }
    pub fn detach(&self, k: &K) {
        if let Some(i) = self.find(k) {
            self.0.borrow_mut().remove(i);
        }
    }
    pub fn contains(&self, k: &K) -> bool {
        self.find(k).is_some()
    }
    pub fn get(&self, k: &K) -> Option<V> {
        self.find(k).map(|i| self.0.borrow()[i].1.clone())
    }
    pub fn idx(&self, k: &K) -> V {
        self.get(k).expect("Object not found in SplObjectStorage")
    }
    pub fn count(&self) -> i64 {
        self.0.borrow().len() as i64
    }
    pub fn add_all(&self, other: &Self) {
        for (k, v) in other.0.borrow().iter() {
            self.attach(k.clone(), v.clone());
        }
    }
    pub fn iter_objects(&self) -> std::vec::IntoIter<(i64, K)> {
        self.0.borrow().iter().enumerate().map(|(i, (k, _))| (i as i64, k.clone())).collect::<Vec<_>>().into_iter()
    }
    pub fn into_pairs(&self) -> Vec<(K, V)> {
        self.0.borrow().clone()
    }
    pub fn get_info(&self) -> V {
        self.0.borrow().first().map(|(_, v)| v.clone()).expect("empty storage")
    }
}
impl<K: PhpObject + Clone, V: Clone> Default for SplObjectStorage<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
impl<K, V> crate::traits::PhpKind for SplObjectStorage<K, V> { fn php_kind(&self) -> crate::traits::Kind { crate::traits::Kind::Obj } }
impl<K, V> crate::traits::InstanceOfName for SplObjectStorage<K, V> { fn php_instance_of(&self, name: &[u8]) -> bool { name.eq_ignore_ascii_case(b"splobjectstorage") } }
impl<K, V> Truthy for SplObjectStorage<K, V> {
    fn truthy(&self) -> bool {
        true
    }
}
impl<K, V> Identical for SplObjectStorage<K, V> {
    fn identical(&self, o: &Self) -> bool {
        Rc::ptr_eq(&self.0, &o.0)
    }
}
impl<K, V> Len for SplObjectStorage<K, V> {
    fn php_count(&self) -> i64 {
        self.0.borrow().len() as i64
    }
}
impl<K, V> PhpObject for SplObjectStorage<K, V>
where
    K: PhpObject + Clone + 'static,
    V: Clone + crate::cast::CastTo<Mixed> + Send + Sync + 'static,
    Mixed: crate::cast::CastTo<K> + crate::cast::CastTo<V>,
{
    fn class_name(&self) -> &'static str {
        "SplObjectStorage"
    }
    fn class_ancestors(&self) -> &'static [&'static str] {
        &["splobjectstorage", "countable", "iterator", "traversable", "arrayaccess"]
    }
    fn obj_id(&self) -> usize {
        Rc::as_ptr(&self.0) as *const u8 as usize
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Mixed {
        use crate::cast::CastTo;
        let arg = |i: usize| args.get(i).cloned().unwrap_or(Mixed::Null);
        let key = |i: usize| -> K { CastTo::<K>::cast_to(arg(i)) };
        match name {
            "attach" | "offsetset" => {
                let v: V = CastTo::<V>::cast_to(arg(1));
                self.attach(key(0), v);
                Mixed::Null
            }
            "detach" | "offsetunset" => {
                self.detach(&key(0));
                Mixed::Null
            }
            "contains" | "offsetexists" => Mixed::Bool(self.contains(&key(0))),
            "offsetget" => match self.get(&key(0)) {
                Some(v) => CastTo::<Mixed>::cast_to(v),
                None => {
                    panic!("Uncaught exception: UnexpectedValueException: Object not found");
                }
            },
            "count" => Mixed::Int(self.count()),
            "getinfo" => CastTo::<Mixed>::cast_to(self.get_info()),
            "addall" => {
                let other: SplObjectStorage<K, V> = CastTo::<SplObjectStorage<K, V>>::cast_to(arg(0));
                self.add_all(&other);
                Mixed::Null
            }
            _ => {
                panic!("Uncaught exception: Call to undefined method SplObjectStorage::{}()", name);
            }
        }
    }
}
impl<K, V> crate::cast::CastTo<Mixed> for SplObjectStorage<K, V>
where
    K: PhpObject + Clone + 'static,
    V: Clone + crate::cast::CastTo<Mixed> + Send + Sync + 'static,
    Mixed: crate::cast::CastTo<K> + crate::cast::CastTo<V>,
{
    fn cast_to(self) -> Mixed {
        Mixed::Obj(Rc::new(self))
    }
}
impl<K, V> crate::cast::CastTo<SplObjectStorage<K, V>> for Mixed
where
    K: PhpObject + Clone + 'static,
    V: Clone + crate::cast::CastTo<Mixed> + Send + Sync + 'static,
    Mixed: crate::cast::CastTo<K> + crate::cast::CastTo<V>,
{
    fn cast_to(self) -> SplObjectStorage<K, V> {
        if let Mixed::Obj(o) = &self {
            if let Some(g) = o.as_any().downcast_ref::<SplObjectStorage<K, V>>() {
                return g.clone();
            }
        }
        panic!("Mixed value is not a SplObjectStorage")
    }
}
impl<K, V> std::fmt::Debug for SplObjectStorage<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SplObjectStorage")
    }
}
pub type WeakMap<K, V> = SplObjectStorage<K, V>;

// ---------------------------------------------------------------- WeakReference

#[derive(Clone)]
pub struct WeakReference<T>(Rc<RwCell<Option<T>>>);
impl<T: Clone + PhpObject + 'static> WeakReference<T> {
    /// As in PHP, creating a weak reference to the same object twice yields the same instance.
    pub fn create(v: T) -> Self {
        thread_local! {
            static REGISTRY: RefCell<crate::FastMap<usize, Box<dyn std::any::Any>>> = RefCell::new(crate::fast_map());
        }
        let id = v.obj_id();
        REGISTRY.with(|r| {
            if let Some(existing) = r.borrow().get(&id).and_then(|b| b.downcast_ref::<WeakReference<T>>()) {
                return existing.clone();
            }
            let w = WeakReference(Rc::new(RwCell::new(Some(v))));
            r.borrow_mut().insert(id, Box::new(w.clone()));
            w
        })
    }
}
impl<T: Clone> WeakReference<T> {
    pub fn get(&self) -> Option<T> {
        self.0.borrow().clone()
    }
}
impl<T> crate::traits::PhpKind for WeakReference<T> { fn php_kind(&self) -> crate::traits::Kind { crate::traits::Kind::Obj } }
impl<T> crate::traits::InstanceOfName for WeakReference<T> { fn php_instance_of(&self, name: &[u8]) -> bool { name.eq_ignore_ascii_case(b"weakreference") } }
impl<T> Truthy for WeakReference<T> {
    fn truthy(&self) -> bool {
        true
    }
}
impl<T> Identical for WeakReference<T> {
    fn identical(&self, o: &Self) -> bool {
        Rc::ptr_eq(&self.0, &o.0)
    }
}
impl<T> std::fmt::Debug for WeakReference<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WeakReference")
    }
}
impl<T: Clone + crate::cast::CastTo<Mixed> + Send + Sync + 'static> PhpObject for WeakReference<T> {
    fn class_name(&self) -> &'static str {
        "WeakReference"
    }
    fn class_ancestors(&self) -> &'static [&'static str] {
        &["weakreference"]
    }
    fn obj_id(&self) -> usize {
        Rc::as_ptr(&self.0) as *const u8 as usize
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn call_method(&self, name: &str, _args: Vec<Mixed>) -> Mixed {
        match name {
            "get" => match self.get() {
                Some(v) => crate::cast::CastTo::<Mixed>::cast_to(v),
                None => Mixed::Null,
            },
            _ => panic!("Uncaught exception: Call to undefined method WeakReference::{}()", name),
        }
    }
}
impl<T: Clone + crate::cast::CastTo<Mixed> + Send + Sync + 'static> crate::cast::CastTo<Mixed> for WeakReference<T> {
    fn cast_to(self) -> Mixed {
        Mixed::Obj(Rc::new(self))
    }
}
impl<T: Clone + crate::cast::CastTo<Mixed> + Send + Sync + 'static> crate::cast::CastTo<WeakReference<T>> for Mixed {
    fn cast_to(self) -> WeakReference<T> {
        if let Mixed::Obj(o) = &self {
            if let Some(w) = o.as_any().downcast_ref::<WeakReference<T>>() {
                return w.clone();
            }
        }
        panic!("Mixed value is not a WeakReference")
    }
}

// ---------------------------------------------------------------- stdClass

#[derive(Clone, Default)]
pub struct StdClass(Rc<RwCell<Map<Str, Mixed>>>);
impl StdClass {
    pub fn new() -> Self {
        StdClass(Rc::new(RwCell::new(Map::new())))
    }
    pub fn from_map(m: Map<Str, Mixed>) -> Self {
        StdClass(Rc::new(RwCell::new(m)))
    }
    pub fn from_mixed(m: Mixed) -> Self {
        match m {
            Mixed::Arr(a) => Self::from_map(a.into_iter().map(|(k, v)| (k.to_str(), v)).collect()),
            Mixed::Obj(o) => {
                if let Some(s) = o.as_any().downcast_ref::<StdClass>() {
                    return s.clone();
                }
                Self::from_map(o.props().into_iter().collect())
            }
            Mixed::Null => Self::new(),
            other => Self::from_map(Map::from_pairs([(Str::from_static("scalar"), other)])),
        }
    }
    pub fn get(&self, name: &Str) -> Option<Mixed> {
        self.0.borrow().get(name).cloned().and_then(|v| v.to_option())
    }
    pub fn get_or_null(&self, name: &Str) -> Mixed {
        self.0.borrow().get(name).cloned().unwrap_or_default()
    }
    pub fn set(&self, name: &Str, v: Mixed) {
        self.0.borrow_mut().insert(name.clone(), v);
    }
    pub fn to_map(&self) -> Map<Str, Mixed> {
        self.0.borrow().clone()
    }
}
impl crate::traits::PhpKind for StdClass { fn php_kind(&self) -> crate::traits::Kind { crate::traits::Kind::Obj } }
impl crate::traits::InstanceOfName for StdClass { fn php_instance_of(&self, name: &[u8]) -> bool { name.eq_ignore_ascii_case(b"stdclass") } }
impl Truthy for StdClass {
    fn truthy(&self) -> bool {
        true
    }
}
impl Identical for StdClass {
    fn identical(&self, o: &Self) -> bool {
        Rc::ptr_eq(&self.0, &o.0)
    }
}
impl PhpObject for StdClass {
    fn class_name(&self) -> &'static str {
        "stdClass"
    }
    fn class_ancestors(&self) -> &'static [&'static str] {
        &["stdclass"]
    }
    fn obj_id(&self) -> usize {
        Rc::as_ptr(&self.0) as *const u8 as usize
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn props(&self) -> Vec<(Str, Mixed)> {
        self.0.borrow().iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}
impl crate::cast::CastTo<Mixed> for StdClass {
    fn cast_to(self) -> Mixed {
        Mixed::Obj(Rc::new(self))
    }
}
impl crate::cast::CastTo<StdClass> for Mixed {
    fn cast_to(self) -> StdClass {
        StdClass::from_mixed(self)
    }
}
impl std::fmt::Debug for StdClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "stdClass{:?}", self.0.borrow())
    }
}

// ---------------------------------------------------------------- DynCallable

/// Error raised through a dynamically-typed callable: either a runtime error or a PHP exception object.
#[derive(Clone, Debug)]
pub enum DynError {
    Rt(RtError),
    Obj(Mixed),
}
impl From<RtError> for DynError {
    fn from(e: RtError) -> Self {
        DynError::Rt(e)
    }
}

type DynFn = dyn Fn(Vec<Mixed>) -> Mixed + Send + Sync;

/// A callable of unknown signature: arguments and result travel as Mixed.
#[derive(Clone)]
pub struct DynCallable {
    pub arity: usize,
    f: Rc<DynFn>,
    is_null: bool,
}
impl DynCallable {
    pub fn new<F: Fn(Vec<Mixed>) -> Mixed + Send + Sync + 'static>(arity: usize, f: F) -> Self {
        DynCallable { arity, f: Rc::new(f), is_null: false }
    }
    pub fn from_rt<F: Fn(Vec<Mixed>) -> Result<Mixed, RtError> + Send + Sync + 'static>(arity: usize, f: F) -> Self {
        DynCallable { arity, f: Rc::new(move |a| f(a).unwrap_or_else(|__e| panic!("Uncaught exception: {}", __e))), is_null: false }
    }
    /// A `null` stored where a callable is expected (docblocks like `callable[]` holding nulls):
    /// invoking it is an Error, as in PHP, and `=== null` comparisons see it as null.
    pub fn null_callable() -> Self {
        let mut c = DynCallable::from_rt(0, |_| panic!("Uncaught exception: Value of type null is not callable"));
        c.is_null = true;
        c
    }
    pub fn is_null(&self) -> bool {
        self.is_null
    }
    /// `None` for a null callable, `Some` otherwise.
    pub fn into_option(self) -> Option<DynCallable> {
        if self.is_null { None } else { Some(self) }
    }
    pub fn call(&self, mut args: Vec<Mixed>) -> Mixed {
        while args.len() < self.arity {
            args.push(Mixed::Null);
        }
        (self.f)(args)
    }
}
impl crate::traits::PhpKind for DynCallable { fn php_kind(&self) -> crate::traits::Kind { crate::traits::Kind::Closure } }
impl crate::traits::InstanceOfName for DynCallable { fn php_instance_of(&self, name: &[u8]) -> bool { name.eq_ignore_ascii_case(b"closure") } }
impl Truthy for DynCallable {
    fn truthy(&self) -> bool {
        !self.is_null
    }
}
impl Identical for DynCallable {
    fn identical(&self, o: &Self) -> bool {
        (self.is_null && o.is_null) || Rc::ptr_eq(&self.f, &o.f)
    }
}
impl std::fmt::Debug for DynCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Closure")
    }
}
impl crate::cast::CastTo<Mixed> for DynCallable {
    fn cast_to(self) -> Mixed {
        if self.is_null { Mixed::Null } else { Mixed::Closure(Rc::new(self)) }
    }
}
impl crate::cast::CastTo<DynCallable> for Mixed {
    fn cast_to(self) -> DynCallable {
        to_callable(&self)
    }
}
impl ToStr for DynCallable {
    fn to_php_str(&self) -> Str {
        Str::from_static("Closure")
    }
}

/// Resolve a Mixed callable (closure, function name) into a DynCallable.
pub fn to_callable(m: &Mixed) -> DynCallable {
    match m {
        Mixed::Closure(c) => {
            if let Some(d) = c.downcast_ref::<DynCallable>() {
                return d.clone();
            }
            panic!("unsupported closure payload")
        }
        Mixed::Str(s) => {
            // functions are never looked up by name (closed world): string callables are resolved at compile time
            let name = s.clone();
            DynCallable::from_rt(0, move |_| Err(RtError::error(crate::sfmt!("Call to undefined function {}()", name))))
        }
        _ => DynCallable::from_rt(0, |_| Err(RtError::error("Value not callable"))),
    }
}

// ---------------------------------------------------------------- resources

pub enum ResourceKind {
    Stdin,
    Stdout,
    Stderr,
    File(RwCell<std::fs::File>, Str),
    Memory(RwCell<Vec<u8>>, AtomicUsize),
    Closed,
}

pub struct Resource {
    pub id: usize,
    pub kind: RwCell<ResourceKind>,
}

impl Resource {
    pub fn id(&self) -> usize {
        self.id
    }
}
impl std::fmt::Debug for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resource id #{}", self.id)
    }
}
impl<T: ?Sized> crate::traits::PhpKind for Rc<T> { fn php_kind(&self) -> crate::traits::Kind { crate::traits::Kind::Closure } }
impl<T: ?Sized> crate::traits::InstanceOfName for Rc<T> { fn php_instance_of(&self, name: &[u8]) -> bool { name.eq_ignore_ascii_case(b"closure") } }
impl<T: ?Sized> Truthy for Rc<T> {
    fn truthy(&self) -> bool {
        true
    }
}
impl<T: ?Sized> Identical for Rc<T> {
    fn identical(&self, o: &Self) -> bool {
        Rc::ptr_eq(self, o)
    }
}
impl PhpCmp for Rc<Resource> {
    fn php_cmp(&self, o: &Self) -> std::cmp::Ordering {
        self.id.cmp(&o.id)
    }
}
impl ToStr for Rc<Resource> {
    fn to_php_str(&self) -> Str {
        crate::sfmt!("Resource id #{}", self.id)
    }
}
impl crate::cast::CastTo<Mixed> for Rc<Resource> {
    fn cast_to(self) -> Mixed {
        Mixed::Int(self.id as i64)
    }
}

impl crate::cast::CastTo<Rc<Resource>> for Mixed {
    fn cast_to(self) -> Rc<Resource> {
        match self {
            Mixed::Int(id) => resource_by_id(id as usize).unwrap_or_else(|| panic!("unknown resource #{}", id)),
            m => panic!("Mixed value is not a resource: {:?}", m),
        }
    }
}

/// A resource by id (resources travel through `Mixed` as their integer id).
pub fn resource_by_id(id: usize) -> Option<Rc<Resource>> {
    match id {
        1 => Some(stdin_res()),
        2 => Some(stdout_res()),
        3 => Some(stderr_res()),
        _ => RESOURCES.with(|r| r.borrow().get(&id).cloned()),
    }
}

thread_local! {
    static RESOURCES: RefCell<crate::FastMap<usize, Rc<Resource>>> = RefCell::new(crate::fast_map());
    static RES_COUNTER: AtomicUsize = AtomicUsize::new(4);
    static STDIN_RES: Rc<Resource> = Rc::new(Resource { id: 1, kind: RwCell::new(ResourceKind::Stdin) });
    static STDOUT_RES: Rc<Resource> = Rc::new(Resource { id: 2, kind: RwCell::new(ResourceKind::Stdout) });
    static STDERR_RES: Rc<Resource> = Rc::new(Resource { id: 3, kind: RwCell::new(ResourceKind::Stderr) });
}
pub fn new_resource(kind: ResourceKind) -> Rc<Resource> {
    let id = RES_COUNTER.with(|c| {
        let v = c.load(std::sync::atomic::Ordering::Relaxed);
        c.store(v + 1, std::sync::atomic::Ordering::Relaxed);
        v
    });
    let r = Rc::new(Resource { id, kind: RwCell::new(kind) });
    RESOURCES.with(|m| m.borrow_mut().insert(id, r.clone()));
    r
}
pub fn stdin_res() -> Rc<Resource> {
    STDIN_RES.with(|r| r.clone())
}
pub fn stdout_res() -> Rc<Resource> {
    STDOUT_RES.with(|r| r.clone())
}
pub fn stderr_res() -> Rc<Resource> {
    STDERR_RES.with(|r| r.clone())
}

/// Keep a weak handle type name around for generic code.
pub type WeakObj = Weak<dyn PhpObject>;
pub type AnyObjRef = AnyObj;
