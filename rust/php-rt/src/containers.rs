//! Runtime implementations of builtin generic classes and callable/resource values.

use crate::error::RtError;
use crate::key::{ArrayKey, MapKey};
use crate::list::List;
use crate::map::Map;
use crate::mixed::{AnyObj, Mixed, PhpObject};
use crate::string::Str;
use crate::traits::*;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

// ---------------------------------------------------------------- Generator (eagerly evaluated)

pub struct GeneratorData<K, V> {
    pairs: Vec<(K, V)>,
    pos: Cell<usize>,
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
        Generator(Rc::new(GeneratorData { pairs, pos: Cell::new(0) }))
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
        self.0.pairs.get(self.0.pos.get()).map(|(_, v)| v.clone())
    }
    pub fn key(&self) -> Option<K> {
        self.0.pairs.get(self.0.pos.get()).map(|(k, _)| k.clone())
    }
    pub fn next(&self) {
        self.0.pos.set(self.0.pos.get() + 1);
    }
    pub fn rewind(&self) {
        self.0.pos.set(0);
    }
    pub fn valid(&self) -> bool {
        self.0.pos.get() < self.0.pairs.len()
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
impl<K: Clone + crate::cast::CastTo<Mixed> + 'static, V: Clone + crate::cast::CastTo<Mixed> + 'static> PhpObject for Generator<K, V> {
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
impl<K: Clone + crate::cast::CastTo<Mixed> + 'static, V: Clone + crate::cast::CastTo<Mixed> + 'static> crate::cast::CastTo<Mixed> for Generator<K, V> {
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
                o.call_method(name, Vec::new()).unwrap_or_else(|e| match e {
                    DynError::Rt(r) => panic!("{}", r.message),
                    DynError::Obj(_) => panic!("exception while iterating {}", o.class_name()),
                })
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

pub struct ArrayObject<K, V>(Rc<RefCell<Map<K, V>>>, Cell<usize>);

impl<K, V> Clone for ArrayObject<K, V> {
    fn clone(&self) -> Self {
        ArrayObject(self.0.clone(), Cell::new(self.1.get()))
    }
}
impl<K: MapKey, V: Clone> ArrayObject<K, V> {
    pub fn new(m: Map<K, V>) -> Self {
        ArrayObject(Rc::new(RefCell::new(m)), Cell::new(0))
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
        ArrayObject(self.0.clone(), Cell::new(0))
    }
    pub fn into_pairs(&self) -> Vec<(K, V)> {
        self.0.borrow().to_pairs()
    }
    pub fn current(&self) -> Option<V> {
        self.0.borrow().iter().nth(self.1.get()).map(|(_, v)| v.clone())
    }
    pub fn key(&self) -> Option<K> {
        self.0.borrow().iter().nth(self.1.get()).map(|(k, _)| k.clone())
    }
    pub fn next(&self) {
        self.1.set(self.1.get() + 1);
    }
    pub fn rewind(&self) {
        self.1.set(0);
    }
    pub fn valid(&self) -> bool {
        self.1.get() < self.0.borrow().len()
    }
}
pub type ArrayIterator<K, V> = ArrayObject<K, V>;
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
impl<K: MapKey + 'static, V: Clone + 'static> PhpObject for ArrayObject<K, V> {
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
impl<K: MapKey + 'static, V: Clone + 'static> crate::cast::CastTo<Mixed> for ArrayObject<K, V> {
    fn cast_to(self) -> Mixed {
        Mixed::Obj(Rc::new(self))
    }
}
impl<K: MapKey + 'static, V: Clone + 'static> crate::cast::CastTo<ArrayObject<K, V>> for Mixed {
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

pub struct SplObjectStorage<K, V>(Rc<RefCell<Vec<(K, V)>>>);

impl<K, V> Clone for SplObjectStorage<K, V> {
    fn clone(&self) -> Self {
        SplObjectStorage(self.0.clone())
    }
}
impl<K: PhpObject + Clone, V: Clone> SplObjectStorage<K, V> {
    pub fn new() -> Self {
        SplObjectStorage(Rc::new(RefCell::new(Vec::new())))
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
impl<K: 'static, V: 'static> PhpObject for SplObjectStorage<K, V> {
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
}
impl<K: 'static, V: 'static> crate::cast::CastTo<Mixed> for SplObjectStorage<K, V> {
    fn cast_to(self) -> Mixed {
        Mixed::Obj(Rc::new(self))
    }
}
impl<K: 'static, V: 'static> crate::cast::CastTo<SplObjectStorage<K, V>> for Mixed {
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
pub struct WeakReference<T>(Rc<RefCell<Option<T>>>);
impl<T: Clone> WeakReference<T> {
    pub fn create(v: T) -> Self {
        WeakReference(Rc::new(RefCell::new(Some(v))))
    }
    pub fn get(&self) -> Option<T> {
        self.0.borrow().clone()
    }
}
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
impl<T: 'static> PhpObject for WeakReference<T> {
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
}
impl<T: 'static> crate::cast::CastTo<Mixed> for WeakReference<T> {
    fn cast_to(self) -> Mixed {
        Mixed::Obj(Rc::new(self))
    }
}

// ---------------------------------------------------------------- stdClass

#[derive(Clone, Default)]
pub struct StdClass(Rc<RefCell<Map<Str, Mixed>>>);
impl StdClass {
    pub fn new() -> Self {
        StdClass(Rc::new(RefCell::new(Map::new())))
    }
    pub fn from_map(m: Map<Str, Mixed>) -> Self {
        StdClass(Rc::new(RefCell::new(m)))
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

type DynFn = dyn Fn(Vec<Mixed>) -> Result<Mixed, DynError>;

/// A callable of unknown signature: arguments and result travel as Mixed.
#[derive(Clone)]
pub struct DynCallable {
    pub arity: usize,
    f: Rc<DynFn>,
    is_null: bool,
}
impl DynCallable {
    pub fn new<F: Fn(Vec<Mixed>) -> Result<Mixed, DynError> + 'static>(arity: usize, f: F) -> Self {
        DynCallable { arity, f: Rc::new(f), is_null: false }
    }
    pub fn from_rt<F: Fn(Vec<Mixed>) -> Result<Mixed, RtError> + 'static>(arity: usize, f: F) -> Self {
        DynCallable { arity, f: Rc::new(move |a| f(a).map_err(DynError::Rt)), is_null: false }
    }
    /// A `null` stored where a callable is expected (docblocks like `callable[]` holding nulls):
    /// invoking it is an Error, as in PHP, and `=== null` comparisons see it as null.
    pub fn null_callable() -> Self {
        let mut c = DynCallable::from_rt(0, |_| Err(RtError::error("Value of type null is not callable")));
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
    pub fn call(&self, mut args: Vec<Mixed>) -> Result<Mixed, DynError> {
        while args.len() < self.arity {
            args.push(Mixed::Null);
        }
        (self.f)(args)
    }
}
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
            let name = s.clone();
            match crate::registry::function_by_name(&name) {
                Some(f) => f,
                None => DynCallable::from_rt(0, move |_| Err(RtError::error(crate::sfmt!("Call to undefined function {}()", name)))),
            }
        }
        _ => DynCallable::from_rt(0, |_| Err(RtError::error("Value not callable"))),
    }
}

// ---------------------------------------------------------------- resources

pub enum ResourceKind {
    Stdin,
    Stdout,
    Stderr,
    File(RefCell<std::fs::File>, Str),
    Memory(RefCell<Vec<u8>>, Cell<usize>),
    Closed,
}

pub struct Resource {
    pub id: usize,
    pub kind: RefCell<ResourceKind>,
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

thread_local! {
    static RES_COUNTER: Cell<usize> = Cell::new(4);
    static STDIN_RES: Rc<Resource> = Rc::new(Resource { id: 1, kind: RefCell::new(ResourceKind::Stdin) });
    static STDOUT_RES: Rc<Resource> = Rc::new(Resource { id: 2, kind: RefCell::new(ResourceKind::Stdout) });
    static STDERR_RES: Rc<Resource> = Rc::new(Resource { id: 3, kind: RefCell::new(ResourceKind::Stderr) });
}
pub fn new_resource(kind: ResourceKind) -> Rc<Resource> {
    let id = RES_COUNTER.with(|c| {
        let v = c.get();
        c.set(v + 1);
        v
    });
    Rc::new(Resource { id, kind: RefCell::new(kind) })
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
