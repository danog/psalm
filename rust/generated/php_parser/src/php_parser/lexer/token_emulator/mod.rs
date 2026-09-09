use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub struct AsymmetricVisibilityTokenEmulatorObj {
}
#[derive(Clone)]
pub struct AsymmetricVisibilityTokenEmulator(pub Rc<RefCell<AsymmetricVisibilityTokenEmulatorObj>>);
impl AsymmetricVisibilityTokenEmulator {
    pub fn new() -> Result<AsymmetricVisibilityTokenEmulator, Throw> {
        let this = AsymmetricVisibilityTokenEmulator(Rc::new(RefCell::new(AsymmetricVisibilityTokenEmulatorObj {
        })));
        Ok(this)
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> {
    return Ok(crate::php_parser::PhpVersion::fromComponents(8i64, 4i64)?);
    }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> {
    code = strtolower(&code.clone());
    return Ok((((!strpos(&code.clone(), &Str::from_static("public(set)"), 0).is_none()) || (!strpos(&code.clone(), &Str::from_static("protected(set)"), 0).is_none())) || (!strpos(&code.clone(), &Str::from_static("private(set)"), 0).is_none())));
    }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    let mut map_v: Shape_326_Int_325_Int_324_Int = Default::default();
    let mut i: i64 = Default::default();
    let mut c: i64 = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    map_v = Shape_326_Int_325_Int_324_Int { k326: 329i64, k325: 328i64, k324: 327i64 };
    {
        i = 0i64;
        c = tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).count();
        'l1: loop {
            if !((i < c)) { break; }
            'c2: {
                token.set(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&i)).clone());
                if ((((((({ let __k = to_key(&token.get().clone().p_id_get()); Some(map_v.clone()).and_then(|__b| { let __c18 = __b; let mut __m: Map<ArrayKey, i64> = Default::default(); __m.insert(ArrayKey::from(Str::from_static("326")), __c18.k326); __m.insert(ArrayKey::from(Str::from_static("325")), __c18.k325); __m.insert(ArrayKey::from(Str::from_static("324")), __c18.k324); __m }.get(&__k).cloned()) }.is_some()) && ((i).wrapping_add(3i64) < c)) && identical(&tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(i).wrapping_add(1i64))).clone().p_text_get(), &Str::from_static("("))) && (tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(i).wrapping_add(2i64))).clone().p_id_get() == 262i64)) && identical(&strtolower(&tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(i).wrapping_add(2i64))).clone().p_text_get()), &Str::from_static("set"))) && identical(&tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(i).wrapping_add(3i64))).clone().p_text_get(), &Str::from_static(")"))) && self.isKeywordContext(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)), i)?) {
                    let _: List<Mixed> = { let __off = i; let __len = Some(4i64); let __repl = { let __c20 = (crate::php_parser::Token::new({ let __c19 = map_v.clone(); let mut __m: Map<ArrayKey, i64> = Default::default(); __m.insert(ArrayKey::from(Str::from_static("326")), __c19.k326); __m.insert(ArrayKey::from(Str::from_static("325")), __c19.k325); __m.insert(ArrayKey::from(Str::from_static("324")), __c19.k324); __m }.idx(&to_key(&token.get().clone().p_id_get())).clone(), concat(concat(concat(token.get().clone().p_text_get(), Str::from_static("(")), tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(i).wrapping_add(2i64))).clone().p_text_get()), Str::from_static(")")), token.get().clone().p_line_get(), token.get().clone().p_pos_get())?,); List::from_vec(vec![cast::<Mixed>(__c20.0)]) }.into_vec(); let __r = array_splice_m(&mut tokens_v, __off, __len, __repl); __r };
                    { let __t1 = 3i64; c = (c).wrapping_sub(__t1); }
                }
            }
            let _ = { let __t2 = i; i = __t2.wrapping_add(1); __t2.wrapping_add(1) };
        }
    }
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    let mut reverseMap: Shape_329_Int_328_Int_327_Int = Default::default();
    let mut i: i64 = Default::default();
    let mut c: i64 = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    let mut matches: Map<ArrayKey, Str> = Default::default();
    let mut modifier: Str = Default::default();
    let mut set_v: Str = Default::default();
    let mut modifierLen: i64 = Default::default();
    reverseMap = Shape_329_Int_328_Int_327_Int { k329: 326i64, k328: 325i64, k327: 324i64 };
    {
        i = 0i64;
        c = tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).count();
        'l1: loop {
            if !((i < c)) { break; }
            'c2: {
                token.set(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&i)).clone());
                if (({ let __k = to_key(&token.get().clone().p_id_get()); Some(reverseMap.clone()).and_then(|__b| { let __c21 = __b; let mut __m: Map<ArrayKey, i64> = Default::default(); __m.insert(ArrayKey::from(Str::from_static("329")), __c21.k329); __m.insert(ArrayKey::from(Str::from_static("328")), __c21.k328); __m.insert(ArrayKey::from(Str::from_static("327")), __c21.k327); __m }.get(&__k).cloned()) }.is_some()) && truthy(&{ let (__r, __m) = preg_match_groups(&Str::from_static("/(public|protected|private)\\((set)\\)/i"), &token.get().clone().p_text_get(), 0)?; matches = __m; __r })) {
                    let __d1 = matches.clone(); modifier = __d1.idx(&to_key(&1i64)).clone(); set_v = __d1.idx(&to_key(&2i64)).clone(); 
                    modifierLen = strlen(&modifier.clone());
                    let _: List<Mixed> = { let __off = i; let __len = Some(1i64); let __repl = { let __c23 = (crate::php_parser::Token::new({ let __c22 = reverseMap.clone(); let mut __m: Map<ArrayKey, i64> = Default::default(); __m.insert(ArrayKey::from(Str::from_static("329")), __c22.k329); __m.insert(ArrayKey::from(Str::from_static("328")), __c22.k328); __m.insert(ArrayKey::from(Str::from_static("327")), __c22.k327); __m }.idx(&to_key(&token.get().clone().p_id_get())).clone(), modifier.clone(), token.get().clone().p_line_get(), token.get().clone().p_pos_get())?, crate::php_parser::Token::new(ord(&Str::from_static("(")), Str::from_static("("), token.get().clone().p_line_get(), (token.get().clone().p_pos_get()).wrapping_add(modifierLen))?, crate::php_parser::Token::new(262i64, set_v.clone(), token.get().clone().p_line_get(), ((token.get().clone().p_pos_get()).wrapping_add(modifierLen)).wrapping_add(1i64))?, crate::php_parser::Token::new(ord(&Str::from_static(")")), Str::from_static(")"), token.get().clone().p_line_get(), ((token.get().clone().p_pos_get()).wrapping_add(modifierLen)).wrapping_add(4i64))?); List::from_vec(vec![cast::<Mixed>(__c23.0), cast::<Mixed>(__c23.1), cast::<Mixed>(__c23.2), cast::<Mixed>(__c23.3)]) }.into_vec(); let __r = array_splice_m(&mut tokens_v, __off, __len, __repl); __r };
                    { let __t2 = 3i64; i = (i).wrapping_add(__t2); }
                    { let __t3 = 3i64; c = (c).wrapping_add(__t3); }
                }
            }
            let _ = { let __t4 = i; i = __t4.wrapping_add(1); __t4.wrapping_add(1) };
        }
    }
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn isKeywordContext(&self, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>, mut pos: i64) -> Result<bool, Throw> {
    let mut prevToken: Option<crate::php_parser::Token> = Default::default();
    prevToken = self.getPreviousNonSpaceToken(tokens_v.clone(), pos)?;
    if prevToken.clone().is_none() {
        return Ok(false);
    }
    return Ok(((!(prevToken.clone().unwrap().p_id_get() == 389i64)) && (!(prevToken.clone().unwrap().p_id_get() == 390i64))));
    }
    pub fn getPreviousNonSpaceToken(&self, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>, mut start: i64) -> Result<Option<crate::php_parser::Token>, Throw> {
    let mut i: i64 = Default::default();
    {
        i = (start).wrapping_sub(1i64);
        'l1: loop {
            if !((i >= 0i64)) { break; }
            'c2: {
                if (tokens_v.clone().idx(&to_key(&i)).clone().p_id_get() == 397i64) {
                    { break 'c2 };
                }
                return Ok(Some(tokens_v.clone().idx(&to_key(&i)).clone()));
            }
            let _ = { let __t1 = i; i = __t1.wrapping_sub(1); __t1.wrapping_sub(1) };
        }
    }
    return Ok({ let _ = (); None::<crate::php_parser::Token> });
    }
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> { cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(self.clone()).preprocessCode__impl(code, patches) }
    pub fn new_same_class(&self) -> Result<crate::php_parser::lexer::token_emulator::AsymmetricVisibilityTokenEmulator, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for AsymmetricVisibilityTokenEmulator {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\TokenEmulator\\AsymmetricVisibilityTokenEmulator" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\tokenemulator\\asymmetricvisibilitytokenemulator", "phpparser\\lexer\\tokenemulator\\tokenemulator"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getphpversion" => { let __r = self.getPhpVersion().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "isemulationneeded" => { let __r = self.isEmulationNeeded((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "emulate" => { let __r = self.emulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "reverseemulate" => { let __r = self.reverseEmulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "iskeywordcontext" => { let __r = self.isKeywordContext((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, crate::php_parser::Token>>(__a.clone()), None => <Map<ArrayKey, crate::php_parser::Token>>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getpreviousnonspacetoken" => { let __r = self.getPreviousNonSpaceToken((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, crate::php_parser::Token>>(__a.clone()), None => <Map<ArrayKey, crate::php_parser::Token>>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Lexer\\TokenEmulator\\AsymmetricVisibilityTokenEmulator", name)))) } }
}
impl AsymmetricVisibilityTokenEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\TokenEmulator\\AsymmetricVisibilityTokenEmulator could not be converted to string"))) } }
impl php_rt::PhpClone for AsymmetricVisibilityTokenEmulator { fn php_clone(&self) -> Self { let c = AsymmetricVisibilityTokenEmulator(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for AsymmetricVisibilityTokenEmulatorObj { fn clone(&self) -> Self { AsymmetricVisibilityTokenEmulatorObj {  } } }
impl AsymmetricVisibilityTokenEmulator {
}
pub struct AttributeEmulatorObj {
}
#[derive(Clone)]
pub struct AttributeEmulator(pub Rc<RefCell<AttributeEmulatorObj>>);
impl AttributeEmulator {
    pub fn new() -> Result<AttributeEmulator, Throw> {
        let this = AttributeEmulator(Rc::new(RefCell::new(AttributeEmulatorObj {
        })));
        Ok(this)
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> {
    return Ok(crate::php_parser::PhpVersion::fromComponents(8i64, 0i64)?);
    }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> {
    return Ok((!strpos(&code.clone(), &Str::from_static("#["), 0).is_none()));
    }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    let mut i: i64 = Default::default();
    let mut c: i64 = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    {
        i = 0i64;
        c = tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).count();
        'l1: loop {
            if !((i < c)) { break; }
            'c2: {
                token.set(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&i)).clone());
                if ((identical(&token.get().clone().p_text_get(), &Str::from_static("#")) && ({ let __k = to_key(&(i).wrapping_add(1i64)); Some(tokens_v.clone()).and_then(|__b| __b.get(&__k).cloned()) }.and_then(|__m| __m.to_option()).is_some())) && identical(&tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(i).wrapping_add(1i64))).clone().p_text_get(), &Str::from_static("["))) {
                    let _: List<Mixed> = { let __off = i; let __len = Some(2i64); let __repl = { let __c24 = (crate::php_parser::Token::new(355i64, Str::from_static("#["), token.get().clone().p_line_get(), token.get().clone().p_pos_get())?,); List::from_vec(vec![cast::<Mixed>(__c24.0)]) }.into_vec(); let __r = array_splice_m(&mut tokens_v, __off, __len, __repl); __r };
                    let _ = { let __t1 = c; c = __t1.wrapping_sub(1); __t1 };
                    { break 'c2 };
                }
            }
            let _ = { let __t2 = i; i = __t2.wrapping_add(1); __t2.wrapping_add(1) };
        }
    }
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> {
    let mut pos: i64 = Default::default();
    pos = 0i64;
    'l1: loop {
        if !(!(cast::<i64>(false) == { let __t2 = strpos(&code.clone(), &Str::from_static("#["), pos).unwrap_or_default(); pos = __t2.clone(); __t2 })) { break; }
        { let __h3 = Str::from_static("%"); str_set_index(&mut code, pos, &__h3); }
        { let __h4 = (pos, Str::from_static("replace"), Str::from_static("#")); (*patches).push(__h4); }
        { let __t5 = 2i64; pos = (pos).wrapping_add(__t5); }
    }
    return Ok(code.clone());
    }
    pub fn new_same_class(&self) -> Result<crate::php_parser::lexer::token_emulator::AttributeEmulator, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for AttributeEmulator {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\TokenEmulator\\AttributeEmulator" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\tokenemulator\\attributeemulator", "phpparser\\lexer\\tokenemulator\\tokenemulator"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getphpversion" => { let __r = self.getPhpVersion().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "isemulationneeded" => { let __r = self.isEmulationNeeded((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "emulate" => { let __r = self.emulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "reverseemulate" => { let __r = self.reverseEmulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Lexer\\TokenEmulator\\AttributeEmulator", name)))) } }
}
impl AttributeEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\TokenEmulator\\AttributeEmulator could not be converted to string"))) } }
impl php_rt::PhpClone for AttributeEmulator { fn php_clone(&self) -> Self { let c = AttributeEmulator(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for AttributeEmulatorObj { fn clone(&self) -> Self { AttributeEmulatorObj {  } } }
impl AttributeEmulator {
}
pub struct EnumTokenEmulatorObj {
}
#[derive(Clone)]
pub struct EnumTokenEmulator(pub Rc<RefCell<EnumTokenEmulatorObj>>);
impl EnumTokenEmulator {
    pub fn new() -> Result<EnumTokenEmulator, Throw> {
        let this = EnumTokenEmulator(Rc::new(RefCell::new(EnumTokenEmulatorObj {
        })));
        Ok(this)
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> {
    return Ok(crate::php_parser::PhpVersion::fromComponents(8i64, 1i64)?);
    }
    pub fn getKeywordString(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("enum"));
    }
    pub fn getKeywordToken(&self) -> Result<i64, Throw> {
    return Ok(339i64);
    }
    pub fn isKeywordContext(&self, mut tokens_v: Map<ArrayKey, Mixed>, mut pos: i64) -> Result<bool, Throw> {
    return Ok((((cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).isKeywordContext__impl(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)), pos)? && ({ let __k = to_key(&(pos).wrapping_add(2i64)); Some(tokens_v.clone()).and_then(|__b| __b.get(&__k).cloned()) }.and_then(|__m| __m.to_option()).is_some())) && (tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(pos).wrapping_add(1i64))).clone().p_id_get() == 397i64)) && (tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(pos).wrapping_add(2i64))).clone().p_id_get() == 262i64)));
    }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).isEmulationNeeded__impl(code) }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).emulate__impl(code, tokens_v) }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).reverseEmulate__impl(code, tokens_v) }
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> { cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(self.clone()).preprocessCode__impl(code, patches) }
    pub fn new_same_class(&self) -> Result<crate::php_parser::lexer::token_emulator::EnumTokenEmulator, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for EnumTokenEmulator {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\TokenEmulator\\EnumTokenEmulator" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\tokenemulator\\enumtokenemulator", "phpparser\\lexer\\tokenemulator\\keywordemulator", "phpparser\\lexer\\tokenemulator\\tokenemulator"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getphpversion" => { let __r = self.getPhpVersion().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getkeywordstring" => { let __r = self.getKeywordString().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getkeywordtoken" => { let __r = self.getKeywordToken().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "iskeywordcontext" => { let __r = self.isKeywordContext((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "isemulationneeded" => { let __r = self.isEmulationNeeded((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "emulate" => { let __r = self.emulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "reverseemulate" => { let __r = self.reverseEmulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Lexer\\TokenEmulator\\EnumTokenEmulator", name)))) } }
}
impl EnumTokenEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\TokenEmulator\\EnumTokenEmulator could not be converted to string"))) } }
impl php_rt::PhpClone for EnumTokenEmulator { fn php_clone(&self) -> Self { let c = EnumTokenEmulator(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for EnumTokenEmulatorObj { fn clone(&self) -> Self { EnumTokenEmulatorObj {  } } }
impl EnumTokenEmulator {
}
pub struct ExplicitOctalEmulatorObj {
}
#[derive(Clone)]
pub struct ExplicitOctalEmulator(pub Rc<RefCell<ExplicitOctalEmulatorObj>>);
impl ExplicitOctalEmulator {
    pub fn new() -> Result<ExplicitOctalEmulator, Throw> {
        let this = ExplicitOctalEmulator(Rc::new(RefCell::new(ExplicitOctalEmulatorObj {
        })));
        Ok(this)
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> {
    return Ok(crate::php_parser::PhpVersion::fromComponents(8i64, 1i64)?);
    }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> {
    return Ok(((!strpos(&code.clone(), &Str::from_static("0o"), 0).is_none()) || (!strpos(&code.clone(), &Str::from_static("0O"), 0).is_none())));
    }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    let mut i: i64 = Default::default();
    let mut c: i64 = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    let mut tokenKind: i64 = Default::default();
    {
        i = 0i64;
        c = tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).count();
        'l1: loop {
            if !((i < c)) { break; }
            'c2: {
                token.set(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&i)).clone());
                if (((((token.get().clone().p_id_get() == 260i64) && identical(&token.get().clone().p_text_get(), &Str::from_static("0"))) && ({ let __k = to_key(&(i).wrapping_add(1i64)); Some(tokens_v.clone()).and_then(|__b| __b.get(&__k).cloned()) }.and_then(|__m| __m.to_option()).is_some())) && (tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(i).wrapping_add(1i64))).clone().p_id_get() == 262i64)) && truthy(&preg_match(&Str::from_static("/[oO][0-7]+(?:_[0-7]+)*/"), &tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(i).wrapping_add(1i64))).clone().p_text_get(), 0)?)) {
                    tokenKind = self.resolveIntegerOrFloatToken(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(i).wrapping_add(1i64))).clone().p_text_get())?;
                    let _: List<Mixed> = { let __off = i; let __len = Some(2i64); let __repl = { let __c25 = (crate::php_parser::Token::new(tokenKind, concat(Str::from_static("0"), tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(i).wrapping_add(1i64))).clone().p_text_get()), token.get().clone().p_line_get(), token.get().clone().p_pos_get())?,); List::from_vec(vec![cast::<Mixed>(__c25.0)]) }.into_vec(); let __r = array_splice_m(&mut tokens_v, __off, __len, __repl); __r };
                    let _ = { let __t1 = c; c = __t1.wrapping_sub(1); __t1 };
                }
            }
            let _ = { let __t2 = i; i = __t2.wrapping_add(1); __t2.wrapping_add(1) };
        }
    }
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn resolveIntegerOrFloatToken(&self, mut str: Str) -> Result<i64, Throw> {
    let mut num: Late<U_Float_or_Int> = Late::uninit();
    str = substr(&str.clone(), 1i64, None);
    str = str_replace(&Str::from_static("_"), &Str::from_static(""), &str.clone());
    num.set(U_Float_or_Int::Int(octdec(&str.clone())));
    return Ok((if (match num.get().clone() { U_Float_or_Int::Float(_) => true, U_Float_or_Int::Other__(__m) => __m.is_float(), _ => false }) { 261i64 } else { 260i64 }));
    }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> { cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(self.clone()).preprocessCode__impl(code, patches) }
    pub fn new_same_class(&self) -> Result<crate::php_parser::lexer::token_emulator::ExplicitOctalEmulator, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for ExplicitOctalEmulator {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\TokenEmulator\\ExplicitOctalEmulator" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\tokenemulator\\explicitoctalemulator", "phpparser\\lexer\\tokenemulator\\tokenemulator"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getphpversion" => { let __r = self.getPhpVersion().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "isemulationneeded" => { let __r = self.isEmulationNeeded((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "emulate" => { let __r = self.emulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "resolveintegerorfloattoken" => { let __r = self.resolveIntegerOrFloatToken((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "reverseemulate" => { let __r = self.reverseEmulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Lexer\\TokenEmulator\\ExplicitOctalEmulator", name)))) } }
}
impl ExplicitOctalEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\TokenEmulator\\ExplicitOctalEmulator could not be converted to string"))) } }
impl php_rt::PhpClone for ExplicitOctalEmulator { fn php_clone(&self) -> Self { let c = ExplicitOctalEmulator(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ExplicitOctalEmulatorObj { fn clone(&self) -> Self { ExplicitOctalEmulatorObj {  } } }
impl ExplicitOctalEmulator {
}
pub struct FnTokenEmulatorObj {
}
#[derive(Clone)]
pub struct FnTokenEmulator(pub Rc<RefCell<FnTokenEmulatorObj>>);
impl FnTokenEmulator {
    pub fn new() -> Result<FnTokenEmulator, Throw> {
        let this = FnTokenEmulator(Rc::new(RefCell::new(FnTokenEmulatorObj {
        })));
        Ok(this)
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> {
    return Ok(crate::php_parser::PhpVersion::fromString(Str::from_static("7.4"))?);
    }
    pub fn getKeywordString(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("fn"));
    }
    pub fn getKeywordToken(&self) -> Result<i64, Throw> {
    return Ok(311i64);
    }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).isEmulationNeeded__impl(code) }
    pub fn isKeywordContext(&self, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>, mut pos: i64) -> Result<bool, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).isKeywordContext__impl(tokens_v, pos) }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).emulate__impl(code, tokens_v) }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).reverseEmulate__impl(code, tokens_v) }
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> { cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(self.clone()).preprocessCode__impl(code, patches) }
    pub fn new_same_class(&self) -> Result<crate::php_parser::lexer::token_emulator::FnTokenEmulator, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for FnTokenEmulator {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\TokenEmulator\\FnTokenEmulator" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\tokenemulator\\fntokenemulator", "phpparser\\lexer\\tokenemulator\\keywordemulator", "phpparser\\lexer\\tokenemulator\\tokenemulator"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getphpversion" => { let __r = self.getPhpVersion().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getkeywordstring" => { let __r = self.getKeywordString().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getkeywordtoken" => { let __r = self.getKeywordToken().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "isemulationneeded" => { let __r = self.isEmulationNeeded((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "iskeywordcontext" => { let __r = self.isKeywordContext((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, crate::php_parser::Token>>(__a.clone()), None => <Map<ArrayKey, crate::php_parser::Token>>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "emulate" => { let __r = self.emulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "reverseemulate" => { let __r = self.reverseEmulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Lexer\\TokenEmulator\\FnTokenEmulator", name)))) } }
}
impl FnTokenEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\TokenEmulator\\FnTokenEmulator could not be converted to string"))) } }
impl php_rt::PhpClone for FnTokenEmulator { fn php_clone(&self) -> Self { let c = FnTokenEmulator(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for FnTokenEmulatorObj { fn clone(&self) -> Self { FnTokenEmulatorObj {  } } }
impl FnTokenEmulator {
}
#[derive(Clone)]
pub enum KeywordEmulator {
    PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(crate::php_parser::lexer::token_emulator::EnumTokenEmulator),
    PhpParser_Lexer_TokenEmulator_FnTokenEmulator(crate::php_parser::lexer::token_emulator::FnTokenEmulator),
    PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(crate::php_parser::lexer::token_emulator::MatchTokenEmulator),
    PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(crate::php_parser::lexer::token_emulator::PropertyTokenEmulator),
    PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(crate::php_parser::lexer::token_emulator::ReadonlyFunctionTokenEmulator),
    PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(crate::php_parser::lexer::token_emulator::ReadonlyTokenEmulator),
}
impl php_rt::PhpObject for KeywordEmulator {
    fn class_name(&self) -> &'static str { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.class_name(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.class_name(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.class_name(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.class_name(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.class_name(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.class_name(), _ => unreachable!() } }
    fn class_ancestors(&self) -> &'static [&'static str] { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.class_ancestors(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.class_ancestors(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.class_ancestors(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.class_ancestors(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.class_ancestors(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.class_ancestors(), _ => unreachable!() } }
    fn obj_id(&self) -> usize { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.obj_id(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.obj_id(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.obj_id(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.obj_id(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.obj_id(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.obj_id(), _ => unreachable!() } }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.props(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.props(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.props(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.props(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.props(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.props(), _ => unreachable!() } }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.set_prop(name, value), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.set_prop(name, value), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.set_prop(name, value), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.set_prop(name, value), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.set_prop(name, value), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.set_prop(name, value), _ => unreachable!() } }
    fn php_to_string(&self) -> Option<Str> { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.php_to_string(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.php_to_string(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.php_to_string(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.php_to_string(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.php_to_string(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.php_to_string(), _ => unreachable!() } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.call_method(name, args), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.call_method(name, args), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.call_method(name, args), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.call_method(name, args), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.call_method(name, args), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.call_method(name, args), _ => unreachable!() } }
    fn public_props(&self) -> Vec<(Str, Mixed)> { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.public_props(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.public_props(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.public_props(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.public_props(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.public_props(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.public_props(), _ => unreachable!() } }
}
impl KeywordEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.to_php_string(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.to_php_string(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.to_php_string(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.to_php_string(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.to_php_string(), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.to_php_string(), _ => unreachable!() } } }
impl php_rt::PhpClone for KeywordEmulator { fn php_clone(&self) -> Self { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h.php_clone()), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h.php_clone()), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h.php_clone()), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h.php_clone()), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h.php_clone()), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h.php_clone()), _ => unreachable!() } } }
impl KeywordEmulator {
    pub fn getKeywordString(&self) -> Result<Str, Throw> { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(__h.getKeywordString()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(__h.getKeywordString()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(__h.getKeywordString()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(__h.getKeywordString()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(__h.getKeywordString()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(__h.getKeywordString()?), _ => unreachable!() } }
    pub fn getKeywordToken(&self) -> Result<i64, Throw> { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(__h.getKeywordToken()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(__h.getKeywordToken()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(__h.getKeywordToken()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(__h.getKeywordToken()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(__h.getKeywordToken()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(__h.getKeywordToken()?), _ => unreachable!() } }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), _ => unreachable!() } }
    pub fn isEmulationNeeded__impl(&self, mut code: Str) -> Result<bool, Throw> {
    return Ok((!strpos(&strtolower(&code.clone()), &self.getKeywordString()?, 0).is_none()));
    }
    pub fn isKeywordContext(&self, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>, mut pos: i64) -> Result<bool, Throw> { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(__h.isKeywordContext(tokens_v.map_values(|v| cast::<Mixed>(v)), pos)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(__h.isKeywordContext(tokens_v, pos)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(__h.isKeywordContext(tokens_v, pos)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(__h.isKeywordContext(tokens_v, pos)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(__h.isKeywordContext(tokens_v, pos)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(__h.isKeywordContext(tokens_v.map_values(|v| cast::<Mixed>(v)), pos)?), _ => unreachable!() } }
    pub fn isKeywordContext__impl(&self, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>, mut pos: i64) -> Result<bool, Throw> {
    let mut prevToken: Option<crate::php_parser::Token> = Default::default();
    prevToken = self.getPreviousNonIgnorableToken(tokens_v.clone(), pos)?;
    if prevToken.clone().is_none() {
        return Ok(false);
    }
    return Ok(((!(prevToken.clone().unwrap().p_id_get() == 389i64)) && (!(prevToken.clone().unwrap().p_id_get() == 390i64))));
    }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(__h.emulate(code, tokens_v)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(__h.emulate(code, tokens_v)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(__h.emulate(code, tokens_v)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(__h.emulate(code, tokens_v)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(__h.emulate(code, tokens_v)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(__h.emulate(code, tokens_v)?), _ => unreachable!() } }
    pub fn emulate__impl(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    let mut keywordString: Str = Default::default();
    let mut i: ArrayKey = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    keywordString = self.getKeywordString()?;
    'l1: for __kv1 in tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).into_iter() {
        i = __kv1.0;
        token.set(__kv1.1);
        if (((token.get().clone().p_id_get() == 262i64) && identical(&strtolower(&token.get().clone().p_text_get()), &keywordString.clone())) && self.isKeywordContext(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)), cast::<i64>(i.clone()))?) {
            token.get().clone().set_p_id(self.getKeywordToken()?);
        }
    }
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn getPreviousNonIgnorableToken(&self, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>, mut start: i64) -> Result<Option<crate::php_parser::Token>, Throw> {
    let mut i: i64 = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    {
        i = (start).wrapping_sub(1i64);
        'l1: loop {
            if !((i >= 0i64)) { break; }
            'c2: {
                token.set(tokens_v.clone().idx(&to_key(&i)).clone());
                if (((token.get().clone().p_id_get() == 397i64) || (token.get().clone().p_id_get() == 392i64)) || (token.get().clone().p_id_get() == 393i64)) {
                    { break 'c2 };
                }
                return Ok(Some(token.get().clone()));
            }
            let _ = { let __t1 = i; i = __t1.wrapping_sub(1); __t1.wrapping_sub(1) };
        }
    }
    return Ok({ let _ = (); None::<crate::php_parser::Token> });
    }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v)?), _ => unreachable!() } }
    pub fn reverseEmulate__impl(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    let mut keywordToken: i64 = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    keywordToken = self.getKeywordToken()?;
    'l1: for __kv1 in tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).into_iter() {
        token.set(__kv1.1);
        if (token.get().clone().p_id_get() == keywordToken) {
            token.get().clone().set_p_id(262i64);
        }
    }
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(__h.getPhpVersion()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(__h.getPhpVersion()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(__h.getPhpVersion()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(__h.getPhpVersion()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(__h.getPhpVersion()?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(__h.getPhpVersion()?), _ => unreachable!() } }
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), _ => unreachable!() } }
    pub fn preprocessCode__impl(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> { cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(self.clone()).preprocessCode__impl(code, patches) }
    pub fn new_same_class(&self) -> Result<KeywordEmulator, Throw> { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(__h.new_same_class()?)), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(__h.new_same_class()?)), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(__h.new_same_class()?)), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(__h.new_same_class()?)), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(__h.new_same_class()?)), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(__h.new_same_class()?)), _ => unreachable!() } }
}
pub struct MatchTokenEmulatorObj {
}
#[derive(Clone)]
pub struct MatchTokenEmulator(pub Rc<RefCell<MatchTokenEmulatorObj>>);
impl MatchTokenEmulator {
    pub fn new() -> Result<MatchTokenEmulator, Throw> {
        let this = MatchTokenEmulator(Rc::new(RefCell::new(MatchTokenEmulatorObj {
        })));
        Ok(this)
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> {
    return Ok(crate::php_parser::PhpVersion::fromComponents(8i64, 0i64)?);
    }
    pub fn getKeywordString(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("match"));
    }
    pub fn getKeywordToken(&self) -> Result<i64, Throw> {
    return Ok(306i64);
    }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).isEmulationNeeded__impl(code) }
    pub fn isKeywordContext(&self, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>, mut pos: i64) -> Result<bool, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).isKeywordContext__impl(tokens_v, pos) }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).emulate__impl(code, tokens_v) }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).reverseEmulate__impl(code, tokens_v) }
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> { cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(self.clone()).preprocessCode__impl(code, patches) }
    pub fn new_same_class(&self) -> Result<crate::php_parser::lexer::token_emulator::MatchTokenEmulator, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for MatchTokenEmulator {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\TokenEmulator\\MatchTokenEmulator" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\tokenemulator\\matchtokenemulator", "phpparser\\lexer\\tokenemulator\\keywordemulator", "phpparser\\lexer\\tokenemulator\\tokenemulator"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getphpversion" => { let __r = self.getPhpVersion().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getkeywordstring" => { let __r = self.getKeywordString().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getkeywordtoken" => { let __r = self.getKeywordToken().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "isemulationneeded" => { let __r = self.isEmulationNeeded((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "iskeywordcontext" => { let __r = self.isKeywordContext((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, crate::php_parser::Token>>(__a.clone()), None => <Map<ArrayKey, crate::php_parser::Token>>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "emulate" => { let __r = self.emulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "reverseemulate" => { let __r = self.reverseEmulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Lexer\\TokenEmulator\\MatchTokenEmulator", name)))) } }
}
impl MatchTokenEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\TokenEmulator\\MatchTokenEmulator could not be converted to string"))) } }
impl php_rt::PhpClone for MatchTokenEmulator { fn php_clone(&self) -> Self { let c = MatchTokenEmulator(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for MatchTokenEmulatorObj { fn clone(&self) -> Self { MatchTokenEmulatorObj {  } } }
impl MatchTokenEmulator {
}
pub struct NullsafeTokenEmulatorObj {
}
#[derive(Clone)]
pub struct NullsafeTokenEmulator(pub Rc<RefCell<NullsafeTokenEmulatorObj>>);
impl NullsafeTokenEmulator {
    pub fn new() -> Result<NullsafeTokenEmulator, Throw> {
        let this = NullsafeTokenEmulator(Rc::new(RefCell::new(NullsafeTokenEmulatorObj {
        })));
        Ok(this)
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> {
    return Ok(crate::php_parser::PhpVersion::fromComponents(8i64, 0i64)?);
    }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> {
    return Ok((!strpos(&code.clone(), &Str::from_static("?->"), 0).is_none()));
    }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    let mut i: i64 = Default::default();
    let mut c: i64 = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    let mut matches: Map<ArrayKey, Str> = Default::default();
    let mut replacement: List<crate::php_parser::Token> = Default::default();
    let mut matchLen: i64 = Default::default();
    {
        i = 0i64;
        c = tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).count();
        'l1: loop {
            if !((i < c)) { break; }
            'c2: {
                token.set(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&i)).clone());
                if ((identical(&token.get().clone().p_text_get(), &Str::from_static("?")) && ({ let __k = to_key(&(i).wrapping_add(1i64)); Some(tokens_v.clone()).and_then(|__b| __b.get(&__k).cloned()) }.and_then(|__m| __m.to_option()).is_some())) && (tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(i).wrapping_add(1i64))).clone().p_id_get() == 389i64)) {
                    let _: List<Mixed> = { let __off = i; let __len = Some(2i64); let __repl = { let __c26 = (crate::php_parser::Token::new(390i64, Str::from_static("?->"), token.get().clone().p_line_get(), token.get().clone().p_pos_get())?,); List::from_vec(vec![cast::<Mixed>(__c26.0)]) }.into_vec(); let __r = array_splice_m(&mut tokens_v, __off, __len, __repl); __r };
                    let _ = { let __t1 = c; c = __t1.wrapping_sub(1); __t1 };
                    { break 'c2 };
                }
                if ((((token.get().clone().p_id_get() == 268i64) && ({ let __k = to_key(&(i).wrapping_sub(1i64)); Some(tokens_v.clone()).and_then(|__b| __b.get(&__k).cloned()) }.and_then(|__m| __m.to_option()).is_some())) && (tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(i).wrapping_sub(1i64))).clone().p_id_get() == 266i64)) && truthy(&{ let (__r, __m) = preg_match_groups(&Str::from_static("/^\\?->([a-zA-Z_\\x80-\\xff][a-zA-Z0-9_\\x80-\\xff]*)/"), &token.get().clone().p_text_get(), 0)?; matches = __m; __r })) {
                    replacement = list![crate::php_parser::Token::new(390i64, Str::from_static("?->"), token.get().clone().p_line_get(), token.get().clone().p_pos_get())?, crate::php_parser::Token::new(262i64, matches.clone().idx(&to_key(&1i64)).clone(), token.get().clone().p_line_get(), (token.get().clone().p_pos_get()).wrapping_add(3i64))?];
                    matchLen = strlen(&matches.clone().idx(&to_key(&0i64)).clone());
                    if (!(matchLen == strlen(&token.get().clone().p_text_get()))) {
                        { let __h2 = crate::php_parser::Token::new(268i64, substr(&token.get().clone().p_text_get(), matchLen, None), token.get().clone().p_line_get(), (token.get().clone().p_pos_get()).wrapping_add(matchLen))?; replacement.push(__h2); }
                    }
                    let _: List<Mixed> = { let __off = i; let __len = Some(1i64); let __repl = replacement.clone().map_elems(|v| cast::<Mixed>(v)).into_vec(); let __r = array_splice_m(&mut tokens_v, __off, __len, __repl); __r };
                    { let __t3 = (replacement.clone().count()).wrapping_sub(1i64); c = (c).wrapping_add(__t3); }
                    { break 'c2 };
                }
            }
            let _ = { let __t4 = i; i = __t4.wrapping_add(1); __t4.wrapping_add(1) };
        }
    }
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> { cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(self.clone()).preprocessCode__impl(code, patches) }
    pub fn new_same_class(&self) -> Result<crate::php_parser::lexer::token_emulator::NullsafeTokenEmulator, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for NullsafeTokenEmulator {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\TokenEmulator\\NullsafeTokenEmulator" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\tokenemulator\\nullsafetokenemulator", "phpparser\\lexer\\tokenemulator\\tokenemulator"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getphpversion" => { let __r = self.getPhpVersion().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "isemulationneeded" => { let __r = self.isEmulationNeeded((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "emulate" => { let __r = self.emulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "reverseemulate" => { let __r = self.reverseEmulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Lexer\\TokenEmulator\\NullsafeTokenEmulator", name)))) } }
}
impl NullsafeTokenEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\TokenEmulator\\NullsafeTokenEmulator could not be converted to string"))) } }
impl php_rt::PhpClone for NullsafeTokenEmulator { fn php_clone(&self) -> Self { let c = NullsafeTokenEmulator(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for NullsafeTokenEmulatorObj { fn clone(&self) -> Self { NullsafeTokenEmulatorObj {  } } }
impl NullsafeTokenEmulator {
}
pub struct PipeOperatorEmulatorObj {
}
#[derive(Clone)]
pub struct PipeOperatorEmulator(pub Rc<RefCell<PipeOperatorEmulatorObj>>);
impl PipeOperatorEmulator {
    pub fn new() -> Result<PipeOperatorEmulator, Throw> {
        let this = PipeOperatorEmulator(Rc::new(RefCell::new(PipeOperatorEmulatorObj {
        })));
        Ok(this)
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> {
    return Ok(crate::php_parser::PhpVersion::fromComponents(8i64, 5i64)?);
    }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> {
    return Ok((!strpos(&code.clone(), &Str::from_static("|>"), 0).is_none()));
    }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    let mut i: i64 = Default::default();
    let mut c: i64 = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    {
        i = 0i64;
        c = tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).count();
        'l1: loop {
            if !((i < c)) { break; }
            'c2: {
                token.set(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&i)).clone());
                if ((identical(&token.get().clone().p_text_get(), &Str::from_static("|")) && ({ let __k = to_key(&(i).wrapping_add(1i64)); Some(tokens_v.clone()).and_then(|__b| __b.get(&__k).cloned()) }.and_then(|__m| __m.to_option()).is_some())) && identical(&tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(i).wrapping_add(1i64))).clone().p_text_get(), &Str::from_static(">"))) {
                    let _: List<Mixed> = { let __off = i; let __len = Some(2i64); let __repl = { let __c27 = (crate::php_parser::Token::new(408i64, Str::from_static("|>"), token.get().clone().p_line_get(), token.get().clone().p_pos_get())?,); List::from_vec(vec![cast::<Mixed>(__c27.0)]) }.into_vec(); let __r = array_splice_m(&mut tokens_v, __off, __len, __repl); __r };
                    let _ = { let __t1 = c; c = __t1.wrapping_sub(1); __t1 };
                }
            }
            let _ = { let __t2 = i; i = __t2.wrapping_add(1); __t2.wrapping_add(1) };
        }
    }
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    let mut i: i64 = Default::default();
    let mut c: i64 = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    {
        i = 0i64;
        c = tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).count();
        'l1: loop {
            if !((i < c)) { break; }
            'c2: {
                token.set(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&i)).clone());
                if (token.get().clone().p_id_get() == 408i64) {
                    let _: List<Mixed> = { let __off = i; let __len = Some(1i64); let __repl = { let __c28 = (crate::php_parser::Token::new(ord(&Str::from_static("|")), Str::from_static("|"), token.get().clone().p_line_get(), token.get().clone().p_pos_get())?, crate::php_parser::Token::new(ord(&Str::from_static(">")), Str::from_static(">"), token.get().clone().p_line_get(), (token.get().clone().p_pos_get()).wrapping_add(1i64))?); List::from_vec(vec![cast::<Mixed>(__c28.0), cast::<Mixed>(__c28.1)]) }.into_vec(); let __r = array_splice_m(&mut tokens_v, __off, __len, __repl); __r };
                    let _ = { let __t1 = i; i = __t1.wrapping_add(1); __t1 };
                    let _ = { let __t2 = c; c = __t2.wrapping_add(1); __t2 };
                }
            }
            let _ = { let __t3 = i; i = __t3.wrapping_add(1); __t3.wrapping_add(1) };
        }
    }
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> { cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(self.clone()).preprocessCode__impl(code, patches) }
    pub fn new_same_class(&self) -> Result<crate::php_parser::lexer::token_emulator::PipeOperatorEmulator, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for PipeOperatorEmulator {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\TokenEmulator\\PipeOperatorEmulator" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\tokenemulator\\pipeoperatoremulator", "phpparser\\lexer\\tokenemulator\\tokenemulator"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getphpversion" => { let __r = self.getPhpVersion().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "isemulationneeded" => { let __r = self.isEmulationNeeded((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "emulate" => { let __r = self.emulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "reverseemulate" => { let __r = self.reverseEmulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Lexer\\TokenEmulator\\PipeOperatorEmulator", name)))) } }
}
impl PipeOperatorEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\TokenEmulator\\PipeOperatorEmulator could not be converted to string"))) } }
impl php_rt::PhpClone for PipeOperatorEmulator { fn php_clone(&self) -> Self { let c = PipeOperatorEmulator(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for PipeOperatorEmulatorObj { fn clone(&self) -> Self { PipeOperatorEmulatorObj {  } } }
impl PipeOperatorEmulator {
}
pub struct PropertyTokenEmulatorObj {
}
#[derive(Clone)]
pub struct PropertyTokenEmulator(pub Rc<RefCell<PropertyTokenEmulatorObj>>);
impl PropertyTokenEmulator {
    pub fn new() -> Result<PropertyTokenEmulator, Throw> {
        let this = PropertyTokenEmulator(Rc::new(RefCell::new(PropertyTokenEmulatorObj {
        })));
        Ok(this)
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> {
    return Ok(crate::php_parser::PhpVersion::fromComponents(8i64, 4i64)?);
    }
    pub fn getKeywordString(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("__property__"));
    }
    pub fn getKeywordToken(&self) -> Result<i64, Throw> {
    return Ok(353i64);
    }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).isEmulationNeeded__impl(code) }
    pub fn isKeywordContext(&self, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>, mut pos: i64) -> Result<bool, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).isKeywordContext__impl(tokens_v, pos) }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).emulate__impl(code, tokens_v) }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).reverseEmulate__impl(code, tokens_v) }
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> { cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(self.clone()).preprocessCode__impl(code, patches) }
    pub fn new_same_class(&self) -> Result<crate::php_parser::lexer::token_emulator::PropertyTokenEmulator, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for PropertyTokenEmulator {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\TokenEmulator\\PropertyTokenEmulator" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\tokenemulator\\propertytokenemulator", "phpparser\\lexer\\tokenemulator\\keywordemulator", "phpparser\\lexer\\tokenemulator\\tokenemulator"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getphpversion" => { let __r = self.getPhpVersion().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getkeywordstring" => { let __r = self.getKeywordString().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getkeywordtoken" => { let __r = self.getKeywordToken().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "isemulationneeded" => { let __r = self.isEmulationNeeded((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "iskeywordcontext" => { let __r = self.isKeywordContext((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, crate::php_parser::Token>>(__a.clone()), None => <Map<ArrayKey, crate::php_parser::Token>>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "emulate" => { let __r = self.emulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "reverseemulate" => { let __r = self.reverseEmulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Lexer\\TokenEmulator\\PropertyTokenEmulator", name)))) } }
}
impl PropertyTokenEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\TokenEmulator\\PropertyTokenEmulator could not be converted to string"))) } }
impl php_rt::PhpClone for PropertyTokenEmulator { fn php_clone(&self) -> Self { let c = PropertyTokenEmulator(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for PropertyTokenEmulatorObj { fn clone(&self) -> Self { PropertyTokenEmulatorObj {  } } }
impl PropertyTokenEmulator {
}
pub struct ReadonlyFunctionTokenEmulatorObj {
}
#[derive(Clone)]
pub struct ReadonlyFunctionTokenEmulator(pub Rc<RefCell<ReadonlyFunctionTokenEmulatorObj>>);
impl ReadonlyFunctionTokenEmulator {
    pub fn new() -> Result<ReadonlyFunctionTokenEmulator, Throw> {
        let this = ReadonlyFunctionTokenEmulator(Rc::new(RefCell::new(ReadonlyFunctionTokenEmulatorObj {
        })));
        Ok(this)
    }
    pub fn getKeywordString(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("readonly"));
    }
    pub fn getKeywordToken(&self) -> Result<i64, Throw> {
    return Ok(330i64);
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> {
    return Ok(crate::php_parser::PhpVersion::fromComponents(8i64, 2i64)?);
    }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).isEmulationNeeded__impl(code) }
    pub fn isKeywordContext(&self, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>, mut pos: i64) -> Result<bool, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).isKeywordContext__impl(tokens_v, pos) }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).emulate__impl(code, tokens_v) }
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> { cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(self.clone()).preprocessCode__impl(code, patches) }
    pub fn new_same_class(&self) -> Result<crate::php_parser::lexer::token_emulator::ReadonlyFunctionTokenEmulator, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for ReadonlyFunctionTokenEmulator {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\TokenEmulator\\ReadonlyFunctionTokenEmulator" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\tokenemulator\\readonlyfunctiontokenemulator", "phpparser\\lexer\\tokenemulator\\keywordemulator", "phpparser\\lexer\\tokenemulator\\tokenemulator"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getkeywordstring" => { let __r = self.getKeywordString().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getkeywordtoken" => { let __r = self.getKeywordToken().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getphpversion" => { let __r = self.getPhpVersion().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "reverseemulate" => { let __r = self.reverseEmulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "isemulationneeded" => { let __r = self.isEmulationNeeded((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "iskeywordcontext" => { let __r = self.isKeywordContext((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, crate::php_parser::Token>>(__a.clone()), None => <Map<ArrayKey, crate::php_parser::Token>>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "emulate" => { let __r = self.emulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Lexer\\TokenEmulator\\ReadonlyFunctionTokenEmulator", name)))) } }
}
impl ReadonlyFunctionTokenEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\TokenEmulator\\ReadonlyFunctionTokenEmulator could not be converted to string"))) } }
impl php_rt::PhpClone for ReadonlyFunctionTokenEmulator { fn php_clone(&self) -> Self { let c = ReadonlyFunctionTokenEmulator(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ReadonlyFunctionTokenEmulatorObj { fn clone(&self) -> Self { ReadonlyFunctionTokenEmulatorObj {  } } }
impl ReadonlyFunctionTokenEmulator {
}
pub struct ReadonlyTokenEmulatorObj {
}
#[derive(Clone)]
pub struct ReadonlyTokenEmulator(pub Rc<RefCell<ReadonlyTokenEmulatorObj>>);
impl ReadonlyTokenEmulator {
    pub fn new() -> Result<ReadonlyTokenEmulator, Throw> {
        let this = ReadonlyTokenEmulator(Rc::new(RefCell::new(ReadonlyTokenEmulatorObj {
        })));
        Ok(this)
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> {
    return Ok(crate::php_parser::PhpVersion::fromComponents(8i64, 1i64)?);
    }
    pub fn getKeywordString(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("readonly"));
    }
    pub fn getKeywordToken(&self) -> Result<i64, Throw> {
    return Ok(330i64);
    }
    pub fn isKeywordContext(&self, mut tokens_v: Map<ArrayKey, Mixed>, mut pos: i64) -> Result<bool, Throw> {
    if (!cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).isKeywordContext__impl(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)), pos)?) {
        return Ok(false);
    }
    return Ok((!(({ let __k = to_key(&(pos).wrapping_add(1i64)); Some(tokens_v.clone()).and_then(|__b| __b.get(&__k).cloned()) }.and_then(|__m| __m.to_option()).is_some()) && (identical(&tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(pos).wrapping_add(1i64))).clone().p_text_get(), &Str::from_static("(")) || (((tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(pos).wrapping_add(1i64))).clone().p_id_get() == 397i64) && ({ let __k = to_key(&(pos).wrapping_add(2i64)); Some(tokens_v.clone()).and_then(|__b| __b.get(&__k).cloned()) }.and_then(|__m| __m.to_option()).is_some())) && identical(&tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&(pos).wrapping_add(2i64))).clone().p_text_get(), &Str::from_static("(")))))));
    }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).isEmulationNeeded__impl(code) }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).emulate__impl(code, tokens_v) }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> { cast::<crate::php_parser::lexer::token_emulator::KeywordEmulator>(self.clone()).reverseEmulate__impl(code, tokens_v) }
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> { cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(self.clone()).preprocessCode__impl(code, patches) }
    pub fn new_same_class(&self) -> Result<crate::php_parser::lexer::token_emulator::ReadonlyTokenEmulator, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for ReadonlyTokenEmulator {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\TokenEmulator\\ReadonlyTokenEmulator" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\tokenemulator\\readonlytokenemulator", "phpparser\\lexer\\tokenemulator\\keywordemulator", "phpparser\\lexer\\tokenemulator\\tokenemulator"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getphpversion" => { let __r = self.getPhpVersion().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getkeywordstring" => { let __r = self.getKeywordString().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getkeywordtoken" => { let __r = self.getKeywordToken().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "iskeywordcontext" => { let __r = self.isKeywordContext((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "isemulationneeded" => { let __r = self.isEmulationNeeded((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "emulate" => { let __r = self.emulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "reverseemulate" => { let __r = self.reverseEmulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Lexer\\TokenEmulator\\ReadonlyTokenEmulator", name)))) } }
}
impl ReadonlyTokenEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\TokenEmulator\\ReadonlyTokenEmulator could not be converted to string"))) } }
impl php_rt::PhpClone for ReadonlyTokenEmulator { fn php_clone(&self) -> Self { let c = ReadonlyTokenEmulator(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ReadonlyTokenEmulatorObj { fn clone(&self) -> Self { ReadonlyTokenEmulatorObj {  } } }
impl ReadonlyTokenEmulator {
}
pub struct ReverseEmulatorObj {
    pub emulator: Late<crate::php_parser::lexer::token_emulator::TokenEmulator>,
}
#[derive(Clone)]
pub struct ReverseEmulator(pub Rc<RefCell<ReverseEmulatorObj>>);
impl ReverseEmulator {
    pub fn p_emulator(&self) -> Ref<'_, crate::php_parser::lexer::token_emulator::TokenEmulator> { Ref::map(self.0.borrow(), |o| o.emulator.get()) }
    pub fn p_emulator_get(&self) -> crate::php_parser::lexer::token_emulator::TokenEmulator { self.0.borrow().emulator.get().clone() }
    pub fn p_emulator_opt(&self) -> Option<crate::php_parser::lexer::token_emulator::TokenEmulator> { self.0.borrow().emulator.as_option().cloned() }
    pub fn p_emulator_mut(&self) -> RefMut<'_, crate::php_parser::lexer::token_emulator::TokenEmulator> { RefMut::map(self.0.borrow_mut(), |o| o.emulator.get_mut()) }
    pub fn set_p_emulator(&self, v: crate::php_parser::lexer::token_emulator::TokenEmulator) { self.0.borrow_mut().emulator.set(v); }
    pub fn new(mut emulator: crate::php_parser::lexer::token_emulator::TokenEmulator) -> Result<ReverseEmulator, Throw> {
        let this = ReverseEmulator(Rc::new(RefCell::new(ReverseEmulatorObj {
            emulator: Late::uninit(),
        })));
        this.magic__construct(emulator)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut emulator: crate::php_parser::lexer::token_emulator::TokenEmulator) -> Result<Mixed, Throw> {
    self.set_p_emulator(emulator.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> {
    return Ok(self.p_emulator_get().getPhpVersion()?);
    }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> {
    return Ok(self.p_emulator_get().isEmulationNeeded(code.clone())?);
    }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok(self.p_emulator_get().reverseEmulate(code.clone(), tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)))?.map_values(|v| cast::<Mixed>(v)));
    }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok(self.p_emulator_get().emulate(code.clone(), tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)))?.map_values(|v| cast::<Mixed>(v)));
    }
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> {
    return Ok(code.clone());
    }
    pub fn new_same_class(&self, mut emulator: crate::php_parser::lexer::token_emulator::TokenEmulator) -> Result<crate::php_parser::lexer::token_emulator::ReverseEmulator, Throw> { Ok(Self::new(emulator)?) }
}
impl php_rt::PhpObject for ReverseEmulator {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\TokenEmulator\\ReverseEmulator" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\tokenemulator\\reverseemulator", "phpparser\\lexer\\tokenemulator\\tokenemulator"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_emulator_opt() { out.push((Str::from_static("emulator"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "emulator" => { self.set_p_emulator(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(__a.clone()), None => unreachable!("no default for crate::php_parser::lexer::token_emulator::TokenEmulator") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getphpversion" => { let __r = self.getPhpVersion().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "isemulationneeded" => { let __r = self.isEmulationNeeded((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "emulate" => { let __r = self.emulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "reverseemulate" => { let __r = self.reverseEmulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Lexer\\TokenEmulator\\ReverseEmulator", name)))) } }
}
impl ReverseEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\TokenEmulator\\ReverseEmulator could not be converted to string"))) } }
impl php_rt::PhpClone for ReverseEmulator { fn php_clone(&self) -> Self { let c = ReverseEmulator(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ReverseEmulatorObj { fn clone(&self) -> Self { ReverseEmulatorObj { emulator: self.emulator.clone() } } }
impl ReverseEmulator {
}
#[derive(Clone)]
pub enum TokenEmulator {
    PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(crate::php_parser::lexer::token_emulator::AsymmetricVisibilityTokenEmulator),
    PhpParser_Lexer_TokenEmulator_AttributeEmulator(crate::php_parser::lexer::token_emulator::AttributeEmulator),
    PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(crate::php_parser::lexer::token_emulator::EnumTokenEmulator),
    PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(crate::php_parser::lexer::token_emulator::ExplicitOctalEmulator),
    PhpParser_Lexer_TokenEmulator_FnTokenEmulator(crate::php_parser::lexer::token_emulator::FnTokenEmulator),
    PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(crate::php_parser::lexer::token_emulator::MatchTokenEmulator),
    PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(crate::php_parser::lexer::token_emulator::NullsafeTokenEmulator),
    PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(crate::php_parser::lexer::token_emulator::PipeOperatorEmulator),
    PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(crate::php_parser::lexer::token_emulator::PropertyTokenEmulator),
    PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(crate::php_parser::lexer::token_emulator::ReadonlyFunctionTokenEmulator),
    PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(crate::php_parser::lexer::token_emulator::ReadonlyTokenEmulator),
    PhpParser_Lexer_TokenEmulator_ReverseEmulator(crate::php_parser::lexer::token_emulator::ReverseEmulator),
    PhpParser_Lexer_TokenEmulator_VoidCastEmulator(crate::php_parser::lexer::token_emulator::VoidCastEmulator),
}
impl php_rt::PhpObject for TokenEmulator {
    fn class_name(&self) -> &'static str { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => __h.class_name(), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => __h.class_name(), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.class_name(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => __h.class_name(), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.class_name(), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.class_name(), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => __h.class_name(), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => __h.class_name(), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.class_name(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.class_name(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.class_name(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => __h.class_name(), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => __h.class_name(), _ => unreachable!() } }
    fn class_ancestors(&self) -> &'static [&'static str] { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => __h.class_ancestors(), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => __h.class_ancestors(), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.class_ancestors(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => __h.class_ancestors(), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.class_ancestors(), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.class_ancestors(), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => __h.class_ancestors(), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => __h.class_ancestors(), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.class_ancestors(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.class_ancestors(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.class_ancestors(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => __h.class_ancestors(), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => __h.class_ancestors(), _ => unreachable!() } }
    fn obj_id(&self) -> usize { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => __h.obj_id(), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => __h.obj_id(), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.obj_id(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => __h.obj_id(), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.obj_id(), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.obj_id(), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => __h.obj_id(), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => __h.obj_id(), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.obj_id(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.obj_id(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.obj_id(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => __h.obj_id(), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => __h.obj_id(), _ => unreachable!() } }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => __h.props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => __h.props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => __h.props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => __h.props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => __h.props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => __h.props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => __h.props(), _ => unreachable!() } }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => __h.set_prop(name, value), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => __h.set_prop(name, value), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.set_prop(name, value), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => __h.set_prop(name, value), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.set_prop(name, value), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.set_prop(name, value), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => __h.set_prop(name, value), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => __h.set_prop(name, value), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.set_prop(name, value), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.set_prop(name, value), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.set_prop(name, value), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => __h.set_prop(name, value), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => __h.set_prop(name, value), _ => unreachable!() } }
    fn php_to_string(&self) -> Option<Str> { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => __h.php_to_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => __h.php_to_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.php_to_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => __h.php_to_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.php_to_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.php_to_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => __h.php_to_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => __h.php_to_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.php_to_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.php_to_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.php_to_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => __h.php_to_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => __h.php_to_string(), _ => unreachable!() } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => __h.call_method(name, args), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => __h.call_method(name, args), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.call_method(name, args), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => __h.call_method(name, args), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.call_method(name, args), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.call_method(name, args), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => __h.call_method(name, args), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => __h.call_method(name, args), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.call_method(name, args), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.call_method(name, args), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.call_method(name, args), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => __h.call_method(name, args), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => __h.call_method(name, args), _ => unreachable!() } }
    fn public_props(&self) -> Vec<(Str, Mixed)> { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => __h.public_props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => __h.public_props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.public_props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => __h.public_props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.public_props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.public_props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => __h.public_props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => __h.public_props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.public_props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.public_props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.public_props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => __h.public_props(), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => __h.public_props(), _ => unreachable!() } }
}
impl TokenEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => __h.to_php_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => __h.to_php_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => __h.to_php_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => __h.to_php_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => __h.to_php_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => __h.to_php_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => __h.to_php_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => __h.to_php_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => __h.to_php_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => __h.to_php_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => __h.to_php_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => __h.to_php_string(), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => __h.to_php_string(), _ => unreachable!() } } }
impl php_rt::PhpClone for TokenEmulator { fn php_clone(&self) -> Self { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h.php_clone()), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h.php_clone()), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h.php_clone()), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h.php_clone()), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h.php_clone()), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h.php_clone()), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h.php_clone()), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h.php_clone()), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h.php_clone()), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h.php_clone()), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h.php_clone()), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h.php_clone()), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h.php_clone()), _ => unreachable!() } } }
impl TokenEmulator {
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => Ok(__h.preprocessCode(code, patches)?), _ => unreachable!() } }
    pub fn preprocessCode__impl(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> {
    return Ok(code.clone());
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => Ok(__h.getPhpVersion()?), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => Ok(__h.getPhpVersion()?), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(__h.getPhpVersion()?), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => Ok(__h.getPhpVersion()?), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(__h.getPhpVersion()?), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(__h.getPhpVersion()?), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => Ok(__h.getPhpVersion()?), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => Ok(__h.getPhpVersion()?), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(__h.getPhpVersion()?), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(__h.getPhpVersion()?), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(__h.getPhpVersion()?), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => Ok(__h.getPhpVersion()?), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => Ok(__h.getPhpVersion()?), _ => unreachable!() } }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => Ok(__h.isEmulationNeeded(code)?), _ => unreachable!() } }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>) -> Result<Map<ArrayKey, crate::php_parser::Token>, Throw> { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => Ok(__h.emulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => Ok(__h.emulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(__h.emulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => Ok(__h.emulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(__h.emulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(__h.emulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => Ok(__h.emulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => Ok(__h.emulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(__h.emulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(__h.emulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(__h.emulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => Ok(__h.emulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => Ok(__h.emulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), _ => unreachable!() } }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>) -> Result<Map<ArrayKey, crate::php_parser::Token>, Throw> { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => Ok(__h.reverseEmulate(code, tokens_v.map_values(|v| cast::<Mixed>(v)))?.map_values(|v| cast::<crate::php_parser::Token>(v))), _ => unreachable!() } }
    pub fn new_same_class(&self) -> Result<TokenEmulator, Throw> { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(__h.new_same_class()?)), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(__h.new_same_class()?)), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(__h.new_same_class()?)), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(__h.new_same_class()?)), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(__h.new_same_class()?)), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(__h.new_same_class()?)), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(__h.new_same_class()?)), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(__h.new_same_class()?)), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(__h.new_same_class()?)), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(__h.new_same_class()?)), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(__h.new_same_class()?)), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(__h.new_same_class(unreachable!("no default for crate::php_parser::lexer::token_emulator::TokenEmulator"))?)), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__h) => Ok(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(__h.new_same_class()?)), _ => unreachable!() } }
}
pub struct VoidCastEmulatorObj {
}
#[derive(Clone)]
pub struct VoidCastEmulator(pub Rc<RefCell<VoidCastEmulatorObj>>);
impl VoidCastEmulator {
    pub fn new() -> Result<VoidCastEmulator, Throw> {
        let this = VoidCastEmulator(Rc::new(RefCell::new(VoidCastEmulatorObj {
        })));
        Ok(this)
    }
    pub fn getPhpVersion(&self) -> Result<crate::php_parser::PhpVersion, Throw> {
    return Ok(crate::php_parser::PhpVersion::fromComponents(8i64, 5i64)?);
    }
    pub fn isEmulationNeeded(&self, mut code: Str) -> Result<bool, Throw> {
    return Ok(truthy(&preg_match(&Str::from_static("/\\([ \\t]*void[ \\t]*\\)/i"), &code.clone(), 0)?));
    }
    pub fn emulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    let mut i: i64 = Default::default();
    let mut c: i64 = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    let mut numTokens: i64 = Default::default();
    let mut text: Str = Default::default();
    let mut j: i64 = Default::default();
    let mut k: i64 = Default::default();
    {
        i = 0i64;
        c = tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).count();
        'l1: loop {
            if !((i < c)) { break; }
            'c2: {
                token.set(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&i)).clone());
                if (!identical(&token.get().clone().p_text_get(), &Str::from_static("("))) {
                    { break 'c2 };
                }
                numTokens = 1i64;
                text = Str::from_static("(");
                j = (i).wrapping_add(1i64);
                if (((j < c) && (tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&j)).clone().p_id_get() == 397i64)) && truthy(&preg_match(&Str::from_static("/[ \\t]+/"), &tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&j)).clone().p_text_get(), 0)?)) {
                    { let __t1 = tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&j)).clone().p_text_get(); append(&mut text, __t1); }
                    let _ = { let __t2 = numTokens; numTokens = __t2.wrapping_add(1); __t2 };
                    let _ = { let __t3 = j; j = __t3.wrapping_add(1); __t3 };
                }
                if (((j >= c) || (!(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&j)).clone().p_id_get() == 262i64))) || (!identical(&strtolower(&tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&j)).clone().p_text_get()), &Str::from_static("void")))) {
                    { break 'c2 };
                }
                { let __t4 = tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&j)).clone().p_text_get(); append(&mut text, __t4); }
                let _ = { let __t5 = numTokens; numTokens = __t5.wrapping_add(1); __t5 };
                k = (j).wrapping_add(1i64);
                if (((k < c) && (tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&k)).clone().p_id_get() == 397i64)) && truthy(&preg_match(&Str::from_static("/[ \\t]+/"), &tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&k)).clone().p_text_get(), 0)?)) {
                    { let __t6 = tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&k)).clone().p_text_get(); append(&mut text, __t6); }
                    let _ = { let __t7 = numTokens; numTokens = __t7.wrapping_add(1); __t7 };
                    let _ = { let __t8 = k; k = __t8.wrapping_add(1); __t8 };
                }
                if ((k >= c) || (!identical(&tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&k)).clone().p_text_get(), &Str::from_static(")")))) {
                    { break 'c2 };
                }
                { let __t9 = Str::from_static(")"); append(&mut text, __t9); }
                let _ = { let __t10 = numTokens; numTokens = __t10.wrapping_add(1); __t10 };
                let _: List<Mixed> = { let __off = i; let __len = Some(numTokens); let __repl = { let __c29 = (crate::php_parser::Token::new(388i64, text.clone(), token.get().clone().p_line_get(), token.get().clone().p_pos_get())?,); List::from_vec(vec![cast::<Mixed>(__c29.0)]) }.into_vec(); let __r = array_splice_m(&mut tokens_v, __off, __len, __repl); __r };
                { let __t11 = (numTokens).wrapping_sub(1i64); c = (c).wrapping_sub(__t11); }
            }
            let _ = { let __t12 = i; i = __t12.wrapping_add(1); __t12.wrapping_add(1) };
        }
    }
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn reverseEmulate(&self, mut code: Str, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    let mut i: i64 = Default::default();
    let mut c: i64 = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    let mut match_: Map<ArrayKey, Str> = Default::default();
    let mut newTokens: List<crate::php_parser::Token> = Default::default();
    let mut pos: i64 = Default::default();
    {
        i = 0i64;
        c = tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).count();
        'l1: loop {
            if !((i < c)) { break; }
            'c2: {
                token.set(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).idx(&to_key(&i)).clone());
                if (!(token.get().clone().p_id_get() == 388i64)) {
                    { break 'c2 };
                }
                if (!truthy(&{ let (__r, __m) = preg_match_groups(&Str::from_static("/^\\(([ \\t]*)(void)([ \\t]*)\\)$/i"), &token.get().clone().p_text_get(), 0)?; match_ = __m; __r })) {
                    return Err(cast::<crate::g::Throwable>(crate::g::LogicException::new(Str::from_static("Unexpected T_VOID_CAST contents"), 0i64, { let _ = (); None::<crate::g::Throwable> })?));
                }
                newTokens = List::<crate::php_parser::Token>::new();
                pos = token.get().clone().p_pos_get();
                { let __h1 = crate::php_parser::Token::new(ord(&Str::from_static("(")), Str::from_static("("), token.get().clone().p_line_get(), pos)?; newTokens.push(__h1); }
                let _ = { let __t2 = pos; pos = __t2.wrapping_add(1); __t2 };
                if (!identical(&match_.clone().idx(&to_key(&1i64)).clone(), &Str::from_static(""))) {
                    { let __h3 = crate::php_parser::Token::new(397i64, match_.clone().idx(&to_key(&1i64)).clone(), token.get().clone().p_line_get(), pos)?; newTokens.push(__h3); }
                    { let __t4 = strlen(&match_.clone().idx(&to_key(&1i64)).clone()); pos = (pos).wrapping_add(__t4); }
                }
                { let __h5 = crate::php_parser::Token::new(262i64, match_.clone().idx(&to_key(&2i64)).clone(), token.get().clone().p_line_get(), pos)?; newTokens.push(__h5); }
                { let __t6 = strlen(&match_.clone().idx(&to_key(&2i64)).clone()); pos = (pos).wrapping_add(__t6); }
                if (!identical(&match_.clone().idx(&to_key(&3i64)).clone(), &Str::from_static(""))) {
                    { let __h7 = crate::php_parser::Token::new(397i64, match_.clone().idx(&to_key(&3i64)).clone(), token.get().clone().p_line_get(), pos)?; newTokens.push(__h7); }
                    { let __t8 = strlen(&match_.clone().idx(&to_key(&3i64)).clone()); pos = (pos).wrapping_add(__t8); }
                }
                { let __h9 = crate::php_parser::Token::new(ord(&Str::from_static(")")), Str::from_static(")"), token.get().clone().p_line_get(), pos)?; newTokens.push(__h9); }
                let _: List<Mixed> = { let __off = i; let __len = Some(1i64); let __repl = newTokens.clone().map_elems(|v| cast::<Mixed>(v)).into_vec(); let __r = array_splice_m(&mut tokens_v, __off, __len, __repl); __r };
                { let __t10 = (newTokens.clone().count()).wrapping_sub(1i64); i = (i).wrapping_add(__t10); }
                { let __t11 = (newTokens.clone().count()).wrapping_sub(1i64); c = (c).wrapping_add(__t11); }
            }
            let _ = { let __t12 = i; i = __t12.wrapping_add(1); __t12.wrapping_add(1) };
        }
    }
    return Ok(tokens_v.clone().map_values(|v| cast::<crate::php_parser::Token>(v)).map_values(|v| cast::<Mixed>(v)));
    }
    pub fn preprocessCode(&self, mut code: Str, mut patches: &mut Map<ArrayKey, (i64, Str, Str)>) -> Result<Str, Throw> { cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(self.clone()).preprocessCode__impl(code, patches) }
    pub fn new_same_class(&self) -> Result<crate::php_parser::lexer::token_emulator::VoidCastEmulator, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for VoidCastEmulator {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\TokenEmulator\\VoidCastEmulator" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\tokenemulator\\voidcastemulator", "phpparser\\lexer\\tokenemulator\\tokenemulator"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "getphpversion" => { let __r = self.getPhpVersion().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "isemulationneeded" => { let __r = self.isEmulationNeeded((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "emulate" => { let __r = self.emulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "reverseemulate" => { let __r = self.reverseEmulate((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Lexer\\TokenEmulator\\VoidCastEmulator", name)))) } }
}
impl VoidCastEmulator { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\TokenEmulator\\VoidCastEmulator could not be converted to string"))) } }
impl php_rt::PhpClone for VoidCastEmulator { fn php_clone(&self) -> Self { let c = VoidCastEmulator(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for VoidCastEmulatorObj { fn clone(&self) -> Self { VoidCastEmulatorObj {  } } }
impl VoidCastEmulator {
}
impl php_rt::Truthy for AsymmetricVisibilityTokenEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for AsymmetricVisibilityTokenEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\AsymmetricVisibilityTokenEmulator")) } }
impl php_rt::Identical for AsymmetricVisibilityTokenEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for AsymmetricVisibilityTokenEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for AsymmetricVisibilityTokenEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for AsymmetricVisibilityTokenEmulator { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<AsymmetricVisibilityTokenEmulator> for Mixed { fn cast_to(self) -> AsymmetricVisibilityTokenEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::AsymmetricVisibilityTokenEmulator>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\AsymmetricVisibilityTokenEmulator") } }
impl php_rt::TryDowncast for AsymmetricVisibilityTokenEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::AsymmetricVisibilityTokenEmulator>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for AsymmetricVisibilityTokenEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<AsymmetricVisibilityTokenEmulator> for AnyObject { fn cast_to(self) -> AsymmetricVisibilityTokenEmulator { cast::<AsymmetricVisibilityTokenEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<AsymmetricVisibilityTokenEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\asymmetricvisibilitytokenemulator") } }
impl php_rt::InstanceOf<AsymmetricVisibilityTokenEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\asymmetricvisibilitytokenemulator") } }
impl php_rt::InstanceOf<AsymmetricVisibilityTokenEmulator> for AsymmetricVisibilityTokenEmulator { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for AttributeEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for AttributeEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\AttributeEmulator")) } }
impl php_rt::Identical for AttributeEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for AttributeEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for AttributeEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for AttributeEmulator { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<AttributeEmulator> for Mixed { fn cast_to(self) -> AttributeEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::AttributeEmulator>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\AttributeEmulator") } }
impl php_rt::TryDowncast for AttributeEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::AttributeEmulator>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for AttributeEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<AttributeEmulator> for AnyObject { fn cast_to(self) -> AttributeEmulator { cast::<AttributeEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<AttributeEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\attributeemulator") } }
impl php_rt::InstanceOf<AttributeEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\attributeemulator") } }
impl php_rt::InstanceOf<AttributeEmulator> for AttributeEmulator { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for EnumTokenEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for EnumTokenEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\EnumTokenEmulator")) } }
impl php_rt::Identical for EnumTokenEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for EnumTokenEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for EnumTokenEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for EnumTokenEmulator { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<EnumTokenEmulator> for Mixed { fn cast_to(self) -> EnumTokenEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::EnumTokenEmulator>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\EnumTokenEmulator") } }
impl php_rt::TryDowncast for EnumTokenEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::EnumTokenEmulator>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for EnumTokenEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<EnumTokenEmulator> for AnyObject { fn cast_to(self) -> EnumTokenEmulator { cast::<EnumTokenEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<EnumTokenEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\enumtokenemulator") } }
impl php_rt::InstanceOf<EnumTokenEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\enumtokenemulator") } }
impl php_rt::InstanceOf<EnumTokenEmulator> for EnumTokenEmulator { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for ExplicitOctalEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for ExplicitOctalEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\ExplicitOctalEmulator")) } }
impl php_rt::Identical for ExplicitOctalEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for ExplicitOctalEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for ExplicitOctalEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for ExplicitOctalEmulator { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<ExplicitOctalEmulator> for Mixed { fn cast_to(self) -> ExplicitOctalEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ExplicitOctalEmulator>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\ExplicitOctalEmulator") } }
impl php_rt::TryDowncast for ExplicitOctalEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ExplicitOctalEmulator>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for ExplicitOctalEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<ExplicitOctalEmulator> for AnyObject { fn cast_to(self) -> ExplicitOctalEmulator { cast::<ExplicitOctalEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<ExplicitOctalEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\explicitoctalemulator") } }
impl php_rt::InstanceOf<ExplicitOctalEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\explicitoctalemulator") } }
impl php_rt::InstanceOf<ExplicitOctalEmulator> for ExplicitOctalEmulator { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for FnTokenEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for FnTokenEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\FnTokenEmulator")) } }
impl php_rt::Identical for FnTokenEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for FnTokenEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for FnTokenEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for FnTokenEmulator { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<FnTokenEmulator> for Mixed { fn cast_to(self) -> FnTokenEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::FnTokenEmulator>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\FnTokenEmulator") } }
impl php_rt::TryDowncast for FnTokenEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::FnTokenEmulator>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for FnTokenEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<FnTokenEmulator> for AnyObject { fn cast_to(self) -> FnTokenEmulator { cast::<FnTokenEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<FnTokenEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\fntokenemulator") } }
impl php_rt::InstanceOf<FnTokenEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\fntokenemulator") } }
impl php_rt::InstanceOf<FnTokenEmulator> for FnTokenEmulator { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for KeywordEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for KeywordEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\KeywordEmulator")) } }
impl php_rt::Identical for KeywordEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for KeywordEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for KeywordEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for KeywordEmulator { fn cast_to(self) -> Mixed { match self { KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(v) => Mixed::Obj(Rc::new(v)), KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(v) => Mixed::Obj(Rc::new(v)), KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(v) => Mixed::Obj(Rc::new(v)), KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(v) => Mixed::Obj(Rc::new(v)), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(v) => Mixed::Obj(Rc::new(v)), KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(v) => Mixed::Obj(Rc::new(v)), _ => unreachable!() } } }
impl php_rt::CastTo<KeywordEmulator> for Mixed { fn cast_to(self) -> KeywordEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::EnumTokenEmulator>() { return crate::php_parser::lexer::token_emulator::KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::FnTokenEmulator>() { return crate::php_parser::lexer::token_emulator::KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::MatchTokenEmulator>() { return crate::php_parser::lexer::token_emulator::KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::PropertyTokenEmulator>() { return crate::php_parser::lexer::token_emulator::KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReadonlyFunctionTokenEmulator>() { return crate::php_parser::lexer::token_emulator::KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReadonlyTokenEmulator>() { return crate::php_parser::lexer::token_emulator::KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(v.clone()); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\KeywordEmulator") } }
impl php_rt::TryDowncast for KeywordEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::EnumTokenEmulator>() { return Some(crate::php_parser::lexer::token_emulator::KeywordEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::FnTokenEmulator>() { return Some(crate::php_parser::lexer::token_emulator::KeywordEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::MatchTokenEmulator>() { return Some(crate::php_parser::lexer::token_emulator::KeywordEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::PropertyTokenEmulator>() { return Some(crate::php_parser::lexer::token_emulator::KeywordEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReadonlyFunctionTokenEmulator>() { return Some(crate::php_parser::lexer::token_emulator::KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReadonlyTokenEmulator>() { return Some(crate::php_parser::lexer::token_emulator::KeywordEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(v.clone())); } None } }
impl php_rt::CastTo<AnyObject> for KeywordEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<KeywordEmulator> for AnyObject { fn cast_to(self) -> KeywordEmulator { cast::<KeywordEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<KeywordEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\keywordemulator") } }
impl php_rt::InstanceOf<KeywordEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\keywordemulator") } }
impl php_rt::InstanceOf<KeywordEmulator> for KeywordEmulator { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for MatchTokenEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for MatchTokenEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\MatchTokenEmulator")) } }
impl php_rt::Identical for MatchTokenEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for MatchTokenEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for MatchTokenEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for MatchTokenEmulator { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<MatchTokenEmulator> for Mixed { fn cast_to(self) -> MatchTokenEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::MatchTokenEmulator>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\MatchTokenEmulator") } }
impl php_rt::TryDowncast for MatchTokenEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::MatchTokenEmulator>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for MatchTokenEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<MatchTokenEmulator> for AnyObject { fn cast_to(self) -> MatchTokenEmulator { cast::<MatchTokenEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<MatchTokenEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\matchtokenemulator") } }
impl php_rt::InstanceOf<MatchTokenEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\matchtokenemulator") } }
impl php_rt::InstanceOf<MatchTokenEmulator> for MatchTokenEmulator { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for NullsafeTokenEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for NullsafeTokenEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\NullsafeTokenEmulator")) } }
impl php_rt::Identical for NullsafeTokenEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for NullsafeTokenEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for NullsafeTokenEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for NullsafeTokenEmulator { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<NullsafeTokenEmulator> for Mixed { fn cast_to(self) -> NullsafeTokenEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::NullsafeTokenEmulator>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\NullsafeTokenEmulator") } }
impl php_rt::TryDowncast for NullsafeTokenEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::NullsafeTokenEmulator>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for NullsafeTokenEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<NullsafeTokenEmulator> for AnyObject { fn cast_to(self) -> NullsafeTokenEmulator { cast::<NullsafeTokenEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<NullsafeTokenEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\nullsafetokenemulator") } }
impl php_rt::InstanceOf<NullsafeTokenEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\nullsafetokenemulator") } }
impl php_rt::InstanceOf<NullsafeTokenEmulator> for NullsafeTokenEmulator { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for PipeOperatorEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for PipeOperatorEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\PipeOperatorEmulator")) } }
impl php_rt::Identical for PipeOperatorEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for PipeOperatorEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for PipeOperatorEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for PipeOperatorEmulator { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<PipeOperatorEmulator> for Mixed { fn cast_to(self) -> PipeOperatorEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::PipeOperatorEmulator>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\PipeOperatorEmulator") } }
impl php_rt::TryDowncast for PipeOperatorEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::PipeOperatorEmulator>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for PipeOperatorEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<PipeOperatorEmulator> for AnyObject { fn cast_to(self) -> PipeOperatorEmulator { cast::<PipeOperatorEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<PipeOperatorEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\pipeoperatoremulator") } }
impl php_rt::InstanceOf<PipeOperatorEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\pipeoperatoremulator") } }
impl php_rt::InstanceOf<PipeOperatorEmulator> for PipeOperatorEmulator { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for PropertyTokenEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for PropertyTokenEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\PropertyTokenEmulator")) } }
impl php_rt::Identical for PropertyTokenEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for PropertyTokenEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for PropertyTokenEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for PropertyTokenEmulator { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<PropertyTokenEmulator> for Mixed { fn cast_to(self) -> PropertyTokenEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::PropertyTokenEmulator>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\PropertyTokenEmulator") } }
impl php_rt::TryDowncast for PropertyTokenEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::PropertyTokenEmulator>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for PropertyTokenEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<PropertyTokenEmulator> for AnyObject { fn cast_to(self) -> PropertyTokenEmulator { cast::<PropertyTokenEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<PropertyTokenEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\propertytokenemulator") } }
impl php_rt::InstanceOf<PropertyTokenEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\propertytokenemulator") } }
impl php_rt::InstanceOf<PropertyTokenEmulator> for PropertyTokenEmulator { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for ReadonlyFunctionTokenEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for ReadonlyFunctionTokenEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\ReadonlyFunctionTokenEmulator")) } }
impl php_rt::Identical for ReadonlyFunctionTokenEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for ReadonlyFunctionTokenEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for ReadonlyFunctionTokenEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for ReadonlyFunctionTokenEmulator { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<ReadonlyFunctionTokenEmulator> for Mixed { fn cast_to(self) -> ReadonlyFunctionTokenEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReadonlyFunctionTokenEmulator>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\ReadonlyFunctionTokenEmulator") } }
impl php_rt::TryDowncast for ReadonlyFunctionTokenEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReadonlyFunctionTokenEmulator>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for ReadonlyFunctionTokenEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<ReadonlyFunctionTokenEmulator> for AnyObject { fn cast_to(self) -> ReadonlyFunctionTokenEmulator { cast::<ReadonlyFunctionTokenEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<ReadonlyFunctionTokenEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\readonlyfunctiontokenemulator") } }
impl php_rt::InstanceOf<ReadonlyFunctionTokenEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\readonlyfunctiontokenemulator") } }
impl php_rt::InstanceOf<ReadonlyFunctionTokenEmulator> for ReadonlyFunctionTokenEmulator { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for ReadonlyTokenEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for ReadonlyTokenEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\ReadonlyTokenEmulator")) } }
impl php_rt::Identical for ReadonlyTokenEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for ReadonlyTokenEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for ReadonlyTokenEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for ReadonlyTokenEmulator { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<ReadonlyTokenEmulator> for Mixed { fn cast_to(self) -> ReadonlyTokenEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReadonlyTokenEmulator>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\ReadonlyTokenEmulator") } }
impl php_rt::TryDowncast for ReadonlyTokenEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReadonlyTokenEmulator>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for ReadonlyTokenEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<ReadonlyTokenEmulator> for AnyObject { fn cast_to(self) -> ReadonlyTokenEmulator { cast::<ReadonlyTokenEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<ReadonlyTokenEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\readonlytokenemulator") } }
impl php_rt::InstanceOf<ReadonlyTokenEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\readonlytokenemulator") } }
impl php_rt::InstanceOf<ReadonlyTokenEmulator> for ReadonlyTokenEmulator { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for ReverseEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for ReverseEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\ReverseEmulator")) } }
impl php_rt::Identical for ReverseEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for ReverseEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for ReverseEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for ReverseEmulator { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<ReverseEmulator> for Mixed { fn cast_to(self) -> ReverseEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReverseEmulator>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\ReverseEmulator") } }
impl php_rt::TryDowncast for ReverseEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReverseEmulator>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for ReverseEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<ReverseEmulator> for AnyObject { fn cast_to(self) -> ReverseEmulator { cast::<ReverseEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<ReverseEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\reverseemulator") } }
impl php_rt::InstanceOf<ReverseEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\reverseemulator") } }
impl php_rt::InstanceOf<ReverseEmulator> for ReverseEmulator { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for TokenEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for TokenEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\TokenEmulator")) } }
impl php_rt::Identical for TokenEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for TokenEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for TokenEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for TokenEmulator { fn cast_to(self) -> Mixed { match self { TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(v) => Mixed::Obj(Rc::new(v)), TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(v) => Mixed::Obj(Rc::new(v)), TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(v) => Mixed::Obj(Rc::new(v)), TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(v) => Mixed::Obj(Rc::new(v)), TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(v) => Mixed::Obj(Rc::new(v)), TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(v) => Mixed::Obj(Rc::new(v)), TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(v) => Mixed::Obj(Rc::new(v)), TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(v) => Mixed::Obj(Rc::new(v)), TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(v) => Mixed::Obj(Rc::new(v)), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(v) => Mixed::Obj(Rc::new(v)), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(v) => Mixed::Obj(Rc::new(v)), TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(v) => Mixed::Obj(Rc::new(v)), TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(v) => Mixed::Obj(Rc::new(v)), _ => unreachable!() } } }
impl php_rt::CastTo<TokenEmulator> for Mixed { fn cast_to(self) -> TokenEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::AsymmetricVisibilityTokenEmulator>() { return crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::AttributeEmulator>() { return crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::EnumTokenEmulator>() { return crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ExplicitOctalEmulator>() { return crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::FnTokenEmulator>() { return crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::MatchTokenEmulator>() { return crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::NullsafeTokenEmulator>() { return crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::PipeOperatorEmulator>() { return crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::PropertyTokenEmulator>() { return crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReadonlyFunctionTokenEmulator>() { return crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReadonlyTokenEmulator>() { return crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReverseEmulator>() { return crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(v.clone()); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::VoidCastEmulator>() { return crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(v.clone()); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\TokenEmulator") } }
impl php_rt::TryDowncast for TokenEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::AsymmetricVisibilityTokenEmulator>() { return Some(crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::AttributeEmulator>() { return Some(crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_AttributeEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::EnumTokenEmulator>() { return Some(crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ExplicitOctalEmulator>() { return Some(crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::FnTokenEmulator>() { return Some(crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::MatchTokenEmulator>() { return Some(crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::NullsafeTokenEmulator>() { return Some(crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::PipeOperatorEmulator>() { return Some(crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::PropertyTokenEmulator>() { return Some(crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReadonlyFunctionTokenEmulator>() { return Some(crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReadonlyTokenEmulator>() { return Some(crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::ReverseEmulator>() { return Some(crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_ReverseEmulator(v.clone())); } if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::VoidCastEmulator>() { return Some(crate::php_parser::lexer::token_emulator::TokenEmulator::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(v.clone())); } None } }
impl php_rt::CastTo<AnyObject> for TokenEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<TokenEmulator> for AnyObject { fn cast_to(self) -> TokenEmulator { cast::<TokenEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<TokenEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\tokenemulator") } }
impl php_rt::InstanceOf<TokenEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\tokenemulator") } }
impl php_rt::InstanceOf<TokenEmulator> for TokenEmulator { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for VoidCastEmulator { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for VoidCastEmulator { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\TokenEmulator\\VoidCastEmulator")) } }
impl php_rt::Identical for VoidCastEmulator { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for VoidCastEmulator { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for VoidCastEmulator { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for VoidCastEmulator { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<VoidCastEmulator> for Mixed { fn cast_to(self) -> VoidCastEmulator { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::VoidCastEmulator>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\TokenEmulator\\VoidCastEmulator") } }
impl php_rt::TryDowncast for VoidCastEmulator { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::token_emulator::VoidCastEmulator>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for VoidCastEmulator { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<VoidCastEmulator> for AnyObject { fn cast_to(self) -> VoidCastEmulator { cast::<VoidCastEmulator>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<VoidCastEmulator> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\tokenemulator\\voidcastemulator") } }
impl php_rt::InstanceOf<VoidCastEmulator> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\tokenemulator\\voidcastemulator") } }
impl php_rt::InstanceOf<VoidCastEmulator> for VoidCastEmulator { fn is_instance(&self) -> bool { true } }
