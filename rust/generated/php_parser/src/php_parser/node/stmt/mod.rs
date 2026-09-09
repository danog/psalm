use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub mod trait_use_adaptation;
pub struct BlockObj {
    pub attributes: Map<Str, Mixed>,
    pub stmts: Map<ArrayKey, crate::php_parser::node::Stmt>,
}
#[derive(Clone)]
pub struct Block(pub Rc<RefCell<BlockObj>>);
impl Block {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_stmts(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> Map<ArrayKey, crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: Map<ArrayKey, crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn new(mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Block, Throw> {
        let this = Block(Rc::new(RefCell::new(BlockObj {
            attributes: Default::default(),
            stmts: Default::default(),
        })));
        this.magic__construct(stmts, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_stmts(stmts.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Block"));
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
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
    pub fn new_same_class(&self, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Block, Throw> { Ok(Self::new(stmts, attributes)?) }
}
impl php_rt::PhpObject for Block {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Block" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\block", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "stmts" => { self.set_p_stmts(cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(value)); true }, _ => false } }
}
impl Block { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Block could not be converted to string"))) } }
impl php_rt::PhpClone for Block { fn php_clone(&self) -> Self { let c = Block(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for BlockObj { fn clone(&self) -> Self { BlockObj { attributes: self.attributes.clone(), stmts: self.stmts.clone() } } }
impl Block {
}
pub struct Break_Obj {
    pub attributes: Map<Str, Mixed>,
    pub num: Option<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Break_(pub Rc<RefCell<Break_Obj>>);
impl Break_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_num(&self) -> Ref<'_, Option<crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.num) }
    pub fn p_num_get(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().num.clone() }
    pub fn p_num_opt(&self) -> Option<Option<crate::php_parser::node::Expr>> { Some(self.0.borrow().num.clone()) }
    pub fn p_num_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.num) }
    pub fn set_p_num(&self, v: Option<crate::php_parser::node::Expr>) { self.0.borrow_mut().num = v; }
    pub fn new(mut num: Option<crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<Break_, Throw> {
        let this = Break_(Rc::new(RefCell::new(Break_Obj {
            attributes: Default::default(),
            num: Default::default(),
        })));
        this.magic__construct(num, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut num: Option<crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_num(num.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("num"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Break"));
    }
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
    pub fn new_same_class(&self, mut num: Option<crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Break_, Throw> { Ok(Self::new(num, attributes)?) }
}
impl php_rt::PhpObject for Break_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Break_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\break_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_num_get()) { out.push((Str::from_static("num"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "num" => { self.set_p_num(value.to_option().map(|__m| cast::<crate::php_parser::node::Expr>(__m))); true }, _ => false } }
}
impl Break_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Break_ could not be converted to string"))) } }
impl php_rt::PhpClone for Break_ { fn php_clone(&self) -> Self { let c = Break_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Break_Obj { fn clone(&self) -> Self { Break_Obj { attributes: self.attributes.clone(), num: self.num.clone() } } }
impl Break_ {
}
pub struct Case_Obj {
    pub attributes: Map<Str, Mixed>,
    pub cond: Option<crate::php_parser::node::Expr>,
    pub stmts: List<crate::php_parser::node::Stmt>,
}
#[derive(Clone)]
pub struct Case_(pub Rc<RefCell<Case_Obj>>);
impl Case_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_cond(&self) -> Ref<'_, Option<crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.cond) }
    pub fn p_cond_get(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().cond.clone() }
    pub fn p_cond_opt(&self) -> Option<Option<crate::php_parser::node::Expr>> { Some(self.0.borrow().cond.clone()) }
    pub fn p_cond_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.cond) }
    pub fn set_p_cond(&self, v: Option<crate::php_parser::node::Expr>) { self.0.borrow_mut().cond = v; }
    pub fn p_stmts(&self) -> Ref<'_, List<crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> List<crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<List<crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: List<crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn new(mut cond: Option<crate::php_parser::node::Expr>, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Case_, Throw> {
        let this = Case_(Rc::new(RefCell::new(Case_Obj {
            attributes: Default::default(),
            cond: Default::default(),
            stmts: Default::default(),
        })));
        this.magic__construct(cond, stmts, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut cond: Option<crate::php_parser::node::Expr>, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_cond(cond.clone());
    self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(stmts.clone()));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("cond"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Case"));
    }
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
    pub fn new_same_class(&self, mut cond: Option<crate::php_parser::node::Expr>, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Case_, Throw> { Ok(Self::new(cond, stmts, attributes)?) }
}
impl php_rt::PhpObject for Case_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Case_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\case_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_cond_get()) { out.push((Str::from_static("cond"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "cond" => { self.set_p_cond(value.to_option().map(|__m| cast::<crate::php_parser::node::Expr>(__m))); true }, "stmts" => { self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(value)); true }, _ => false } }
}
impl Case_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Case_ could not be converted to string"))) } }
impl php_rt::PhpClone for Case_ { fn php_clone(&self) -> Self { let c = Case_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Case_Obj { fn clone(&self) -> Self { Case_Obj { attributes: self.attributes.clone(), cond: self.cond.clone(), stmts: self.stmts.clone() } } }
impl Case_ {
}
pub struct Catch_Obj {
    pub attributes: Map<Str, Mixed>,
    pub types: Map<ArrayKey, crate::php_parser::node::Name>,
    pub var: Option<crate::php_parser::node::expr::Variable>,
    pub stmts: List<crate::php_parser::node::Stmt>,
}
#[derive(Clone)]
pub struct Catch_(pub Rc<RefCell<Catch_Obj>>);
impl Catch_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_types(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.types) }
    pub fn p_types_get(&self) -> Map<ArrayKey, crate::php_parser::node::Name> { self.0.borrow().types.clone() }
    pub fn p_types_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Name>> { Some(self.0.borrow().types.clone()) }
    pub fn p_types_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.types) }
    pub fn set_p_types(&self, v: Map<ArrayKey, crate::php_parser::node::Name>) { self.0.borrow_mut().types = v; }
    pub fn p_var(&self) -> Ref<'_, Option<crate::php_parser::node::expr::Variable>> { Ref::map(self.0.borrow(), |o| &o.var) }
    pub fn p_var_get(&self) -> Option<crate::php_parser::node::expr::Variable> { self.0.borrow().var.clone() }
    pub fn p_var_opt(&self) -> Option<Option<crate::php_parser::node::expr::Variable>> { Some(self.0.borrow().var.clone()) }
    pub fn p_var_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::expr::Variable>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.var) }
    pub fn set_p_var(&self, v: Option<crate::php_parser::node::expr::Variable>) { self.0.borrow_mut().var = v; }
    pub fn p_stmts(&self) -> Ref<'_, List<crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> List<crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<List<crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: List<crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn new(mut types: Map<ArrayKey, crate::php_parser::node::Name>, mut var: Option<crate::php_parser::node::expr::Variable>, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Catch_, Throw> {
        let this = Catch_(Rc::new(RefCell::new(Catch_Obj {
            attributes: Default::default(),
            types: Default::default(),
            var: Default::default(),
            stmts: Default::default(),
        })));
        this.magic__construct(types, var, stmts, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut types: Map<ArrayKey, crate::php_parser::node::Name>, mut var: Option<crate::php_parser::node::expr::Variable>, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_types(types.clone());
    self.set_p_var(var.clone());
    self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(stmts.clone()));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("types"))); __m1.push(cast::<Mixed>(Str::from_static("var"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Catch"));
    }
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
    pub fn new_same_class(&self, mut types: Map<ArrayKey, crate::php_parser::node::Name>, mut var: Option<crate::php_parser::node::expr::Variable>, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Catch_, Throw> { Ok(Self::new(types, var, stmts, attributes)?) }
}
impl php_rt::PhpObject for Catch_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Catch_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\catch_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_types_get()) { out.push((Str::from_static("types"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_var_get()) { out.push((Str::from_static("var"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "types" => { self.set_p_types(cast::<Map<ArrayKey, crate::php_parser::node::Name>>(value)); true }, "var" => { self.set_p_var(value.to_option().map(|__m| cast::<crate::php_parser::node::expr::Variable>(__m))); true }, "stmts" => { self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(value)); true }, _ => false } }
}
impl Catch_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Catch_ could not be converted to string"))) } }
impl php_rt::PhpClone for Catch_ { fn php_clone(&self) -> Self { let c = Catch_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Catch_Obj { fn clone(&self) -> Self { Catch_Obj { attributes: self.attributes.clone(), types: self.types.clone(), var: self.var.clone(), stmts: self.stmts.clone() } } }
impl Catch_ {
}
pub struct ClassConstObj {
    pub attributes: Map<Str, Mixed>,
    pub flags: i64,
    pub consts: Map<ArrayKey, crate::php_parser::node::Const_>,
    pub attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>,
    pub type_: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>,
}
#[derive(Clone)]
pub struct ClassConst(pub Rc<RefCell<ClassConstObj>>);
impl ClassConst {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_flags(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.flags) }
    pub fn p_flags_get(&self) -> i64 { self.0.borrow().flags.clone() }
    pub fn p_flags_opt(&self) -> Option<i64> { Some(self.0.borrow().flags.clone()) }
    pub fn p_flags_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.flags) }
    pub fn set_p_flags(&self, v: i64) { self.0.borrow_mut().flags = v; }
    pub fn p_consts(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Const_>> { Ref::map(self.0.borrow(), |o| &o.consts) }
    pub fn p_consts_get(&self) -> Map<ArrayKey, crate::php_parser::node::Const_> { self.0.borrow().consts.clone() }
    pub fn p_consts_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Const_>> { Some(self.0.borrow().consts.clone()) }
    pub fn p_consts_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Const_>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.consts) }
    pub fn set_p_consts(&self, v: Map<ArrayKey, crate::php_parser::node::Const_>) { self.0.borrow_mut().consts = v; }
    pub fn p_attrGroups(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attrGroups) }
    pub fn p_attrGroups_get(&self) -> Map<ArrayKey, crate::php_parser::node::AttributeGroup> { self.0.borrow().attrGroups.clone() }
    pub fn p_attrGroups_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attrGroups.clone()) }
    pub fn p_attrGroups_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attrGroups) }
    pub fn set_p_attrGroups(&self, v: Map<ArrayKey, crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attrGroups = v; }
    pub fn p_type_(&self) -> Ref<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { Ref::map(self.0.borrow(), |o| &o.type_) }
    pub fn p_type__get(&self) -> Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> { self.0.borrow().type_.clone() }
    pub fn p_type__opt(&self) -> Option<Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { Some(self.0.borrow().type_.clone()) }
    pub fn p_type__mut(&self) -> RefMut<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.type_) }
    pub fn set_p_type_(&self, v: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>) { self.0.borrow_mut().type_ = v; }
    pub fn new(mut consts_v: Map<ArrayKey, crate::php_parser::node::Const_>, mut flags: i64, mut attributes: Map<Str, Mixed>, mut attrGroups: List<crate::php_parser::node::AttributeGroup>, mut type_: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>) -> Result<ClassConst, Throw> {
        let this = ClassConst(Rc::new(RefCell::new(ClassConstObj {
            attributes: Default::default(),
            flags: Default::default(),
            consts: Default::default(),
            attrGroups: Default::default(),
            type_: Default::default(),
        })));
        this.magic__construct(consts_v, flags, attributes, attrGroups, type_)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut consts_v: Map<ArrayKey, crate::php_parser::node::Const_>, mut flags: i64, mut attributes: Map<Str, Mixed>, mut attrGroups: List<crate::php_parser::node::AttributeGroup>, mut type_: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_flags(flags);
    self.set_p_consts(consts_v.clone());
    self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(attrGroups.clone()));
    self.set_p_type_(type_.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("attrGroups"))); __m1.push(cast::<Mixed>(Str::from_static("flags"))); __m1.push(cast::<Mixed>(Str::from_static("type"))); __m1.push(cast::<Mixed>(Str::from_static("consts"))); __m1 });
    }
    pub fn isPublic(&self) -> Result<bool, Throw> {
    return Ok(((!((self.p_flags_get() & crate::php_parser::Modifiers::PUBLIC()) == 0i64)) || ((self.p_flags_get() & crate::php_parser::Modifiers::VISIBILITY_MASK()) == 0i64)));
    }
    pub fn isProtected(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::PROTECTED())));
    }
    pub fn isPrivate(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::PRIVATE())));
    }
    pub fn isFinal(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::FINAL())));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_ClassConst"));
    }
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
    pub fn new_same_class(&self, mut consts_v: Map<ArrayKey, crate::php_parser::node::Const_>, mut flags: i64, mut attributes: Map<Str, Mixed>, mut attrGroups: List<crate::php_parser::node::AttributeGroup>, mut type_: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>) -> Result<crate::php_parser::node::stmt::ClassConst, Throw> { Ok(Self::new(consts_v, flags, attributes, attrGroups, type_)?) }
}
impl php_rt::PhpObject for ClassConst {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\ClassConst" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\classconst", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_flags_get()) { out.push((Str::from_static("flags"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_consts_get()) { out.push((Str::from_static("consts"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attrGroups_get()) { out.push((Str::from_static("attrGroups"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_type__get()) { out.push((Str::from_static("type"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "flags" => { self.set_p_flags(cast::<i64>(value)); true }, "consts" => { self.set_p_consts(cast::<Map<ArrayKey, crate::php_parser::node::Const_>>(value)); true }, "attrGroups" => { self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(value)); true }, "type" => { self.set_p_type_(value.to_option().map(|__m| cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(__m))); true }, _ => false } }
}
impl ClassConst { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\ClassConst could not be converted to string"))) } }
impl php_rt::PhpClone for ClassConst { fn php_clone(&self) -> Self { let c = ClassConst(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ClassConstObj { fn clone(&self) -> Self { ClassConstObj { attributes: self.attributes.clone(), flags: self.flags.clone(), consts: self.consts.clone(), attrGroups: self.attrGroups.clone(), type_: self.type_.clone() } } }
impl ClassConst {
}
#[derive(Clone)]
pub enum ClassLike {
    PhpParser_Node_Stmt_Class_(crate::php_parser::node::stmt::Class_),
    PhpParser_Node_Stmt_Enum_(crate::php_parser::node::stmt::Enum_),
    PhpParser_Node_Stmt_Interface_(crate::php_parser::node::stmt::Interface_),
    PhpParser_Node_Stmt_Trait_(crate::php_parser::node::stmt::Trait_),
}
impl php_rt::PhpObject for ClassLike {
    fn class_name(&self) -> &'static str { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.class_name(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.class_name(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.class_name(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.class_name(), _ => unreachable!() } }
    fn class_ancestors(&self) -> &'static [&'static str] { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.class_ancestors(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.class_ancestors(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.class_ancestors(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.class_ancestors(), _ => unreachable!() } }
    fn obj_id(&self) -> usize { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.obj_id(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.obj_id(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.obj_id(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.obj_id(), _ => unreachable!() } }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.props(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.props(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.props(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.props(), _ => unreachable!() } }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.set_prop(name, value), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.set_prop(name, value), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.set_prop(name, value), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.set_prop(name, value), _ => unreachable!() } }
    fn php_to_string(&self) -> Option<Str> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.php_to_string(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.php_to_string(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.php_to_string(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.php_to_string(), _ => unreachable!() } }
}
impl ClassLike { pub fn to_php_string(&self) -> Result<Str, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.to_php_string(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.to_php_string(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.to_php_string(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.to_php_string(), _ => unreachable!() } } }
impl php_rt::PhpClone for ClassLike { fn php_clone(&self) -> Self { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => ClassLike::PhpParser_Node_Stmt_Class_(__h.php_clone()), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => ClassLike::PhpParser_Node_Stmt_Enum_(__h.php_clone()), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => ClassLike::PhpParser_Node_Stmt_Interface_(__h.php_clone()), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => ClassLike::PhpParser_Node_Stmt_Trait_(__h.php_clone()), _ => unreachable!() } } }
impl ClassLike {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_attributes(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_attributes(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_attributes(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_attributes(), _ => unreachable!() } }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_attributes_get(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_attributes_get(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_attributes_get(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_attributes_get(), _ => unreachable!() } }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_attributes_opt(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_attributes_opt(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_attributes_opt(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_attributes_opt(), _ => unreachable!() } }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_attributes_mut(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_attributes_mut(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_attributes_mut(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_attributes_mut(), _ => unreachable!() } }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.set_p_attributes(v), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.set_p_attributes(v), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.set_p_attributes(v), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.set_p_attributes(v), _ => unreachable!() } }
    pub fn p_name(&self) -> Ref<'_, Option<crate::php_parser::node::Identifier>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_name(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_name(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_name(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_name(), _ => unreachable!() } }
    pub fn p_name_get(&self) -> Option<crate::php_parser::node::Identifier> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_name_get(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_name_get(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_name_get(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_name_get(), _ => unreachable!() } }
    pub fn p_name_opt(&self) -> Option<Option<crate::php_parser::node::Identifier>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_name_opt(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_name_opt(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_name_opt(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_name_opt(), _ => unreachable!() } }
    pub fn p_name_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Identifier>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_name_mut(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_name_mut(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_name_mut(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_name_mut(), _ => unreachable!() } }
    pub fn set_p_name(&self, v: Option<crate::php_parser::node::Identifier>) { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.set_p_name(v), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.set_p_name(v), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.set_p_name(v), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.set_p_name(v), _ => unreachable!() } }
    pub fn p_stmts(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_stmts(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_stmts(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_stmts(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_stmts(), _ => unreachable!() } }
    pub fn p_stmts_get(&self) -> Map<ArrayKey, crate::php_parser::node::Stmt> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_stmts_get(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_stmts_get(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_stmts_get(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_stmts_get(), _ => unreachable!() } }
    pub fn p_stmts_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Stmt>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_stmts_opt(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_stmts_opt(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_stmts_opt(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_stmts_opt(), _ => unreachable!() } }
    pub fn p_stmts_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_stmts_mut(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_stmts_mut(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_stmts_mut(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_stmts_mut(), _ => unreachable!() } }
    pub fn set_p_stmts(&self, v: Map<ArrayKey, crate::php_parser::node::Stmt>) { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.set_p_stmts(v), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.set_p_stmts(v), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.set_p_stmts(v), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.set_p_stmts(v), _ => unreachable!() } }
    pub fn p_attrGroups(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_attrGroups(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_attrGroups(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_attrGroups(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_attrGroups(), _ => unreachable!() } }
    pub fn p_attrGroups_get(&self) -> Map<ArrayKey, crate::php_parser::node::AttributeGroup> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_attrGroups_get(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_attrGroups_get(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_attrGroups_get(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_attrGroups_get(), _ => unreachable!() } }
    pub fn p_attrGroups_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_attrGroups_opt(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_attrGroups_opt(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_attrGroups_opt(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_attrGroups_opt(), _ => unreachable!() } }
    pub fn p_attrGroups_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_attrGroups_mut(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_attrGroups_mut(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_attrGroups_mut(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_attrGroups_mut(), _ => unreachable!() } }
    pub fn set_p_attrGroups(&self, v: Map<ArrayKey, crate::php_parser::node::AttributeGroup>) { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.set_p_attrGroups(v), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.set_p_attrGroups(v), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.set_p_attrGroups(v), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.set_p_attrGroups(v), _ => unreachable!() } }
    pub fn p_namespacedName(&self) -> Ref<'_, Option<crate::php_parser::node::Name>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_namespacedName(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_namespacedName(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_namespacedName(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_namespacedName(), _ => unreachable!() } }
    pub fn p_namespacedName_get(&self) -> Option<crate::php_parser::node::Name> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_namespacedName_get(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_namespacedName_get(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_namespacedName_get(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_namespacedName_get(), _ => unreachable!() } }
    pub fn p_namespacedName_opt(&self) -> Option<Option<crate::php_parser::node::Name>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_namespacedName_opt(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_namespacedName_opt(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_namespacedName_opt(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_namespacedName_opt(), _ => unreachable!() } }
    pub fn p_namespacedName_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Name>> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.p_namespacedName_mut(), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.p_namespacedName_mut(), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.p_namespacedName_mut(), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.p_namespacedName_mut(), _ => unreachable!() } }
    pub fn set_p_namespacedName(&self, v: Option<crate::php_parser::node::Name>) { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => __h.set_p_namespacedName(v), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => __h.set_p_namespacedName(v), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => __h.set_p_namespacedName(v), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => __h.set_p_namespacedName(v), _ => unreachable!() } }
    pub fn getTraitUses(&self) -> Result<List<crate::php_parser::node::stmt::TraitUse>, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getTraitUses()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getTraitUses()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getTraitUses()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getTraitUses()?), _ => unreachable!() } }
    pub fn getTraitUses__impl(&self) -> Result<List<crate::php_parser::node::stmt::TraitUse>, Throw> {
    let mut traitUses: List<crate::php_parser::node::stmt::TraitUse> = Default::default();
    let mut stmt: Late<crate::php_parser::node::Stmt> = Late::uninit();
    traitUses = List::<crate::php_parser::node::stmt::TraitUse>::new();
    'l1: for __kv1 in self.p_stmts_get().into_iter() {
        stmt.set(__kv1.1);
        if is_instance::<crate::php_parser::node::stmt::TraitUse>(&stmt.get().clone()) {
            traitUses.push(cast::<crate::php_parser::node::stmt::TraitUse>(stmt.get().clone()));
        }
    }
    return Ok(traitUses.clone());
    }
    pub fn getConstants(&self) -> Result<List<crate::php_parser::node::stmt::ClassConst>, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getConstants()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getConstants()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getConstants()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getConstants()?), _ => unreachable!() } }
    pub fn getConstants__impl(&self) -> Result<List<crate::php_parser::node::stmt::ClassConst>, Throw> {
    let mut constants: List<crate::php_parser::node::stmt::ClassConst> = Default::default();
    let mut stmt: Late<crate::php_parser::node::Stmt> = Late::uninit();
    constants = List::<crate::php_parser::node::stmt::ClassConst>::new();
    'l1: for __kv1 in self.p_stmts_get().into_iter() {
        stmt.set(__kv1.1);
        if is_instance::<crate::php_parser::node::stmt::ClassConst>(&stmt.get().clone()) {
            constants.push(cast::<crate::php_parser::node::stmt::ClassConst>(stmt.get().clone()));
        }
    }
    return Ok(constants.clone());
    }
    pub fn getProperties(&self) -> Result<List<crate::php_parser::node::stmt::Property>, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getProperties()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getProperties()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getProperties()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getProperties()?), _ => unreachable!() } }
    pub fn getProperties__impl(&self) -> Result<List<crate::php_parser::node::stmt::Property>, Throw> {
    let mut properties: List<crate::php_parser::node::stmt::Property> = Default::default();
    let mut stmt: Late<crate::php_parser::node::Stmt> = Late::uninit();
    properties = List::<crate::php_parser::node::stmt::Property>::new();
    'l1: for __kv1 in self.p_stmts_get().into_iter() {
        stmt.set(__kv1.1);
        if is_instance::<crate::php_parser::node::stmt::Property>(&stmt.get().clone()) {
            properties.push(cast::<crate::php_parser::node::stmt::Property>(stmt.get().clone()));
        }
    }
    return Ok(properties.clone());
    }
    pub fn getProperty(&self, mut name: Str) -> Result<Option<crate::php_parser::node::stmt::Property>, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getProperty(name)?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getProperty(name)?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getProperty(name)?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getProperty(name)?), _ => unreachable!() } }
    pub fn getProperty__impl(&self, mut name: Str) -> Result<Option<crate::php_parser::node::stmt::Property>, Throw> {
    let mut stmt: Late<crate::php_parser::node::Stmt> = Late::uninit();
    let mut prop: Late<crate::php_parser::node::PropertyItem> = Late::uninit();
    'l1: for __kv1 in self.p_stmts_get().into_iter() {
        stmt.set(__kv1.1);
        if is_instance::<crate::php_parser::node::stmt::Property>(&stmt.get().clone()) {
            'l2: for __kv2 in cast::<crate::php_parser::node::stmt::Property>(stmt.get().clone()).p_props_get().into_iter() {
                prop.set(__kv2.1);
                if ({ let _ = prop.get().clone(); true } && identical(&name.clone(), &prop.get().clone().p_name_get().toString()?)) {
                    return Ok(Some(cast::<crate::php_parser::node::stmt::Property>(stmt.get().clone())));
                }
            }
        }
    }
    return Ok({ let _ = (); None::<crate::php_parser::node::stmt::Property> });
    }
    pub fn getMethods(&self) -> Result<List<crate::php_parser::node::stmt::ClassMethod>, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getMethods()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getMethods()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getMethods()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getMethods()?), _ => unreachable!() } }
    pub fn getMethods__impl(&self) -> Result<List<crate::php_parser::node::stmt::ClassMethod>, Throw> {
    let mut methods: List<crate::php_parser::node::stmt::ClassMethod> = Default::default();
    let mut stmt: Late<crate::php_parser::node::Stmt> = Late::uninit();
    methods = List::<crate::php_parser::node::stmt::ClassMethod>::new();
    'l1: for __kv1 in self.p_stmts_get().into_iter() {
        stmt.set(__kv1.1);
        if is_instance::<crate::php_parser::node::stmt::ClassMethod>(&stmt.get().clone()) {
            methods.push(cast::<crate::php_parser::node::stmt::ClassMethod>(stmt.get().clone()));
        }
    }
    return Ok(methods.clone());
    }
    pub fn getMethod(&self, mut name: Str) -> Result<Option<crate::php_parser::node::stmt::ClassMethod>, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getMethod(name)?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getMethod(name)?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getMethod(name)?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getMethod(name)?), _ => unreachable!() } }
    pub fn getMethod__impl(&self, mut name: Str) -> Result<Option<crate::php_parser::node::stmt::ClassMethod>, Throw> {
    let mut lowerName: Str = Default::default();
    let mut stmt: Late<crate::php_parser::node::Stmt> = Late::uninit();
    lowerName = strtolower(&name.clone());
    'l1: for __kv1 in self.p_stmts_get().into_iter() {
        stmt.set(__kv1.1);
        if (is_instance::<crate::php_parser::node::stmt::ClassMethod>(&stmt.get().clone()) && identical(&lowerName.clone(), &cast::<crate::php_parser::node::stmt::ClassMethod>(stmt.get().clone()).p_name_get().toLowerString()?)) {
            return Ok(Some(cast::<crate::php_parser::node::stmt::ClassMethod>(stmt.get().clone())));
        }
    }
    return Ok({ let _ = (); None::<crate::php_parser::node::stmt::ClassMethod> });
    }
    pub fn magic__construct(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.magic__construct(Some(cast::<U_PhpParser_Node_Identifier_or_Str>(attributes)), <Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__467d114822>::default(), <Map<Str, Mixed>>::default())?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.magic__construct(Some(cast::<U_PhpParser_Node_Identifier_or_Str>(attributes)), <Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_implement_38f3035d6c>::default(), <Map<Str, Mixed>>::default())?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.magic__construct(cast::<U_PhpParser_Node_Identifier_or_Str>(attributes), <Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__786d9d6046>::default(), <Map<Str, Mixed>>::default())?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.magic__construct(cast::<U_PhpParser_Node_Identifier_or_Str>(attributes), <Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_stmtsq_Ma_b3c75089bc>::default(), <Map<Str, Mixed>>::default())?), _ => unreachable!() } }
    pub fn magic__construct__impl(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).magic__construct__impl(attributes) }
    pub fn getLine(&self) -> Result<i64, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getLine()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getLine()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getLine()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getLine()?), _ => unreachable!() } }
    pub fn getLine__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getLine__impl() }
    pub fn getStartLine(&self) -> Result<i64, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getStartLine()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getStartLine()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getStartLine()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getStartLine()?), _ => unreachable!() } }
    pub fn getStartLine__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartLine__impl() }
    pub fn getEndLine(&self) -> Result<i64, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getEndLine()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getEndLine()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getEndLine()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getEndLine()?), _ => unreachable!() } }
    pub fn getEndLine__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndLine__impl() }
    pub fn getStartTokenPos(&self) -> Result<i64, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getStartTokenPos()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getStartTokenPos()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getStartTokenPos()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getStartTokenPos()?), _ => unreachable!() } }
    pub fn getStartTokenPos__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartTokenPos__impl() }
    pub fn getEndTokenPos(&self) -> Result<i64, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getEndTokenPos()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getEndTokenPos()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getEndTokenPos()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getEndTokenPos()?), _ => unreachable!() } }
    pub fn getEndTokenPos__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndTokenPos__impl() }
    pub fn getStartFilePos(&self) -> Result<i64, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getStartFilePos()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getStartFilePos()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getStartFilePos()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getStartFilePos()?), _ => unreachable!() } }
    pub fn getStartFilePos__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartFilePos__impl() }
    pub fn getEndFilePos(&self) -> Result<i64, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getEndFilePos()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getEndFilePos()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getEndFilePos()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getEndFilePos()?), _ => unreachable!() } }
    pub fn getEndFilePos__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndFilePos__impl() }
    pub fn getComments(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getComments()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getComments()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getComments()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getComments()?), _ => unreachable!() } }
    pub fn getComments__impl(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getComments__impl() }
    pub fn getDocComment(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getDocComment()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getDocComment()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getDocComment()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getDocComment()?), _ => unreachable!() } }
    pub fn getDocComment__impl(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getDocComment__impl() }
    pub fn setDocComment(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.setDocComment(docComment)?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.setDocComment(docComment)?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.setDocComment(docComment)?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.setDocComment(docComment)?), _ => unreachable!() } }
    pub fn setDocComment__impl(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setDocComment__impl(docComment) }
    pub fn setAttribute(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.setAttribute(key_v, value)?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.setAttribute(key_v, value)?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.setAttribute(key_v, value)?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.setAttribute(key_v, value)?), _ => unreachable!() } }
    pub fn setAttribute__impl(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttribute__impl(key_v, value) }
    pub fn hasAttribute(&self, mut key_v: Str) -> Result<bool, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.hasAttribute(key_v)?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.hasAttribute(key_v)?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.hasAttribute(key_v)?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.hasAttribute(key_v)?), _ => unreachable!() } }
    pub fn hasAttribute__impl(&self, mut key_v: Str) -> Result<bool, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).hasAttribute__impl(key_v) }
    pub fn getAttribute(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getAttribute(key_v, default)?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getAttribute(key_v, default)?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getAttribute(key_v, default)?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getAttribute(key_v, default)?), _ => unreachable!() } }
    pub fn getAttribute__impl(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttribute__impl(key_v, default) }
    pub fn getAttributes(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getAttributes()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getAttributes()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getAttributes()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getAttributes()?), _ => unreachable!() } }
    pub fn getAttributes__impl(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttributes__impl() }
    pub fn setAttributes(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.setAttributes(attributes)?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.setAttributes(attributes)?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.setAttributes(attributes)?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.setAttributes(attributes)?), _ => unreachable!() } }
    pub fn setAttributes__impl(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttributes__impl(attributes) }
    pub fn jsonSerialize(&self) -> Result<Map<Str, Mixed>, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.jsonSerialize()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.jsonSerialize()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.jsonSerialize()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.jsonSerialize()?), _ => unreachable!() } }
    pub fn jsonSerialize__impl(&self) -> Result<Map<Str, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).jsonSerialize__impl() }
    pub fn getType(&self) -> Result<Str, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getType()?), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getType()?), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getType()?), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getType()?), _ => unreachable!() } }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Str>, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(__h.getSubNodeNames()?.map_values(|v| cast::<Str>(v))), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(__h.getSubNodeNames()?.map_values(|v| cast::<Str>(v))), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(__h.getSubNodeNames()?.map_values(|v| cast::<Str>(v))), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(__h.getSubNodeNames()?.map_values(|v| cast::<Str>(v))), _ => unreachable!() } }
    pub fn new_same_class(&self, mut attributes: Map<Str, Mixed>) -> Result<ClassLike, Throw> { match self { ClassLike::PhpParser_Node_Stmt_Class_(__h) => Ok(cast::<crate::php_parser::node::stmt::ClassLike>(__h.new_same_class(Some(cast::<U_PhpParser_Node_Identifier_or_Str>(attributes.clone())), <Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__467d114822>::default(), <Map<Str, Mixed>>::default())?)), ClassLike::PhpParser_Node_Stmt_Enum_(__h) => Ok(cast::<crate::php_parser::node::stmt::ClassLike>(__h.new_same_class(Some(cast::<U_PhpParser_Node_Identifier_or_Str>(attributes.clone())), <Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_implement_38f3035d6c>::default(), <Map<Str, Mixed>>::default())?)), ClassLike::PhpParser_Node_Stmt_Interface_(__h) => Ok(cast::<crate::php_parser::node::stmt::ClassLike>(__h.new_same_class(cast::<U_PhpParser_Node_Identifier_or_Str>(attributes.clone()), <Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__786d9d6046>::default(), <Map<Str, Mixed>>::default())?)), ClassLike::PhpParser_Node_Stmt_Trait_(__h) => Ok(cast::<crate::php_parser::node::stmt::ClassLike>(__h.new_same_class(cast::<U_PhpParser_Node_Identifier_or_Str>(attributes.clone()), <Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_stmtsq_Ma_b3c75089bc>::default(), <Map<Str, Mixed>>::default())?)), _ => unreachable!() } }
}
pub struct ClassMethodObj {
    pub attributes: Map<Str, Mixed>,
    pub flags: i64,
    pub byRef: bool,
    pub name: Late<crate::php_parser::node::Identifier>,
    pub params: Map<ArrayKey, crate::php_parser::node::Param>,
    pub returnType: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>,
    pub stmts: Option<Map<ArrayKey, crate::php_parser::node::Stmt>>,
    pub attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>,
}
#[derive(Clone)]
pub struct ClassMethod(pub Rc<RefCell<ClassMethodObj>>);
impl ClassMethod {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_flags(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.flags) }
    pub fn p_flags_get(&self) -> i64 { self.0.borrow().flags.clone() }
    pub fn p_flags_opt(&self) -> Option<i64> { Some(self.0.borrow().flags.clone()) }
    pub fn p_flags_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.flags) }
    pub fn set_p_flags(&self, v: i64) { self.0.borrow_mut().flags = v; }
    pub fn p_byRef(&self) -> Ref<'_, bool> { Ref::map(self.0.borrow(), |o| &o.byRef) }
    pub fn p_byRef_get(&self) -> bool { self.0.borrow().byRef.clone() }
    pub fn p_byRef_opt(&self) -> Option<bool> { Some(self.0.borrow().byRef.clone()) }
    pub fn p_byRef_mut(&self) -> RefMut<'_, bool> { RefMut::map(self.0.borrow_mut(), |o| &mut o.byRef) }
    pub fn set_p_byRef(&self, v: bool) { self.0.borrow_mut().byRef = v; }
    pub fn p_name(&self) -> Ref<'_, crate::php_parser::node::Identifier> { Ref::map(self.0.borrow(), |o| o.name.get()) }
    pub fn p_name_get(&self) -> crate::php_parser::node::Identifier { self.0.borrow().name.get().clone() }
    pub fn p_name_opt(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().name.as_option().cloned() }
    pub fn p_name_mut(&self) -> RefMut<'_, crate::php_parser::node::Identifier> { RefMut::map(self.0.borrow_mut(), |o| o.name.get_mut()) }
    pub fn set_p_name(&self, v: crate::php_parser::node::Identifier) { self.0.borrow_mut().name.set(v); }
    pub fn p_params(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Param>> { Ref::map(self.0.borrow(), |o| &o.params) }
    pub fn p_params_get(&self) -> Map<ArrayKey, crate::php_parser::node::Param> { self.0.borrow().params.clone() }
    pub fn p_params_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Param>> { Some(self.0.borrow().params.clone()) }
    pub fn p_params_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Param>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.params) }
    pub fn set_p_params(&self, v: Map<ArrayKey, crate::php_parser::node::Param>) { self.0.borrow_mut().params = v; }
    pub fn p_returnType(&self) -> Ref<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { Ref::map(self.0.borrow(), |o| &o.returnType) }
    pub fn p_returnType_get(&self) -> Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> { self.0.borrow().returnType.clone() }
    pub fn p_returnType_opt(&self) -> Option<Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { Some(self.0.borrow().returnType.clone()) }
    pub fn p_returnType_mut(&self) -> RefMut<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.returnType) }
    pub fn set_p_returnType(&self, v: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>) { self.0.borrow_mut().returnType = v; }
    pub fn p_stmts(&self) -> Ref<'_, Option<Map<ArrayKey, crate::php_parser::node::Stmt>>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Stmt>> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<Option<Map<ArrayKey, crate::php_parser::node::Stmt>>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, Option<Map<ArrayKey, crate::php_parser::node::Stmt>>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: Option<Map<ArrayKey, crate::php_parser::node::Stmt>>) { self.0.borrow_mut().stmts = v; }
    pub fn p_attrGroups(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attrGroups) }
    pub fn p_attrGroups_get(&self) -> Map<ArrayKey, crate::php_parser::node::AttributeGroup> { self.0.borrow().attrGroups.clone() }
    pub fn p_attrGroups_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attrGroups.clone()) }
    pub fn p_attrGroups_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attrGroups) }
    pub fn set_p_attrGroups(&self, v: Map<ArrayKey, crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attrGroups = v; }
    pub fn new(mut name: U_PhpParser_Node_Identifier_or_Str, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_6534c97760, mut attributes: Map<Str, Mixed>) -> Result<ClassMethod, Throw> {
        let this = ClassMethod(Rc::new(RefCell::new(ClassMethodObj {
            attributes: Default::default(),
            flags: Default::default(),
            byRef: Default::default(),
            name: Late::uninit(),
            params: Default::default(),
            returnType: Default::default(),
            stmts: Default::default(),
            attrGroups: Default::default(),
        })));
        this.magic__construct(name, subNodes, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_6534c97760, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_flags((match Some({ let __c58 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_b8c64b7d57 { attrGroups: __c58.attrGroups, byRef: __c58.byRef, flags: __c58.flags.unwrap(), params: __c58.params, returnType: __c58.returnType, stmts: __c58.stmts } }).and_then(|__b| Some(__b.flags)) { Some(__v) => __v, None => cast::<i64>((match Some({ let __c59 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_ac1de80901 { attrGroups: __c59.attrGroups, byRef: __c59.byRef, flags: __c59.flags, params: __c59.params, returnType: __c59.returnType, stmts: __c59.stmts, type_: <Mixed>::default() } }).and_then(|__b| Some(__b.type_)).and_then(|__m| __m.to_option()) { Some(__v) => __v, None => cast::<Mixed>(0i64) })) }));
    self.set_p_byRef((match Some({ let __c60 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRef_Boo_427b4aa951 { attrGroups: __c60.attrGroups, byRef: __c60.byRef.unwrap(), flags: __c60.flags, params: __c60.params, returnType: __c60.returnType, stmts: __c60.stmts, type_: None } }).and_then(|__b| Some(__b.byRef)) { Some(__v) => __v, None => false }));
    self.set_p_name((if (match name.clone() { U_PhpParser_Node_Identifier_or_Str::Str(_) => true, _ => false }) { crate::php_parser::node::Identifier::new(cast::<Str>(name.clone()), Map::<Str, Mixed>::new())? } else { cast::<crate::php_parser::node::Identifier>(name.clone()) }));
    self.set_p_params((match Some({ let __c61 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_2490efd48c { attrGroups: __c61.attrGroups, byRef: __c61.byRef, flags: __c61.flags, params: __c61.params.unwrap(), returnType: __c61.returnType, stmts: __c61.stmts, type_: None } }).and_then(|__b| Some(__b.params)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Param>::new() }));
    self.set_p_returnType((match Some({ let __c62 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_3f862ca630 { attrGroups: __c62.attrGroups, byRef: __c62.byRef, flags: __c62.flags, params: __c62.params, returnType: __c62.returnType.unwrap(), stmts: __c62.stmts, type_: None } }).and_then(|__b| Some(__b.returnType)) { Some(__v) => Some(__v), None => { let _ = (); None::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> } }));
    self.set_p_stmts((if { let __c64 = { let __c63 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_ee6bd02860 { attrGroups: __c63.attrGroups, byRef: __c63.byRef, flags: __c63.flags, params: __c63.params, returnType: __c63.returnType, stmts: __c63.stmts, type_: None } }; let mut __m: Map<Str, Mixed> = Default::default(); if let Some(v) = __c64.attrGroups { __m.insert(Str::from_static("attrGroups"), cast::<Mixed>(v)); } if let Some(v) = __c64.byRef { __m.insert(Str::from_static("byRef"), cast::<Mixed>(v)); } if let Some(v) = __c64.flags { __m.insert(Str::from_static("flags"), cast::<Mixed>(v)); } if let Some(v) = __c64.params { __m.insert(Str::from_static("params"), cast::<Mixed>(v)); } if let Some(v) = __c64.returnType { __m.insert(Str::from_static("returnType"), cast::<Mixed>(v)); } if let Some(v) = __c64.stmts { __m.insert(Str::from_static("stmts"), cast::<Mixed>(v)); } if let Some(v) = __c64.type_ { __m.insert(Str::from_static("type"), v); } __m }.contains_key(&Str::from_static("stmts")) { { let __c65 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_418b4417be { attrGroups: __c65.attrGroups, byRef: __c65.byRef, flags: __c65.flags, params: __c65.params, returnType: __c65.returnType, stmts: __c65.stmts, type_: None } }.stmts } else { Some(Map::<ArrayKey, Mixed>::new().map_values(|v| cast::<crate::php_parser::node::Stmt>(v))) }));
    self.set_p_attrGroups((match Some({ let __c66 = subNodes.clone(); Shape_attrGroups_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Boo_5ddc41098d { attrGroups: __c66.attrGroups.unwrap(), byRef: __c66.byRef, flags: __c66.flags, params: __c66.params, returnType: __c66.returnType, stmts: __c66.stmts, type_: None } }).and_then(|__b| Some(__b.attrGroups)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::AttributeGroup>::new() }));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("attrGroups"))); __m1.push(cast::<Mixed>(Str::from_static("flags"))); __m1.push(cast::<Mixed>(Str::from_static("byRef"))); __m1.push(cast::<Mixed>(Str::from_static("name"))); __m1.push(cast::<Mixed>(Str::from_static("params"))); __m1.push(cast::<Mixed>(Str::from_static("returnType"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn returnsByRef(&self) -> Result<bool, Throw> {
    return Ok(self.p_byRef_get());
    }
    pub fn getParams(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok(self.p_params_get().map_values(|v| cast::<Mixed>(v)));
    }
    pub fn getReturnType(&self) -> Result<Mixed, Throw> {
    return Ok(cast::<Mixed>(self.p_returnType_get()));
    }
    pub fn getStmts(&self) -> Result<Option<Map<ArrayKey, Mixed>>, Throw> {
    return Ok(self.p_stmts_get().map(|v| v.map_values(|v| cast::<Mixed>(v))));
    }
    pub fn getAttrGroups(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok(self.p_attrGroups_get().map_values(|v| cast::<Mixed>(v)));
    }
    pub fn isPublic(&self) -> Result<bool, Throw> {
    return Ok(((!((self.p_flags_get() & crate::php_parser::Modifiers::PUBLIC()) == 0i64)) || ((self.p_flags_get() & crate::php_parser::Modifiers::VISIBILITY_MASK()) == 0i64)));
    }
    pub fn isProtected(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::PROTECTED())));
    }
    pub fn isPrivate(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::PRIVATE())));
    }
    pub fn isAbstract(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::ABSTRACT())));
    }
    pub fn isFinal(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::FINAL())));
    }
    pub fn isStatic(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::STATIC())));
    }
    pub fn isMagic(&self) -> Result<bool, Throw> {
    return Ok(({ let __k = self.p_name_get().toLowerString()?; Some(crate::php_parser::node::stmt::ClassMethod::st_magicNames()).and_then(|__b| __b.get(&__k).cloned()) }.is_some()));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_ClassMethod"));
    }
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
    pub fn new_same_class(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_6534c97760, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::ClassMethod, Throw> { Ok(Self::new(name, subNodes, attributes)?) }
}
impl php_rt::PhpObject for ClassMethod {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\ClassMethod" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\classmethod", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable", "phpparser\\node\\functionlike"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_flags_get()) { out.push((Str::from_static("flags"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_byRef_get()) { out.push((Str::from_static("byRef"), cast::<Mixed>(v))); } if let Some(v) = self.p_name_opt() { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_params_get()) { out.push((Str::from_static("params"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_returnType_get()) { out.push((Str::from_static("returnType"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attrGroups_get()) { out.push((Str::from_static("attrGroups"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "flags" => { self.set_p_flags(cast::<i64>(value)); true }, "byRef" => { self.set_p_byRef(cast::<bool>(value)); true }, "name" => { self.set_p_name(cast::<crate::php_parser::node::Identifier>(value)); true }, "params" => { self.set_p_params(cast::<Map<ArrayKey, crate::php_parser::node::Param>>(value)); true }, "returnType" => { self.set_p_returnType(value.to_option().map(|__m| cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(__m))); true }, "stmts" => { self.set_p_stmts(value.to_option().map(|__m| cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(__m))); true }, "attrGroups" => { self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(value)); true }, _ => false } }
}
impl ClassMethod { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\ClassMethod could not be converted to string"))) } }
impl php_rt::PhpClone for ClassMethod { fn php_clone(&self) -> Self { let c = ClassMethod(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ClassMethodObj { fn clone(&self) -> Self { ClassMethodObj { attributes: self.attributes.clone(), flags: self.flags.clone(), byRef: self.byRef.clone(), name: self.name.clone(), params: self.params.clone(), returnType: self.returnType.clone(), stmts: self.stmts.clone(), attrGroups: self.attrGroups.clone() } } }
impl ClassMethod {
    pub fn st_magicNames_cell() -> &'static std::thread::LocalKey<RefCell<Map<Str, bool>>> { thread_local! { static CELL: RefCell<Map<Str, bool>> = RefCell::new({ let mut __m: Map<Str, bool> = Map::new(); __m.insert(Str::from_static("__construct"), true); __m.insert(Str::from_static("__destruct"), true); __m.insert(Str::from_static("__call"), true); __m.insert(Str::from_static("__callstatic"), true); __m.insert(Str::from_static("__get"), true); __m.insert(Str::from_static("__set"), true); __m.insert(Str::from_static("__isset"), true); __m.insert(Str::from_static("__unset"), true); __m.insert(Str::from_static("__sleep"), true); __m.insert(Str::from_static("__wakeup"), true); __m.insert(Str::from_static("__tostring"), true); __m.insert(Str::from_static("__set_state"), true); __m.insert(Str::from_static("__clone"), true); __m.insert(Str::from_static("__invoke"), true); __m.insert(Str::from_static("__debuginfo"), true); __m.insert(Str::from_static("__serialize"), true); __m.insert(Str::from_static("__unserialize"), true); __m }); } &CELL }
    pub fn st_magicNames() -> Map<Str, bool> { Self::st_magicNames_cell().with(|c| c.borrow().clone()) }
    pub fn st_magicNames_set(v: Map<Str, bool>) { Self::st_magicNames_cell().with(|c| { *c.borrow_mut() = v; }) }
    pub fn st_magicNames_with<R>(f: impl FnOnce(&mut Map<Str, bool>) -> R) -> R { Self::st_magicNames_cell().with(|c| f(&mut *c.borrow_mut())) }
}
pub struct Class_Obj {
    pub attributes: Map<Str, Mixed>,
    pub name: Option<crate::php_parser::node::Identifier>,
    pub stmts: Map<ArrayKey, crate::php_parser::node::Stmt>,
    pub attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>,
    pub namespacedName: Option<crate::php_parser::node::Name>,
    pub flags: i64,
    pub extends: Option<crate::php_parser::node::Name>,
    pub implements: Map<ArrayKey, crate::php_parser::node::Name>,
}
#[derive(Clone)]
pub struct Class_(pub Rc<RefCell<Class_Obj>>);
impl Class_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_name(&self) -> Ref<'_, Option<crate::php_parser::node::Identifier>> { Ref::map(self.0.borrow(), |o| &o.name) }
    pub fn p_name_get(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().name.clone() }
    pub fn p_name_opt(&self) -> Option<Option<crate::php_parser::node::Identifier>> { Some(self.0.borrow().name.clone()) }
    pub fn p_name_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Identifier>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.name) }
    pub fn set_p_name(&self, v: Option<crate::php_parser::node::Identifier>) { self.0.borrow_mut().name = v; }
    pub fn p_stmts(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> Map<ArrayKey, crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: Map<ArrayKey, crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn p_attrGroups(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attrGroups) }
    pub fn p_attrGroups_get(&self) -> Map<ArrayKey, crate::php_parser::node::AttributeGroup> { self.0.borrow().attrGroups.clone() }
    pub fn p_attrGroups_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attrGroups.clone()) }
    pub fn p_attrGroups_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attrGroups) }
    pub fn set_p_attrGroups(&self, v: Map<ArrayKey, crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attrGroups = v; }
    pub fn p_namespacedName(&self) -> Ref<'_, Option<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.namespacedName) }
    pub fn p_namespacedName_get(&self) -> Option<crate::php_parser::node::Name> { self.0.borrow().namespacedName.clone() }
    pub fn p_namespacedName_opt(&self) -> Option<Option<crate::php_parser::node::Name>> { Some(self.0.borrow().namespacedName.clone()) }
    pub fn p_namespacedName_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.namespacedName) }
    pub fn set_p_namespacedName(&self, v: Option<crate::php_parser::node::Name>) { self.0.borrow_mut().namespacedName = v; }
    pub fn p_flags(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.flags) }
    pub fn p_flags_get(&self) -> i64 { self.0.borrow().flags.clone() }
    pub fn p_flags_opt(&self) -> Option<i64> { Some(self.0.borrow().flags.clone()) }
    pub fn p_flags_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.flags) }
    pub fn set_p_flags(&self, v: i64) { self.0.borrow_mut().flags = v; }
    pub fn p_extends(&self) -> Ref<'_, Option<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.extends) }
    pub fn p_extends_get(&self) -> Option<crate::php_parser::node::Name> { self.0.borrow().extends.clone() }
    pub fn p_extends_opt(&self) -> Option<Option<crate::php_parser::node::Name>> { Some(self.0.borrow().extends.clone()) }
    pub fn p_extends_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.extends) }
    pub fn set_p_extends(&self, v: Option<crate::php_parser::node::Name>) { self.0.borrow_mut().extends = v; }
    pub fn p_implements(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.implements) }
    pub fn p_implements_get(&self) -> Map<ArrayKey, crate::php_parser::node::Name> { self.0.borrow().implements.clone() }
    pub fn p_implements_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Name>> { Some(self.0.borrow().implements.clone()) }
    pub fn p_implements_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.implements) }
    pub fn set_p_implements(&self, v: Map<ArrayKey, crate::php_parser::node::Name>) { self.0.borrow_mut().implements = v; }
    pub fn new(mut name: Option<U_PhpParser_Node_Identifier_or_Str>, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__467d114822, mut attributes: Map<Str, Mixed>) -> Result<Class_, Throw> {
        let this = Class_(Rc::new(RefCell::new(Class_Obj {
            attributes: Default::default(),
            name: Default::default(),
            stmts: Default::default(),
            attrGroups: Default::default(),
            namespacedName: Default::default(),
            flags: Default::default(),
            extends: Default::default(),
            implements: Default::default(),
        })));
        this.magic__construct(name, subNodes, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: Option<U_PhpParser_Node_Identifier_or_Str>, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__467d114822, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_flags((match Some({ let __c67 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__5dc919da86 { attrGroups: __c67.attrGroups, extends: __c67.extends, flags: __c67.flags.unwrap(), implements: __c67.implements, stmts: __c67.stmts } }).and_then(|__b| Some(__b.flags)) { Some(__v) => __v, None => cast::<i64>((match Some({ let __c68 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__e50280a625 { attrGroups: __c68.attrGroups, extends: __c68.extends, flags: __c68.flags, implements: __c68.implements, stmts: __c68.stmts, type_: <Mixed>::default() } }).and_then(|__b| Some(__b.type_)).and_then(|__m| __m.to_option()) { Some(__v) => __v, None => cast::<Mixed>(0i64) })) }));
    self.set_p_name((if (match name.clone() { Some(__u) => match __u { U_PhpParser_Node_Identifier_or_Str::Str(_) => true, _ => false }, None => false }) { Some(crate::php_parser::node::Identifier::new(cast::<Str>(name.clone().unwrap()), Map::<Str, Mixed>::new())?) } else { name.clone().map(|v| cast::<crate::php_parser::node::Identifier>(v)) }));
    self.set_p_extends((match Some({ let __c69 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extends_P_b62f1a2796 { attrGroups: __c69.attrGroups, extends: __c69.extends.unwrap(), flags: __c69.flags, implements: __c69.implements, stmts: __c69.stmts, type_: None } }).and_then(|__b| Some(__b.extends)) { Some(__v) => Some(__v), None => { let _ = (); None::<crate::php_parser::node::Name> } }));
    self.set_p_implements((match Some({ let __c70 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__e1e5ef5365 { attrGroups: __c70.attrGroups, extends: __c70.extends, flags: __c70.flags, implements: __c70.implements.unwrap(), stmts: __c70.stmts, type_: None } }).and_then(|__b| Some(__b.implements)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Name>::new() }));
    self.set_p_stmts((match Some({ let __c71 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__d0e8168234 { attrGroups: __c71.attrGroups, extends: __c71.extends, flags: __c71.flags, implements: __c71.implements, stmts: __c71.stmts.unwrap(), type_: None } }).and_then(|__b| Some(__b.stmts)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Stmt>::new() }));
    self.set_p_attrGroups((match Some({ let __c72 = subNodes.clone(); Shape_attrGroups_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq_O_a3a18b7d2a { attrGroups: __c72.attrGroups.unwrap(), extends: __c72.extends, flags: __c72.flags, implements: __c72.implements, stmts: __c72.stmts, type_: None } }).and_then(|__b| Some(__b.attrGroups)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::AttributeGroup>::new() }));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("attrGroups"))); __m1.push(cast::<Mixed>(Str::from_static("flags"))); __m1.push(cast::<Mixed>(Str::from_static("name"))); __m1.push(cast::<Mixed>(Str::from_static("extends"))); __m1.push(cast::<Mixed>(Str::from_static("implements"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn isAbstract(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::ABSTRACT())));
    }
    pub fn isFinal(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::FINAL())));
    }
    pub fn isReadonly(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::READONLY())));
    }
    pub fn isAnonymous(&self) -> Result<bool, Throw> {
    return Ok(Some(self.clone()).and_then(|__b| Some(__b.p_name_get())).flatten().is_none());
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Class"));
    }
    pub fn getTraitUses(&self) -> Result<List<crate::php_parser::node::stmt::TraitUse>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getTraitUses__impl() }
    pub fn getConstants(&self) -> Result<List<crate::php_parser::node::stmt::ClassConst>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getConstants__impl() }
    pub fn getProperties(&self) -> Result<List<crate::php_parser::node::stmt::Property>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getProperties__impl() }
    pub fn getProperty(&self, mut name: Str) -> Result<Option<crate::php_parser::node::stmt::Property>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getProperty__impl(name) }
    pub fn getMethods(&self) -> Result<List<crate::php_parser::node::stmt::ClassMethod>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getMethods__impl() }
    pub fn getMethod(&self, mut name: Str) -> Result<Option<crate::php_parser::node::stmt::ClassMethod>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getMethod__impl(name) }
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
    pub fn new_same_class(&self, mut name: Option<U_PhpParser_Node_Identifier_or_Str>, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__467d114822, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Class_, Throw> { Ok(Self::new(name, subNodes, attributes)?) }
}
impl php_rt::PhpObject for Class_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Class_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\class_", "phpparser\\node\\stmt\\classlike", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attrGroups_get()) { out.push((Str::from_static("attrGroups"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_namespacedName_get()) { out.push((Str::from_static("namespacedName"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_flags_get()) { out.push((Str::from_static("flags"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_extends_get()) { out.push((Str::from_static("extends"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_implements_get()) { out.push((Str::from_static("implements"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(value.to_option().map(|__m| cast::<crate::php_parser::node::Identifier>(__m))); true }, "stmts" => { self.set_p_stmts(cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(value)); true }, "attrGroups" => { self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(value)); true }, "namespacedName" => { self.set_p_namespacedName(value.to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m))); true }, "flags" => { self.set_p_flags(cast::<i64>(value)); true }, "extends" => { self.set_p_extends(value.to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m))); true }, "implements" => { self.set_p_implements(cast::<Map<ArrayKey, crate::php_parser::node::Name>>(value)); true }, _ => false } }
}
impl Class_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Class_ could not be converted to string"))) } }
impl php_rt::PhpClone for Class_ { fn php_clone(&self) -> Self { let c = Class_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Class_Obj { fn clone(&self) -> Self { Class_Obj { attributes: self.attributes.clone(), name: self.name.clone(), stmts: self.stmts.clone(), attrGroups: self.attrGroups.clone(), namespacedName: self.namespacedName.clone(), flags: self.flags.clone(), extends: self.extends.clone(), implements: self.implements.clone() } } }
impl Class_ {
    pub fn MODIFIER_PUBLIC() -> i64 { 1i64 }
    pub fn MODIFIER_PROTECTED() -> i64 { 2i64 }
    pub fn MODIFIER_PRIVATE() -> i64 { 4i64 }
    pub fn MODIFIER_STATIC() -> i64 { 8i64 }
    pub fn MODIFIER_ABSTRACT() -> i64 { 16i64 }
    pub fn MODIFIER_FINAL() -> i64 { 32i64 }
    pub fn MODIFIER_READONLY() -> i64 { 64i64 }
    pub fn VISIBILITY_MODIFIER_MASK() -> i64 { 7i64 }
}
pub struct Const_Obj {
    pub attributes: Map<Str, Mixed>,
    pub consts: Map<ArrayKey, crate::php_parser::node::Const_>,
    pub attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>,
}
#[derive(Clone)]
pub struct Const_(pub Rc<RefCell<Const_Obj>>);
impl Const_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_consts(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Const_>> { Ref::map(self.0.borrow(), |o| &o.consts) }
    pub fn p_consts_get(&self) -> Map<ArrayKey, crate::php_parser::node::Const_> { self.0.borrow().consts.clone() }
    pub fn p_consts_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Const_>> { Some(self.0.borrow().consts.clone()) }
    pub fn p_consts_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Const_>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.consts) }
    pub fn set_p_consts(&self, v: Map<ArrayKey, crate::php_parser::node::Const_>) { self.0.borrow_mut().consts = v; }
    pub fn p_attrGroups(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attrGroups) }
    pub fn p_attrGroups_get(&self) -> Map<ArrayKey, crate::php_parser::node::AttributeGroup> { self.0.borrow().attrGroups.clone() }
    pub fn p_attrGroups_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attrGroups.clone()) }
    pub fn p_attrGroups_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attrGroups) }
    pub fn set_p_attrGroups(&self, v: Map<ArrayKey, crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attrGroups = v; }
    pub fn new(mut consts_v: Map<ArrayKey, crate::php_parser::node::Const_>, mut attributes: Map<Str, Mixed>, mut attrGroups: List<crate::php_parser::node::AttributeGroup>) -> Result<Const_, Throw> {
        let this = Const_(Rc::new(RefCell::new(Const_Obj {
            attributes: Default::default(),
            consts: Default::default(),
            attrGroups: Default::default(),
        })));
        this.magic__construct(consts_v, attributes, attrGroups)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut consts_v: Map<ArrayKey, crate::php_parser::node::Const_>, mut attributes: Map<Str, Mixed>, mut attrGroups: List<crate::php_parser::node::AttributeGroup>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(attrGroups.clone()));
    self.set_p_consts(consts_v.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("attrGroups"))); __m1.push(cast::<Mixed>(Str::from_static("consts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Const"));
    }
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
    pub fn new_same_class(&self, mut consts_v: Map<ArrayKey, crate::php_parser::node::Const_>, mut attributes: Map<Str, Mixed>, mut attrGroups: List<crate::php_parser::node::AttributeGroup>) -> Result<crate::php_parser::node::stmt::Const_, Throw> { Ok(Self::new(consts_v, attributes, attrGroups)?) }
}
impl php_rt::PhpObject for Const_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Const_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\const_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_consts_get()) { out.push((Str::from_static("consts"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attrGroups_get()) { out.push((Str::from_static("attrGroups"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "consts" => { self.set_p_consts(cast::<Map<ArrayKey, crate::php_parser::node::Const_>>(value)); true }, "attrGroups" => { self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(value)); true }, _ => false } }
}
impl Const_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Const_ could not be converted to string"))) } }
impl php_rt::PhpClone for Const_ { fn php_clone(&self) -> Self { let c = Const_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Const_Obj { fn clone(&self) -> Self { Const_Obj { attributes: self.attributes.clone(), consts: self.consts.clone(), attrGroups: self.attrGroups.clone() } } }
impl Const_ {
}
pub struct Continue_Obj {
    pub attributes: Map<Str, Mixed>,
    pub num: Option<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Continue_(pub Rc<RefCell<Continue_Obj>>);
impl Continue_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_num(&self) -> Ref<'_, Option<crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.num) }
    pub fn p_num_get(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().num.clone() }
    pub fn p_num_opt(&self) -> Option<Option<crate::php_parser::node::Expr>> { Some(self.0.borrow().num.clone()) }
    pub fn p_num_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.num) }
    pub fn set_p_num(&self, v: Option<crate::php_parser::node::Expr>) { self.0.borrow_mut().num = v; }
    pub fn new(mut num: Option<crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<Continue_, Throw> {
        let this = Continue_(Rc::new(RefCell::new(Continue_Obj {
            attributes: Default::default(),
            num: Default::default(),
        })));
        this.magic__construct(num, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut num: Option<crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_num(num.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("num"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Continue"));
    }
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
    pub fn new_same_class(&self, mut num: Option<crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Continue_, Throw> { Ok(Self::new(num, attributes)?) }
}
impl php_rt::PhpObject for Continue_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Continue_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\continue_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_num_get()) { out.push((Str::from_static("num"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "num" => { self.set_p_num(value.to_option().map(|__m| cast::<crate::php_parser::node::Expr>(__m))); true }, _ => false } }
}
impl Continue_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Continue_ could not be converted to string"))) } }
impl php_rt::PhpClone for Continue_ { fn php_clone(&self) -> Self { let c = Continue_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Continue_Obj { fn clone(&self) -> Self { Continue_Obj { attributes: self.attributes.clone(), num: self.num.clone() } } }
impl Continue_ {
}
pub struct Declare_Obj {
    pub attributes: Map<Str, Mixed>,
    pub declares: Map<ArrayKey, crate::php_parser::node::DeclareItem>,
    pub stmts: Option<Map<ArrayKey, crate::php_parser::node::Stmt>>,
}
#[derive(Clone)]
pub struct Declare_(pub Rc<RefCell<Declare_Obj>>);
impl Declare_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_declares(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::DeclareItem>> { Ref::map(self.0.borrow(), |o| &o.declares) }
    pub fn p_declares_get(&self) -> Map<ArrayKey, crate::php_parser::node::DeclareItem> { self.0.borrow().declares.clone() }
    pub fn p_declares_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::DeclareItem>> { Some(self.0.borrow().declares.clone()) }
    pub fn p_declares_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::DeclareItem>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.declares) }
    pub fn set_p_declares(&self, v: Map<ArrayKey, crate::php_parser::node::DeclareItem>) { self.0.borrow_mut().declares = v; }
    pub fn p_stmts(&self) -> Ref<'_, Option<Map<ArrayKey, crate::php_parser::node::Stmt>>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Stmt>> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<Option<Map<ArrayKey, crate::php_parser::node::Stmt>>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, Option<Map<ArrayKey, crate::php_parser::node::Stmt>>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: Option<Map<ArrayKey, crate::php_parser::node::Stmt>>) { self.0.borrow_mut().stmts = v; }
    pub fn new(mut declares: Map<ArrayKey, crate::php_parser::node::DeclareItem>, mut stmts: Option<Map<ArrayKey, crate::php_parser::node::Stmt>>, mut attributes: Map<Str, Mixed>) -> Result<Declare_, Throw> {
        let this = Declare_(Rc::new(RefCell::new(Declare_Obj {
            attributes: Default::default(),
            declares: Default::default(),
            stmts: Default::default(),
        })));
        this.magic__construct(declares, stmts, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut declares: Map<ArrayKey, crate::php_parser::node::DeclareItem>, mut stmts: Option<Map<ArrayKey, crate::php_parser::node::Stmt>>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_declares(declares.clone());
    self.set_p_stmts(stmts.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("declares"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Declare"));
    }
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
    pub fn new_same_class(&self, mut declares: Map<ArrayKey, crate::php_parser::node::DeclareItem>, mut stmts: Option<Map<ArrayKey, crate::php_parser::node::Stmt>>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Declare_, Throw> { Ok(Self::new(declares, stmts, attributes)?) }
}
impl php_rt::PhpObject for Declare_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Declare_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\declare_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_declares_get()) { out.push((Str::from_static("declares"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "declares" => { self.set_p_declares(cast::<Map<ArrayKey, crate::php_parser::node::DeclareItem>>(value)); true }, "stmts" => { self.set_p_stmts(value.to_option().map(|__m| cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(__m))); true }, _ => false } }
}
impl Declare_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Declare_ could not be converted to string"))) } }
impl php_rt::PhpClone for Declare_ { fn php_clone(&self) -> Self { let c = Declare_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Declare_Obj { fn clone(&self) -> Self { Declare_Obj { attributes: self.attributes.clone(), declares: self.declares.clone(), stmts: self.stmts.clone() } } }
impl Declare_ {
}
pub struct Do_Obj {
    pub attributes: Map<Str, Mixed>,
    pub stmts: List<crate::php_parser::node::Stmt>,
    pub cond: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Do_(pub Rc<RefCell<Do_Obj>>);
impl Do_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_stmts(&self) -> Ref<'_, List<crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> List<crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<List<crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: List<crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn p_cond(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.cond.get()) }
    pub fn p_cond_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().cond.get().clone() }
    pub fn p_cond_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().cond.as_option().cloned() }
    pub fn p_cond_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.cond.get_mut()) }
    pub fn set_p_cond(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().cond.set(v); }
    pub fn new(mut cond: crate::php_parser::node::Expr, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Do_, Throw> {
        let this = Do_(Rc::new(RefCell::new(Do_Obj {
            attributes: Default::default(),
            stmts: Default::default(),
            cond: Late::uninit(),
        })));
        this.magic__construct(cond, stmts, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut cond: crate::php_parser::node::Expr, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_cond(cond.clone());
    self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(stmts.clone()));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1.push(cast::<Mixed>(Str::from_static("cond"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Do"));
    }
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
    pub fn new_same_class(&self, mut cond: crate::php_parser::node::Expr, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Do_, Throw> { Ok(Self::new(cond, stmts, attributes)?) }
}
impl php_rt::PhpObject for Do_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Do_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\do_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } if let Some(v) = self.p_cond_opt() { out.push((Str::from_static("cond"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "stmts" => { self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(value)); true }, "cond" => { self.set_p_cond(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
}
impl Do_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Do_ could not be converted to string"))) } }
impl php_rt::PhpClone for Do_ { fn php_clone(&self) -> Self { let c = Do_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Do_Obj { fn clone(&self) -> Self { Do_Obj { attributes: self.attributes.clone(), stmts: self.stmts.clone(), cond: self.cond.clone() } } }
impl Do_ {
}
pub struct Echo_Obj {
    pub attributes: Map<Str, Mixed>,
    pub exprs: Map<ArrayKey, crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Echo_(pub Rc<RefCell<Echo_Obj>>);
impl Echo_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_exprs(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.exprs) }
    pub fn p_exprs_get(&self) -> Map<ArrayKey, crate::php_parser::node::Expr> { self.0.borrow().exprs.clone() }
    pub fn p_exprs_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Expr>> { Some(self.0.borrow().exprs.clone()) }
    pub fn p_exprs_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.exprs) }
    pub fn set_p_exprs(&self, v: Map<ArrayKey, crate::php_parser::node::Expr>) { self.0.borrow_mut().exprs = v; }
    pub fn new(mut exprs: Map<ArrayKey, crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<Echo_, Throw> {
        let this = Echo_(Rc::new(RefCell::new(Echo_Obj {
            attributes: Default::default(),
            exprs: Default::default(),
        })));
        this.magic__construct(exprs, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut exprs: Map<ArrayKey, crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_exprs(exprs.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("exprs"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Echo"));
    }
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
    pub fn new_same_class(&self, mut exprs: Map<ArrayKey, crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Echo_, Throw> { Ok(Self::new(exprs, attributes)?) }
}
impl php_rt::PhpObject for Echo_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Echo_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\echo_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_exprs_get()) { out.push((Str::from_static("exprs"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "exprs" => { self.set_p_exprs(cast::<Map<ArrayKey, crate::php_parser::node::Expr>>(value)); true }, _ => false } }
}
impl Echo_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Echo_ could not be converted to string"))) } }
impl php_rt::PhpClone for Echo_ { fn php_clone(&self) -> Self { let c = Echo_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Echo_Obj { fn clone(&self) -> Self { Echo_Obj { attributes: self.attributes.clone(), exprs: self.exprs.clone() } } }
impl Echo_ {
}
pub struct ElseIf_Obj {
    pub attributes: Map<Str, Mixed>,
    pub cond: Late<crate::php_parser::node::Expr>,
    pub stmts: List<crate::php_parser::node::Stmt>,
}
#[derive(Clone)]
pub struct ElseIf_(pub Rc<RefCell<ElseIf_Obj>>);
impl ElseIf_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_cond(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.cond.get()) }
    pub fn p_cond_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().cond.get().clone() }
    pub fn p_cond_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().cond.as_option().cloned() }
    pub fn p_cond_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.cond.get_mut()) }
    pub fn set_p_cond(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().cond.set(v); }
    pub fn p_stmts(&self) -> Ref<'_, List<crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> List<crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<List<crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: List<crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn new(mut cond: crate::php_parser::node::Expr, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<ElseIf_, Throw> {
        let this = ElseIf_(Rc::new(RefCell::new(ElseIf_Obj {
            attributes: Default::default(),
            cond: Late::uninit(),
            stmts: Default::default(),
        })));
        this.magic__construct(cond, stmts, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut cond: crate::php_parser::node::Expr, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_cond(cond.clone());
    self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(stmts.clone()));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("cond"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_ElseIf"));
    }
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
    pub fn new_same_class(&self, mut cond: crate::php_parser::node::Expr, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::ElseIf_, Throw> { Ok(Self::new(cond, stmts, attributes)?) }
}
impl php_rt::PhpObject for ElseIf_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\ElseIf_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\elseif_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_cond_opt() { out.push((Str::from_static("cond"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "cond" => { self.set_p_cond(cast::<crate::php_parser::node::Expr>(value)); true }, "stmts" => { self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(value)); true }, _ => false } }
}
impl ElseIf_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\ElseIf_ could not be converted to string"))) } }
impl php_rt::PhpClone for ElseIf_ { fn php_clone(&self) -> Self { let c = ElseIf_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ElseIf_Obj { fn clone(&self) -> Self { ElseIf_Obj { attributes: self.attributes.clone(), cond: self.cond.clone(), stmts: self.stmts.clone() } } }
impl ElseIf_ {
}
pub struct Else_Obj {
    pub attributes: Map<Str, Mixed>,
    pub stmts: List<crate::php_parser::node::Stmt>,
}
#[derive(Clone)]
pub struct Else_(pub Rc<RefCell<Else_Obj>>);
impl Else_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_stmts(&self) -> Ref<'_, List<crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> List<crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<List<crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: List<crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn new(mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Else_, Throw> {
        let this = Else_(Rc::new(RefCell::new(Else_Obj {
            attributes: Default::default(),
            stmts: Default::default(),
        })));
        this.magic__construct(stmts, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(stmts.clone()));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Else"));
    }
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
    pub fn new_same_class(&self, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Else_, Throw> { Ok(Self::new(stmts, attributes)?) }
}
impl php_rt::PhpObject for Else_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Else_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\else_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "stmts" => { self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(value)); true }, _ => false } }
}
impl Else_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Else_ could not be converted to string"))) } }
impl php_rt::PhpClone for Else_ { fn php_clone(&self) -> Self { let c = Else_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Else_Obj { fn clone(&self) -> Self { Else_Obj { attributes: self.attributes.clone(), stmts: self.stmts.clone() } } }
impl Else_ {
}
pub struct EnumCaseObj {
    pub attributes: Map<Str, Mixed>,
    pub name: Late<crate::php_parser::node::Identifier>,
    pub expr: Option<crate::php_parser::node::Expr>,
    pub attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>,
}
#[derive(Clone)]
pub struct EnumCase(pub Rc<RefCell<EnumCaseObj>>);
impl EnumCase {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_name(&self) -> Ref<'_, crate::php_parser::node::Identifier> { Ref::map(self.0.borrow(), |o| o.name.get()) }
    pub fn p_name_get(&self) -> crate::php_parser::node::Identifier { self.0.borrow().name.get().clone() }
    pub fn p_name_opt(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().name.as_option().cloned() }
    pub fn p_name_mut(&self) -> RefMut<'_, crate::php_parser::node::Identifier> { RefMut::map(self.0.borrow_mut(), |o| o.name.get_mut()) }
    pub fn set_p_name(&self, v: crate::php_parser::node::Identifier) { self.0.borrow_mut().name.set(v); }
    pub fn p_expr(&self) -> Ref<'_, Option<crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.expr) }
    pub fn p_expr_get(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.clone() }
    pub fn p_expr_opt(&self) -> Option<Option<crate::php_parser::node::Expr>> { Some(self.0.borrow().expr.clone()) }
    pub fn p_expr_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.expr) }
    pub fn set_p_expr(&self, v: Option<crate::php_parser::node::Expr>) { self.0.borrow_mut().expr = v; }
    pub fn p_attrGroups(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attrGroups) }
    pub fn p_attrGroups_get(&self) -> Map<ArrayKey, crate::php_parser::node::AttributeGroup> { self.0.borrow().attrGroups.clone() }
    pub fn p_attrGroups_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attrGroups.clone()) }
    pub fn p_attrGroups_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attrGroups) }
    pub fn set_p_attrGroups(&self, v: Map<ArrayKey, crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attrGroups = v; }
    pub fn new(mut name: U_PhpParser_Node_Identifier_or_Str, mut expr: Option<crate::php_parser::node::Expr>, mut attrGroups: List<crate::php_parser::node::AttributeGroup>, mut attributes: Map<Str, Mixed>) -> Result<EnumCase, Throw> {
        let this = EnumCase(Rc::new(RefCell::new(EnumCaseObj {
            attributes: Default::default(),
            name: Late::uninit(),
            expr: Default::default(),
            attrGroups: Default::default(),
        })));
        this.magic__construct(name, expr, attrGroups, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut expr: Option<crate::php_parser::node::Expr>, mut attrGroups: List<crate::php_parser::node::AttributeGroup>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    let _: Mixed = cast::<crate::php_parser::NodeAbstract>(self.clone()).magic__construct__impl(attributes.clone())?;
    self.set_p_name((if (match name.clone() { U_PhpParser_Node_Identifier_or_Str::Str(_) => true, _ => false }) { crate::php_parser::node::Identifier::new(cast::<Str>(name.clone()), Map::<Str, Mixed>::new())? } else { cast::<crate::php_parser::node::Identifier>(name.clone()) }));
    self.set_p_expr(expr.clone());
    self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(attrGroups.clone()));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("attrGroups"))); __m1.push(cast::<Mixed>(Str::from_static("name"))); __m1.push(cast::<Mixed>(Str::from_static("expr"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_EnumCase"));
    }
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
    pub fn new_same_class(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut expr: Option<crate::php_parser::node::Expr>, mut attrGroups: List<crate::php_parser::node::AttributeGroup>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::EnumCase, Throw> { Ok(Self::new(name, expr, attrGroups, attributes)?) }
}
impl php_rt::PhpObject for EnumCase {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\EnumCase" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\enumcase", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_name_opt() { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_expr_get()) { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attrGroups_get()) { out.push((Str::from_static("attrGroups"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(cast::<crate::php_parser::node::Identifier>(value)); true }, "expr" => { self.set_p_expr(value.to_option().map(|__m| cast::<crate::php_parser::node::Expr>(__m))); true }, "attrGroups" => { self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(value)); true }, _ => false } }
}
impl EnumCase { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\EnumCase could not be converted to string"))) } }
impl php_rt::PhpClone for EnumCase { fn php_clone(&self) -> Self { let c = EnumCase(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for EnumCaseObj { fn clone(&self) -> Self { EnumCaseObj { attributes: self.attributes.clone(), name: self.name.clone(), expr: self.expr.clone(), attrGroups: self.attrGroups.clone() } } }
impl EnumCase {
}
pub struct Enum_Obj {
    pub attributes: Map<Str, Mixed>,
    pub name: Option<crate::php_parser::node::Identifier>,
    pub stmts: Map<ArrayKey, crate::php_parser::node::Stmt>,
    pub attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>,
    pub namespacedName: Option<crate::php_parser::node::Name>,
    pub scalarType: Option<crate::php_parser::node::Identifier>,
    pub implements: Map<ArrayKey, crate::php_parser::node::Name>,
}
#[derive(Clone)]
pub struct Enum_(pub Rc<RefCell<Enum_Obj>>);
impl Enum_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_name(&self) -> Ref<'_, Option<crate::php_parser::node::Identifier>> { Ref::map(self.0.borrow(), |o| &o.name) }
    pub fn p_name_get(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().name.clone() }
    pub fn p_name_opt(&self) -> Option<Option<crate::php_parser::node::Identifier>> { Some(self.0.borrow().name.clone()) }
    pub fn p_name_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Identifier>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.name) }
    pub fn set_p_name(&self, v: Option<crate::php_parser::node::Identifier>) { self.0.borrow_mut().name = v; }
    pub fn p_stmts(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> Map<ArrayKey, crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: Map<ArrayKey, crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn p_attrGroups(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attrGroups) }
    pub fn p_attrGroups_get(&self) -> Map<ArrayKey, crate::php_parser::node::AttributeGroup> { self.0.borrow().attrGroups.clone() }
    pub fn p_attrGroups_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attrGroups.clone()) }
    pub fn p_attrGroups_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attrGroups) }
    pub fn set_p_attrGroups(&self, v: Map<ArrayKey, crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attrGroups = v; }
    pub fn p_namespacedName(&self) -> Ref<'_, Option<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.namespacedName) }
    pub fn p_namespacedName_get(&self) -> Option<crate::php_parser::node::Name> { self.0.borrow().namespacedName.clone() }
    pub fn p_namespacedName_opt(&self) -> Option<Option<crate::php_parser::node::Name>> { Some(self.0.borrow().namespacedName.clone()) }
    pub fn p_namespacedName_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.namespacedName) }
    pub fn set_p_namespacedName(&self, v: Option<crate::php_parser::node::Name>) { self.0.borrow_mut().namespacedName = v; }
    pub fn p_scalarType(&self) -> Ref<'_, Option<crate::php_parser::node::Identifier>> { Ref::map(self.0.borrow(), |o| &o.scalarType) }
    pub fn p_scalarType_get(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().scalarType.clone() }
    pub fn p_scalarType_opt(&self) -> Option<Option<crate::php_parser::node::Identifier>> { Some(self.0.borrow().scalarType.clone()) }
    pub fn p_scalarType_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Identifier>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.scalarType) }
    pub fn set_p_scalarType(&self, v: Option<crate::php_parser::node::Identifier>) { self.0.borrow_mut().scalarType = v; }
    pub fn p_implements(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.implements) }
    pub fn p_implements_get(&self) -> Map<ArrayKey, crate::php_parser::node::Name> { self.0.borrow().implements.clone() }
    pub fn p_implements_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Name>> { Some(self.0.borrow().implements.clone()) }
    pub fn p_implements_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.implements) }
    pub fn set_p_implements(&self, v: Map<ArrayKey, crate::php_parser::node::Name>) { self.0.borrow_mut().implements = v; }
    pub fn new(mut name: Option<U_PhpParser_Node_Identifier_or_Str>, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_implement_38f3035d6c, mut attributes: Map<Str, Mixed>) -> Result<Enum_, Throw> {
        let this = Enum_(Rc::new(RefCell::new(Enum_Obj {
            attributes: Default::default(),
            name: Default::default(),
            stmts: Default::default(),
            attrGroups: Default::default(),
            namespacedName: Default::default(),
            scalarType: Default::default(),
            implements: Default::default(),
        })));
        this.magic__construct(name, subNodes, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: Option<U_PhpParser_Node_Identifier_or_Str>, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_implement_38f3035d6c, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_name((if (match name.clone() { Some(__u) => match __u { U_PhpParser_Node_Identifier_or_Str::Str(_) => true, _ => false }, None => false }) { Some(crate::php_parser::node::Identifier::new(cast::<Str>(name.clone().unwrap()), Map::<Str, Mixed>::new())?) } else { name.clone().map(|v| cast::<crate::php_parser::node::Identifier>(v)) }));
    self.set_p_scalarType((match Some({ let __c73 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_implement_91444321f0 { attrGroups: __c73.attrGroups, implements: __c73.implements, scalarType: __c73.scalarType.unwrap(), stmts: __c73.stmts } }).and_then(|__b| Some(__b.scalarType)) { Some(__v) => Some(__v), None => { let _ = (); None::<crate::php_parser::node::Identifier> } }));
    self.set_p_implements((match Some({ let __c74 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_implement_7aa05101ca { attrGroups: __c74.attrGroups, implements: __c74.implements.unwrap(), scalarType: __c74.scalarType, stmts: __c74.stmts } }).and_then(|__b| Some(__b.implements)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Name>::new() }));
    self.set_p_stmts((match Some({ let __c75 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_implement_45d5f13a5d { attrGroups: __c75.attrGroups, implements: __c75.implements, scalarType: __c75.scalarType, stmts: __c75.stmts.unwrap() } }).and_then(|__b| Some(__b.stmts)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Stmt>::new() }));
    self.set_p_attrGroups((match Some({ let __c76 = subNodes.clone(); Shape_attrGroups_Map_ArrayKey_PhpParser_Node_AttributeGroup_implements_fe2f08e8aa { attrGroups: __c76.attrGroups.unwrap(), implements: __c76.implements, scalarType: __c76.scalarType, stmts: __c76.stmts } }).and_then(|__b| Some(__b.attrGroups)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::AttributeGroup>::new() }));
    let _: Mixed = cast::<crate::php_parser::NodeAbstract>(self.clone()).magic__construct__impl(attributes.clone())?;
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("attrGroups"))); __m1.push(cast::<Mixed>(Str::from_static("name"))); __m1.push(cast::<Mixed>(Str::from_static("scalarType"))); __m1.push(cast::<Mixed>(Str::from_static("implements"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Enum"));
    }
    pub fn getTraitUses(&self) -> Result<List<crate::php_parser::node::stmt::TraitUse>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getTraitUses__impl() }
    pub fn getConstants(&self) -> Result<List<crate::php_parser::node::stmt::ClassConst>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getConstants__impl() }
    pub fn getProperties(&self) -> Result<List<crate::php_parser::node::stmt::Property>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getProperties__impl() }
    pub fn getProperty(&self, mut name: Str) -> Result<Option<crate::php_parser::node::stmt::Property>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getProperty__impl(name) }
    pub fn getMethods(&self) -> Result<List<crate::php_parser::node::stmt::ClassMethod>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getMethods__impl() }
    pub fn getMethod(&self, mut name: Str) -> Result<Option<crate::php_parser::node::stmt::ClassMethod>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getMethod__impl(name) }
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
    pub fn new_same_class(&self, mut name: Option<U_PhpParser_Node_Identifier_or_Str>, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_implement_38f3035d6c, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Enum_, Throw> { Ok(Self::new(name, subNodes, attributes)?) }
}
impl php_rt::PhpObject for Enum_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Enum_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\enum_", "phpparser\\node\\stmt\\classlike", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attrGroups_get()) { out.push((Str::from_static("attrGroups"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_namespacedName_get()) { out.push((Str::from_static("namespacedName"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_scalarType_get()) { out.push((Str::from_static("scalarType"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_implements_get()) { out.push((Str::from_static("implements"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(value.to_option().map(|__m| cast::<crate::php_parser::node::Identifier>(__m))); true }, "stmts" => { self.set_p_stmts(cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(value)); true }, "attrGroups" => { self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(value)); true }, "namespacedName" => { self.set_p_namespacedName(value.to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m))); true }, "scalarType" => { self.set_p_scalarType(value.to_option().map(|__m| cast::<crate::php_parser::node::Identifier>(__m))); true }, "implements" => { self.set_p_implements(cast::<Map<ArrayKey, crate::php_parser::node::Name>>(value)); true }, _ => false } }
}
impl Enum_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Enum_ could not be converted to string"))) } }
impl php_rt::PhpClone for Enum_ { fn php_clone(&self) -> Self { let c = Enum_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Enum_Obj { fn clone(&self) -> Self { Enum_Obj { attributes: self.attributes.clone(), name: self.name.clone(), stmts: self.stmts.clone(), attrGroups: self.attrGroups.clone(), namespacedName: self.namespacedName.clone(), scalarType: self.scalarType.clone(), implements: self.implements.clone() } } }
impl Enum_ {
}
pub struct ExpressionObj {
    pub attributes: Map<Str, Mixed>,
    pub expr: Late<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Expression(pub Rc<RefCell<ExpressionObj>>);
impl Expression {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn new(mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Expression, Throw> {
        let this = Expression(Rc::new(RefCell::new(ExpressionObj {
            attributes: Default::default(),
            expr: Late::uninit(),
        })));
        this.magic__construct(expr, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_expr(expr.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("expr"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Expression"));
    }
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
    pub fn new_same_class(&self, mut expr: crate::php_parser::node::Expr, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Expression, Throw> { Ok(Self::new(expr, attributes)?) }
}
impl php_rt::PhpObject for Expression {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Expression" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\expression", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, _ => false } }
}
impl Expression { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Expression could not be converted to string"))) } }
impl php_rt::PhpClone for Expression { fn php_clone(&self) -> Self { let c = Expression(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ExpressionObj { fn clone(&self) -> Self { ExpressionObj { attributes: self.attributes.clone(), expr: self.expr.clone() } } }
impl Expression {
}
pub struct Finally_Obj {
    pub attributes: Map<Str, Mixed>,
    pub stmts: List<crate::php_parser::node::Stmt>,
}
#[derive(Clone)]
pub struct Finally_(pub Rc<RefCell<Finally_Obj>>);
impl Finally_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_stmts(&self) -> Ref<'_, List<crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> List<crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<List<crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: List<crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn new(mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Finally_, Throw> {
        let this = Finally_(Rc::new(RefCell::new(Finally_Obj {
            attributes: Default::default(),
            stmts: Default::default(),
        })));
        this.magic__construct(stmts, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(stmts.clone()));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Finally"));
    }
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
    pub fn new_same_class(&self, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Finally_, Throw> { Ok(Self::new(stmts, attributes)?) }
}
impl php_rt::PhpObject for Finally_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Finally_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\finally_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "stmts" => { self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(value)); true }, _ => false } }
}
impl Finally_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Finally_ could not be converted to string"))) } }
impl php_rt::PhpClone for Finally_ { fn php_clone(&self) -> Self { let c = Finally_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Finally_Obj { fn clone(&self) -> Self { Finally_Obj { attributes: self.attributes.clone(), stmts: self.stmts.clone() } } }
impl Finally_ {
}
pub struct For_Obj {
    pub attributes: Map<Str, Mixed>,
    pub init: Map<ArrayKey, crate::php_parser::node::Expr>,
    pub cond: Map<ArrayKey, crate::php_parser::node::Expr>,
    pub loop_: Map<ArrayKey, crate::php_parser::node::Expr>,
    pub stmts: List<crate::php_parser::node::Stmt>,
}
#[derive(Clone)]
pub struct For_(pub Rc<RefCell<For_Obj>>);
impl For_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_init(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.init) }
    pub fn p_init_get(&self) -> Map<ArrayKey, crate::php_parser::node::Expr> { self.0.borrow().init.clone() }
    pub fn p_init_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Expr>> { Some(self.0.borrow().init.clone()) }
    pub fn p_init_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.init) }
    pub fn set_p_init(&self, v: Map<ArrayKey, crate::php_parser::node::Expr>) { self.0.borrow_mut().init = v; }
    pub fn p_cond(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.cond) }
    pub fn p_cond_get(&self) -> Map<ArrayKey, crate::php_parser::node::Expr> { self.0.borrow().cond.clone() }
    pub fn p_cond_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Expr>> { Some(self.0.borrow().cond.clone()) }
    pub fn p_cond_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.cond) }
    pub fn set_p_cond(&self, v: Map<ArrayKey, crate::php_parser::node::Expr>) { self.0.borrow_mut().cond = v; }
    pub fn p_loop_(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.loop_) }
    pub fn p_loop__get(&self) -> Map<ArrayKey, crate::php_parser::node::Expr> { self.0.borrow().loop_.clone() }
    pub fn p_loop__opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Expr>> { Some(self.0.borrow().loop_.clone()) }
    pub fn p_loop__mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.loop_) }
    pub fn set_p_loop_(&self, v: Map<ArrayKey, crate::php_parser::node::Expr>) { self.0.borrow_mut().loop_ = v; }
    pub fn p_stmts(&self) -> Ref<'_, List<crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> List<crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<List<crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: List<crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn new(mut subNodes: Shape_condq_Map_ArrayKey_PhpParser_Node_Expr_initq_Map_ArrayKey_PhpPar_80e922f32d, mut attributes: Map<Str, Mixed>) -> Result<For_, Throw> {
        let this = For_(Rc::new(RefCell::new(For_Obj {
            attributes: Default::default(),
            init: Default::default(),
            cond: Default::default(),
            loop_: Default::default(),
            stmts: Default::default(),
        })));
        this.magic__construct(subNodes, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut subNodes: Shape_condq_Map_ArrayKey_PhpParser_Node_Expr_initq_Map_ArrayKey_PhpPar_80e922f32d, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_init((match Some({ let __c77 = subNodes.clone(); Shape_condq_Map_ArrayKey_PhpParser_Node_Expr_init_Map_ArrayKey_PhpPars_ca956a77ff { cond: __c77.cond, init: __c77.init.unwrap(), loop_: __c77.loop_, stmts: __c77.stmts } }).and_then(|__b| Some(__b.init)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Expr>::new() }));
    self.set_p_cond((match Some({ let __c78 = subNodes.clone(); Shape_cond_Map_ArrayKey_PhpParser_Node_Expr_initq_Map_ArrayKey_PhpPars_f1baea72e7 { cond: __c78.cond.unwrap(), init: __c78.init, loop_: __c78.loop_, stmts: __c78.stmts } }).and_then(|__b| Some(__b.cond)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Expr>::new() }));
    self.set_p_loop_((match Some({ let __c79 = subNodes.clone(); Shape_condq_Map_ArrayKey_PhpParser_Node_Expr_initq_Map_ArrayKey_PhpPar_e436fb3227 { cond: __c79.cond, init: __c79.init, loop_: __c79.loop_.unwrap(), stmts: __c79.stmts } }).and_then(|__b| Some(__b.loop_)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Expr>::new() }));
    self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>((match Some({ let __c80 = subNodes.clone(); Shape_condq_Map_ArrayKey_PhpParser_Node_Expr_initq_Map_ArrayKey_PhpPar_618327657e { cond: __c80.cond, init: __c80.init, loop_: __c80.loop_, stmts: __c80.stmts.unwrap() } }).and_then(|__b| Some(__b.stmts)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Stmt>::new() })));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("init"))); __m1.push(cast::<Mixed>(Str::from_static("cond"))); __m1.push(cast::<Mixed>(Str::from_static("loop"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_For"));
    }
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
    pub fn new_same_class(&self, mut subNodes: Shape_condq_Map_ArrayKey_PhpParser_Node_Expr_initq_Map_ArrayKey_PhpPar_80e922f32d, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::For_, Throw> { Ok(Self::new(subNodes, attributes)?) }
}
impl php_rt::PhpObject for For_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\For_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\for_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_init_get()) { out.push((Str::from_static("init"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_cond_get()) { out.push((Str::from_static("cond"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_loop__get()) { out.push((Str::from_static("loop"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "init" => { self.set_p_init(cast::<Map<ArrayKey, crate::php_parser::node::Expr>>(value)); true }, "cond" => { self.set_p_cond(cast::<Map<ArrayKey, crate::php_parser::node::Expr>>(value)); true }, "loop" => { self.set_p_loop_(cast::<Map<ArrayKey, crate::php_parser::node::Expr>>(value)); true }, "stmts" => { self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(value)); true }, _ => false } }
}
impl For_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\For_ could not be converted to string"))) } }
impl php_rt::PhpClone for For_ { fn php_clone(&self) -> Self { let c = For_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for For_Obj { fn clone(&self) -> Self { For_Obj { attributes: self.attributes.clone(), init: self.init.clone(), cond: self.cond.clone(), loop_: self.loop_.clone(), stmts: self.stmts.clone() } } }
impl For_ {
}
pub struct Foreach_Obj {
    pub attributes: Map<Str, Mixed>,
    pub expr: Late<crate::php_parser::node::Expr>,
    pub keyVar: Option<crate::php_parser::node::Expr>,
    pub byRef: bool,
    pub valueVar: Late<crate::php_parser::node::Expr>,
    pub stmts: List<crate::php_parser::node::Stmt>,
}
#[derive(Clone)]
pub struct Foreach_(pub Rc<RefCell<Foreach_Obj>>);
impl Foreach_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_expr(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.expr.get()) }
    pub fn p_expr_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().expr.get().clone() }
    pub fn p_expr_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.as_option().cloned() }
    pub fn p_expr_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.expr.get_mut()) }
    pub fn set_p_expr(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().expr.set(v); }
    pub fn p_keyVar(&self) -> Ref<'_, Option<crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.keyVar) }
    pub fn p_keyVar_get(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().keyVar.clone() }
    pub fn p_keyVar_opt(&self) -> Option<Option<crate::php_parser::node::Expr>> { Some(self.0.borrow().keyVar.clone()) }
    pub fn p_keyVar_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.keyVar) }
    pub fn set_p_keyVar(&self, v: Option<crate::php_parser::node::Expr>) { self.0.borrow_mut().keyVar = v; }
    pub fn p_byRef(&self) -> Ref<'_, bool> { Ref::map(self.0.borrow(), |o| &o.byRef) }
    pub fn p_byRef_get(&self) -> bool { self.0.borrow().byRef.clone() }
    pub fn p_byRef_opt(&self) -> Option<bool> { Some(self.0.borrow().byRef.clone()) }
    pub fn p_byRef_mut(&self) -> RefMut<'_, bool> { RefMut::map(self.0.borrow_mut(), |o| &mut o.byRef) }
    pub fn set_p_byRef(&self, v: bool) { self.0.borrow_mut().byRef = v; }
    pub fn p_valueVar(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.valueVar.get()) }
    pub fn p_valueVar_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().valueVar.get().clone() }
    pub fn p_valueVar_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().valueVar.as_option().cloned() }
    pub fn p_valueVar_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.valueVar.get_mut()) }
    pub fn set_p_valueVar(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().valueVar.set(v); }
    pub fn p_stmts(&self) -> Ref<'_, List<crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> List<crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<List<crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: List<crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn new(mut expr: crate::php_parser::node::Expr, mut valueVar: crate::php_parser::node::Expr, mut subNodes: Shape_byRefq_Bool_keyVarq_Opt_PhpParser_Node_Expr_stmtsq_Map_ArrayKey_PhpParser_Node_Stmt, mut attributes: Map<Str, Mixed>) -> Result<Foreach_, Throw> {
        let this = Foreach_(Rc::new(RefCell::new(Foreach_Obj {
            attributes: Default::default(),
            expr: Late::uninit(),
            keyVar: Default::default(),
            byRef: Default::default(),
            valueVar: Late::uninit(),
            stmts: Default::default(),
        })));
        this.magic__construct(expr, valueVar, subNodes, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut expr: crate::php_parser::node::Expr, mut valueVar: crate::php_parser::node::Expr, mut subNodes: Shape_byRefq_Bool_keyVarq_Opt_PhpParser_Node_Expr_stmtsq_Map_ArrayKey_PhpParser_Node_Stmt, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_expr(expr.clone());
    self.set_p_keyVar((match Some({ let __c81 = subNodes.clone(); Shape_byRefq_Bool_keyVar_PhpParser_Node_Expr_stmtsq_Map_ArrayKey_PhpParser_Node_Stmt { byRef: __c81.byRef, keyVar: __c81.keyVar.unwrap(), stmts: __c81.stmts } }).and_then(|__b| Some(__b.keyVar)) { Some(__v) => Some(__v), None => { let _ = (); None::<crate::php_parser::node::Expr> } }));
    self.set_p_byRef((match Some({ let __c82 = subNodes.clone(); Shape_byRef_Bool_keyVarq_Opt_PhpParser_Node_Expr_stmtsq_Map_ArrayKey_PhpParser_Node_Stmt { byRef: __c82.byRef.unwrap(), keyVar: __c82.keyVar, stmts: __c82.stmts } }).and_then(|__b| Some(__b.byRef)) { Some(__v) => __v, None => false }));
    self.set_p_valueVar(valueVar.clone());
    self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>((match Some({ let __c83 = subNodes.clone(); Shape_byRefq_Bool_keyVarq_Opt_PhpParser_Node_Expr_stmts_Map_ArrayKey_PhpParser_Node_Stmt { byRef: __c83.byRef, keyVar: __c83.keyVar, stmts: __c83.stmts.unwrap() } }).and_then(|__b| Some(__b.stmts)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Stmt>::new() })));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("expr"))); __m1.push(cast::<Mixed>(Str::from_static("keyVar"))); __m1.push(cast::<Mixed>(Str::from_static("byRef"))); __m1.push(cast::<Mixed>(Str::from_static("valueVar"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Foreach"));
    }
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
    pub fn new_same_class(&self, mut expr: crate::php_parser::node::Expr, mut valueVar: crate::php_parser::node::Expr, mut subNodes: Shape_byRefq_Bool_keyVarq_Opt_PhpParser_Node_Expr_stmtsq_Map_ArrayKey_PhpParser_Node_Stmt, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Foreach_, Throw> { Ok(Self::new(expr, valueVar, subNodes, attributes)?) }
}
impl php_rt::PhpObject for Foreach_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Foreach_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\foreach_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_expr_opt() { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_keyVar_get()) { out.push((Str::from_static("keyVar"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_byRef_get()) { out.push((Str::from_static("byRef"), cast::<Mixed>(v))); } if let Some(v) = self.p_valueVar_opt() { out.push((Str::from_static("valueVar"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "expr" => { self.set_p_expr(cast::<crate::php_parser::node::Expr>(value)); true }, "keyVar" => { self.set_p_keyVar(value.to_option().map(|__m| cast::<crate::php_parser::node::Expr>(__m))); true }, "byRef" => { self.set_p_byRef(cast::<bool>(value)); true }, "valueVar" => { self.set_p_valueVar(cast::<crate::php_parser::node::Expr>(value)); true }, "stmts" => { self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(value)); true }, _ => false } }
}
impl Foreach_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Foreach_ could not be converted to string"))) } }
impl php_rt::PhpClone for Foreach_ { fn php_clone(&self) -> Self { let c = Foreach_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Foreach_Obj { fn clone(&self) -> Self { Foreach_Obj { attributes: self.attributes.clone(), expr: self.expr.clone(), keyVar: self.keyVar.clone(), byRef: self.byRef.clone(), valueVar: self.valueVar.clone(), stmts: self.stmts.clone() } } }
impl Foreach_ {
}
pub struct Function_Obj {
    pub attributes: Map<Str, Mixed>,
    pub byRef: bool,
    pub name: Late<crate::php_parser::node::Identifier>,
    pub params: Map<ArrayKey, crate::php_parser::node::Param>,
    pub returnType: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>,
    pub stmts: Map<ArrayKey, crate::php_parser::node::Stmt>,
    pub attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>,
    pub namespacedName: Option<crate::php_parser::node::Name>,
}
#[derive(Clone)]
pub struct Function_(pub Rc<RefCell<Function_Obj>>);
impl Function_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_byRef(&self) -> Ref<'_, bool> { Ref::map(self.0.borrow(), |o| &o.byRef) }
    pub fn p_byRef_get(&self) -> bool { self.0.borrow().byRef.clone() }
    pub fn p_byRef_opt(&self) -> Option<bool> { Some(self.0.borrow().byRef.clone()) }
    pub fn p_byRef_mut(&self) -> RefMut<'_, bool> { RefMut::map(self.0.borrow_mut(), |o| &mut o.byRef) }
    pub fn set_p_byRef(&self, v: bool) { self.0.borrow_mut().byRef = v; }
    pub fn p_name(&self) -> Ref<'_, crate::php_parser::node::Identifier> { Ref::map(self.0.borrow(), |o| o.name.get()) }
    pub fn p_name_get(&self) -> crate::php_parser::node::Identifier { self.0.borrow().name.get().clone() }
    pub fn p_name_opt(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().name.as_option().cloned() }
    pub fn p_name_mut(&self) -> RefMut<'_, crate::php_parser::node::Identifier> { RefMut::map(self.0.borrow_mut(), |o| o.name.get_mut()) }
    pub fn set_p_name(&self, v: crate::php_parser::node::Identifier) { self.0.borrow_mut().name.set(v); }
    pub fn p_params(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Param>> { Ref::map(self.0.borrow(), |o| &o.params) }
    pub fn p_params_get(&self) -> Map<ArrayKey, crate::php_parser::node::Param> { self.0.borrow().params.clone() }
    pub fn p_params_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Param>> { Some(self.0.borrow().params.clone()) }
    pub fn p_params_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Param>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.params) }
    pub fn set_p_params(&self, v: Map<ArrayKey, crate::php_parser::node::Param>) { self.0.borrow_mut().params = v; }
    pub fn p_returnType(&self) -> Ref<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { Ref::map(self.0.borrow(), |o| &o.returnType) }
    pub fn p_returnType_get(&self) -> Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> { self.0.borrow().returnType.clone() }
    pub fn p_returnType_opt(&self) -> Option<Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { Some(self.0.borrow().returnType.clone()) }
    pub fn p_returnType_mut(&self) -> RefMut<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.returnType) }
    pub fn set_p_returnType(&self, v: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>) { self.0.borrow_mut().returnType = v; }
    pub fn p_stmts(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> Map<ArrayKey, crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: Map<ArrayKey, crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn p_attrGroups(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attrGroups) }
    pub fn p_attrGroups_get(&self) -> Map<ArrayKey, crate::php_parser::node::AttributeGroup> { self.0.borrow().attrGroups.clone() }
    pub fn p_attrGroups_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attrGroups.clone()) }
    pub fn p_attrGroups_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attrGroups) }
    pub fn set_p_attrGroups(&self, v: Map<ArrayKey, crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attrGroups = v; }
    pub fn p_namespacedName(&self) -> Ref<'_, Option<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.namespacedName) }
    pub fn p_namespacedName_get(&self) -> Option<crate::php_parser::node::Name> { self.0.borrow().namespacedName.clone() }
    pub fn p_namespacedName_opt(&self) -> Option<Option<crate::php_parser::node::Name>> { Some(self.0.borrow().namespacedName.clone()) }
    pub fn p_namespacedName_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.namespacedName) }
    pub fn set_p_namespacedName(&self, v: Option<crate::php_parser::node::Name>) { self.0.borrow_mut().namespacedName = v; }
    pub fn new(mut name: U_PhpParser_Node_Identifier_or_Str, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_b3b37d5382, mut attributes: Map<Str, Mixed>) -> Result<Function_, Throw> {
        let this = Function_(Rc::new(RefCell::new(Function_Obj {
            attributes: Default::default(),
            byRef: Default::default(),
            name: Late::uninit(),
            params: Default::default(),
            returnType: Default::default(),
            stmts: Default::default(),
            attrGroups: Default::default(),
            namespacedName: Default::default(),
        })));
        this.magic__construct(name, subNodes, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_b3b37d5382, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_byRef((match Some({ let __c84 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRef_Boo_f6b56c0e89 { attrGroups: __c84.attrGroups, byRef: __c84.byRef.unwrap(), params: __c84.params, returnType: __c84.returnType, stmts: __c84.stmts } }).and_then(|__b| Some(__b.byRef)) { Some(__v) => __v, None => false }));
    self.set_p_name((if (match name.clone() { U_PhpParser_Node_Identifier_or_Str::Str(_) => true, _ => false }) { crate::php_parser::node::Identifier::new(cast::<Str>(name.clone()), Map::<Str, Mixed>::new())? } else { cast::<crate::php_parser::node::Identifier>(name.clone()) }));
    self.set_p_params((match Some({ let __c85 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_1e62798976 { attrGroups: __c85.attrGroups, byRef: __c85.byRef, params: __c85.params.unwrap(), returnType: __c85.returnType, stmts: __c85.stmts } }).and_then(|__b| Some(__b.params)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Param>::new() }));
    self.set_p_returnType((match Some({ let __c86 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_fc45eef33d { attrGroups: __c86.attrGroups, byRef: __c86.byRef, params: __c86.params, returnType: __c86.returnType.unwrap(), stmts: __c86.stmts } }).and_then(|__b| Some(__b.returnType)) { Some(__v) => Some(__v), None => { let _ = (); None::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> } }));
    self.set_p_stmts((match Some({ let __c87 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_6d759b9110 { attrGroups: __c87.attrGroups, byRef: __c87.byRef, params: __c87.params, returnType: __c87.returnType, stmts: __c87.stmts.unwrap() } }).and_then(|__b| Some(__b.stmts)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Stmt>::new() }));
    self.set_p_attrGroups((match Some({ let __c88 = subNodes.clone(); Shape_attrGroups_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Boo_ef3ee6736b { attrGroups: __c88.attrGroups.unwrap(), byRef: __c88.byRef, params: __c88.params, returnType: __c88.returnType, stmts: __c88.stmts } }).and_then(|__b| Some(__b.attrGroups)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::AttributeGroup>::new() }));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("attrGroups"))); __m1.push(cast::<Mixed>(Str::from_static("byRef"))); __m1.push(cast::<Mixed>(Str::from_static("name"))); __m1.push(cast::<Mixed>(Str::from_static("params"))); __m1.push(cast::<Mixed>(Str::from_static("returnType"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn returnsByRef(&self) -> Result<bool, Throw> {
    return Ok(self.p_byRef_get());
    }
    pub fn getParams(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok(self.p_params_get().map_values(|v| cast::<Mixed>(v)));
    }
    pub fn getReturnType(&self) -> Result<Mixed, Throw> {
    return Ok(cast::<Mixed>(self.p_returnType_get()));
    }
    pub fn getAttrGroups(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok(self.p_attrGroups_get().map_values(|v| cast::<Mixed>(v)));
    }
    pub fn getStmts(&self) -> Result<Map<ArrayKey, crate::php_parser::node::Stmt>, Throw> {
    return Ok(self.p_stmts_get());
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Function"));
    }
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
    pub fn new_same_class(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_b3b37d5382, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Function_, Throw> { Ok(Self::new(name, subNodes, attributes)?) }
}
impl php_rt::PhpObject for Function_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Function_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\function_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable", "phpparser\\node\\functionlike"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_byRef_get()) { out.push((Str::from_static("byRef"), cast::<Mixed>(v))); } if let Some(v) = self.p_name_opt() { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_params_get()) { out.push((Str::from_static("params"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_returnType_get()) { out.push((Str::from_static("returnType"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attrGroups_get()) { out.push((Str::from_static("attrGroups"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_namespacedName_get()) { out.push((Str::from_static("namespacedName"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "byRef" => { self.set_p_byRef(cast::<bool>(value)); true }, "name" => { self.set_p_name(cast::<crate::php_parser::node::Identifier>(value)); true }, "params" => { self.set_p_params(cast::<Map<ArrayKey, crate::php_parser::node::Param>>(value)); true }, "returnType" => { self.set_p_returnType(value.to_option().map(|__m| cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(__m))); true }, "stmts" => { self.set_p_stmts(cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(value)); true }, "attrGroups" => { self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(value)); true }, "namespacedName" => { self.set_p_namespacedName(value.to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m))); true }, _ => false } }
}
impl Function_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Function_ could not be converted to string"))) } }
impl php_rt::PhpClone for Function_ { fn php_clone(&self) -> Self { let c = Function_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Function_Obj { fn clone(&self) -> Self { Function_Obj { attributes: self.attributes.clone(), byRef: self.byRef.clone(), name: self.name.clone(), params: self.params.clone(), returnType: self.returnType.clone(), stmts: self.stmts.clone(), attrGroups: self.attrGroups.clone(), namespacedName: self.namespacedName.clone() } } }
impl Function_ {
}
pub struct Global_Obj {
    pub attributes: Map<Str, Mixed>,
    pub vars: Map<ArrayKey, crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Global_(pub Rc<RefCell<Global_Obj>>);
impl Global_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_vars(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.vars) }
    pub fn p_vars_get(&self) -> Map<ArrayKey, crate::php_parser::node::Expr> { self.0.borrow().vars.clone() }
    pub fn p_vars_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Expr>> { Some(self.0.borrow().vars.clone()) }
    pub fn p_vars_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.vars) }
    pub fn set_p_vars(&self, v: Map<ArrayKey, crate::php_parser::node::Expr>) { self.0.borrow_mut().vars = v; }
    pub fn new(mut vars: Map<ArrayKey, crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<Global_, Throw> {
        let this = Global_(Rc::new(RefCell::new(Global_Obj {
            attributes: Default::default(),
            vars: Default::default(),
        })));
        this.magic__construct(vars, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut vars: Map<ArrayKey, crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_vars(vars.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("vars"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Global"));
    }
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
    pub fn new_same_class(&self, mut vars: Map<ArrayKey, crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Global_, Throw> { Ok(Self::new(vars, attributes)?) }
}
impl php_rt::PhpObject for Global_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Global_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\global_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_vars_get()) { out.push((Str::from_static("vars"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "vars" => { self.set_p_vars(cast::<Map<ArrayKey, crate::php_parser::node::Expr>>(value)); true }, _ => false } }
}
impl Global_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Global_ could not be converted to string"))) } }
impl php_rt::PhpClone for Global_ { fn php_clone(&self) -> Self { let c = Global_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Global_Obj { fn clone(&self) -> Self { Global_Obj { attributes: self.attributes.clone(), vars: self.vars.clone() } } }
impl Global_ {
}
pub struct Goto_Obj {
    pub attributes: Map<Str, Mixed>,
    pub name: Late<crate::php_parser::node::Identifier>,
}
#[derive(Clone)]
pub struct Goto_(pub Rc<RefCell<Goto_Obj>>);
impl Goto_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_name(&self) -> Ref<'_, crate::php_parser::node::Identifier> { Ref::map(self.0.borrow(), |o| o.name.get()) }
    pub fn p_name_get(&self) -> crate::php_parser::node::Identifier { self.0.borrow().name.get().clone() }
    pub fn p_name_opt(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().name.as_option().cloned() }
    pub fn p_name_mut(&self) -> RefMut<'_, crate::php_parser::node::Identifier> { RefMut::map(self.0.borrow_mut(), |o| o.name.get_mut()) }
    pub fn set_p_name(&self, v: crate::php_parser::node::Identifier) { self.0.borrow_mut().name.set(v); }
    pub fn new(mut name: U_PhpParser_Node_Identifier_or_Str, mut attributes: Map<Str, Mixed>) -> Result<Goto_, Throw> {
        let this = Goto_(Rc::new(RefCell::new(Goto_Obj {
            attributes: Default::default(),
            name: Late::uninit(),
        })));
        this.magic__construct(name, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_name((if (match name.clone() { U_PhpParser_Node_Identifier_or_Str::Str(_) => true, _ => false }) { crate::php_parser::node::Identifier::new(cast::<Str>(name.clone()), Map::<Str, Mixed>::new())? } else { cast::<crate::php_parser::node::Identifier>(name.clone()) }));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("name"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Goto"));
    }
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
    pub fn new_same_class(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Goto_, Throw> { Ok(Self::new(name, attributes)?) }
}
impl php_rt::PhpObject for Goto_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Goto_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\goto_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_name_opt() { out.push((Str::from_static("name"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(cast::<crate::php_parser::node::Identifier>(value)); true }, _ => false } }
}
impl Goto_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Goto_ could not be converted to string"))) } }
impl php_rt::PhpClone for Goto_ { fn php_clone(&self) -> Self { let c = Goto_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Goto_Obj { fn clone(&self) -> Self { Goto_Obj { attributes: self.attributes.clone(), name: self.name.clone() } } }
impl Goto_ {
}
pub struct GroupUseObj {
    pub attributes: Map<Str, Mixed>,
    pub type_: Mixed,
    pub prefix: Late<crate::php_parser::node::Name>,
    pub uses: Map<ArrayKey, crate::php_parser::node::UseItem>,
}
#[derive(Clone)]
pub struct GroupUse(pub Rc<RefCell<GroupUseObj>>);
impl GroupUse {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_type_(&self) -> Ref<'_, Mixed> { Ref::map(self.0.borrow(), |o| &o.type_) }
    pub fn p_type__get(&self) -> Mixed { self.0.borrow().type_.clone() }
    pub fn p_type__opt(&self) -> Option<Mixed> { Some(self.0.borrow().type_.clone()) }
    pub fn p_type__mut(&self) -> RefMut<'_, Mixed> { RefMut::map(self.0.borrow_mut(), |o| &mut o.type_) }
    pub fn set_p_type_(&self, v: Mixed) { self.0.borrow_mut().type_ = v; }
    pub fn p_prefix(&self) -> Ref<'_, crate::php_parser::node::Name> { Ref::map(self.0.borrow(), |o| o.prefix.get()) }
    pub fn p_prefix_get(&self) -> crate::php_parser::node::Name { self.0.borrow().prefix.get().clone() }
    pub fn p_prefix_opt(&self) -> Option<crate::php_parser::node::Name> { self.0.borrow().prefix.as_option().cloned() }
    pub fn p_prefix_mut(&self) -> RefMut<'_, crate::php_parser::node::Name> { RefMut::map(self.0.borrow_mut(), |o| o.prefix.get_mut()) }
    pub fn set_p_prefix(&self, v: crate::php_parser::node::Name) { self.0.borrow_mut().prefix.set(v); }
    pub fn p_uses(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::UseItem>> { Ref::map(self.0.borrow(), |o| &o.uses) }
    pub fn p_uses_get(&self) -> Map<ArrayKey, crate::php_parser::node::UseItem> { self.0.borrow().uses.clone() }
    pub fn p_uses_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::UseItem>> { Some(self.0.borrow().uses.clone()) }
    pub fn p_uses_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::UseItem>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.uses) }
    pub fn set_p_uses(&self, v: Map<ArrayKey, crate::php_parser::node::UseItem>) { self.0.borrow_mut().uses = v; }
    pub fn new(mut prefix: crate::php_parser::node::Name, mut uses: Map<ArrayKey, crate::php_parser::node::UseItem>, mut type_: Mixed, mut attributes: Map<Str, Mixed>) -> Result<GroupUse, Throw> {
        let this = GroupUse(Rc::new(RefCell::new(GroupUseObj {
            attributes: Default::default(),
            type_: Default::default(),
            prefix: Late::uninit(),
            uses: Default::default(),
        })));
        this.magic__construct(prefix, uses, type_, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut prefix: crate::php_parser::node::Name, mut uses: Map<ArrayKey, crate::php_parser::node::UseItem>, mut type_: Mixed, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_type_(cast::<Mixed>(cast::<i64>(type_.clone())));
    self.set_p_prefix(prefix.clone());
    self.set_p_uses(uses.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("type"))); __m1.push(cast::<Mixed>(Str::from_static("prefix"))); __m1.push(cast::<Mixed>(Str::from_static("uses"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_GroupUse"));
    }
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
    pub fn new_same_class(&self, mut prefix: crate::php_parser::node::Name, mut uses: Map<ArrayKey, crate::php_parser::node::UseItem>, mut type_: Mixed, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::GroupUse, Throw> { Ok(Self::new(prefix, uses, type_, attributes)?) }
}
impl php_rt::PhpObject for GroupUse {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\GroupUse" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\groupuse", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_type__get()) { out.push((Str::from_static("type"), v)); } if let Some(v) = self.p_prefix_opt() { out.push((Str::from_static("prefix"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_uses_get()) { out.push((Str::from_static("uses"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "type" => { self.set_p_type_(value); true }, "prefix" => { self.set_p_prefix(cast::<crate::php_parser::node::Name>(value)); true }, "uses" => { self.set_p_uses(cast::<Map<ArrayKey, crate::php_parser::node::UseItem>>(value)); true }, _ => false } }
}
impl GroupUse { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\GroupUse could not be converted to string"))) } }
impl php_rt::PhpClone for GroupUse { fn php_clone(&self) -> Self { let c = GroupUse(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for GroupUseObj { fn clone(&self) -> Self { GroupUseObj { attributes: self.attributes.clone(), type_: self.type_.clone(), prefix: self.prefix.clone(), uses: self.uses.clone() } } }
impl GroupUse {
}
pub struct HaltCompilerObj {
    pub attributes: Map<Str, Mixed>,
    pub remaining: Str,
}
#[derive(Clone)]
pub struct HaltCompiler(pub Rc<RefCell<HaltCompilerObj>>);
impl HaltCompiler {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_remaining(&self) -> Ref<'_, Str> { Ref::map(self.0.borrow(), |o| &o.remaining) }
    pub fn p_remaining_get(&self) -> Str { self.0.borrow().remaining.clone() }
    pub fn p_remaining_opt(&self) -> Option<Str> { Some(self.0.borrow().remaining.clone()) }
    pub fn p_remaining_mut(&self) -> RefMut<'_, Str> { RefMut::map(self.0.borrow_mut(), |o| &mut o.remaining) }
    pub fn set_p_remaining(&self, v: Str) { self.0.borrow_mut().remaining = v; }
    pub fn new(mut remaining: Str, mut attributes: Map<Str, Mixed>) -> Result<HaltCompiler, Throw> {
        let this = HaltCompiler(Rc::new(RefCell::new(HaltCompilerObj {
            attributes: Default::default(),
            remaining: Default::default(),
        })));
        this.magic__construct(remaining, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut remaining: Str, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_remaining(remaining.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("remaining"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_HaltCompiler"));
    }
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
    pub fn new_same_class(&self, mut remaining: Str, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::HaltCompiler, Throw> { Ok(Self::new(remaining, attributes)?) }
}
impl php_rt::PhpObject for HaltCompiler {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\HaltCompiler" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\haltcompiler", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_remaining_get()) { out.push((Str::from_static("remaining"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "remaining" => { self.set_p_remaining(cast::<Str>(value)); true }, _ => false } }
}
impl HaltCompiler { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\HaltCompiler could not be converted to string"))) } }
impl php_rt::PhpClone for HaltCompiler { fn php_clone(&self) -> Self { let c = HaltCompiler(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for HaltCompilerObj { fn clone(&self) -> Self { HaltCompilerObj { attributes: self.attributes.clone(), remaining: self.remaining.clone() } } }
impl HaltCompiler {
}
pub struct If_Obj {
    pub attributes: Map<Str, Mixed>,
    pub cond: Late<crate::php_parser::node::Expr>,
    pub stmts: List<crate::php_parser::node::Stmt>,
    pub elseifs: Map<ArrayKey, crate::php_parser::node::stmt::ElseIf_>,
    pub else_: Option<crate::php_parser::node::stmt::Else_>,
}
#[derive(Clone)]
pub struct If_(pub Rc<RefCell<If_Obj>>);
impl If_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_cond(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.cond.get()) }
    pub fn p_cond_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().cond.get().clone() }
    pub fn p_cond_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().cond.as_option().cloned() }
    pub fn p_cond_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.cond.get_mut()) }
    pub fn set_p_cond(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().cond.set(v); }
    pub fn p_stmts(&self) -> Ref<'_, List<crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> List<crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<List<crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: List<crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn p_elseifs(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::stmt::ElseIf_>> { Ref::map(self.0.borrow(), |o| &o.elseifs) }
    pub fn p_elseifs_get(&self) -> Map<ArrayKey, crate::php_parser::node::stmt::ElseIf_> { self.0.borrow().elseifs.clone() }
    pub fn p_elseifs_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::stmt::ElseIf_>> { Some(self.0.borrow().elseifs.clone()) }
    pub fn p_elseifs_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::stmt::ElseIf_>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.elseifs) }
    pub fn set_p_elseifs(&self, v: Map<ArrayKey, crate::php_parser::node::stmt::ElseIf_>) { self.0.borrow_mut().elseifs = v; }
    pub fn p_else_(&self) -> Ref<'_, Option<crate::php_parser::node::stmt::Else_>> { Ref::map(self.0.borrow(), |o| &o.else_) }
    pub fn p_else__get(&self) -> Option<crate::php_parser::node::stmt::Else_> { self.0.borrow().else_.clone() }
    pub fn p_else__opt(&self) -> Option<Option<crate::php_parser::node::stmt::Else_>> { Some(self.0.borrow().else_.clone()) }
    pub fn p_else__mut(&self) -> RefMut<'_, Option<crate::php_parser::node::stmt::Else_>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.else_) }
    pub fn set_p_else_(&self, v: Option<crate::php_parser::node::stmt::Else_>) { self.0.borrow_mut().else_ = v; }
    pub fn new(mut cond: crate::php_parser::node::Expr, mut subNodes: Shape_else_q_Opt_PhpParser_Node_Stmt_Else__elseifsq_Map_ArrayKey_PhpPa_600b493e60, mut attributes: Map<Str, Mixed>) -> Result<If_, Throw> {
        let this = If_(Rc::new(RefCell::new(If_Obj {
            attributes: Default::default(),
            cond: Late::uninit(),
            stmts: Default::default(),
            elseifs: Default::default(),
            else_: Default::default(),
        })));
        this.magic__construct(cond, subNodes, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut cond: crate::php_parser::node::Expr, mut subNodes: Shape_else_q_Opt_PhpParser_Node_Stmt_Else__elseifsq_Map_ArrayKey_PhpPa_600b493e60, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_cond(cond.clone());
    self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>((match Some({ let __c89 = subNodes.clone(); Shape_else_q_Opt_PhpParser_Node_Stmt_Else__elseifsq_Map_ArrayKey_PhpPa_7d8dd66346 { else_: __c89.else_, elseifs: __c89.elseifs, stmts: __c89.stmts.unwrap() } }).and_then(|__b| Some(__b.stmts)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Stmt>::new() })));
    self.set_p_elseifs((match Some({ let __c90 = subNodes.clone(); Shape_else_q_Opt_PhpParser_Node_Stmt_Else__elseifs_Map_ArrayKey_PhpPar_c066fd6c75 { else_: __c90.else_, elseifs: __c90.elseifs.unwrap(), stmts: __c90.stmts } }).and_then(|__b| Some(__b.elseifs)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::stmt::ElseIf_>::new() }));
    self.set_p_else_((match Some({ let __c91 = subNodes.clone(); Shape_else__PhpParser_Node_Stmt_Else__elseifsq_Map_ArrayKey_PhpParser__5c81eadc3d { else_: __c91.else_.unwrap(), elseifs: __c91.elseifs, stmts: __c91.stmts } }).and_then(|__b| Some(__b.else_)) { Some(__v) => Some(__v), None => { let _ = (); None::<crate::php_parser::node::stmt::Else_> } }));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("cond"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1.push(cast::<Mixed>(Str::from_static("elseifs"))); __m1.push(cast::<Mixed>(Str::from_static("else"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_If"));
    }
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
    pub fn new_same_class(&self, mut cond: crate::php_parser::node::Expr, mut subNodes: Shape_else_q_Opt_PhpParser_Node_Stmt_Else__elseifsq_Map_ArrayKey_PhpPa_600b493e60, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::If_, Throw> { Ok(Self::new(cond, subNodes, attributes)?) }
}
impl php_rt::PhpObject for If_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\If_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\if_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_cond_opt() { out.push((Str::from_static("cond"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_elseifs_get()) { out.push((Str::from_static("elseifs"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_else__get()) { out.push((Str::from_static("else"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "cond" => { self.set_p_cond(cast::<crate::php_parser::node::Expr>(value)); true }, "stmts" => { self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(value)); true }, "elseifs" => { self.set_p_elseifs(cast::<Map<ArrayKey, crate::php_parser::node::stmt::ElseIf_>>(value)); true }, "else" => { self.set_p_else_(value.to_option().map(|__m| cast::<crate::php_parser::node::stmt::Else_>(__m))); true }, _ => false } }
}
impl If_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\If_ could not be converted to string"))) } }
impl php_rt::PhpClone for If_ { fn php_clone(&self) -> Self { let c = If_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for If_Obj { fn clone(&self) -> Self { If_Obj { attributes: self.attributes.clone(), cond: self.cond.clone(), stmts: self.stmts.clone(), elseifs: self.elseifs.clone(), else_: self.else_.clone() } } }
impl If_ {
}
pub struct InlineHTMLObj {
    pub attributes: Map<Str, Mixed>,
    pub value: Str,
}
#[derive(Clone)]
pub struct InlineHTML(pub Rc<RefCell<InlineHTMLObj>>);
impl InlineHTML {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_value(&self) -> Ref<'_, Str> { Ref::map(self.0.borrow(), |o| &o.value) }
    pub fn p_value_get(&self) -> Str { self.0.borrow().value.clone() }
    pub fn p_value_opt(&self) -> Option<Str> { Some(self.0.borrow().value.clone()) }
    pub fn p_value_mut(&self) -> RefMut<'_, Str> { RefMut::map(self.0.borrow_mut(), |o| &mut o.value) }
    pub fn set_p_value(&self, v: Str) { self.0.borrow_mut().value = v; }
    pub fn new(mut value: Str, mut attributes: Map<Str, Mixed>) -> Result<InlineHTML, Throw> {
        let this = InlineHTML(Rc::new(RefCell::new(InlineHTMLObj {
            attributes: Default::default(),
            value: Default::default(),
        })));
        this.magic__construct(value, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut value: Str, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_value(value.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("value"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_InlineHTML"));
    }
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
    pub fn new_same_class(&self, mut value: Str, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::InlineHTML, Throw> { Ok(Self::new(value, attributes)?) }
}
impl php_rt::PhpObject for InlineHTML {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\InlineHTML" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\inlinehtml", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_value_get()) { out.push((Str::from_static("value"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "value" => { self.set_p_value(cast::<Str>(value)); true }, _ => false } }
}
impl InlineHTML { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\InlineHTML could not be converted to string"))) } }
impl php_rt::PhpClone for InlineHTML { fn php_clone(&self) -> Self { let c = InlineHTML(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for InlineHTMLObj { fn clone(&self) -> Self { InlineHTMLObj { attributes: self.attributes.clone(), value: self.value.clone() } } }
impl InlineHTML {
}
pub struct Interface_Obj {
    pub attributes: Map<Str, Mixed>,
    pub name: Option<crate::php_parser::node::Identifier>,
    pub stmts: Map<ArrayKey, crate::php_parser::node::Stmt>,
    pub attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>,
    pub namespacedName: Option<crate::php_parser::node::Name>,
    pub extends: Map<ArrayKey, crate::php_parser::node::Name>,
}
#[derive(Clone)]
pub struct Interface_(pub Rc<RefCell<Interface_Obj>>);
impl Interface_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_name(&self) -> Ref<'_, Option<crate::php_parser::node::Identifier>> { Ref::map(self.0.borrow(), |o| &o.name) }
    pub fn p_name_get(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().name.clone() }
    pub fn p_name_opt(&self) -> Option<Option<crate::php_parser::node::Identifier>> { Some(self.0.borrow().name.clone()) }
    pub fn p_name_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Identifier>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.name) }
    pub fn set_p_name(&self, v: Option<crate::php_parser::node::Identifier>) { self.0.borrow_mut().name = v; }
    pub fn p_stmts(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> Map<ArrayKey, crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: Map<ArrayKey, crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn p_attrGroups(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attrGroups) }
    pub fn p_attrGroups_get(&self) -> Map<ArrayKey, crate::php_parser::node::AttributeGroup> { self.0.borrow().attrGroups.clone() }
    pub fn p_attrGroups_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attrGroups.clone()) }
    pub fn p_attrGroups_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attrGroups) }
    pub fn set_p_attrGroups(&self, v: Map<ArrayKey, crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attrGroups = v; }
    pub fn p_namespacedName(&self) -> Ref<'_, Option<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.namespacedName) }
    pub fn p_namespacedName_get(&self) -> Option<crate::php_parser::node::Name> { self.0.borrow().namespacedName.clone() }
    pub fn p_namespacedName_opt(&self) -> Option<Option<crate::php_parser::node::Name>> { Some(self.0.borrow().namespacedName.clone()) }
    pub fn p_namespacedName_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.namespacedName) }
    pub fn set_p_namespacedName(&self, v: Option<crate::php_parser::node::Name>) { self.0.borrow_mut().namespacedName = v; }
    pub fn p_extends(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.extends) }
    pub fn p_extends_get(&self) -> Map<ArrayKey, crate::php_parser::node::Name> { self.0.borrow().extends.clone() }
    pub fn p_extends_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Name>> { Some(self.0.borrow().extends.clone()) }
    pub fn p_extends_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.extends) }
    pub fn set_p_extends(&self, v: Map<ArrayKey, crate::php_parser::node::Name>) { self.0.borrow_mut().extends = v; }
    pub fn new(mut name: U_PhpParser_Node_Identifier_or_Str, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__786d9d6046, mut attributes: Map<Str, Mixed>) -> Result<Interface_, Throw> {
        let this = Interface_(Rc::new(RefCell::new(Interface_Obj {
            attributes: Default::default(),
            name: Default::default(),
            stmts: Default::default(),
            attrGroups: Default::default(),
            namespacedName: Default::default(),
            extends: Default::default(),
        })));
        this.magic__construct(name, subNodes, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__786d9d6046, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_name(Some((if (match name.clone() { U_PhpParser_Node_Identifier_or_Str::Str(_) => true, _ => false }) { crate::php_parser::node::Identifier::new(cast::<Str>(name.clone()), Map::<Str, Mixed>::new())? } else { cast::<crate::php_parser::node::Identifier>(name.clone()) })));
    self.set_p_extends((match Some({ let __c92 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extends_M_cdbdbd89e4 { attrGroups: __c92.attrGroups, extends: __c92.extends.unwrap(), stmts: __c92.stmts } }).and_then(|__b| Some(__b.extends)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Name>::new() }));
    self.set_p_stmts((match Some({ let __c93 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__d66c45bdc1 { attrGroups: __c93.attrGroups, extends: __c93.extends, stmts: __c93.stmts.unwrap() } }).and_then(|__b| Some(__b.stmts)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Stmt>::new() }));
    self.set_p_attrGroups((match Some({ let __c94 = subNodes.clone(); Shape_attrGroups_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq_M_0aeb1e38fb { attrGroups: __c94.attrGroups.unwrap(), extends: __c94.extends, stmts: __c94.stmts } }).and_then(|__b| Some(__b.attrGroups)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::AttributeGroup>::new() }));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("attrGroups"))); __m1.push(cast::<Mixed>(Str::from_static("name"))); __m1.push(cast::<Mixed>(Str::from_static("extends"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Interface"));
    }
    pub fn getTraitUses(&self) -> Result<List<crate::php_parser::node::stmt::TraitUse>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getTraitUses__impl() }
    pub fn getConstants(&self) -> Result<List<crate::php_parser::node::stmt::ClassConst>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getConstants__impl() }
    pub fn getProperties(&self) -> Result<List<crate::php_parser::node::stmt::Property>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getProperties__impl() }
    pub fn getProperty(&self, mut name: Str) -> Result<Option<crate::php_parser::node::stmt::Property>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getProperty__impl(name) }
    pub fn getMethods(&self) -> Result<List<crate::php_parser::node::stmt::ClassMethod>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getMethods__impl() }
    pub fn getMethod(&self, mut name: Str) -> Result<Option<crate::php_parser::node::stmt::ClassMethod>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getMethod__impl(name) }
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
    pub fn new_same_class(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__786d9d6046, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Interface_, Throw> { Ok(Self::new(name, subNodes, attributes)?) }
}
impl php_rt::PhpObject for Interface_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Interface_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\interface_", "phpparser\\node\\stmt\\classlike", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attrGroups_get()) { out.push((Str::from_static("attrGroups"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_namespacedName_get()) { out.push((Str::from_static("namespacedName"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_extends_get()) { out.push((Str::from_static("extends"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(value.to_option().map(|__m| cast::<crate::php_parser::node::Identifier>(__m))); true }, "stmts" => { self.set_p_stmts(cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(value)); true }, "attrGroups" => { self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(value)); true }, "namespacedName" => { self.set_p_namespacedName(value.to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m))); true }, "extends" => { self.set_p_extends(cast::<Map<ArrayKey, crate::php_parser::node::Name>>(value)); true }, _ => false } }
}
impl Interface_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Interface_ could not be converted to string"))) } }
impl php_rt::PhpClone for Interface_ { fn php_clone(&self) -> Self { let c = Interface_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Interface_Obj { fn clone(&self) -> Self { Interface_Obj { attributes: self.attributes.clone(), name: self.name.clone(), stmts: self.stmts.clone(), attrGroups: self.attrGroups.clone(), namespacedName: self.namespacedName.clone(), extends: self.extends.clone() } } }
impl Interface_ {
}
pub struct LabelObj {
    pub attributes: Map<Str, Mixed>,
    pub name: Late<crate::php_parser::node::Identifier>,
}
#[derive(Clone)]
pub struct Label(pub Rc<RefCell<LabelObj>>);
impl Label {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_name(&self) -> Ref<'_, crate::php_parser::node::Identifier> { Ref::map(self.0.borrow(), |o| o.name.get()) }
    pub fn p_name_get(&self) -> crate::php_parser::node::Identifier { self.0.borrow().name.get().clone() }
    pub fn p_name_opt(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().name.as_option().cloned() }
    pub fn p_name_mut(&self) -> RefMut<'_, crate::php_parser::node::Identifier> { RefMut::map(self.0.borrow_mut(), |o| o.name.get_mut()) }
    pub fn set_p_name(&self, v: crate::php_parser::node::Identifier) { self.0.borrow_mut().name.set(v); }
    pub fn new(mut name: U_PhpParser_Node_Identifier_or_Str, mut attributes: Map<Str, Mixed>) -> Result<Label, Throw> {
        let this = Label(Rc::new(RefCell::new(LabelObj {
            attributes: Default::default(),
            name: Late::uninit(),
        })));
        this.magic__construct(name, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_name((if (match name.clone() { U_PhpParser_Node_Identifier_or_Str::Str(_) => true, _ => false }) { crate::php_parser::node::Identifier::new(cast::<Str>(name.clone()), Map::<Str, Mixed>::new())? } else { cast::<crate::php_parser::node::Identifier>(name.clone()) }));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("name"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Label"));
    }
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
    pub fn new_same_class(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Label, Throw> { Ok(Self::new(name, attributes)?) }
}
impl php_rt::PhpObject for Label {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Label" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\label", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_name_opt() { out.push((Str::from_static("name"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(cast::<crate::php_parser::node::Identifier>(value)); true }, _ => false } }
}
impl Label { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Label could not be converted to string"))) } }
impl php_rt::PhpClone for Label { fn php_clone(&self) -> Self { let c = Label(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for LabelObj { fn clone(&self) -> Self { LabelObj { attributes: self.attributes.clone(), name: self.name.clone() } } }
impl Label {
}
pub struct Namespace_Obj {
    pub attributes: Map<Str, Mixed>,
    pub name: Option<crate::php_parser::node::Name>,
    pub stmts: List<crate::php_parser::node::Stmt>,
}
#[derive(Clone)]
pub struct Namespace_(pub Rc<RefCell<Namespace_Obj>>);
impl Namespace_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_name(&self) -> Ref<'_, Option<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.name) }
    pub fn p_name_get(&self) -> Option<crate::php_parser::node::Name> { self.0.borrow().name.clone() }
    pub fn p_name_opt(&self) -> Option<Option<crate::php_parser::node::Name>> { Some(self.0.borrow().name.clone()) }
    pub fn p_name_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.name) }
    pub fn set_p_name(&self, v: Option<crate::php_parser::node::Name>) { self.0.borrow_mut().name = v; }
    pub fn p_stmts(&self) -> Ref<'_, List<crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> List<crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<List<crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: List<crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn new(mut name: Option<crate::php_parser::node::Name>, mut stmts: Option<Map<ArrayKey, crate::php_parser::node::Stmt>>, mut attributes: Map<Str, Mixed>) -> Result<Namespace_, Throw> {
        let this = Namespace_(Rc::new(RefCell::new(Namespace_Obj {
            attributes: Default::default(),
            name: Default::default(),
            stmts: Default::default(),
        })));
        this.magic__construct(name, stmts, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: Option<crate::php_parser::node::Name>, mut stmts: Option<Map<ArrayKey, crate::php_parser::node::Stmt>>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_name(name.clone());
    self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(stmts.clone().unwrap()));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("name"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Namespace"));
    }
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
    pub fn new_same_class(&self, mut name: Option<crate::php_parser::node::Name>, mut stmts: Option<Map<ArrayKey, crate::php_parser::node::Stmt>>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Namespace_, Throw> { Ok(Self::new(name, stmts, attributes)?) }
}
impl php_rt::PhpObject for Namespace_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Namespace_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\namespace_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(value.to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m))); true }, "stmts" => { self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(value)); true }, _ => false } }
}
impl Namespace_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Namespace_ could not be converted to string"))) } }
impl php_rt::PhpClone for Namespace_ { fn php_clone(&self) -> Self { let c = Namespace_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Namespace_Obj { fn clone(&self) -> Self { Namespace_Obj { attributes: self.attributes.clone(), name: self.name.clone(), stmts: self.stmts.clone() } } }
impl Namespace_ {
    pub fn KIND_SEMICOLON() -> i64 { 1i64 }
    pub fn KIND_BRACED() -> i64 { 2i64 }
}
pub struct NopObj {
    pub attributes: Map<Str, Mixed>,
}
#[derive(Clone)]
pub struct Nop(pub Rc<RefCell<NopObj>>);
impl Nop {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn new(mut attributes: Map<Str, Mixed>) -> Result<Nop, Throw> {
        let this = Nop(Rc::new(RefCell::new(NopObj {
            attributes: Default::default(),
        })));
        this.magic__construct(attributes)?;
        Ok(this)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok(Map::<ArrayKey, Mixed>::new());
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Nop"));
    }
    pub fn magic__construct(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).magic__construct__impl(attributes) }
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
    pub fn new_same_class(&self, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Nop, Throw> { Ok(Self::new(attributes)?) }
}
impl php_rt::PhpObject for Nop {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Nop" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\nop", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, _ => false } }
}
impl Nop { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Nop could not be converted to string"))) } }
impl php_rt::PhpClone for Nop { fn php_clone(&self) -> Self { let c = Nop(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for NopObj { fn clone(&self) -> Self { NopObj { attributes: self.attributes.clone() } } }
impl Nop {
}
pub struct PropertyObj {
    pub attributes: Map<Str, Mixed>,
    pub flags: i64,
    pub props: Map<ArrayKey, crate::php_parser::node::PropertyItem>,
    pub type_: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>,
    pub attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>,
    pub hooks: Map<ArrayKey, crate::php_parser::node::PropertyHook>,
}
#[derive(Clone)]
pub struct Property(pub Rc<RefCell<PropertyObj>>);
impl Property {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_flags(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.flags) }
    pub fn p_flags_get(&self) -> i64 { self.0.borrow().flags.clone() }
    pub fn p_flags_opt(&self) -> Option<i64> { Some(self.0.borrow().flags.clone()) }
    pub fn p_flags_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.flags) }
    pub fn set_p_flags(&self, v: i64) { self.0.borrow_mut().flags = v; }
    pub fn p_props(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::PropertyItem>> { Ref::map(self.0.borrow(), |o| &o.props) }
    pub fn p_props_get(&self) -> Map<ArrayKey, crate::php_parser::node::PropertyItem> { self.0.borrow().props.clone() }
    pub fn p_props_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::PropertyItem>> { Some(self.0.borrow().props.clone()) }
    pub fn p_props_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::PropertyItem>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.props) }
    pub fn set_p_props(&self, v: Map<ArrayKey, crate::php_parser::node::PropertyItem>) { self.0.borrow_mut().props = v; }
    pub fn p_type_(&self) -> Ref<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { Ref::map(self.0.borrow(), |o| &o.type_) }
    pub fn p_type__get(&self) -> Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> { self.0.borrow().type_.clone() }
    pub fn p_type__opt(&self) -> Option<Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { Some(self.0.borrow().type_.clone()) }
    pub fn p_type__mut(&self) -> RefMut<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.type_) }
    pub fn set_p_type_(&self, v: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>) { self.0.borrow_mut().type_ = v; }
    pub fn p_attrGroups(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attrGroups) }
    pub fn p_attrGroups_get(&self) -> Map<ArrayKey, crate::php_parser::node::AttributeGroup> { self.0.borrow().attrGroups.clone() }
    pub fn p_attrGroups_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attrGroups.clone()) }
    pub fn p_attrGroups_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attrGroups) }
    pub fn set_p_attrGroups(&self, v: Map<ArrayKey, crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attrGroups = v; }
    pub fn p_hooks(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::PropertyHook>> { Ref::map(self.0.borrow(), |o| &o.hooks) }
    pub fn p_hooks_get(&self) -> Map<ArrayKey, crate::php_parser::node::PropertyHook> { self.0.borrow().hooks.clone() }
    pub fn p_hooks_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::PropertyHook>> { Some(self.0.borrow().hooks.clone()) }
    pub fn p_hooks_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::PropertyHook>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.hooks) }
    pub fn set_p_hooks(&self, v: Map<ArrayKey, crate::php_parser::node::PropertyHook>) { self.0.borrow_mut().hooks = v; }
    pub fn new(mut flags: i64, mut props: Map<ArrayKey, crate::php_parser::node::PropertyItem>, mut attributes: Map<Str, Mixed>, mut type_: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>, mut attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>, mut hooks: Map<ArrayKey, crate::php_parser::node::PropertyHook>) -> Result<Property, Throw> {
        let this = Property(Rc::new(RefCell::new(PropertyObj {
            attributes: Default::default(),
            flags: Default::default(),
            props: Default::default(),
            type_: Default::default(),
            attrGroups: Default::default(),
            hooks: Default::default(),
        })));
        this.magic__construct(flags, props, attributes, type_, attrGroups, hooks)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut flags: i64, mut props: Map<ArrayKey, crate::php_parser::node::PropertyItem>, mut attributes: Map<Str, Mixed>, mut type_: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>, mut attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>, mut hooks: Map<ArrayKey, crate::php_parser::node::PropertyHook>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_flags(flags);
    self.set_p_props(props.clone());
    self.set_p_type_(type_.clone());
    self.set_p_attrGroups(attrGroups.clone());
    self.set_p_hooks(hooks.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("attrGroups"))); __m1.push(cast::<Mixed>(Str::from_static("flags"))); __m1.push(cast::<Mixed>(Str::from_static("type"))); __m1.push(cast::<Mixed>(Str::from_static("props"))); __m1.push(cast::<Mixed>(Str::from_static("hooks"))); __m1 });
    }
    pub fn isPublic(&self) -> Result<bool, Throw> {
    return Ok(((!((self.p_flags_get() & crate::php_parser::Modifiers::PUBLIC()) == 0i64)) || ((self.p_flags_get() & crate::php_parser::Modifiers::VISIBILITY_MASK()) == 0i64)));
    }
    pub fn isProtected(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::PROTECTED())));
    }
    pub fn isPrivate(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::PRIVATE())));
    }
    pub fn isStatic(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::STATIC())));
    }
    pub fn isReadonly(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::READONLY())));
    }
    pub fn isAbstract(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::ABSTRACT())));
    }
    pub fn isFinal(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::FINAL())));
    }
    pub fn isPublicSet(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::PUBLIC_SET())));
    }
    pub fn isProtectedSet(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::PROTECTED_SET())));
    }
    pub fn isPrivateSet(&self) -> Result<bool, Throw> {
    return Ok(truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::PRIVATE_SET())));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Property"));
    }
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
    pub fn new_same_class(&self, mut flags: i64, mut props: Map<ArrayKey, crate::php_parser::node::PropertyItem>, mut attributes: Map<Str, Mixed>, mut type_: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>, mut attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>, mut hooks: Map<ArrayKey, crate::php_parser::node::PropertyHook>) -> Result<crate::php_parser::node::stmt::Property, Throw> { Ok(Self::new(flags, props, attributes, type_, attrGroups, hooks)?) }
}
impl php_rt::PhpObject for Property {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Property" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\property", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_flags_get()) { out.push((Str::from_static("flags"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_props_get()) { out.push((Str::from_static("props"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_type__get()) { out.push((Str::from_static("type"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attrGroups_get()) { out.push((Str::from_static("attrGroups"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_hooks_get()) { out.push((Str::from_static("hooks"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "flags" => { self.set_p_flags(cast::<i64>(value)); true }, "props" => { self.set_p_props(cast::<Map<ArrayKey, crate::php_parser::node::PropertyItem>>(value)); true }, "type" => { self.set_p_type_(value.to_option().map(|__m| cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(__m))); true }, "attrGroups" => { self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(value)); true }, "hooks" => { self.set_p_hooks(cast::<Map<ArrayKey, crate::php_parser::node::PropertyHook>>(value)); true }, _ => false } }
}
impl Property { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Property could not be converted to string"))) } }
impl php_rt::PhpClone for Property { fn php_clone(&self) -> Self { let c = Property(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for PropertyObj { fn clone(&self) -> Self { PropertyObj { attributes: self.attributes.clone(), flags: self.flags.clone(), props: self.props.clone(), type_: self.type_.clone(), attrGroups: self.attrGroups.clone(), hooks: self.hooks.clone() } } }
impl Property {
}
pub struct Return_Obj {
    pub attributes: Map<Str, Mixed>,
    pub expr: Option<crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Return_(pub Rc<RefCell<Return_Obj>>);
impl Return_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_expr(&self) -> Ref<'_, Option<crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.expr) }
    pub fn p_expr_get(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().expr.clone() }
    pub fn p_expr_opt(&self) -> Option<Option<crate::php_parser::node::Expr>> { Some(self.0.borrow().expr.clone()) }
    pub fn p_expr_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.expr) }
    pub fn set_p_expr(&self, v: Option<crate::php_parser::node::Expr>) { self.0.borrow_mut().expr = v; }
    pub fn new(mut expr: Option<crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<Return_, Throw> {
        let this = Return_(Rc::new(RefCell::new(Return_Obj {
            attributes: Default::default(),
            expr: Default::default(),
        })));
        this.magic__construct(expr, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut expr: Option<crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_expr(expr.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("expr"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Return"));
    }
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
    pub fn new_same_class(&self, mut expr: Option<crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Return_, Throw> { Ok(Self::new(expr, attributes)?) }
}
impl php_rt::PhpObject for Return_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Return_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\return_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_expr_get()) { out.push((Str::from_static("expr"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "expr" => { self.set_p_expr(value.to_option().map(|__m| cast::<crate::php_parser::node::Expr>(__m))); true }, _ => false } }
}
impl Return_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Return_ could not be converted to string"))) } }
impl php_rt::PhpClone for Return_ { fn php_clone(&self) -> Self { let c = Return_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Return_Obj { fn clone(&self) -> Self { Return_Obj { attributes: self.attributes.clone(), expr: self.expr.clone() } } }
impl Return_ {
}
pub struct Static_Obj {
    pub attributes: Map<Str, Mixed>,
    pub vars: Map<ArrayKey, crate::php_parser::node::StaticVar>,
}
#[derive(Clone)]
pub struct Static_(pub Rc<RefCell<Static_Obj>>);
impl Static_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_vars(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::StaticVar>> { Ref::map(self.0.borrow(), |o| &o.vars) }
    pub fn p_vars_get(&self) -> Map<ArrayKey, crate::php_parser::node::StaticVar> { self.0.borrow().vars.clone() }
    pub fn p_vars_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::StaticVar>> { Some(self.0.borrow().vars.clone()) }
    pub fn p_vars_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::StaticVar>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.vars) }
    pub fn set_p_vars(&self, v: Map<ArrayKey, crate::php_parser::node::StaticVar>) { self.0.borrow_mut().vars = v; }
    pub fn new(mut vars: Map<ArrayKey, crate::php_parser::node::StaticVar>, mut attributes: Map<Str, Mixed>) -> Result<Static_, Throw> {
        let this = Static_(Rc::new(RefCell::new(Static_Obj {
            attributes: Default::default(),
            vars: Default::default(),
        })));
        this.magic__construct(vars, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut vars: Map<ArrayKey, crate::php_parser::node::StaticVar>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_vars(vars.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("vars"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Static"));
    }
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
    pub fn new_same_class(&self, mut vars: Map<ArrayKey, crate::php_parser::node::StaticVar>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Static_, Throw> { Ok(Self::new(vars, attributes)?) }
}
impl php_rt::PhpObject for Static_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Static_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\static_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_vars_get()) { out.push((Str::from_static("vars"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "vars" => { self.set_p_vars(cast::<Map<ArrayKey, crate::php_parser::node::StaticVar>>(value)); true }, _ => false } }
}
impl Static_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Static_ could not be converted to string"))) } }
impl php_rt::PhpClone for Static_ { fn php_clone(&self) -> Self { let c = Static_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Static_Obj { fn clone(&self) -> Self { Static_Obj { attributes: self.attributes.clone(), vars: self.vars.clone() } } }
impl Static_ {
}
pub struct Switch_Obj {
    pub attributes: Map<Str, Mixed>,
    pub cond: Late<crate::php_parser::node::Expr>,
    pub cases: Map<ArrayKey, crate::php_parser::node::stmt::Case_>,
}
#[derive(Clone)]
pub struct Switch_(pub Rc<RefCell<Switch_Obj>>);
impl Switch_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_cond(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.cond.get()) }
    pub fn p_cond_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().cond.get().clone() }
    pub fn p_cond_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().cond.as_option().cloned() }
    pub fn p_cond_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.cond.get_mut()) }
    pub fn set_p_cond(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().cond.set(v); }
    pub fn p_cases(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::stmt::Case_>> { Ref::map(self.0.borrow(), |o| &o.cases) }
    pub fn p_cases_get(&self) -> Map<ArrayKey, crate::php_parser::node::stmt::Case_> { self.0.borrow().cases.clone() }
    pub fn p_cases_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::stmt::Case_>> { Some(self.0.borrow().cases.clone()) }
    pub fn p_cases_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::stmt::Case_>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.cases) }
    pub fn set_p_cases(&self, v: Map<ArrayKey, crate::php_parser::node::stmt::Case_>) { self.0.borrow_mut().cases = v; }
    pub fn new(mut cond: crate::php_parser::node::Expr, mut cases: Map<ArrayKey, crate::php_parser::node::stmt::Case_>, mut attributes: Map<Str, Mixed>) -> Result<Switch_, Throw> {
        let this = Switch_(Rc::new(RefCell::new(Switch_Obj {
            attributes: Default::default(),
            cond: Late::uninit(),
            cases: Default::default(),
        })));
        this.magic__construct(cond, cases, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut cond: crate::php_parser::node::Expr, mut cases: Map<ArrayKey, crate::php_parser::node::stmt::Case_>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_cond(cond.clone());
    self.set_p_cases(cases.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("cond"))); __m1.push(cast::<Mixed>(Str::from_static("cases"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Switch"));
    }
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
    pub fn new_same_class(&self, mut cond: crate::php_parser::node::Expr, mut cases: Map<ArrayKey, crate::php_parser::node::stmt::Case_>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Switch_, Throw> { Ok(Self::new(cond, cases, attributes)?) }
}
impl php_rt::PhpObject for Switch_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Switch_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\switch_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_cond_opt() { out.push((Str::from_static("cond"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_cases_get()) { out.push((Str::from_static("cases"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "cond" => { self.set_p_cond(cast::<crate::php_parser::node::Expr>(value)); true }, "cases" => { self.set_p_cases(cast::<Map<ArrayKey, crate::php_parser::node::stmt::Case_>>(value)); true }, _ => false } }
}
impl Switch_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Switch_ could not be converted to string"))) } }
impl php_rt::PhpClone for Switch_ { fn php_clone(&self) -> Self { let c = Switch_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Switch_Obj { fn clone(&self) -> Self { Switch_Obj { attributes: self.attributes.clone(), cond: self.cond.clone(), cases: self.cases.clone() } } }
impl Switch_ {
}
pub struct TraitUseObj {
    pub attributes: Map<Str, Mixed>,
    pub traits: Map<ArrayKey, crate::php_parser::node::Name>,
    pub adaptations: Map<ArrayKey, crate::php_parser::node::stmt::TraitUseAdaptation>,
}
#[derive(Clone)]
pub struct TraitUse(pub Rc<RefCell<TraitUseObj>>);
impl TraitUse {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_traits(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.traits) }
    pub fn p_traits_get(&self) -> Map<ArrayKey, crate::php_parser::node::Name> { self.0.borrow().traits.clone() }
    pub fn p_traits_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Name>> { Some(self.0.borrow().traits.clone()) }
    pub fn p_traits_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.traits) }
    pub fn set_p_traits(&self, v: Map<ArrayKey, crate::php_parser::node::Name>) { self.0.borrow_mut().traits = v; }
    pub fn p_adaptations(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::stmt::TraitUseAdaptation>> { Ref::map(self.0.borrow(), |o| &o.adaptations) }
    pub fn p_adaptations_get(&self) -> Map<ArrayKey, crate::php_parser::node::stmt::TraitUseAdaptation> { self.0.borrow().adaptations.clone() }
    pub fn p_adaptations_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::stmt::TraitUseAdaptation>> { Some(self.0.borrow().adaptations.clone()) }
    pub fn p_adaptations_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::stmt::TraitUseAdaptation>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.adaptations) }
    pub fn set_p_adaptations(&self, v: Map<ArrayKey, crate::php_parser::node::stmt::TraitUseAdaptation>) { self.0.borrow_mut().adaptations = v; }
    pub fn new(mut traits: Map<ArrayKey, crate::php_parser::node::Name>, mut adaptations: Map<ArrayKey, crate::php_parser::node::stmt::TraitUseAdaptation>, mut attributes: Map<Str, Mixed>) -> Result<TraitUse, Throw> {
        let this = TraitUse(Rc::new(RefCell::new(TraitUseObj {
            attributes: Default::default(),
            traits: Default::default(),
            adaptations: Default::default(),
        })));
        this.magic__construct(traits, adaptations, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut traits: Map<ArrayKey, crate::php_parser::node::Name>, mut adaptations: Map<ArrayKey, crate::php_parser::node::stmt::TraitUseAdaptation>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_traits(traits.clone());
    self.set_p_adaptations(adaptations.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("traits"))); __m1.push(cast::<Mixed>(Str::from_static("adaptations"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_TraitUse"));
    }
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
    pub fn new_same_class(&self, mut traits: Map<ArrayKey, crate::php_parser::node::Name>, mut adaptations: Map<ArrayKey, crate::php_parser::node::stmt::TraitUseAdaptation>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::TraitUse, Throw> { Ok(Self::new(traits, adaptations, attributes)?) }
}
impl php_rt::PhpObject for TraitUse {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\TraitUse" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\traituse", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_traits_get()) { out.push((Str::from_static("traits"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_adaptations_get()) { out.push((Str::from_static("adaptations"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "traits" => { self.set_p_traits(cast::<Map<ArrayKey, crate::php_parser::node::Name>>(value)); true }, "adaptations" => { self.set_p_adaptations(cast::<Map<ArrayKey, crate::php_parser::node::stmt::TraitUseAdaptation>>(value)); true }, _ => false } }
}
impl TraitUse { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\TraitUse could not be converted to string"))) } }
impl php_rt::PhpClone for TraitUse { fn php_clone(&self) -> Self { let c = TraitUse(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for TraitUseObj { fn clone(&self) -> Self { TraitUseObj { attributes: self.attributes.clone(), traits: self.traits.clone(), adaptations: self.adaptations.clone() } } }
impl TraitUse {
}
#[derive(Clone)]
pub enum TraitUseAdaptation {
    PhpParser_Node_Stmt_TraitUseAdaptation_Alias(crate::php_parser::node::stmt::trait_use_adaptation::Alias),
    PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(crate::php_parser::node::stmt::trait_use_adaptation::Precedence),
}
impl php_rt::PhpObject for TraitUseAdaptation {
    fn class_name(&self) -> &'static str { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.class_name(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.class_name(), _ => unreachable!() } }
    fn class_ancestors(&self) -> &'static [&'static str] { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.class_ancestors(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.class_ancestors(), _ => unreachable!() } }
    fn obj_id(&self) -> usize { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.obj_id(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.obj_id(), _ => unreachable!() } }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.props(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.props(), _ => unreachable!() } }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.set_prop(name, value), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.set_prop(name, value), _ => unreachable!() } }
    fn php_to_string(&self) -> Option<Str> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.php_to_string(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.php_to_string(), _ => unreachable!() } }
}
impl TraitUseAdaptation { pub fn to_php_string(&self) -> Result<Str, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.to_php_string(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.to_php_string(), _ => unreachable!() } } }
impl php_rt::PhpClone for TraitUseAdaptation { fn php_clone(&self) -> Self { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h.php_clone()), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h.php_clone()), _ => unreachable!() } } }
impl TraitUseAdaptation {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.p_attributes(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.p_attributes(), _ => unreachable!() } }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.p_attributes_get(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.p_attributes_get(), _ => unreachable!() } }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.p_attributes_opt(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.p_attributes_opt(), _ => unreachable!() } }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.p_attributes_mut(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.p_attributes_mut(), _ => unreachable!() } }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.set_p_attributes(v), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.set_p_attributes(v), _ => unreachable!() } }
    pub fn p_trait_(&self) -> Ref<'_, Option<crate::php_parser::node::Name>> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.p_trait_(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.p_trait_(), _ => unreachable!() } }
    pub fn p_trait__get(&self) -> Option<crate::php_parser::node::Name> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.p_trait__get(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.p_trait__get(), _ => unreachable!() } }
    pub fn p_trait__opt(&self) -> Option<Option<crate::php_parser::node::Name>> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.p_trait__opt(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.p_trait__opt(), _ => unreachable!() } }
    pub fn p_trait__mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Name>> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.p_trait__mut(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.p_trait__mut(), _ => unreachable!() } }
    pub fn set_p_trait_(&self, v: Option<crate::php_parser::node::Name>) { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.set_p_trait_(v), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.set_p_trait_(v), _ => unreachable!() } }
    pub fn p_method(&self) -> Ref<'_, crate::php_parser::node::Identifier> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.p_method(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.p_method(), _ => unreachable!() } }
    pub fn p_method_get(&self) -> crate::php_parser::node::Identifier { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.p_method_get(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.p_method_get(), _ => unreachable!() } }
    pub fn p_method_opt(&self) -> Option<crate::php_parser::node::Identifier> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.p_method_opt(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.p_method_opt(), _ => unreachable!() } }
    pub fn p_method_mut(&self) -> RefMut<'_, crate::php_parser::node::Identifier> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.p_method_mut(), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.p_method_mut(), _ => unreachable!() } }
    pub fn set_p_method(&self, v: crate::php_parser::node::Identifier) { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => __h.set_p_method(v), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => __h.set_p_method(v), _ => unreachable!() } }
    pub fn magic__construct(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.magic__construct(Some(cast::<crate::php_parser::node::Name>(attributes)), unreachable!("no default for U_PhpParser_Node_Identifier_or_Str"), <Option<i64>>::default(), <Option<U_PhpParser_Node_Identifier_or_Str>>::default(), <Map<Str, Mixed>>::default())?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.magic__construct(cast::<crate::php_parser::node::Name>(attributes), unreachable!("no default for U_PhpParser_Node_Identifier_or_Str"), <Map<ArrayKey, crate::php_parser::node::Name>>::default(), <Map<Str, Mixed>>::default())?), _ => unreachable!() } }
    pub fn magic__construct__impl(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).magic__construct__impl(attributes) }
    pub fn getLine(&self) -> Result<i64, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.getLine()?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.getLine()?), _ => unreachable!() } }
    pub fn getLine__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getLine__impl() }
    pub fn getStartLine(&self) -> Result<i64, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.getStartLine()?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.getStartLine()?), _ => unreachable!() } }
    pub fn getStartLine__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartLine__impl() }
    pub fn getEndLine(&self) -> Result<i64, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.getEndLine()?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.getEndLine()?), _ => unreachable!() } }
    pub fn getEndLine__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndLine__impl() }
    pub fn getStartTokenPos(&self) -> Result<i64, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.getStartTokenPos()?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.getStartTokenPos()?), _ => unreachable!() } }
    pub fn getStartTokenPos__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartTokenPos__impl() }
    pub fn getEndTokenPos(&self) -> Result<i64, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.getEndTokenPos()?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.getEndTokenPos()?), _ => unreachable!() } }
    pub fn getEndTokenPos__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndTokenPos__impl() }
    pub fn getStartFilePos(&self) -> Result<i64, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.getStartFilePos()?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.getStartFilePos()?), _ => unreachable!() } }
    pub fn getStartFilePos__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartFilePos__impl() }
    pub fn getEndFilePos(&self) -> Result<i64, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.getEndFilePos()?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.getEndFilePos()?), _ => unreachable!() } }
    pub fn getEndFilePos__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndFilePos__impl() }
    pub fn getComments(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.getComments()?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.getComments()?), _ => unreachable!() } }
    pub fn getComments__impl(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getComments__impl() }
    pub fn getDocComment(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.getDocComment()?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.getDocComment()?), _ => unreachable!() } }
    pub fn getDocComment__impl(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getDocComment__impl() }
    pub fn setDocComment(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.setDocComment(docComment)?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.setDocComment(docComment)?), _ => unreachable!() } }
    pub fn setDocComment__impl(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setDocComment__impl(docComment) }
    pub fn setAttribute(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.setAttribute(key_v, value)?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.setAttribute(key_v, value)?), _ => unreachable!() } }
    pub fn setAttribute__impl(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttribute__impl(key_v, value) }
    pub fn hasAttribute(&self, mut key_v: Str) -> Result<bool, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.hasAttribute(key_v)?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.hasAttribute(key_v)?), _ => unreachable!() } }
    pub fn hasAttribute__impl(&self, mut key_v: Str) -> Result<bool, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).hasAttribute__impl(key_v) }
    pub fn getAttribute(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.getAttribute(key_v, default)?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.getAttribute(key_v, default)?), _ => unreachable!() } }
    pub fn getAttribute__impl(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttribute__impl(key_v, default) }
    pub fn getAttributes(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.getAttributes()?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.getAttributes()?), _ => unreachable!() } }
    pub fn getAttributes__impl(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttributes__impl() }
    pub fn setAttributes(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.setAttributes(attributes)?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.setAttributes(attributes)?), _ => unreachable!() } }
    pub fn setAttributes__impl(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttributes__impl(attributes) }
    pub fn jsonSerialize(&self) -> Result<Map<Str, Mixed>, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.jsonSerialize()?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.jsonSerialize()?), _ => unreachable!() } }
    pub fn jsonSerialize__impl(&self) -> Result<Map<Str, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).jsonSerialize__impl() }
    pub fn getType(&self) -> Result<Str, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.getType()?), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.getType()?), _ => unreachable!() } }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Str>, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(__h.getSubNodeNames()?.map_values(|v| cast::<Str>(v))), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(__h.getSubNodeNames()?.map_values(|v| cast::<Str>(v))), _ => unreachable!() } }
    pub fn new_same_class(&self, mut attributes: Map<Str, Mixed>) -> Result<TraitUseAdaptation, Throw> { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(__h) => Ok(cast::<crate::php_parser::node::stmt::TraitUseAdaptation>(__h.new_same_class(Some(cast::<crate::php_parser::node::Name>(attributes.clone())), unreachable!("no default for U_PhpParser_Node_Identifier_or_Str"), <Option<i64>>::default(), <Option<U_PhpParser_Node_Identifier_or_Str>>::default(), <Map<Str, Mixed>>::default())?)), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(__h) => Ok(cast::<crate::php_parser::node::stmt::TraitUseAdaptation>(__h.new_same_class(cast::<crate::php_parser::node::Name>(attributes.clone()), unreachable!("no default for U_PhpParser_Node_Identifier_or_Str"), <Map<ArrayKey, crate::php_parser::node::Name>>::default(), <Map<Str, Mixed>>::default())?)), _ => unreachable!() } }
}
pub struct Trait_Obj {
    pub attributes: Map<Str, Mixed>,
    pub name: Option<crate::php_parser::node::Identifier>,
    pub stmts: Map<ArrayKey, crate::php_parser::node::Stmt>,
    pub attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>,
    pub namespacedName: Option<crate::php_parser::node::Name>,
}
#[derive(Clone)]
pub struct Trait_(pub Rc<RefCell<Trait_Obj>>);
impl Trait_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_name(&self) -> Ref<'_, Option<crate::php_parser::node::Identifier>> { Ref::map(self.0.borrow(), |o| &o.name) }
    pub fn p_name_get(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().name.clone() }
    pub fn p_name_opt(&self) -> Option<Option<crate::php_parser::node::Identifier>> { Some(self.0.borrow().name.clone()) }
    pub fn p_name_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Identifier>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.name) }
    pub fn set_p_name(&self, v: Option<crate::php_parser::node::Identifier>) { self.0.borrow_mut().name = v; }
    pub fn p_stmts(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> Map<ArrayKey, crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: Map<ArrayKey, crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn p_attrGroups(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attrGroups) }
    pub fn p_attrGroups_get(&self) -> Map<ArrayKey, crate::php_parser::node::AttributeGroup> { self.0.borrow().attrGroups.clone() }
    pub fn p_attrGroups_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attrGroups.clone()) }
    pub fn p_attrGroups_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attrGroups) }
    pub fn set_p_attrGroups(&self, v: Map<ArrayKey, crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attrGroups = v; }
    pub fn p_namespacedName(&self) -> Ref<'_, Option<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.namespacedName) }
    pub fn p_namespacedName_get(&self) -> Option<crate::php_parser::node::Name> { self.0.borrow().namespacedName.clone() }
    pub fn p_namespacedName_opt(&self) -> Option<Option<crate::php_parser::node::Name>> { Some(self.0.borrow().namespacedName.clone()) }
    pub fn p_namespacedName_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.namespacedName) }
    pub fn set_p_namespacedName(&self, v: Option<crate::php_parser::node::Name>) { self.0.borrow_mut().namespacedName = v; }
    pub fn new(mut name: U_PhpParser_Node_Identifier_or_Str, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_stmtsq_Ma_b3c75089bc, mut attributes: Map<Str, Mixed>) -> Result<Trait_, Throw> {
        let this = Trait_(Rc::new(RefCell::new(Trait_Obj {
            attributes: Default::default(),
            name: Default::default(),
            stmts: Default::default(),
            attrGroups: Default::default(),
            namespacedName: Default::default(),
        })));
        this.magic__construct(name, subNodes, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_stmtsq_Ma_b3c75089bc, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_name(Some((if (match name.clone() { U_PhpParser_Node_Identifier_or_Str::Str(_) => true, _ => false }) { crate::php_parser::node::Identifier::new(cast::<Str>(name.clone()), Map::<Str, Mixed>::new())? } else { cast::<crate::php_parser::node::Identifier>(name.clone()) })));
    self.set_p_stmts((match Some({ let __c95 = subNodes.clone(); Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_stmts_Map_714ef7277e { attrGroups: __c95.attrGroups, stmts: __c95.stmts.unwrap() } }).and_then(|__b| Some(__b.stmts)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::Stmt>::new() }));
    self.set_p_attrGroups((match Some({ let __c96 = subNodes.clone(); Shape_attrGroups_Map_ArrayKey_PhpParser_Node_AttributeGroup_stmtsq_Map_551d605ab2 { attrGroups: __c96.attrGroups.unwrap(), stmts: __c96.stmts } }).and_then(|__b| Some(__b.attrGroups)) { Some(__v) => __v, None => Map::<ArrayKey, crate::php_parser::node::AttributeGroup>::new() }));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("attrGroups"))); __m1.push(cast::<Mixed>(Str::from_static("name"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Trait"));
    }
    pub fn getTraitUses(&self) -> Result<List<crate::php_parser::node::stmt::TraitUse>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getTraitUses__impl() }
    pub fn getConstants(&self) -> Result<List<crate::php_parser::node::stmt::ClassConst>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getConstants__impl() }
    pub fn getProperties(&self) -> Result<List<crate::php_parser::node::stmt::Property>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getProperties__impl() }
    pub fn getProperty(&self, mut name: Str) -> Result<Option<crate::php_parser::node::stmt::Property>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getProperty__impl(name) }
    pub fn getMethods(&self) -> Result<List<crate::php_parser::node::stmt::ClassMethod>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getMethods__impl() }
    pub fn getMethod(&self, mut name: Str) -> Result<Option<crate::php_parser::node::stmt::ClassMethod>, Throw> { cast::<crate::php_parser::node::stmt::ClassLike>(self.clone()).getMethod__impl(name) }
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
    pub fn new_same_class(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut subNodes: Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_stmtsq_Ma_b3c75089bc, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Trait_, Throw> { Ok(Self::new(name, subNodes, attributes)?) }
}
impl php_rt::PhpObject for Trait_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Trait_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\trait_", "phpparser\\node\\stmt\\classlike", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attrGroups_get()) { out.push((Str::from_static("attrGroups"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_namespacedName_get()) { out.push((Str::from_static("namespacedName"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(value.to_option().map(|__m| cast::<crate::php_parser::node::Identifier>(__m))); true }, "stmts" => { self.set_p_stmts(cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(value)); true }, "attrGroups" => { self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(value)); true }, "namespacedName" => { self.set_p_namespacedName(value.to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m))); true }, _ => false } }
}
impl Trait_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Trait_ could not be converted to string"))) } }
impl php_rt::PhpClone for Trait_ { fn php_clone(&self) -> Self { let c = Trait_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Trait_Obj { fn clone(&self) -> Self { Trait_Obj { attributes: self.attributes.clone(), name: self.name.clone(), stmts: self.stmts.clone(), attrGroups: self.attrGroups.clone(), namespacedName: self.namespacedName.clone() } } }
impl Trait_ {
}
pub struct TryCatchObj {
    pub attributes: Map<Str, Mixed>,
    pub stmts: List<crate::php_parser::node::Stmt>,
    pub catches: Map<ArrayKey, crate::php_parser::node::stmt::Catch_>,
    pub finally: Option<crate::php_parser::node::stmt::Finally_>,
}
#[derive(Clone)]
pub struct TryCatch(pub Rc<RefCell<TryCatchObj>>);
impl TryCatch {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_stmts(&self) -> Ref<'_, List<crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> List<crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<List<crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: List<crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn p_catches(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::stmt::Catch_>> { Ref::map(self.0.borrow(), |o| &o.catches) }
    pub fn p_catches_get(&self) -> Map<ArrayKey, crate::php_parser::node::stmt::Catch_> { self.0.borrow().catches.clone() }
    pub fn p_catches_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::stmt::Catch_>> { Some(self.0.borrow().catches.clone()) }
    pub fn p_catches_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::stmt::Catch_>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.catches) }
    pub fn set_p_catches(&self, v: Map<ArrayKey, crate::php_parser::node::stmt::Catch_>) { self.0.borrow_mut().catches = v; }
    pub fn p_finally(&self) -> Ref<'_, Option<crate::php_parser::node::stmt::Finally_>> { Ref::map(self.0.borrow(), |o| &o.finally) }
    pub fn p_finally_get(&self) -> Option<crate::php_parser::node::stmt::Finally_> { self.0.borrow().finally.clone() }
    pub fn p_finally_opt(&self) -> Option<Option<crate::php_parser::node::stmt::Finally_>> { Some(self.0.borrow().finally.clone()) }
    pub fn p_finally_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::stmt::Finally_>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.finally) }
    pub fn set_p_finally(&self, v: Option<crate::php_parser::node::stmt::Finally_>) { self.0.borrow_mut().finally = v; }
    pub fn new(mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut catches: Map<ArrayKey, crate::php_parser::node::stmt::Catch_>, mut finally: Option<crate::php_parser::node::stmt::Finally_>, mut attributes: Map<Str, Mixed>) -> Result<TryCatch, Throw> {
        let this = TryCatch(Rc::new(RefCell::new(TryCatchObj {
            attributes: Default::default(),
            stmts: Default::default(),
            catches: Default::default(),
            finally: Default::default(),
        })));
        this.magic__construct(stmts, catches, finally, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut catches: Map<ArrayKey, crate::php_parser::node::stmt::Catch_>, mut finally: Option<crate::php_parser::node::stmt::Finally_>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(stmts.clone()));
    self.set_p_catches(catches.clone());
    self.set_p_finally(finally.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1.push(cast::<Mixed>(Str::from_static("catches"))); __m1.push(cast::<Mixed>(Str::from_static("finally"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_TryCatch"));
    }
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
    pub fn new_same_class(&self, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut catches: Map<ArrayKey, crate::php_parser::node::stmt::Catch_>, mut finally: Option<crate::php_parser::node::stmt::Finally_>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::TryCatch, Throw> { Ok(Self::new(stmts, catches, finally, attributes)?) }
}
impl php_rt::PhpObject for TryCatch {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\TryCatch" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\trycatch", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_catches_get()) { out.push((Str::from_static("catches"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_finally_get()) { out.push((Str::from_static("finally"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "stmts" => { self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(value)); true }, "catches" => { self.set_p_catches(cast::<Map<ArrayKey, crate::php_parser::node::stmt::Catch_>>(value)); true }, "finally" => { self.set_p_finally(value.to_option().map(|__m| cast::<crate::php_parser::node::stmt::Finally_>(__m))); true }, _ => false } }
}
impl TryCatch { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\TryCatch could not be converted to string"))) } }
impl php_rt::PhpClone for TryCatch { fn php_clone(&self) -> Self { let c = TryCatch(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for TryCatchObj { fn clone(&self) -> Self { TryCatchObj { attributes: self.attributes.clone(), stmts: self.stmts.clone(), catches: self.catches.clone(), finally: self.finally.clone() } } }
impl TryCatch {
}
pub struct Unset_Obj {
    pub attributes: Map<Str, Mixed>,
    pub vars: Map<ArrayKey, crate::php_parser::node::Expr>,
}
#[derive(Clone)]
pub struct Unset_(pub Rc<RefCell<Unset_Obj>>);
impl Unset_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_vars(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.vars) }
    pub fn p_vars_get(&self) -> Map<ArrayKey, crate::php_parser::node::Expr> { self.0.borrow().vars.clone() }
    pub fn p_vars_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Expr>> { Some(self.0.borrow().vars.clone()) }
    pub fn p_vars_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.vars) }
    pub fn set_p_vars(&self, v: Map<ArrayKey, crate::php_parser::node::Expr>) { self.0.borrow_mut().vars = v; }
    pub fn new(mut vars: Map<ArrayKey, crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<Unset_, Throw> {
        let this = Unset_(Rc::new(RefCell::new(Unset_Obj {
            attributes: Default::default(),
            vars: Default::default(),
        })));
        this.magic__construct(vars, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut vars: Map<ArrayKey, crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_vars(vars.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("vars"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Unset"));
    }
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
    pub fn new_same_class(&self, mut vars: Map<ArrayKey, crate::php_parser::node::Expr>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Unset_, Throw> { Ok(Self::new(vars, attributes)?) }
}
impl php_rt::PhpObject for Unset_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Unset_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\unset_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_vars_get()) { out.push((Str::from_static("vars"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "vars" => { self.set_p_vars(cast::<Map<ArrayKey, crate::php_parser::node::Expr>>(value)); true }, _ => false } }
}
impl Unset_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Unset_ could not be converted to string"))) } }
impl php_rt::PhpClone for Unset_ { fn php_clone(&self) -> Self { let c = Unset_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Unset_Obj { fn clone(&self) -> Self { Unset_Obj { attributes: self.attributes.clone(), vars: self.vars.clone() } } }
impl Unset_ {
}
pub struct Use_Obj {
    pub attributes: Map<Str, Mixed>,
    pub type_: Mixed,
    pub uses: Map<ArrayKey, crate::php_parser::node::UseItem>,
}
#[derive(Clone)]
pub struct Use_(pub Rc<RefCell<Use_Obj>>);
impl Use_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_type_(&self) -> Ref<'_, Mixed> { Ref::map(self.0.borrow(), |o| &o.type_) }
    pub fn p_type__get(&self) -> Mixed { self.0.borrow().type_.clone() }
    pub fn p_type__opt(&self) -> Option<Mixed> { Some(self.0.borrow().type_.clone()) }
    pub fn p_type__mut(&self) -> RefMut<'_, Mixed> { RefMut::map(self.0.borrow_mut(), |o| &mut o.type_) }
    pub fn set_p_type_(&self, v: Mixed) { self.0.borrow_mut().type_ = v; }
    pub fn p_uses(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::UseItem>> { Ref::map(self.0.borrow(), |o| &o.uses) }
    pub fn p_uses_get(&self) -> Map<ArrayKey, crate::php_parser::node::UseItem> { self.0.borrow().uses.clone() }
    pub fn p_uses_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::UseItem>> { Some(self.0.borrow().uses.clone()) }
    pub fn p_uses_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::UseItem>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.uses) }
    pub fn set_p_uses(&self, v: Map<ArrayKey, crate::php_parser::node::UseItem>) { self.0.borrow_mut().uses = v; }
    pub fn new(mut uses: Map<ArrayKey, crate::php_parser::node::UseItem>, mut type_: Mixed, mut attributes: Map<Str, Mixed>) -> Result<Use_, Throw> {
        let this = Use_(Rc::new(RefCell::new(Use_Obj {
            attributes: Default::default(),
            type_: Default::default(),
            uses: Default::default(),
        })));
        this.magic__construct(uses, type_, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut uses: Map<ArrayKey, crate::php_parser::node::UseItem>, mut type_: Mixed, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_type_(cast::<Mixed>(cast::<i64>(type_.clone())));
    self.set_p_uses(uses.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("type"))); __m1.push(cast::<Mixed>(Str::from_static("uses"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_Use"));
    }
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
    pub fn new_same_class(&self, mut uses: Map<ArrayKey, crate::php_parser::node::UseItem>, mut type_: Mixed, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::Use_, Throw> { Ok(Self::new(uses, type_, attributes)?) }
}
impl php_rt::PhpObject for Use_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\Use_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\use_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_type__get()) { out.push((Str::from_static("type"), v)); } if let Some(v) = Some(self.p_uses_get()) { out.push((Str::from_static("uses"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "type" => { self.set_p_type_(value); true }, "uses" => { self.set_p_uses(cast::<Map<ArrayKey, crate::php_parser::node::UseItem>>(value)); true }, _ => false } }
}
impl Use_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\Use_ could not be converted to string"))) } }
impl php_rt::PhpClone for Use_ { fn php_clone(&self) -> Self { let c = Use_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Use_Obj { fn clone(&self) -> Self { Use_Obj { attributes: self.attributes.clone(), type_: self.type_.clone(), uses: self.uses.clone() } } }
impl Use_ {
    pub fn TYPE_UNKNOWN() -> i64 { 0i64 }
    pub fn TYPE_NORMAL() -> i64 { 1i64 }
    pub fn TYPE_FUNCTION() -> i64 { 2i64 }
    pub fn TYPE_CONSTANT() -> i64 { 3i64 }
}
pub struct While_Obj {
    pub attributes: Map<Str, Mixed>,
    pub cond: Late<crate::php_parser::node::Expr>,
    pub stmts: List<crate::php_parser::node::Stmt>,
}
#[derive(Clone)]
pub struct While_(pub Rc<RefCell<While_Obj>>);
impl While_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_cond(&self) -> Ref<'_, crate::php_parser::node::Expr> { Ref::map(self.0.borrow(), |o| o.cond.get()) }
    pub fn p_cond_get(&self) -> crate::php_parser::node::Expr { self.0.borrow().cond.get().clone() }
    pub fn p_cond_opt(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().cond.as_option().cloned() }
    pub fn p_cond_mut(&self) -> RefMut<'_, crate::php_parser::node::Expr> { RefMut::map(self.0.borrow_mut(), |o| o.cond.get_mut()) }
    pub fn set_p_cond(&self, v: crate::php_parser::node::Expr) { self.0.borrow_mut().cond.set(v); }
    pub fn p_stmts(&self) -> Ref<'_, List<crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> List<crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<List<crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: List<crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn new(mut cond: crate::php_parser::node::Expr, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<While_, Throw> {
        let this = While_(Rc::new(RefCell::new(While_Obj {
            attributes: Default::default(),
            cond: Late::uninit(),
            stmts: Default::default(),
        })));
        this.magic__construct(cond, stmts, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut cond: crate::php_parser::node::Expr, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_cond(cond.clone());
    self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(stmts.clone()));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("cond"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_While"));
    }
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
    pub fn new_same_class(&self, mut cond: crate::php_parser::node::Expr, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::While_, Throw> { Ok(Self::new(cond, stmts, attributes)?) }
}
impl php_rt::PhpObject for While_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\While_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\while_", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_cond_opt() { out.push((Str::from_static("cond"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "cond" => { self.set_p_cond(cast::<crate::php_parser::node::Expr>(value)); true }, "stmts" => { self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(value)); true }, _ => false } }
}
impl While_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\While_ could not be converted to string"))) } }
impl php_rt::PhpClone for While_ { fn php_clone(&self) -> Self { let c = While_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for While_Obj { fn clone(&self) -> Self { While_Obj { attributes: self.attributes.clone(), cond: self.cond.clone(), stmts: self.stmts.clone() } } }
impl While_ {
}
impl php_rt::Truthy for Block { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Block { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Block")) } }
impl php_rt::Identical for Block { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Block { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Block { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Block { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Block> for Mixed { fn cast_to(self) -> Block { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Block>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Block") } }
impl php_rt::TryDowncast for Block { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Block>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Block { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Block> for AnyObject { fn cast_to(self) -> Block { cast::<Block>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Block> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\block") } }
impl php_rt::InstanceOf<Block> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\block") } }
impl php_rt::InstanceOf<Block> for Block { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Break_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Break_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Break_")) } }
impl php_rt::Identical for Break_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Break_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Break_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Break_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Break_> for Mixed { fn cast_to(self) -> Break_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Break_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Break_") } }
impl php_rt::TryDowncast for Break_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Break_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Break_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Break_> for AnyObject { fn cast_to(self) -> Break_ { cast::<Break_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Break_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\break_") } }
impl php_rt::InstanceOf<Break_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\break_") } }
impl php_rt::InstanceOf<Break_> for Break_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Case_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Case_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Case_")) } }
impl php_rt::Identical for Case_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Case_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Case_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Case_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Case_> for Mixed { fn cast_to(self) -> Case_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Case_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Case_") } }
impl php_rt::TryDowncast for Case_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Case_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Case_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Case_> for AnyObject { fn cast_to(self) -> Case_ { cast::<Case_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Case_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\case_") } }
impl php_rt::InstanceOf<Case_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\case_") } }
impl php_rt::InstanceOf<Case_> for Case_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Catch_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Catch_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Catch_")) } }
impl php_rt::Identical for Catch_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Catch_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Catch_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Catch_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Catch_> for Mixed { fn cast_to(self) -> Catch_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Catch_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Catch_") } }
impl php_rt::TryDowncast for Catch_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Catch_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Catch_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Catch_> for AnyObject { fn cast_to(self) -> Catch_ { cast::<Catch_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Catch_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\catch_") } }
impl php_rt::InstanceOf<Catch_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\catch_") } }
impl php_rt::InstanceOf<Catch_> for Catch_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for ClassConst { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for ClassConst { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\ClassConst")) } }
impl php_rt::Identical for ClassConst { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for ClassConst { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for ClassConst { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for ClassConst { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<ClassConst> for Mixed { fn cast_to(self) -> ClassConst { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::ClassConst>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\ClassConst") } }
impl php_rt::TryDowncast for ClassConst { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::ClassConst>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for ClassConst { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<ClassConst> for AnyObject { fn cast_to(self) -> ClassConst { cast::<ClassConst>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<ClassConst> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\classconst") } }
impl php_rt::InstanceOf<ClassConst> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\classconst") } }
impl php_rt::InstanceOf<ClassConst> for ClassConst { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for ClassLike { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for ClassLike { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\ClassLike")) } }
impl php_rt::Identical for ClassLike { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for ClassLike { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for ClassLike { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for ClassLike { fn cast_to(self) -> Mixed { match self { ClassLike::PhpParser_Node_Stmt_Class_(v) => Mixed::Obj(Rc::new(v)), ClassLike::PhpParser_Node_Stmt_Enum_(v) => Mixed::Obj(Rc::new(v)), ClassLike::PhpParser_Node_Stmt_Interface_(v) => Mixed::Obj(Rc::new(v)), ClassLike::PhpParser_Node_Stmt_Trait_(v) => Mixed::Obj(Rc::new(v)), _ => unreachable!() } } }
impl php_rt::CastTo<ClassLike> for Mixed { fn cast_to(self) -> ClassLike { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Class_>() { return crate::php_parser::node::stmt::ClassLike::PhpParser_Node_Stmt_Class_(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Enum_>() { return crate::php_parser::node::stmt::ClassLike::PhpParser_Node_Stmt_Enum_(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Interface_>() { return crate::php_parser::node::stmt::ClassLike::PhpParser_Node_Stmt_Interface_(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Trait_>() { return crate::php_parser::node::stmt::ClassLike::PhpParser_Node_Stmt_Trait_(v.clone()); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\ClassLike") } }
impl php_rt::TryDowncast for ClassLike { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Class_>() { return Some(crate::php_parser::node::stmt::ClassLike::PhpParser_Node_Stmt_Class_(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Enum_>() { return Some(crate::php_parser::node::stmt::ClassLike::PhpParser_Node_Stmt_Enum_(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Interface_>() { return Some(crate::php_parser::node::stmt::ClassLike::PhpParser_Node_Stmt_Interface_(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Trait_>() { return Some(crate::php_parser::node::stmt::ClassLike::PhpParser_Node_Stmt_Trait_(v.clone())); } None } }
impl php_rt::CastTo<AnyObject> for ClassLike { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<ClassLike> for AnyObject { fn cast_to(self) -> ClassLike { cast::<ClassLike>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<ClassLike> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\classlike") } }
impl php_rt::InstanceOf<ClassLike> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\classlike") } }
impl php_rt::InstanceOf<ClassLike> for ClassLike { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for ClassMethod { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for ClassMethod { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\ClassMethod")) } }
impl php_rt::Identical for ClassMethod { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for ClassMethod { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for ClassMethod { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for ClassMethod { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<ClassMethod> for Mixed { fn cast_to(self) -> ClassMethod { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::ClassMethod>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\ClassMethod") } }
impl php_rt::TryDowncast for ClassMethod { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::ClassMethod>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for ClassMethod { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<ClassMethod> for AnyObject { fn cast_to(self) -> ClassMethod { cast::<ClassMethod>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<ClassMethod> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\classmethod") } }
impl php_rt::InstanceOf<ClassMethod> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\classmethod") } }
impl php_rt::InstanceOf<ClassMethod> for ClassMethod { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Class_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Class_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Class_")) } }
impl php_rt::Identical for Class_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Class_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Class_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Class_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Class_> for Mixed { fn cast_to(self) -> Class_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Class_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Class_") } }
impl php_rt::TryDowncast for Class_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Class_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Class_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Class_> for AnyObject { fn cast_to(self) -> Class_ { cast::<Class_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Class_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\class_") } }
impl php_rt::InstanceOf<Class_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\class_") } }
impl php_rt::InstanceOf<Class_> for Class_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Const_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Const_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Const_")) } }
impl php_rt::Identical for Const_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Const_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Const_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Const_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Const_> for Mixed { fn cast_to(self) -> Const_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Const_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Const_") } }
impl php_rt::TryDowncast for Const_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Const_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Const_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Const_> for AnyObject { fn cast_to(self) -> Const_ { cast::<Const_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Const_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\const_") } }
impl php_rt::InstanceOf<Const_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\const_") } }
impl php_rt::InstanceOf<Const_> for Const_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Continue_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Continue_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Continue_")) } }
impl php_rt::Identical for Continue_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Continue_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Continue_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Continue_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Continue_> for Mixed { fn cast_to(self) -> Continue_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Continue_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Continue_") } }
impl php_rt::TryDowncast for Continue_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Continue_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Continue_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Continue_> for AnyObject { fn cast_to(self) -> Continue_ { cast::<Continue_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Continue_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\continue_") } }
impl php_rt::InstanceOf<Continue_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\continue_") } }
impl php_rt::InstanceOf<Continue_> for Continue_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Declare_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Declare_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Declare_")) } }
impl php_rt::Identical for Declare_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Declare_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Declare_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Declare_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Declare_> for Mixed { fn cast_to(self) -> Declare_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Declare_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Declare_") } }
impl php_rt::TryDowncast for Declare_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Declare_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Declare_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Declare_> for AnyObject { fn cast_to(self) -> Declare_ { cast::<Declare_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Declare_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\declare_") } }
impl php_rt::InstanceOf<Declare_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\declare_") } }
impl php_rt::InstanceOf<Declare_> for Declare_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Do_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Do_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Do_")) } }
impl php_rt::Identical for Do_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Do_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Do_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Do_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Do_> for Mixed { fn cast_to(self) -> Do_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Do_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Do_") } }
impl php_rt::TryDowncast for Do_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Do_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Do_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Do_> for AnyObject { fn cast_to(self) -> Do_ { cast::<Do_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Do_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\do_") } }
impl php_rt::InstanceOf<Do_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\do_") } }
impl php_rt::InstanceOf<Do_> for Do_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Echo_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Echo_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Echo_")) } }
impl php_rt::Identical for Echo_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Echo_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Echo_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Echo_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Echo_> for Mixed { fn cast_to(self) -> Echo_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Echo_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Echo_") } }
impl php_rt::TryDowncast for Echo_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Echo_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Echo_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Echo_> for AnyObject { fn cast_to(self) -> Echo_ { cast::<Echo_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Echo_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\echo_") } }
impl php_rt::InstanceOf<Echo_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\echo_") } }
impl php_rt::InstanceOf<Echo_> for Echo_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for ElseIf_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for ElseIf_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\ElseIf_")) } }
impl php_rt::Identical for ElseIf_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for ElseIf_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for ElseIf_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for ElseIf_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<ElseIf_> for Mixed { fn cast_to(self) -> ElseIf_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::ElseIf_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\ElseIf_") } }
impl php_rt::TryDowncast for ElseIf_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::ElseIf_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for ElseIf_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<ElseIf_> for AnyObject { fn cast_to(self) -> ElseIf_ { cast::<ElseIf_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<ElseIf_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\elseif_") } }
impl php_rt::InstanceOf<ElseIf_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\elseif_") } }
impl php_rt::InstanceOf<ElseIf_> for ElseIf_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Else_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Else_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Else_")) } }
impl php_rt::Identical for Else_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Else_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Else_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Else_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Else_> for Mixed { fn cast_to(self) -> Else_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Else_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Else_") } }
impl php_rt::TryDowncast for Else_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Else_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Else_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Else_> for AnyObject { fn cast_to(self) -> Else_ { cast::<Else_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Else_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\else_") } }
impl php_rt::InstanceOf<Else_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\else_") } }
impl php_rt::InstanceOf<Else_> for Else_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for EnumCase { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for EnumCase { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\EnumCase")) } }
impl php_rt::Identical for EnumCase { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for EnumCase { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for EnumCase { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for EnumCase { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<EnumCase> for Mixed { fn cast_to(self) -> EnumCase { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::EnumCase>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\EnumCase") } }
impl php_rt::TryDowncast for EnumCase { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::EnumCase>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for EnumCase { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<EnumCase> for AnyObject { fn cast_to(self) -> EnumCase { cast::<EnumCase>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<EnumCase> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\enumcase") } }
impl php_rt::InstanceOf<EnumCase> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\enumcase") } }
impl php_rt::InstanceOf<EnumCase> for EnumCase { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Enum_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Enum_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Enum_")) } }
impl php_rt::Identical for Enum_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Enum_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Enum_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Enum_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Enum_> for Mixed { fn cast_to(self) -> Enum_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Enum_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Enum_") } }
impl php_rt::TryDowncast for Enum_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Enum_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Enum_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Enum_> for AnyObject { fn cast_to(self) -> Enum_ { cast::<Enum_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Enum_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\enum_") } }
impl php_rt::InstanceOf<Enum_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\enum_") } }
impl php_rt::InstanceOf<Enum_> for Enum_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Expression { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Expression { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Expression")) } }
impl php_rt::Identical for Expression { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Expression { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Expression { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Expression { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Expression> for Mixed { fn cast_to(self) -> Expression { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Expression>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Expression") } }
impl php_rt::TryDowncast for Expression { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Expression>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Expression { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Expression> for AnyObject { fn cast_to(self) -> Expression { cast::<Expression>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Expression> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\expression") } }
impl php_rt::InstanceOf<Expression> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\expression") } }
impl php_rt::InstanceOf<Expression> for Expression { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Finally_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Finally_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Finally_")) } }
impl php_rt::Identical for Finally_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Finally_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Finally_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Finally_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Finally_> for Mixed { fn cast_to(self) -> Finally_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Finally_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Finally_") } }
impl php_rt::TryDowncast for Finally_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Finally_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Finally_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Finally_> for AnyObject { fn cast_to(self) -> Finally_ { cast::<Finally_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Finally_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\finally_") } }
impl php_rt::InstanceOf<Finally_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\finally_") } }
impl php_rt::InstanceOf<Finally_> for Finally_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for For_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for For_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\For_")) } }
impl php_rt::Identical for For_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for For_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for For_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for For_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<For_> for Mixed { fn cast_to(self) -> For_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::For_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\For_") } }
impl php_rt::TryDowncast for For_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::For_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for For_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<For_> for AnyObject { fn cast_to(self) -> For_ { cast::<For_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<For_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\for_") } }
impl php_rt::InstanceOf<For_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\for_") } }
impl php_rt::InstanceOf<For_> for For_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Foreach_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Foreach_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Foreach_")) } }
impl php_rt::Identical for Foreach_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Foreach_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Foreach_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Foreach_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Foreach_> for Mixed { fn cast_to(self) -> Foreach_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Foreach_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Foreach_") } }
impl php_rt::TryDowncast for Foreach_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Foreach_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Foreach_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Foreach_> for AnyObject { fn cast_to(self) -> Foreach_ { cast::<Foreach_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Foreach_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\foreach_") } }
impl php_rt::InstanceOf<Foreach_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\foreach_") } }
impl php_rt::InstanceOf<Foreach_> for Foreach_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Function_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Function_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Function_")) } }
impl php_rt::Identical for Function_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Function_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Function_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Function_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Function_> for Mixed { fn cast_to(self) -> Function_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Function_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Function_") } }
impl php_rt::TryDowncast for Function_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Function_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Function_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Function_> for AnyObject { fn cast_to(self) -> Function_ { cast::<Function_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Function_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\function_") } }
impl php_rt::InstanceOf<Function_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\function_") } }
impl php_rt::InstanceOf<Function_> for Function_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Global_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Global_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Global_")) } }
impl php_rt::Identical for Global_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Global_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Global_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Global_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Global_> for Mixed { fn cast_to(self) -> Global_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Global_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Global_") } }
impl php_rt::TryDowncast for Global_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Global_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Global_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Global_> for AnyObject { fn cast_to(self) -> Global_ { cast::<Global_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Global_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\global_") } }
impl php_rt::InstanceOf<Global_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\global_") } }
impl php_rt::InstanceOf<Global_> for Global_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Goto_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Goto_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Goto_")) } }
impl php_rt::Identical for Goto_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Goto_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Goto_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Goto_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Goto_> for Mixed { fn cast_to(self) -> Goto_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Goto_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Goto_") } }
impl php_rt::TryDowncast for Goto_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Goto_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Goto_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Goto_> for AnyObject { fn cast_to(self) -> Goto_ { cast::<Goto_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Goto_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\goto_") } }
impl php_rt::InstanceOf<Goto_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\goto_") } }
impl php_rt::InstanceOf<Goto_> for Goto_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for GroupUse { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for GroupUse { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\GroupUse")) } }
impl php_rt::Identical for GroupUse { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for GroupUse { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for GroupUse { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for GroupUse { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<GroupUse> for Mixed { fn cast_to(self) -> GroupUse { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::GroupUse>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\GroupUse") } }
impl php_rt::TryDowncast for GroupUse { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::GroupUse>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for GroupUse { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<GroupUse> for AnyObject { fn cast_to(self) -> GroupUse { cast::<GroupUse>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<GroupUse> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\groupuse") } }
impl php_rt::InstanceOf<GroupUse> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\groupuse") } }
impl php_rt::InstanceOf<GroupUse> for GroupUse { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for HaltCompiler { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for HaltCompiler { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\HaltCompiler")) } }
impl php_rt::Identical for HaltCompiler { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for HaltCompiler { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for HaltCompiler { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for HaltCompiler { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<HaltCompiler> for Mixed { fn cast_to(self) -> HaltCompiler { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::HaltCompiler>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\HaltCompiler") } }
impl php_rt::TryDowncast for HaltCompiler { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::HaltCompiler>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for HaltCompiler { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<HaltCompiler> for AnyObject { fn cast_to(self) -> HaltCompiler { cast::<HaltCompiler>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<HaltCompiler> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\haltcompiler") } }
impl php_rt::InstanceOf<HaltCompiler> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\haltcompiler") } }
impl php_rt::InstanceOf<HaltCompiler> for HaltCompiler { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for If_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for If_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\If_")) } }
impl php_rt::Identical for If_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for If_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for If_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for If_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<If_> for Mixed { fn cast_to(self) -> If_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::If_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\If_") } }
impl php_rt::TryDowncast for If_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::If_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for If_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<If_> for AnyObject { fn cast_to(self) -> If_ { cast::<If_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<If_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\if_") } }
impl php_rt::InstanceOf<If_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\if_") } }
impl php_rt::InstanceOf<If_> for If_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for InlineHTML { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for InlineHTML { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\InlineHTML")) } }
impl php_rt::Identical for InlineHTML { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for InlineHTML { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for InlineHTML { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for InlineHTML { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<InlineHTML> for Mixed { fn cast_to(self) -> InlineHTML { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::InlineHTML>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\InlineHTML") } }
impl php_rt::TryDowncast for InlineHTML { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::InlineHTML>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for InlineHTML { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<InlineHTML> for AnyObject { fn cast_to(self) -> InlineHTML { cast::<InlineHTML>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<InlineHTML> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\inlinehtml") } }
impl php_rt::InstanceOf<InlineHTML> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\inlinehtml") } }
impl php_rt::InstanceOf<InlineHTML> for InlineHTML { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Interface_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Interface_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Interface_")) } }
impl php_rt::Identical for Interface_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Interface_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Interface_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Interface_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Interface_> for Mixed { fn cast_to(self) -> Interface_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Interface_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Interface_") } }
impl php_rt::TryDowncast for Interface_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Interface_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Interface_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Interface_> for AnyObject { fn cast_to(self) -> Interface_ { cast::<Interface_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Interface_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\interface_") } }
impl php_rt::InstanceOf<Interface_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\interface_") } }
impl php_rt::InstanceOf<Interface_> for Interface_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Label { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Label { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Label")) } }
impl php_rt::Identical for Label { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Label { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Label { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Label { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Label> for Mixed { fn cast_to(self) -> Label { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Label>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Label") } }
impl php_rt::TryDowncast for Label { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Label>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Label { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Label> for AnyObject { fn cast_to(self) -> Label { cast::<Label>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Label> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\label") } }
impl php_rt::InstanceOf<Label> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\label") } }
impl php_rt::InstanceOf<Label> for Label { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Namespace_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Namespace_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Namespace_")) } }
impl php_rt::Identical for Namespace_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Namespace_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Namespace_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Namespace_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Namespace_> for Mixed { fn cast_to(self) -> Namespace_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Namespace_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Namespace_") } }
impl php_rt::TryDowncast for Namespace_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Namespace_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Namespace_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Namespace_> for AnyObject { fn cast_to(self) -> Namespace_ { cast::<Namespace_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Namespace_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\namespace_") } }
impl php_rt::InstanceOf<Namespace_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\namespace_") } }
impl php_rt::InstanceOf<Namespace_> for Namespace_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Nop { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Nop { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Nop")) } }
impl php_rt::Identical for Nop { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Nop { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Nop { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Nop { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Nop> for Mixed { fn cast_to(self) -> Nop { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Nop>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Nop") } }
impl php_rt::TryDowncast for Nop { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Nop>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Nop { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Nop> for AnyObject { fn cast_to(self) -> Nop { cast::<Nop>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Nop> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\nop") } }
impl php_rt::InstanceOf<Nop> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\nop") } }
impl php_rt::InstanceOf<Nop> for Nop { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Property { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Property { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Property")) } }
impl php_rt::Identical for Property { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Property { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Property { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Property { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Property> for Mixed { fn cast_to(self) -> Property { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Property>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Property") } }
impl php_rt::TryDowncast for Property { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Property>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Property { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Property> for AnyObject { fn cast_to(self) -> Property { cast::<Property>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Property> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\property") } }
impl php_rt::InstanceOf<Property> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\property") } }
impl php_rt::InstanceOf<Property> for Property { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Return_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Return_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Return_")) } }
impl php_rt::Identical for Return_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Return_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Return_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Return_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Return_> for Mixed { fn cast_to(self) -> Return_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Return_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Return_") } }
impl php_rt::TryDowncast for Return_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Return_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Return_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Return_> for AnyObject { fn cast_to(self) -> Return_ { cast::<Return_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Return_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\return_") } }
impl php_rt::InstanceOf<Return_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\return_") } }
impl php_rt::InstanceOf<Return_> for Return_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Static_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Static_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Static_")) } }
impl php_rt::Identical for Static_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Static_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Static_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Static_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Static_> for Mixed { fn cast_to(self) -> Static_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Static_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Static_") } }
impl php_rt::TryDowncast for Static_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Static_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Static_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Static_> for AnyObject { fn cast_to(self) -> Static_ { cast::<Static_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Static_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\static_") } }
impl php_rt::InstanceOf<Static_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\static_") } }
impl php_rt::InstanceOf<Static_> for Static_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Switch_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Switch_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Switch_")) } }
impl php_rt::Identical for Switch_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Switch_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Switch_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Switch_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Switch_> for Mixed { fn cast_to(self) -> Switch_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Switch_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Switch_") } }
impl php_rt::TryDowncast for Switch_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Switch_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Switch_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Switch_> for AnyObject { fn cast_to(self) -> Switch_ { cast::<Switch_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Switch_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\switch_") } }
impl php_rt::InstanceOf<Switch_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\switch_") } }
impl php_rt::InstanceOf<Switch_> for Switch_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for TraitUse { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for TraitUse { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\TraitUse")) } }
impl php_rt::Identical for TraitUse { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for TraitUse { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for TraitUse { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for TraitUse { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<TraitUse> for Mixed { fn cast_to(self) -> TraitUse { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::TraitUse>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\TraitUse") } }
impl php_rt::TryDowncast for TraitUse { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::TraitUse>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for TraitUse { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<TraitUse> for AnyObject { fn cast_to(self) -> TraitUse { cast::<TraitUse>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<TraitUse> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\traituse") } }
impl php_rt::InstanceOf<TraitUse> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\traituse") } }
impl php_rt::InstanceOf<TraitUse> for TraitUse { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for TraitUseAdaptation { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for TraitUseAdaptation { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\TraitUseAdaptation")) } }
impl php_rt::Identical for TraitUseAdaptation { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for TraitUseAdaptation { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for TraitUseAdaptation { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for TraitUseAdaptation { fn cast_to(self) -> Mixed { match self { TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(v) => Mixed::Obj(Rc::new(v)), TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(v) => Mixed::Obj(Rc::new(v)), _ => unreachable!() } } }
impl php_rt::CastTo<TraitUseAdaptation> for Mixed { fn cast_to(self) -> TraitUseAdaptation { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::trait_use_adaptation::Alias>() { return crate::php_parser::node::stmt::TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::trait_use_adaptation::Precedence>() { return crate::php_parser::node::stmt::TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(v.clone()); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\TraitUseAdaptation") } }
impl php_rt::TryDowncast for TraitUseAdaptation { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::trait_use_adaptation::Alias>() { return Some(crate::php_parser::node::stmt::TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Alias(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::trait_use_adaptation::Precedence>() { return Some(crate::php_parser::node::stmt::TraitUseAdaptation::PhpParser_Node_Stmt_TraitUseAdaptation_Precedence(v.clone())); } None } }
impl php_rt::CastTo<AnyObject> for TraitUseAdaptation { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<TraitUseAdaptation> for AnyObject { fn cast_to(self) -> TraitUseAdaptation { cast::<TraitUseAdaptation>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<TraitUseAdaptation> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\traituseadaptation") } }
impl php_rt::InstanceOf<TraitUseAdaptation> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\traituseadaptation") } }
impl php_rt::InstanceOf<TraitUseAdaptation> for TraitUseAdaptation { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Trait_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Trait_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Trait_")) } }
impl php_rt::Identical for Trait_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Trait_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Trait_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Trait_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Trait_> for Mixed { fn cast_to(self) -> Trait_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Trait_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Trait_") } }
impl php_rt::TryDowncast for Trait_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Trait_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Trait_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Trait_> for AnyObject { fn cast_to(self) -> Trait_ { cast::<Trait_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Trait_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\trait_") } }
impl php_rt::InstanceOf<Trait_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\trait_") } }
impl php_rt::InstanceOf<Trait_> for Trait_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for TryCatch { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for TryCatch { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\TryCatch")) } }
impl php_rt::Identical for TryCatch { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for TryCatch { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for TryCatch { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for TryCatch { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<TryCatch> for Mixed { fn cast_to(self) -> TryCatch { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::TryCatch>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\TryCatch") } }
impl php_rt::TryDowncast for TryCatch { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::TryCatch>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for TryCatch { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<TryCatch> for AnyObject { fn cast_to(self) -> TryCatch { cast::<TryCatch>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<TryCatch> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\trycatch") } }
impl php_rt::InstanceOf<TryCatch> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\trycatch") } }
impl php_rt::InstanceOf<TryCatch> for TryCatch { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Unset_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Unset_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Unset_")) } }
impl php_rt::Identical for Unset_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Unset_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Unset_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Unset_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Unset_> for Mixed { fn cast_to(self) -> Unset_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Unset_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Unset_") } }
impl php_rt::TryDowncast for Unset_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Unset_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Unset_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Unset_> for AnyObject { fn cast_to(self) -> Unset_ { cast::<Unset_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Unset_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\unset_") } }
impl php_rt::InstanceOf<Unset_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\unset_") } }
impl php_rt::InstanceOf<Unset_> for Unset_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Use_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Use_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\Use_")) } }
impl php_rt::Identical for Use_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Use_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Use_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Use_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Use_> for Mixed { fn cast_to(self) -> Use_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Use_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\Use_") } }
impl php_rt::TryDowncast for Use_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::Use_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Use_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Use_> for AnyObject { fn cast_to(self) -> Use_ { cast::<Use_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Use_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\use_") } }
impl php_rt::InstanceOf<Use_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\use_") } }
impl php_rt::InstanceOf<Use_> for Use_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for While_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for While_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\While_")) } }
impl php_rt::Identical for While_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for While_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for While_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for While_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<While_> for Mixed { fn cast_to(self) -> While_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::While_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\While_") } }
impl php_rt::TryDowncast for While_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::While_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for While_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<While_> for AnyObject { fn cast_to(self) -> While_ { cast::<While_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<While_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\while_") } }
impl php_rt::InstanceOf<While_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\while_") } }
impl php_rt::InstanceOf<While_> for While_ { fn is_instance(&self) -> bool { true } }
