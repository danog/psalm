use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub mod magic_const;
pub struct Float_Obj {
    pub attributes: Map<Str, Mixed>,
    pub value: f64,
}
#[derive(Clone)]
pub struct Float_(pub Rc<RefCell<Float_Obj>>);
impl Float_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_value(&self) -> Ref<'_, f64> { Ref::map(self.0.borrow(), |o| &o.value) }
    pub fn p_value_get(&self) -> f64 { self.0.borrow().value.clone() }
    pub fn p_value_opt(&self) -> Option<f64> { Some(self.0.borrow().value.clone()) }
    pub fn p_value_mut(&self) -> RefMut<'_, f64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.value) }
    pub fn set_p_value(&self, v: f64) { self.0.borrow_mut().value = v; }
    pub fn new(mut value: f64, mut attributes: Map<Str, Mixed>) -> Result<Float_, Throw> {
        let this = Float_(Rc::new(RefCell::new(Float_Obj {
            attributes: Default::default(),
            value: Default::default(),
        })));
        this.magic__construct(value, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut value: f64, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_value(value);
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("value"))); __m1 });
    }
    pub fn fromString(mut str: Str, mut attributes: Map<ArrayKey, Mixed>) -> Result<crate::php_parser::node::scalar::Float_, Throw> {
    let mut float: f64 = Default::default();
    attributes.insert(to_key(&Str::from_static("rawValue")), cast::<Mixed>(str.clone()));
    float = crate::php_parser::node::scalar::Float_::parse(str.clone())?;
    return Ok(crate::php_parser::node::scalar::Float_::new(float, attributes.clone().map_entries(|k, v| (cast::<Str>(k), v)))?);
    }
    pub fn parse(mut str: Str) -> Result<f64, Throw> {
    str = str_replace(&Str::from_static("_"), &Str::from_static(""), &str.clone());
    if identical(&Str::from_static("0"), &str_index(&str.clone(), 0i64)) {
        if (identical(&Str::from_static("x"), &str_index(&str.clone(), 1i64)) || identical(&Str::from_static("X"), &str_index(&str.clone(), 1i64))) {
            return Ok((hexdec(&str.clone()) as f64));
        }
        if (identical(&Str::from_static("b"), &str_index(&str.clone(), 1i64)) || identical(&Str::from_static("B"), &str_index(&str.clone(), 1i64))) {
            return Ok((bindec(&str.clone()) as f64));
        }
        if identical(&cast::<Mixed>(false), &cast::<Mixed>(strpbrk(&str.clone(), &Str::from_static(".eE")))) {
            return Ok((octdec(&substr(&str.clone(), 0i64, Some(strcspn(&str.clone(), &Str::from_static("89"))))) as f64));
        }
    }
    return Ok(to_float(&str.clone()));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Scalar_Float"));
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
    pub fn new_same_class(&self, mut value: f64, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::scalar::Float_, Throw> { Ok(Self::new(value, attributes)?) }
}
impl php_rt::PhpObject for Float_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Scalar\\Float_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\scalar\\float_", "phpparser\\node\\scalar", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_value_get()) { out.push((Str::from_static("value"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "value" => { self.set_p_value(cast::<f64>(value)); true }, _ => false } }
}
impl Float_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Scalar\\Float_ could not be converted to string"))) } }
impl php_rt::PhpClone for Float_ { fn php_clone(&self) -> Self { let c = Float_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Float_Obj { fn clone(&self) -> Self { Float_Obj { attributes: self.attributes.clone(), value: self.value.clone() } } }
impl Float_ {
}
pub struct InterpolatedStringObj {
    pub attributes: Map<Str, Mixed>,
    pub parts: Map<ArrayKey, U_PhpParser_Node_Expr_or_PhpParser_Node_InterpolatedStringPart>,
}
#[derive(Clone)]
pub struct InterpolatedString(pub Rc<RefCell<InterpolatedStringObj>>);
impl InterpolatedString {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_parts(&self) -> Ref<'_, Map<ArrayKey, U_PhpParser_Node_Expr_or_PhpParser_Node_InterpolatedStringPart>> { Ref::map(self.0.borrow(), |o| &o.parts) }
    pub fn p_parts_get(&self) -> Map<ArrayKey, U_PhpParser_Node_Expr_or_PhpParser_Node_InterpolatedStringPart> { self.0.borrow().parts.clone() }
    pub fn p_parts_opt(&self) -> Option<Map<ArrayKey, U_PhpParser_Node_Expr_or_PhpParser_Node_InterpolatedStringPart>> { Some(self.0.borrow().parts.clone()) }
    pub fn p_parts_mut(&self) -> RefMut<'_, Map<ArrayKey, U_PhpParser_Node_Expr_or_PhpParser_Node_InterpolatedStringPart>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.parts) }
    pub fn set_p_parts(&self, v: Map<ArrayKey, U_PhpParser_Node_Expr_or_PhpParser_Node_InterpolatedStringPart>) { self.0.borrow_mut().parts = v; }
    pub fn new(mut parts: Map<ArrayKey, U_PhpParser_Node_Expr_or_PhpParser_Node_InterpolatedStringPart>, mut attributes: Map<Str, Mixed>) -> Result<InterpolatedString, Throw> {
        let this = InterpolatedString(Rc::new(RefCell::new(InterpolatedStringObj {
            attributes: Default::default(),
            parts: Default::default(),
        })));
        this.magic__construct(parts, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut parts: Map<ArrayKey, U_PhpParser_Node_Expr_or_PhpParser_Node_InterpolatedStringPart>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_parts(parts.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("parts"))); __m1 });
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Scalar_InterpolatedString"));
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
    pub fn new_same_class(&self, mut parts: Map<ArrayKey, U_PhpParser_Node_Expr_or_PhpParser_Node_InterpolatedStringPart>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::scalar::InterpolatedString, Throw> { Ok(Self::new(parts, attributes)?) }
}
impl php_rt::PhpObject for InterpolatedString {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Scalar\\InterpolatedString" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\scalar\\interpolatedstring", "phpparser\\node\\scalar", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_parts_get()) { out.push((Str::from_static("parts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "parts" => { self.set_p_parts(cast::<Map<ArrayKey, U_PhpParser_Node_Expr_or_PhpParser_Node_InterpolatedStringPart>>(value)); true }, _ => false } }
}
impl InterpolatedString { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Scalar\\InterpolatedString could not be converted to string"))) } }
impl php_rt::PhpClone for InterpolatedString { fn php_clone(&self) -> Self { let c = InterpolatedString(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for InterpolatedStringObj { fn clone(&self) -> Self { InterpolatedStringObj { attributes: self.attributes.clone(), parts: self.parts.clone() } } }
impl InterpolatedString {
}
pub struct Int_Obj {
    pub attributes: Map<Str, Mixed>,
    pub value: i64,
}
#[derive(Clone)]
pub struct Int_(pub Rc<RefCell<Int_Obj>>);
impl Int_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_value(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.value) }
    pub fn p_value_get(&self) -> i64 { self.0.borrow().value.clone() }
    pub fn p_value_opt(&self) -> Option<i64> { Some(self.0.borrow().value.clone()) }
    pub fn p_value_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.value) }
    pub fn set_p_value(&self, v: i64) { self.0.borrow_mut().value = v; }
    pub fn new(mut value: i64, mut attributes: Map<Str, Mixed>) -> Result<Int_, Throw> {
        let this = Int_(Rc::new(RefCell::new(Int_Obj {
            attributes: Default::default(),
            value: Default::default(),
        })));
        this.magic__construct(value, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut value: i64, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    self.set_p_value(value);
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("value"))); __m1 });
    }
    pub fn fromString(mut str: Str, mut attributes: Map<Str, Mixed>, mut allowInvalidOctal: bool) -> Result<crate::php_parser::node::scalar::Int_, Throw> {
    attributes.insert(Str::from_static("rawValue"), cast::<Mixed>(str.clone()));
    str = str_replace(&Str::from_static("_"), &Str::from_static(""), &str.clone());
    if ((!identical(&Str::from_static("0"), &str_index(&str.clone(), 0i64))) || identical(&Str::from_static("0"), &str.clone())) {
        attributes.insert(Str::from_static("kind"), cast::<Mixed>(crate::php_parser::node::scalar::Int_::KIND_DEC()));
        return Ok(crate::php_parser::node::scalar::Int_::new(to_int(&str.clone()), attributes.clone())?);
    }
    if (identical(&Str::from_static("x"), &str_index(&str.clone(), 1i64)) || identical(&Str::from_static("X"), &str_index(&str.clone(), 1i64))) {
        attributes.insert(Str::from_static("kind"), cast::<Mixed>(crate::php_parser::node::scalar::Int_::KIND_HEX()));
        return Ok(crate::php_parser::node::scalar::Int_::new(hexdec(&str.clone()), attributes.clone())?);
    }
    if (identical(&Str::from_static("b"), &str_index(&str.clone(), 1i64)) || identical(&Str::from_static("B"), &str_index(&str.clone(), 1i64))) {
        attributes.insert(Str::from_static("kind"), cast::<Mixed>(crate::php_parser::node::scalar::Int_::KIND_BIN()));
        return Ok(crate::php_parser::node::scalar::Int_::new(bindec(&str.clone()), attributes.clone())?);
    }
    if ((!allowInvalidOctal) && truthy(&strpbrk(&str.clone(), &Str::from_static("89")))) {
        return Err(cast::<crate::g::Throwable>(crate::php_parser::Error::new(Str::from_static("Invalid numeric literal"), attributes.clone())?));
    }
    if (identical(&Str::from_static("o"), &str_index(&str.clone(), 1i64)) || identical(&Str::from_static("O"), &str_index(&str.clone(), 1i64))) {
        str = substr(&str.clone(), 2i64, None);
    }
    attributes.insert(Str::from_static("kind"), cast::<Mixed>(crate::php_parser::node::scalar::Int_::KIND_OCT()));
    return Ok(crate::php_parser::node::scalar::Int_::new(intval(&str.clone(), 8i64), attributes.clone())?);
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Scalar_Int"));
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
    pub fn new_same_class(&self, mut value: i64, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::scalar::Int_, Throw> { Ok(Self::new(value, attributes)?) }
}
impl php_rt::PhpObject for Int_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Scalar\\Int_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\scalar\\int_", "phpparser\\node\\scalar", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_value_get()) { out.push((Str::from_static("value"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "value" => { self.set_p_value(cast::<i64>(value)); true }, _ => false } }
}
impl Int_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Scalar\\Int_ could not be converted to string"))) } }
impl php_rt::PhpClone for Int_ { fn php_clone(&self) -> Self { let c = Int_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Int_Obj { fn clone(&self) -> Self { Int_Obj { attributes: self.attributes.clone(), value: self.value.clone() } } }
impl Int_ {
    pub fn KIND_BIN() -> i64 { 2i64 }
    pub fn KIND_OCT() -> i64 { 8i64 }
    pub fn KIND_DEC() -> i64 { 10i64 }
    pub fn KIND_HEX() -> i64 { 16i64 }
}
#[derive(Clone)]
pub enum MagicConst {
    PhpParser_Node_Scalar_MagicConst_Class_(crate::php_parser::node::scalar::magic_const::Class_),
    PhpParser_Node_Scalar_MagicConst_Dir(crate::php_parser::node::scalar::magic_const::Dir),
    PhpParser_Node_Scalar_MagicConst_File(crate::php_parser::node::scalar::magic_const::File),
    PhpParser_Node_Scalar_MagicConst_Function_(crate::php_parser::node::scalar::magic_const::Function_),
    PhpParser_Node_Scalar_MagicConst_Line(crate::php_parser::node::scalar::magic_const::Line),
    PhpParser_Node_Scalar_MagicConst_Method(crate::php_parser::node::scalar::magic_const::Method),
    PhpParser_Node_Scalar_MagicConst_Namespace_(crate::php_parser::node::scalar::magic_const::Namespace_),
    PhpParser_Node_Scalar_MagicConst_Property(crate::php_parser::node::scalar::magic_const::Property),
    PhpParser_Node_Scalar_MagicConst_Trait_(crate::php_parser::node::scalar::magic_const::Trait_),
}
impl php_rt::PhpObject for MagicConst {
    fn class_name(&self) -> &'static str { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => __h.class_name(), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => __h.class_name(), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => __h.class_name(), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => __h.class_name(), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => __h.class_name(), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => __h.class_name(), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => __h.class_name(), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => __h.class_name(), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => __h.class_name(), _ => unreachable!() } }
    fn class_ancestors(&self) -> &'static [&'static str] { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => __h.class_ancestors(), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => __h.class_ancestors(), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => __h.class_ancestors(), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => __h.class_ancestors(), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => __h.class_ancestors(), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => __h.class_ancestors(), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => __h.class_ancestors(), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => __h.class_ancestors(), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => __h.class_ancestors(), _ => unreachable!() } }
    fn obj_id(&self) -> usize { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => __h.obj_id(), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => __h.obj_id(), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => __h.obj_id(), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => __h.obj_id(), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => __h.obj_id(), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => __h.obj_id(), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => __h.obj_id(), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => __h.obj_id(), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => __h.obj_id(), _ => unreachable!() } }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => __h.props(), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => __h.props(), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => __h.props(), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => __h.props(), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => __h.props(), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => __h.props(), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => __h.props(), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => __h.props(), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => __h.props(), _ => unreachable!() } }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => __h.set_prop(name, value), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => __h.set_prop(name, value), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => __h.set_prop(name, value), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => __h.set_prop(name, value), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => __h.set_prop(name, value), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => __h.set_prop(name, value), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => __h.set_prop(name, value), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => __h.set_prop(name, value), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => __h.set_prop(name, value), _ => unreachable!() } }
    fn php_to_string(&self) -> Option<Str> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => __h.php_to_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => __h.php_to_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => __h.php_to_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => __h.php_to_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => __h.php_to_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => __h.php_to_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => __h.php_to_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => __h.php_to_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => __h.php_to_string(), _ => unreachable!() } }
}
impl MagicConst { pub fn to_php_string(&self) -> Result<Str, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => __h.to_php_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => __h.to_php_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => __h.to_php_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => __h.to_php_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => __h.to_php_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => __h.to_php_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => __h.to_php_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => __h.to_php_string(), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => __h.to_php_string(), _ => unreachable!() } } }
impl php_rt::PhpClone for MagicConst { fn php_clone(&self) -> Self { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h.php_clone()), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h.php_clone()), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h.php_clone()), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h.php_clone()), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h.php_clone()), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h.php_clone()), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h.php_clone()), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h.php_clone()), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h.php_clone()), _ => unreachable!() } } }
impl MagicConst {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => __h.p_attributes(), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => __h.p_attributes(), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => __h.p_attributes(), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => __h.p_attributes(), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => __h.p_attributes(), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => __h.p_attributes(), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => __h.p_attributes(), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => __h.p_attributes(), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => __h.p_attributes(), _ => unreachable!() } }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => __h.p_attributes_get(), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => __h.p_attributes_get(), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => __h.p_attributes_get(), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => __h.p_attributes_get(), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => __h.p_attributes_get(), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => __h.p_attributes_get(), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => __h.p_attributes_get(), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => __h.p_attributes_get(), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => __h.p_attributes_get(), _ => unreachable!() } }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => __h.p_attributes_opt(), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => __h.p_attributes_opt(), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => __h.p_attributes_opt(), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => __h.p_attributes_opt(), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => __h.p_attributes_opt(), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => __h.p_attributes_opt(), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => __h.p_attributes_opt(), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => __h.p_attributes_opt(), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => __h.p_attributes_opt(), _ => unreachable!() } }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => __h.p_attributes_mut(), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => __h.p_attributes_mut(), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => __h.p_attributes_mut(), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => __h.p_attributes_mut(), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => __h.p_attributes_mut(), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => __h.p_attributes_mut(), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => __h.p_attributes_mut(), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => __h.p_attributes_mut(), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => __h.p_attributes_mut(), _ => unreachable!() } }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => __h.set_p_attributes(v), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => __h.set_p_attributes(v), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => __h.set_p_attributes(v), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => __h.set_p_attributes(v), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => __h.set_p_attributes(v), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => __h.set_p_attributes(v), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => __h.set_p_attributes(v), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => __h.set_p_attributes(v), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => __h.set_p_attributes(v), _ => unreachable!() } }
    pub fn magic__construct(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.magic__construct(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.magic__construct(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.magic__construct(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.magic__construct(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.magic__construct(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.magic__construct(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.magic__construct(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.magic__construct(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.magic__construct(attributes)?), _ => unreachable!() } }
    pub fn magic__construct__impl(&self, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_attributes(attributes.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.getSubNodeNames()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.getSubNodeNames()?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.getSubNodeNames()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.getSubNodeNames()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.getSubNodeNames()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.getSubNodeNames()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.getSubNodeNames()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.getSubNodeNames()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.getSubNodeNames()?), _ => unreachable!() } }
    pub fn getSubNodeNames__impl(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok(Map::<ArrayKey, Mixed>::new());
    }
    pub fn getName(&self) -> Result<Str, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.getName()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.getName()?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.getName()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.getName()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.getName()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.getName()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.getName()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.getName()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.getName()?), _ => unreachable!() } }
    pub fn getLine(&self) -> Result<i64, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.getLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.getLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.getLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.getLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.getLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.getLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.getLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.getLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.getLine()?), _ => unreachable!() } }
    pub fn getLine__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getLine__impl() }
    pub fn getStartLine(&self) -> Result<i64, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.getStartLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.getStartLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.getStartLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.getStartLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.getStartLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.getStartLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.getStartLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.getStartLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.getStartLine()?), _ => unreachable!() } }
    pub fn getStartLine__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartLine__impl() }
    pub fn getEndLine(&self) -> Result<i64, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.getEndLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.getEndLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.getEndLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.getEndLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.getEndLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.getEndLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.getEndLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.getEndLine()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.getEndLine()?), _ => unreachable!() } }
    pub fn getEndLine__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndLine__impl() }
    pub fn getStartTokenPos(&self) -> Result<i64, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.getStartTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.getStartTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.getStartTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.getStartTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.getStartTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.getStartTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.getStartTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.getStartTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.getStartTokenPos()?), _ => unreachable!() } }
    pub fn getStartTokenPos__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartTokenPos__impl() }
    pub fn getEndTokenPos(&self) -> Result<i64, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.getEndTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.getEndTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.getEndTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.getEndTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.getEndTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.getEndTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.getEndTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.getEndTokenPos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.getEndTokenPos()?), _ => unreachable!() } }
    pub fn getEndTokenPos__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndTokenPos__impl() }
    pub fn getStartFilePos(&self) -> Result<i64, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.getStartFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.getStartFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.getStartFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.getStartFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.getStartFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.getStartFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.getStartFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.getStartFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.getStartFilePos()?), _ => unreachable!() } }
    pub fn getStartFilePos__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getStartFilePos__impl() }
    pub fn getEndFilePos(&self) -> Result<i64, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.getEndFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.getEndFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.getEndFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.getEndFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.getEndFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.getEndFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.getEndFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.getEndFilePos()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.getEndFilePos()?), _ => unreachable!() } }
    pub fn getEndFilePos__impl(&self) -> Result<i64, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getEndFilePos__impl() }
    pub fn getComments(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.getComments()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.getComments()?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.getComments()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.getComments()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.getComments()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.getComments()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.getComments()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.getComments()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.getComments()?), _ => unreachable!() } }
    pub fn getComments__impl(&self) -> Result<Map<ArrayKey, crate::php_parser::Comment>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getComments__impl() }
    pub fn getDocComment(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.getDocComment()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.getDocComment()?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.getDocComment()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.getDocComment()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.getDocComment()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.getDocComment()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.getDocComment()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.getDocComment()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.getDocComment()?), _ => unreachable!() } }
    pub fn getDocComment__impl(&self) -> Result<Option<crate::php_parser::comment::Doc>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getDocComment__impl() }
    pub fn setDocComment(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.setDocComment(docComment)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.setDocComment(docComment)?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.setDocComment(docComment)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.setDocComment(docComment)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.setDocComment(docComment)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.setDocComment(docComment)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.setDocComment(docComment)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.setDocComment(docComment)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.setDocComment(docComment)?), _ => unreachable!() } }
    pub fn setDocComment__impl(&self, mut docComment: crate::php_parser::comment::Doc) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setDocComment__impl(docComment) }
    pub fn setAttribute(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.setAttribute(key_v, value)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.setAttribute(key_v, value)?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.setAttribute(key_v, value)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.setAttribute(key_v, value)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.setAttribute(key_v, value)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.setAttribute(key_v, value)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.setAttribute(key_v, value)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.setAttribute(key_v, value)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.setAttribute(key_v, value)?), _ => unreachable!() } }
    pub fn setAttribute__impl(&self, mut key_v: Str, mut value: Mixed) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttribute__impl(key_v, value) }
    pub fn hasAttribute(&self, mut key_v: Str) -> Result<bool, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.hasAttribute(key_v)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.hasAttribute(key_v)?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.hasAttribute(key_v)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.hasAttribute(key_v)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.hasAttribute(key_v)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.hasAttribute(key_v)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.hasAttribute(key_v)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.hasAttribute(key_v)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.hasAttribute(key_v)?), _ => unreachable!() } }
    pub fn hasAttribute__impl(&self, mut key_v: Str) -> Result<bool, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).hasAttribute__impl(key_v) }
    pub fn getAttribute(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.getAttribute(key_v, default)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.getAttribute(key_v, default)?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.getAttribute(key_v, default)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.getAttribute(key_v, default)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.getAttribute(key_v, default)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.getAttribute(key_v, default)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.getAttribute(key_v, default)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.getAttribute(key_v, default)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.getAttribute(key_v, default)?), _ => unreachable!() } }
    pub fn getAttribute__impl(&self, mut key_v: Str, mut default: Mixed) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttribute__impl(key_v, default) }
    pub fn getAttributes(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.getAttributes()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.getAttributes()?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.getAttributes()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.getAttributes()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.getAttributes()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.getAttributes()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.getAttributes()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.getAttributes()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.getAttributes()?), _ => unreachable!() } }
    pub fn getAttributes__impl(&self) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).getAttributes__impl() }
    pub fn setAttributes(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.setAttributes(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.setAttributes(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.setAttributes(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.setAttributes(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.setAttributes(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.setAttributes(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.setAttributes(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.setAttributes(attributes)?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.setAttributes(attributes)?), _ => unreachable!() } }
    pub fn setAttributes__impl(&self, mut attributes: Map<ArrayKey, Mixed>) -> Result<(), Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).setAttributes__impl(attributes) }
    pub fn jsonSerialize(&self) -> Result<Map<Str, Mixed>, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.jsonSerialize()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.jsonSerialize()?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.jsonSerialize()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.jsonSerialize()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.jsonSerialize()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.jsonSerialize()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.jsonSerialize()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.jsonSerialize()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.jsonSerialize()?), _ => unreachable!() } }
    pub fn jsonSerialize__impl(&self) -> Result<Map<Str, Mixed>, Throw> { cast::<crate::php_parser::NodeAbstract>(self.clone()).jsonSerialize__impl() }
    pub fn getType(&self) -> Result<Str, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(__h.getType()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(__h.getType()?), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(__h.getType()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(__h.getType()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(__h.getType()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(__h.getType()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(__h.getType()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(__h.getType()?), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(__h.getType()?), _ => unreachable!() } }
    pub fn new_same_class(&self, mut attributes: Map<Str, Mixed>) -> Result<MagicConst, Throw> { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(__h) => Ok(cast::<crate::php_parser::node::scalar::MagicConst>(__h.new_same_class(attributes.clone())?)), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(__h) => Ok(cast::<crate::php_parser::node::scalar::MagicConst>(__h.new_same_class(attributes.clone())?)), MagicConst::PhpParser_Node_Scalar_MagicConst_File(__h) => Ok(cast::<crate::php_parser::node::scalar::MagicConst>(__h.new_same_class(attributes.clone())?)), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(__h) => Ok(cast::<crate::php_parser::node::scalar::MagicConst>(__h.new_same_class(attributes.clone())?)), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(__h) => Ok(cast::<crate::php_parser::node::scalar::MagicConst>(__h.new_same_class(attributes.clone())?)), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(__h) => Ok(cast::<crate::php_parser::node::scalar::MagicConst>(__h.new_same_class(attributes.clone())?)), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(__h) => Ok(cast::<crate::php_parser::node::scalar::MagicConst>(__h.new_same_class(attributes.clone())?)), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(__h) => Ok(cast::<crate::php_parser::node::scalar::MagicConst>(__h.new_same_class(attributes.clone())?)), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(__h) => Ok(cast::<crate::php_parser::node::scalar::MagicConst>(__h.new_same_class(attributes.clone())?)), _ => unreachable!() } }
}
pub struct String_Obj {
    pub attributes: Map<Str, Mixed>,
    pub value: Str,
}
#[derive(Clone)]
pub struct String_(pub Rc<RefCell<String_Obj>>);
impl String_ {
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
    pub fn new(mut value: Str, mut attributes: Map<Str, Mixed>) -> Result<String_, Throw> {
        let this = String_(Rc::new(RefCell::new(String_Obj {
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
    pub fn fromString(mut str: Str, mut attributes: Map<Str, Mixed>, mut parseUnicodeEscape: bool) -> Result<crate::php_parser::node::scalar::String_, Throw> {
    let mut string: Str = Default::default();
    attributes.insert(Str::from_static("kind"), cast::<Mixed>((if (identical(&str_index(&str.clone(), 0i64), &Str::from_static("'")) || (identical(&str_index(&str.clone(), 1i64), &Str::from_static("'")) && (identical(&str_index(&str.clone(), 0i64), &Str::from_static("b")) || identical(&str_index(&str.clone(), 0i64), &Str::from_static("B"))))) { crate::php_parser::node::scalar::String_::KIND_SINGLE_QUOTED() } else { crate::php_parser::node::scalar::String_::KIND_DOUBLE_QUOTED() })));
    attributes.insert(Str::from_static("rawValue"), cast::<Mixed>(str.clone()));
    string = crate::php_parser::node::scalar::String_::parse(str.clone(), parseUnicodeEscape)?;
    return Ok(crate::php_parser::node::scalar::String_::new(string.clone(), attributes.clone())?);
    }
    pub fn parse(mut str: Str, mut parseUnicodeEscape: bool) -> Result<Str, Throw> {
    let mut bLength: i64 = Default::default();
    bLength = 0i64;
    if (identical(&Str::from_static("b"), &str_index(&str.clone(), 0i64)) || identical(&Str::from_static("B"), &str_index(&str.clone(), 0i64))) {
        bLength = 1i64;
    }
    if identical(&Str::from_static("'"), &str_index(&str.clone(), bLength)) {
        return Ok(str_replace_arr(&{ let __c53 = (Str::from_static("\\\\"), Str::from_static("\\'")); List::from_vec(vec![__c53.0, __c53.1]) }, &{ let __c54 = (Str::from_static("\\"), Str::from_static("'")); List::from_vec(vec![__c54.0, __c54.1]) }, &substr(&str.clone(), (bLength).wrapping_add(1i64), Some((1i64).wrapping_neg()))));
    } else {
        return Ok(crate::php_parser::node::scalar::String_::parseEscapeSequences(substr(&str.clone(), (bLength).wrapping_add(1i64), Some((1i64).wrapping_neg())), Some(Str::from_static("\"")), parseUnicodeEscape)?);
    }
    #[allow(unreachable_code)] unreachable!("missing return")
    }
    pub fn parseEscapeSequences(mut str: Str, mut quote: Option<Str>, mut parseUnicodeEscape: bool) -> Result<Str, Throw> {
    let mut extra: Str = Default::default();
    if (!quote.clone().is_none()) {
        str = str_replace(&concat(Str::from_static("\\"), quote.clone().unwrap()), &quote.clone().unwrap(), &str.clone());
    }
    extra = Str::from_static("");
    if parseUnicodeEscape {
        extra = Str::from_static("|u\\{([0-9a-fA-F]+)\\}");
    }
    return Ok(preg_replace_callback(&concat(concat(Str::from_static("~\\\\([\\\\$nrtfve]|[xX][0-9a-fA-F]{1,2}|[0-7]{1,3}"), extra.clone()), Str::from_static(")~")), &str.clone(), -1, { let __f = { let __c55 = {  Rc::new(move |mut matches: Map<ArrayKey, Str>| -> Result<Mixed, Throw> { 
    let mut str: Str = Default::default();
    let mut dec: Late<U_Float_or_Int> = Late::uninit();
    str = matches.clone().idx(&to_key(&1i64)).clone();
    if ({ let __k = str.clone(); Some(crate::php_parser::node::scalar::String_::st_replacements()).and_then(|__b| __b.get(&__k).cloned()) }.is_some()) {
        return Ok(cast::<Mixed>(crate::php_parser::node::scalar::String_::st_replacements().idx(&str.clone()).clone()));
    }
    if (identical(&Str::from_static("x"), &str_index(&str.clone(), 0i64)) || identical(&Str::from_static("X"), &str_index(&str.clone(), 0i64))) {
        return Ok(cast::<Mixed>(chr(hexdec(&substr(&str.clone(), 1i64, None)))));
    }
    if identical(&Str::from_static("u"), &str_index(&str.clone(), 0i64)) {
        dec.set(U_Float_or_Int::Int(hexdec(&matches.clone().idx(&to_key(&2i64)).clone())));
        return Ok(cast::<Mixed>(crate::php_parser::node::scalar::String_::codePointToUtf8((if { let _ = cast::<i64>(dec.get().clone()); true } { cast::<i64>(dec.get().clone()) } else { consts::PHP_INT_MAX }))?));
    } else {
        return Ok(cast::<Mixed>(chr((octdec(&str.clone()) & 255i64))));
    }
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }) as Rc<dyn Fn(Map<ArrayKey, Str>) -> Result<Mixed, Throw>> }; Rc::new(move |__p0: Map<ArrayKey, Str>| -> Result<Str, Throw> { Ok(cast::<Str>(__c55(__p0)?)) }) as Rc<dyn Fn(Map<ArrayKey, Str>) -> Result<Str, Throw>> }; move |__m: Map<ArrayKey, Str>| __f(__m) })?);
    }
    pub fn codePointToUtf8(mut num: i64) -> Result<Str, Throw> {
    if (num <= 127i64) {
        return Ok(chr(num));
    }
    if (num <= 2047i64) {
        return Ok(concat(chr(((num).wrapping_shr((6i64) as u32)).wrapping_add(192i64)), chr(((num & 63i64)).wrapping_add(128i64))));
    }
    if (num <= 65535i64) {
        return Ok(concat(concat(chr(((num).wrapping_shr((12i64) as u32)).wrapping_add(224i64)), chr((((num).wrapping_shr((6i64) as u32) & 63i64)).wrapping_add(128i64))), chr(((num & 63i64)).wrapping_add(128i64))));
    }
    if (num <= 2097151i64) {
        return Ok(concat(concat(concat(chr(((num).wrapping_shr((18i64) as u32)).wrapping_add(240i64)), chr((((num).wrapping_shr((12i64) as u32) & 63i64)).wrapping_add(128i64))), chr((((num).wrapping_shr((6i64) as u32) & 63i64)).wrapping_add(128i64))), chr(((num & 63i64)).wrapping_add(128i64))));
    }
    return Err(cast::<crate::g::Throwable>(crate::php_parser::Error::new(Str::from_static("Invalid UTF-8 codepoint escape sequence: Codepoint too large"), Map::<Str, Mixed>::new())?));
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Scalar_String"));
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
    pub fn new_same_class(&self, mut value: Str, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::node::scalar::String_, Throw> { Ok(Self::new(value, attributes)?) }
}
impl php_rt::PhpObject for String_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Node\\Scalar\\String_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\node\\scalar\\string_", "phpparser\\node\\scalar", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_value_get()) { out.push((Str::from_static("value"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "value" => { self.set_p_value(cast::<Str>(value)); true }, _ => false } }
}
impl String_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Node\\Scalar\\String_ could not be converted to string"))) } }
impl php_rt::PhpClone for String_ { fn php_clone(&self) -> Self { let c = String_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for String_Obj { fn clone(&self) -> Self { String_Obj { attributes: self.attributes.clone(), value: self.value.clone() } } }
impl String_ {
    pub fn st_replacements_cell() -> &'static std::thread::LocalKey<RefCell<Map<Str, Str>>> { thread_local! { static CELL: RefCell<Map<Str, Str>> = RefCell::new({ let mut __m: Map<Str, Str> = Map::new(); __m.insert(Str::from_static("\\"), Str::from_static("\\")); __m.insert(Str::from_static("$"), Str::from_static("$")); __m.insert(Str::from_static("n"), Str::from_static("\n")); __m.insert(Str::from_static("r"), Str::from_static("\r")); __m.insert(Str::from_static("t"), Str::from_static("\t")); __m.insert(Str::from_static("f"), Str::from_static("\u{c}")); __m.insert(Str::from_static("v"), Str::from_static("\u{b}")); __m.insert(Str::from_static("e"), Str::from_static("\u{1b}")); __m }); } &CELL }
    pub fn st_replacements() -> Map<Str, Str> { Self::st_replacements_cell().with(|c| c.borrow().clone()) }
    pub fn st_replacements_set(v: Map<Str, Str>) { Self::st_replacements_cell().with(|c| { *c.borrow_mut() = v; }) }
    pub fn st_replacements_with<R>(f: impl FnOnce(&mut Map<Str, Str>) -> R) -> R { Self::st_replacements_cell().with(|c| f(&mut *c.borrow_mut())) }
    pub fn KIND_SINGLE_QUOTED() -> i64 { 1i64 }
    pub fn KIND_DOUBLE_QUOTED() -> i64 { 2i64 }
    pub fn KIND_HEREDOC() -> i64 { 3i64 }
    pub fn KIND_NOWDOC() -> i64 { 4i64 }
}
impl php_rt::Truthy for Float_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Float_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Scalar\\Float_")) } }
impl php_rt::Identical for Float_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Float_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Float_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Float_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Float_> for Mixed { fn cast_to(self) -> Float_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::Float_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Scalar\\Float_") } }
impl php_rt::TryDowncast for Float_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::Float_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Float_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Float_> for AnyObject { fn cast_to(self) -> Float_ { cast::<Float_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Float_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\scalar\\float_") } }
impl php_rt::InstanceOf<Float_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\scalar\\float_") } }
impl php_rt::InstanceOf<Float_> for Float_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for InterpolatedString { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for InterpolatedString { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Scalar\\InterpolatedString")) } }
impl php_rt::Identical for InterpolatedString { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for InterpolatedString { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for InterpolatedString { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for InterpolatedString { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<InterpolatedString> for Mixed { fn cast_to(self) -> InterpolatedString { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::InterpolatedString>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Scalar\\InterpolatedString") } }
impl php_rt::TryDowncast for InterpolatedString { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::InterpolatedString>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for InterpolatedString { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<InterpolatedString> for AnyObject { fn cast_to(self) -> InterpolatedString { cast::<InterpolatedString>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<InterpolatedString> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\scalar\\interpolatedstring") } }
impl php_rt::InstanceOf<InterpolatedString> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\scalar\\interpolatedstring") } }
impl php_rt::InstanceOf<InterpolatedString> for InterpolatedString { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Int_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Int_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Scalar\\Int_")) } }
impl php_rt::Identical for Int_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Int_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Int_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Int_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Int_> for Mixed { fn cast_to(self) -> Int_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::Int_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Scalar\\Int_") } }
impl php_rt::TryDowncast for Int_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::Int_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Int_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Int_> for AnyObject { fn cast_to(self) -> Int_ { cast::<Int_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Int_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\scalar\\int_") } }
impl php_rt::InstanceOf<Int_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\scalar\\int_") } }
impl php_rt::InstanceOf<Int_> for Int_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for MagicConst { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for MagicConst { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Scalar\\MagicConst")) } }
impl php_rt::Identical for MagicConst { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for MagicConst { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for MagicConst { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for MagicConst { fn cast_to(self) -> Mixed { match self { MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(v) => Mixed::Obj(Rc::new(v)), MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(v) => Mixed::Obj(Rc::new(v)), MagicConst::PhpParser_Node_Scalar_MagicConst_File(v) => Mixed::Obj(Rc::new(v)), MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(v) => Mixed::Obj(Rc::new(v)), MagicConst::PhpParser_Node_Scalar_MagicConst_Line(v) => Mixed::Obj(Rc::new(v)), MagicConst::PhpParser_Node_Scalar_MagicConst_Method(v) => Mixed::Obj(Rc::new(v)), MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(v) => Mixed::Obj(Rc::new(v)), MagicConst::PhpParser_Node_Scalar_MagicConst_Property(v) => Mixed::Obj(Rc::new(v)), MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(v) => Mixed::Obj(Rc::new(v)), _ => unreachable!() } } }
impl php_rt::CastTo<MagicConst> for Mixed { fn cast_to(self) -> MagicConst { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Class_>() { return crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Dir>() { return crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::File>() { return crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_File(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Function_>() { return crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Line>() { return crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Line(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Method>() { return crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Method(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Namespace_>() { return crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Property>() { return crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Property(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Trait_>() { return crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(v.clone()); } } panic!("Mixed value is not a PhpParser\\Node\\Scalar\\MagicConst") } }
impl php_rt::TryDowncast for MagicConst { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Class_>() { return Some(crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Class_(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Dir>() { return Some(crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Dir(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::File>() { return Some(crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_File(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Function_>() { return Some(crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Function_(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Line>() { return Some(crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Line(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Method>() { return Some(crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Method(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Namespace_>() { return Some(crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Namespace_(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Property>() { return Some(crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Property(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::magic_const::Trait_>() { return Some(crate::php_parser::node::scalar::MagicConst::PhpParser_Node_Scalar_MagicConst_Trait_(v.clone())); } None } }
impl php_rt::CastTo<AnyObject> for MagicConst { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<MagicConst> for AnyObject { fn cast_to(self) -> MagicConst { cast::<MagicConst>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<MagicConst> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\scalar\\magicconst") } }
impl php_rt::InstanceOf<MagicConst> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\scalar\\magicconst") } }
impl php_rt::InstanceOf<MagicConst> for MagicConst { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for String_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for String_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Node\\Scalar\\String_")) } }
impl php_rt::Identical for String_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for String_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for String_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for String_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<String_> for Mixed { fn cast_to(self) -> String_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::String_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Node\\Scalar\\String_") } }
impl php_rt::TryDowncast for String_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node::scalar::String_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for String_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<String_> for AnyObject { fn cast_to(self) -> String_ { cast::<String_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<String_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\node\\scalar\\string_") } }
impl php_rt::InstanceOf<String_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\node\\scalar\\string_") } }
impl php_rt::InstanceOf<String_> for String_ { fn is_instance(&self) -> bool { true } }
