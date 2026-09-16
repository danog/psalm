//! The `mixed` type: a dynamically typed PHP value, used only where Psalm infers `mixed`.

use crate::key::ArrayKey;
use crate::list::List;
use crate::map::Map;
use crate::string::Str;
use std::any::Any;
use std::fmt;
use std::sync::Arc as Rc;

/// Implemented (by generated code) for every class handle type.
pub trait PhpObject: Any + Send + Sync {
    fn class_name(&self) -> &'static str;
    /// Lower-cased fully qualified names of the class and all its ancestors/interfaces.
    fn class_ancestors(&self) -> &'static [&'static str];
    fn obj_id(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
    /// Program-wide class number assigned by the transpiler (0: not a generated class).
    fn class_id(&self) -> u32 {
        0
    }
    /// Class numbers of the class and of all its ancestors and interfaces.
    fn class_ancestor_ids(&self) -> &'static [u32] {
        &[]
    }
    fn instance_of_id(&self, id: u32) -> bool {
        self.class_ancestor_ids().contains(&id)
    }
    fn props(&self) -> Vec<(Str, Mixed)> {
        Vec::new()
    }
    /// Public properties only (object iteration / get_object_vars from outside the class).
    fn public_props(&self) -> Vec<(Str, Mixed)> {
        self.props()
    }
    fn php_to_string(&self) -> Option<Str> {
        None
    }
    /// `clone $obj` on a type-erased object (generated classes return a fresh `Rc` of their own handle).
    fn php_clone_dyn(&self) -> AnyObj {
        panic!("Uncaught exception: object of class {} is not cloneable", self.class_name())
    }
    /// Dynamic property write; returns false when the property is unknown.
    fn set_prop(&self, _name: &str, _value: Mixed) -> bool {
        false
    }
    /// Dynamic property read (`None` when the property is unknown or uninitialized).
    fn get_prop(&self, name: &str) -> Option<Mixed> {
        self.props().into_iter().find(|(k, _)| k.as_bytes() == name.as_bytes()).map(|(_, v)| v)
    }
    fn instance_of_name(&self, lname: &str) -> bool {
        self.class_ancestors().iter().any(|a| *a == lname)
    }
    /// Call a method by (case-insensitive) name with dynamically typed arguments.
    fn call_method(&self, name: &str, _args: Vec<Mixed>) -> Mixed {
        panic!("Uncaught exception: Call to undefined method {}::{}()", self.class_name(), name)
    }

    /// Cross-hierarchy ("sideways") cast for the closed world, registry-free. A concrete class
    /// overrides this to return, for each ancestor-hierarchy dispatch trait `H__Dyn` it implements
    /// (identified by the transpiler's `hid` = the hierarchy's class id), `self` viewed as
    /// `Rc<dyn H__Dyn>`, type-erased via [`erase_dyn`]. The caller — which statically knows the target
    /// hierarchy — reconstructs it with [`unerase_dyn`]. `None` means the object is not of that
    /// hierarchy. No global registry: the vtable travels with the object. `self: Rc<Self>` so the
    /// returned trait object shares the object's refcount.
    fn query(self: Rc<Self>, _hid: u32) -> Option<*mut ()> {
        None
    }
}

/// Type-erase an `Rc<dyn Trait>` (a fat pointer) to a thin pointer, soundly on stable Rust by boxing
/// it. Pairs with [`unerase_dyn`]. Used by generated `PhpObject::query` implementations.
#[inline]
pub fn erase_dyn<T: ?Sized>(rc: Rc<T>) -> *mut () {
    Box::into_raw(Box::new(rc)) as *mut ()
}

/// Reconstruct an `Rc<dyn Trait>` erased by [`erase_dyn`].
///
/// # Safety
/// `e` must be a pointer returned by [`erase_dyn`]/`query` for the *same* trait `T`, consumed once.
#[inline]
pub unsafe fn unerase_dyn<T: ?Sized>(e: *mut ()) -> Rc<T> {
    *Box::from_raw(e as *mut Rc<T>)
}

pub type AnyObj = Rc<dyn PhpObject>;

#[derive(Clone)]
pub enum Mixed {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(Str),
    Arr(Map<ArrayKey, Mixed>),
    Obj(AnyObj),
    Closure(Rc<dyn Any + Send + Sync>),
}

impl Default for Mixed {
    fn default() -> Mixed {
        Mixed::Null
    }
}

