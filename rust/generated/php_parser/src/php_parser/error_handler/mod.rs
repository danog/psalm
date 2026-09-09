use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub struct CollectingObj {
    pub errors: Map<ArrayKey, crate::php_parser::Error>,
}
#[derive(Clone)]
pub struct Collecting(pub Rc<RefCell<CollectingObj>>);
impl Collecting {
    pub fn p_errors(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::Error>> { Ref::map(self.0.borrow(), |o| &o.errors) }
    pub fn p_errors_get(&self) -> Map<ArrayKey, crate::php_parser::Error> { self.0.borrow().errors.clone() }
    pub fn p_errors_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::Error>> { Some(self.0.borrow().errors.clone()) }
    pub fn p_errors_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::Error>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.errors) }
    pub fn set_p_errors(&self, v: Map<ArrayKey, crate::php_parser::Error>) { self.0.borrow_mut().errors = v; }
    pub fn new() -> Result<Collecting, Throw> {
        let this = Collecting(Rc::new(RefCell::new(CollectingObj {
            errors: Map::<ArrayKey, crate::php_parser::Error>::new(),
        })));
        Ok(this)
    }
    pub fn handleError(&self, mut error_v: crate::php_parser::Error) -> Result<(), Throw> {
    (*self.p_errors_mut()).push(error_v.clone());
    #[allow(unreachable_code)] Ok(())
    }
    pub fn getErrors(&self) -> Result<Map<ArrayKey, crate::php_parser::Error>, Throw> {
    return Ok(self.p_errors_get());
    }
    pub fn hasErrors(&self) -> Result<bool, Throw> {
    return Ok((!(!truthy(&Some(self.clone()).and_then(|__b| Some(__b.p_errors_get()))))));
    }
    pub fn clearErrors(&self) -> Result<(), Throw> {
    self.set_p_errors(Map::<ArrayKey, crate::php_parser::Error>::new());
    #[allow(unreachable_code)] Ok(())
    }
    pub fn new_same_class(&self) -> Result<crate::php_parser::error_handler::Collecting, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for Collecting {
    fn class_name(&self) -> &'static str { "PhpParser\\ErrorHandler\\Collecting" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\errorhandler\\collecting", "phpparser\\errorhandler"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_errors_get()) { out.push((Str::from_static("errors"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "errors" => { self.set_p_errors(cast::<Map<ArrayKey, crate::php_parser::Error>>(value)); true }, _ => false } }
}
impl Collecting { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\ErrorHandler\\Collecting could not be converted to string"))) } }
impl php_rt::PhpClone for Collecting { fn php_clone(&self) -> Self { let c = Collecting(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for CollectingObj { fn clone(&self) -> Self { CollectingObj { errors: self.errors.clone() } } }
impl Collecting {
}
pub struct ThrowingObj {
}
#[derive(Clone)]
pub struct Throwing(pub Rc<RefCell<ThrowingObj>>);
impl Throwing {
    pub fn new() -> Result<Throwing, Throw> {
        let this = Throwing(Rc::new(RefCell::new(ThrowingObj {
        })));
        Ok(this)
    }
    pub fn handleError(&self, mut error_v: crate::php_parser::Error) -> Result<(), Throw> {
    return Err(cast::<crate::g::Throwable>(error_v.clone()));
    }
    pub fn new_same_class(&self) -> Result<crate::php_parser::error_handler::Throwing, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for Throwing {
    fn class_name(&self) -> &'static str { "PhpParser\\ErrorHandler\\Throwing" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\errorhandler\\throwing", "phpparser\\errorhandler"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { _ => false } }
}
impl Throwing { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\ErrorHandler\\Throwing could not be converted to string"))) } }
impl php_rt::PhpClone for Throwing { fn php_clone(&self) -> Self { let c = Throwing(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ThrowingObj { fn clone(&self) -> Self { ThrowingObj {  } } }
impl Throwing {
}
impl php_rt::Truthy for Collecting { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Collecting { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\ErrorHandler\\Collecting")) } }
impl php_rt::Identical for Collecting { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Collecting { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Collecting { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Collecting { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Collecting> for Mixed { fn cast_to(self) -> Collecting { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::error_handler::Collecting>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\ErrorHandler\\Collecting") } }
impl php_rt::TryDowncast for Collecting { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::error_handler::Collecting>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Collecting { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Collecting> for AnyObject { fn cast_to(self) -> Collecting { cast::<Collecting>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Collecting> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\errorhandler\\collecting") } }
impl php_rt::InstanceOf<Collecting> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\errorhandler\\collecting") } }
impl php_rt::InstanceOf<Collecting> for Collecting { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Throwing { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Throwing { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\ErrorHandler\\Throwing")) } }
impl php_rt::Identical for Throwing { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Throwing { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Throwing { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Throwing { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Throwing> for Mixed { fn cast_to(self) -> Throwing { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::error_handler::Throwing>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\ErrorHandler\\Throwing") } }
impl php_rt::TryDowncast for Throwing { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::error_handler::Throwing>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Throwing { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Throwing> for AnyObject { fn cast_to(self) -> Throwing { cast::<Throwing>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Throwing> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\errorhandler\\throwing") } }
impl php_rt::InstanceOf<Throwing> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\errorhandler\\throwing") } }
impl php_rt::InstanceOf<Throwing> for Throwing { fn is_instance(&self) -> bool { true } }
