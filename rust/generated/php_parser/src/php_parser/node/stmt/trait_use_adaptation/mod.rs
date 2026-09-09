use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub struct AliasObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub trait_: Late<Option<crate::php_parser::node::Name>>,
    pub method: Late<crate::php_parser::node::Identifier>,
    pub newModifier: Late<Option<i64>>,
    pub newName: Late<Option<crate::php_parser::node::Identifier>>,
}
#[derive(Clone)]
pub struct Alias(pub Rc<RefCell<AliasObj>>);
impl Alias {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_trait_(&self) -> Ref<'_, Option<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| o.trait_.get()) }
    pub fn p_trait__get(&self) -> Option<crate::php_parser::node::Name> { self.0.borrow().trait_.get().clone() }
    pub fn p_trait__opt(&self) -> Option<Option<crate::php_parser::node::Name>> { self.0.borrow().trait_.as_option().cloned() }
    pub fn p_trait__mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| o.trait_.get_or_default_mut()) }
    pub fn set_p_trait_(&self, v: Option<crate::php_parser::node::Name>) { self.0.borrow_mut().trait_.set(v); }
    pub fn p_method(&self) -> Ref<'_, crate::php_parser::node::Identifier> { Ref::map(self.0.borrow(), |o| o.method.get()) }
    pub fn p_method_get(&self) -> crate::php_parser::node::Identifier { self.0.borrow().method.get().clone() }
    pub fn p_method_opt(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().method.as_option().cloned() }
    pub fn p_method_mut(&self) -> RefMut<'_, crate::php_parser::node::Identifier> { RefMut::map(self.0.borrow_mut(), |o| o.method.get_mut()) }
    pub fn set_p_method(&self, v: crate::php_parser::node::Identifier) { self.0.borrow_mut().method.set(v); }
    pub fn p_newModifier(&self) -> Ref<'_, Option<i64>> { Ref::map(self.0.borrow(), |o| o.newModifier.get()) }
    pub fn p_newModifier_get(&self) -> Option<i64> { self.0.borrow().newModifier.get().clone() }
    pub fn p_newModifier_opt(&self) -> Option<Option<i64>> { self.0.borrow().newModifier.as_option().cloned() }
    pub fn p_newModifier_mut(&self) -> RefMut<'_, Option<i64>> { RefMut::map(self.0.borrow_mut(), |o| o.newModifier.get_or_default_mut()) }
    pub fn set_p_newModifier(&self, v: Option<i64>) { self.0.borrow_mut().newModifier.set(v); }
    pub fn p_newName(&self) -> Ref<'_, Option<crate::php_parser::node::Identifier>> { Ref::map(self.0.borrow(), |o| o.newName.get()) }
    pub fn p_newName_get(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().newName.get().clone() }
    pub fn p_newName_opt(&self) -> Option<Option<crate::php_parser::node::Identifier>> { self.0.borrow().newName.as_option().cloned() }
    pub fn p_newName_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Identifier>> { RefMut::map(self.0.borrow_mut(), |o| o.newName.get_or_default_mut()) }
    pub fn set_p_newName(&self, v: Option<crate::php_parser::node::Identifier>) { self.0.borrow_mut().newName.set(v); }
    pub fn new_uninit() -> Alias {
        Alias(Rc::new(RefCell::new(AliasObj {
            attributes: Late::uninit(),
            trait_: Late::uninit(),
            method: Late::uninit(),
            newModifier: Late::uninit(),
            newName: Late::uninit(),
        })))
    }
    pub fn new(mut trait_: Option<crate::php_parser::node::Name>, mut method: U_PhpParser_Node_Identifier_or_Str, mut newModifier: Option<i64>, mut newName: Option<U_PhpParser_Node_Identifier_or_Str>, mut attributes: Map<Str, Mixed>) -> Result<Alias, Throw> {
        let this = Alias(Rc::new(RefCell::new(AliasObj {
            attributes: Late::uninit(),
            trait_: Late::uninit(),
            method: Late::uninit(),
            newModifier: Late::uninit(),
            newName: Late::uninit(),
        })));
        this.magic__construct(trait_, method, newModifier, newName, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut trait_: Option<crate::php_parser::node::Name>, mut method: U_PhpParser_Node_Identifier_or_Str, mut newModifier: Option<i64>, mut newName: Option<U_PhpParser_Node_Identifier_or_Str>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_trait_(trait_.clone());
    self.set_p_method((if (match method.clone() { U_PhpParser_Node_Identifier_or_Str::Str(_) => true, U_PhpParser_Node_Identifier_or_Str::Other__(__m) => __m.is_string(), _ => false }) { crate::php_parser::node::Identifier::new(cast::<Str>(method.clone()), Map::<Str, Mixed>::new())? } else { cast::<crate::php_parser::node::Identifier>(method.clone()) }));
    self.set_p_newModifier(newModifier);
    self.set_p_newName((if (match newName.clone() { Some(__u) => match __u { U_PhpParser_Node_Identifier_or_Str::Str(_) => true, U_PhpParser_Node_Identifier_or_Str::Other__(__m) => __m.is_string(), _ => false }, None => false }) { Some(crate::php_parser::node::Identifier::new((match newName.clone() { Some(__o) => cast::<Str>(__o), None => <Str>::default() }), Map::<Str, Mixed>::new())?) } else { newName.clone().map(|v| cast::<crate::php_parser::node::Identifier>(v)) }));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("trait"))); __m1.push(cast::<Mixed>(Str::from_static("method"))); __m1.push(cast::<Mixed>(Str::from_static("newModifier"))); __m1.push(cast::<Mixed>(Str::from_static("newName"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_TraitUseAdaptation_Alias"));
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
    pub fn new_same_class(&self, mut trait_: Option<crate::php_parser::node::Name>, mut method: U_PhpParser_Node_Identifier_or_Str, mut newModifier: Option<i64>, mut newName: Option<U_PhpParser_Node_Identifier_or_Str>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::trait_use_adaptation::Alias, Throw> { Ok(Self::new(trait_, method, newModifier, newName, attributes)?) }
}
impl php_rt::PhpObject for Alias {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\TraitUseAdaptation\\Alias" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\traituseadaptation\\alias", "phpparser\\node\\stmt\\traituseadaptation", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_trait__opt() { out.push((Str::from_static("trait"), cast::<Mixed>(v))); } if let Some(v) = self.p_method_opt() { out.push((Str::from_static("method"), cast::<Mixed>(v))); } if let Some(v) = self.p_newModifier_opt() { out.push((Str::from_static("newModifier"), cast::<Mixed>(v))); } if let Some(v) = self.p_newName_opt() { out.push((Str::from_static("newName"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_trait__opt() { out.push((Str::from_static("trait"), cast::<Mixed>(v))); } if let Some(v) = self.p_method_opt() { out.push((Str::from_static("method"), cast::<Mixed>(v))); } if let Some(v) = self.p_newModifier_opt() { out.push((Str::from_static("newModifier"), cast::<Mixed>(v))); } if let Some(v) = self.p_newName_opt() { out.push((Str::from_static("newName"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "trait" => { self.set_p_trait_(value.to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m))); true }, "method" => { self.set_p_method(cast::<crate::php_parser::node::Identifier>(value)); true }, "newModifier" => { self.set_p_newModifier(value.to_option().map(|__m| cast::<i64>(__m))); true }, "newName" => { self.set_p_newName(value.to_option().map(|__m| cast::<crate::php_parser::node::Identifier>(__m))); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "trait" => self.p_trait__opt().map(|v| cast::<Mixed>(v)), "method" => self.p_method_opt().map(|v| cast::<Mixed>(v)), "newModifier" => self.p_newModifier_opt().map(|v| cast::<Mixed>(v)), "newName" => self.p_newName_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => __a.clone().to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m)), None => <Option<crate::php_parser::node::Name>>::default() }), (match args.get(1) { Some(__a) => cast::<U_PhpParser_Node_Identifier_or_Str>(__a.clone()), None => unreachable!("no default for U_PhpParser_Node_Identifier_or_Str") }), (match args.get(2) { Some(__a) => __a.clone().to_option().map(|__m| cast::<i64>(__m)), None => <Option<i64>>::default() }), (match args.get(3) { Some(__a) => __a.clone().to_option().map(|__m| cast::<U_PhpParser_Node_Identifier_or_Str>(__m)), None => <Option<U_PhpParser_Node_Identifier_or_Str>>::default() }), (match args.get(4) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Stmt\\TraitUseAdaptation\\Alias", name)))) } }
}
impl Alias { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\TraitUseAdaptation\\Alias could not be converted to string"))) } }
impl php_rt::PhpClone for Alias { fn php_clone(&self) -> Self { let c = Alias(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for AliasObj { fn clone(&self) -> Self { AliasObj { attributes: self.attributes.clone(), trait_: self.trait_.clone(), method: self.method.clone(), newModifier: self.newModifier.clone(), newName: self.newName.clone() } } }
impl Alias {
}
pub struct PrecedenceObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub trait_: Late<Option<crate::php_parser::node::Name>>,
    pub method: Late<crate::php_parser::node::Identifier>,
    pub insteadof: Late<Map<ArrayKey, crate::php_parser::node::Name>>,
}
#[derive(Clone)]
pub struct Precedence(pub Rc<RefCell<PrecedenceObj>>);
impl Precedence {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_trait_(&self) -> Ref<'_, Option<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| o.trait_.get()) }
    pub fn p_trait__get(&self) -> Option<crate::php_parser::node::Name> { self.0.borrow().trait_.get().clone() }
    pub fn p_trait__opt(&self) -> Option<Option<crate::php_parser::node::Name>> { self.0.borrow().trait_.as_option().cloned() }
    pub fn p_trait__mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| o.trait_.get_or_default_mut()) }
    pub fn set_p_trait_(&self, v: Option<crate::php_parser::node::Name>) { self.0.borrow_mut().trait_.set(v); }
    pub fn p_method(&self) -> Ref<'_, crate::php_parser::node::Identifier> { Ref::map(self.0.borrow(), |o| o.method.get()) }
    pub fn p_method_get(&self) -> crate::php_parser::node::Identifier { self.0.borrow().method.get().clone() }
    pub fn p_method_opt(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().method.as_option().cloned() }
    pub fn p_method_mut(&self) -> RefMut<'_, crate::php_parser::node::Identifier> { RefMut::map(self.0.borrow_mut(), |o| o.method.get_mut()) }
    pub fn set_p_method(&self, v: crate::php_parser::node::Identifier) { self.0.borrow_mut().method.set(v); }
    pub fn p_insteadof(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| o.insteadof.get()) }
    pub fn p_insteadof_get(&self) -> Map<ArrayKey, crate::php_parser::node::Name> { self.0.borrow().insteadof.get().clone() }
    pub fn p_insteadof_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Name>> { self.0.borrow().insteadof.as_option().cloned() }
    pub fn p_insteadof_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| o.insteadof.get_or_default_mut()) }
    pub fn set_p_insteadof(&self, v: Map<ArrayKey, crate::php_parser::node::Name>) { self.0.borrow_mut().insteadof.set(v); }
    pub fn new_uninit() -> Precedence {
        Precedence(Rc::new(RefCell::new(PrecedenceObj {
            attributes: Late::uninit(),
            trait_: Late::uninit(),
            method: Late::uninit(),
            insteadof: Late::uninit(),
        })))
    }
    pub fn new(mut trait_: crate::php_parser::node::Name, mut method: U_PhpParser_Node_Identifier_or_Str, mut insteadof: Map<ArrayKey, crate::php_parser::node::Name>, mut attributes: Map<Str, Mixed>) -> Result<Precedence, Throw> {
        let this = Precedence(Rc::new(RefCell::new(PrecedenceObj {
            attributes: Late::uninit(),
            trait_: Late::uninit(),
            method: Late::uninit(),
            insteadof: Late::uninit(),
        })));
        this.magic__construct(trait_, method, insteadof, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut trait_: crate::php_parser::node::Name, mut method: U_PhpParser_Node_Identifier_or_Str, mut insteadof: Map<ArrayKey, crate::php_parser::node::Name>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_trait_(Some(trait_.clone()));
    self.set_p_method((if (match method.clone() { U_PhpParser_Node_Identifier_or_Str::Str(_) => true, U_PhpParser_Node_Identifier_or_Str::Other__(__m) => __m.is_string(), _ => false }) { crate::php_parser::node::Identifier::new(cast::<Str>(method.clone()), Map::<Str, Mixed>::new())? } else { cast::<crate::php_parser::node::Identifier>(method.clone()) }));
    self.set_p_insteadof(insteadof.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("trait"))); __m1.push(cast::<Mixed>(Str::from_static("method"))); __m1.push(cast::<Mixed>(Str::from_static("insteadof"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Stmt_TraitUseAdaptation_Precedence"));
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
    pub fn new_same_class(&self, mut trait_: crate::php_parser::node::Name, mut method: U_PhpParser_Node_Identifier_or_Str, mut insteadof: Map<ArrayKey, crate::php_parser::node::Name>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::stmt::trait_use_adaptation::Precedence, Throw> { Ok(Self::new(trait_, method, insteadof, attributes)?) }
}
impl php_rt::PhpObject for Precedence {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Stmt\\TraitUseAdaptation\\Precedence" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\stmt\\traituseadaptation\\precedence", "phpparser\\node\\stmt\\traituseadaptation", "phpparser\\node\\stmt", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_trait__opt() { out.push((Str::from_static("trait"), cast::<Mixed>(v))); } if let Some(v) = self.p_method_opt() { out.push((Str::from_static("method"), cast::<Mixed>(v))); } if let Some(v) = self.p_insteadof_opt() { out.push((Str::from_static("insteadof"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_trait__opt() { out.push((Str::from_static("trait"), cast::<Mixed>(v))); } if let Some(v) = self.p_method_opt() { out.push((Str::from_static("method"), cast::<Mixed>(v))); } if let Some(v) = self.p_insteadof_opt() { out.push((Str::from_static("insteadof"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "trait" => { self.set_p_trait_(value.to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m))); true }, "method" => { self.set_p_method(cast::<crate::php_parser::node::Identifier>(value)); true }, "insteadof" => { self.set_p_insteadof(cast::<Map<ArrayKey, crate::php_parser::node::Name>>(value)); true }, _ => false } }
    fn get_prop(&self, name: &str) -> Option<Mixed> { match name { "attributes" => self.p_attributes_opt().map(|v| cast::<Mixed>(v)), "trait" => self.p_trait__opt().map(|v| cast::<Mixed>(v)), "method" => self.p_method_opt().map(|v| cast::<Mixed>(v)), "insteadof" => self.p_insteadof_opt().map(|v| cast::<Mixed>(v)), _ => None } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::Name>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::Name") }), (match args.get(1) { Some(__a) => cast::<U_PhpParser_Node_Identifier_or_Str>(__a.clone()), None => unreachable!("no default for U_PhpParser_Node_Identifier_or_Str") }), (match args.get(2) { Some(__a) => cast::<Map<ArrayKey, crate::php_parser::node::Name>>(__a.clone()), None => <Map<ArrayKey, crate::php_parser::node::Name>>::default() }), (match args.get(3) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Node\\Stmt\\TraitUseAdaptation\\Precedence", name)))) } }
}
impl Precedence { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Stmt\\TraitUseAdaptation\\Precedence could not be converted to string"))) } }
impl php_rt::PhpClone for Precedence { fn php_clone(&self) -> Self { let c = Precedence(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for PrecedenceObj { fn clone(&self) -> Self { PrecedenceObj { attributes: self.attributes.clone(), trait_: self.trait_.clone(), method: self.method.clone(), insteadof: self.insteadof.clone() } } }
impl Precedence {
}
impl php_rt::Truthy for Alias { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Alias { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Alias { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\TraitUseAdaptation\\Alias")) } }
impl php_rt::PhpCmp for Alias { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Alias { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Alias { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Alias> for Mixed { fn cast_to(self) -> Alias { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::trait_use_adaptation::Alias>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\TraitUseAdaptation\\Alias") } }
impl php_rt::TryDowncast for Alias { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::trait_use_adaptation::Alias>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Alias { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Alias> for AnyObject { fn cast_to(self) -> Alias { cast::<Alias>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Alias> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\traituseadaptation\\alias") } }
impl php_rt::InstanceOf<Alias> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\traituseadaptation\\alias") } }
impl php_rt::InstanceOf<Alias> for Alias { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Precedence { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Precedence { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Precedence { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Stmt\\TraitUseAdaptation\\Precedence")) } }
impl php_rt::PhpCmp for Precedence { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Precedence { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Precedence { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Precedence> for Mixed { fn cast_to(self) -> Precedence { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::trait_use_adaptation::Precedence>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Stmt\\TraitUseAdaptation\\Precedence") } }
impl php_rt::TryDowncast for Precedence { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::stmt::trait_use_adaptation::Precedence>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Precedence { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Precedence> for AnyObject { fn cast_to(self) -> Precedence { cast::<Precedence>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Precedence> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\stmt\\traituseadaptation\\precedence") } }
impl php_rt::InstanceOf<Precedence> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\stmt\\traituseadaptation\\precedence") } }
impl php_rt::InstanceOf<Precedence> for Precedence { fn is_instance(&self) -> bool { true } }
