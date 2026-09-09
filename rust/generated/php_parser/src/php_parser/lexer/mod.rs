use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub mod token_emulator;
pub struct EmulativeObj {
    pub patches: Map<ArrayKey, (i64, Str, Str)>,
    pub emulators: List<crate::php_parser::lexer::token_emulator::TokenEmulator>,
    pub targetPhpVersion: Late<crate::php_parser::PhpVersion>,
    pub hostPhpVersion: Late<crate::php_parser::PhpVersion>,
}
#[derive(Clone)]
pub struct Emulative(pub Rc<RefCell<EmulativeObj>>);
impl Emulative {
    pub fn p_patches(&self) -> Ref<'_, Map<ArrayKey, (i64, Str, Str)>> { Ref::map(self.0.borrow(), |o| &o.patches) }
    pub fn p_patches_get(&self) -> Map<ArrayKey, (i64, Str, Str)> { self.0.borrow().patches.clone() }
    pub fn p_patches_opt(&self) -> Option<Map<ArrayKey, (i64, Str, Str)>> { Some(self.0.borrow().patches.clone()) }
    pub fn p_patches_mut(&self) -> RefMut<'_, Map<ArrayKey, (i64, Str, Str)>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.patches) }
    pub fn set_p_patches(&self, v: Map<ArrayKey, (i64, Str, Str)>) { self.0.borrow_mut().patches = v; }
    pub fn p_emulators(&self) -> Ref<'_, List<crate::php_parser::lexer::token_emulator::TokenEmulator>> { Ref::map(self.0.borrow(), |o| &o.emulators) }
    pub fn p_emulators_get(&self) -> List<crate::php_parser::lexer::token_emulator::TokenEmulator> { self.0.borrow().emulators.clone() }
    pub fn p_emulators_opt(&self) -> Option<List<crate::php_parser::lexer::token_emulator::TokenEmulator>> { Some(self.0.borrow().emulators.clone()) }
    pub fn p_emulators_mut(&self) -> RefMut<'_, List<crate::php_parser::lexer::token_emulator::TokenEmulator>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.emulators) }
    pub fn set_p_emulators(&self, v: List<crate::php_parser::lexer::token_emulator::TokenEmulator>) { self.0.borrow_mut().emulators = v; }
    pub fn p_targetPhpVersion(&self) -> Ref<'_, crate::php_parser::PhpVersion> { Ref::map(self.0.borrow(), |o| o.targetPhpVersion.get()) }
    pub fn p_targetPhpVersion_get(&self) -> crate::php_parser::PhpVersion { self.0.borrow().targetPhpVersion.get().clone() }
    pub fn p_targetPhpVersion_opt(&self) -> Option<crate::php_parser::PhpVersion> { self.0.borrow().targetPhpVersion.as_option().cloned() }
    pub fn p_targetPhpVersion_mut(&self) -> RefMut<'_, crate::php_parser::PhpVersion> { RefMut::map(self.0.borrow_mut(), |o| o.targetPhpVersion.get_mut()) }
    pub fn set_p_targetPhpVersion(&self, v: crate::php_parser::PhpVersion) { self.0.borrow_mut().targetPhpVersion.set(v); }
    pub fn p_hostPhpVersion(&self) -> Ref<'_, crate::php_parser::PhpVersion> { Ref::map(self.0.borrow(), |o| o.hostPhpVersion.get()) }
    pub fn p_hostPhpVersion_get(&self) -> crate::php_parser::PhpVersion { self.0.borrow().hostPhpVersion.get().clone() }
    pub fn p_hostPhpVersion_opt(&self) -> Option<crate::php_parser::PhpVersion> { self.0.borrow().hostPhpVersion.as_option().cloned() }
    pub fn p_hostPhpVersion_mut(&self) -> RefMut<'_, crate::php_parser::PhpVersion> { RefMut::map(self.0.borrow_mut(), |o| o.hostPhpVersion.get_mut()) }
    pub fn set_p_hostPhpVersion(&self, v: crate::php_parser::PhpVersion) { self.0.borrow_mut().hostPhpVersion.set(v); }
    pub fn new(mut phpVersion: Option<crate::php_parser::PhpVersion>) -> Result<Emulative, Throw> {
        let this = Emulative(Rc::new(RefCell::new(EmulativeObj {
            patches: Map::<ArrayKey, (i64, Str, Str)>::new(),
            emulators: List::<crate::php_parser::lexer::token_emulator::TokenEmulator>::new(),
            targetPhpVersion: Late::uninit(),
            hostPhpVersion: Late::uninit(),
        })));
        this.magic__construct(phpVersion)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut phpVersion: Option<crate::php_parser::PhpVersion>) -> Result<Mixed, Throw> {
    let mut emulators: Late<(crate::php_parser::lexer::token_emulator::FnTokenEmulator, crate::php_parser::lexer::token_emulator::MatchTokenEmulator, crate::php_parser::lexer::token_emulator::NullsafeTokenEmulator, crate::php_parser::lexer::token_emulator::AttributeEmulator, crate::php_parser::lexer::token_emulator::EnumTokenEmulator, crate::php_parser::lexer::token_emulator::ReadonlyTokenEmulator, crate::php_parser::lexer::token_emulator::ExplicitOctalEmulator, crate::php_parser::lexer::token_emulator::ReadonlyFunctionTokenEmulator, crate::php_parser::lexer::token_emulator::PropertyTokenEmulator, crate::php_parser::lexer::token_emulator::AsymmetricVisibilityTokenEmulator, crate::php_parser::lexer::token_emulator::PipeOperatorEmulator, crate::php_parser::lexer::token_emulator::VoidCastEmulator)> = Late::uninit();
    let mut emulator: Late<U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a> = Late::uninit();
    let mut emulatorPhpVersion: Late<crate::php_parser::PhpVersion> = Late::uninit();
    self.set_p_targetPhpVersion((match phpVersion.clone() { Some(__v) => __v, None => crate::php_parser::PhpVersion::getNewestSupported()? }));
    self.set_p_hostPhpVersion(crate::php_parser::PhpVersion::getHostVersion()?);
    emulators.set((crate::php_parser::lexer::token_emulator::FnTokenEmulator::new()?, crate::php_parser::lexer::token_emulator::MatchTokenEmulator::new()?, crate::php_parser::lexer::token_emulator::NullsafeTokenEmulator::new()?, crate::php_parser::lexer::token_emulator::AttributeEmulator::new()?, crate::php_parser::lexer::token_emulator::EnumTokenEmulator::new()?, crate::php_parser::lexer::token_emulator::ReadonlyTokenEmulator::new()?, crate::php_parser::lexer::token_emulator::ExplicitOctalEmulator::new()?, crate::php_parser::lexer::token_emulator::ReadonlyFunctionTokenEmulator::new()?, crate::php_parser::lexer::token_emulator::PropertyTokenEmulator::new()?, crate::php_parser::lexer::token_emulator::AsymmetricVisibilityTokenEmulator::new()?, crate::php_parser::lexer::token_emulator::PipeOperatorEmulator::new()?, crate::php_parser::lexer::token_emulator::VoidCastEmulator::new()?));
    'l1: for __kv1 in { let __c13 = emulators.get().clone(); List::from_vec(vec![U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__c13.0), U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__c13.1), U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__c13.2), U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__c13.3), U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__c13.4), U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__c13.5), U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__c13.6), U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__c13.7), U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__c13.8), U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__c13.9), U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__c13.10), U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__c13.11)]) }.into_iter().enumerate().map(|(__i, __v)| (__i as i64, __v)) {
        emulator.set(__kv1.1);
        emulatorPhpVersion.set((match emulator.get().clone() { U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator(__o) => __o.getPhpVersion()?, U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_AttributeEmulator(__o) => __o.getPhpVersion()?, U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_EnumTokenEmulator(__o) => __o.getPhpVersion()?, U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_ExplicitOctalEmulator(__o) => __o.getPhpVersion()?, U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_FnTokenEmulator(__o) => __o.getPhpVersion()?, U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_MatchTokenEmulator(__o) => __o.getPhpVersion()?, U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_NullsafeTokenEmulator(__o) => __o.getPhpVersion()?, U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_PipeOperatorEmulator(__o) => __o.getPhpVersion()?, U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_PropertyTokenEmulator(__o) => __o.getPhpVersion()?, U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_ReadonlyFunctionTokenEmulator(__o) => __o.getPhpVersion()?, U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_ReadonlyTokenEmulator(__o) => __o.getPhpVersion()?, U_PhpParser_Lexer_TokenEmulator_AsymmetricVisibilityTokenEmulator_or_P_6d726a203a::PhpParser_Lexer_TokenEmulator_VoidCastEmulator(__o) => __o.getPhpVersion()?, _ => unreachable!() }));
        if self.isForwardEmulationNeeded(emulatorPhpVersion.get().clone())? {
            (*self.p_emulators_mut()).push(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(emulator.get().clone()));
        } else if self.isReverseEmulationNeeded(emulatorPhpVersion.get().clone())? {
            (*self.p_emulators_mut()).push(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(crate::php_parser::lexer::token_emulator::ReverseEmulator::new(cast::<crate::php_parser::lexer::token_emulator::TokenEmulator>(emulator.get().clone()))?));
        }
    }
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn tokenize(&self, mut code: Str, mut errorHandler: Option<crate::php_parser::ErrorHandler>) -> Result<Map<ArrayKey, Mixed>, Throw> {
    let mut emulators: Map<i64, crate::php_parser::lexer::token_emulator::TokenEmulator> = Default::default();
    let mut emulator: Late<crate::php_parser::lexer::token_emulator::TokenEmulator> = Late::uninit();
    let mut collector: Late<crate::php_parser::error_handler::Collecting> = Late::uninit();
    let mut tokens_v: Map<ArrayKey, crate::php_parser::Token> = Default::default();
    let mut errors: Map<ArrayKey, crate::php_parser::Error> = Default::default();
    let mut error_v: Late<crate::php_parser::Error> = Late::uninit();
    emulators = array_filter_cb_l(&self.p_emulators_get(), { let __f = { let __c14 = { let code = code.clone(); let this = self.clone(); Rc::new(move |mut emulator: crate::php_parser::lexer::token_emulator::TokenEmulator| -> Result<Mixed, Throw> { let mut code = code.clone(); 
    let mut code: Str = Default::default();
    return Ok(cast::<Mixed>(emulator.clone().isEmulationNeeded(code.clone())?));
    }) as Rc<dyn Fn(crate::php_parser::lexer::token_emulator::TokenEmulator) -> Result<Mixed, Throw>> }; Rc::new(move |__p0: crate::php_parser::lexer::token_emulator::TokenEmulator| -> Result<bool, Throw> { Ok(cast::<bool>(__c14(__p0)?)) }) as Rc<dyn Fn(crate::php_parser::lexer::token_emulator::TokenEmulator) -> Result<bool, Throw>> }; move |__a0| __f(__a0) })?;
    if (!truthy(&emulators.clone())) {
        return Ok(cast::<crate::php_parser::Lexer>(self.clone()).tokenize__impl(code.clone(), errorHandler.clone())?.map_values(|v| cast::<Mixed>(v)));
    }
    if errorHandler.clone().is_none() {
        errorHandler = Some(cast::<crate::php_parser::ErrorHandler>(crate::php_parser::error_handler::Throwing::new()?));
    }
    self.set_p_patches(Map::<ArrayKey, (i64, Str, Str)>::new());
    'l1: for __kv1 in emulators.clone().into_iter() {
        emulator.set(__kv1.1);
        code = emulator.get().clone().preprocessCode(code.clone(), &mut (*self.p_patches_mut()))?;
    }
    collector.set(crate::php_parser::error_handler::Collecting::new()?);
    tokens_v = cast::<crate::php_parser::Lexer>(self.clone()).tokenize__impl(code.clone(), Some(cast::<crate::php_parser::ErrorHandler>(collector.get().clone())))?;
    self.sortPatches()?;
    tokens_v = cast::<Map<ArrayKey, crate::php_parser::Token>>(self.fixupTokens(cast::<List<crate::php_parser::Token>>(tokens_v.clone()))?);
    errors = collector.get().clone().getErrors()?;
    if (!(!truthy(&errors.clone()))) {
        self.fixupErrors(errors.clone())?;
        'l2: for __kv2 in errors.clone().into_iter() {
            error_v.set(__kv2.1);
            errorHandler.clone().unwrap().handleError(error_v.get().clone())?;
        }
    }
    'l3: for __kv3 in emulators.clone().into_iter() {
        emulator.set(__kv3.1);
        tokens_v = emulator.get().clone().emulate(code.clone(), tokens_v.clone())?;
    }
    return Ok(tokens_v.clone().map_values(|v| cast::<Mixed>(v)));
    }
    pub fn isForwardEmulationNeeded(&self, mut emulatorPhpVersion: crate::php_parser::PhpVersion) -> Result<bool, Throw> {
    return Ok((self.p_hostPhpVersion_get().older(emulatorPhpVersion.clone())? && self.p_targetPhpVersion_get().newerOrEqual(emulatorPhpVersion.clone())?));
    }
    pub fn isReverseEmulationNeeded(&self, mut emulatorPhpVersion: crate::php_parser::PhpVersion) -> Result<bool, Throw> {
    return Ok((self.p_hostPhpVersion_get().newerOrEqual(emulatorPhpVersion.clone())? && self.p_targetPhpVersion_get().older(emulatorPhpVersion.clone())?));
    }
    pub fn sortPatches(&self) -> Result<(), Throw> {
    let _: bool = { let __sorted = usort_m(&self.p_patches_get(), { let __f = { let __c15 = { let this = self.clone(); Rc::new(move |mut p1: Mixed, mut p2: Mixed| -> Result<Mixed, Throw> { 
    return Ok(cast::<Mixed>(spaceship(&cast::<Mixed>(mixed_get(&p1.clone(), &to_key(&0i64))), &cast::<Mixed>(mixed_get(&p2.clone(), &to_key(&0i64))))));
    }) as Rc<dyn Fn(Mixed, Mixed) -> Result<Mixed, Throw>> }; Rc::new(move |__p0: (i64, Str, Str), __p1: (i64, Str, Str)| -> Result<i64, Throw> { Ok(cast::<i64>(__c15({ let __c16 = __p0; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c16.0)); __m.push(cast::<Mixed>(__c16.1)); __m.push(cast::<Mixed>(__c16.2)); Mixed::Arr(__m) }, { let __c17 = __p1; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c17.0)); __m.push(cast::<Mixed>(__c17.1)); __m.push(cast::<Mixed>(__c17.2)); Mixed::Arr(__m) })?)) }) as Rc<dyn Fn((i64, Str, Str), (i64, Str, Str)) -> Result<i64, Throw>> }; move |__a0, __a1| __f(__a0, __a1) })?; self.set_p_patches(cast::<Map<ArrayKey, (i64, Str, Str)>>(__sorted)); true };
    #[allow(unreachable_code)] Ok(())
    }
    pub fn fixupTokens(&self, mut tokens_v: List<crate::php_parser::Token>) -> Result<List<crate::php_parser::Token>, Throw> {
    let mut patchIdx: i64 = Default::default();
    let mut patchPos: i64 = Default::default();
    let mut patchType: Str = Default::default();
    let mut patchText: Str = Default::default();
    let mut posDelta: i64 = Default::default();
    let mut lineDelta: i64 = Default::default();
    let mut i: i64 = Default::default();
    let mut c: i64 = Default::default();
    let mut token: Mixed = Default::default();
    let mut pos: Mixed = Default::default();
    let mut localPosDelta: i64 = Default::default();
    let mut len_v: i64 = Default::default();
    let mut patchTextLen: i64 = Default::default();
    if (self.p_patches_get().count() == 0i64) {
        return Ok(tokens_v.clone());
    }
    patchIdx = 0i64;
    let __d1 = self.p_patches_get().idx(&to_key(&patchIdx)).clone(); patchPos = __d1.0.clone(); patchType = __d1.1.clone(); patchText = __d1.2.clone(); 
    posDelta = 0i64;
    lineDelta = 0i64;
    {
        i = 0i64;
        c = tokens_v.clone().count();
        'l1: loop {
            if !((i < c)) { break; }
            'c2: {
                token = cast::<Mixed>(tokens_v.clone().idx(i).clone());
                pos = cast::<Mixed>(mixed_prop(&token.clone(), &Str::from_static("pos")));
                { let __t2 = num_add(to_num(&mixed_prop(&token.clone(), &Str::from_static("pos")).unwrap_or_default()), Num::Int(posDelta)); mixed_set_prop(&token.clone(), &Str::from_static("pos"), __t2.to_mixed()); }
                { let __t3 = num_add(to_num(&mixed_prop(&token.clone(), &Str::from_static("line")).unwrap_or_default()), Num::Int(lineDelta)); mixed_set_prop(&token.clone(), &Str::from_static("line"), __t3.to_mixed()); }
                localPosDelta = 0i64;
                len_v = strlen(&cast::<Str>(cast::<Mixed>(mixed_prop(&token.clone(), &Str::from_static("text")))));
                'l3: loop {
                    if !(php_ge(&cast::<Mixed>(patchPos), &pos.clone()) && php_lt(&cast::<Mixed>(patchPos), &num_add(to_num(&pos.clone()), Num::Int(len_v)).to_mixed())) { break; }
                    patchTextLen = strlen(&patchText.clone());
                    if identical(&patchType.clone(), &Str::from_static("remove")) {
                        if (identical(&cast::<Mixed>(patchPos), &pos.clone()) && (patchTextLen == len_v)) {
                            let _: List<crate::php_parser::Token> = { let __off = i; let __len = Some(1i64); let __repl = cast::<List<crate::php_parser::Token>>(Map::<ArrayKey, Mixed>::new().map_values(|v| cast::<crate::php_parser::Token>(v))).into_vec(); let __r = array_splice_l(&mut tokens_v, __off, __len, __repl); __r };
                            let _ = { let __t4 = i; i = __t4.wrapping_sub(1); __t4 };
                            let _ = { let __t5 = c; c = __t5.wrapping_sub(1); __t5 };
                        } else {
                            mixed_set_prop(&token.clone(), &Str::from_static("text"), cast::<Mixed>(substr_replace(&cast::<Str>(mixed_prop(&token.clone(), &Str::from_static("text")).unwrap()), &Str::from_static(""), cast::<i64>(num_add(to_num(&num_sub(Num::Int(patchPos), to_num(&pos.clone())).to_mixed()), Num::Int(localPosDelta)).to_mixed()), Some(patchTextLen))));
                            { let __t6 = patchTextLen; localPosDelta = (localPosDelta).wrapping_sub(__t6); }
                        }
                        { let __t7 = substr_count(&patchText.clone(), &Str::from_static("\n")); lineDelta = (lineDelta).wrapping_sub(__t7); }
                    } else if identical(&patchType.clone(), &Str::from_static("add")) {
                        mixed_set_prop(&token.clone(), &Str::from_static("text"), cast::<Mixed>(substr_replace(&cast::<Str>(mixed_prop(&token.clone(), &Str::from_static("text")).unwrap()), &patchText.clone(), cast::<i64>(num_add(to_num(&num_sub(Num::Int(patchPos), to_num(&pos.clone())).to_mixed()), Num::Int(localPosDelta)).to_mixed()), Some(0i64))));
                        { let __t8 = patchTextLen; localPosDelta = (localPosDelta).wrapping_add(__t8); }
                        { let __t9 = substr_count(&patchText.clone(), &Str::from_static("\n")); lineDelta = (lineDelta).wrapping_add(__t9); }
                    } else if identical(&patchType.clone(), &Str::from_static("replace")) {
                        mixed_set_prop(&token.clone(), &Str::from_static("text"), cast::<Mixed>(substr_replace(&cast::<Str>(mixed_prop(&token.clone(), &Str::from_static("text")).unwrap()), &patchText.clone(), cast::<i64>(num_add(to_num(&num_sub(Num::Int(patchPos), to_num(&pos.clone())).to_mixed()), Num::Int(localPosDelta)).to_mixed()), Some(patchTextLen))));
                    } else {
                        let _: bool = { if !(false) { return Err(Throw::assertion(Str::from_static("assert(Expr_ConstFetch)"))); } true };
                    }
                    let _ = { let __t10 = patchIdx; patchIdx = __t10.wrapping_add(1); __t10 };
                    if (patchIdx >= self.p_patches_get().count()) {
                        patchPos = consts::PHP_INT_MAX;
                        { break 'l3 };
                    }
                    let __d11 = self.p_patches_get().idx(&to_key(&patchIdx)).clone(); patchPos = __d11.0.clone(); patchType = __d11.1.clone(); patchText = __d11.2.clone(); 
                }
                { let __t12 = localPosDelta; posDelta = (posDelta).wrapping_add(__t12); }
            }
            let _ = { let __t13 = i; i = __t13.wrapping_add(1); __t13 };
        }
    }
    return Ok(tokens_v.clone());
    }
    pub fn fixupErrors(&self, mut errors: Map<ArrayKey, crate::php_parser::Error>) -> Result<(), Throw> {
    let mut error_v: Late<crate::php_parser::Error> = Late::uninit();
    let mut attrs: Map<Str, Mixed> = Default::default();
    let mut posDelta: i64 = Default::default();
    let mut lineDelta: i64 = Default::default();
    let mut patch: (i64, Str, Str) = Default::default();
    let mut patchPos: i64 = Default::default();
    let mut patchType: Str = Default::default();
    let mut patchText: Str = Default::default();
    'l1: for __kv1 in errors.clone().into_iter() {
        error_v.set(__kv1.1);
        attrs = error_v.get().clone().getAttributes()?;
        posDelta = 0i64;
        lineDelta = 0i64;
        'l2: for __kv2 in self.p_patches_get().into_iter() {
            patch = __kv2.1;
            let __d3 = patch.clone(); patchPos = __d3.0.clone(); patchType = __d3.1.clone(); patchText = __d3.2.clone(); 
            if php_ge(&cast::<Mixed>(patchPos), &attrs.clone().idx(&Str::from_static("startFilePos")).clone()) {
                { break 'l2 };
            }
            if identical(&patchType.clone(), &Str::from_static("add")) {
                { let __t4 = strlen(&patchText.clone()); posDelta = (posDelta).wrapping_add(__t4); }
                { let __t5 = substr_count(&patchText.clone(), &Str::from_static("\n")); lineDelta = (lineDelta).wrapping_add(__t5); }
            } else if identical(&patchType.clone(), &Str::from_static("remove")) {
                { let __t6 = strlen(&patchText.clone()); posDelta = (posDelta).wrapping_sub(__t6); }
                { let __t7 = substr_count(&patchText.clone(), &Str::from_static("\n")); lineDelta = (lineDelta).wrapping_sub(__t7); }
            }
        }
        { let __t8 = num_add(to_num(&attrs.clone().idx(&Str::from_static("startFilePos")).clone()), Num::Int(posDelta)); attrs.insert(Str::from_static("startFilePos"), __t8.to_mixed()); }
        { let __t9 = num_add(to_num(&attrs.clone().idx(&Str::from_static("endFilePos")).clone()), Num::Int(posDelta)); attrs.insert(Str::from_static("endFilePos"), __t9.to_mixed()); }
        { let __t10 = num_add(to_num(&attrs.clone().idx(&Str::from_static("startLine")).clone()), Num::Int(lineDelta)); attrs.insert(Str::from_static("startLine"), __t10.to_mixed()); }
        { let __t11 = num_add(to_num(&attrs.clone().idx(&Str::from_static("endLine")).clone()), Num::Int(lineDelta)); attrs.insert(Str::from_static("endLine"), __t11.to_mixed()); }
        error_v.get().clone().setAttributes(attrs.clone())?;
    }
    #[allow(unreachable_code)] Ok(())
    }
    pub fn postprocessTokens(&self, mut tokens_v: &mut List<crate::php_parser::Token>, mut errorHandler: crate::php_parser::ErrorHandler) -> Result<(), Throw> { cast::<crate::php_parser::Lexer>(self.clone()).postprocessTokens__impl(tokens_v, errorHandler) }
    pub fn new_same_class(&self, mut phpVersion: Option<crate::php_parser::PhpVersion>) -> Result<crate::php_parser::lexer::Emulative, Throw> { Ok(Self::new(phpVersion)?) }
}
impl php_rt::PhpObject for Emulative {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\Emulative" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\emulative", "phpparser\\lexer"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_patches_get()) { out.push((Str::from_static("patches"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_emulators_get()) { out.push((Str::from_static("emulators"), cast::<Mixed>(v))); } if let Some(v) = self.p_targetPhpVersion_opt() { out.push((Str::from_static("targetPhpVersion"), cast::<Mixed>(v))); } if let Some(v) = self.p_hostPhpVersion_opt() { out.push((Str::from_static("hostPhpVersion"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "patches" => { self.set_p_patches(cast::<Map<ArrayKey, (i64, Str, Str)>>(value)); true }, "emulators" => { self.set_p_emulators(cast::<List<crate::php_parser::lexer::token_emulator::TokenEmulator>>(value)); true }, "targetPhpVersion" => { self.set_p_targetPhpVersion(cast::<crate::php_parser::PhpVersion>(value)); true }, "hostPhpVersion" => { self.set_p_hostPhpVersion(cast::<crate::php_parser::PhpVersion>(value)); true }, _ => false } }
}
impl Emulative { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\Emulative could not be converted to string"))) } }
impl php_rt::PhpClone for Emulative { fn php_clone(&self) -> Self { let c = Emulative(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for EmulativeObj { fn clone(&self) -> Self { EmulativeObj { patches: self.patches.clone(), emulators: self.emulators.clone(), targetPhpVersion: self.targetPhpVersion.clone(), hostPhpVersion: self.hostPhpVersion.clone() } } }
impl Emulative {
}
impl php_rt::Truthy for Emulative { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for Emulative { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\Emulative")) } }
impl php_rt::Identical for Emulative { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for Emulative { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Emulative { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Emulative { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Emulative> for Mixed { fn cast_to(self) -> Emulative { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::Emulative>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\Emulative") } }
impl php_rt::TryDowncast for Emulative { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::Emulative>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Emulative { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Emulative> for AnyObject { fn cast_to(self) -> Emulative { cast::<Emulative>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Emulative> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\emulative") } }
impl php_rt::InstanceOf<Emulative> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\emulative") } }
impl php_rt::InstanceOf<Emulative> for Emulative { fn is_instance(&self) -> bool { true } }
