use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub struct FullyQualifiedObj {
    pub attributes: Map<Str, Mixed>,
    pub name: Str,
    pub parts: List<Str>,
}
#[derive(Clone)]
pub struct FullyQualified(pub Rc<RefCell<FullyQualifiedObj>>);
impl FullyQualified {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_name(&self) -> Ref<'_, Str> { Ref::map(self.0.borrow(), |o| &o.name) }
    pub fn p_name_get(&self) -> Str { self.0.borrow().name.clone() }
    pub fn p_name_opt(&self) -> Option<Str> { Some(self.0.borrow().name.clone()) }
    pub fn p_name_mut(&self) -> RefMut<'_, Str> { RefMut::map(self.0.borrow_mut(), |o| &mut o.name) }
    pub fn set_p_name(&self, v: Str) { self.0.borrow_mut().name = v; }
    pub fn p_parts(&self) -> Ref<'_, List<Str>> { Ref::map(self.0.borrow(), |o| &o.parts) }
    pub fn p_parts_get(&self) -> List<Str> { self.0.borrow().parts.clone() }
    pub fn p_parts_opt(&self) -> Option<List<Str>> { Some(self.0.borrow().parts.clone()) }
    pub fn p_parts_mut(&self) -> RefMut<'_, List<Str>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.parts) }
    pub fn set_p_parts(&self, v: List<Str>) { self.0.borrow_mut().parts = v; }
    pub fn new(mut name: U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str, mut attributes: Map<Str, Mixed>) -> Result<FullyQualified, Throw> {
        let this = FullyQualified(Rc::new(RefCell::new(FullyQualifiedObj {
            attributes: Default::default(),
            name: Default::default(),
            parts: Default::default(),
        })));
        this.magic__construct(name, attributes)?;
        Ok(this)
    }
    pub fn isUnqualified(&self) -> Result<bool, Throw> {
    return Ok(false);
    }
    pub fn isQualified(&self) -> Result<bool, Throw> {
    return Ok(false);
    }
    pub fn isFullyQualified(&self) -> Result<bool, Throw> {
    return Ok(true);
    }
    pub fn isRelative(&self) -> Result<bool, Throw> {
    return Ok(false);
    }
    pub fn toCodeString(&self) -> Result<Str, Throw> {
    return Ok(concat(Str::from_static("\\"), self.toString()?));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Name_FullyQualified"));
    }
    pub fn magic__construct(&self, mut name: U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).magic__construct__impl(name, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).getSubNodeNames__impl() }
    pub fn getParts(&self) -> Result<List<Str>, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).getParts__impl() }
    pub fn getFirst(&self) -> Result<Str, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).getFirst__impl() }
    pub fn getLast(&self) -> Result<Str, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).getLast__impl() }
    pub fn toString(&self) -> Result<Str, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).toString__impl() }
    pub fn toLowerString(&self) -> Result<Str, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).toLowerString__impl() }
    pub fn isSpecialClassName(&self) -> Result<bool, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).isSpecialClassName__impl() }
    pub fn magic__toString(&self) -> Result<Str, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).magic__toString__impl() }
    pub fn slice(&self, mut offset: i64, mut length: Option<i64>) -> Result<Option<crate::php_parser::node::Name>, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).slice__impl(offset, length) }
    pub fn concat(mut name1: Option<U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str>, mut name2: Option<U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str>, mut attributes: Map<Str, Mixed>) -> Result<Option<crate::php_parser::node::Name>, Throw> { crate::php_parser::node::Name::concat(name1, name2, attributes) }
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
    pub fn new_same_class(&self, mut name: U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::name::FullyQualified, Throw> { Ok(Self::new(name, attributes)?) }
}
impl php_rt::PhpObject for FullyQualified {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Name\\FullyQualified" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\name\\fullyqualified", "phpparser\\node\\name", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable", "stringable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_parts_get()) { out.push((Str::from_static("parts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(cast::<Str>(value)); true }, "parts" => { self.set_p_parts(cast::<List<Str>>(value)); true }, _ => false } }
    fn php_to_string(&self) -> Option<Str> { self.magic__toString().ok() }
}
impl FullyQualified { pub fn to_php_string(&self) -> Result<Str, Throw> { self.magic__toString() } }
impl php_rt::PhpClone for FullyQualified { fn php_clone(&self) -> Self { let c = FullyQualified(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for FullyQualifiedObj { fn clone(&self) -> Self { FullyQualifiedObj { attributes: self.attributes.clone(), name: self.name.clone(), parts: self.parts.clone() } } }
impl FullyQualified {
}
pub struct RelativeObj {
    pub attributes: Map<Str, Mixed>,
    pub name: Str,
    pub parts: List<Str>,
}
#[derive(Clone)]
pub struct Relative(pub Rc<RefCell<RelativeObj>>);
impl Relative {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_name(&self) -> Ref<'_, Str> { Ref::map(self.0.borrow(), |o| &o.name) }
    pub fn p_name_get(&self) -> Str { self.0.borrow().name.clone() }
    pub fn p_name_opt(&self) -> Option<Str> { Some(self.0.borrow().name.clone()) }
    pub fn p_name_mut(&self) -> RefMut<'_, Str> { RefMut::map(self.0.borrow_mut(), |o| &mut o.name) }
    pub fn set_p_name(&self, v: Str) { self.0.borrow_mut().name = v; }
    pub fn p_parts(&self) -> Ref<'_, List<Str>> { Ref::map(self.0.borrow(), |o| &o.parts) }
    pub fn p_parts_get(&self) -> List<Str> { self.0.borrow().parts.clone() }
    pub fn p_parts_opt(&self) -> Option<List<Str>> { Some(self.0.borrow().parts.clone()) }
    pub fn p_parts_mut(&self) -> RefMut<'_, List<Str>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.parts) }
    pub fn set_p_parts(&self, v: List<Str>) { self.0.borrow_mut().parts = v; }
    pub fn new(mut name: U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str, mut attributes: Map<Str, Mixed>) -> Result<Relative, Throw> {
        let this = Relative(Rc::new(RefCell::new(RelativeObj {
            attributes: Default::default(),
            name: Default::default(),
            parts: Default::default(),
        })));
        this.magic__construct(name, attributes)?;
        Ok(this)
    }
    pub fn isUnqualified(&self) -> Result<bool, Throw> {
    return Ok(false);
    }
    pub fn isQualified(&self) -> Result<bool, Throw> {
    return Ok(false);
    }
    pub fn isFullyQualified(&self) -> Result<bool, Throw> {
    return Ok(false);
    }
    pub fn isRelative(&self) -> Result<bool, Throw> {
    return Ok(true);
    }
    pub fn toCodeString(&self) -> Result<Str, Throw> {
    return Ok(concat(Str::from_static("namespace\\"), self.toString()?));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Name_Relative"));
    }
    pub fn magic__construct(&self, mut name: U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).magic__construct__impl(name, attributes) }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).getSubNodeNames__impl() }
    pub fn getParts(&self) -> Result<List<Str>, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).getParts__impl() }
    pub fn getFirst(&self) -> Result<Str, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).getFirst__impl() }
    pub fn getLast(&self) -> Result<Str, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).getLast__impl() }
    pub fn toString(&self) -> Result<Str, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).toString__impl() }
    pub fn toLowerString(&self) -> Result<Str, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).toLowerString__impl() }
    pub fn isSpecialClassName(&self) -> Result<bool, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).isSpecialClassName__impl() }
    pub fn magic__toString(&self) -> Result<Str, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).magic__toString__impl() }
    pub fn slice(&self, mut offset: i64, mut length: Option<i64>) -> Result<Option<crate::php_parser::node::Name>, Throw> { cast::<crate::php_parser::node::Name>(self.clone()).slice__impl(offset, length) }
    pub fn concat(mut name1: Option<U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str>, mut name2: Option<U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str>, mut attributes: Map<Str, Mixed>) -> Result<Option<crate::php_parser::node::Name>, Throw> { crate::php_parser::node::Name::concat(name1, name2, attributes) }
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
    pub fn new_same_class(&self, mut name: U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::name::Relative, Throw> { Ok(Self::new(name, attributes)?) }
}
impl php_rt::PhpObject for Relative {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Name\\Relative" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\name\\relative", "phpparser\\node\\name", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable", "stringable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_parts_get()) { out.push((Str::from_static("parts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(cast::<Str>(value)); true }, "parts" => { self.set_p_parts(cast::<List<Str>>(value)); true }, _ => false } }
    fn php_to_string(&self) -> Option<Str> { self.magic__toString().ok() }
}
impl Relative { pub fn to_php_string(&self) -> Result<Str, Throw> { self.magic__toString() } }
impl php_rt::PhpClone for Relative { fn php_clone(&self) -> Self { let c = Relative(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for RelativeObj { fn clone(&self) -> Self { RelativeObj { attributes: self.attributes.clone(), name: self.name.clone(), parts: self.parts.clone() } } }
impl Relative {
}
impl php_rt::Truthy for FullyQualified { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for FullyQualified { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Name\\FullyQualified")) } }
impl php_rt::Identical for FullyQualified { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for FullyQualified { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for FullyQualified { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for FullyQualified { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<FullyQualified> for Mixed { fn cast_to(self) -> FullyQualified { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::name::FullyQualified>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Name\\FullyQualified") } }
impl php_rt::TryDowncast for FullyQualified { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::name::FullyQualified>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for FullyQualified { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<FullyQualified> for AnyObject { fn cast_to(self) -> FullyQualified { cast::<FullyQualified>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<FullyQualified> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\name\\fullyqualified") } }
impl php_rt::InstanceOf<FullyQualified> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\name\\fullyqualified") } }
impl php_rt::InstanceOf<FullyQualified> for FullyQualified { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Relative { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Relative { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Name\\Relative")) } }
impl php_rt::Identical for Relative { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Relative { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Relative { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Relative { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Relative> for Mixed { fn cast_to(self) -> Relative { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::name::Relative>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Name\\Relative") } }
impl php_rt::TryDowncast for Relative { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::name::Relative>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Relative { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Relative> for AnyObject { fn cast_to(self) -> Relative { cast::<Relative>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Relative> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\name\\relative") } }
impl php_rt::InstanceOf<Relative> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\name\\relative") } }
impl php_rt::InstanceOf<Relative> for Relative { fn is_instance(&self) -> bool { true } }
