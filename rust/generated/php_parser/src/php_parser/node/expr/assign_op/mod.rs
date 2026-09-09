use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub struct BitwiseAndObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub var: Late<crate::php_parser::node::Expr>,
    pub expr: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct BitwiseAnd(pub Rc<RefCell<BitwiseAndObj>>);
impl BitwiseAnd {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_var(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.var.get()) }
    pub fn p_var_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().var.get().clone() }
    pub fn p_var_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().var.as_option().cloned() }
    pub fn p_var_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.var.get_mut()) }
    pub fn set_p_var(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().var.set(v); }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn new_uninit() -> BitwiseAnd {
        BitwiseAnd(Rc::new(RefCell::new(BitwiseAndObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })))
    }
    pub fn new(mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<BitwiseAnd, Throw> {
        let this = BitwiseAnd(Rc::new(RefCell::new(BitwiseAndObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })));
        this.magic__construct(var, expr, attributes)?;
        Ok(this)
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_AssignOp_BitwiseAnd"));
    }
    pub fn magic__construct(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).magic__construct__impl(var, expr, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::assign_op::BitwiseAnd, Throw> { Ok(Self::new(var, expr, attributes)?) }
}
impl php_rt::PhpObject for BitwiseAnd {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\AssignOp\\BitwiseAnd" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\assignop\\bitwiseand", "phpparser\\node\\expr\\assignop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "var" => { self.set_p_var(cast::<crate::php_parser::node::Expr>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "var" => self.p_var_opt().map(|v| cast::<Mixed>(v)), "expr" => self.p_expr_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\AssignOp\\BitwiseAnd", name)))) } }
}
impl BitwiseAnd { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\AssignOp\\BitwiseAnd could not be converted to string"))) } }
impl php_rt::PhpClone for BitwiseAnd { fn php_clone(&self) -> Self { let c = BitwiseAnd(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for BitwiseAndObj { fn clone(&self) -> Self { BitwiseAndObj { attributes: self.attributes.clone(), var: self.var.clone(), expr: self.expr.clone() } } }
impl BitwiseAnd {
}
pub struct BitwiseOrObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub var: Late<crate::php_parser::node::Expr>,
    pub expr: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct BitwiseOr(pub Rc<RefCell<BitwiseOrObj>>);
impl BitwiseOr {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_var(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.var.get()) }
    pub fn p_var_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().var.get().clone() }
    pub fn p_var_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().var.as_option().cloned() }
    pub fn p_var_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.var.get_mut()) }
    pub fn set_p_var(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().var.set(v); }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn new_uninit() -> BitwiseOr {
        BitwiseOr(Rc::new(RefCell::new(BitwiseOrObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })))
    }
    pub fn new(mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<BitwiseOr, Throw> {
        let this = BitwiseOr(Rc::new(RefCell::new(BitwiseOrObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })));
        this.magic__construct(var, expr, attributes)?;
        Ok(this)
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_AssignOp_BitwiseOr"));
    }
    pub fn magic__construct(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).magic__construct__impl(var, expr, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::assign_op::BitwiseOr, Throw> { Ok(Self::new(var, expr, attributes)?) }
}
impl php_rt::PhpObject for BitwiseOr {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\AssignOp\\BitwiseOr" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\assignop\\bitwiseor", "phpparser\\node\\expr\\assignop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "var" => { self.set_p_var(cast::<crate::php_parser::node::Expr>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "var" => self.p_var_opt().map(|v| cast::<Mixed>(v)), "expr" => self.p_expr_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\AssignOp\\BitwiseOr", name)))) } }
}
impl BitwiseOr { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\AssignOp\\BitwiseOr could not be converted to string"))) } }
impl php_rt::PhpClone for BitwiseOr { fn php_clone(&self) -> Self { let c = BitwiseOr(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for BitwiseOrObj { fn clone(&self) -> Self { BitwiseOrObj { attributes: self.attributes.clone(), var: self.var.clone(), expr: self.expr.clone() } } }
impl BitwiseOr {
}
pub struct BitwiseXorObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub var: Late<crate::php_parser::node::Expr>,
    pub expr: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct BitwiseXor(pub Rc<RefCell<BitwiseXorObj>>);
impl BitwiseXor {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_var(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.var.get()) }
    pub fn p_var_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().var.get().clone() }
    pub fn p_var_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().var.as_option().cloned() }
    pub fn p_var_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.var.get_mut()) }
    pub fn set_p_var(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().var.set(v); }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn new_uninit() -> BitwiseXor {
        BitwiseXor(Rc::new(RefCell::new(BitwiseXorObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })))
    }
    pub fn new(mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<BitwiseXor, Throw> {
        let this = BitwiseXor(Rc::new(RefCell::new(BitwiseXorObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })));
        this.magic__construct(var, expr, attributes)?;
        Ok(this)
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_AssignOp_BitwiseXor"));
    }
    pub fn magic__construct(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).magic__construct__impl(var, expr, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::assign_op::BitwiseXor, Throw> { Ok(Self::new(var, expr, attributes)?) }
}
impl php_rt::PhpObject for BitwiseXor {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\AssignOp\\BitwiseXor" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\assignop\\bitwisexor", "phpparser\\node\\expr\\assignop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "var" => { self.set_p_var(cast::<crate::php_parser::node::Expr>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "var" => self.p_var_opt().map(|v| cast::<Mixed>(v)), "expr" => self.p_expr_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\AssignOp\\BitwiseXor", name)))) } }
}
impl BitwiseXor { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\AssignOp\\BitwiseXor could not be converted to string"))) } }
impl php_rt::PhpClone for BitwiseXor { fn php_clone(&self) -> Self { let c = BitwiseXor(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for BitwiseXorObj { fn clone(&self) -> Self { BitwiseXorObj { attributes: self.attributes.clone(), var: self.var.clone(), expr: self.expr.clone() } } }
impl BitwiseXor {
}
pub struct CoalesceObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub var: Late<crate::php_parser::node::Expr>,
    pub expr: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Coalesce(pub Rc<RefCell<CoalesceObj>>);
impl Coalesce {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_var(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.var.get()) }
    pub fn p_var_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().var.get().clone() }
    pub fn p_var_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().var.as_option().cloned() }
    pub fn p_var_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.var.get_mut()) }
    pub fn set_p_var(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().var.set(v); }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn new_uninit() -> Coalesce {
        Coalesce(Rc::new(RefCell::new(CoalesceObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })))
    }
    pub fn new(mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Coalesce, Throw> {
        let this = Coalesce(Rc::new(RefCell::new(CoalesceObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })));
        this.magic__construct(var, expr, attributes)?;
        Ok(this)
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_AssignOp_Coalesce"));
    }
    pub fn magic__construct(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).magic__construct__impl(var, expr, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::assign_op::Coalesce, Throw> { Ok(Self::new(var, expr, attributes)?) }
}
impl php_rt::PhpObject for Coalesce {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\AssignOp\\Coalesce" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\assignop\\coalesce", "phpparser\\node\\expr\\assignop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "var" => { self.set_p_var(cast::<crate::php_parser::node::Expr>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "var" => self.p_var_opt().map(|v| cast::<Mixed>(v)), "expr" => self.p_expr_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\AssignOp\\Coalesce", name)))) } }
}
impl Coalesce { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\AssignOp\\Coalesce could not be converted to string"))) } }
impl php_rt::PhpClone for Coalesce { fn php_clone(&self) -> Self { let c = Coalesce(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for CoalesceObj { fn clone(&self) -> Self { CoalesceObj { attributes: self.attributes.clone(), var: self.var.clone(), expr: self.expr.clone() } } }
impl Coalesce {
}
pub struct ConcatObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub var: Late<crate::php_parser::node::Expr>,
    pub expr: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Concat(pub Rc<RefCell<ConcatObj>>);
impl Concat {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_var(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.var.get()) }
    pub fn p_var_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().var.get().clone() }
    pub fn p_var_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().var.as_option().cloned() }
    pub fn p_var_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.var.get_mut()) }
    pub fn set_p_var(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().var.set(v); }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn new_uninit() -> Concat {
        Concat(Rc::new(RefCell::new(ConcatObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })))
    }
    pub fn new(mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Concat, Throw> {
        let this = Concat(Rc::new(RefCell::new(ConcatObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })));
        this.magic__construct(var, expr, attributes)?;
        Ok(this)
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_AssignOp_Concat"));
    }
    pub fn magic__construct(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).magic__construct__impl(var, expr, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::assign_op::Concat, Throw> { Ok(Self::new(var, expr, attributes)?) }
}
impl php_rt::PhpObject for Concat {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\AssignOp\\Concat" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\assignop\\concat", "phpparser\\node\\expr\\assignop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "var" => { self.set_p_var(cast::<crate::php_parser::node::Expr>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "var" => self.p_var_opt().map(|v| cast::<Mixed>(v)), "expr" => self.p_expr_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\AssignOp\\Concat", name)))) } }
}
impl Concat { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\AssignOp\\Concat could not be converted to string"))) } }
impl php_rt::PhpClone for Concat { fn php_clone(&self) -> Self { let c = Concat(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ConcatObj { fn clone(&self) -> Self { ConcatObj { attributes: self.attributes.clone(), var: self.var.clone(), expr: self.expr.clone() } } }
impl Concat {
}
pub struct DivObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub var: Late<crate::php_parser::node::Expr>,
    pub expr: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Div(pub Rc<RefCell<DivObj>>);
impl Div {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_var(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.var.get()) }
    pub fn p_var_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().var.get().clone() }
    pub fn p_var_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().var.as_option().cloned() }
    pub fn p_var_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.var.get_mut()) }
    pub fn set_p_var(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().var.set(v); }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn new_uninit() -> Div {
        Div(Rc::new(RefCell::new(DivObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })))
    }
    pub fn new(mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Div, Throw> {
        let this = Div(Rc::new(RefCell::new(DivObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })));
        this.magic__construct(var, expr, attributes)?;
        Ok(this)
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_AssignOp_Div"));
    }
    pub fn magic__construct(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).magic__construct__impl(var, expr, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::assign_op::Div, Throw> { Ok(Self::new(var, expr, attributes)?) }
}
impl php_rt::PhpObject for Div {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\AssignOp\\Div" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\assignop\\div", "phpparser\\node\\expr\\assignop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "var" => { self.set_p_var(cast::<crate::php_parser::node::Expr>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "var" => self.p_var_opt().map(|v| cast::<Mixed>(v)), "expr" => self.p_expr_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\AssignOp\\Div", name)))) } }
}
impl Div { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\AssignOp\\Div could not be converted to string"))) } }
impl php_rt::PhpClone for Div { fn php_clone(&self) -> Self { let c = Div(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for DivObj { fn clone(&self) -> Self { DivObj { attributes: self.attributes.clone(), var: self.var.clone(), expr: self.expr.clone() } } }
impl Div {
}
pub struct MinusObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub var: Late<crate::php_parser::node::Expr>,
    pub expr: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Minus(pub Rc<RefCell<MinusObj>>);
impl Minus {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_var(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.var.get()) }
    pub fn p_var_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().var.get().clone() }
    pub fn p_var_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().var.as_option().cloned() }
    pub fn p_var_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.var.get_mut()) }
    pub fn set_p_var(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().var.set(v); }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn new_uninit() -> Minus {
        Minus(Rc::new(RefCell::new(MinusObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })))
    }
    pub fn new(mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Minus, Throw> {
        let this = Minus(Rc::new(RefCell::new(MinusObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })));
        this.magic__construct(var, expr, attributes)?;
        Ok(this)
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_AssignOp_Minus"));
    }
    pub fn magic__construct(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).magic__construct__impl(var, expr, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::assign_op::Minus, Throw> { Ok(Self::new(var, expr, attributes)?) }
}
impl php_rt::PhpObject for Minus {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\AssignOp\\Minus" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\assignop\\minus", "phpparser\\node\\expr\\assignop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "var" => { self.set_p_var(cast::<crate::php_parser::node::Expr>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "var" => self.p_var_opt().map(|v| cast::<Mixed>(v)), "expr" => self.p_expr_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\AssignOp\\Minus", name)))) } }
}
impl Minus { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\AssignOp\\Minus could not be converted to string"))) } }
impl php_rt::PhpClone for Minus { fn php_clone(&self) -> Self { let c = Minus(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for MinusObj { fn clone(&self) -> Self { MinusObj { attributes: self.attributes.clone(), var: self.var.clone(), expr: self.expr.clone() } } }
impl Minus {
}
pub struct ModObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub var: Late<crate::php_parser::node::Expr>,
    pub expr: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Mod(pub Rc<RefCell<ModObj>>);
impl Mod {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_var(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.var.get()) }
    pub fn p_var_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().var.get().clone() }
    pub fn p_var_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().var.as_option().cloned() }
    pub fn p_var_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.var.get_mut()) }
    pub fn set_p_var(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().var.set(v); }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn new_uninit() -> Mod {
        Mod(Rc::new(RefCell::new(ModObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })))
    }
    pub fn new(mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mod, Throw> {
        let this = Mod(Rc::new(RefCell::new(ModObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })));
        this.magic__construct(var, expr, attributes)?;
        Ok(this)
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_AssignOp_Mod"));
    }
    pub fn magic__construct(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).magic__construct__impl(var, expr, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::assign_op::Mod, Throw> { Ok(Self::new(var, expr, attributes)?) }
}
impl php_rt::PhpObject for Mod {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\AssignOp\\Mod" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\assignop\\mod", "phpparser\\node\\expr\\assignop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "var" => { self.set_p_var(cast::<crate::php_parser::node::Expr>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "var" => self.p_var_opt().map(|v| cast::<Mixed>(v)), "expr" => self.p_expr_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\AssignOp\\Mod", name)))) } }
}
impl Mod { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\AssignOp\\Mod could not be converted to string"))) } }
impl php_rt::PhpClone for Mod { fn php_clone(&self) -> Self { let c = Mod(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ModObj { fn clone(&self) -> Self { ModObj { attributes: self.attributes.clone(), var: self.var.clone(), expr: self.expr.clone() } } }
impl Mod {
}
pub struct MulObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub var: Late<crate::php_parser::node::Expr>,
    pub expr: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Mul(pub Rc<RefCell<MulObj>>);
impl Mul {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_var(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.var.get()) }
    pub fn p_var_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().var.get().clone() }
    pub fn p_var_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().var.as_option().cloned() }
    pub fn p_var_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.var.get_mut()) }
    pub fn set_p_var(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().var.set(v); }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn new_uninit() -> Mul {
        Mul(Rc::new(RefCell::new(MulObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })))
    }
    pub fn new(mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mul, Throw> {
        let this = Mul(Rc::new(RefCell::new(MulObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })));
        this.magic__construct(var, expr, attributes)?;
        Ok(this)
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_AssignOp_Mul"));
    }
    pub fn magic__construct(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).magic__construct__impl(var, expr, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::assign_op::Mul, Throw> { Ok(Self::new(var, expr, attributes)?) }
}
impl php_rt::PhpObject for Mul {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\AssignOp\\Mul" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\assignop\\mul", "phpparser\\node\\expr\\assignop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "var" => { self.set_p_var(cast::<crate::php_parser::node::Expr>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "var" => self.p_var_opt().map(|v| cast::<Mixed>(v)), "expr" => self.p_expr_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\AssignOp\\Mul", name)))) } }
}
impl Mul { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\AssignOp\\Mul could not be converted to string"))) } }
impl php_rt::PhpClone for Mul { fn php_clone(&self) -> Self { let c = Mul(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for MulObj { fn clone(&self) -> Self { MulObj { attributes: self.attributes.clone(), var: self.var.clone(), expr: self.expr.clone() } } }
impl Mul {
}
pub struct PlusObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub var: Late<crate::php_parser::node::Expr>,
    pub expr: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Plus(pub Rc<RefCell<PlusObj>>);
impl Plus {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_var(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.var.get()) }
    pub fn p_var_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().var.get().clone() }
    pub fn p_var_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().var.as_option().cloned() }
    pub fn p_var_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.var.get_mut()) }
    pub fn set_p_var(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().var.set(v); }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn new_uninit() -> Plus {
        Plus(Rc::new(RefCell::new(PlusObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })))
    }
    pub fn new(mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Plus, Throw> {
        let this = Plus(Rc::new(RefCell::new(PlusObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })));
        this.magic__construct(var, expr, attributes)?;
        Ok(this)
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_AssignOp_Plus"));
    }
    pub fn magic__construct(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).magic__construct__impl(var, expr, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::assign_op::Plus, Throw> { Ok(Self::new(var, expr, attributes)?) }
}
impl php_rt::PhpObject for Plus {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\AssignOp\\Plus" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\assignop\\plus", "phpparser\\node\\expr\\assignop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "var" => { self.set_p_var(cast::<crate::php_parser::node::Expr>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "var" => self.p_var_opt().map(|v| cast::<Mixed>(v)), "expr" => self.p_expr_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\AssignOp\\Plus", name)))) } }
}
impl Plus { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\AssignOp\\Plus could not be converted to string"))) } }
impl php_rt::PhpClone for Plus { fn php_clone(&self) -> Self { let c = Plus(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for PlusObj { fn clone(&self) -> Self { PlusObj { attributes: self.attributes.clone(), var: self.var.clone(), expr: self.expr.clone() } } }
impl Plus {
}
pub struct PowObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub var: Late<crate::php_parser::node::Expr>,
    pub expr: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Pow(pub Rc<RefCell<PowObj>>);
impl Pow {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_var(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.var.get()) }
    pub fn p_var_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().var.get().clone() }
    pub fn p_var_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().var.as_option().cloned() }
    pub fn p_var_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.var.get_mut()) }
    pub fn set_p_var(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().var.set(v); }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn new_uninit() -> Pow {
        Pow(Rc::new(RefCell::new(PowObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })))
    }
    pub fn new(mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Pow, Throw> {
        let this = Pow(Rc::new(RefCell::new(PowObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })));
        this.magic__construct(var, expr, attributes)?;
        Ok(this)
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_AssignOp_Pow"));
    }
    pub fn magic__construct(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).magic__construct__impl(var, expr, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::assign_op::Pow, Throw> { Ok(Self::new(var, expr, attributes)?) }
}
impl php_rt::PhpObject for Pow {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\AssignOp\\Pow" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\assignop\\pow", "phpparser\\node\\expr\\assignop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "var" => { self.set_p_var(cast::<crate::php_parser::node::Expr>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "var" => self.p_var_opt().map(|v| cast::<Mixed>(v)), "expr" => self.p_expr_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\AssignOp\\Pow", name)))) } }
}
impl Pow { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\AssignOp\\Pow could not be converted to string"))) } }
impl php_rt::PhpClone for Pow { fn php_clone(&self) -> Self { let c = Pow(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for PowObj { fn clone(&self) -> Self { PowObj { attributes: self.attributes.clone(), var: self.var.clone(), expr: self.expr.clone() } } }
impl Pow {
}
pub struct ShiftLeftObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub var: Late<crate::php_parser::node::Expr>,
    pub expr: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct ShiftLeft(pub Rc<RefCell<ShiftLeftObj>>);
impl ShiftLeft {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_var(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.var.get()) }
    pub fn p_var_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().var.get().clone() }
    pub fn p_var_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().var.as_option().cloned() }
    pub fn p_var_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.var.get_mut()) }
    pub fn set_p_var(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().var.set(v); }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn new_uninit() -> ShiftLeft {
        ShiftLeft(Rc::new(RefCell::new(ShiftLeftObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })))
    }
    pub fn new(mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<ShiftLeft, Throw> {
        let this = ShiftLeft(Rc::new(RefCell::new(ShiftLeftObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })));
        this.magic__construct(var, expr, attributes)?;
        Ok(this)
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_AssignOp_ShiftLeft"));
    }
    pub fn magic__construct(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).magic__construct__impl(var, expr, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::assign_op::ShiftLeft, Throw> { Ok(Self::new(var, expr, attributes)?) }
}
impl php_rt::PhpObject for ShiftLeft {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\AssignOp\\ShiftLeft" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\assignop\\shiftleft", "phpparser\\node\\expr\\assignop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "var" => { self.set_p_var(cast::<crate::php_parser::node::Expr>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "var" => self.p_var_opt().map(|v| cast::<Mixed>(v)), "expr" => self.p_expr_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\AssignOp\\ShiftLeft", name)))) } }
}
impl ShiftLeft { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\AssignOp\\ShiftLeft could not be converted to string"))) } }
impl php_rt::PhpClone for ShiftLeft { fn php_clone(&self) -> Self { let c = ShiftLeft(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ShiftLeftObj { fn clone(&self) -> Self { ShiftLeftObj { attributes: self.attributes.clone(), var: self.var.clone(), expr: self.expr.clone() } } }
impl ShiftLeft {
}
pub struct ShiftRightObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub var: Late<crate::php_parser::node::Expr>,
    pub expr: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct ShiftRight(pub Rc<RefCell<ShiftRightObj>>);
impl ShiftRight {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_var(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.var.get()) }
    pub fn p_var_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().var.get().clone() }
    pub fn p_var_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().var.as_option().cloned() }
    pub fn p_var_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.var.get_mut()) }
    pub fn set_p_var(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().var.set(v); }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn new_uninit() -> ShiftRight {
        ShiftRight(Rc::new(RefCell::new(ShiftRightObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })))
    }
    pub fn new(mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<ShiftRight, Throw> {
        let this = ShiftRight(Rc::new(RefCell::new(ShiftRightObj {
            attributes: Late::uninit(),
            var: Late::uninit(),
            expr: Late::uninit(),
        })));
        this.magic__construct(var, expr, attributes)?;
        Ok(this)
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_AssignOp_ShiftRight"));
    }
    pub fn magic__construct(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).magic__construct__impl(var, expr, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::expr::AssignOp>(self.clone()).getSubNodeNames__impl() }
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
    pub fn new_same_class(&self, mut var: crate::php_parser::node::Expr, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::expr::assign_op::ShiftRight, Throw> { Ok(Self::new(var, expr, attributes)?) }
}
impl php_rt::PhpObject for ShiftRight {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Expr\\AssignOp\\ShiftRight" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\expr\\assignop\\shiftright", "phpparser\\node\\expr\\assignop", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_var_opt() { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "var" => { self.set_p_var(cast::<crate::php_parser::node::Expr>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "var" => self.p_var_opt().map(|v| cast::<Mixed>(v)), "expr" => self.p_expr_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(1) { Some(__a) => cast::<crate::php_parser::node::Expr>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Expr") }), (match args.get(2) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Expr\\AssignOp\\ShiftRight", name)))) } }
}
impl ShiftRight { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Expr\\AssignOp\\ShiftRight could not be converted to string"))) } }
impl php_rt::PhpClone for ShiftRight { fn php_clone(&self) -> Self { let c = ShiftRight(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ShiftRightObj { fn clone(&self) -> Self { ShiftRightObj { attributes: self.attributes.clone(), var: self.var.clone(), expr: self.expr.clone() } } }
impl ShiftRight {
}
impl php_rt::Truthy for BitwiseAnd { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for BitwiseAnd { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for BitwiseAnd { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\AssignOp\\BitwiseAnd")) } }
impl php_rt::PhpCmp for BitwiseAnd { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for BitwiseAnd { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for BitwiseAnd { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<BitwiseAnd> for Mixed { fn cast_to(self) -> BitwiseAnd { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::BitwiseAnd>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\AssignOp\\BitwiseAnd") } }
impl php_rt::TryDowncast for BitwiseAnd { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::BitwiseAnd>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for BitwiseAnd { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<BitwiseAnd> for AnyObject { fn cast_to(self) -> BitwiseAnd { cast::<BitwiseAnd>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<BitwiseAnd> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\assignop\\bitwiseand") } }
impl php_rt::InstanceOf<BitwiseAnd> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\assignop\\bitwiseand") } }
impl php_rt::InstanceOf<BitwiseAnd> for BitwiseAnd { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for BitwiseOr { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for BitwiseOr { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for BitwiseOr { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\AssignOp\\BitwiseOr")) } }
impl php_rt::PhpCmp for BitwiseOr { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for BitwiseOr { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for BitwiseOr { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<BitwiseOr> for Mixed { fn cast_to(self) -> BitwiseOr { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::BitwiseOr>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\AssignOp\\BitwiseOr") } }
impl php_rt::TryDowncast for BitwiseOr { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::BitwiseOr>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for BitwiseOr { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<BitwiseOr> for AnyObject { fn cast_to(self) -> BitwiseOr { cast::<BitwiseOr>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<BitwiseOr> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\assignop\\bitwiseor") } }
impl php_rt::InstanceOf<BitwiseOr> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\assignop\\bitwiseor") } }
impl php_rt::InstanceOf<BitwiseOr> for BitwiseOr { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for BitwiseXor { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for BitwiseXor { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for BitwiseXor { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\AssignOp\\BitwiseXor")) } }
impl php_rt::PhpCmp for BitwiseXor { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for BitwiseXor { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for BitwiseXor { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<BitwiseXor> for Mixed { fn cast_to(self) -> BitwiseXor { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::BitwiseXor>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\AssignOp\\BitwiseXor") } }
impl php_rt::TryDowncast for BitwiseXor { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::BitwiseXor>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for BitwiseXor { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<BitwiseXor> for AnyObject { fn cast_to(self) -> BitwiseXor { cast::<BitwiseXor>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<BitwiseXor> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\assignop\\bitwisexor") } }
impl php_rt::InstanceOf<BitwiseXor> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\assignop\\bitwisexor") } }
impl php_rt::InstanceOf<BitwiseXor> for BitwiseXor { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Coalesce { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Coalesce { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Coalesce { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\AssignOp\\Coalesce")) } }
impl php_rt::PhpCmp for Coalesce { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Coalesce { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Coalesce { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Coalesce> for Mixed { fn cast_to(self) -> Coalesce { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Coalesce>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\AssignOp\\Coalesce") } }
impl php_rt::TryDowncast for Coalesce { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Coalesce>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Coalesce { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Coalesce> for AnyObject { fn cast_to(self) -> Coalesce { cast::<Coalesce>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Coalesce> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\assignop\\coalesce") } }
impl php_rt::InstanceOf<Coalesce> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\assignop\\coalesce") } }
impl php_rt::InstanceOf<Coalesce> for Coalesce { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Concat { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Concat { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Concat { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\AssignOp\\Concat")) } }
impl php_rt::PhpCmp for Concat { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Concat { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Concat { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Concat> for Mixed { fn cast_to(self) -> Concat { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Concat>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\AssignOp\\Concat") } }
impl php_rt::TryDowncast for Concat { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Concat>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Concat { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Concat> for AnyObject { fn cast_to(self) -> Concat { cast::<Concat>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Concat> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\assignop\\concat") } }
impl php_rt::InstanceOf<Concat> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\assignop\\concat") } }
impl php_rt::InstanceOf<Concat> for Concat { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Div { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Div { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Div { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\AssignOp\\Div")) } }
impl php_rt::PhpCmp for Div { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Div { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Div { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Div> for Mixed { fn cast_to(self) -> Div { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Div>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\AssignOp\\Div") } }
impl php_rt::TryDowncast for Div { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Div>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Div { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Div> for AnyObject { fn cast_to(self) -> Div { cast::<Div>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Div> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\assignop\\div") } }
impl php_rt::InstanceOf<Div> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\assignop\\div") } }
impl php_rt::InstanceOf<Div> for Div { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Minus { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Minus { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Minus { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\AssignOp\\Minus")) } }
impl php_rt::PhpCmp for Minus { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Minus { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Minus { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Minus> for Mixed { fn cast_to(self) -> Minus { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Minus>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\AssignOp\\Minus") } }
impl php_rt::TryDowncast for Minus { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Minus>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Minus { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Minus> for AnyObject { fn cast_to(self) -> Minus { cast::<Minus>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Minus> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\assignop\\minus") } }
impl php_rt::InstanceOf<Minus> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\assignop\\minus") } }
impl php_rt::InstanceOf<Minus> for Minus { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Mod { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Mod { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Mod { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\AssignOp\\Mod")) } }
impl php_rt::PhpCmp for Mod { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Mod { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Mod { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Mod> for Mixed { fn cast_to(self) -> Mod { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Mod>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\AssignOp\\Mod") } }
impl php_rt::TryDowncast for Mod { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Mod>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Mod { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Mod> for AnyObject { fn cast_to(self) -> Mod { cast::<Mod>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Mod> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\assignop\\mod") } }
impl php_rt::InstanceOf<Mod> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\assignop\\mod") } }
impl php_rt::InstanceOf<Mod> for Mod { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Mul { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Mul { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Mul { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\AssignOp\\Mul")) } }
impl php_rt::PhpCmp for Mul { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Mul { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Mul { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Mul> for Mixed { fn cast_to(self) -> Mul { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Mul>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\AssignOp\\Mul") } }
impl php_rt::TryDowncast for Mul { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Mul>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Mul { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Mul> for AnyObject { fn cast_to(self) -> Mul { cast::<Mul>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Mul> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\assignop\\mul") } }
impl php_rt::InstanceOf<Mul> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\assignop\\mul") } }
impl php_rt::InstanceOf<Mul> for Mul { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Plus { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Plus { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Plus { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\AssignOp\\Plus")) } }
impl php_rt::PhpCmp for Plus { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Plus { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Plus { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Plus> for Mixed { fn cast_to(self) -> Plus { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Plus>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\AssignOp\\Plus") } }
impl php_rt::TryDowncast for Plus { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Plus>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Plus { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Plus> for AnyObject { fn cast_to(self) -> Plus { cast::<Plus>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Plus> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\assignop\\plus") } }
impl php_rt::InstanceOf<Plus> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\assignop\\plus") } }
impl php_rt::InstanceOf<Plus> for Plus { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Pow { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Pow { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Pow { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\AssignOp\\Pow")) } }
impl php_rt::PhpCmp for Pow { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Pow { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Pow { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Pow> for Mixed { fn cast_to(self) -> Pow { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Pow>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\AssignOp\\Pow") } }
impl php_rt::TryDowncast for Pow { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::Pow>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Pow { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Pow> for AnyObject { fn cast_to(self) -> Pow { cast::<Pow>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Pow> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\assignop\\pow") } }
impl php_rt::InstanceOf<Pow> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\assignop\\pow") } }
impl php_rt::InstanceOf<Pow> for Pow { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for ShiftLeft { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for ShiftLeft { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for ShiftLeft { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\AssignOp\\ShiftLeft")) } }
impl php_rt::PhpCmp for ShiftLeft { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for ShiftLeft { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for ShiftLeft { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<ShiftLeft> for Mixed { fn cast_to(self) -> ShiftLeft { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::ShiftLeft>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\AssignOp\\ShiftLeft") } }
impl php_rt::TryDowncast for ShiftLeft { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::ShiftLeft>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for ShiftLeft { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<ShiftLeft> for AnyObject { fn cast_to(self) -> ShiftLeft { cast::<ShiftLeft>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<ShiftLeft> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\assignop\\shiftleft") } }
impl php_rt::InstanceOf<ShiftLeft> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\assignop\\shiftleft") } }
impl php_rt::InstanceOf<ShiftLeft> for ShiftLeft { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for ShiftRight { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for ShiftRight { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for ShiftRight { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Expr\\AssignOp\\ShiftRight")) } }
impl php_rt::PhpCmp for ShiftRight { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for ShiftRight { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for ShiftRight { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<ShiftRight> for Mixed { fn cast_to(self) -> ShiftRight { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::ShiftRight>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Expr\\AssignOp\\ShiftRight") } }
impl php_rt::TryDowncast for ShiftRight { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::expr::assign_op::ShiftRight>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for ShiftRight { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<ShiftRight> for AnyObject { fn cast_to(self) -> ShiftRight { cast::<ShiftRight>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<ShiftRight> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\expr\\assignop\\shiftright") } }
impl php_rt::InstanceOf<ShiftRight> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\expr\\assignop\\shiftright") } }
impl php_rt::InstanceOf<ShiftRight> for ShiftRight { fn is_instance(&self) -> bool { true } }