impl Mixed {
    pub fn is_null(&self) -> bool {
        matches!(self, Mixed::Null)
    }
    pub fn is_int(&self) -> bool {
        matches!(self, Mixed::Int(_))
    }
    pub fn is_float(&self) -> bool {
        matches!(self, Mixed::Float(_))
    }
    pub fn is_string(&self) -> bool {
        matches!(self, Mixed::Str(_))
    }
    pub fn is_bool(&self) -> bool {
        matches!(self, Mixed::Bool(_))
    }
    pub fn is_array(&self) -> bool {
        matches!(self, Mixed::Arr(_))
    }
    pub fn is_object(&self) -> bool {
        matches!(self, Mixed::Obj(_) | Mixed::Closure(_))
    }
    pub fn is_iterable(&self) -> bool {
        match self {
            Mixed::Arr(_) => true,
            Mixed::Obj(o) => o.instance_of_name("traversable"),
            _ => false,
        }
    }
    pub fn is_scalar(&self) -> bool {
        matches!(self, Mixed::Bool(_) | Mixed::Int(_) | Mixed::Float(_) | Mixed::Str(_))
    }
    pub fn is_numeric(&self) -> bool {
        match self {
            Mixed::Int(_) | Mixed::Float(_) => true,
            Mixed::Str(s) => crate::conv::parse_numeric(s.as_bytes()).is_some(),
            _ => false,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Mixed::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&Str> {
        match self {
            Mixed::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_arr(&self) -> Option<&Map<ArrayKey, Mixed>> {
        match self {
            Mixed::Arr(a) => Some(a),
            _ => None,
        }
    }
    pub fn as_obj(&self) -> Option<&AnyObj> {
        match self {
            Mixed::Obj(o) => Some(o),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            Mixed::Null => "NULL",
            Mixed::Bool(_) => "boolean",
            Mixed::Int(_) => "integer",
            Mixed::Float(_) => "double",
            Mixed::Str(_) => "string",
            Mixed::Arr(_) => "array",
            Mixed::Obj(_) | Mixed::Closure(_) => "object",
        }
    }
    pub fn debug_type(&self) -> Str {
        match self {
            Mixed::Null => Str::from_static("null"),
            Mixed::Bool(_) => Str::from_static("bool"),
            Mixed::Int(_) => Str::from_static("int"),
            Mixed::Float(_) => Str::from_static("float"),
            Mixed::Str(_) => Str::from_static("string"),
            Mixed::Arr(_) => Str::from_static("array"),
            Mixed::Obj(o) => Str::from_str(o.class_name()),
            Mixed::Closure(_) => Str::from_static("Closure"),
        }
    }
    pub fn downcast<T: PhpObject + Clone>(&self) -> Option<T> {
        match self {
            Mixed::Obj(o) => o.as_any().downcast_ref::<T>().cloned(),
            _ => None,
        }
    }
    pub fn instance_of(&self, lname: &str) -> bool {
        match self {
            Mixed::Obj(o) => o.instance_of_name(lname),
            Mixed::Closure(_) => lname == "closure",
            _ => false,
        }
    }
    /// `instanceof` a generated class, by its program-wide number.
    #[inline]
    pub fn instance_of_id(&self, id: u32) -> bool {
        match self {
            Mixed::Obj(o) => o.instance_of_id(id),
            _ => false,
        }
    }
}

impl fmt::Debug for Mixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mixed::Null => write!(f, "null"),
            Mixed::Bool(b) => write!(f, "{}", b),
            Mixed::Int(i) => write!(f, "{}", i),
            Mixed::Float(x) => write!(f, "{}", x),
            Mixed::Str(s) => write!(f, "{:?}", s),
            Mixed::Arr(a) => write!(f, "{:?}", a),
            Mixed::Obj(o) => write!(f, "object({})#{}", o.class_name(), o.obj_id()),
            Mixed::Closure(_) => write!(f, "Closure"),
        }
    }
}

impl From<()> for Mixed {
    fn from(_: ()) -> Mixed {
        Mixed::Null
    }
}
impl From<bool> for Mixed {
    fn from(b: bool) -> Mixed {
        Mixed::Bool(b)
    }
}
impl From<i64> for Mixed {
    fn from(i: i64) -> Mixed {
        Mixed::Int(i)
    }
}
impl From<f64> for Mixed {
    fn from(x: f64) -> Mixed {
        Mixed::Float(x)
    }
}
impl From<Str> for Mixed {
    fn from(s: Str) -> Mixed {
        Mixed::Str(s)
    }
}
impl From<&'static str> for Mixed {
    fn from(s: &'static str) -> Mixed {
        Mixed::Str(Str::from_static(s))
    }
}
impl From<String> for Mixed {
    fn from(s: String) -> Mixed {
        Mixed::Str(Str::from_string(s))
    }
}
impl From<ArrayKey> for Mixed {
    fn from(k: ArrayKey) -> Mixed {
        match k {
            ArrayKey::Int(i) => Mixed::Int(i),
            ArrayKey::Str(s) => Mixed::Str(s),
        }
    }
}
impl<T: Into<Mixed>> From<Option<T>> for Mixed {
    fn from(o: Option<T>) -> Mixed {
        match o {
            None => Mixed::Null,
            Some(v) => v.into(),
        }
    }
}
impl<T: Into<Mixed> + Clone> From<List<T>> for Mixed {
    fn from(l: List<T>) -> Mixed {
        Mixed::Arr(l.into_iter().enumerate().map(|(i, v)| (ArrayKey::Int(i as i64), v.into())).collect())
    }
}
impl<K: crate::key::MapKey, V: Into<Mixed> + Clone> From<Map<K, V>> for Mixed {
    fn from(m: Map<K, V>) -> Mixed {
        Mixed::Arr(m.into_iter().map(|(k, v)| (k.to_array_key(), v.into())).collect())
    }
}
impl From<AnyObj> for Mixed {
    fn from(o: AnyObj) -> Mixed {
        Mixed::Obj(o)
    }
}
