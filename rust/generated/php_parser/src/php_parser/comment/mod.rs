use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub struct DocObj {
    pub text: Str,
    pub startLine: i64,
    pub startFilePos: i64,
    pub startTokenPos: i64,
    pub endLine: i64,
    pub endFilePos: i64,
    pub endTokenPos: i64,
}
#[derive(Clone)]
pub struct Doc(pub Rc<RefCell<DocObj>>);
impl Doc {
    pub fn p_text(&self) -> Ref<'_, Str> { Ref::map(self.0.borrow(), |o| &o.text) }
    pub fn p_text_get(&self) -> Str { self.0.borrow().text.clone() }
    pub fn p_text_opt(&self) -> Option<Str> { Some(self.0.borrow().text.clone()) }
    pub fn p_text_mut(&self) -> RefMut<'_, Str> { RefMut::map(self.0.borrow_mut(), |o| &mut o.text) }
    pub fn set_p_text(&self, v: Str) { self.0.borrow_mut().text = v; }
    pub fn p_startLine(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.startLine) }
    pub fn p_startLine_get(&self) -> i64 { self.0.borrow().startLine.clone() }
    pub fn p_startLine_opt(&self) -> Option<i64> { Some(self.0.borrow().startLine.clone()) }
    pub fn p_startLine_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.startLine) }
    pub fn set_p_startLine(&self, v: i64) { self.0.borrow_mut().startLine = v; }
    pub fn p_startFilePos(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.startFilePos) }
    pub fn p_startFilePos_get(&self) -> i64 { self.0.borrow().startFilePos.clone() }
    pub fn p_startFilePos_opt(&self) -> Option<i64> { Some(self.0.borrow().startFilePos.clone()) }
    pub fn p_startFilePos_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.startFilePos) }
    pub fn set_p_startFilePos(&self, v: i64) { self.0.borrow_mut().startFilePos = v; }
    pub fn p_startTokenPos(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.startTokenPos) }
    pub fn p_startTokenPos_get(&self) -> i64 { self.0.borrow().startTokenPos.clone() }
    pub fn p_startTokenPos_opt(&self) -> Option<i64> { Some(self.0.borrow().startTokenPos.clone()) }
    pub fn p_startTokenPos_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.startTokenPos) }
    pub fn set_p_startTokenPos(&self, v: i64) { self.0.borrow_mut().startTokenPos = v; }
    pub fn p_endLine(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.endLine) }
    pub fn p_endLine_get(&self) -> i64 { self.0.borrow().endLine.clone() }
    pub fn p_endLine_opt(&self) -> Option<i64> { Some(self.0.borrow().endLine.clone()) }
    pub fn p_endLine_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.endLine) }
    pub fn set_p_endLine(&self, v: i64) { self.0.borrow_mut().endLine = v; }
    pub fn p_endFilePos(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.endFilePos) }
    pub fn p_endFilePos_get(&self) -> i64 { self.0.borrow().endFilePos.clone() }
    pub fn p_endFilePos_opt(&self) -> Option<i64> { Some(self.0.borrow().endFilePos.clone()) }
    pub fn p_endFilePos_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.endFilePos) }
    pub fn set_p_endFilePos(&self, v: i64) { self.0.borrow_mut().endFilePos = v; }
    pub fn p_endTokenPos(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.endTokenPos) }
    pub fn p_endTokenPos_get(&self) -> i64 { self.0.borrow().endTokenPos.clone() }
    pub fn p_endTokenPos_opt(&self) -> Option<i64> { Some(self.0.borrow().endTokenPos.clone()) }
    pub fn p_endTokenPos_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.endTokenPos) }
    pub fn set_p_endTokenPos(&self, v: i64) { self.0.borrow_mut().endTokenPos = v; }
    pub fn new(mut text: Str, mut startLine: i64, mut startFilePos: i64, mut startTokenPos: i64, mut endLine: i64, mut endFilePos: i64, mut endTokenPos: i64) -> Result<Doc, Throw> {
        let this = Doc(Rc::new(RefCell::new(DocObj {
            text: Default::default(),
            startLine: Default::default(),
            startFilePos: Default::default(),
            startTokenPos: Default::default(),
            endLine: Default::default(),
            endFilePos: Default::default(),
            endTokenPos: Default::default(),
        })));
        this.magic__construct(text, startLine, startFilePos, startTokenPos, endLine, endFilePos, endTokenPos)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut text: Str, mut startLine: i64, mut startFilePos: i64, mut startTokenPos: i64, mut endLine: i64, mut endFilePos: i64, mut endTokenPos: i64) -> Result<Mixed, Throw> { cast::<crate::php_parser::Comment>(self.clone()).magic__construct__impl(text, startLine, startFilePos, startTokenPos, endLine, endFilePos, endTokenPos) }
    pub fn getText(&self) -> Result<Str, Throw> { cast::<crate::php_parser::Comment>(self.clone()).getText__impl() }
    pub fn getStartLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::Comment>(self.clone()).getStartLine__impl() }
    pub fn getStartFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::Comment>(self.clone()).getStartFilePos__impl() }
    pub fn getStartTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::Comment>(self.clone()).getStartTokenPos__impl() }
    pub fn getEndLine(&self) -> Result<i64, Throw> { cast::<crate::php_parser::Comment>(self.clone()).getEndLine__impl() }
    pub fn getEndFilePos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::Comment>(self.clone()).getEndFilePos__impl() }
    pub fn getEndTokenPos(&self) -> Result<i64, Throw> { cast::<crate::php_parser::Comment>(self.clone()).getEndTokenPos__impl() }
    pub fn magic__toString(&self) -> Result<Str, Throw> { cast::<crate::php_parser::Comment>(self.clone()).magic__toString__impl() }
    pub fn getReformattedText(&self) -> Result<Str, Throw> { cast::<crate::php_parser::Comment>(self.clone()).getReformattedText__impl() }
    pub fn jsonSerialize(&self) -> Result<Shape_nodeType_Str_text_Mixed_line_Mixed_filePos_Mixed, Throw> { cast::<crate::php_parser::Comment>(self.clone()).jsonSerialize__impl() }
    pub fn new_same_class(&self, mut text: Str, mut startLine: i64, mut startFilePos: i64, mut startTokenPos: i64, mut endLine: i64, mut endFilePos: i64, mut endTokenPos: i64) -> Result<crate::php_parser::comment::Doc, Throw> { Ok(Self::new(text, startLine, startFilePos, startTokenPos, endLine, endFilePos, endTokenPos)?) }
}
impl php_rt::PhpObject for Doc {
    fn class_name(&self) -> &'static str { "PhpParser\\Comment\\Doc" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\comment\\doc", "phpparser\\comment", "jsonserializable", "stringable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_text_get()) { out.push((Str::from_static("text"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_startLine_get()) { out.push((Str::from_static("startLine"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_startFilePos_get()) { out.push((Str::from_static("startFilePos"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_startTokenPos_get()) { out.push((Str::from_static("startTokenPos"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_endLine_get()) { out.push((Str::from_static("endLine"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_endFilePos_get()) { out.push((Str::from_static("endFilePos"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_endTokenPos_get()) { out.push((Str::from_static("endTokenPos"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "text" => { self.set_p_text(cast::<Str>(value)); true }, "startLine" => { self.set_p_startLine(cast::<i64>(value)); true }, "startFilePos" => { self.set_p_startFilePos(cast::<i64>(value)); true }, "startTokenPos" => { self.set_p_startTokenPos(cast::<i64>(value)); true }, "endLine" => { self.set_p_endLine(cast::<i64>(value)); true }, "endFilePos" => { self.set_p_endFilePos(cast::<i64>(value)); true }, "endTokenPos" => { self.set_p_endTokenPos(cast::<i64>(value)); true }, _ => false } }
    fn php_to_string(&self) -> Option<Str> { self.magic__toString().ok() }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(2) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(3) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(4) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(5) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(6) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "gettext" => { let __r = self.getText().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "__tostring" => { let __r = self.magic__toString().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getreformattedtext" => { let __r = self.getReformattedText().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Comment\\Doc", name)))) } }
}
impl Doc { pub fn to_php_string(&self) -> Result<Str, Throw> { self.magic__toString() } }
impl php_rt::PhpClone for Doc { fn php_clone(&self) -> Self { let c = Doc(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for DocObj { fn clone(&self) -> Self { DocObj { text: self.text.clone(), startLine: self.startLine.clone(), startFilePos: self.startFilePos.clone(), startTokenPos: self.startTokenPos.clone(), endLine: self.endLine.clone(), endFilePos: self.endFilePos.clone(), endTokenPos: self.endTokenPos.clone() } } }
impl Doc {
}
impl php_rt::Truthy for Doc { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Doc { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Comment\\Doc")) } }
impl php_rt::Identical for Doc { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Doc { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Doc { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Doc { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Doc> for Mixed { fn cast_to(self) -> Doc { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::comment::Doc>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Comment\\Doc") } }
impl php_rt::TryDowncast for Doc { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::comment::Doc>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Doc { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Doc> for AnyObject { fn cast_to(self) -> Doc { cast::<Doc>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Doc> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\comment\\doc") } }
impl php_rt::InstanceOf<Doc> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\comment\\doc") } }
impl php_rt::InstanceOf<Doc> for Doc { fn is_instance(&self) -> bool { true } }
