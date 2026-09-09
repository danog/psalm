use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub struct ClassConstObj {
    pub flags: i64,
    pub attributes: Map<Str, Mixed>,
    pub constants: List<crate::php_parser::node::Const_>,
    pub attributeGroups: List<crate::php_parser::node::AttributeGroup>,
    pub type_: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>,
}
#[derive(Clone)]
pub struct ClassConst(pub Rc<RefCell<ClassConstObj>>);
impl ClassConst {
    pub fn p_flags(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.flags) }
    pub fn p_flags_get(&self) -> i64 { self.0.borrow().flags.clone() }
    pub fn p_flags_opt(&self) -> Option<i64> { Some(self.0.borrow().flags.clone()) }
    pub fn p_flags_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.flags) }
    pub fn set_p_flags(&self, v: i64) { self.0.borrow_mut().flags = v; }
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_constants(&self) -> Ref<'_, List<crate::php_parser::node::Const_>> { Ref::map(self.0.borrow(), |o| &o.constants) }
    pub fn p_constants_get(&self) -> List<crate::php_parser::node::Const_> { self.0.borrow().constants.clone() }
    pub fn p_constants_opt(&self) -> Option<List<crate::php_parser::node::Const_>> { Some(self.0.borrow().constants.clone()) }
    pub fn p_constants_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Const_>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.constants) }
    pub fn set_p_constants(&self, v: List<crate::php_parser::node::Const_>) { self.0.borrow_mut().constants = v; }
    pub fn p_attributeGroups(&self) -> Ref<'_, List<crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attributeGroups) }
    pub fn p_attributeGroups_get(&self) -> List<crate::php_parser::node::AttributeGroup> { self.0.borrow().attributeGroups.clone() }
    pub fn p_attributeGroups_opt(&self) -> Option<List<crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attributeGroups.clone()) }
    pub fn p_attributeGroups_mut(&self) -> RefMut<'_, List<crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributeGroups) }
    pub fn set_p_attributeGroups(&self, v: List<crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attributeGroups = v; }
    pub fn p_type_(&self) -> Ref<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { Ref::map(self.0.borrow(), |o| &o.type_) }
    pub fn p_type__get(&self) -> Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> { self.0.borrow().type_.clone() }
    pub fn p_type__opt(&self) -> Option<Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { Some(self.0.borrow().type_.clone()) }
    pub fn p_type__mut(&self) -> RefMut<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.type_) }
    pub fn set_p_type_(&self, v: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>) { self.0.borrow_mut().type_ = v; }
    pub fn new(mut name: U_PhpParser_Node_Identifier_or_Str, mut value: Option<U_Bool_or_Float_or_Int_or_Map_ArrayKey_Mixed_or_PhpParser_Node_Expr_or_Str_or_UnitEnum>) -> Result<ClassConst, Throw> {
        let this = ClassConst(Rc::new(RefCell::new(ClassConstObj {
            flags: 0i64,
            attributes: Map::<Str, Mixed>::new(),
            constants: List::<crate::php_parser::node::Const_>::new(),
            attributeGroups: List::<crate::php_parser::node::AttributeGroup>::new(),
            type_: { let _ = (); None::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> },
        })));
        this.magic__construct(name, value)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut value: Option<U_Bool_or_Float_or_Int_or_Map_ArrayKey_Mixed_or_PhpParser_Node_Expr_or_Str_or_UnitEnum>) -> Result<Mixed, Throw> {
    self.set_p_constants(list![crate::php_parser::node::Const_::new(name.clone(), crate::php_parser::BuilderHelpers::normalizeValue(value.clone())?, Map::<Str, Mixed>::new())?]);
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn addConst(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut value: Option<U_Bool_or_Float_or_Int_or_Map_ArrayKey_Mixed_or_PhpParser_Node_Expr_or_Str_or_UnitEnum>) -> Result<crate::php_parser::builder::ClassConst, Throw> {
    (*self.p_constants_mut()).push(crate::php_parser::node::Const_::new(name.clone(), crate::php_parser::BuilderHelpers::normalizeValue(value.clone())?, Map::<Str, Mixed>::new())?);
    return Ok(self.clone());
    }
    pub fn makePublic(&self) -> Result<crate::php_parser::builder::ClassConst, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PUBLIC())?);
    return Ok(self.clone());
    }
    pub fn makeProtected(&self) -> Result<crate::php_parser::builder::ClassConst, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PROTECTED())?);
    return Ok(self.clone());
    }
    pub fn makePrivate(&self) -> Result<crate::php_parser::builder::ClassConst, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PRIVATE())?);
    return Ok(self.clone());
    }
    pub fn makeFinal(&self) -> Result<crate::php_parser::builder::ClassConst, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::FINAL())?);
    return Ok(self.clone());
    }
    pub fn setDocComment(&self, mut docComment: U_PhpParser_Comment_Doc_or_Str) -> Result<crate::php_parser::builder::ClassConst, Throw> {
    self.set_p_attributes({ let mut __m1: Map<Str, Mixed> = Map::new(); __m1.insert(Str::from_static("comments"), { let __c1 = (crate::php_parser::BuilderHelpers::normalizeDocComment(docComment.clone())?,); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1.0)); Mixed::Arr(__m) }); __m1 });
    return Ok(self.clone());
    }
    pub fn addAttribute(&self, mut attribute: U_PhpParser_Node_Attribute_or_PhpParser_Node_AttributeGroup) -> Result<crate::php_parser::builder::ClassConst, Throw> {
    (*self.p_attributeGroups_mut()).push(crate::php_parser::BuilderHelpers::normalizeAttribute(attribute.clone())?);
    return Ok(self.clone());
    }
    pub fn setType(&self, mut type_: U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name_or_Str) -> Result<crate::php_parser::builder::ClassConst, Throw> {
    self.set_p_type_(Some(crate::php_parser::BuilderHelpers::normalizeType(type_.clone())?));
    return Ok(self.clone());
    }
    pub fn getNode(&self) -> Result<crate::php_parser::node::stmt::ClassConst, Throw> {
    return Ok(crate::php_parser::node::stmt::ClassConst::new(cast::<Map<ArrayKey, crate::php_parser::node::Const_>>(self.p_constants_get()), self.p_flags_get(), self.p_attributes_get(), self.p_attributeGroups_get(), self.p_type__get())?);
    }
    pub fn new_same_class(&self, mut name: U_PhpParser_Node_Identifier_or_Str, mut value: Option<U_Bool_or_Float_or_Int_or_Map_ArrayKey_Mixed_or_PhpParser_Node_Expr_or_Str_or_UnitEnum>) -> Result<crate::php_parser::builder::ClassConst, Throw> { Ok(Self::new(name, value)?) }
}
impl php_rt::PhpObject for ClassConst {
    fn class_name(&self) -> &'static str { "PhpParser\\Builder\\ClassConst" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\builder\\classconst", "phpparser\\builder"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_flags_get()) { out.push((Str::from_static("flags"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_constants_get()) { out.push((Str::from_static("constants"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attributeGroups_get()) { out.push((Str::from_static("attributeGroups"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_type__get()) { out.push((Str::from_static("type"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "flags" => { self.set_p_flags(cast::<i64>(value)); true }, "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "constants" => { self.set_p_constants(cast::<List<crate::php_parser::node::Const_>>(value)); true }, "attributeGroups" => { self.set_p_attributeGroups(cast::<List<crate::php_parser::node::AttributeGroup>>(value)); true }, "type" => { self.set_p_type_(value.to_option().map(|__m| cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(__m))); true }, _ => false } }
}
impl ClassConst { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Builder\\ClassConst could not be converted to string"))) } }
impl php_rt::PhpClone for ClassConst { fn php_clone(&self) -> Self { let c = ClassConst(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ClassConstObj { fn clone(&self) -> Self { ClassConstObj { flags: self.flags.clone(), attributes: self.attributes.clone(), constants: self.constants.clone(), attributeGroups: self.attributeGroups.clone(), type_: self.type_.clone() } } }
impl ClassConst {
}
pub struct Class_Obj {
    pub attributes: Map<Str, Mixed>,
    pub name: Str,
    pub extends: Option<crate::php_parser::node::Name>,
    pub implements: List<crate::php_parser::node::Name>,
    pub flags: i64,
    pub uses: List<crate::php_parser::node::stmt::TraitUse>,
    pub constants: List<crate::php_parser::node::stmt::ClassConst>,
    pub properties: List<crate::php_parser::node::stmt::Property>,
    pub methods: List<crate::php_parser::node::stmt::ClassMethod>,
    pub attributeGroups: List<crate::php_parser::node::AttributeGroup>,
}
#[derive(Clone)]
pub struct Class_(pub Rc<RefCell<Class_Obj>>);
impl Class_ {
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
    pub fn p_extends(&self) -> Ref<'_, Option<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.extends) }
    pub fn p_extends_get(&self) -> Option<crate::php_parser::node::Name> { self.0.borrow().extends.clone() }
    pub fn p_extends_opt(&self) -> Option<Option<crate::php_parser::node::Name>> { Some(self.0.borrow().extends.clone()) }
    pub fn p_extends_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.extends) }
    pub fn set_p_extends(&self, v: Option<crate::php_parser::node::Name>) { self.0.borrow_mut().extends = v; }
    pub fn p_implements(&self) -> Ref<'_, List<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.implements) }
    pub fn p_implements_get(&self) -> List<crate::php_parser::node::Name> { self.0.borrow().implements.clone() }
    pub fn p_implements_opt(&self) -> Option<List<crate::php_parser::node::Name>> { Some(self.0.borrow().implements.clone()) }
    pub fn p_implements_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.implements) }
    pub fn set_p_implements(&self, v: List<crate::php_parser::node::Name>) { self.0.borrow_mut().implements = v; }
    pub fn p_flags(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.flags) }
    pub fn p_flags_get(&self) -> i64 { self.0.borrow().flags.clone() }
    pub fn p_flags_opt(&self) -> Option<i64> { Some(self.0.borrow().flags.clone()) }
    pub fn p_flags_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.flags) }
    pub fn set_p_flags(&self, v: i64) { self.0.borrow_mut().flags = v; }
    pub fn p_uses(&self) -> Ref<'_, List<crate::php_parser::node::stmt::TraitUse>> { Ref::map(self.0.borrow(), |o| &o.uses) }
    pub fn p_uses_get(&self) -> List<crate::php_parser::node::stmt::TraitUse> { self.0.borrow().uses.clone() }
    pub fn p_uses_opt(&self) -> Option<List<crate::php_parser::node::stmt::TraitUse>> { Some(self.0.borrow().uses.clone()) }
    pub fn p_uses_mut(&self) -> RefMut<'_, List<crate::php_parser::node::stmt::TraitUse>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.uses) }
    pub fn set_p_uses(&self, v: List<crate::php_parser::node::stmt::TraitUse>) { self.0.borrow_mut().uses = v; }
    pub fn p_constants(&self) -> Ref<'_, List<crate::php_parser::node::stmt::ClassConst>> { Ref::map(self.0.borrow(), |o| &o.constants) }
    pub fn p_constants_get(&self) -> List<crate::php_parser::node::stmt::ClassConst> { self.0.borrow().constants.clone() }
    pub fn p_constants_opt(&self) -> Option<List<crate::php_parser::node::stmt::ClassConst>> { Some(self.0.borrow().constants.clone()) }
    pub fn p_constants_mut(&self) -> RefMut<'_, List<crate::php_parser::node::stmt::ClassConst>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.constants) }
    pub fn set_p_constants(&self, v: List<crate::php_parser::node::stmt::ClassConst>) { self.0.borrow_mut().constants = v; }
    pub fn p_properties(&self) -> Ref<'_, List<crate::php_parser::node::stmt::Property>> { Ref::map(self.0.borrow(), |o| &o.properties) }
    pub fn p_properties_get(&self) -> List<crate::php_parser::node::stmt::Property> { self.0.borrow().properties.clone() }
    pub fn p_properties_opt(&self) -> Option<List<crate::php_parser::node::stmt::Property>> { Some(self.0.borrow().properties.clone()) }
    pub fn p_properties_mut(&self) -> RefMut<'_, List<crate::php_parser::node::stmt::Property>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.properties) }
    pub fn set_p_properties(&self, v: List<crate::php_parser::node::stmt::Property>) { self.0.borrow_mut().properties = v; }
    pub fn p_methods(&self) -> Ref<'_, List<crate::php_parser::node::stmt::ClassMethod>> { Ref::map(self.0.borrow(), |o| &o.methods) }
    pub fn p_methods_get(&self) -> List<crate::php_parser::node::stmt::ClassMethod> { self.0.borrow().methods.clone() }
    pub fn p_methods_opt(&self) -> Option<List<crate::php_parser::node::stmt::ClassMethod>> { Some(self.0.borrow().methods.clone()) }
    pub fn p_methods_mut(&self) -> RefMut<'_, List<crate::php_parser::node::stmt::ClassMethod>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.methods) }
    pub fn set_p_methods(&self, v: List<crate::php_parser::node::stmt::ClassMethod>) { self.0.borrow_mut().methods = v; }
    pub fn p_attributeGroups(&self) -> Ref<'_, List<crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attributeGroups) }
    pub fn p_attributeGroups_get(&self) -> List<crate::php_parser::node::AttributeGroup> { self.0.borrow().attributeGroups.clone() }
    pub fn p_attributeGroups_opt(&self) -> Option<List<crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attributeGroups.clone()) }
    pub fn p_attributeGroups_mut(&self) -> RefMut<'_, List<crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributeGroups) }
    pub fn set_p_attributeGroups(&self, v: List<crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attributeGroups = v; }
    pub fn new(mut name: Str) -> Result<Class_, Throw> {
        let this = Class_(Rc::new(RefCell::new(Class_Obj {
            attributes: Map::<Str, Mixed>::new(),
            name: Default::default(),
            extends: { let _ = (); None::<crate::php_parser::node::Name> },
            implements: List::<crate::php_parser::node::Name>::new(),
            flags: 0i64,
            uses: List::<crate::php_parser::node::stmt::TraitUse>::new(),
            constants: List::<crate::php_parser::node::stmt::ClassConst>::new(),
            properties: List::<crate::php_parser::node::stmt::Property>::new(),
            methods: List::<crate::php_parser::node::stmt::ClassMethod>::new(),
            attributeGroups: List::<crate::php_parser::node::AttributeGroup>::new(),
        })));
        this.magic__construct(name)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: Str) -> Result<Mixed, Throw> {
    self.set_p_name(name.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn extend(&self, mut class: U_PhpParser_Node_Name_or_Str) -> Result<crate::php_parser::builder::Class_, Throw> {
    self.set_p_extends(Some(crate::php_parser::BuilderHelpers::normalizeName(class.clone())?));
    return Ok(self.clone());
    }
    pub fn implement(&self, mut interfaces: List<U_PhpParser_Node_Name_or_Str>) -> Result<crate::php_parser::builder::Class_, Throw> {
    let mut interface: Late<U_PhpParser_Node_Name_or_Str> = Late::uninit();
    'l1: for __kv1 in cast::<Map<ArrayKey, U_PhpParser_Node_Name_or_Str>>(interfaces.clone()).into_iter() {
        interface.set(__kv1.1);
        (*self.p_implements_mut()).push(crate::php_parser::BuilderHelpers::normalizeName(interface.get().clone())?);
    }
    return Ok(self.clone());
    }
    pub fn makeAbstract(&self) -> Result<crate::php_parser::builder::Class_, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addClassModifier(self.p_flags_get(), crate::php_parser::Modifiers::ABSTRACT())?);
    return Ok(self.clone());
    }
    pub fn makeFinal(&self) -> Result<crate::php_parser::builder::Class_, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addClassModifier(self.p_flags_get(), crate::php_parser::Modifiers::FINAL())?);
    return Ok(self.clone());
    }
    pub fn makeReadonly(&self) -> Result<crate::php_parser::builder::Class_, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addClassModifier(self.p_flags_get(), crate::php_parser::Modifiers::READONLY())?);
    return Ok(self.clone());
    }
    pub fn addStmt(&self, mut stmt: U_PhpParser_Builder_or_PhpParser_Node_Stmt) -> Result<crate::php_parser::builder::Class_, Throw> {
    stmt = cast::<U_PhpParser_Builder_or_PhpParser_Node_Stmt>(crate::php_parser::BuilderHelpers::normalizeNode(cast::<U_PhpParser_Builder_or_PhpParser_Node>(stmt.clone()))?);
    if is_instance::<crate::php_parser::node::stmt::Property>(&cast::<crate::php_parser::Node>(stmt.clone())) {
        (*self.p_properties_mut()).push(cast::<crate::php_parser::node::stmt::Property>(stmt.clone()));
    } else if is_instance::<crate::php_parser::node::stmt::ClassMethod>(&cast::<crate::php_parser::Node>(stmt.clone())) {
        (*self.p_methods_mut()).push(cast::<crate::php_parser::node::stmt::ClassMethod>(stmt.clone()));
    } else if is_instance::<crate::php_parser::node::stmt::TraitUse>(&cast::<crate::php_parser::Node>(stmt.clone())) {
        (*self.p_uses_mut()).push(cast::<crate::php_parser::node::stmt::TraitUse>(stmt.clone()));
    } else if is_instance::<crate::php_parser::node::stmt::ClassConst>(&cast::<crate::php_parser::Node>(stmt.clone())) {
        (*self.p_constants_mut()).push(cast::<crate::php_parser::node::stmt::ClassConst>(stmt.clone()));
    } else {
        return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(sprintf(&Str::from_static("Unexpected node of type \"%s\""), &[FmtArg::from(cast::<crate::php_parser::Node>(stmt.clone()).getType()?)])?, 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    return Ok(self.clone());
    }
    pub fn addAttribute(&self, mut attribute: U_PhpParser_Node_Attribute_or_PhpParser_Node_AttributeGroup) -> Result<crate::php_parser::builder::Class_, Throw> {
    (*self.p_attributeGroups_mut()).push(crate::php_parser::BuilderHelpers::normalizeAttribute(attribute.clone())?);
    return Ok(self.clone());
    }
    pub fn getNode(&self) -> Result<crate::php_parser::node::stmt::Class_, Throw> {
    return Ok(crate::php_parser::node::stmt::Class_::new(Some(U_PhpParser_Node_Identifier_or_Str::Str(self.p_name_get())), Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__467d114822 { flags: Some(self.p_flags_get()), extends: self.p_extends_get(), implements: Some(cast::<Map<ArrayKey, crate::php_parser::node::Name>>(self.p_implements_get())), stmts: Some(cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(array_merge_l(&[&self.p_uses_get().map_elems(|v| U_PhpParser_Node_Stmt_ClassConst_or_PhpParser_Node_Stmt_ClassMethod_or_c7a89e663f::PhpParser_Node_Stmt_TraitUse(v)), &self.p_constants_get().map_elems(|v| U_PhpParser_Node_Stmt_ClassConst_or_PhpParser_Node_Stmt_ClassMethod_or_c7a89e663f::PhpParser_Node_Stmt_ClassConst(v)), &self.p_properties_get().map_elems(|v| U_PhpParser_Node_Stmt_ClassConst_or_PhpParser_Node_Stmt_ClassMethod_or_c7a89e663f::PhpParser_Node_Stmt_Property(v)), &self.p_methods_get().map_elems(|v| U_PhpParser_Node_Stmt_ClassConst_or_PhpParser_Node_Stmt_ClassMethod_or_c7a89e663f::PhpParser_Node_Stmt_ClassMethod(v))]).map_elems(|v| cast::<crate::php_parser::node::Stmt>(v)))), attrGroups: Some(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(self.p_attributeGroups_get())) }, self.p_attributes_get())?);
    }
    pub fn addStmts(&self, mut stmts: Map<ArrayKey, U_PhpParser_Builder_or_PhpParser_Node_Stmt>) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).addStmts__impl(stmts) }
    pub fn setDocComment(&self, mut docComment: U_PhpParser_Comment_Doc_or_Str) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).setDocComment__impl(docComment) }
    pub fn new_same_class(&self, mut name: Str) -> Result<crate::php_parser::builder::Class_, Throw> { Ok(Self::new(name)?) }
}
impl php_rt::PhpObject for Class_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Builder\\Class_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\builder\\class_", "phpparser\\builder\\declaration", "phpparser\\builder"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_extends_get()) { out.push((Str::from_static("extends"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_implements_get()) { out.push((Str::from_static("implements"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_flags_get()) { out.push((Str::from_static("flags"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_uses_get()) { out.push((Str::from_static("uses"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_constants_get()) { out.push((Str::from_static("constants"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_properties_get()) { out.push((Str::from_static("properties"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_methods_get()) { out.push((Str::from_static("methods"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attributeGroups_get()) { out.push((Str::from_static("attributeGroups"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(cast::<Str>(value)); true }, "extends" => { self.set_p_extends(value.to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m))); true }, "implements" => { self.set_p_implements(cast::<List<crate::php_parser::node::Name>>(value)); true }, "flags" => { self.set_p_flags(cast::<i64>(value)); true }, "uses" => { self.set_p_uses(cast::<List<crate::php_parser::node::stmt::TraitUse>>(value)); true }, "constants" => { self.set_p_constants(cast::<List<crate::php_parser::node::stmt::ClassConst>>(value)); true }, "properties" => { self.set_p_properties(cast::<List<crate::php_parser::node::stmt::Property>>(value)); true }, "methods" => { self.set_p_methods(cast::<List<crate::php_parser::node::stmt::ClassMethod>>(value)); true }, "attributeGroups" => { self.set_p_attributeGroups(cast::<List<crate::php_parser::node::AttributeGroup>>(value)); true }, _ => false } }
}
impl Class_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Builder\\Class_ could not be converted to string"))) } }
impl php_rt::PhpClone for Class_ { fn php_clone(&self) -> Self { let c = Class_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Class_Obj { fn clone(&self) -> Self { Class_Obj { attributes: self.attributes.clone(), name: self.name.clone(), extends: self.extends.clone(), implements: self.implements.clone(), flags: self.flags.clone(), uses: self.uses.clone(), constants: self.constants.clone(), properties: self.properties.clone(), methods: self.methods.clone(), attributeGroups: self.attributeGroups.clone() } } }
impl Class_ {
}
#[derive(Clone)]
pub enum Declaration {
    PhpParser_Builder_Class_(crate::php_parser::builder::Class_),
    PhpParser_Builder_Enum_(crate::php_parser::builder::Enum_),
    PhpParser_Builder_Function_(crate::php_parser::builder::Function_),
    PhpParser_Builder_Interface_(crate::php_parser::builder::Interface_),
    PhpParser_Builder_Method(crate::php_parser::builder::Method),
    PhpParser_Builder_Namespace_(crate::php_parser::builder::Namespace_),
    PhpParser_Builder_Trait_(crate::php_parser::builder::Trait_),
}
impl php_rt::PhpObject for Declaration {
    fn class_name(&self) -> &'static str { match self { Declaration::PhpParser_Builder_Class_(__h) => __h.class_name(), Declaration::PhpParser_Builder_Enum_(__h) => __h.class_name(), Declaration::PhpParser_Builder_Function_(__h) => __h.class_name(), Declaration::PhpParser_Builder_Interface_(__h) => __h.class_name(), Declaration::PhpParser_Builder_Method(__h) => __h.class_name(), Declaration::PhpParser_Builder_Namespace_(__h) => __h.class_name(), Declaration::PhpParser_Builder_Trait_(__h) => __h.class_name(), _ => unreachable!() } }
    fn class_ancestors(&self) -> &'static [&'static str] { match self { Declaration::PhpParser_Builder_Class_(__h) => __h.class_ancestors(), Declaration::PhpParser_Builder_Enum_(__h) => __h.class_ancestors(), Declaration::PhpParser_Builder_Function_(__h) => __h.class_ancestors(), Declaration::PhpParser_Builder_Interface_(__h) => __h.class_ancestors(), Declaration::PhpParser_Builder_Method(__h) => __h.class_ancestors(), Declaration::PhpParser_Builder_Namespace_(__h) => __h.class_ancestors(), Declaration::PhpParser_Builder_Trait_(__h) => __h.class_ancestors(), _ => unreachable!() } }
    fn obj_id(&self) -> usize { match self { Declaration::PhpParser_Builder_Class_(__h) => __h.obj_id(), Declaration::PhpParser_Builder_Enum_(__h) => __h.obj_id(), Declaration::PhpParser_Builder_Function_(__h) => __h.obj_id(), Declaration::PhpParser_Builder_Interface_(__h) => __h.obj_id(), Declaration::PhpParser_Builder_Method(__h) => __h.obj_id(), Declaration::PhpParser_Builder_Namespace_(__h) => __h.obj_id(), Declaration::PhpParser_Builder_Trait_(__h) => __h.obj_id(), _ => unreachable!() } }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { match self { Declaration::PhpParser_Builder_Class_(__h) => __h.props(), Declaration::PhpParser_Builder_Enum_(__h) => __h.props(), Declaration::PhpParser_Builder_Function_(__h) => __h.props(), Declaration::PhpParser_Builder_Interface_(__h) => __h.props(), Declaration::PhpParser_Builder_Method(__h) => __h.props(), Declaration::PhpParser_Builder_Namespace_(__h) => __h.props(), Declaration::PhpParser_Builder_Trait_(__h) => __h.props(), _ => unreachable!() } }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match self { Declaration::PhpParser_Builder_Class_(__h) => __h.set_prop(name, value), Declaration::PhpParser_Builder_Enum_(__h) => __h.set_prop(name, value), Declaration::PhpParser_Builder_Function_(__h) => __h.set_prop(name, value), Declaration::PhpParser_Builder_Interface_(__h) => __h.set_prop(name, value), Declaration::PhpParser_Builder_Method(__h) => __h.set_prop(name, value), Declaration::PhpParser_Builder_Namespace_(__h) => __h.set_prop(name, value), Declaration::PhpParser_Builder_Trait_(__h) => __h.set_prop(name, value), _ => unreachable!() } }
    fn php_to_string(&self) -> Option<Str> { match self { Declaration::PhpParser_Builder_Class_(__h) => __h.php_to_string(), Declaration::PhpParser_Builder_Enum_(__h) => __h.php_to_string(), Declaration::PhpParser_Builder_Function_(__h) => __h.php_to_string(), Declaration::PhpParser_Builder_Interface_(__h) => __h.php_to_string(), Declaration::PhpParser_Builder_Method(__h) => __h.php_to_string(), Declaration::PhpParser_Builder_Namespace_(__h) => __h.php_to_string(), Declaration::PhpParser_Builder_Trait_(__h) => __h.php_to_string(), _ => unreachable!() } }
}
impl Declaration { pub fn to_php_string(&self) -> Result<Str, Throw> { match self { Declaration::PhpParser_Builder_Class_(__h) => __h.to_php_string(), Declaration::PhpParser_Builder_Enum_(__h) => __h.to_php_string(), Declaration::PhpParser_Builder_Function_(__h) => __h.to_php_string(), Declaration::PhpParser_Builder_Interface_(__h) => __h.to_php_string(), Declaration::PhpParser_Builder_Method(__h) => __h.to_php_string(), Declaration::PhpParser_Builder_Namespace_(__h) => __h.to_php_string(), Declaration::PhpParser_Builder_Trait_(__h) => __h.to_php_string(), _ => unreachable!() } } }
impl php_rt::PhpClone for Declaration { fn php_clone(&self) -> Self { match self { Declaration::PhpParser_Builder_Class_(__h) => Declaration::PhpParser_Builder_Class_(__h.php_clone()), Declaration::PhpParser_Builder_Enum_(__h) => Declaration::PhpParser_Builder_Enum_(__h.php_clone()), Declaration::PhpParser_Builder_Function_(__h) => Declaration::PhpParser_Builder_Function_(__h.php_clone()), Declaration::PhpParser_Builder_Interface_(__h) => Declaration::PhpParser_Builder_Interface_(__h.php_clone()), Declaration::PhpParser_Builder_Method(__h) => Declaration::PhpParser_Builder_Method(__h.php_clone()), Declaration::PhpParser_Builder_Namespace_(__h) => Declaration::PhpParser_Builder_Namespace_(__h.php_clone()), Declaration::PhpParser_Builder_Trait_(__h) => Declaration::PhpParser_Builder_Trait_(__h.php_clone()), _ => unreachable!() } } }
impl Declaration {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { match self { Declaration::PhpParser_Builder_Class_(__h) => __h.p_attributes(), Declaration::PhpParser_Builder_Enum_(__h) => __h.p_attributes(), Declaration::PhpParser_Builder_Function_(__h) => __h.p_attributes(), Declaration::PhpParser_Builder_Interface_(__h) => __h.p_attributes(), Declaration::PhpParser_Builder_Method(__h) => __h.p_attributes(), Declaration::PhpParser_Builder_Namespace_(__h) => __h.p_attributes(), Declaration::PhpParser_Builder_Trait_(__h) => __h.p_attributes(), _ => unreachable!() } }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { match self { Declaration::PhpParser_Builder_Class_(__h) => __h.p_attributes_get(), Declaration::PhpParser_Builder_Enum_(__h) => __h.p_attributes_get(), Declaration::PhpParser_Builder_Function_(__h) => __h.p_attributes_get(), Declaration::PhpParser_Builder_Interface_(__h) => __h.p_attributes_get(), Declaration::PhpParser_Builder_Method(__h) => __h.p_attributes_get(), Declaration::PhpParser_Builder_Namespace_(__h) => __h.p_attributes_get(), Declaration::PhpParser_Builder_Trait_(__h) => __h.p_attributes_get(), _ => unreachable!() } }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { match self { Declaration::PhpParser_Builder_Class_(__h) => __h.p_attributes_opt(), Declaration::PhpParser_Builder_Enum_(__h) => __h.p_attributes_opt(), Declaration::PhpParser_Builder_Function_(__h) => __h.p_attributes_opt(), Declaration::PhpParser_Builder_Interface_(__h) => __h.p_attributes_opt(), Declaration::PhpParser_Builder_Method(__h) => __h.p_attributes_opt(), Declaration::PhpParser_Builder_Namespace_(__h) => __h.p_attributes_opt(), Declaration::PhpParser_Builder_Trait_(__h) => __h.p_attributes_opt(), _ => unreachable!() } }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { match self { Declaration::PhpParser_Builder_Class_(__h) => __h.p_attributes_mut(), Declaration::PhpParser_Builder_Enum_(__h) => __h.p_attributes_mut(), Declaration::PhpParser_Builder_Function_(__h) => __h.p_attributes_mut(), Declaration::PhpParser_Builder_Interface_(__h) => __h.p_attributes_mut(), Declaration::PhpParser_Builder_Method(__h) => __h.p_attributes_mut(), Declaration::PhpParser_Builder_Namespace_(__h) => __h.p_attributes_mut(), Declaration::PhpParser_Builder_Trait_(__h) => __h.p_attributes_mut(), _ => unreachable!() } }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { match self { Declaration::PhpParser_Builder_Class_(__h) => __h.set_p_attributes(v), Declaration::PhpParser_Builder_Enum_(__h) => __h.set_p_attributes(v), Declaration::PhpParser_Builder_Function_(__h) => __h.set_p_attributes(v), Declaration::PhpParser_Builder_Interface_(__h) => __h.set_p_attributes(v), Declaration::PhpParser_Builder_Method(__h) => __h.set_p_attributes(v), Declaration::PhpParser_Builder_Namespace_(__h) => __h.set_p_attributes(v), Declaration::PhpParser_Builder_Trait_(__h) => __h.set_p_attributes(v), _ => unreachable!() } }
    pub fn addStmt(&self, mut stmt: U_PhpParser_Builder_or_PhpParser_Node_Stmt) -> Result<crate::php_parser::builder::Declaration, Throw> { match self { Declaration::PhpParser_Builder_Class_(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.addStmt(stmt)?)), Declaration::PhpParser_Builder_Enum_(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.addStmt(stmt)?)), Declaration::PhpParser_Builder_Function_(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.addStmt(cast::<U_PhpParser_Builder_or_PhpParser_Node>(stmt))?)), Declaration::PhpParser_Builder_Interface_(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.addStmt(stmt)?)), Declaration::PhpParser_Builder_Method(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.addStmt(cast::<U_PhpParser_Builder_or_PhpParser_Node>(stmt))?)), Declaration::PhpParser_Builder_Namespace_(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.addStmt(cast::<U_PhpParser_Builder_or_PhpParser_Node>(stmt))?)), Declaration::PhpParser_Builder_Trait_(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.addStmt(stmt)?)), _ => unreachable!() } }
    pub fn addStmts(&self, mut stmts: Map<ArrayKey, U_PhpParser_Builder_or_PhpParser_Node_Stmt>) -> Result<crate::php_parser::builder::Declaration, Throw> { match self { Declaration::PhpParser_Builder_Class_(__h) => Ok(__h.addStmts(stmts)?), Declaration::PhpParser_Builder_Enum_(__h) => Ok(__h.addStmts(stmts)?), Declaration::PhpParser_Builder_Function_(__h) => Ok(__h.addStmts(stmts)?), Declaration::PhpParser_Builder_Interface_(__h) => Ok(__h.addStmts(stmts)?), Declaration::PhpParser_Builder_Method(__h) => Ok(__h.addStmts(stmts)?), Declaration::PhpParser_Builder_Namespace_(__h) => Ok(__h.addStmts(stmts)?), Declaration::PhpParser_Builder_Trait_(__h) => Ok(__h.addStmts(stmts)?), _ => unreachable!() } }
    pub fn addStmts__impl(&self, mut stmts: Map<ArrayKey, U_PhpParser_Builder_or_PhpParser_Node_Stmt>) -> Result<crate::php_parser::builder::Declaration, Throw> {
    let mut stmt: Late<U_PhpParser_Builder_or_PhpParser_Node_Stmt> = Late::uninit();
    'l1: for __kv1 in stmts.clone().into_iter() {
        stmt.set(__kv1.1);
        let _: crate::php_parser::builder::Declaration = self.addStmt(stmt.get().clone())?;
    }
    return Ok(self.clone());
    }
    pub fn setDocComment(&self, mut docComment: U_PhpParser_Comment_Doc_or_Str) -> Result<crate::php_parser::builder::Declaration, Throw> { match self { Declaration::PhpParser_Builder_Class_(__h) => Ok(__h.setDocComment(docComment)?), Declaration::PhpParser_Builder_Enum_(__h) => Ok(__h.setDocComment(docComment)?), Declaration::PhpParser_Builder_Function_(__h) => Ok(__h.setDocComment(docComment)?), Declaration::PhpParser_Builder_Interface_(__h) => Ok(__h.setDocComment(docComment)?), Declaration::PhpParser_Builder_Method(__h) => Ok(__h.setDocComment(docComment)?), Declaration::PhpParser_Builder_Namespace_(__h) => Ok(__h.setDocComment(docComment)?), Declaration::PhpParser_Builder_Trait_(__h) => Ok(__h.setDocComment(docComment)?), _ => unreachable!() } }
    pub fn setDocComment__impl(&self, mut docComment: U_PhpParser_Comment_Doc_or_Str) -> Result<crate::php_parser::builder::Declaration, Throw> {
    (*self.p_attributes_mut()).insert(Str::from_static("comments"), { let __c2 = (crate::php_parser::BuilderHelpers::normalizeDocComment(docComment.clone())?,); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c2.0)); Mixed::Arr(__m) });
    return Ok(self.clone());
    }
    pub fn getNode(&self) -> Result<crate::php_parser::Node, Throw> { match self { Declaration::PhpParser_Builder_Class_(__h) => Ok(cast::<crate::php_parser::Node>(__h.getNode()?)), Declaration::PhpParser_Builder_Enum_(__h) => Ok(cast::<crate::php_parser::Node>(__h.getNode()?)), Declaration::PhpParser_Builder_Function_(__h) => Ok(cast::<crate::php_parser::Node>(__h.getNode()?)), Declaration::PhpParser_Builder_Interface_(__h) => Ok(cast::<crate::php_parser::Node>(__h.getNode()?)), Declaration::PhpParser_Builder_Method(__h) => Ok(cast::<crate::php_parser::Node>(__h.getNode()?)), Declaration::PhpParser_Builder_Namespace_(__h) => Ok(cast::<crate::php_parser::Node>(__h.getNode()?)), Declaration::PhpParser_Builder_Trait_(__h) => Ok(cast::<crate::php_parser::Node>(__h.getNode()?)), _ => unreachable!() } }
    pub fn new_same_class(&self) -> Result<Declaration, Throw> { match self { Declaration::PhpParser_Builder_Class_(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.new_same_class(<Str>::default())?)), Declaration::PhpParser_Builder_Enum_(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.new_same_class(<Str>::default())?)), Declaration::PhpParser_Builder_Function_(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.new_same_class(<Str>::default())?)), Declaration::PhpParser_Builder_Interface_(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.new_same_class(<Str>::default())?)), Declaration::PhpParser_Builder_Method(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.new_same_class(<Str>::default())?)), Declaration::PhpParser_Builder_Namespace_(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.new_same_class(<Option<U_PhpParser_Node_Name_or_Str>>::default())?)), Declaration::PhpParser_Builder_Trait_(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.new_same_class(<Str>::default())?)), _ => unreachable!() } }
}
pub struct EnumCaseObj {
    pub name: Late<U_PhpParser_Node_Identifier_or_Str>,
    pub value: Option<crate::php_parser::node::Expr>,
    pub attributes: Map<Str, Mixed>,
    pub attributeGroups: List<crate::php_parser::node::AttributeGroup>,
}
#[derive(Clone)]
pub struct EnumCase(pub Rc<RefCell<EnumCaseObj>>);
impl EnumCase {
    pub fn p_name(&self) -> Ref<'_, U_PhpParser_Node_Identifier_or_Str> { Ref::map(self.0.borrow(), |o| o.name.get()) }
    pub fn p_name_get(&self) -> U_PhpParser_Node_Identifier_or_Str { self.0.borrow().name.get().clone() }
    pub fn p_name_opt(&self) -> Option<U_PhpParser_Node_Identifier_or_Str> { self.0.borrow().name.as_option().cloned() }
    pub fn p_name_mut(&self) -> RefMut<'_, U_PhpParser_Node_Identifier_or_Str> { RefMut::map(self.0.borrow_mut(), |o| o.name.get_mut()) }
    pub fn set_p_name(&self, v: U_PhpParser_Node_Identifier_or_Str) { self.0.borrow_mut().name.set(v); }
    pub fn p_value(&self) -> Ref<'_, Option<crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.value) }
    pub fn p_value_get(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().value.clone() }
    pub fn p_value_opt(&self) -> Option<Option<crate::php_parser::node::Expr>> { Some(self.0.borrow().value.clone()) }
    pub fn p_value_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.value) }
    pub fn set_p_value(&self, v: Option<crate::php_parser::node::Expr>) { self.0.borrow_mut().value = v; }
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_attributeGroups(&self) -> Ref<'_, List<crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attributeGroups) }
    pub fn p_attributeGroups_get(&self) -> List<crate::php_parser::node::AttributeGroup> { self.0.borrow().attributeGroups.clone() }
    pub fn p_attributeGroups_opt(&self) -> Option<List<crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attributeGroups.clone()) }
    pub fn p_attributeGroups_mut(&self) -> RefMut<'_, List<crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributeGroups) }
    pub fn set_p_attributeGroups(&self, v: List<crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attributeGroups = v; }
    pub fn new(mut name: U_PhpParser_Node_Identifier_or_Str) -> Result<EnumCase, Throw> {
        let this = EnumCase(Rc::new(RefCell::new(EnumCaseObj {
            name: Late::uninit(),
            value: { let _ = (); None::<crate::php_parser::node::Expr> },
            attributes: Map::<Str, Mixed>::new(),
            attributeGroups: List::<crate::php_parser::node::AttributeGroup>::new(),
        })));
        this.magic__construct(name)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: U_PhpParser_Node_Identifier_or_Str) -> Result<Mixed, Throw> {
    self.set_p_name(name.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn setValue(&self, mut value: U_Int_or_PhpParser_Node_Expr_or_Str) -> Result<crate::php_parser::builder::EnumCase, Throw> {
    self.set_p_value(Some(crate::php_parser::BuilderHelpers::normalizeValue(Some(cast::<U_Bool_or_Float_or_Int_or_Map_ArrayKey_Mixed_or_PhpParser_Node_Expr_or_Str_or_UnitEnum>(value.clone())))?));
    return Ok(self.clone());
    }
    pub fn setDocComment(&self, mut docComment: U_PhpParser_Comment_Doc_or_Str) -> Result<crate::php_parser::builder::EnumCase, Throw> {
    self.set_p_attributes({ let mut __m1: Map<Str, Mixed> = Map::new(); __m1.insert(Str::from_static("comments"), { let __c3 = (crate::php_parser::BuilderHelpers::normalizeDocComment(docComment.clone())?,); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c3.0)); Mixed::Arr(__m) }); __m1 });
    return Ok(self.clone());
    }
    pub fn addAttribute(&self, mut attribute: U_PhpParser_Node_Attribute_or_PhpParser_Node_AttributeGroup) -> Result<crate::php_parser::builder::EnumCase, Throw> {
    (*self.p_attributeGroups_mut()).push(crate::php_parser::BuilderHelpers::normalizeAttribute(attribute.clone())?);
    return Ok(self.clone());
    }
    pub fn getNode(&self) -> Result<crate::php_parser::node::stmt::EnumCase, Throw> {
    return Ok(crate::php_parser::node::stmt::EnumCase::new(self.p_name_get(), self.p_value_get(), self.p_attributeGroups_get(), self.p_attributes_get())?);
    }
    pub fn new_same_class(&self, mut name: U_PhpParser_Node_Identifier_or_Str) -> Result<crate::php_parser::builder::EnumCase, Throw> { Ok(Self::new(name)?) }
}
impl php_rt::PhpObject for EnumCase {
    fn class_name(&self) -> &'static str { "PhpParser\\Builder\\EnumCase" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\builder\\enumcase", "phpparser\\builder"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_name_opt() { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_value_get()) { out.push((Str::from_static("value"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attributeGroups_get()) { out.push((Str::from_static("attributeGroups"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "name" => { self.set_p_name(cast::<U_PhpParser_Node_Identifier_or_Str>(value)); true }, "value" => { self.set_p_value(value.to_option().map(|__m| cast::<crate::php_parser::node::Expr>(__m))); true }, "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "attributeGroups" => { self.set_p_attributeGroups(cast::<List<crate::php_parser::node::AttributeGroup>>(value)); true }, _ => false } }
}
impl EnumCase { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Builder\\EnumCase could not be converted to string"))) } }
impl php_rt::PhpClone for EnumCase { fn php_clone(&self) -> Self { let c = EnumCase(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for EnumCaseObj { fn clone(&self) -> Self { EnumCaseObj { name: self.name.clone(), value: self.value.clone(), attributes: self.attributes.clone(), attributeGroups: self.attributeGroups.clone() } } }
impl EnumCase {
}
pub struct Enum_Obj {
    pub attributes: Map<Str, Mixed>,
    pub name: Str,
    pub scalarType: Option<crate::php_parser::node::Identifier>,
    pub implements: List<crate::php_parser::node::Name>,
    pub uses: List<crate::php_parser::node::stmt::TraitUse>,
    pub enumCases: List<crate::php_parser::node::stmt::EnumCase>,
    pub constants: List<crate::php_parser::node::stmt::ClassConst>,
    pub methods: List<crate::php_parser::node::stmt::ClassMethod>,
    pub attributeGroups: List<crate::php_parser::node::AttributeGroup>,
}
#[derive(Clone)]
pub struct Enum_(pub Rc<RefCell<Enum_Obj>>);
impl Enum_ {
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
    pub fn p_scalarType(&self) -> Ref<'_, Option<crate::php_parser::node::Identifier>> { Ref::map(self.0.borrow(), |o| &o.scalarType) }
    pub fn p_scalarType_get(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().scalarType.clone() }
    pub fn p_scalarType_opt(&self) -> Option<Option<crate::php_parser::node::Identifier>> { Some(self.0.borrow().scalarType.clone()) }
    pub fn p_scalarType_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Identifier>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.scalarType) }
    pub fn set_p_scalarType(&self, v: Option<crate::php_parser::node::Identifier>) { self.0.borrow_mut().scalarType = v; }
    pub fn p_implements(&self) -> Ref<'_, List<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.implements) }
    pub fn p_implements_get(&self) -> List<crate::php_parser::node::Name> { self.0.borrow().implements.clone() }
    pub fn p_implements_opt(&self) -> Option<List<crate::php_parser::node::Name>> { Some(self.0.borrow().implements.clone()) }
    pub fn p_implements_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.implements) }
    pub fn set_p_implements(&self, v: List<crate::php_parser::node::Name>) { self.0.borrow_mut().implements = v; }
    pub fn p_uses(&self) -> Ref<'_, List<crate::php_parser::node::stmt::TraitUse>> { Ref::map(self.0.borrow(), |o| &o.uses) }
    pub fn p_uses_get(&self) -> List<crate::php_parser::node::stmt::TraitUse> { self.0.borrow().uses.clone() }
    pub fn p_uses_opt(&self) -> Option<List<crate::php_parser::node::stmt::TraitUse>> { Some(self.0.borrow().uses.clone()) }
    pub fn p_uses_mut(&self) -> RefMut<'_, List<crate::php_parser::node::stmt::TraitUse>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.uses) }
    pub fn set_p_uses(&self, v: List<crate::php_parser::node::stmt::TraitUse>) { self.0.borrow_mut().uses = v; }
    pub fn p_enumCases(&self) -> Ref<'_, List<crate::php_parser::node::stmt::EnumCase>> { Ref::map(self.0.borrow(), |o| &o.enumCases) }
    pub fn p_enumCases_get(&self) -> List<crate::php_parser::node::stmt::EnumCase> { self.0.borrow().enumCases.clone() }
    pub fn p_enumCases_opt(&self) -> Option<List<crate::php_parser::node::stmt::EnumCase>> { Some(self.0.borrow().enumCases.clone()) }
    pub fn p_enumCases_mut(&self) -> RefMut<'_, List<crate::php_parser::node::stmt::EnumCase>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.enumCases) }
    pub fn set_p_enumCases(&self, v: List<crate::php_parser::node::stmt::EnumCase>) { self.0.borrow_mut().enumCases = v; }
    pub fn p_constants(&self) -> Ref<'_, List<crate::php_parser::node::stmt::ClassConst>> { Ref::map(self.0.borrow(), |o| &o.constants) }
    pub fn p_constants_get(&self) -> List<crate::php_parser::node::stmt::ClassConst> { self.0.borrow().constants.clone() }
    pub fn p_constants_opt(&self) -> Option<List<crate::php_parser::node::stmt::ClassConst>> { Some(self.0.borrow().constants.clone()) }
    pub fn p_constants_mut(&self) -> RefMut<'_, List<crate::php_parser::node::stmt::ClassConst>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.constants) }
    pub fn set_p_constants(&self, v: List<crate::php_parser::node::stmt::ClassConst>) { self.0.borrow_mut().constants = v; }
    pub fn p_methods(&self) -> Ref<'_, List<crate::php_parser::node::stmt::ClassMethod>> { Ref::map(self.0.borrow(), |o| &o.methods) }
    pub fn p_methods_get(&self) -> List<crate::php_parser::node::stmt::ClassMethod> { self.0.borrow().methods.clone() }
    pub fn p_methods_opt(&self) -> Option<List<crate::php_parser::node::stmt::ClassMethod>> { Some(self.0.borrow().methods.clone()) }
    pub fn p_methods_mut(&self) -> RefMut<'_, List<crate::php_parser::node::stmt::ClassMethod>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.methods) }
    pub fn set_p_methods(&self, v: List<crate::php_parser::node::stmt::ClassMethod>) { self.0.borrow_mut().methods = v; }
    pub fn p_attributeGroups(&self) -> Ref<'_, List<crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attributeGroups) }
    pub fn p_attributeGroups_get(&self) -> List<crate::php_parser::node::AttributeGroup> { self.0.borrow().attributeGroups.clone() }
    pub fn p_attributeGroups_opt(&self) -> Option<List<crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attributeGroups.clone()) }
    pub fn p_attributeGroups_mut(&self) -> RefMut<'_, List<crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributeGroups) }
    pub fn set_p_attributeGroups(&self, v: List<crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attributeGroups = v; }
    pub fn new(mut name: Str) -> Result<Enum_, Throw> {
        let this = Enum_(Rc::new(RefCell::new(Enum_Obj {
            attributes: Map::<Str, Mixed>::new(),
            name: Default::default(),
            scalarType: { let _ = (); None::<crate::php_parser::node::Identifier> },
            implements: List::<crate::php_parser::node::Name>::new(),
            uses: List::<crate::php_parser::node::stmt::TraitUse>::new(),
            enumCases: List::<crate::php_parser::node::stmt::EnumCase>::new(),
            constants: List::<crate::php_parser::node::stmt::ClassConst>::new(),
            methods: List::<crate::php_parser::node::stmt::ClassMethod>::new(),
            attributeGroups: List::<crate::php_parser::node::AttributeGroup>::new(),
        })));
        this.magic__construct(name)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: Str) -> Result<Mixed, Throw> {
    self.set_p_name(name.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn setScalarType(&self, mut scalarType: U_PhpParser_Node_Identifier_or_Str) -> Result<crate::php_parser::builder::Enum_, Throw> {
    self.set_p_scalarType(Some(cast::<crate::php_parser::node::Identifier>(crate::php_parser::BuilderHelpers::normalizeType(cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name_or_Str>(scalarType.clone()))?)));
    return Ok(self.clone());
    }
    pub fn implement(&self, mut interfaces: List<U_PhpParser_Node_Name_or_Str>) -> Result<crate::php_parser::builder::Enum_, Throw> {
    let mut interface: Late<U_PhpParser_Node_Name_or_Str> = Late::uninit();
    'l1: for __kv1 in cast::<Map<ArrayKey, U_PhpParser_Node_Name_or_Str>>(interfaces.clone()).into_iter() {
        interface.set(__kv1.1);
        (*self.p_implements_mut()).push(crate::php_parser::BuilderHelpers::normalizeName(interface.get().clone())?);
    }
    return Ok(self.clone());
    }
    pub fn addStmt(&self, mut stmt: U_PhpParser_Builder_or_PhpParser_Node_Stmt) -> Result<crate::php_parser::builder::Enum_, Throw> {
    stmt = cast::<U_PhpParser_Builder_or_PhpParser_Node_Stmt>(crate::php_parser::BuilderHelpers::normalizeNode(cast::<U_PhpParser_Builder_or_PhpParser_Node>(stmt.clone()))?);
    if is_instance::<crate::php_parser::node::stmt::EnumCase>(&cast::<crate::php_parser::Node>(stmt.clone())) {
        (*self.p_enumCases_mut()).push(cast::<crate::php_parser::node::stmt::EnumCase>(stmt.clone()));
    } else if is_instance::<crate::php_parser::node::stmt::ClassMethod>(&cast::<crate::php_parser::Node>(stmt.clone())) {
        (*self.p_methods_mut()).push(cast::<crate::php_parser::node::stmt::ClassMethod>(stmt.clone()));
    } else if is_instance::<crate::php_parser::node::stmt::TraitUse>(&cast::<crate::php_parser::Node>(stmt.clone())) {
        (*self.p_uses_mut()).push(cast::<crate::php_parser::node::stmt::TraitUse>(stmt.clone()));
    } else if is_instance::<crate::php_parser::node::stmt::ClassConst>(&cast::<crate::php_parser::Node>(stmt.clone())) {
        (*self.p_constants_mut()).push(cast::<crate::php_parser::node::stmt::ClassConst>(stmt.clone()));
    } else {
        return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(sprintf(&Str::from_static("Unexpected node of type \"%s\""), &[FmtArg::from(cast::<crate::php_parser::Node>(stmt.clone()).getType()?)])?, 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    return Ok(self.clone());
    }
    pub fn addAttribute(&self, mut attribute: U_PhpParser_Node_Attribute_or_PhpParser_Node_AttributeGroup) -> Result<crate::php_parser::builder::Enum_, Throw> {
    (*self.p_attributeGroups_mut()).push(crate::php_parser::BuilderHelpers::normalizeAttribute(attribute.clone())?);
    return Ok(self.clone());
    }
    pub fn getNode(&self) -> Result<crate::php_parser::node::stmt::Enum_, Throw> {
    return Ok(crate::php_parser::node::stmt::Enum_::new(Some(U_PhpParser_Node_Identifier_or_Str::Str(self.p_name_get())), Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_implement_38f3035d6c { scalarType: self.p_scalarType_get(), implements: Some(cast::<Map<ArrayKey, crate::php_parser::node::Name>>(self.p_implements_get())), stmts: Some(cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(array_merge_l(&[&self.p_uses_get().map_elems(|v| U_PhpParser_Node_Stmt_ClassConst_or_PhpParser_Node_Stmt_ClassMethod_or_af5c5f32a4::PhpParser_Node_Stmt_TraitUse(v)), &self.p_enumCases_get().map_elems(|v| U_PhpParser_Node_Stmt_ClassConst_or_PhpParser_Node_Stmt_ClassMethod_or_af5c5f32a4::PhpParser_Node_Stmt_EnumCase(v)), &self.p_constants_get().map_elems(|v| U_PhpParser_Node_Stmt_ClassConst_or_PhpParser_Node_Stmt_ClassMethod_or_af5c5f32a4::PhpParser_Node_Stmt_ClassConst(v)), &self.p_methods_get().map_elems(|v| U_PhpParser_Node_Stmt_ClassConst_or_PhpParser_Node_Stmt_ClassMethod_or_af5c5f32a4::PhpParser_Node_Stmt_ClassMethod(v))]).map_elems(|v| cast::<crate::php_parser::node::Stmt>(v)))), attrGroups: Some(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(self.p_attributeGroups_get())) }, self.p_attributes_get())?);
    }
    pub fn addStmts(&self, mut stmts: Map<ArrayKey, U_PhpParser_Builder_or_PhpParser_Node_Stmt>) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).addStmts__impl(stmts) }
    pub fn setDocComment(&self, mut docComment: U_PhpParser_Comment_Doc_or_Str) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).setDocComment__impl(docComment) }
    pub fn new_same_class(&self, mut name: Str) -> Result<crate::php_parser::builder::Enum_, Throw> { Ok(Self::new(name)?) }
}
impl php_rt::PhpObject for Enum_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Builder\\Enum_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\builder\\enum_", "phpparser\\builder\\declaration", "phpparser\\builder"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_scalarType_get()) { out.push((Str::from_static("scalarType"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_implements_get()) { out.push((Str::from_static("implements"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_uses_get()) { out.push((Str::from_static("uses"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_enumCases_get()) { out.push((Str::from_static("enumCases"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_constants_get()) { out.push((Str::from_static("constants"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_methods_get()) { out.push((Str::from_static("methods"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attributeGroups_get()) { out.push((Str::from_static("attributeGroups"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(cast::<Str>(value)); true }, "scalarType" => { self.set_p_scalarType(value.to_option().map(|__m| cast::<crate::php_parser::node::Identifier>(__m))); true }, "implements" => { self.set_p_implements(cast::<List<crate::php_parser::node::Name>>(value)); true }, "uses" => { self.set_p_uses(cast::<List<crate::php_parser::node::stmt::TraitUse>>(value)); true }, "enumCases" => { self.set_p_enumCases(cast::<List<crate::php_parser::node::stmt::EnumCase>>(value)); true }, "constants" => { self.set_p_constants(cast::<List<crate::php_parser::node::stmt::ClassConst>>(value)); true }, "methods" => { self.set_p_methods(cast::<List<crate::php_parser::node::stmt::ClassMethod>>(value)); true }, "attributeGroups" => { self.set_p_attributeGroups(cast::<List<crate::php_parser::node::AttributeGroup>>(value)); true }, _ => false } }
}
impl Enum_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Builder\\Enum_ could not be converted to string"))) } }
impl php_rt::PhpClone for Enum_ { fn php_clone(&self) -> Self { let c = Enum_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Enum_Obj { fn clone(&self) -> Self { Enum_Obj { attributes: self.attributes.clone(), name: self.name.clone(), scalarType: self.scalarType.clone(), implements: self.implements.clone(), uses: self.uses.clone(), enumCases: self.enumCases.clone(), constants: self.constants.clone(), methods: self.methods.clone(), attributeGroups: self.attributeGroups.clone() } } }
impl Enum_ {
}
#[derive(Clone)]
pub enum FunctionLike {
    PhpParser_Builder_Function_(crate::php_parser::builder::Function_),
    PhpParser_Builder_Method(crate::php_parser::builder::Method),
}
impl php_rt::PhpObject for FunctionLike {
    fn class_name(&self) -> &'static str { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.class_name(), FunctionLike::PhpParser_Builder_Method(__h) => __h.class_name(), _ => unreachable!() } }
    fn class_ancestors(&self) -> &'static [&'static str] { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.class_ancestors(), FunctionLike::PhpParser_Builder_Method(__h) => __h.class_ancestors(), _ => unreachable!() } }
    fn obj_id(&self) -> usize { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.obj_id(), FunctionLike::PhpParser_Builder_Method(__h) => __h.obj_id(), _ => unreachable!() } }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.props(), FunctionLike::PhpParser_Builder_Method(__h) => __h.props(), _ => unreachable!() } }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.set_prop(name, value), FunctionLike::PhpParser_Builder_Method(__h) => __h.set_prop(name, value), _ => unreachable!() } }
    fn php_to_string(&self) -> Option<Str> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.php_to_string(), FunctionLike::PhpParser_Builder_Method(__h) => __h.php_to_string(), _ => unreachable!() } }
}
impl FunctionLike { pub fn to_php_string(&self) -> Result<Str, Throw> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.to_php_string(), FunctionLike::PhpParser_Builder_Method(__h) => __h.to_php_string(), _ => unreachable!() } } }
impl php_rt::PhpClone for FunctionLike { fn php_clone(&self) -> Self { match self { FunctionLike::PhpParser_Builder_Function_(__h) => FunctionLike::PhpParser_Builder_Function_(__h.php_clone()), FunctionLike::PhpParser_Builder_Method(__h) => FunctionLike::PhpParser_Builder_Method(__h.php_clone()), _ => unreachable!() } } }
impl FunctionLike {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_attributes(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_attributes(), _ => unreachable!() } }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_attributes_get(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_attributes_get(), _ => unreachable!() } }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_attributes_opt(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_attributes_opt(), _ => unreachable!() } }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_attributes_mut(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_attributes_mut(), _ => unreachable!() } }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.set_p_attributes(v), FunctionLike::PhpParser_Builder_Method(__h) => __h.set_p_attributes(v), _ => unreachable!() } }
    pub fn p_returnByRef(&self) -> Ref<'_, bool> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_returnByRef(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_returnByRef(), _ => unreachable!() } }
    pub fn p_returnByRef_get(&self) -> bool { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_returnByRef_get(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_returnByRef_get(), _ => unreachable!() } }
    pub fn p_returnByRef_opt(&self) -> Option<bool> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_returnByRef_opt(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_returnByRef_opt(), _ => unreachable!() } }
    pub fn p_returnByRef_mut(&self) -> RefMut<'_, bool> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_returnByRef_mut(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_returnByRef_mut(), _ => unreachable!() } }
    pub fn set_p_returnByRef(&self, v: bool) { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.set_p_returnByRef(v), FunctionLike::PhpParser_Builder_Method(__h) => __h.set_p_returnByRef(v), _ => unreachable!() } }
    pub fn p_params(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Param>> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_params(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_params(), _ => unreachable!() } }
    pub fn p_params_get(&self) -> Map<ArrayKey, crate::php_parser::node::Param> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_params_get(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_params_get(), _ => unreachable!() } }
    pub fn p_params_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Param>> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_params_opt(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_params_opt(), _ => unreachable!() } }
    pub fn p_params_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Param>> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_params_mut(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_params_mut(), _ => unreachable!() } }
    pub fn set_p_params(&self, v: Map<ArrayKey, crate::php_parser::node::Param>) { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.set_p_params(v), FunctionLike::PhpParser_Builder_Method(__h) => __h.set_p_params(v), _ => unreachable!() } }
    pub fn p_returnType(&self) -> Ref<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_returnType(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_returnType(), _ => unreachable!() } }
    pub fn p_returnType_get(&self) -> Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_returnType_get(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_returnType_get(), _ => unreachable!() } }
    pub fn p_returnType_opt(&self) -> Option<Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_returnType_opt(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_returnType_opt(), _ => unreachable!() } }
    pub fn p_returnType_mut(&self) -> RefMut<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.p_returnType_mut(), FunctionLike::PhpParser_Builder_Method(__h) => __h.p_returnType_mut(), _ => unreachable!() } }
    pub fn set_p_returnType(&self, v: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>) { match self { FunctionLike::PhpParser_Builder_Function_(__h) => __h.set_p_returnType(v), FunctionLike::PhpParser_Builder_Method(__h) => __h.set_p_returnType(v), _ => unreachable!() } }
    pub fn makeReturnByRef(&self) -> Result<crate::php_parser::builder::FunctionLike, Throw> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => Ok(__h.makeReturnByRef()?), FunctionLike::PhpParser_Builder_Method(__h) => Ok(__h.makeReturnByRef()?), _ => unreachable!() } }
    pub fn makeReturnByRef__impl(&self) -> Result<crate::php_parser::builder::FunctionLike, Throw> {
    self.set_p_returnByRef(true);
    return Ok(self.clone());
    }
    pub fn addParam(&self, mut param: U_PhpParser_Builder_Param_or_PhpParser_Node_Param) -> Result<crate::php_parser::builder::FunctionLike, Throw> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => Ok(__h.addParam(param)?), FunctionLike::PhpParser_Builder_Method(__h) => Ok(__h.addParam(param)?), _ => unreachable!() } }
    pub fn addParam__impl(&self, mut param: U_PhpParser_Builder_Param_or_PhpParser_Node_Param) -> Result<crate::php_parser::builder::FunctionLike, Throw> {
    param = cast::<U_PhpParser_Builder_Param_or_PhpParser_Node_Param>(crate::php_parser::BuilderHelpers::normalizeNode(cast::<U_PhpParser_Builder_or_PhpParser_Node>(param.clone()))?);
    if (!is_instance::<crate::php_parser::node::Param>(&cast::<crate::php_parser::Node>(param.clone()))) {
        return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(sprintf(&Str::from_static("Expected parameter node, got \"%s\""), &[FmtArg::from(cast::<crate::php_parser::Node>(param.clone()).getType()?)])?, 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    (*self.p_params_mut()).push(cast::<crate::php_parser::node::Param>(param.clone()));
    return Ok(self.clone());
    }
    pub fn addParams(&self, mut params: Map<ArrayKey, U_PhpParser_Builder_Param_or_PhpParser_Node_Param>) -> Result<crate::php_parser::builder::FunctionLike, Throw> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => Ok(__h.addParams(params)?), FunctionLike::PhpParser_Builder_Method(__h) => Ok(__h.addParams(params)?), _ => unreachable!() } }
    pub fn addParams__impl(&self, mut params: Map<ArrayKey, U_PhpParser_Builder_Param_or_PhpParser_Node_Param>) -> Result<crate::php_parser::builder::FunctionLike, Throw> {
    let mut param: Late<U_PhpParser_Builder_Param_or_PhpParser_Node_Param> = Late::uninit();
    'l1: for __kv1 in params.clone().into_iter() {
        param.set(__kv1.1);
        let _: crate::php_parser::builder::FunctionLike = self.addParam(param.get().clone())?;
    }
    return Ok(self.clone());
    }
    pub fn setReturnType(&self, mut type_: U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name_or_Str) -> Result<crate::php_parser::builder::FunctionLike, Throw> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => Ok(__h.setReturnType(type_)?), FunctionLike::PhpParser_Builder_Method(__h) => Ok(__h.setReturnType(type_)?), _ => unreachable!() } }
    pub fn setReturnType__impl(&self, mut type_: U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name_or_Str) -> Result<crate::php_parser::builder::FunctionLike, Throw> {
    self.set_p_returnType(Some(crate::php_parser::BuilderHelpers::normalizeType(type_.clone())?));
    return Ok(self.clone());
    }
    pub fn addStmt(&self, mut stmt: U_PhpParser_Builder_or_PhpParser_Node_Stmt) -> Result<crate::php_parser::builder::Declaration, Throw> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.addStmt(cast::<U_PhpParser_Builder_or_PhpParser_Node>(stmt))?)), FunctionLike::PhpParser_Builder_Method(__h) => Ok(cast::<crate::php_parser::builder::Declaration>(__h.addStmt(cast::<U_PhpParser_Builder_or_PhpParser_Node>(stmt))?)), _ => unreachable!() } }
    pub fn addStmts(&self, mut stmts: Map<ArrayKey, U_PhpParser_Builder_or_PhpParser_Node_Stmt>) -> Result<crate::php_parser::builder::Declaration, Throw> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => Ok(__h.addStmts(stmts)?), FunctionLike::PhpParser_Builder_Method(__h) => Ok(__h.addStmts(stmts)?), _ => unreachable!() } }
    pub fn addStmts__impl(&self, mut stmts: Map<ArrayKey, U_PhpParser_Builder_or_PhpParser_Node_Stmt>) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).addStmts__impl(stmts) }
    pub fn setDocComment(&self, mut docComment: U_PhpParser_Comment_Doc_or_Str) -> Result<crate::php_parser::builder::Declaration, Throw> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => Ok(__h.setDocComment(docComment)?), FunctionLike::PhpParser_Builder_Method(__h) => Ok(__h.setDocComment(docComment)?), _ => unreachable!() } }
    pub fn setDocComment__impl(&self, mut docComment: U_PhpParser_Comment_Doc_or_Str) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).setDocComment__impl(docComment) }
    pub fn getNode(&self) -> Result<crate::php_parser::Node, Throw> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => Ok(cast::<crate::php_parser::Node>(__h.getNode()?)), FunctionLike::PhpParser_Builder_Method(__h) => Ok(cast::<crate::php_parser::Node>(__h.getNode()?)), _ => unreachable!() } }
    pub fn new_same_class(&self) -> Result<FunctionLike, Throw> { match self { FunctionLike::PhpParser_Builder_Function_(__h) => Ok(cast::<crate::php_parser::builder::FunctionLike>(__h.new_same_class(<Str>::default())?)), FunctionLike::PhpParser_Builder_Method(__h) => Ok(cast::<crate::php_parser::builder::FunctionLike>(__h.new_same_class(<Str>::default())?)), _ => unreachable!() } }
}
pub struct Function_Obj {
    pub attributes: Map<Str, Mixed>,
    pub returnByRef: bool,
    pub params: Map<ArrayKey, crate::php_parser::node::Param>,
    pub returnType: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>,
    pub name: Str,
    pub stmts: List<crate::php_parser::node::Stmt>,
    pub attributeGroups: List<crate::php_parser::node::AttributeGroup>,
}
#[derive(Clone)]
pub struct Function_(pub Rc<RefCell<Function_Obj>>);
impl Function_ {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_returnByRef(&self) -> Ref<'_, bool> { Ref::map(self.0.borrow(), |o| &o.returnByRef) }
    pub fn p_returnByRef_get(&self) -> bool { self.0.borrow().returnByRef.clone() }
    pub fn p_returnByRef_opt(&self) -> Option<bool> { Some(self.0.borrow().returnByRef.clone()) }
    pub fn p_returnByRef_mut(&self) -> RefMut<'_, bool> { RefMut::map(self.0.borrow_mut(), |o| &mut o.returnByRef) }
    pub fn set_p_returnByRef(&self, v: bool) { self.0.borrow_mut().returnByRef = v; }
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
    pub fn p_name(&self) -> Ref<'_, Str> { Ref::map(self.0.borrow(), |o| &o.name) }
    pub fn p_name_get(&self) -> Str { self.0.borrow().name.clone() }
    pub fn p_name_opt(&self) -> Option<Str> { Some(self.0.borrow().name.clone()) }
    pub fn p_name_mut(&self) -> RefMut<'_, Str> { RefMut::map(self.0.borrow_mut(), |o| &mut o.name) }
    pub fn set_p_name(&self, v: Str) { self.0.borrow_mut().name = v; }
    pub fn p_stmts(&self) -> Ref<'_, List<crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> List<crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<List<crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: List<crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn p_attributeGroups(&self) -> Ref<'_, List<crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attributeGroups) }
    pub fn p_attributeGroups_get(&self) -> List<crate::php_parser::node::AttributeGroup> { self.0.borrow().attributeGroups.clone() }
    pub fn p_attributeGroups_opt(&self) -> Option<List<crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attributeGroups.clone()) }
    pub fn p_attributeGroups_mut(&self) -> RefMut<'_, List<crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributeGroups) }
    pub fn set_p_attributeGroups(&self, v: List<crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attributeGroups = v; }
    pub fn new(mut name: Str) -> Result<Function_, Throw> {
        let this = Function_(Rc::new(RefCell::new(Function_Obj {
            attributes: Map::<Str, Mixed>::new(),
            returnByRef: false,
            params: Map::<ArrayKey, crate::php_parser::node::Param>::new(),
            returnType: { let _ = (); None::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> },
            name: Default::default(),
            stmts: List::<crate::php_parser::node::Stmt>::new(),
            attributeGroups: List::<crate::php_parser::node::AttributeGroup>::new(),
        })));
        this.magic__construct(name)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: Str) -> Result<Mixed, Throw> {
    self.set_p_name(name.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn addStmt(&self, mut stmt: U_PhpParser_Builder_or_PhpParser_Node) -> Result<crate::php_parser::builder::Function_, Throw> {
    (*self.p_stmts_mut()).push(crate::php_parser::BuilderHelpers::normalizeStmt(stmt.clone())?);
    return Ok(self.clone());
    }
    pub fn addAttribute(&self, mut attribute: U_PhpParser_Node_Attribute_or_PhpParser_Node_AttributeGroup) -> Result<crate::php_parser::builder::Function_, Throw> {
    (*self.p_attributeGroups_mut()).push(crate::php_parser::BuilderHelpers::normalizeAttribute(attribute.clone())?);
    return Ok(self.clone());
    }
    pub fn getNode(&self) -> Result<crate::php_parser::node::stmt::Function_, Throw> {
    return Ok(crate::php_parser::node::stmt::Function_::new(U_PhpParser_Node_Identifier_or_Str::Str(self.p_name_get()), Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_b3b37d5382 { byRef: Some(self.p_returnByRef_get()), params: Some(self.p_params_get()), returnType: self.p_returnType_get(), stmts: Some(cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(self.p_stmts_get())), attrGroups: Some(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(self.p_attributeGroups_get())) }, self.p_attributes_get())?);
    }
    pub fn makeReturnByRef(&self) -> Result<crate::php_parser::builder::FunctionLike, Throw> { cast::<crate::php_parser::builder::FunctionLike>(self.clone()).makeReturnByRef__impl() }
    pub fn addParam(&self, mut param: U_PhpParser_Builder_Param_or_PhpParser_Node_Param) -> Result<crate::php_parser::builder::FunctionLike, Throw> { cast::<crate::php_parser::builder::FunctionLike>(self.clone()).addParam__impl(param) }
    pub fn addParams(&self, mut params: Map<ArrayKey, U_PhpParser_Builder_Param_or_PhpParser_Node_Param>) -> Result<crate::php_parser::builder::FunctionLike, Throw> { cast::<crate::php_parser::builder::FunctionLike>(self.clone()).addParams__impl(params) }
    pub fn setReturnType(&self, mut type_: U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name_or_Str) -> Result<crate::php_parser::builder::FunctionLike, Throw> { cast::<crate::php_parser::builder::FunctionLike>(self.clone()).setReturnType__impl(type_) }
    pub fn addStmts(&self, mut stmts: Map<ArrayKey, U_PhpParser_Builder_or_PhpParser_Node_Stmt>) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).addStmts__impl(stmts) }
    pub fn setDocComment(&self, mut docComment: U_PhpParser_Comment_Doc_or_Str) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).setDocComment__impl(docComment) }
    pub fn new_same_class(&self, mut name: Str) -> Result<crate::php_parser::builder::Function_, Throw> { Ok(Self::new(name)?) }
}
impl php_rt::PhpObject for Function_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Builder\\Function_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\builder\\function_", "phpparser\\builder\\functionlike", "phpparser\\builder\\declaration", "phpparser\\builder"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_returnByRef_get()) { out.push((Str::from_static("returnByRef"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_params_get()) { out.push((Str::from_static("params"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_returnType_get()) { out.push((Str::from_static("returnType"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attributeGroups_get()) { out.push((Str::from_static("attributeGroups"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "returnByRef" => { self.set_p_returnByRef(cast::<bool>(value)); true }, "params" => { self.set_p_params(cast::<Map<ArrayKey, crate::php_parser::node::Param>>(value)); true }, "returnType" => { self.set_p_returnType(value.to_option().map(|__m| cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(__m))); true }, "name" => { self.set_p_name(cast::<Str>(value)); true }, "stmts" => { self.set_p_stmts(cast::<List<crate::php_parser::node::Stmt>>(value)); true }, "attributeGroups" => { self.set_p_attributeGroups(cast::<List<crate::php_parser::node::AttributeGroup>>(value)); true }, _ => false } }
}
impl Function_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Builder\\Function_ could not be converted to string"))) } }
impl php_rt::PhpClone for Function_ { fn php_clone(&self) -> Self { let c = Function_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Function_Obj { fn clone(&self) -> Self { Function_Obj { attributes: self.attributes.clone(), returnByRef: self.returnByRef.clone(), params: self.params.clone(), returnType: self.returnType.clone(), name: self.name.clone(), stmts: self.stmts.clone(), attributeGroups: self.attributeGroups.clone() } } }
impl Function_ {
}
pub struct Interface_Obj {
    pub attributes: Map<Str, Mixed>,
    pub name: Str,
    pub extends: List<crate::php_parser::node::Name>,
    pub constants: List<crate::php_parser::node::stmt::ClassConst>,
    pub methods: List<crate::php_parser::node::stmt::ClassMethod>,
    pub attributeGroups: List<crate::php_parser::node::AttributeGroup>,
}
#[derive(Clone)]
pub struct Interface_(pub Rc<RefCell<Interface_Obj>>);
impl Interface_ {
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
    pub fn p_extends(&self) -> Ref<'_, List<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.extends) }
    pub fn p_extends_get(&self) -> List<crate::php_parser::node::Name> { self.0.borrow().extends.clone() }
    pub fn p_extends_opt(&self) -> Option<List<crate::php_parser::node::Name>> { Some(self.0.borrow().extends.clone()) }
    pub fn p_extends_mut(&self) -> RefMut<'_, List<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.extends) }
    pub fn set_p_extends(&self, v: List<crate::php_parser::node::Name>) { self.0.borrow_mut().extends = v; }
    pub fn p_constants(&self) -> Ref<'_, List<crate::php_parser::node::stmt::ClassConst>> { Ref::map(self.0.borrow(), |o| &o.constants) }
    pub fn p_constants_get(&self) -> List<crate::php_parser::node::stmt::ClassConst> { self.0.borrow().constants.clone() }
    pub fn p_constants_opt(&self) -> Option<List<crate::php_parser::node::stmt::ClassConst>> { Some(self.0.borrow().constants.clone()) }
    pub fn p_constants_mut(&self) -> RefMut<'_, List<crate::php_parser::node::stmt::ClassConst>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.constants) }
    pub fn set_p_constants(&self, v: List<crate::php_parser::node::stmt::ClassConst>) { self.0.borrow_mut().constants = v; }
    pub fn p_methods(&self) -> Ref<'_, List<crate::php_parser::node::stmt::ClassMethod>> { Ref::map(self.0.borrow(), |o| &o.methods) }
    pub fn p_methods_get(&self) -> List<crate::php_parser::node::stmt::ClassMethod> { self.0.borrow().methods.clone() }
    pub fn p_methods_opt(&self) -> Option<List<crate::php_parser::node::stmt::ClassMethod>> { Some(self.0.borrow().methods.clone()) }
    pub fn p_methods_mut(&self) -> RefMut<'_, List<crate::php_parser::node::stmt::ClassMethod>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.methods) }
    pub fn set_p_methods(&self, v: List<crate::php_parser::node::stmt::ClassMethod>) { self.0.borrow_mut().methods = v; }
    pub fn p_attributeGroups(&self) -> Ref<'_, List<crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attributeGroups) }
    pub fn p_attributeGroups_get(&self) -> List<crate::php_parser::node::AttributeGroup> { self.0.borrow().attributeGroups.clone() }
    pub fn p_attributeGroups_opt(&self) -> Option<List<crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attributeGroups.clone()) }
    pub fn p_attributeGroups_mut(&self) -> RefMut<'_, List<crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributeGroups) }
    pub fn set_p_attributeGroups(&self, v: List<crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attributeGroups = v; }
    pub fn new(mut name: Str) -> Result<Interface_, Throw> {
        let this = Interface_(Rc::new(RefCell::new(Interface_Obj {
            attributes: Map::<Str, Mixed>::new(),
            name: Default::default(),
            extends: List::<crate::php_parser::node::Name>::new(),
            constants: List::<crate::php_parser::node::stmt::ClassConst>::new(),
            methods: List::<crate::php_parser::node::stmt::ClassMethod>::new(),
            attributeGroups: List::<crate::php_parser::node::AttributeGroup>::new(),
        })));
        this.magic__construct(name)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: Str) -> Result<Mixed, Throw> {
    self.set_p_name(name.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn extend(&self, mut interfaces: List<U_PhpParser_Node_Name_or_Str>) -> Result<crate::php_parser::builder::Interface_, Throw> {
    let mut interface: Late<U_PhpParser_Node_Name_or_Str> = Late::uninit();
    'l1: for __kv1 in cast::<Map<ArrayKey, U_PhpParser_Node_Name_or_Str>>(interfaces.clone()).into_iter() {
        interface.set(__kv1.1);
        (*self.p_extends_mut()).push(crate::php_parser::BuilderHelpers::normalizeName(interface.get().clone())?);
    }
    return Ok(self.clone());
    }
    pub fn addStmt(&self, mut stmt: U_PhpParser_Builder_or_PhpParser_Node_Stmt) -> Result<crate::php_parser::builder::Interface_, Throw> {
    stmt = cast::<U_PhpParser_Builder_or_PhpParser_Node_Stmt>(crate::php_parser::BuilderHelpers::normalizeNode(cast::<U_PhpParser_Builder_or_PhpParser_Node>(stmt.clone()))?);
    if is_instance::<crate::php_parser::node::stmt::ClassConst>(&cast::<crate::php_parser::Node>(stmt.clone())) {
        (*self.p_constants_mut()).push(cast::<crate::php_parser::node::stmt::ClassConst>(stmt.clone()));
    } else if is_instance::<crate::php_parser::node::stmt::ClassMethod>(&cast::<crate::php_parser::Node>(stmt.clone())) {
        cast::<crate::php_parser::node::stmt::ClassMethod>(stmt.clone()).set_p_stmts({ let _ = (); None::<Map<ArrayKey, crate::php_parser::node::Stmt>> });
        (*self.p_methods_mut()).push(cast::<crate::php_parser::node::stmt::ClassMethod>(stmt.clone()));
    } else {
        return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(sprintf(&Str::from_static("Unexpected node of type \"%s\""), &[FmtArg::from(cast::<crate::php_parser::Node>(stmt.clone()).getType()?)])?, 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    return Ok(self.clone());
    }
    pub fn addAttribute(&self, mut attribute: U_PhpParser_Node_Attribute_or_PhpParser_Node_AttributeGroup) -> Result<crate::php_parser::builder::Interface_, Throw> {
    (*self.p_attributeGroups_mut()).push(crate::php_parser::BuilderHelpers::normalizeAttribute(attribute.clone())?);
    return Ok(self.clone());
    }
    pub fn getNode(&self) -> Result<crate::php_parser::node::stmt::Interface_, Throw> {
    return Ok(crate::php_parser::node::stmt::Interface_::new(U_PhpParser_Node_Identifier_or_Str::Str(self.p_name_get()), Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_extendsq__786d9d6046 { extends: Some(cast::<Map<ArrayKey, crate::php_parser::node::Name>>(self.p_extends_get())), stmts: Some(cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(array_merge_l(&[&self.p_constants_get().map_elems(|v| U_PhpParser_Node_Stmt_ClassConst_or_PhpParser_Node_Stmt_ClassMethod::PhpParser_Node_Stmt_ClassConst(v)), &self.p_methods_get().map_elems(|v| U_PhpParser_Node_Stmt_ClassConst_or_PhpParser_Node_Stmt_ClassMethod::PhpParser_Node_Stmt_ClassMethod(v))]).map_elems(|v| cast::<crate::php_parser::node::Stmt>(v)))), attrGroups: Some(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(self.p_attributeGroups_get())) }, self.p_attributes_get())?);
    }
    pub fn addStmts(&self, mut stmts: Map<ArrayKey, U_PhpParser_Builder_or_PhpParser_Node_Stmt>) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).addStmts__impl(stmts) }
    pub fn setDocComment(&self, mut docComment: U_PhpParser_Comment_Doc_or_Str) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).setDocComment__impl(docComment) }
    pub fn new_same_class(&self, mut name: Str) -> Result<crate::php_parser::builder::Interface_, Throw> { Ok(Self::new(name)?) }
}
impl php_rt::PhpObject for Interface_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Builder\\Interface_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\builder\\interface_", "phpparser\\builder\\declaration", "phpparser\\builder"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_extends_get()) { out.push((Str::from_static("extends"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_constants_get()) { out.push((Str::from_static("constants"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_methods_get()) { out.push((Str::from_static("methods"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attributeGroups_get()) { out.push((Str::from_static("attributeGroups"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(cast::<Str>(value)); true }, "extends" => { self.set_p_extends(cast::<List<crate::php_parser::node::Name>>(value)); true }, "constants" => { self.set_p_constants(cast::<List<crate::php_parser::node::stmt::ClassConst>>(value)); true }, "methods" => { self.set_p_methods(cast::<List<crate::php_parser::node::stmt::ClassMethod>>(value)); true }, "attributeGroups" => { self.set_p_attributeGroups(cast::<List<crate::php_parser::node::AttributeGroup>>(value)); true }, _ => false } }
}
impl Interface_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Builder\\Interface_ could not be converted to string"))) } }
impl php_rt::PhpClone for Interface_ { fn php_clone(&self) -> Self { let c = Interface_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Interface_Obj { fn clone(&self) -> Self { Interface_Obj { attributes: self.attributes.clone(), name: self.name.clone(), extends: self.extends.clone(), constants: self.constants.clone(), methods: self.methods.clone(), attributeGroups: self.attributeGroups.clone() } } }
impl Interface_ {
}
pub struct MethodObj {
    pub attributes: Map<Str, Mixed>,
    pub returnByRef: bool,
    pub params: Map<ArrayKey, crate::php_parser::node::Param>,
    pub returnType: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>,
    pub name: Str,
    pub flags: i64,
    pub stmts: Option<List<crate::php_parser::node::Stmt>>,
    pub attributeGroups: List<crate::php_parser::node::AttributeGroup>,
}
#[derive(Clone)]
pub struct Method(pub Rc<RefCell<MethodObj>>);
impl Method {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_returnByRef(&self) -> Ref<'_, bool> { Ref::map(self.0.borrow(), |o| &o.returnByRef) }
    pub fn p_returnByRef_get(&self) -> bool { self.0.borrow().returnByRef.clone() }
    pub fn p_returnByRef_opt(&self) -> Option<bool> { Some(self.0.borrow().returnByRef.clone()) }
    pub fn p_returnByRef_mut(&self) -> RefMut<'_, bool> { RefMut::map(self.0.borrow_mut(), |o| &mut o.returnByRef) }
    pub fn set_p_returnByRef(&self, v: bool) { self.0.borrow_mut().returnByRef = v; }
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
    pub fn p_name(&self) -> Ref<'_, Str> { Ref::map(self.0.borrow(), |o| &o.name) }
    pub fn p_name_get(&self) -> Str { self.0.borrow().name.clone() }
    pub fn p_name_opt(&self) -> Option<Str> { Some(self.0.borrow().name.clone()) }
    pub fn p_name_mut(&self) -> RefMut<'_, Str> { RefMut::map(self.0.borrow_mut(), |o| &mut o.name) }
    pub fn set_p_name(&self, v: Str) { self.0.borrow_mut().name = v; }
    pub fn p_flags(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.flags) }
    pub fn p_flags_get(&self) -> i64 { self.0.borrow().flags.clone() }
    pub fn p_flags_opt(&self) -> Option<i64> { Some(self.0.borrow().flags.clone()) }
    pub fn p_flags_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.flags) }
    pub fn set_p_flags(&self, v: i64) { self.0.borrow_mut().flags = v; }
    pub fn p_stmts(&self) -> Ref<'_, Option<List<crate::php_parser::node::Stmt>>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> Option<List<crate::php_parser::node::Stmt>> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<Option<List<crate::php_parser::node::Stmt>>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, Option<List<crate::php_parser::node::Stmt>>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: Option<List<crate::php_parser::node::Stmt>>) { self.0.borrow_mut().stmts = v; }
    pub fn p_attributeGroups(&self) -> Ref<'_, List<crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attributeGroups) }
    pub fn p_attributeGroups_get(&self) -> List<crate::php_parser::node::AttributeGroup> { self.0.borrow().attributeGroups.clone() }
    pub fn p_attributeGroups_opt(&self) -> Option<List<crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attributeGroups.clone()) }
    pub fn p_attributeGroups_mut(&self) -> RefMut<'_, List<crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributeGroups) }
    pub fn set_p_attributeGroups(&self, v: List<crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attributeGroups = v; }
    pub fn new(mut name: Str) -> Result<Method, Throw> {
        let this = Method(Rc::new(RefCell::new(MethodObj {
            attributes: Map::<Str, Mixed>::new(),
            returnByRef: false,
            params: Map::<ArrayKey, crate::php_parser::node::Param>::new(),
            returnType: { let _ = (); None::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> },
            name: Default::default(),
            flags: 0i64,
            stmts: Some(List::<crate::php_parser::node::Stmt>::new()),
            attributeGroups: List::<crate::php_parser::node::AttributeGroup>::new(),
        })));
        this.magic__construct(name)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: Str) -> Result<Mixed, Throw> {
    self.set_p_name(name.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn makePublic(&self) -> Result<crate::php_parser::builder::Method, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PUBLIC())?);
    return Ok(self.clone());
    }
    pub fn makeProtected(&self) -> Result<crate::php_parser::builder::Method, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PROTECTED())?);
    return Ok(self.clone());
    }
    pub fn makePrivate(&self) -> Result<crate::php_parser::builder::Method, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PRIVATE())?);
    return Ok(self.clone());
    }
    pub fn makeStatic(&self) -> Result<crate::php_parser::builder::Method, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::STATIC())?);
    return Ok(self.clone());
    }
    pub fn makeAbstract(&self) -> Result<crate::php_parser::builder::Method, Throw> {
    if (!(!truthy(&Some(self.clone()).and_then(|__b| Some(__b.p_stmts_get())).flatten()))) {
        return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(Str::from_static("Cannot make method with statements abstract"), 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::ABSTRACT())?);
    self.set_p_stmts({ let _ = (); None::<List<crate::php_parser::node::Stmt>> });
    return Ok(self.clone());
    }
    pub fn makeFinal(&self) -> Result<crate::php_parser::builder::Method, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::FINAL())?);
    return Ok(self.clone());
    }
    pub fn addStmt(&self, mut stmt: U_PhpParser_Builder_or_PhpParser_Node) -> Result<crate::php_parser::builder::Method, Throw> {
    if Some(self.clone()).and_then(|__b| Some(__b.p_stmts_get())).flatten().is_none() {
        return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(Str::from_static("Cannot add statements to an abstract method"), 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    (*(*self.p_stmts_mut()).get_or_insert_with(Default::default)).push(crate::php_parser::BuilderHelpers::normalizeStmt(stmt.clone())?);
    return Ok(self.clone());
    }
    pub fn addAttribute(&self, mut attribute: U_PhpParser_Node_Attribute_or_PhpParser_Node_AttributeGroup) -> Result<crate::php_parser::builder::Method, Throw> {
    (*self.p_attributeGroups_mut()).push(crate::php_parser::BuilderHelpers::normalizeAttribute(attribute.clone())?);
    return Ok(self.clone());
    }
    pub fn getNode(&self) -> Result<crate::php_parser::node::stmt::ClassMethod, Throw> {
    return Ok(crate::php_parser::node::stmt::ClassMethod::new(U_PhpParser_Node_Identifier_or_Str::Str(self.p_name_get()), Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_byRefq_Bo_6534c97760 { flags: Some(self.p_flags_get()), byRef: Some(self.p_returnByRef_get()), params: Some(self.p_params_get()), returnType: self.p_returnType_get(), stmts: self.p_stmts_get().map(|v| cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(v)), attrGroups: Some(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(self.p_attributeGroups_get())) }, self.p_attributes_get())?);
    }
    pub fn makeReturnByRef(&self) -> Result<crate::php_parser::builder::FunctionLike, Throw> { cast::<crate::php_parser::builder::FunctionLike>(self.clone()).makeReturnByRef__impl() }
    pub fn addParam(&self, mut param: U_PhpParser_Builder_Param_or_PhpParser_Node_Param) -> Result<crate::php_parser::builder::FunctionLike, Throw> { cast::<crate::php_parser::builder::FunctionLike>(self.clone()).addParam__impl(param) }
    pub fn addParams(&self, mut params: Map<ArrayKey, U_PhpParser_Builder_Param_or_PhpParser_Node_Param>) -> Result<crate::php_parser::builder::FunctionLike, Throw> { cast::<crate::php_parser::builder::FunctionLike>(self.clone()).addParams__impl(params) }
    pub fn setReturnType(&self, mut type_: U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name_or_Str) -> Result<crate::php_parser::builder::FunctionLike, Throw> { cast::<crate::php_parser::builder::FunctionLike>(self.clone()).setReturnType__impl(type_) }
    pub fn addStmts(&self, mut stmts: Map<ArrayKey, U_PhpParser_Builder_or_PhpParser_Node_Stmt>) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).addStmts__impl(stmts) }
    pub fn setDocComment(&self, mut docComment: U_PhpParser_Comment_Doc_or_Str) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).setDocComment__impl(docComment) }
    pub fn new_same_class(&self, mut name: Str) -> Result<crate::php_parser::builder::Method, Throw> { Ok(Self::new(name)?) }
}
impl php_rt::PhpObject for Method {
    fn class_name(&self) -> &'static str { "PhpParser\\Builder\\Method" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\builder\\method", "phpparser\\builder\\functionlike", "phpparser\\builder\\declaration", "phpparser\\builder"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_returnByRef_get()) { out.push((Str::from_static("returnByRef"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_params_get()) { out.push((Str::from_static("params"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_returnType_get()) { out.push((Str::from_static("returnType"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_flags_get()) { out.push((Str::from_static("flags"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attributeGroups_get()) { out.push((Str::from_static("attributeGroups"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "returnByRef" => { self.set_p_returnByRef(cast::<bool>(value)); true }, "params" => { self.set_p_params(cast::<Map<ArrayKey, crate::php_parser::node::Param>>(value)); true }, "returnType" => { self.set_p_returnType(value.to_option().map(|__m| cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(__m))); true }, "name" => { self.set_p_name(cast::<Str>(value)); true }, "flags" => { self.set_p_flags(cast::<i64>(value)); true }, "stmts" => { self.set_p_stmts(value.to_option().map(|__m| cast::<List<crate::php_parser::node::Stmt>>(__m))); true }, "attributeGroups" => { self.set_p_attributeGroups(cast::<List<crate::php_parser::node::AttributeGroup>>(value)); true }, _ => false } }
}
impl Method { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Builder\\Method could not be converted to string"))) } }
impl php_rt::PhpClone for Method { fn php_clone(&self) -> Self { let c = Method(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for MethodObj { fn clone(&self) -> Self { MethodObj { attributes: self.attributes.clone(), returnByRef: self.returnByRef.clone(), params: self.params.clone(), returnType: self.returnType.clone(), name: self.name.clone(), flags: self.flags.clone(), stmts: self.stmts.clone(), attributeGroups: self.attributeGroups.clone() } } }
impl Method {
}
pub struct Namespace_Obj {
    pub attributes: Map<Str, Mixed>,
    pub name: Option<crate::php_parser::node::Name>,
    pub stmts: Map<ArrayKey, crate::php_parser::node::Stmt>,
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
    pub fn p_stmts(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> Map<ArrayKey, crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: Map<ArrayKey, crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn new(mut name: Option<U_PhpParser_Node_Name_or_Str>) -> Result<Namespace_, Throw> {
        let this = Namespace_(Rc::new(RefCell::new(Namespace_Obj {
            attributes: Map::<Str, Mixed>::new(),
            name: Default::default(),
            stmts: Map::<ArrayKey, crate::php_parser::node::Stmt>::new(),
        })));
        this.magic__construct(name)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: Option<U_PhpParser_Node_Name_or_Str>) -> Result<Mixed, Throw> {
    self.set_p_name((if (!name.clone().is_none()) { Some(crate::php_parser::BuilderHelpers::normalizeName(name.clone().unwrap())?) } else { { let _ = (); None::<crate::php_parser::node::Name> } }));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn addStmt(&self, mut stmt: U_PhpParser_Builder_or_PhpParser_Node) -> Result<crate::php_parser::builder::Namespace_, Throw> {
    (*self.p_stmts_mut()).push(crate::php_parser::BuilderHelpers::normalizeStmt(stmt.clone())?);
    return Ok(self.clone());
    }
    pub fn getNode(&self) -> Result<crate::php_parser::node::stmt::Namespace_, Throw> {
    return Ok(crate::php_parser::node::stmt::Namespace_::new(self.p_name_get(), Some(self.p_stmts_get()), self.p_attributes_get())?);
    }
    pub fn addStmts(&self, mut stmts: Map<ArrayKey, U_PhpParser_Builder_or_PhpParser_Node_Stmt>) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).addStmts__impl(stmts) }
    pub fn setDocComment(&self, mut docComment: U_PhpParser_Comment_Doc_or_Str) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).setDocComment__impl(docComment) }
    pub fn new_same_class(&self, mut name: Option<U_PhpParser_Node_Name_or_Str>) -> Result<crate::php_parser::builder::Namespace_, Throw> { Ok(Self::new(name)?) }
}
impl php_rt::PhpObject for Namespace_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Builder\\Namespace_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\builder\\namespace_", "phpparser\\builder\\declaration", "phpparser\\builder"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(value.to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m))); true }, "stmts" => { self.set_p_stmts(cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(value)); true }, _ => false } }
}
impl Namespace_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Builder\\Namespace_ could not be converted to string"))) } }
impl php_rt::PhpClone for Namespace_ { fn php_clone(&self) -> Self { let c = Namespace_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Namespace_Obj { fn clone(&self) -> Self { Namespace_Obj { attributes: self.attributes.clone(), name: self.name.clone(), stmts: self.stmts.clone() } } }
impl Namespace_ {
}
pub struct ParamObj {
    pub name: Str,
    pub default: Option<crate::php_parser::node::Expr>,
    pub type_: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>,
    pub byRef: bool,
    pub flags: i64,
    pub variadic: bool,
    pub attributeGroups: List<crate::php_parser::node::AttributeGroup>,
}
#[derive(Clone)]
pub struct Param(pub Rc<RefCell<ParamObj>>);
impl Param {
    pub fn p_name(&self) -> Ref<'_, Str> { Ref::map(self.0.borrow(), |o| &o.name) }
    pub fn p_name_get(&self) -> Str { self.0.borrow().name.clone() }
    pub fn p_name_opt(&self) -> Option<Str> { Some(self.0.borrow().name.clone()) }
    pub fn p_name_mut(&self) -> RefMut<'_, Str> { RefMut::map(self.0.borrow_mut(), |o| &mut o.name) }
    pub fn set_p_name(&self, v: Str) { self.0.borrow_mut().name = v; }
    pub fn p_default(&self) -> Ref<'_, Option<crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.default) }
    pub fn p_default_get(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().default.clone() }
    pub fn p_default_opt(&self) -> Option<Option<crate::php_parser::node::Expr>> { Some(self.0.borrow().default.clone()) }
    pub fn p_default_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.default) }
    pub fn set_p_default(&self, v: Option<crate::php_parser::node::Expr>) { self.0.borrow_mut().default = v; }
    pub fn p_type_(&self) -> Ref<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { Ref::map(self.0.borrow(), |o| &o.type_) }
    pub fn p_type__get(&self) -> Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> { self.0.borrow().type_.clone() }
    pub fn p_type__opt(&self) -> Option<Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { Some(self.0.borrow().type_.clone()) }
    pub fn p_type__mut(&self) -> RefMut<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.type_) }
    pub fn set_p_type_(&self, v: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>) { self.0.borrow_mut().type_ = v; }
    pub fn p_byRef(&self) -> Ref<'_, bool> { Ref::map(self.0.borrow(), |o| &o.byRef) }
    pub fn p_byRef_get(&self) -> bool { self.0.borrow().byRef.clone() }
    pub fn p_byRef_opt(&self) -> Option<bool> { Some(self.0.borrow().byRef.clone()) }
    pub fn p_byRef_mut(&self) -> RefMut<'_, bool> { RefMut::map(self.0.borrow_mut(), |o| &mut o.byRef) }
    pub fn set_p_byRef(&self, v: bool) { self.0.borrow_mut().byRef = v; }
    pub fn p_flags(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.flags) }
    pub fn p_flags_get(&self) -> i64 { self.0.borrow().flags.clone() }
    pub fn p_flags_opt(&self) -> Option<i64> { Some(self.0.borrow().flags.clone()) }
    pub fn p_flags_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.flags) }
    pub fn set_p_flags(&self, v: i64) { self.0.borrow_mut().flags = v; }
    pub fn p_variadic(&self) -> Ref<'_, bool> { Ref::map(self.0.borrow(), |o| &o.variadic) }
    pub fn p_variadic_get(&self) -> bool { self.0.borrow().variadic.clone() }
    pub fn p_variadic_opt(&self) -> Option<bool> { Some(self.0.borrow().variadic.clone()) }
    pub fn p_variadic_mut(&self) -> RefMut<'_, bool> { RefMut::map(self.0.borrow_mut(), |o| &mut o.variadic) }
    pub fn set_p_variadic(&self, v: bool) { self.0.borrow_mut().variadic = v; }
    pub fn p_attributeGroups(&self) -> Ref<'_, List<crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attributeGroups) }
    pub fn p_attributeGroups_get(&self) -> List<crate::php_parser::node::AttributeGroup> { self.0.borrow().attributeGroups.clone() }
    pub fn p_attributeGroups_opt(&self) -> Option<List<crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attributeGroups.clone()) }
    pub fn p_attributeGroups_mut(&self) -> RefMut<'_, List<crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributeGroups) }
    pub fn set_p_attributeGroups(&self, v: List<crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attributeGroups = v; }
    pub fn new(mut name: Str) -> Result<Param, Throw> {
        let this = Param(Rc::new(RefCell::new(ParamObj {
            name: Default::default(),
            default: { let _ = (); None::<crate::php_parser::node::Expr> },
            type_: { let _ = (); None::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> },
            byRef: false,
            flags: 0i64,
            variadic: false,
            attributeGroups: List::<crate::php_parser::node::AttributeGroup>::new(),
        })));
        this.magic__construct(name)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: Str) -> Result<Mixed, Throw> {
    self.set_p_name(name.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn setDefault(&self, mut value: Mixed) -> Result<crate::php_parser::builder::Param, Throw> {
    self.set_p_default(Some(crate::php_parser::BuilderHelpers::normalizeValue(value.clone().to_option().map(|__m| cast::<U_Bool_or_Float_or_Int_or_Map_ArrayKey_Mixed_or_PhpParser_Node_Expr_or_Str_or_UnitEnum>(__m)))?));
    return Ok(self.clone());
    }
    pub fn setType(&self, mut type_: U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name_or_Str) -> Result<crate::php_parser::builder::Param, Throw> {
    self.set_p_type_(Some(crate::php_parser::BuilderHelpers::normalizeType(type_.clone())?));
    if loose_eq(&cast::<Mixed>(self.p_type__get().unwrap()), &cast::<Mixed>(Str::from_static("void"))) {
        return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(Str::from_static("Parameter type cannot be void"), 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    return Ok(self.clone());
    }
    pub fn makeByRef(&self) -> Result<crate::php_parser::builder::Param, Throw> {
    self.set_p_byRef(true);
    return Ok(self.clone());
    }
    pub fn makeVariadic(&self) -> Result<crate::php_parser::builder::Param, Throw> {
    self.set_p_variadic(true);
    return Ok(self.clone());
    }
    pub fn makePublic(&self) -> Result<crate::php_parser::builder::Param, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PUBLIC())?);
    return Ok(self.clone());
    }
    pub fn makeProtected(&self) -> Result<crate::php_parser::builder::Param, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PROTECTED())?);
    return Ok(self.clone());
    }
    pub fn makePrivate(&self) -> Result<crate::php_parser::builder::Param, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PRIVATE())?);
    return Ok(self.clone());
    }
    pub fn makeReadonly(&self) -> Result<crate::php_parser::builder::Param, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::READONLY())?);
    return Ok(self.clone());
    }
    pub fn makePrivateSet(&self) -> Result<crate::php_parser::builder::Param, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PRIVATE_SET())?);
    return Ok(self.clone());
    }
    pub fn makeProtectedSet(&self) -> Result<crate::php_parser::builder::Param, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PROTECTED_SET())?);
    return Ok(self.clone());
    }
    pub fn addAttribute(&self, mut attribute: U_PhpParser_Node_Attribute_or_PhpParser_Node_AttributeGroup) -> Result<crate::php_parser::builder::Param, Throw> {
    (*self.p_attributeGroups_mut()).push(crate::php_parser::BuilderHelpers::normalizeAttribute(attribute.clone())?);
    return Ok(self.clone());
    }
    pub fn getNode(&self) -> Result<crate::php_parser::node::Param, Throw> {
    return Ok(crate::php_parser::node::Param::new(U_PhpParser_Node_Expr_Error_or_PhpParser_Node_Expr_Variable::PhpParser_Node_Expr_Variable(crate::php_parser::node::expr::Variable::new(U_PhpParser_Node_Expr_or_Str::Str(self.p_name_get()), Map::<Str, Mixed>::new())?), self.p_default_get(), self.p_type__get(), self.p_byRef_get(), self.p_variadic_get(), Map::<Str, Mixed>::new(), self.p_flags_get(), self.p_attributeGroups_get(), Map::<ArrayKey, crate::php_parser::node::PropertyHook>::new())?);
    }
    pub fn new_same_class(&self, mut name: Str) -> Result<crate::php_parser::builder::Param, Throw> { Ok(Self::new(name)?) }
}
impl php_rt::PhpObject for Param {
    fn class_name(&self) -> &'static str { "PhpParser\\Builder\\Param" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\builder\\param", "phpparser\\builder"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_default_get()) { out.push((Str::from_static("default"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_type__get()) { out.push((Str::from_static("type"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_byRef_get()) { out.push((Str::from_static("byRef"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_flags_get()) { out.push((Str::from_static("flags"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_variadic_get()) { out.push((Str::from_static("variadic"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attributeGroups_get()) { out.push((Str::from_static("attributeGroups"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "name" => { self.set_p_name(cast::<Str>(value)); true }, "default" => { self.set_p_default(value.to_option().map(|__m| cast::<crate::php_parser::node::Expr>(__m))); true }, "type" => { self.set_p_type_(value.to_option().map(|__m| cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(__m))); true }, "byRef" => { self.set_p_byRef(cast::<bool>(value)); true }, "flags" => { self.set_p_flags(cast::<i64>(value)); true }, "variadic" => { self.set_p_variadic(cast::<bool>(value)); true }, "attributeGroups" => { self.set_p_attributeGroups(cast::<List<crate::php_parser::node::AttributeGroup>>(value)); true }, _ => false } }
}
impl Param { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Builder\\Param could not be converted to string"))) } }
impl php_rt::PhpClone for Param { fn php_clone(&self) -> Self { let c = Param(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ParamObj { fn clone(&self) -> Self { ParamObj { name: self.name.clone(), default: self.default.clone(), type_: self.type_.clone(), byRef: self.byRef.clone(), flags: self.flags.clone(), variadic: self.variadic.clone(), attributeGroups: self.attributeGroups.clone() } } }
impl Param {
}
pub struct PropertyObj {
    pub name: Str,
    pub flags: i64,
    pub default: Option<crate::php_parser::node::Expr>,
    pub attributes: Map<Str, Mixed>,
    pub type_: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>,
    pub attributeGroups: List<crate::php_parser::node::AttributeGroup>,
    pub hooks: List<crate::php_parser::node::PropertyHook>,
}
#[derive(Clone)]
pub struct Property(pub Rc<RefCell<PropertyObj>>);
impl Property {
    pub fn p_name(&self) -> Ref<'_, Str> { Ref::map(self.0.borrow(), |o| &o.name) }
    pub fn p_name_get(&self) -> Str { self.0.borrow().name.clone() }
    pub fn p_name_opt(&self) -> Option<Str> { Some(self.0.borrow().name.clone()) }
    pub fn p_name_mut(&self) -> RefMut<'_, Str> { RefMut::map(self.0.borrow_mut(), |o| &mut o.name) }
    pub fn set_p_name(&self, v: Str) { self.0.borrow_mut().name = v; }
    pub fn p_flags(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.flags) }
    pub fn p_flags_get(&self) -> i64 { self.0.borrow().flags.clone() }
    pub fn p_flags_opt(&self) -> Option<i64> { Some(self.0.borrow().flags.clone()) }
    pub fn p_flags_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.flags) }
    pub fn set_p_flags(&self, v: i64) { self.0.borrow_mut().flags = v; }
    pub fn p_default(&self) -> Ref<'_, Option<crate::php_parser::node::Expr>> { Ref::map(self.0.borrow(), |o| &o.default) }
    pub fn p_default_get(&self) -> Option<crate::php_parser::node::Expr> { self.0.borrow().default.clone() }
    pub fn p_default_opt(&self) -> Option<Option<crate::php_parser::node::Expr>> { Some(self.0.borrow().default.clone()) }
    pub fn p_default_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Expr>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.default) }
    pub fn set_p_default(&self, v: Option<crate::php_parser::node::Expr>) { self.0.borrow_mut().default = v; }
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_type_(&self) -> Ref<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { Ref::map(self.0.borrow(), |o| &o.type_) }
    pub fn p_type__get(&self) -> Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> { self.0.borrow().type_.clone() }
    pub fn p_type__opt(&self) -> Option<Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { Some(self.0.borrow().type_.clone()) }
    pub fn p_type__mut(&self) -> RefMut<'_, Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.type_) }
    pub fn set_p_type_(&self, v: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>) { self.0.borrow_mut().type_ = v; }
    pub fn p_attributeGroups(&self) -> Ref<'_, List<crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attributeGroups) }
    pub fn p_attributeGroups_get(&self) -> List<crate::php_parser::node::AttributeGroup> { self.0.borrow().attributeGroups.clone() }
    pub fn p_attributeGroups_opt(&self) -> Option<List<crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attributeGroups.clone()) }
    pub fn p_attributeGroups_mut(&self) -> RefMut<'_, List<crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributeGroups) }
    pub fn set_p_attributeGroups(&self, v: List<crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attributeGroups = v; }
    pub fn p_hooks(&self) -> Ref<'_, List<crate::php_parser::node::PropertyHook>> { Ref::map(self.0.borrow(), |o| &o.hooks) }
    pub fn p_hooks_get(&self) -> List<crate::php_parser::node::PropertyHook> { self.0.borrow().hooks.clone() }
    pub fn p_hooks_opt(&self) -> Option<List<crate::php_parser::node::PropertyHook>> { Some(self.0.borrow().hooks.clone()) }
    pub fn p_hooks_mut(&self) -> RefMut<'_, List<crate::php_parser::node::PropertyHook>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.hooks) }
    pub fn set_p_hooks(&self, v: List<crate::php_parser::node::PropertyHook>) { self.0.borrow_mut().hooks = v; }
    pub fn new(mut name: Str) -> Result<Property, Throw> {
        let this = Property(Rc::new(RefCell::new(PropertyObj {
            name: Default::default(),
            flags: 0i64,
            default: { let _ = (); None::<crate::php_parser::node::Expr> },
            attributes: Map::<Str, Mixed>::new(),
            type_: { let _ = (); None::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name> },
            attributeGroups: List::<crate::php_parser::node::AttributeGroup>::new(),
            hooks: List::<crate::php_parser::node::PropertyHook>::new(),
        })));
        this.magic__construct(name)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: Str) -> Result<Mixed, Throw> {
    self.set_p_name(name.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn makePublic(&self) -> Result<crate::php_parser::builder::Property, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PUBLIC())?);
    return Ok(self.clone());
    }
    pub fn makeProtected(&self) -> Result<crate::php_parser::builder::Property, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PROTECTED())?);
    return Ok(self.clone());
    }
    pub fn makePrivate(&self) -> Result<crate::php_parser::builder::Property, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PRIVATE())?);
    return Ok(self.clone());
    }
    pub fn makeStatic(&self) -> Result<crate::php_parser::builder::Property, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::STATIC())?);
    return Ok(self.clone());
    }
    pub fn makeReadonly(&self) -> Result<crate::php_parser::builder::Property, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::READONLY())?);
    return Ok(self.clone());
    }
    pub fn makeAbstract(&self) -> Result<crate::php_parser::builder::Property, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::ABSTRACT())?);
    return Ok(self.clone());
    }
    pub fn makeFinal(&self) -> Result<crate::php_parser::builder::Property, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::FINAL())?);
    return Ok(self.clone());
    }
    pub fn makePrivateSet(&self) -> Result<crate::php_parser::builder::Property, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PRIVATE_SET())?);
    return Ok(self.clone());
    }
    pub fn makeProtectedSet(&self) -> Result<crate::php_parser::builder::Property, Throw> {
    self.set_p_flags(crate::php_parser::BuilderHelpers::addModifier(self.p_flags_get(), crate::php_parser::Modifiers::PROTECTED_SET())?);
    return Ok(self.clone());
    }
    pub fn setDefault(&self, mut value: Mixed) -> Result<crate::php_parser::builder::Property, Throw> {
    self.set_p_default(Some(crate::php_parser::BuilderHelpers::normalizeValue(value.clone().to_option().map(|__m| cast::<U_Bool_or_Float_or_Int_or_Map_ArrayKey_Mixed_or_PhpParser_Node_Expr_or_Str_or_UnitEnum>(__m)))?));
    return Ok(self.clone());
    }
    pub fn setDocComment(&self, mut docComment: U_PhpParser_Comment_Doc_or_Str) -> Result<crate::php_parser::builder::Property, Throw> {
    self.set_p_attributes({ let mut __m1: Map<Str, Mixed> = Map::new(); __m1.insert(Str::from_static("comments"), { let __c4 = (crate::php_parser::BuilderHelpers::normalizeDocComment(docComment.clone())?,); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c4.0)); Mixed::Arr(__m) }); __m1 });
    return Ok(self.clone());
    }
    pub fn setType(&self, mut type_: U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name_or_Str) -> Result<crate::php_parser::builder::Property, Throw> {
    self.set_p_type_(Some(crate::php_parser::BuilderHelpers::normalizeType(type_.clone())?));
    return Ok(self.clone());
    }
    pub fn addAttribute(&self, mut attribute: U_PhpParser_Node_Attribute_or_PhpParser_Node_AttributeGroup) -> Result<crate::php_parser::builder::Property, Throw> {
    (*self.p_attributeGroups_mut()).push(crate::php_parser::BuilderHelpers::normalizeAttribute(attribute.clone())?);
    return Ok(self.clone());
    }
    pub fn addHook(&self, mut hook: crate::php_parser::node::PropertyHook) -> Result<crate::php_parser::builder::Property, Throw> {
    (*self.p_hooks_mut()).push(hook.clone());
    return Ok(self.clone());
    }
    pub fn getNode(&self) -> Result<crate::php_parser::node::stmt::Property, Throw> {
    if (truthy(&(self.p_flags_get() & crate::php_parser::Modifiers::ABSTRACT())) && (!truthy(&self.p_hooks_get()))) {
        return Err(cast::<crate::g::Throwable>(crate::php_parser::Error::new(Str::from_static("Only hooked properties may be declared abstract"), Map::<Str, Mixed>::new())?));
    }
    return Ok(crate::php_parser::node::stmt::Property::new((if (!(self.p_flags_get() == 0i64)) { self.p_flags_get() } else { crate::php_parser::Modifiers::PUBLIC() }), { let mut __m1: Map<ArrayKey, crate::php_parser::node::PropertyItem> = Map::new(); __m1.push(crate::php_parser::node::PropertyItem::new(U_PhpParser_Node_VarLikeIdentifier_or_Str::Str(self.p_name_get()), self.p_default_get(), Map::<Str, Mixed>::new())?); __m1 }, self.p_attributes_get(), self.p_type__get(), cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(self.p_attributeGroups_get()), cast::<Map<ArrayKey, crate::php_parser::node::PropertyHook>>(self.p_hooks_get()))?);
    }
    pub fn new_same_class(&self, mut name: Str) -> Result<crate::php_parser::builder::Property, Throw> { Ok(Self::new(name)?) }
}
impl php_rt::PhpObject for Property {
    fn class_name(&self) -> &'static str { "PhpParser\\Builder\\Property" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\builder\\property", "phpparser\\builder"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_flags_get()) { out.push((Str::from_static("flags"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_default_get()) { out.push((Str::from_static("default"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_type__get()) { out.push((Str::from_static("type"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attributeGroups_get()) { out.push((Str::from_static("attributeGroups"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_hooks_get()) { out.push((Str::from_static("hooks"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "name" => { self.set_p_name(cast::<Str>(value)); true }, "flags" => { self.set_p_flags(cast::<i64>(value)); true }, "default" => { self.set_p_default(value.to_option().map(|__m| cast::<crate::php_parser::node::Expr>(__m))); true }, "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "type" => { self.set_p_type_(value.to_option().map(|__m| cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(__m))); true }, "attributeGroups" => { self.set_p_attributeGroups(cast::<List<crate::php_parser::node::AttributeGroup>>(value)); true }, "hooks" => { self.set_p_hooks(cast::<List<crate::php_parser::node::PropertyHook>>(value)); true }, _ => false } }
}
impl Property { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Builder\\Property could not be converted to string"))) } }
impl php_rt::PhpClone for Property { fn php_clone(&self) -> Self { let c = Property(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for PropertyObj { fn clone(&self) -> Self { PropertyObj { name: self.name.clone(), flags: self.flags.clone(), default: self.default.clone(), attributes: self.attributes.clone(), type_: self.type_.clone(), attributeGroups: self.attributeGroups.clone(), hooks: self.hooks.clone() } } }
impl Property {
}
pub struct TraitUseObj {
    pub traits: Map<ArrayKey, crate::php_parser::node::Name>,
    pub adaptations: Map<ArrayKey, crate::php_parser::node::stmt::TraitUseAdaptation>,
}
#[derive(Clone)]
pub struct TraitUse(pub Rc<RefCell<TraitUseObj>>);
impl TraitUse {
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
    pub fn new(mut traits: List<U_PhpParser_Node_Name_or_Str>) -> Result<TraitUse, Throw> {
        let this = TraitUse(Rc::new(RefCell::new(TraitUseObj {
            traits: Map::<ArrayKey, crate::php_parser::node::Name>::new(),
            adaptations: Map::<ArrayKey, crate::php_parser::node::stmt::TraitUseAdaptation>::new(),
        })));
        this.magic__construct(traits)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut traits: List<U_PhpParser_Node_Name_or_Str>) -> Result<Mixed, Throw> {
    let mut name: Str = Default::default();
    let mut trait_: Late<U_PhpParser_Node_Name_or_Str> = Late::uninit();
    'l1: for __kv1 in cast::<Map<ArrayKey, U_PhpParser_Node_Name_or_Str>>(traits.clone()).into_iter() {
        trait_.set(__kv1.1);
        let _: crate::php_parser::builder::TraitUse = self.and(trait_.get().clone())?;
    }
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn and(&self, mut trait_: U_PhpParser_Node_Name_or_Str) -> Result<crate::php_parser::builder::TraitUse, Throw> {
    (*self.p_traits_mut()).push(crate::php_parser::BuilderHelpers::normalizeName(trait_.clone())?);
    return Ok(self.clone());
    }
    pub fn with(&self, mut adaptation: U_PhpParser_Builder_TraitUseAdaptation_or_PhpParser_Node_Stmt_TraitUseAdaptation) -> Result<crate::php_parser::builder::TraitUse, Throw> {
    adaptation = cast::<U_PhpParser_Builder_TraitUseAdaptation_or_PhpParser_Node_Stmt_TraitUseAdaptation>(crate::php_parser::BuilderHelpers::normalizeNode(cast::<U_PhpParser_Builder_or_PhpParser_Node>(adaptation.clone()))?);
    if (!is_instance::<crate::php_parser::node::stmt::TraitUseAdaptation>(&cast::<crate::php_parser::Node>(adaptation.clone()))) {
        return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(Str::from_static("Adaptation must have type TraitUseAdaptation"), 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    (*self.p_adaptations_mut()).push(cast::<crate::php_parser::node::stmt::TraitUseAdaptation>(adaptation.clone()));
    return Ok(self.clone());
    }
    pub fn getNode(&self) -> Result<crate::php_parser::Node, Throw> {
    return Ok(cast::<crate::php_parser::Node>(crate::php_parser::node::stmt::TraitUse::new(self.p_traits_get(), self.p_adaptations_get(), Map::<Str, Mixed>::new())?));
    }
    pub fn new_same_class(&self, mut traits: List<U_PhpParser_Node_Name_or_Str>) -> Result<crate::php_parser::builder::TraitUse, Throw> { Ok(Self::new(traits)?) }
}
impl php_rt::PhpObject for TraitUse {
    fn class_name(&self) -> &'static str { "PhpParser\\Builder\\TraitUse" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\builder\\traituse", "phpparser\\builder"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_traits_get()) { out.push((Str::from_static("traits"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_adaptations_get()) { out.push((Str::from_static("adaptations"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "traits" => { self.set_p_traits(cast::<Map<ArrayKey, crate::php_parser::node::Name>>(value)); true }, "adaptations" => { self.set_p_adaptations(cast::<Map<ArrayKey, crate::php_parser::node::stmt::TraitUseAdaptation>>(value)); true }, _ => false } }
}
impl TraitUse { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Builder\\TraitUse could not be converted to string"))) } }
impl php_rt::PhpClone for TraitUse { fn php_clone(&self) -> Self { let c = TraitUse(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for TraitUseObj { fn clone(&self) -> Self { TraitUseObj { traits: self.traits.clone(), adaptations: self.adaptations.clone() } } }
impl TraitUse {
}
pub struct TraitUseAdaptationObj {
    pub type_: i64,
    pub trait_: Option<crate::php_parser::node::Name>,
    pub method: Late<crate::php_parser::node::Identifier>,
    pub modifier: Option<i64>,
    pub alias: Option<crate::php_parser::node::Identifier>,
    pub insteadof: Map<ArrayKey, crate::php_parser::node::Name>,
}
#[derive(Clone)]
pub struct TraitUseAdaptation(pub Rc<RefCell<TraitUseAdaptationObj>>);
impl TraitUseAdaptation {
    pub fn p_type_(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.type_) }
    pub fn p_type__get(&self) -> i64 { self.0.borrow().type_.clone() }
    pub fn p_type__opt(&self) -> Option<i64> { Some(self.0.borrow().type_.clone()) }
    pub fn p_type__mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.type_) }
    pub fn set_p_type_(&self, v: i64) { self.0.borrow_mut().type_ = v; }
    pub fn p_trait_(&self) -> Ref<'_, Option<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.trait_) }
    pub fn p_trait__get(&self) -> Option<crate::php_parser::node::Name> { self.0.borrow().trait_.clone() }
    pub fn p_trait__opt(&self) -> Option<Option<crate::php_parser::node::Name>> { Some(self.0.borrow().trait_.clone()) }
    pub fn p_trait__mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.trait_) }
    pub fn set_p_trait_(&self, v: Option<crate::php_parser::node::Name>) { self.0.borrow_mut().trait_ = v; }
    pub fn p_method(&self) -> Ref<'_, crate::php_parser::node::Identifier> { Ref::map(self.0.borrow(), |o| o.method.get()) }
    pub fn p_method_get(&self) -> crate::php_parser::node::Identifier { self.0.borrow().method.get().clone() }
    pub fn p_method_opt(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().method.as_option().cloned() }
    pub fn p_method_mut(&self) -> RefMut<'_, crate::php_parser::node::Identifier> { RefMut::map(self.0.borrow_mut(), |o| o.method.get_mut()) }
    pub fn set_p_method(&self, v: crate::php_parser::node::Identifier) { self.0.borrow_mut().method.set(v); }
    pub fn p_modifier(&self) -> Ref<'_, Option<i64>> { Ref::map(self.0.borrow(), |o| &o.modifier) }
    pub fn p_modifier_get(&self) -> Option<i64> { self.0.borrow().modifier.clone() }
    pub fn p_modifier_opt(&self) -> Option<Option<i64>> { Some(self.0.borrow().modifier.clone()) }
    pub fn p_modifier_mut(&self) -> RefMut<'_, Option<i64>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.modifier) }
    pub fn set_p_modifier(&self, v: Option<i64>) { self.0.borrow_mut().modifier = v; }
    pub fn p_alias(&self) -> Ref<'_, Option<crate::php_parser::node::Identifier>> { Ref::map(self.0.borrow(), |o| &o.alias) }
    pub fn p_alias_get(&self) -> Option<crate::php_parser::node::Identifier> { self.0.borrow().alias.clone() }
    pub fn p_alias_opt(&self) -> Option<Option<crate::php_parser::node::Identifier>> { Some(self.0.borrow().alias.clone()) }
    pub fn p_alias_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Identifier>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.alias) }
    pub fn set_p_alias(&self, v: Option<crate::php_parser::node::Identifier>) { self.0.borrow_mut().alias = v; }
    pub fn p_insteadof(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| &o.insteadof) }
    pub fn p_insteadof_get(&self) -> Map<ArrayKey, crate::php_parser::node::Name> { self.0.borrow().insteadof.clone() }
    pub fn p_insteadof_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Name>> { Some(self.0.borrow().insteadof.clone()) }
    pub fn p_insteadof_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.insteadof) }
    pub fn set_p_insteadof(&self, v: Map<ArrayKey, crate::php_parser::node::Name>) { self.0.borrow_mut().insteadof = v; }
    pub fn new(mut trait_: Option<U_PhpParser_Node_Name_or_Str>, mut method: U_PhpParser_Node_Identifier_or_Str) -> Result<TraitUseAdaptation, Throw> {
        let this = TraitUseAdaptation(Rc::new(RefCell::new(TraitUseAdaptationObj {
            type_: Default::default(),
            trait_: Default::default(),
            method: Late::uninit(),
            modifier: { let _ = (); None::<i64> },
            alias: { let _ = (); None::<crate::php_parser::node::Identifier> },
            insteadof: Map::<ArrayKey, crate::php_parser::node::Name>::new(),
        })));
        this.magic__construct(trait_, method)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut trait_: Option<U_PhpParser_Node_Name_or_Str>, mut method: U_PhpParser_Node_Identifier_or_Str) -> Result<Mixed, Throw> {
    self.set_p_type_(crate::php_parser::builder::TraitUseAdaptation::TYPE_UNDEFINED());
    self.set_p_trait_((if trait_.clone().is_none() { { let _ = (); None::<crate::php_parser::node::Name> } } else { Some(crate::php_parser::BuilderHelpers::normalizeName(trait_.clone().unwrap())?) }));
    self.set_p_method(crate::php_parser::BuilderHelpers::normalizeIdentifier(method.clone())?);
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn as_(&self, mut alias: U_PhpParser_Node_Identifier_or_Str) -> Result<crate::php_parser::builder::TraitUseAdaptation, Throw> {
    if (self.p_type__get() == crate::php_parser::builder::TraitUseAdaptation::TYPE_UNDEFINED()) {
        self.set_p_type_(crate::php_parser::builder::TraitUseAdaptation::TYPE_ALIAS());
    }
    if (!(self.p_type__get() == crate::php_parser::builder::TraitUseAdaptation::TYPE_ALIAS())) {
        return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(Str::from_static("Cannot set alias for not alias adaptation buider"), 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    self.set_p_alias(Some(crate::php_parser::BuilderHelpers::normalizeIdentifier(alias.clone())?));
    return Ok(self.clone());
    }
    pub fn makePublic(&self) -> Result<crate::php_parser::builder::TraitUseAdaptation, Throw> {
    self.setModifier(crate::php_parser::Modifiers::PUBLIC())?;
    return Ok(self.clone());
    }
    pub fn makeProtected(&self) -> Result<crate::php_parser::builder::TraitUseAdaptation, Throw> {
    self.setModifier(crate::php_parser::Modifiers::PROTECTED())?;
    return Ok(self.clone());
    }
    pub fn makePrivate(&self) -> Result<crate::php_parser::builder::TraitUseAdaptation, Throw> {
    self.setModifier(crate::php_parser::Modifiers::PRIVATE())?;
    return Ok(self.clone());
    }
    pub fn insteadof(&self, mut traits: List<U_PhpParser_Node_Name_or_Str>) -> Result<crate::php_parser::builder::TraitUseAdaptation, Throw> {
    let mut trait_: Late<U_PhpParser_Node_Name_or_Str> = Late::uninit();
    if (self.p_type__get() == crate::php_parser::builder::TraitUseAdaptation::TYPE_UNDEFINED()) {
        if self.p_trait__get().is_none() {
            return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(Str::from_static("Precedence adaptation must have trait"), 0i64, { let _ = (); None::<crate::g::Throwable> })?));
        }
        self.set_p_type_(crate::php_parser::builder::TraitUseAdaptation::TYPE_PRECEDENCE());
    }
    if (!(self.p_type__get() == crate::php_parser::builder::TraitUseAdaptation::TYPE_PRECEDENCE())) {
        return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(Str::from_static("Cannot add overwritten traits for not precedence adaptation buider"), 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    'l1: for __kv1 in cast::<Map<ArrayKey, U_PhpParser_Node_Name_or_Str>>(traits.clone()).into_iter() {
        trait_.set(__kv1.1);
        (*self.p_insteadof_mut()).push(crate::php_parser::BuilderHelpers::normalizeName(trait_.get().clone())?);
    }
    return Ok(self.clone());
    }
    pub fn setModifier(&self, mut modifier: i64) -> Result<(), Throw> {
    if (self.p_type__get() == crate::php_parser::builder::TraitUseAdaptation::TYPE_UNDEFINED()) {
        self.set_p_type_(crate::php_parser::builder::TraitUseAdaptation::TYPE_ALIAS());
    }
    if (!(self.p_type__get() == crate::php_parser::builder::TraitUseAdaptation::TYPE_ALIAS())) {
        return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(Str::from_static("Cannot set access modifier for not alias adaptation buider"), 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    if self.p_modifier_get().is_none() {
        self.set_p_modifier(Some(modifier));
    } else {
        return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(Str::from_static("Multiple access type modifiers are not allowed"), 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    #[allow(unreachable_code)] Ok(())
    }
    pub fn getNode(&self) -> Result<crate::php_parser::Node, Throw> {
    'sw1: {
        let __sw1 = self.p_type__get();
        let __idx2: usize = if loose_eq(&__sw1.clone(), &crate::php_parser::builder::TraitUseAdaptation::TYPE_ALIAS()) { 0 } else if loose_eq(&__sw1.clone(), &crate::php_parser::builder::TraitUseAdaptation::TYPE_PRECEDENCE()) { 1 } else { 2 };
        if __idx2 <= 0 {
            return Ok(cast::<crate::php_parser::Node>(crate::php_parser::node::stmt::trait_use_adaptation::Alias::new(self.p_trait__get(), U_PhpParser_Node_Identifier_or_Str::PhpParser_Node_Identifier(self.p_method_get()), self.p_modifier_get(), self.p_alias_get().map(|v| U_PhpParser_Node_Identifier_or_Str::PhpParser_Node_Identifier(v)), Map::<Str, Mixed>::new())?));
        }
        if __idx2 <= 1 {
            return Ok(cast::<crate::php_parser::Node>(crate::php_parser::node::stmt::trait_use_adaptation::Precedence::new(self.p_trait__get().unwrap(), U_PhpParser_Node_Identifier_or_Str::PhpParser_Node_Identifier(self.p_method_get()), self.p_insteadof_get(), Map::<Str, Mixed>::new())?));
        }
        if __idx2 <= 2 {
            return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(Str::from_static("Type of adaptation is not defined"), 0i64, { let _ = (); None::<crate::g::Throwable> })?));
        }
    }
    #[allow(unreachable_code)] unreachable!("missing return")
    }
    pub fn new_same_class(&self, mut trait_: Option<U_PhpParser_Node_Name_or_Str>, mut method: U_PhpParser_Node_Identifier_or_Str) -> Result<crate::php_parser::builder::TraitUseAdaptation, Throw> { Ok(Self::new(trait_, method)?) }
}
impl php_rt::PhpObject for TraitUseAdaptation {
    fn class_name(&self) -> &'static str { "PhpParser\\Builder\\TraitUseAdaptation" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\builder\\traituseadaptation", "phpparser\\builder"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_type__get()) { out.push((Str::from_static("type"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_trait__get()) { out.push((Str::from_static("trait"), cast::<Mixed>(v))); } if let Some(v) = self.p_method_opt() { out.push((Str::from_static("method"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_modifier_get()) { out.push((Str::from_static("modifier"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_alias_get()) { out.push((Str::from_static("alias"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_insteadof_get()) { out.push((Str::from_static("insteadof"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "type" => { self.set_p_type_(cast::<i64>(value)); true }, "trait" => { self.set_p_trait_(value.to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m))); true }, "method" => { self.set_p_method(cast::<crate::php_parser::node::Identifier>(value)); true }, "modifier" => { self.set_p_modifier(value.to_option().map(|__m| cast::<i64>(__m))); true }, "alias" => { self.set_p_alias(value.to_option().map(|__m| cast::<crate::php_parser::node::Identifier>(__m))); true }, "insteadof" => { self.set_p_insteadof(cast::<Map<ArrayKey, crate::php_parser::node::Name>>(value)); true }, _ => false } }
}
impl TraitUseAdaptation { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Builder\\TraitUseAdaptation could not be converted to string"))) } }
impl php_rt::PhpClone for TraitUseAdaptation { fn php_clone(&self) -> Self { let c = TraitUseAdaptation(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for TraitUseAdaptationObj { fn clone(&self) -> Self { TraitUseAdaptationObj { type_: self.type_.clone(), trait_: self.trait_.clone(), method: self.method.clone(), modifier: self.modifier.clone(), alias: self.alias.clone(), insteadof: self.insteadof.clone() } } }
impl TraitUseAdaptation {
    pub fn TYPE_UNDEFINED() -> i64 { 0i64 }
    pub fn TYPE_ALIAS() -> i64 { 1i64 }
    pub fn TYPE_PRECEDENCE() -> i64 { 2i64 }
}
pub struct Trait_Obj {
    pub attributes: Map<Str, Mixed>,
    pub name: Str,
    pub uses: List<crate::php_parser::node::stmt::TraitUse>,
    pub constants: List<crate::php_parser::node::stmt::ClassConst>,
    pub properties: List<crate::php_parser::node::stmt::Property>,
    pub methods: List<crate::php_parser::node::stmt::ClassMethod>,
    pub attributeGroups: List<crate::php_parser::node::AttributeGroup>,
}
#[derive(Clone)]
pub struct Trait_(pub Rc<RefCell<Trait_Obj>>);
impl Trait_ {
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
    pub fn p_uses(&self) -> Ref<'_, List<crate::php_parser::node::stmt::TraitUse>> { Ref::map(self.0.borrow(), |o| &o.uses) }
    pub fn p_uses_get(&self) -> List<crate::php_parser::node::stmt::TraitUse> { self.0.borrow().uses.clone() }
    pub fn p_uses_opt(&self) -> Option<List<crate::php_parser::node::stmt::TraitUse>> { Some(self.0.borrow().uses.clone()) }
    pub fn p_uses_mut(&self) -> RefMut<'_, List<crate::php_parser::node::stmt::TraitUse>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.uses) }
    pub fn set_p_uses(&self, v: List<crate::php_parser::node::stmt::TraitUse>) { self.0.borrow_mut().uses = v; }
    pub fn p_constants(&self) -> Ref<'_, List<crate::php_parser::node::stmt::ClassConst>> { Ref::map(self.0.borrow(), |o| &o.constants) }
    pub fn p_constants_get(&self) -> List<crate::php_parser::node::stmt::ClassConst> { self.0.borrow().constants.clone() }
    pub fn p_constants_opt(&self) -> Option<List<crate::php_parser::node::stmt::ClassConst>> { Some(self.0.borrow().constants.clone()) }
    pub fn p_constants_mut(&self) -> RefMut<'_, List<crate::php_parser::node::stmt::ClassConst>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.constants) }
    pub fn set_p_constants(&self, v: List<crate::php_parser::node::stmt::ClassConst>) { self.0.borrow_mut().constants = v; }
    pub fn p_properties(&self) -> Ref<'_, List<crate::php_parser::node::stmt::Property>> { Ref::map(self.0.borrow(), |o| &o.properties) }
    pub fn p_properties_get(&self) -> List<crate::php_parser::node::stmt::Property> { self.0.borrow().properties.clone() }
    pub fn p_properties_opt(&self) -> Option<List<crate::php_parser::node::stmt::Property>> { Some(self.0.borrow().properties.clone()) }
    pub fn p_properties_mut(&self) -> RefMut<'_, List<crate::php_parser::node::stmt::Property>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.properties) }
    pub fn set_p_properties(&self, v: List<crate::php_parser::node::stmt::Property>) { self.0.borrow_mut().properties = v; }
    pub fn p_methods(&self) -> Ref<'_, List<crate::php_parser::node::stmt::ClassMethod>> { Ref::map(self.0.borrow(), |o| &o.methods) }
    pub fn p_methods_get(&self) -> List<crate::php_parser::node::stmt::ClassMethod> { self.0.borrow().methods.clone() }
    pub fn p_methods_opt(&self) -> Option<List<crate::php_parser::node::stmt::ClassMethod>> { Some(self.0.borrow().methods.clone()) }
    pub fn p_methods_mut(&self) -> RefMut<'_, List<crate::php_parser::node::stmt::ClassMethod>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.methods) }
    pub fn set_p_methods(&self, v: List<crate::php_parser::node::stmt::ClassMethod>) { self.0.borrow_mut().methods = v; }
    pub fn p_attributeGroups(&self) -> Ref<'_, List<crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attributeGroups) }
    pub fn p_attributeGroups_get(&self) -> List<crate::php_parser::node::AttributeGroup> { self.0.borrow().attributeGroups.clone() }
    pub fn p_attributeGroups_opt(&self) -> Option<List<crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attributeGroups.clone()) }
    pub fn p_attributeGroups_mut(&self) -> RefMut<'_, List<crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributeGroups) }
    pub fn set_p_attributeGroups(&self, v: List<crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attributeGroups = v; }
    pub fn new(mut name: Str) -> Result<Trait_, Throw> {
        let this = Trait_(Rc::new(RefCell::new(Trait_Obj {
            attributes: Map::<Str, Mixed>::new(),
            name: Default::default(),
            uses: List::<crate::php_parser::node::stmt::TraitUse>::new(),
            constants: List::<crate::php_parser::node::stmt::ClassConst>::new(),
            properties: List::<crate::php_parser::node::stmt::Property>::new(),
            methods: List::<crate::php_parser::node::stmt::ClassMethod>::new(),
            attributeGroups: List::<crate::php_parser::node::AttributeGroup>::new(),
        })));
        this.magic__construct(name)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: Str) -> Result<Mixed, Throw> {
    self.set_p_name(name.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn addStmt(&self, mut stmt: U_PhpParser_Builder_or_PhpParser_Node_Stmt) -> Result<crate::php_parser::builder::Trait_, Throw> {
    stmt = cast::<U_PhpParser_Builder_or_PhpParser_Node_Stmt>(crate::php_parser::BuilderHelpers::normalizeNode(cast::<U_PhpParser_Builder_or_PhpParser_Node>(stmt.clone()))?);
    if is_instance::<crate::php_parser::node::stmt::Property>(&cast::<crate::php_parser::Node>(stmt.clone())) {
        (*self.p_properties_mut()).push(cast::<crate::php_parser::node::stmt::Property>(stmt.clone()));
    } else if is_instance::<crate::php_parser::node::stmt::ClassMethod>(&cast::<crate::php_parser::Node>(stmt.clone())) {
        (*self.p_methods_mut()).push(cast::<crate::php_parser::node::stmt::ClassMethod>(stmt.clone()));
    } else if is_instance::<crate::php_parser::node::stmt::TraitUse>(&cast::<crate::php_parser::Node>(stmt.clone())) {
        (*self.p_uses_mut()).push(cast::<crate::php_parser::node::stmt::TraitUse>(stmt.clone()));
    } else if is_instance::<crate::php_parser::node::stmt::ClassConst>(&cast::<crate::php_parser::Node>(stmt.clone())) {
        (*self.p_constants_mut()).push(cast::<crate::php_parser::node::stmt::ClassConst>(stmt.clone()));
    } else {
        return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(sprintf(&Str::from_static("Unexpected node of type \"%s\""), &[FmtArg::from(cast::<crate::php_parser::Node>(stmt.clone()).getType()?)])?, 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    return Ok(self.clone());
    }
    pub fn addAttribute(&self, mut attribute: U_PhpParser_Node_Attribute_or_PhpParser_Node_AttributeGroup) -> Result<crate::php_parser::builder::Trait_, Throw> {
    (*self.p_attributeGroups_mut()).push(crate::php_parser::BuilderHelpers::normalizeAttribute(attribute.clone())?);
    return Ok(self.clone());
    }
    pub fn getNode(&self) -> Result<crate::php_parser::node::stmt::Trait_, Throw> {
    return Ok(crate::php_parser::node::stmt::Trait_::new(U_PhpParser_Node_Identifier_or_Str::Str(self.p_name_get()), Shape_attrGroupsq_Map_ArrayKey_PhpParser_Node_AttributeGroup_stmtsq_Ma_b3c75089bc { stmts: Some(cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(array_merge_l(&[&self.p_uses_get().map_elems(|v| U_PhpParser_Node_Stmt_ClassConst_or_PhpParser_Node_Stmt_ClassMethod_or_c7a89e663f::PhpParser_Node_Stmt_TraitUse(v)), &self.p_constants_get().map_elems(|v| U_PhpParser_Node_Stmt_ClassConst_or_PhpParser_Node_Stmt_ClassMethod_or_c7a89e663f::PhpParser_Node_Stmt_ClassConst(v)), &self.p_properties_get().map_elems(|v| U_PhpParser_Node_Stmt_ClassConst_or_PhpParser_Node_Stmt_ClassMethod_or_c7a89e663f::PhpParser_Node_Stmt_Property(v)), &self.p_methods_get().map_elems(|v| U_PhpParser_Node_Stmt_ClassConst_or_PhpParser_Node_Stmt_ClassMethod_or_c7a89e663f::PhpParser_Node_Stmt_ClassMethod(v))]).map_elems(|v| cast::<crate::php_parser::node::Stmt>(v)))), attrGroups: Some(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(self.p_attributeGroups_get())) }, self.p_attributes_get())?);
    }
    pub fn addStmts(&self, mut stmts: Map<ArrayKey, U_PhpParser_Builder_or_PhpParser_Node_Stmt>) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).addStmts__impl(stmts) }
    pub fn setDocComment(&self, mut docComment: U_PhpParser_Comment_Doc_or_Str) -> Result<crate::php_parser::builder::Declaration, Throw> { cast::<crate::php_parser::builder::Declaration>(self.clone()).setDocComment__impl(docComment) }
    pub fn new_same_class(&self, mut name: Str) -> Result<crate::php_parser::builder::Trait_, Throw> { Ok(Self::new(name)?) }
}
impl php_rt::PhpObject for Trait_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Builder\\Trait_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\builder\\trait_", "phpparser\\builder\\declaration", "phpparser\\builder"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_uses_get()) { out.push((Str::from_static("uses"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_constants_get()) { out.push((Str::from_static("constants"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_properties_get()) { out.push((Str::from_static("properties"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_methods_get()) { out.push((Str::from_static("methods"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attributeGroups_get()) { out.push((Str::from_static("attributeGroups"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "name" => { self.set_p_name(cast::<Str>(value)); true }, "uses" => { self.set_p_uses(cast::<List<crate::php_parser::node::stmt::TraitUse>>(value)); true }, "constants" => { self.set_p_constants(cast::<List<crate::php_parser::node::stmt::ClassConst>>(value)); true }, "properties" => { self.set_p_properties(cast::<List<crate::php_parser::node::stmt::Property>>(value)); true }, "methods" => { self.set_p_methods(cast::<List<crate::php_parser::node::stmt::ClassMethod>>(value)); true }, "attributeGroups" => { self.set_p_attributeGroups(cast::<List<crate::php_parser::node::AttributeGroup>>(value)); true }, _ => false } }
}
impl Trait_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Builder\\Trait_ could not be converted to string"))) } }
impl php_rt::PhpClone for Trait_ { fn php_clone(&self) -> Self { let c = Trait_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Trait_Obj { fn clone(&self) -> Self { Trait_Obj { attributes: self.attributes.clone(), name: self.name.clone(), uses: self.uses.clone(), constants: self.constants.clone(), properties: self.properties.clone(), methods: self.methods.clone(), attributeGroups: self.attributeGroups.clone() } } }
impl Trait_ {
}
pub struct Use_Obj {
    pub name: Late<crate::php_parser::node::Name>,
    pub type_: Mixed,
    pub alias: Option<Str>,
}
#[derive(Clone)]
pub struct Use_(pub Rc<RefCell<Use_Obj>>);
impl Use_ {
    pub fn p_name(&self) -> Ref<'_, crate::php_parser::node::Name> { Ref::map(self.0.borrow(), |o| o.name.get()) }
    pub fn p_name_get(&self) -> crate::php_parser::node::Name { self.0.borrow().name.get().clone() }
    pub fn p_name_opt(&self) -> Option<crate::php_parser::node::Name> { self.0.borrow().name.as_option().cloned() }
    pub fn p_name_mut(&self) -> RefMut<'_, crate::php_parser::node::Name> { RefMut::map(self.0.borrow_mut(), |o| o.name.get_mut()) }
    pub fn set_p_name(&self, v: crate::php_parser::node::Name) { self.0.borrow_mut().name.set(v); }
    pub fn p_type_(&self) -> Ref<'_, Mixed> { Ref::map(self.0.borrow(), |o| &o.type_) }
    pub fn p_type__get(&self) -> Mixed { self.0.borrow().type_.clone() }
    pub fn p_type__opt(&self) -> Option<Mixed> { Some(self.0.borrow().type_.clone()) }
    pub fn p_type__mut(&self) -> RefMut<'_, Mixed> { RefMut::map(self.0.borrow_mut(), |o| &mut o.type_) }
    pub fn set_p_type_(&self, v: Mixed) { self.0.borrow_mut().type_ = v; }
    pub fn p_alias(&self) -> Ref<'_, Option<Str>> { Ref::map(self.0.borrow(), |o| &o.alias) }
    pub fn p_alias_get(&self) -> Option<Str> { self.0.borrow().alias.clone() }
    pub fn p_alias_opt(&self) -> Option<Option<Str>> { Some(self.0.borrow().alias.clone()) }
    pub fn p_alias_mut(&self) -> RefMut<'_, Option<Str>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.alias) }
    pub fn set_p_alias(&self, v: Option<Str>) { self.0.borrow_mut().alias = v; }
    pub fn new(mut name: U_PhpParser_Node_Name_or_Str, mut type_: Mixed) -> Result<Use_, Throw> {
        let this = Use_(Rc::new(RefCell::new(Use_Obj {
            name: Late::uninit(),
            type_: Default::default(),
            alias: { let _ = (); None::<Str> },
        })));
        this.magic__construct(name, type_)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut name: U_PhpParser_Node_Name_or_Str, mut type_: Mixed) -> Result<Mixed, Throw> {
    self.set_p_name(crate::php_parser::BuilderHelpers::normalizeName(name.clone())?);
    self.set_p_type_(cast::<Mixed>(cast::<i64>(type_.clone())));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn as_(&self, mut alias: Str) -> Result<crate::php_parser::builder::Use_, Throw> {
    self.set_p_alias(Some(alias.clone()));
    return Ok(self.clone());
    }
    pub fn getNode(&self) -> Result<crate::php_parser::node::stmt::Use_, Throw> {
    return Ok(crate::php_parser::node::stmt::Use_::new({ let mut __m1: Map<ArrayKey, crate::php_parser::node::UseItem> = Map::new(); __m1.push(crate::php_parser::node::UseItem::new(self.p_name_get(), self.p_alias_get().map(|v| U_PhpParser_Node_Identifier_or_Str::Str(v)), cast::<Mixed>(crate::php_parser::node::stmt::Use_::TYPE_UNKNOWN()), Map::<Str, Mixed>::new())?); __m1 }, cast::<Mixed>(cast::<i64>(self.p_type__get())), Map::<Str, Mixed>::new())?);
    }
    pub fn new_same_class(&self, mut name: U_PhpParser_Node_Name_or_Str, mut type_: Mixed) -> Result<crate::php_parser::builder::Use_, Throw> { Ok(Self::new(name, type_)?) }
}
impl php_rt::PhpObject for Use_ {
    fn class_name(&self) -> &'static str { "PhpParser\\Builder\\Use_" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\builder\\use_", "phpparser\\builder"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_name_opt() { out.push((Str::from_static("name"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_type__get()) { out.push((Str::from_static("type"), v)); } if let Some(v) = Some(self.p_alias_get()) { out.push((Str::from_static("alias"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "name" => { self.set_p_name(cast::<crate::php_parser::node::Name>(value)); true }, "type" => { self.set_p_type_(value); true }, "alias" => { self.set_p_alias(value.to_option().map(|__m| cast::<Str>(__m))); true }, _ => false } }
}
impl Use_ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Builder\\Use_ could not be converted to string"))) } }
impl php_rt::PhpClone for Use_ { fn php_clone(&self) -> Self { let c = Use_(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for Use_Obj { fn clone(&self) -> Self { Use_Obj { name: self.name.clone(), type_: self.type_.clone(), alias: self.alias.clone() } } }
impl Use_ {
}
impl php_rt::Truthy for ClassConst { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for ClassConst { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\ClassConst")) } }
impl php_rt::Identical for ClassConst { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for ClassConst { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for ClassConst { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for ClassConst { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<ClassConst> for Mixed { fn cast_to(self) -> ClassConst { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::ClassConst>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Builder\\ClassConst") } }
impl php_rt::TryDowncast for ClassConst { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::ClassConst>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for ClassConst { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<ClassConst> for AnyObject { fn cast_to(self) -> ClassConst { cast::<ClassConst>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<ClassConst> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\classconst") } }
impl php_rt::InstanceOf<ClassConst> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\classconst") } }
impl php_rt::InstanceOf<ClassConst> for ClassConst { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Class_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Class_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\Class_")) } }
impl php_rt::Identical for Class_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Class_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Class_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Class_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Class_> for Mixed { fn cast_to(self) -> Class_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Class_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Builder\\Class_") } }
impl php_rt::TryDowncast for Class_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Class_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Class_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Class_> for AnyObject { fn cast_to(self) -> Class_ { cast::<Class_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Class_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\class_") } }
impl php_rt::InstanceOf<Class_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\class_") } }
impl php_rt::InstanceOf<Class_> for Class_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Declaration { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Declaration { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\Declaration")) } }
impl php_rt::Identical for Declaration { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Declaration { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Declaration { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Declaration { fn cast_to(self) -> Mixed { match self { Declaration::PhpParser_Builder_Class_(v) => Mixed::Obj(Rc::new(v)), Declaration::PhpParser_Builder_Enum_(v) => Mixed::Obj(Rc::new(v)), Declaration::PhpParser_Builder_Function_(v) => Mixed::Obj(Rc::new(v)), Declaration::PhpParser_Builder_Interface_(v) => Mixed::Obj(Rc::new(v)), Declaration::PhpParser_Builder_Method(v) => Mixed::Obj(Rc::new(v)), Declaration::PhpParser_Builder_Namespace_(v) => Mixed::Obj(Rc::new(v)), Declaration::PhpParser_Builder_Trait_(v) => Mixed::Obj(Rc::new(v)), _ => unreachable!() } } }
impl php_rt::CastTo<Declaration> for Mixed { fn cast_to(self) -> Declaration { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Class_>() { return crate::php_parser::builder::Declaration::PhpParser_Builder_Class_(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Enum_>() { return crate::php_parser::builder::Declaration::PhpParser_Builder_Enum_(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Function_>() { return crate::php_parser::builder::Declaration::PhpParser_Builder_Function_(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Interface_>() { return crate::php_parser::builder::Declaration::PhpParser_Builder_Interface_(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Method>() { return crate::php_parser::builder::Declaration::PhpParser_Builder_Method(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Namespace_>() { return crate::php_parser::builder::Declaration::PhpParser_Builder_Namespace_(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Trait_>() { return crate::php_parser::builder::Declaration::PhpParser_Builder_Trait_(v.clone()); } } panic!("Mixed value is not a PhpParser\\Builder\\Declaration") } }
impl php_rt::TryDowncast for Declaration { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Class_>() { return Some(crate::php_parser::builder::Declaration::PhpParser_Builder_Class_(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Enum_>() { return Some(crate::php_parser::builder::Declaration::PhpParser_Builder_Enum_(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Function_>() { return Some(crate::php_parser::builder::Declaration::PhpParser_Builder_Function_(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Interface_>() { return Some(crate::php_parser::builder::Declaration::PhpParser_Builder_Interface_(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Method>() { return Some(crate::php_parser::builder::Declaration::PhpParser_Builder_Method(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Namespace_>() { return Some(crate::php_parser::builder::Declaration::PhpParser_Builder_Namespace_(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Trait_>() { return Some(crate::php_parser::builder::Declaration::PhpParser_Builder_Trait_(v.clone())); } None } }
impl php_rt::CastTo<AnyObject> for Declaration { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Declaration> for AnyObject { fn cast_to(self) -> Declaration { cast::<Declaration>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Declaration> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\declaration") } }
impl php_rt::InstanceOf<Declaration> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\declaration") } }
impl php_rt::InstanceOf<Declaration> for Declaration { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for EnumCase { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for EnumCase { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\EnumCase")) } }
impl php_rt::Identical for EnumCase { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for EnumCase { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for EnumCase { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for EnumCase { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<EnumCase> for Mixed { fn cast_to(self) -> EnumCase { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::EnumCase>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Builder\\EnumCase") } }
impl php_rt::TryDowncast for EnumCase { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::EnumCase>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for EnumCase { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<EnumCase> for AnyObject { fn cast_to(self) -> EnumCase { cast::<EnumCase>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<EnumCase> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\enumcase") } }
impl php_rt::InstanceOf<EnumCase> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\enumcase") } }
impl php_rt::InstanceOf<EnumCase> for EnumCase { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Enum_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Enum_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\Enum_")) } }
impl php_rt::Identical for Enum_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Enum_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Enum_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Enum_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Enum_> for Mixed { fn cast_to(self) -> Enum_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Enum_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Builder\\Enum_") } }
impl php_rt::TryDowncast for Enum_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Enum_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Enum_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Enum_> for AnyObject { fn cast_to(self) -> Enum_ { cast::<Enum_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Enum_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\enum_") } }
impl php_rt::InstanceOf<Enum_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\enum_") } }
impl php_rt::InstanceOf<Enum_> for Enum_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for FunctionLike { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for FunctionLike { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\FunctionLike")) } }
impl php_rt::Identical for FunctionLike { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for FunctionLike { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for FunctionLike { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for FunctionLike { fn cast_to(self) -> Mixed { match self { FunctionLike::PhpParser_Builder_Function_(v) => Mixed::Obj(Rc::new(v)), FunctionLike::PhpParser_Builder_Method(v) => Mixed::Obj(Rc::new(v)), _ => unreachable!() } } }
impl php_rt::CastTo<FunctionLike> for Mixed { fn cast_to(self) -> FunctionLike { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Function_>() { return crate::php_parser::builder::FunctionLike::PhpParser_Builder_Function_(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Method>() { return crate::php_parser::builder::FunctionLike::PhpParser_Builder_Method(v.clone()); } } panic!("Mixed value is not a PhpParser\\Builder\\FunctionLike") } }
impl php_rt::TryDowncast for FunctionLike { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Function_>() { return Some(crate::php_parser::builder::FunctionLike::PhpParser_Builder_Function_(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Method>() { return Some(crate::php_parser::builder::FunctionLike::PhpParser_Builder_Method(v.clone())); } None } }
impl php_rt::CastTo<AnyObject> for FunctionLike { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<FunctionLike> for AnyObject { fn cast_to(self) -> FunctionLike { cast::<FunctionLike>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<FunctionLike> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\functionlike") } }
impl php_rt::InstanceOf<FunctionLike> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\functionlike") } }
impl php_rt::InstanceOf<FunctionLike> for FunctionLike { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Function_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Function_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\Function_")) } }
impl php_rt::Identical for Function_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Function_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Function_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Function_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Function_> for Mixed { fn cast_to(self) -> Function_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Function_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Builder\\Function_") } }
impl php_rt::TryDowncast for Function_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Function_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Function_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Function_> for AnyObject { fn cast_to(self) -> Function_ { cast::<Function_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Function_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\function_") } }
impl php_rt::InstanceOf<Function_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\function_") } }
impl php_rt::InstanceOf<Function_> for Function_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Interface_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Interface_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\Interface_")) } }
impl php_rt::Identical for Interface_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Interface_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Interface_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Interface_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Interface_> for Mixed { fn cast_to(self) -> Interface_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Interface_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Builder\\Interface_") } }
impl php_rt::TryDowncast for Interface_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Interface_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Interface_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Interface_> for AnyObject { fn cast_to(self) -> Interface_ { cast::<Interface_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Interface_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\interface_") } }
impl php_rt::InstanceOf<Interface_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\interface_") } }
impl php_rt::InstanceOf<Interface_> for Interface_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Method { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Method { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\Method")) } }
impl php_rt::Identical for Method { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Method { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Method { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Method { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Method> for Mixed { fn cast_to(self) -> Method { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Method>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Builder\\Method") } }
impl php_rt::TryDowncast for Method { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Method>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Method { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Method> for AnyObject { fn cast_to(self) -> Method { cast::<Method>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Method> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\method") } }
impl php_rt::InstanceOf<Method> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\method") } }
impl php_rt::InstanceOf<Method> for Method { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Namespace_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Namespace_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\Namespace_")) } }
impl php_rt::Identical for Namespace_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Namespace_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Namespace_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Namespace_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Namespace_> for Mixed { fn cast_to(self) -> Namespace_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Namespace_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Builder\\Namespace_") } }
impl php_rt::TryDowncast for Namespace_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Namespace_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Namespace_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Namespace_> for AnyObject { fn cast_to(self) -> Namespace_ { cast::<Namespace_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Namespace_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\namespace_") } }
impl php_rt::InstanceOf<Namespace_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\namespace_") } }
impl php_rt::InstanceOf<Namespace_> for Namespace_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Param { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Param { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\Param")) } }
impl php_rt::Identical for Param { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Param { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Param { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Param { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Param> for Mixed { fn cast_to(self) -> Param { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Param>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Builder\\Param") } }
impl php_rt::TryDowncast for Param { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Param>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Param { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Param> for AnyObject { fn cast_to(self) -> Param { cast::<Param>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Param> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\param") } }
impl php_rt::InstanceOf<Param> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\param") } }
impl php_rt::InstanceOf<Param> for Param { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Property { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Property { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\Property")) } }
impl php_rt::Identical for Property { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Property { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Property { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Property { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Property> for Mixed { fn cast_to(self) -> Property { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Property>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Builder\\Property") } }
impl php_rt::TryDowncast for Property { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Property>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Property { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Property> for AnyObject { fn cast_to(self) -> Property { cast::<Property>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Property> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\property") } }
impl php_rt::InstanceOf<Property> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\property") } }
impl php_rt::InstanceOf<Property> for Property { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for TraitUse { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for TraitUse { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\TraitUse")) } }
impl php_rt::Identical for TraitUse { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for TraitUse { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for TraitUse { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for TraitUse { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<TraitUse> for Mixed { fn cast_to(self) -> TraitUse { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::TraitUse>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Builder\\TraitUse") } }
impl php_rt::TryDowncast for TraitUse { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::TraitUse>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for TraitUse { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<TraitUse> for AnyObject { fn cast_to(self) -> TraitUse { cast::<TraitUse>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<TraitUse> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\traituse") } }
impl php_rt::InstanceOf<TraitUse> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\traituse") } }
impl php_rt::InstanceOf<TraitUse> for TraitUse { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for TraitUseAdaptation { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for TraitUseAdaptation { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\TraitUseAdaptation")) } }
impl php_rt::Identical for TraitUseAdaptation { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for TraitUseAdaptation { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for TraitUseAdaptation { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for TraitUseAdaptation { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<TraitUseAdaptation> for Mixed { fn cast_to(self) -> TraitUseAdaptation { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::TraitUseAdaptation>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Builder\\TraitUseAdaptation") } }
impl php_rt::TryDowncast for TraitUseAdaptation { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::TraitUseAdaptation>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for TraitUseAdaptation { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<TraitUseAdaptation> for AnyObject { fn cast_to(self) -> TraitUseAdaptation { cast::<TraitUseAdaptation>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<TraitUseAdaptation> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\traituseadaptation") } }
impl php_rt::InstanceOf<TraitUseAdaptation> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\traituseadaptation") } }
impl php_rt::InstanceOf<TraitUseAdaptation> for TraitUseAdaptation { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Trait_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Trait_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\Trait_")) } }
impl php_rt::Identical for Trait_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Trait_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Trait_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Trait_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Trait_> for Mixed { fn cast_to(self) -> Trait_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Trait_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Builder\\Trait_") } }
impl php_rt::TryDowncast for Trait_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Trait_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Trait_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Trait_> for AnyObject { fn cast_to(self) -> Trait_ { cast::<Trait_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Trait_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\trait_") } }
impl php_rt::InstanceOf<Trait_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\trait_") } }
impl php_rt::InstanceOf<Trait_> for Trait_ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Use_ { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Use_ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Builder\\Use_")) } }
impl php_rt::Identical for Use_ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Use_ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Use_ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Use_ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Use_> for Mixed { fn cast_to(self) -> Use_ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Use_>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Builder\\Use_") } }
impl php_rt::TryDowncast for Use_ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::builder::Use_>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Use_ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Use_> for AnyObject { fn cast_to(self) -> Use_ { cast::<Use_>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Use_> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\builder\\use_") } }
impl php_rt::InstanceOf<Use_> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\builder\\use_") } }
impl php_rt::InstanceOf<Use_> for Use_ { fn is_instance(&self) -> bool { true } }
