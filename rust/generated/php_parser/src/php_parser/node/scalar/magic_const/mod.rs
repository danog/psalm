use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub struct Class_Obj {
    pub attributes: Late<Map<Str, Mixed>>,
}
#[derive(Clone)]
pub struct Class_(pub Rc<RefCell<Class_Obj>>);
impl Class_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn new_uninit() -> Class_ {
        Class_(Rc::new(RefCell::new(Class_Obj {
            attributes: Late::uninit(),
        })))
    }
    pub fn new(mut attributes: Map<Str, Mixed>) -> Result<Class_, Throw> {
        let this = Class_(Rc::new(RefCell::new(Class_Obj {
            attributes: Late::uninit(),
        })));
        this.magic__construct(attributes)?;
        Ok(this)
    }
    pub fn getName(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("__CLASS__"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Scalar_MagicConst_Class"));
    }
    pub fn magic__construct(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).magic__construct__impl(attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).getSubNodeNames__impl() }
    pub fn getLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getLine__impl() }
    pub fn getStartLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartLine__impl() }
    pub fn getEndLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndLine__impl() }
    pub fn getStartTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartTokenPos__impl() }
    pub fn getEndTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndTokenPos__impl() }
    pub fn getStartFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartFilePos__impl() }
    pub fn getEndFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndFilePos__impl() }
    pub fn getComments(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getComments__impl() }
    pub fn getDocComment(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getDocComment__impl() }
    pub fn setDocComment(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setDocComment__impl(docComment) }
    pub fn setAttribute(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttribute__impl(key_v, value) }
    pub fn hasAttribute(&self, mut key_v: Str) -> Result<bool, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).hasAttribute__impl(key_v) }
    pub fn getAttribute(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttribute__impl(key_v, default) }
    pub fn getAttributes(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttributes__impl() }
    pub fn setAttributes(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttributes__impl(attributes) }
    pub fn jsonSerialize(&self) -> Result<Map<Str, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).jsonSerialize__impl() }
    pub fn new_same_class(&self, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::scalar::magic_const::Class_, Throw> { Ok(Self::new(attributes)?) }
}
impl php_rt::PhpObject for Class_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Scalar\\MagicConst\\Class_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\scalar\\magicconst\\class_", "phpparser\\node\\scalar\\magicconst", "phpparser\\node\\scalar", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getname" => { let __r = self.getName().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Scalar\\MagicConst\\Class_", name)))) } }
}
impl Class_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Scalar\\MagicConst\\Class_ could not be converted to string"))) } }
impl php_rt::PhpClone for Class_ { fn php_clone(&self) -> Self { let c = Class_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Class_Obj { fn clone(&self) -> Self { Class_Obj { attributes: self.attributes.clone() } } }
impl Class_ {
}
pub struct DirObj {
    pub attributes: Late<Map<Str, Mixed>>,
}
#[derive(Clone)]
pub struct Dir(pub Rc<RefCell<DirObj>>);
impl Dir {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn new_uninit() -> Dir {
        Dir(Rc::new(RefCell::new(DirObj {
            attributes: Late::uninit(),
        })))
    }
    pub fn new(mut attributes: Map<Str, Mixed>) -> Result<Dir, Throw> {
        let this = Dir(Rc::new(RefCell::new(DirObj {
            attributes: Late::uninit(),
        })));
        this.magic__construct(attributes)?;
        Ok(this)
    }
    pub fn getName(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("__DIR__"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Scalar_MagicConst_Dir"));
    }
    pub fn magic__construct(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).magic__construct__impl(attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).getSubNodeNames__impl() }
    pub fn getLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getLine__impl() }
    pub fn getStartLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartLine__impl() }
    pub fn getEndLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndLine__impl() }
    pub fn getStartTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartTokenPos__impl() }
    pub fn getEndTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndTokenPos__impl() }
    pub fn getStartFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartFilePos__impl() }
    pub fn getEndFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndFilePos__impl() }
    pub fn getComments(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getComments__impl() }
    pub fn getDocComment(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getDocComment__impl() }
    pub fn setDocComment(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setDocComment__impl(docComment) }
    pub fn setAttribute(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttribute__impl(key_v, value) }
    pub fn hasAttribute(&self, mut key_v: Str) -> Result<bool, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).hasAttribute__impl(key_v) }
    pub fn getAttribute(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttribute__impl(key_v, default) }
    pub fn getAttributes(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttributes__impl() }
    pub fn setAttributes(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttributes__impl(attributes) }
    pub fn jsonSerialize(&self) -> Result<Map<Str, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).jsonSerialize__impl() }
    pub fn new_same_class(&self, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::scalar::magic_const::Dir, Throw> { Ok(Self::new(attributes)?) }
}
impl php_rt::PhpObject for Dir {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Scalar\\MagicConst\\Dir" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\scalar\\magicconst\\dir", "phpparser\\node\\scalar\\magicconst", "phpparser\\node\\scalar", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getname" => { let __r = self.getName().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Scalar\\MagicConst\\Dir", name)))) } }
}
impl Dir { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Scalar\\MagicConst\\Dir could not be converted to string"))) } }
impl php_rt::PhpClone for Dir { fn php_clone(&self) -> Self { let c = Dir(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for DirObj { fn clone(&self) -> Self { DirObj { attributes: self.attributes.clone() } } }
impl Dir {
}
pub struct FileObj {
    pub attributes: Late<Map<Str, Mixed>>,
}
#[derive(Clone)]
pub struct File(pub Rc<RefCell<FileObj>>);
impl File {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn new_uninit() -> File {
        File(Rc::new(RefCell::new(FileObj {
            attributes: Late::uninit(),
        })))
    }
    pub fn new(mut attributes: Map<Str, Mixed>) -> Result<File, Throw> {
        let this = File(Rc::new(RefCell::new(FileObj {
            attributes: Late::uninit(),
        })));
        this.magic__construct(attributes)?;
        Ok(this)
    }
    pub fn getName(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("__FILE__"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Scalar_MagicConst_File"));
    }
    pub fn magic__construct(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).magic__construct__impl(attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).getSubNodeNames__impl() }
    pub fn getLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getLine__impl() }
    pub fn getStartLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartLine__impl() }
    pub fn getEndLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndLine__impl() }
    pub fn getStartTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartTokenPos__impl() }
    pub fn getEndTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndTokenPos__impl() }
    pub fn getStartFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartFilePos__impl() }
    pub fn getEndFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndFilePos__impl() }
    pub fn getComments(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getComments__impl() }
    pub fn getDocComment(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getDocComment__impl() }
    pub fn setDocComment(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setDocComment__impl(docComment) }
    pub fn setAttribute(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttribute__impl(key_v, value) }
    pub fn hasAttribute(&self, mut key_v: Str) -> Result<bool, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).hasAttribute__impl(key_v) }
    pub fn getAttribute(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttribute__impl(key_v, default) }
    pub fn getAttributes(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttributes__impl() }
    pub fn setAttributes(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttributes__impl(attributes) }
    pub fn jsonSerialize(&self) -> Result<Map<Str, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).jsonSerialize__impl() }
    pub fn new_same_class(&self, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::scalar::magic_const::File, Throw> { Ok(Self::new(attributes)?) }
}
impl php_rt::PhpObject for File {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Scalar\\MagicConst\\File" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\scalar\\magicconst\\file", "phpparser\\node\\scalar\\magicconst", "phpparser\\node\\scalar", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getname" => { let __r = self.getName().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Scalar\\MagicConst\\File", name)))) } }
}
impl File { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Scalar\\MagicConst\\File could not be converted to string"))) } }
impl php_rt::PhpClone for File { fn php_clone(&self) -> Self { let c = File(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for FileObj { fn clone(&self) -> Self { FileObj { attributes: self.attributes.clone() } } }
impl File {
}
pub struct Function_Obj {
    pub attributes: Late<Map<Str, Mixed>>,
}
#[derive(Clone)]
pub struct Function_(pub Rc<RefCell<Function_Obj>>);
impl Function_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn new_uninit() -> Function_ {
        Function_(Rc::new(RefCell::new(Function_Obj {
            attributes: Late::uninit(),
        })))
    }
    pub fn new(mut attributes: Map<Str, Mixed>) -> Result<Function_, Throw> {
        let this = Function_(Rc::new(RefCell::new(Function_Obj {
            attributes: Late::uninit(),
        })));
        this.magic__construct(attributes)?;
        Ok(this)
    }
    pub fn getName(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("__FUNCTION__"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Scalar_MagicConst_Function"));
    }
    pub fn magic__construct(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).magic__construct__impl(attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).getSubNodeNames__impl() }
    pub fn getLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getLine__impl() }
    pub fn getStartLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartLine__impl() }
    pub fn getEndLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndLine__impl() }
    pub fn getStartTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartTokenPos__impl() }
    pub fn getEndTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndTokenPos__impl() }
    pub fn getStartFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartFilePos__impl() }
    pub fn getEndFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndFilePos__impl() }
    pub fn getComments(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getComments__impl() }
    pub fn getDocComment(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getDocComment__impl() }
    pub fn setDocComment(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setDocComment__impl(docComment) }
    pub fn setAttribute(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttribute__impl(key_v, value) }
    pub fn hasAttribute(&self, mut key_v: Str) -> Result<bool, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).hasAttribute__impl(key_v) }
    pub fn getAttribute(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttribute__impl(key_v, default) }
    pub fn getAttributes(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttributes__impl() }
    pub fn setAttributes(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttributes__impl(attributes) }
    pub fn jsonSerialize(&self) -> Result<Map<Str, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).jsonSerialize__impl() }
    pub fn new_same_class(&self, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::scalar::magic_const::Function_, Throw> { Ok(Self::new(attributes)?) }
}
impl php_rt::PhpObject for Function_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Scalar\\MagicConst\\Function_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\scalar\\magicconst\\function_", "phpparser\\node\\scalar\\magicconst", "phpparser\\node\\scalar", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getname" => { let __r = self.getName().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Scalar\\MagicConst\\Function_", name)))) } }
}
impl Function_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Scalar\\MagicConst\\Function_ could not be converted to string"))) } }
impl php_rt::PhpClone for Function_ { fn php_clone(&self) -> Self { let c = Function_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Function_Obj { fn clone(&self) -> Self { Function_Obj { attributes: self.attributes.clone() } } }
impl Function_ {
}
pub struct LineObj {
    pub attributes: Late<Map<Str, Mixed>>,
}
#[derive(Clone)]
pub struct Line(pub Rc<RefCell<LineObj>>);
impl Line {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn new_uninit() -> Line {
        Line(Rc::new(RefCell::new(LineObj {
            attributes: Late::uninit(),
        })))
    }
    pub fn new(mut attributes: Map<Str, Mixed>) -> Result<Line, Throw> {
        let this = Line(Rc::new(RefCell::new(LineObj {
            attributes: Late::uninit(),
        })));
        this.magic__construct(attributes)?;
        Ok(this)
    }
    pub fn getName(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("__LINE__"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Scalar_MagicConst_Line"));
    }
    pub fn magic__construct(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).magic__construct__impl(attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).getSubNodeNames__impl() }
    pub fn getLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getLine__impl() }
    pub fn getStartLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartLine__impl() }
    pub fn getEndLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndLine__impl() }
    pub fn getStartTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartTokenPos__impl() }
    pub fn getEndTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndTokenPos__impl() }
    pub fn getStartFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartFilePos__impl() }
    pub fn getEndFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndFilePos__impl() }
    pub fn getComments(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getComments__impl() }
    pub fn getDocComment(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getDocComment__impl() }
    pub fn setDocComment(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setDocComment__impl(docComment) }
    pub fn setAttribute(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttribute__impl(key_v, value) }
    pub fn hasAttribute(&self, mut key_v: Str) -> Result<bool, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).hasAttribute__impl(key_v) }
    pub fn getAttribute(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttribute__impl(key_v, default) }
    pub fn getAttributes(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttributes__impl() }
    pub fn setAttributes(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttributes__impl(attributes) }
    pub fn jsonSerialize(&self) -> Result<Map<Str, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).jsonSerialize__impl() }
    pub fn new_same_class(&self, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::scalar::magic_const::Line, Throw> { Ok(Self::new(attributes)?) }
}
impl php_rt::PhpObject for Line {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Scalar\\MagicConst\\Line" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\scalar\\magicconst\\line", "phpparser\\node\\scalar\\magicconst", "phpparser\\node\\scalar", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getname" => { let __r = self.getName().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Scalar\\MagicConst\\Line", name)))) } }
}
impl Line { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Scalar\\MagicConst\\Line could not be converted to string"))) } }
impl php_rt::PhpClone for Line { fn php_clone(&self) -> Self { let c = Line(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for LineObj { fn clone(&self) -> Self { LineObj { attributes: self.attributes.clone() } } }
impl Line {
}
pub struct MethodObj {
    pub attributes: Late<Map<Str, Mixed>>,
}
#[derive(Clone)]
pub struct Method(pub Rc<RefCell<MethodObj>>);
impl Method {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn new_uninit() -> Method {
        Method(Rc::new(RefCell::new(MethodObj {
            attributes: Late::uninit(),
        })))
    }
    pub fn new(mut attributes: Map<Str, Mixed>) -> Result<Method, Throw> {
        let this = Method(Rc::new(RefCell::new(MethodObj {
            attributes: Late::uninit(),
        })));
        this.magic__construct(attributes)?;
        Ok(this)
    }
    pub fn getName(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("__METHOD__"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Scalar_MagicConst_Method"));
    }
    pub fn magic__construct(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).magic__construct__impl(attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).getSubNodeNames__impl() }
    pub fn getLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getLine__impl() }
    pub fn getStartLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartLine__impl() }
    pub fn getEndLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndLine__impl() }
    pub fn getStartTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartTokenPos__impl() }
    pub fn getEndTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndTokenPos__impl() }
    pub fn getStartFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartFilePos__impl() }
    pub fn getEndFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndFilePos__impl() }
    pub fn getComments(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getComments__impl() }
    pub fn getDocComment(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getDocComment__impl() }
    pub fn setDocComment(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setDocComment__impl(docComment) }
    pub fn setAttribute(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttribute__impl(key_v, value) }
    pub fn hasAttribute(&self, mut key_v: Str) -> Result<bool, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).hasAttribute__impl(key_v) }
    pub fn getAttribute(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttribute__impl(key_v, default) }
    pub fn getAttributes(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttributes__impl() }
    pub fn setAttributes(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttributes__impl(attributes) }
    pub fn jsonSerialize(&self) -> Result<Map<Str, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).jsonSerialize__impl() }
    pub fn new_same_class(&self, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::scalar::magic_const::Method, Throw> { Ok(Self::new(attributes)?) }
}
impl php_rt::PhpObject for Method {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Scalar\\MagicConst\\Method" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\scalar\\magicconst\\method", "phpparser\\node\\scalar\\magicconst", "phpparser\\node\\scalar", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getname" => { let __r = self.getName().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Scalar\\MagicConst\\Method", name)))) } }
}
impl Method { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Scalar\\MagicConst\\Method could not be converted to string"))) } }
impl php_rt::PhpClone for Method { fn php_clone(&self) -> Self { let c = Method(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for MethodObj { fn clone(&self) -> Self { MethodObj { attributes: self.attributes.clone() } } }
impl Method {
}
pub struct Namespace_Obj {
    pub attributes: Late<Map<Str, Mixed>>,
}
#[derive(Clone)]
pub struct Namespace_(pub Rc<RefCell<Namespace_Obj>>);
impl Namespace_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn new_uninit() -> Namespace_ {
        Namespace_(Rc::new(RefCell::new(Namespace_Obj {
            attributes: Late::uninit(),
        })))
    }
    pub fn new(mut attributes: Map<Str, Mixed>) -> Result<Namespace_, Throw> {
        let this = Namespace_(Rc::new(RefCell::new(Namespace_Obj {
            attributes: Late::uninit(),
        })));
        this.magic__construct(attributes)?;
        Ok(this)
    }
    pub fn getName(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("__NAMESPACE__"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Scalar_MagicConst_Namespace"));
    }
    pub fn magic__construct(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).magic__construct__impl(attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).getSubNodeNames__impl() }
    pub fn getLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getLine__impl() }
    pub fn getStartLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartLine__impl() }
    pub fn getEndLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndLine__impl() }
    pub fn getStartTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartTokenPos__impl() }
    pub fn getEndTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndTokenPos__impl() }
    pub fn getStartFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartFilePos__impl() }
    pub fn getEndFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndFilePos__impl() }
    pub fn getComments(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getComments__impl() }
    pub fn getDocComment(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getDocComment__impl() }
    pub fn setDocComment(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setDocComment__impl(docComment) }
    pub fn setAttribute(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttribute__impl(key_v, value) }
    pub fn hasAttribute(&self, mut key_v: Str) -> Result<bool, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).hasAttribute__impl(key_v) }
    pub fn getAttribute(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttribute__impl(key_v, default) }
    pub fn getAttributes(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttributes__impl() }
    pub fn setAttributes(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttributes__impl(attributes) }
    pub fn jsonSerialize(&self) -> Result<Map<Str, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).jsonSerialize__impl() }
    pub fn new_same_class(&self, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::scalar::magic_const::Namespace_, Throw> { Ok(Self::new(attributes)?) }
}
impl php_rt::PhpObject for Namespace_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Scalar\\MagicConst\\Namespace_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\scalar\\magicconst\\namespace_", "phpparser\\node\\scalar\\magicconst", "phpparser\\node\\scalar", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getname" => { let __r = self.getName().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Scalar\\MagicConst\\Namespace_", name)))) } }
}
impl Namespace_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Scalar\\MagicConst\\Namespace_ could not be converted to string"))) } }
impl php_rt::PhpClone for Namespace_ { fn php_clone(&self) -> Self { let c = Namespace_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Namespace_Obj { fn clone(&self) -> Self { Namespace_Obj { attributes: self.attributes.clone() } } }
impl Namespace_ {
}
pub struct PropertyObj {
    pub attributes: Late<Map<Str, Mixed>>,
}
#[derive(Clone)]
pub struct Property(pub Rc<RefCell<PropertyObj>>);
impl Property {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn new_uninit() -> Property {
        Property(Rc::new(RefCell::new(PropertyObj {
            attributes: Late::uninit(),
        })))
    }
    pub fn new(mut attributes: Map<Str, Mixed>) -> Result<Property, Throw> {
        let this = Property(Rc::new(RefCell::new(PropertyObj {
            attributes: Late::uninit(),
        })));
        this.magic__construct(attributes)?;
        Ok(this)
    }
    pub fn getName(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("__PROPERTY__"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Scalar_MagicConst_Property"));
    }
    pub fn magic__construct(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).magic__construct__impl(attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).getSubNodeNames__impl() }
    pub fn getLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getLine__impl() }
    pub fn getStartLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartLine__impl() }
    pub fn getEndLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndLine__impl() }
    pub fn getStartTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartTokenPos__impl() }
    pub fn getEndTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndTokenPos__impl() }
    pub fn getStartFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartFilePos__impl() }
    pub fn getEndFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndFilePos__impl() }
    pub fn getComments(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getComments__impl() }
    pub fn getDocComment(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getDocComment__impl() }
    pub fn setDocComment(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setDocComment__impl(docComment) }
    pub fn setAttribute(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttribute__impl(key_v, value) }
    pub fn hasAttribute(&self, mut key_v: Str) -> Result<bool, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).hasAttribute__impl(key_v) }
    pub fn getAttribute(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttribute__impl(key_v, default) }
    pub fn getAttributes(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttributes__impl() }
    pub fn setAttributes(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttributes__impl(attributes) }
    pub fn jsonSerialize(&self) -> Result<Map<Str, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).jsonSerialize__impl() }
    pub fn new_same_class(&self, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::scalar::magic_const::Property, Throw> { Ok(Self::new(attributes)?) }
}
impl php_rt::PhpObject for Property {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Scalar\\MagicConst\\Property" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\scalar\\magicconst\\property", "phpparser\\node\\scalar\\magicconst", "phpparser\\node\\scalar", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getname" => { let __r = self.getName().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Scalar\\MagicConst\\Property", name)))) } }
}
impl Property { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Scalar\\MagicConst\\Property could not be converted to string"))) } }
impl php_rt::PhpClone for Property { fn php_clone(&self) -> Self { let c = Property(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for PropertyObj { fn clone(&self) -> Self { PropertyObj { attributes: self.attributes.clone() } } }
impl Property {
}
pub struct Trait_Obj {
    pub attributes: Late<Map<Str, Mixed>>,
}
#[derive(Clone)]
pub struct Trait_(pub Rc<RefCell<Trait_Obj>>);
impl Trait_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn new_uninit() -> Trait_ {
        Trait_(Rc::new(RefCell::new(Trait_Obj {
            attributes: Late::uninit(),
        })))
    }
    pub fn new(mut attributes: Map<Str, Mixed>) -> Result<Trait_, Throw> {
        let this = Trait_(Rc::new(RefCell::new(Trait_Obj {
            attributes: Late::uninit(),
        })));
        this.magic__construct(attributes)?;
        Ok(this)
    }
    pub fn getName(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("__TRAIT__"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Scalar_MagicConst_Trait"));
    }
    pub fn magic__construct(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).magic__construct__impl(attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::scalar::MagicConst>(self.clone()).getSubNodeNames__impl() }
    pub fn getLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getLine__impl() }
    pub fn getStartLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartLine__impl() }
    pub fn getEndLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndLine__impl() }
    pub fn getStartTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartTokenPos__impl() }
    pub fn getEndTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndTokenPos__impl() }
    pub fn getStartFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartFilePos__impl() }
    pub fn getEndFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndFilePos__impl() }
    pub fn getComments(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getComments__impl() }
    pub fn getDocComment(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getDocComment__impl() }
    pub fn setDocComment(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setDocComment__impl(docComment) }
    pub fn setAttribute(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttribute__impl(key_v, value) }
    pub fn hasAttribute(&self, mut key_v: Str) -> Result<bool, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).hasAttribute__impl(key_v) }
    pub fn getAttribute(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttribute__impl(key_v, default) }
    pub fn getAttributes(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttributes__impl() }
    pub fn setAttributes(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttributes__impl(attributes) }
    pub fn jsonSerialize(&self) -> Result<Map<Str, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).jsonSerialize__impl() }
    pub fn new_same_class(&self, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::scalar::magic_const::Trait_, Throw> { Ok(Self::new(attributes)?) }
}
impl php_rt::PhpObject for Trait_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Scalar\\MagicConst\\Trait_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\scalar\\magicconst\\trait_", "phpparser\\node\\scalar\\magicconst", "phpparser\\node\\scalar", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getname" => { let __r = self.getName().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Scalar\\MagicConst\\Trait_", name)))) } }
}
impl Trait_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Scalar\\MagicConst\\Trait_ could not be converted to string"))) } }
impl php_rt::PhpClone for Trait_ { fn php_clone(&self) -> Self { let c = Trait_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Trait_Obj { fn clone(&self) -> Self { Trait_Obj { attributes: self.attributes.clone() } } }
impl Trait_ {
}
impl php_rt::Truthy for Class_ { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Class_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Class_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Scalar\\MagicConst\\Class_")) } }
impl php_rt::PhpCmp for Class_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Class_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Class_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Class_> for Mixed { fn cast_to(self) -> Class_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Class_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Scalar\\MagicConst\\Class_") } }
impl php_rt::TryDowncast for Class_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Class_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Class_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Class_> for AnyObject { fn cast_to(self) -> Class_ { cast::<Class_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Class_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\scalar\\magicconst\\class_") } }
impl php_rt::InstanceOf<Class_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\scalar\\magicconst\\class_") } }
impl php_rt::InstanceOf<Class_> for Class_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Dir { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Dir { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Dir { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Scalar\\MagicConst\\Dir")) } }
impl php_rt::PhpCmp for Dir { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Dir { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Dir { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Dir> for Mixed { fn cast_to(self) -> Dir { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Dir>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Scalar\\MagicConst\\Dir") } }
impl php_rt::TryDowncast for Dir { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Dir>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Dir { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Dir> for AnyObject { fn cast_to(self) -> Dir { cast::<Dir>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Dir> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\scalar\\magicconst\\dir") } }
impl php_rt::InstanceOf<Dir> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\scalar\\magicconst\\dir") } }
impl php_rt::InstanceOf<Dir> for Dir { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for File { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for File { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for File { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Scalar\\MagicConst\\File")) } }
impl php_rt::PhpCmp for File { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for File { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for File { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<File> for Mixed { fn cast_to(self) -> File { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::File>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Scalar\\MagicConst\\File") } }
impl php_rt::TryDowncast for File { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::File>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for File { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<File> for AnyObject { fn cast_to(self) -> File { cast::<File>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<File> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\scalar\\magicconst\\file") } }
impl php_rt::InstanceOf<File> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\scalar\\magicconst\\file") } }
impl php_rt::InstanceOf<File> for File { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Function_ { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Function_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Function_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Scalar\\MagicConst\\Function_")) } }
impl php_rt::PhpCmp for Function_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Function_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Function_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Function_> for Mixed { fn cast_to(self) -> Function_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Function_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Scalar\\MagicConst\\Function_") } }
impl php_rt::TryDowncast for Function_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Function_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Function_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Function_> for AnyObject { fn cast_to(self) -> Function_ { cast::<Function_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Function_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\scalar\\magicconst\\function_") } }
impl php_rt::InstanceOf<Function_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\scalar\\magicconst\\function_") } }
impl php_rt::InstanceOf<Function_> for Function_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Line { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Line { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Line { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Scalar\\MagicConst\\Line")) } }
impl php_rt::PhpCmp for Line { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Line { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Line { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Line> for Mixed { fn cast_to(self) -> Line { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Line>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Scalar\\MagicConst\\Line") } }
impl php_rt::TryDowncast for Line { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Line>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Line { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Line> for AnyObject { fn cast_to(self) -> Line { cast::<Line>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Line> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\scalar\\magicconst\\line") } }
impl php_rt::InstanceOf<Line> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\scalar\\magicconst\\line") } }
impl php_rt::InstanceOf<Line> for Line { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Method { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Method { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Method { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Scalar\\MagicConst\\Method")) } }
impl php_rt::PhpCmp for Method { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Method { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Method { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Method> for Mixed { fn cast_to(self) -> Method { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Method>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Scalar\\MagicConst\\Method") } }
impl php_rt::TryDowncast for Method { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Method>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Method { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Method> for AnyObject { fn cast_to(self) -> Method { cast::<Method>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Method> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\scalar\\magicconst\\method") } }
impl php_rt::InstanceOf<Method> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\scalar\\magicconst\\method") } }
impl php_rt::InstanceOf<Method> for Method { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Namespace_ { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Namespace_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Namespace_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Scalar\\MagicConst\\Namespace_")) } }
impl php_rt::PhpCmp for Namespace_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Namespace_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Namespace_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Namespace_> for Mixed { fn cast_to(self) -> Namespace_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Namespace_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Scalar\\MagicConst\\Namespace_") } }
impl php_rt::TryDowncast for Namespace_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Namespace_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Namespace_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Namespace_> for AnyObject { fn cast_to(self) -> Namespace_ { cast::<Namespace_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Namespace_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\scalar\\magicconst\\namespace_") } }
impl php_rt::InstanceOf<Namespace_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\scalar\\magicconst\\namespace_") } }
impl php_rt::InstanceOf<Namespace_> for Namespace_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Property { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Property { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Property { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Scalar\\MagicConst\\Property")) } }
impl php_rt::PhpCmp for Property { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Property { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Property { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Property> for Mixed { fn cast_to(self) -> Property { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Property>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Scalar\\MagicConst\\Property") } }
impl php_rt::TryDowncast for Property { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Property>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Property { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Property> for AnyObject { fn cast_to(self) -> Property { cast::<Property>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Property> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\scalar\\magicconst\\property") } }
impl php_rt::InstanceOf<Property> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\scalar\\magicconst\\property") } }
impl php_rt::InstanceOf<Property> for Property { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Trait_ { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Trait_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Trait_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Scalar\\MagicConst\\Trait_")) } }
impl php_rt::PhpCmp for Trait_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Trait_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Trait_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Trait_> for Mixed { fn cast_to(self) -> Trait_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Trait_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Scalar\\MagicConst\\Trait_") } }
impl php_rt::TryDowncast for Trait_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Trait_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Trait_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Trait_> for AnyObject { fn cast_to(self) -> Trait_ { cast::<Trait_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Trait_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\scalar\\magicconst\\trait_") } }
impl php_rt::InstanceOf<Trait_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\scalar\\magicconst\\trait_") } }
impl php_rt::InstanceOf<Trait_> for Trait_ { fn is_instance(&self) -> bool { true } }
