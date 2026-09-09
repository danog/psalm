use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub struct BitwiseAndObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct BitwiseAnd(pub Rc<RefCell<BitwiseAndObj>>);
impl BitwiseAnd {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> BitwiseAnd {
        BitwiseAnd(Rc::new(RefCell::new(BitwiseAndObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<BitwiseAnd, Throw> {
        let this = BitwiseAnd(Rc::new(RefCell::new(BitwiseAndObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("&"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_BitwiseAnd"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::BitwiseAnd, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for BitwiseAnd {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\BitwiseAnd" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\bitwiseand", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\BitwiseAnd", name)))) } }
}
impl BitwiseAnd { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\BitwiseAnd could not be converted to string"))) } }
impl php_rt::PhpClone for BitwiseAnd { fn php_clone(&self) -> Self { let c = BitwiseAnd(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for BitwiseAndObj { fn clone(&self) -> Self { BitwiseAndObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl BitwiseAnd {
}
pub struct BitwiseOrObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct BitwiseOr(pub Rc<RefCell<BitwiseOrObj>>);
impl BitwiseOr {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> BitwiseOr {
        BitwiseOr(Rc::new(RefCell::new(BitwiseOrObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<BitwiseOr, Throw> {
        let this = BitwiseOr(Rc::new(RefCell::new(BitwiseOrObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("|"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_BitwiseOr"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::BitwiseOr, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for BitwiseOr {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\BitwiseOr" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\bitwiseor", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\BitwiseOr", name)))) } }
}
impl BitwiseOr { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\BitwiseOr could not be converted to string"))) } }
impl php_rt::PhpClone for BitwiseOr { fn php_clone(&self) -> Self { let c = BitwiseOr(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for BitwiseOrObj { fn clone(&self) -> Self { BitwiseOrObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl BitwiseOr {
}
pub struct BitwiseXorObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct BitwiseXor(pub Rc<RefCell<BitwiseXorObj>>);
impl BitwiseXor {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> BitwiseXor {
        BitwiseXor(Rc::new(RefCell::new(BitwiseXorObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<BitwiseXor, Throw> {
        let this = BitwiseXor(Rc::new(RefCell::new(BitwiseXorObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("^"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_BitwiseXor"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::BitwiseXor, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for BitwiseXor {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\BitwiseXor" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\bitwisexor", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\BitwiseXor", name)))) } }
}
impl BitwiseXor { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\BitwiseXor could not be converted to string"))) } }
impl php_rt::PhpClone for BitwiseXor { fn php_clone(&self) -> Self { let c = BitwiseXor(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for BitwiseXorObj { fn clone(&self) -> Self { BitwiseXorObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl BitwiseXor {
}
pub struct BooleanAndObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct BooleanAnd(pub Rc<RefCell<BooleanAndObj>>);
impl BooleanAnd {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> BooleanAnd {
        BooleanAnd(Rc::new(RefCell::new(BooleanAndObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<BooleanAnd, Throw> {
        let this = BooleanAnd(Rc::new(RefCell::new(BooleanAndObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("&&"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_BooleanAnd"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::BooleanAnd, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for BooleanAnd {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\BooleanAnd" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\booleanand", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\BooleanAnd", name)))) } }
}
impl BooleanAnd { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\BooleanAnd could not be converted to string"))) } }
impl php_rt::PhpClone for BooleanAnd { fn php_clone(&self) -> Self { let c = BooleanAnd(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for BooleanAndObj { fn clone(&self) -> Self { BooleanAndObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl BooleanAnd {
}
pub struct BooleanOrObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct BooleanOr(pub Rc<RefCell<BooleanOrObj>>);
impl BooleanOr {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> BooleanOr {
        BooleanOr(Rc::new(RefCell::new(BooleanOrObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<BooleanOr, Throw> {
        let this = BooleanOr(Rc::new(RefCell::new(BooleanOrObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("||"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_BooleanOr"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::BooleanOr, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for BooleanOr {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\BooleanOr" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\booleanor", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\BooleanOr", name)))) } }
}
impl BooleanOr { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\BooleanOr could not be converted to string"))) } }
impl php_rt::PhpClone for BooleanOr { fn php_clone(&self) -> Self { let c = BooleanOr(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for BooleanOrObj { fn clone(&self) -> Self { BooleanOrObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl BooleanOr {
}
pub struct CoalesceObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Coalesce(pub Rc<RefCell<CoalesceObj>>);
impl Coalesce {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> Coalesce {
        Coalesce(Rc::new(RefCell::new(CoalesceObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Coalesce, Throw> {
        let this = Coalesce(Rc::new(RefCell::new(CoalesceObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("??"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_Coalesce"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::Coalesce, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for Coalesce {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\Coalesce" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\coalesce", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\Coalesce", name)))) } }
}
impl Coalesce { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\Coalesce could not be converted to string"))) } }
impl php_rt::PhpClone for Coalesce { fn php_clone(&self) -> Self { let c = Coalesce(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for CoalesceObj { fn clone(&self) -> Self { CoalesceObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl Coalesce {
}
pub struct ConcatObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Concat(pub Rc<RefCell<ConcatObj>>);
impl Concat {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> Concat {
        Concat(Rc::new(RefCell::new(ConcatObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Concat, Throw> {
        let this = Concat(Rc::new(RefCell::new(ConcatObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("."));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_Concat"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::Concat, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for Concat {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\Concat" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\concat", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\Concat", name)))) } }
}
impl Concat { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\Concat could not be converted to string"))) } }
impl php_rt::PhpClone for Concat { fn php_clone(&self) -> Self { let c = Concat(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ConcatObj { fn clone(&self) -> Self { ConcatObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl Concat {
}
pub struct DivObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Div(pub Rc<RefCell<DivObj>>);
impl Div {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> Div {
        Div(Rc::new(RefCell::new(DivObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Div, Throw> {
        let this = Div(Rc::new(RefCell::new(DivObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("/"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_Div"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::Div, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for Div {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\Div" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\div", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\Div", name)))) } }
}
impl Div { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\Div could not be converted to string"))) } }
impl php_rt::PhpClone for Div { fn php_clone(&self) -> Self { let c = Div(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for DivObj { fn clone(&self) -> Self { DivObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl Div {
}
pub struct EqualObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Equal(pub Rc<RefCell<EqualObj>>);
impl Equal {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> Equal {
        Equal(Rc::new(RefCell::new(EqualObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Equal, Throw> {
        let this = Equal(Rc::new(RefCell::new(EqualObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("=="));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_Equal"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::Equal, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for Equal {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\Equal" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\equal", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\Equal", name)))) } }
}
impl Equal { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\Equal could not be converted to string"))) } }
impl php_rt::PhpClone for Equal { fn php_clone(&self) -> Self { let c = Equal(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for EqualObj { fn clone(&self) -> Self { EqualObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl Equal {
}
pub struct GreaterObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Greater(pub Rc<RefCell<GreaterObj>>);
impl Greater {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> Greater {
        Greater(Rc::new(RefCell::new(GreaterObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Greater, Throw> {
        let this = Greater(Rc::new(RefCell::new(GreaterObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static(">"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_Greater"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::Greater, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for Greater {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\Greater" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\greater", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\Greater", name)))) } }
}
impl Greater { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\Greater could not be converted to string"))) } }
impl php_rt::PhpClone for Greater { fn php_clone(&self) -> Self { let c = Greater(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for GreaterObj { fn clone(&self) -> Self { GreaterObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl Greater {
}
pub struct GreaterOrEqualObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct GreaterOrEqual(pub Rc<RefCell<GreaterOrEqualObj>>);
impl GreaterOrEqual {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> GreaterOrEqual {
        GreaterOrEqual(Rc::new(RefCell::new(GreaterOrEqualObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<GreaterOrEqual, Throw> {
        let this = GreaterOrEqual(Rc::new(RefCell::new(GreaterOrEqualObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static(">="));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_GreaterOrEqual"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::GreaterOrEqual, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for GreaterOrEqual {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\GreaterOrEqual" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\greaterorequal", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\GreaterOrEqual", name)))) } }
}
impl GreaterOrEqual { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\GreaterOrEqual could not be converted to string"))) } }
impl php_rt::PhpClone for GreaterOrEqual { fn php_clone(&self) -> Self { let c = GreaterOrEqual(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for GreaterOrEqualObj { fn clone(&self) -> Self { GreaterOrEqualObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl GreaterOrEqual {
}
pub struct IdenticalObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Identical(pub Rc<RefCell<IdenticalObj>>);
impl Identical {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> Identical {
        Identical(Rc::new(RefCell::new(IdenticalObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Identical, Throw> {
        let this = Identical(Rc::new(RefCell::new(IdenticalObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("==="));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_Identical"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::Identical, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for Identical {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\Identical" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\identical", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\Identical", name)))) } }
}
impl Identical { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\Identical could not be converted to string"))) } }
impl php_rt::PhpClone for Identical { fn php_clone(&self) -> Self { let c = Identical(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for IdenticalObj { fn clone(&self) -> Self { IdenticalObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl Identical {
}
pub struct LogicalAndObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct LogicalAnd(pub Rc<RefCell<LogicalAndObj>>);
impl LogicalAnd {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> LogicalAnd {
        LogicalAnd(Rc::new(RefCell::new(LogicalAndObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<LogicalAnd, Throw> {
        let this = LogicalAnd(Rc::new(RefCell::new(LogicalAndObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("and"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_LogicalAnd"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::LogicalAnd, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for LogicalAnd {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\LogicalAnd" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\logicaland", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\LogicalAnd", name)))) } }
}
impl LogicalAnd { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\LogicalAnd could not be converted to string"))) } }
impl php_rt::PhpClone for LogicalAnd { fn php_clone(&self) -> Self { let c = LogicalAnd(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for LogicalAndObj { fn clone(&self) -> Self { LogicalAndObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl LogicalAnd {
}
pub struct LogicalOrObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct LogicalOr(pub Rc<RefCell<LogicalOrObj>>);
impl LogicalOr {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> LogicalOr {
        LogicalOr(Rc::new(RefCell::new(LogicalOrObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<LogicalOr, Throw> {
        let this = LogicalOr(Rc::new(RefCell::new(LogicalOrObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("or"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_LogicalOr"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::LogicalOr, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for LogicalOr {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\LogicalOr" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\logicalor", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\LogicalOr", name)))) } }
}
impl LogicalOr { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\LogicalOr could not be converted to string"))) } }
impl php_rt::PhpClone for LogicalOr { fn php_clone(&self) -> Self { let c = LogicalOr(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for LogicalOrObj { fn clone(&self) -> Self { LogicalOrObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl LogicalOr {
}
pub struct LogicalXorObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct LogicalXor(pub Rc<RefCell<LogicalXorObj>>);
impl LogicalXor {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> LogicalXor {
        LogicalXor(Rc::new(RefCell::new(LogicalXorObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<LogicalXor, Throw> {
        let this = LogicalXor(Rc::new(RefCell::new(LogicalXorObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("xor"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_LogicalXor"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::LogicalXor, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for LogicalXor {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\LogicalXor" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\logicalxor", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\LogicalXor", name)))) } }
}
impl LogicalXor { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\LogicalXor could not be converted to string"))) } }
impl php_rt::PhpClone for LogicalXor { fn php_clone(&self) -> Self { let c = LogicalXor(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for LogicalXorObj { fn clone(&self) -> Self { LogicalXorObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl LogicalXor {
}
pub struct MinusObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Minus(pub Rc<RefCell<MinusObj>>);
impl Minus {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> Minus {
        Minus(Rc::new(RefCell::new(MinusObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Minus, Throw> {
        let this = Minus(Rc::new(RefCell::new(MinusObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("-"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_Minus"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::Minus, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for Minus {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\Minus" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\minus", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\Minus", name)))) } }
}
impl Minus { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\Minus could not be converted to string"))) } }
impl php_rt::PhpClone for Minus { fn php_clone(&self) -> Self { let c = Minus(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for MinusObj { fn clone(&self) -> Self { MinusObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl Minus {
}
pub struct ModObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Mod(pub Rc<RefCell<ModObj>>);
impl Mod {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> Mod {
        Mod(Rc::new(RefCell::new(ModObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mod, Throw> {
        let this = Mod(Rc::new(RefCell::new(ModObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("%"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_Mod"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::Mod, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for Mod {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\Mod" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\mod", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\Mod", name)))) } }
}
impl Mod { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\Mod could not be converted to string"))) } }
impl php_rt::PhpClone for Mod { fn php_clone(&self) -> Self { let c = Mod(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ModObj { fn clone(&self) -> Self { ModObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl Mod {
}
pub struct MulObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Mul(pub Rc<RefCell<MulObj>>);
impl Mul {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> Mul {
        Mul(Rc::new(RefCell::new(MulObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mul, Throw> {
        let this = Mul(Rc::new(RefCell::new(MulObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("*"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_Mul"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::Mul, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for Mul {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\Mul" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\mul", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\Mul", name)))) } }
}
impl Mul { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\Mul could not be converted to string"))) } }
impl php_rt::PhpClone for Mul { fn php_clone(&self) -> Self { let c = Mul(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for MulObj { fn clone(&self) -> Self { MulObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl Mul {
}
pub struct NotEqualObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct NotEqual(pub Rc<RefCell<NotEqualObj>>);
impl NotEqual {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> NotEqual {
        NotEqual(Rc::new(RefCell::new(NotEqualObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<NotEqual, Throw> {
        let this = NotEqual(Rc::new(RefCell::new(NotEqualObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("!="));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_NotEqual"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::NotEqual, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for NotEqual {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\NotEqual" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\notequal", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\NotEqual", name)))) } }
}
impl NotEqual { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\NotEqual could not be converted to string"))) } }
impl php_rt::PhpClone for NotEqual { fn php_clone(&self) -> Self { let c = NotEqual(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for NotEqualObj { fn clone(&self) -> Self { NotEqualObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl NotEqual {
}
pub struct NotIdenticalObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct NotIdentical(pub Rc<RefCell<NotIdenticalObj>>);
impl NotIdentical {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> NotIdentical {
        NotIdentical(Rc::new(RefCell::new(NotIdenticalObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<NotIdentical, Throw> {
        let this = NotIdentical(Rc::new(RefCell::new(NotIdenticalObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("!=="));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_NotIdentical"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::NotIdentical, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for NotIdentical {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\NotIdentical" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\notidentical", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\NotIdentical", name)))) } }
}
impl NotIdentical { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\NotIdentical could not be converted to string"))) } }
impl php_rt::PhpClone for NotIdentical { fn php_clone(&self) -> Self { let c = NotIdentical(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for NotIdenticalObj { fn clone(&self) -> Self { NotIdenticalObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl NotIdentical {
}
pub struct PipeObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Pipe(pub Rc<RefCell<PipeObj>>);
impl Pipe {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> Pipe {
        Pipe(Rc::new(RefCell::new(PipeObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Pipe, Throw> {
        let this = Pipe(Rc::new(RefCell::new(PipeObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("|>"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_Pipe"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::Pipe, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for Pipe {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\Pipe" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\pipe", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\Pipe", name)))) } }
}
impl Pipe { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\Pipe could not be converted to string"))) } }
impl php_rt::PhpClone for Pipe { fn php_clone(&self) -> Self { let c = Pipe(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for PipeObj { fn clone(&self) -> Self { PipeObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl Pipe {
}
pub struct PlusObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Plus(pub Rc<RefCell<PlusObj>>);
impl Plus {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> Plus {
        Plus(Rc::new(RefCell::new(PlusObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Plus, Throw> {
        let this = Plus(Rc::new(RefCell::new(PlusObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("+"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_Plus"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::Plus, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for Plus {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\Plus" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\plus", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\Plus", name)))) } }
}
impl Plus { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\Plus could not be converted to string"))) } }
impl php_rt::PhpClone for Plus { fn php_clone(&self) -> Self { let c = Plus(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for PlusObj { fn clone(&self) -> Self { PlusObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl Plus {
}
pub struct PowObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Pow(pub Rc<RefCell<PowObj>>);
impl Pow {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> Pow {
        Pow(Rc::new(RefCell::new(PowObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Pow, Throw> {
        let this = Pow(Rc::new(RefCell::new(PowObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("**"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_Pow"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::Pow, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for Pow {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\Pow" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\pow", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\Pow", name)))) } }
}
impl Pow { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\Pow could not be converted to string"))) } }
impl php_rt::PhpClone for Pow { fn php_clone(&self) -> Self { let c = Pow(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for PowObj { fn clone(&self) -> Self { PowObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl Pow {
}
pub struct ShiftLeftObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct ShiftLeft(pub Rc<RefCell<ShiftLeftObj>>);
impl ShiftLeft {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> ShiftLeft {
        ShiftLeft(Rc::new(RefCell::new(ShiftLeftObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<ShiftLeft, Throw> {
        let this = ShiftLeft(Rc::new(RefCell::new(ShiftLeftObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("<<"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_ShiftLeft"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::ShiftLeft, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for ShiftLeft {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\ShiftLeft" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\shiftleft", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\ShiftLeft", name)))) } }
}
impl ShiftLeft { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\ShiftLeft could not be converted to string"))) } }
impl php_rt::PhpClone for ShiftLeft { fn php_clone(&self) -> Self { let c = ShiftLeft(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ShiftLeftObj { fn clone(&self) -> Self { ShiftLeftObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl ShiftLeft {
}
pub struct ShiftRightObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct ShiftRight(pub Rc<RefCell<ShiftRightObj>>);
impl ShiftRight {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> ShiftRight {
        ShiftRight(Rc::new(RefCell::new(ShiftRightObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<ShiftRight, Throw> {
        let this = ShiftRight(Rc::new(RefCell::new(ShiftRightObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static(">>"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_ShiftRight"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::ShiftRight, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for ShiftRight {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\ShiftRight" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\shiftright", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\ShiftRight", name)))) } }
}
impl ShiftRight { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\ShiftRight could not be converted to string"))) } }
impl php_rt::PhpClone for ShiftRight { fn php_clone(&self) -> Self { let c = ShiftRight(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ShiftRightObj { fn clone(&self) -> Self { ShiftRightObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl ShiftRight {
}
pub struct SmallerObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Smaller(pub Rc<RefCell<SmallerObj>>);
impl Smaller {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> Smaller {
        Smaller(Rc::new(RefCell::new(SmallerObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Smaller, Throw> {
        let this = Smaller(Rc::new(RefCell::new(SmallerObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("<"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_Smaller"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::Smaller, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for Smaller {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\Smaller" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\smaller", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\Smaller", name)))) } }
}
impl Smaller { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\Smaller could not be converted to string"))) } }
impl php_rt::PhpClone for Smaller { fn php_clone(&self) -> Self { let c = Smaller(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for SmallerObj { fn clone(&self) -> Self { SmallerObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl Smaller {
}
pub struct SmallerOrEqualObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct SmallerOrEqual(pub Rc<RefCell<SmallerOrEqualObj>>);
impl SmallerOrEqual {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> SmallerOrEqual {
        SmallerOrEqual(Rc::new(RefCell::new(SmallerOrEqualObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<SmallerOrEqual, Throw> {
        let this = SmallerOrEqual(Rc::new(RefCell::new(SmallerOrEqualObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("<="));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_SmallerOrEqual"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::SmallerOrEqual, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for SmallerOrEqual {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\SmallerOrEqual" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\smallerorequal", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\SmallerOrEqual", name)))) } }
}
impl SmallerOrEqual { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\SmallerOrEqual could not be converted to string"))) } }
impl php_rt::PhpClone for SmallerOrEqual { fn php_clone(&self) -> Self { let c = SmallerOrEqual(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for SmallerOrEqualObj { fn clone(&self) -> Self { SmallerOrEqualObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl SmallerOrEqual {
}
pub struct SpaceshipObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub left: Late<crate::php_parser::node::Expr>,
    pub right: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Spaceship(pub Rc<RefCell<SpaceshipObj>>);
impl Spaceship {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_left(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.left.get()) }
    pub fn p_left_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().left.get().clone() }
    pub fn p_left_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().left.as_option().cloned() }
    pub fn p_left_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.left.get_mut()) }
    pub fn set_p_left(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().left.set(v); }
    pub fn p_right(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.right.get()) }
    pub fn p_right_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().right.get().clone() }
    pub fn p_right_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().right.as_option().cloned() }
    pub fn p_right_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.right.get_mut()) }
    pub fn set_p_right(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().right.set(v); }
    pub fn new_uninit() -> Spaceship {
        Spaceship(Rc::new(RefCell::new(SpaceshipObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })))
    }
    pub fn new(mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Spaceship, Throw> {
        let this = Spaceship(Rc::new(RefCell::new(SpaceshipObj {
            attributes: Late::uninit(),
            left: Late::uninit(),
            right: Late::uninit(),
        })));
        this.magic__construct(left, right, attributes)?;
        Ok(this)
    }
    pub fn getOperatorSigil(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("<=>"));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_BinaryOp_Spaceship"));
    }
    pub fn magic__construct(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).magic__construct__impl(left, right, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::BinaryOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut left: crate::php_parser::node::Expr, mut right: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::binary_op::Spaceship, Throw> { Ok(Self::new(left, right, attributes)?) }
}
impl php_rt::PhpObject for Spaceship {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\BinaryOp\\Spaceship" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\binaryop\\spaceship", "phpparser\\node\\expr\\binaryop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_left_opt() { out.push((Str::from_static("left"), cast::<Mixed>(v))); } if let Some(v) = self.p_right_opt() { out.push((Str::from_static("right"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "left" => { self.set_p_left(cast::<crate::php_parser::node::Expr>(value)); true }, "right" => { self.set_p_right(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getoperatorsigil" => { let __r = self.getOperatorSigil().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\BinaryOp\\Spaceship", name)))) } }
}
impl Spaceship { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\BinaryOp\\Spaceship could not be converted to string"))) } }
impl php_rt::PhpClone for Spaceship { fn php_clone(&self) -> Self { let c = Spaceship(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for SpaceshipObj { fn clone(&self) -> Self { SpaceshipObj { attributes: self.attributes.clone(), left: self.left.clone(), right: self.right.clone() } } }
impl Spaceship {
}
impl php_rt::Truthy for BitwiseAnd { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for BitwiseAnd { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\BitwiseAnd")) } }
impl php_rt::Identical for BitwiseAnd { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for BitwiseAnd { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for BitwiseAnd { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for BitwiseAnd { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<BitwiseAnd> for Mixed { fn cast_to(self) -> BitwiseAnd { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::BitwiseAnd>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\BitwiseAnd") } }
impl php_rt::TryDowncast for BitwiseAnd { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::BitwiseAnd>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for BitwiseAnd { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<BitwiseAnd> for AnyObject { fn cast_to(self) -> BitwiseAnd { cast::<BitwiseAnd>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<BitwiseAnd> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\bitwiseand") } }
impl php_rt::InstanceOf<BitwiseAnd> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\bitwiseand") } }
impl php_rt::InstanceOf<BitwiseAnd> for BitwiseAnd { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for BitwiseOr { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for BitwiseOr { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\BitwiseOr")) } }
impl php_rt::Identical for BitwiseOr { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for BitwiseOr { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for BitwiseOr { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for BitwiseOr { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<BitwiseOr> for Mixed { fn cast_to(self) -> BitwiseOr { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::BitwiseOr>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\BitwiseOr") } }
impl php_rt::TryDowncast for BitwiseOr { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::BitwiseOr>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for BitwiseOr { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<BitwiseOr> for AnyObject { fn cast_to(self) -> BitwiseOr { cast::<BitwiseOr>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<BitwiseOr> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\bitwiseor") } }
impl php_rt::InstanceOf<BitwiseOr> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\bitwiseor") } }
impl php_rt::InstanceOf<BitwiseOr> for BitwiseOr { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for BitwiseXor { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for BitwiseXor { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\BitwiseXor")) } }
impl php_rt::Identical for BitwiseXor { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for BitwiseXor { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for BitwiseXor { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for BitwiseXor { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<BitwiseXor> for Mixed { fn cast_to(self) -> BitwiseXor { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::BitwiseXor>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\BitwiseXor") } }
impl php_rt::TryDowncast for BitwiseXor { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::BitwiseXor>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for BitwiseXor { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<BitwiseXor> for AnyObject { fn cast_to(self) -> BitwiseXor { cast::<BitwiseXor>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<BitwiseXor> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\bitwisexor") } }
impl php_rt::InstanceOf<BitwiseXor> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\bitwisexor") } }
impl php_rt::InstanceOf<BitwiseXor> for BitwiseXor { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for BooleanAnd { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for BooleanAnd { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\BooleanAnd")) } }
impl php_rt::Identical for BooleanAnd { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for BooleanAnd { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for BooleanAnd { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for BooleanAnd { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<BooleanAnd> for Mixed { fn cast_to(self) -> BooleanAnd { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::BooleanAnd>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\BooleanAnd") } }
impl php_rt::TryDowncast for BooleanAnd { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::BooleanAnd>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for BooleanAnd { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<BooleanAnd> for AnyObject { fn cast_to(self) -> BooleanAnd { cast::<BooleanAnd>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<BooleanAnd> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\booleanand") } }
impl php_rt::InstanceOf<BooleanAnd> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\booleanand") } }
impl php_rt::InstanceOf<BooleanAnd> for BooleanAnd { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for BooleanOr { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for BooleanOr { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\BooleanOr")) } }
impl php_rt::Identical for BooleanOr { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for BooleanOr { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for BooleanOr { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for BooleanOr { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<BooleanOr> for Mixed { fn cast_to(self) -> BooleanOr { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::BooleanOr>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\BooleanOr") } }
impl php_rt::TryDowncast for BooleanOr { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::BooleanOr>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for BooleanOr { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<BooleanOr> for AnyObject { fn cast_to(self) -> BooleanOr { cast::<BooleanOr>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<BooleanOr> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\booleanor") } }
impl php_rt::InstanceOf<BooleanOr> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\booleanor") } }
impl php_rt::InstanceOf<BooleanOr> for BooleanOr { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Coalesce { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Coalesce { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\Coalesce")) } }
impl php_rt::Identical for Coalesce { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Coalesce { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Coalesce { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Coalesce { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Coalesce> for Mixed { fn cast_to(self) -> Coalesce { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Coalesce>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\Coalesce") } }
impl php_rt::TryDowncast for Coalesce { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Coalesce>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Coalesce { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Coalesce> for AnyObject { fn cast_to(self) -> Coalesce { cast::<Coalesce>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Coalesce> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\coalesce") } }
impl php_rt::InstanceOf<Coalesce> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\coalesce") } }
impl php_rt::InstanceOf<Coalesce> for Coalesce { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Concat { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Concat { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\Concat")) } }
impl php_rt::Identical for Concat { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Concat { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Concat { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Concat { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Concat> for Mixed { fn cast_to(self) -> Concat { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Concat>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\Concat") } }
impl php_rt::TryDowncast for Concat { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Concat>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Concat { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Concat> for AnyObject { fn cast_to(self) -> Concat { cast::<Concat>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Concat> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\concat") } }
impl php_rt::InstanceOf<Concat> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\concat") } }
impl php_rt::InstanceOf<Concat> for Concat { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Div { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Div { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\Div")) } }
impl php_rt::Identical for Div { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Div { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Div { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Div { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Div> for Mixed { fn cast_to(self) -> Div { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Div>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\Div") } }
impl php_rt::TryDowncast for Div { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Div>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Div { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Div> for AnyObject { fn cast_to(self) -> Div { cast::<Div>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Div> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\div") } }
impl php_rt::InstanceOf<Div> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\div") } }
impl php_rt::InstanceOf<Div> for Div { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Equal { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Equal { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\Equal")) } }
impl php_rt::Identical for Equal { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Equal { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Equal { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Equal { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Equal> for Mixed { fn cast_to(self) -> Equal { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Equal>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\Equal") } }
impl php_rt::TryDowncast for Equal { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Equal>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Equal { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Equal> for AnyObject { fn cast_to(self) -> Equal { cast::<Equal>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Equal> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\equal") } }
impl php_rt::InstanceOf<Equal> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\equal") } }
impl php_rt::InstanceOf<Equal> for Equal { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Greater { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Greater { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\Greater")) } }
impl php_rt::Identical for Greater { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Greater { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Greater { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Greater { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Greater> for Mixed { fn cast_to(self) -> Greater { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Greater>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\Greater") } }
impl php_rt::TryDowncast for Greater { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Greater>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Greater { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Greater> for AnyObject { fn cast_to(self) -> Greater { cast::<Greater>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Greater> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\greater") } }
impl php_rt::InstanceOf<Greater> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\greater") } }
impl php_rt::InstanceOf<Greater> for Greater { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for GreaterOrEqual { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for GreaterOrEqual { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\GreaterOrEqual")) } }
impl php_rt::Identical for GreaterOrEqual { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for GreaterOrEqual { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for GreaterOrEqual { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for GreaterOrEqual { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<GreaterOrEqual> for Mixed { fn cast_to(self) -> GreaterOrEqual { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::GreaterOrEqual>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\GreaterOrEqual") } }
impl php_rt::TryDowncast for GreaterOrEqual { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::GreaterOrEqual>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for GreaterOrEqual { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<GreaterOrEqual> for AnyObject { fn cast_to(self) -> GreaterOrEqual { cast::<GreaterOrEqual>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<GreaterOrEqual> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\greaterorequal") } }
impl php_rt::InstanceOf<GreaterOrEqual> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\greaterorequal") } }
impl php_rt::InstanceOf<GreaterOrEqual> for GreaterOrEqual { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Identical { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Identical { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\Identical")) } }
impl php_rt::Identical for Identical { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Identical { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Identical { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Identical { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Identical> for Mixed { fn cast_to(self) -> Identical { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Identical>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\Identical") } }
impl php_rt::TryDowncast for Identical { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Identical>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Identical { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Identical> for AnyObject { fn cast_to(self) -> Identical { cast::<Identical>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Identical> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\identical") } }
impl php_rt::InstanceOf<Identical> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\identical") } }
impl php_rt::InstanceOf<Identical> for Identical { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for LogicalAnd { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for LogicalAnd { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\LogicalAnd")) } }
impl php_rt::Identical for LogicalAnd { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for LogicalAnd { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for LogicalAnd { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for LogicalAnd { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<LogicalAnd> for Mixed { fn cast_to(self) -> LogicalAnd { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::LogicalAnd>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\LogicalAnd") } }
impl php_rt::TryDowncast for LogicalAnd { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::LogicalAnd>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for LogicalAnd { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<LogicalAnd> for AnyObject { fn cast_to(self) -> LogicalAnd { cast::<LogicalAnd>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<LogicalAnd> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\logicaland") } }
impl php_rt::InstanceOf<LogicalAnd> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\logicaland") } }
impl php_rt::InstanceOf<LogicalAnd> for LogicalAnd { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for LogicalOr { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for LogicalOr { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\LogicalOr")) } }
impl php_rt::Identical for LogicalOr { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for LogicalOr { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for LogicalOr { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for LogicalOr { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<LogicalOr> for Mixed { fn cast_to(self) -> LogicalOr { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::LogicalOr>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\LogicalOr") } }
impl php_rt::TryDowncast for LogicalOr { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::LogicalOr>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for LogicalOr { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<LogicalOr> for AnyObject { fn cast_to(self) -> LogicalOr { cast::<LogicalOr>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<LogicalOr> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\logicalor") } }
impl php_rt::InstanceOf<LogicalOr> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\logicalor") } }
impl php_rt::InstanceOf<LogicalOr> for LogicalOr { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for LogicalXor { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for LogicalXor { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\LogicalXor")) } }
impl php_rt::Identical for LogicalXor { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for LogicalXor { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for LogicalXor { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for LogicalXor { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<LogicalXor> for Mixed { fn cast_to(self) -> LogicalXor { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::LogicalXor>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\LogicalXor") } }
impl php_rt::TryDowncast for LogicalXor { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::LogicalXor>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for LogicalXor { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<LogicalXor> for AnyObject { fn cast_to(self) -> LogicalXor { cast::<LogicalXor>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<LogicalXor> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\logicalxor") } }
impl php_rt::InstanceOf<LogicalXor> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\logicalxor") } }
impl php_rt::InstanceOf<LogicalXor> for LogicalXor { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Minus { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Minus { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\Minus")) } }
impl php_rt::Identical for Minus { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Minus { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Minus { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Minus { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Minus> for Mixed { fn cast_to(self) -> Minus { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Minus>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\Minus") } }
impl php_rt::TryDowncast for Minus { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Minus>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Minus { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Minus> for AnyObject { fn cast_to(self) -> Minus { cast::<Minus>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Minus> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\minus") } }
impl php_rt::InstanceOf<Minus> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\minus") } }
impl php_rt::InstanceOf<Minus> for Minus { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Mod { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Mod { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\Mod")) } }
impl php_rt::Identical for Mod { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Mod { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Mod { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Mod { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Mod> for Mixed { fn cast_to(self) -> Mod { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Mod>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\Mod") } }
impl php_rt::TryDowncast for Mod { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Mod>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Mod { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Mod> for AnyObject { fn cast_to(self) -> Mod { cast::<Mod>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Mod> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\mod") } }
impl php_rt::InstanceOf<Mod> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\mod") } }
impl php_rt::InstanceOf<Mod> for Mod { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Mul { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Mul { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\Mul")) } }
impl php_rt::Identical for Mul { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Mul { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Mul { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Mul { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Mul> for Mixed { fn cast_to(self) -> Mul { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Mul>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\Mul") } }
impl php_rt::TryDowncast for Mul { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Mul>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Mul { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Mul> for AnyObject { fn cast_to(self) -> Mul { cast::<Mul>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Mul> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\mul") } }
impl php_rt::InstanceOf<Mul> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\mul") } }
impl php_rt::InstanceOf<Mul> for Mul { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for NotEqual { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for NotEqual { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\NotEqual")) } }
impl php_rt::Identical for NotEqual { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for NotEqual { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for NotEqual { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for NotEqual { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<NotEqual> for Mixed { fn cast_to(self) -> NotEqual { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::NotEqual>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\NotEqual") } }
impl php_rt::TryDowncast for NotEqual { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::NotEqual>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for NotEqual { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<NotEqual> for AnyObject { fn cast_to(self) -> NotEqual { cast::<NotEqual>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<NotEqual> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\notequal") } }
impl php_rt::InstanceOf<NotEqual> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\notequal") } }
impl php_rt::InstanceOf<NotEqual> for NotEqual { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for NotIdentical { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for NotIdentical { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\NotIdentical")) } }
impl php_rt::Identical for NotIdentical { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for NotIdentical { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for NotIdentical { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for NotIdentical { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<NotIdentical> for Mixed { fn cast_to(self) -> NotIdentical { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::NotIdentical>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\NotIdentical") } }
impl php_rt::TryDowncast for NotIdentical { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::NotIdentical>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for NotIdentical { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<NotIdentical> for AnyObject { fn cast_to(self) -> NotIdentical { cast::<NotIdentical>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<NotIdentical> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\notidentical") } }
impl php_rt::InstanceOf<NotIdentical> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\notidentical") } }
impl php_rt::InstanceOf<NotIdentical> for NotIdentical { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Pipe { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Pipe { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\Pipe")) } }
impl php_rt::Identical for Pipe { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Pipe { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Pipe { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Pipe { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Pipe> for Mixed { fn cast_to(self) -> Pipe { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Pipe>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\Pipe") } }
impl php_rt::TryDowncast for Pipe { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Pipe>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Pipe { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Pipe> for AnyObject { fn cast_to(self) -> Pipe { cast::<Pipe>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Pipe> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\pipe") } }
impl php_rt::InstanceOf<Pipe> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\pipe") } }
impl php_rt::InstanceOf<Pipe> for Pipe { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Plus { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Plus { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\Plus")) } }
impl php_rt::Identical for Plus { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Plus { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Plus { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Plus { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Plus> for Mixed { fn cast_to(self) -> Plus { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Plus>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\Plus") } }
impl php_rt::TryDowncast for Plus { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Plus>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Plus { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Plus> for AnyObject { fn cast_to(self) -> Plus { cast::<Plus>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Plus> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\plus") } }
impl php_rt::InstanceOf<Plus> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\plus") } }
impl php_rt::InstanceOf<Plus> for Plus { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Pow { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Pow { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\Pow")) } }
impl php_rt::Identical for Pow { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Pow { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Pow { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Pow { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Pow> for Mixed { fn cast_to(self) -> Pow { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Pow>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\Pow") } }
impl php_rt::TryDowncast for Pow { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Pow>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Pow { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Pow> for AnyObject { fn cast_to(self) -> Pow { cast::<Pow>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Pow> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\pow") } }
impl php_rt::InstanceOf<Pow> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\pow") } }
impl php_rt::InstanceOf<Pow> for Pow { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for ShiftLeft { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for ShiftLeft { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\ShiftLeft")) } }
impl php_rt::Identical for ShiftLeft { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for ShiftLeft { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for ShiftLeft { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for ShiftLeft { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<ShiftLeft> for Mixed { fn cast_to(self) -> ShiftLeft { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::ShiftLeft>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\ShiftLeft") } }
impl php_rt::TryDowncast for ShiftLeft { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::ShiftLeft>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for ShiftLeft { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<ShiftLeft> for AnyObject { fn cast_to(self) -> ShiftLeft { cast::<ShiftLeft>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<ShiftLeft> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\shiftleft") } }
impl php_rt::InstanceOf<ShiftLeft> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\shiftleft") } }
impl php_rt::InstanceOf<ShiftLeft> for ShiftLeft { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for ShiftRight { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for ShiftRight { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\ShiftRight")) } }
impl php_rt::Identical for ShiftRight { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for ShiftRight { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for ShiftRight { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for ShiftRight { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<ShiftRight> for Mixed { fn cast_to(self) -> ShiftRight { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::ShiftRight>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\ShiftRight") } }
impl php_rt::TryDowncast for ShiftRight { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::ShiftRight>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for ShiftRight { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<ShiftRight> for AnyObject { fn cast_to(self) -> ShiftRight { cast::<ShiftRight>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<ShiftRight> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\shiftright") } }
impl php_rt::InstanceOf<ShiftRight> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\shiftright") } }
impl php_rt::InstanceOf<ShiftRight> for ShiftRight { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Smaller { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Smaller { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\Smaller")) } }
impl php_rt::Identical for Smaller { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Smaller { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Smaller { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Smaller { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Smaller> for Mixed { fn cast_to(self) -> Smaller { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Smaller>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\Smaller") } }
impl php_rt::TryDowncast for Smaller { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Smaller>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Smaller { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Smaller> for AnyObject { fn cast_to(self) -> Smaller { cast::<Smaller>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Smaller> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\smaller") } }
impl php_rt::InstanceOf<Smaller> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\smaller") } }
impl php_rt::InstanceOf<Smaller> for Smaller { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for SmallerOrEqual { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for SmallerOrEqual { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\SmallerOrEqual")) } }
impl php_rt::Identical for SmallerOrEqual { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for SmallerOrEqual { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for SmallerOrEqual { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for SmallerOrEqual { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<SmallerOrEqual> for Mixed { fn cast_to(self) -> SmallerOrEqual { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::SmallerOrEqual>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\SmallerOrEqual") } }
impl php_rt::TryDowncast for SmallerOrEqual { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::SmallerOrEqual>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for SmallerOrEqual { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<SmallerOrEqual> for AnyObject { fn cast_to(self) -> SmallerOrEqual { cast::<SmallerOrEqual>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<SmallerOrEqual> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\smallerorequal") } }
impl php_rt::InstanceOf<SmallerOrEqual> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\smallerorequal") } }
impl php_rt::InstanceOf<SmallerOrEqual> for SmallerOrEqual { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Spaceship { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Spaceship { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\BinaryOp\\Spaceship")) } }
impl php_rt::Identical for Spaceship { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Spaceship { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Spaceship { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Spaceship { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Spaceship> for Mixed { fn cast_to(self) -> Spaceship { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Spaceship>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\BinaryOp\\Spaceship") } }
impl php_rt::TryDowncast for Spaceship { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::binary_op::Spaceship>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Spaceship { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Spaceship> for AnyObject { fn cast_to(self) -> Spaceship { cast::<Spaceship>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Spaceship> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\binaryop\\spaceship") } }
impl php_rt::InstanceOf<Spaceship> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\binaryop\\spaceship") } }
impl php_rt::InstanceOf<Spaceship> for Spaceship { fn is_instance(&self) -> bool { true } }
