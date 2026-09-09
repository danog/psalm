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
    emulators = array_filter_cb_l(&self.p_emulators_get(), { let __f = { let code = code.clone(); let this = self.clone(); Rc::new(move |mut emulator: crate::php_parser::lexer::token_emulator::TokenEmulator| -> Result<bool, Throw> { let mut code = code.clone(); 
    let mut code: Str = Default::default();
    return Ok(emulator.clone().isEmulationNeeded(code.clone())?);
    }) as Rc<dyn Fn(crate::php_parser::lexer::token_emulator::TokenEmulator) -> Result<bool, Throw>> }; move |__a0| __f(__a0) })?;
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
    let _: bool = { let __sorted = usort_m(&self.p_patches_get(), { let __f = { let __c14 = { let this = self.clone(); Rc::new(move |mut p1: Mixed, mut p2: Mixed| -> Result<i64, Throw> { 
    return Ok(spaceship(&cast::<Mixed>(mixed_get(&p1.clone(), &to_key(&0i64))), &cast::<Mixed>(mixed_get(&p2.clone(), &to_key(&0i64)))));
    }) as Rc<dyn Fn(Mixed, Mixed) -> Result<i64, Throw>> }; Rc::new(move |__p0: (i64, Str, Str), __p1: (i64, Str, Str)| -> Result<i64, Throw> { Ok(__c14({ let __c15 = __p0; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c15.0)); __m.push(cast::<Mixed>(__c15.1)); __m.push(cast::<Mixed>(__c15.2)); Mixed::Arr(__m) }, { let __c16 = __p1; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c16.0)); __m.push(cast::<Mixed>(__c16.1)); __m.push(cast::<Mixed>(__c16.2)); Mixed::Arr(__m) })?) }) as Rc<dyn Fn((i64, Str, Str), (i64, Str, Str)) -> Result<i64, Throw>> }; move |__a0, __a1| __f(__a0, __a1) })?; self.set_p_patches(cast::<Map<ArrayKey, (i64, Str, Str)>>(__sorted)); true };
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
pub struct EmulativeTestObj {
    pub expectedException: Option<Str>,
    pub expectedExceptionMessage: Option<Str>,
    pub expectedExceptionMessageRegExp: Option<Str>,
    pub expectedExceptionCode: Option<i64>,
    pub name: Str,
}
#[derive(Clone)]
pub struct EmulativeTest(pub Rc<RefCell<EmulativeTestObj>>);
impl EmulativeTest {
    pub fn p_expectedException(&self) -> Ref<'_, Option<Str>> { Ref::map(self.0.borrow(), |o| &o.expectedException) }
    pub fn p_expectedException_get(&self) -> Option<Str> { self.0.borrow().expectedException.clone() }
    pub fn p_expectedException_opt(&self) -> Option<Option<Str>> { Some(self.0.borrow().expectedException.clone()) }
    pub fn p_expectedException_mut(&self) -> RefMut<'_, Option<Str>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.expectedException) }
    pub fn set_p_expectedException(&self, v: Option<Str>) { self.0.borrow_mut().expectedException = v; }
    pub fn p_expectedExceptionMessage(&self) -> Ref<'_, Option<Str>> { Ref::map(self.0.borrow(), |o| &o.expectedExceptionMessage) }
    pub fn p_expectedExceptionMessage_get(&self) -> Option<Str> { self.0.borrow().expectedExceptionMessage.clone() }
    pub fn p_expectedExceptionMessage_opt(&self) -> Option<Option<Str>> { Some(self.0.borrow().expectedExceptionMessage.clone()) }
    pub fn p_expectedExceptionMessage_mut(&self) -> RefMut<'_, Option<Str>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.expectedExceptionMessage) }
    pub fn set_p_expectedExceptionMessage(&self, v: Option<Str>) { self.0.borrow_mut().expectedExceptionMessage = v; }
    pub fn p_expectedExceptionMessageRegExp(&self) -> Ref<'_, Option<Str>> { Ref::map(self.0.borrow(), |o| &o.expectedExceptionMessageRegExp) }
    pub fn p_expectedExceptionMessageRegExp_get(&self) -> Option<Str> { self.0.borrow().expectedExceptionMessageRegExp.clone() }
    pub fn p_expectedExceptionMessageRegExp_opt(&self) -> Option<Option<Str>> { Some(self.0.borrow().expectedExceptionMessageRegExp.clone()) }
    pub fn p_expectedExceptionMessageRegExp_mut(&self) -> RefMut<'_, Option<Str>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.expectedExceptionMessageRegExp) }
    pub fn set_p_expectedExceptionMessageRegExp(&self, v: Option<Str>) { self.0.borrow_mut().expectedExceptionMessageRegExp = v; }
    pub fn p_expectedExceptionCode(&self) -> Ref<'_, Option<i64>> { Ref::map(self.0.borrow(), |o| &o.expectedExceptionCode) }
    pub fn p_expectedExceptionCode_get(&self) -> Option<i64> { self.0.borrow().expectedExceptionCode.clone() }
    pub fn p_expectedExceptionCode_opt(&self) -> Option<Option<i64>> { Some(self.0.borrow().expectedExceptionCode.clone()) }
    pub fn p_expectedExceptionCode_mut(&self) -> RefMut<'_, Option<i64>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.expectedExceptionCode) }
    pub fn set_p_expectedExceptionCode(&self, v: Option<i64>) { self.0.borrow_mut().expectedExceptionCode = v; }
    pub fn p_name(&self) -> Ref<'_, Str> { Ref::map(self.0.borrow(), |o| &o.name) }
    pub fn p_name_get(&self) -> Str { self.0.borrow().name.clone() }
    pub fn p_name_opt(&self) -> Option<Str> { Some(self.0.borrow().name.clone()) }
    pub fn p_name_mut(&self) -> RefMut<'_, Str> { RefMut::map(self.0.borrow_mut(), |o| &mut o.name) }
    pub fn set_p_name(&self, v: Str) { self.0.borrow_mut().name = v; }
    pub fn new(mut name: Str) -> Result<EmulativeTest, Throw> {
        let this = EmulativeTest(Rc::new(RefCell::new(EmulativeTestObj {
            expectedException: { let _ = (); None::<Str> },
            expectedExceptionMessage: { let _ = (); None::<Str> },
            expectedExceptionMessageRegExp: { let _ = (); None::<Str> },
            expectedExceptionCode: { let _ = (); None::<i64> },
            name: Str::from_static(""),
        })));
        this.magic__construct(name)?;
        Ok(this)
    }
    pub fn getLexer(&self) -> Result<crate::php_parser::lexer::Emulative, Throw> {
    return Ok(crate::php_parser::lexer::Emulative::new({ let _ = (); None::<crate::php_parser::PhpVersion> })?);
    }
    pub fn testReplaceKeywords(&self, mut keyword: Str, mut expectedToken: i64) -> Result<(), Throw> {
    let mut lexer: Late<crate::php_parser::lexer::Emulative> = Late::uninit();
    let mut code: Str = Default::default();
    lexer.set(self.getLexer()?);
    code = concat(Str::from_static("<?php "), keyword.clone());
    { let _ = self; crate::phpunit::framework::Assert::assertEquals({ let __c1182 = (crate::php_parser::Token::new(394i64, Str::from_static("<?php "), 1i64, 0i64)?, crate::php_parser::Token::new(expectedToken, keyword.clone(), 1i64, 6i64)?, crate::php_parser::Token::new(0i64, Str::from_static_bytes(b"\x00"), 1i64, strlen(&code.clone()))?); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1182.0)); __m.push(cast::<Mixed>(__c1182.1)); __m.push(cast::<Mixed>(__c1182.2)); Mixed::Arr(__m) }, cast::<Mixed>(lexer.get().clone().tokenize(code.clone(), { let _ = (); None::<crate::php_parser::ErrorHandler> })?), Str::from_static(""))? };
    #[allow(unreachable_code)] Ok(())
    }
    pub fn testReplaceKeywordsUppercase(&self, mut keyword: Str, mut expectedToken: i64) -> Result<(), Throw> {
    let mut lexer: Late<crate::php_parser::lexer::Emulative> = Late::uninit();
    let mut code: Str = Default::default();
    lexer.set(self.getLexer()?);
    code = concat(Str::from_static("<?php "), strtoupper(&keyword.clone()));
    { let _ = self; crate::phpunit::framework::Assert::assertEquals({ let __c1183 = (crate::php_parser::Token::new(394i64, Str::from_static("<?php "), 1i64, 0i64)?, crate::php_parser::Token::new(expectedToken, strtoupper(&keyword.clone()), 1i64, 6i64)?, crate::php_parser::Token::new(0i64, Str::from_static_bytes(b"\x00"), 1i64, strlen(&code.clone()))?); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1183.0)); __m.push(cast::<Mixed>(__c1183.1)); __m.push(cast::<Mixed>(__c1183.2)); Mixed::Arr(__m) }, cast::<Mixed>(lexer.get().clone().tokenize(code.clone(), { let _ = (); None::<crate::php_parser::ErrorHandler> })?), Str::from_static(""))? };
    #[allow(unreachable_code)] Ok(())
    }
    pub fn testNoReplaceKeywordsAfterObjectOperator(&self, mut keyword: Str) -> Result<(), Throw> {
    let mut lexer: Late<crate::php_parser::lexer::Emulative> = Late::uninit();
    let mut code: Str = Default::default();
    lexer.set(self.getLexer()?);
    code = concat(Str::from_static("<?php ->"), keyword.clone());
    { let _ = self; crate::phpunit::framework::Assert::assertEquals({ let __c1184 = (crate::php_parser::Token::new(394i64, Str::from_static("<?php "), 1i64, 0i64)?, crate::php_parser::Token::new(389i64, Str::from_static("->"), 1i64, 6i64)?, crate::php_parser::Token::new(262i64, keyword.clone(), 1i64, 8i64)?, crate::php_parser::Token::new(0i64, Str::from_static_bytes(b"\x00"), 1i64, strlen(&code.clone()))?); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1184.0)); __m.push(cast::<Mixed>(__c1184.1)); __m.push(cast::<Mixed>(__c1184.2)); __m.push(cast::<Mixed>(__c1184.3)); Mixed::Arr(__m) }, cast::<Mixed>(lexer.get().clone().tokenize(code.clone(), { let _ = (); None::<crate::php_parser::ErrorHandler> })?), Str::from_static(""))? };
    #[allow(unreachable_code)] Ok(())
    }
    pub fn testNoReplaceKeywordsAfterObjectOperatorWithSpaces(&self, mut keyword: Str) -> Result<(), Throw> {
    let mut lexer: Late<crate::php_parser::lexer::Emulative> = Late::uninit();
    let mut code: Str = Default::default();
    lexer.set(self.getLexer()?);
    code = concat(Str::from_static("<?php ->    "), keyword.clone());
    { let _ = self; crate::phpunit::framework::Assert::assertEquals({ let __c1185 = (crate::php_parser::Token::new(394i64, Str::from_static("<?php "), 1i64, 0i64)?, crate::php_parser::Token::new(389i64, Str::from_static("->"), 1i64, 6i64)?, crate::php_parser::Token::new(397i64, Str::from_static("    "), 1i64, 8i64)?, crate::php_parser::Token::new(262i64, keyword.clone(), 1i64, 12i64)?, crate::php_parser::Token::new(0i64, Str::from_static_bytes(b"\x00"), 1i64, strlen(&code.clone()))?); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1185.0)); __m.push(cast::<Mixed>(__c1185.1)); __m.push(cast::<Mixed>(__c1185.2)); __m.push(cast::<Mixed>(__c1185.3)); __m.push(cast::<Mixed>(__c1185.4)); Mixed::Arr(__m) }, cast::<Mixed>(lexer.get().clone().tokenize(code.clone(), { let _ = (); None::<crate::php_parser::ErrorHandler> })?), Str::from_static(""))? };
    #[allow(unreachable_code)] Ok(())
    }
    pub fn testNoReplaceKeywordsAfterObjectOperatorWithComment(&self) -> Result<(), Throw> {
    let mut keyword: Str = Default::default();
    let mut lexer: Late<crate::php_parser::lexer::Emulative> = Late::uninit();
    let mut code: Str = Default::default();
    keyword = Str::from_static("__property__");
    lexer.set(self.getLexer()?);
    code = concat(Str::from_static("<?php ->/* comment */"), keyword.clone());
    { let _ = self; crate::phpunit::framework::Assert::assertEquals({ let __c1186 = (crate::php_parser::Token::new(394i64, Str::from_static("<?php "), 1i64, 0i64)?, crate::php_parser::Token::new(389i64, Str::from_static("->"), 1i64, 6i64)?, crate::php_parser::Token::new(392i64, Str::from_static("/* comment */"), 1i64, 8i64)?, crate::php_parser::Token::new(262i64, keyword.clone(), 1i64, 21i64)?, crate::php_parser::Token::new(0i64, Str::from_static_bytes(b"\x00"), 1i64, strlen(&code.clone()))?); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1186.0)); __m.push(cast::<Mixed>(__c1186.1)); __m.push(cast::<Mixed>(__c1186.2)); __m.push(cast::<Mixed>(__c1186.3)); __m.push(cast::<Mixed>(__c1186.4)); Mixed::Arr(__m) }, cast::<Mixed>(lexer.get().clone().tokenize(code.clone(), { let _ = (); None::<crate::php_parser::ErrorHandler> })?), Str::from_static(""))? };
    #[allow(unreachable_code)] Ok(())
    }
    pub fn testNoReplaceKeywordsAfterNullsafeObjectOperator(&self, mut keyword: Str) -> Result<(), Throw> {
    let mut lexer: Late<crate::php_parser::lexer::Emulative> = Late::uninit();
    let mut code: Str = Default::default();
    lexer.set(self.getLexer()?);
    code = concat(Str::from_static("<?php ?->"), keyword.clone());
    { let _ = self; crate::phpunit::framework::Assert::assertEquals({ let __c1187 = (crate::php_parser::Token::new(394i64, Str::from_static("<?php "), 1i64, 0i64)?, crate::php_parser::Token::new(390i64, Str::from_static("?->"), 1i64, 6i64)?, crate::php_parser::Token::new(262i64, keyword.clone(), 1i64, 9i64)?, crate::php_parser::Token::new(0i64, Str::from_static_bytes(b"\x00"), 1i64, strlen(&code.clone()))?); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1187.0)); __m.push(cast::<Mixed>(__c1187.1)); __m.push(cast::<Mixed>(__c1187.2)); __m.push(cast::<Mixed>(__c1187.3)); Mixed::Arr(__m) }, cast::<Mixed>(lexer.get().clone().tokenize(code.clone(), { let _ = (); None::<crate::php_parser::ErrorHandler> })?), Str::from_static(""))? };
    #[allow(unreachable_code)] Ok(())
    }
    pub fn provideTestReplaceKeywords() -> Result<List<(Str, i64)>, Throw> {
    return Ok(list![(Str::from_static("__PROPERTY__"), 353i64), (Str::from_static("match"), 306i64), (Str::from_static("fn"), 311i64), (Str::from_static("finally"), 316i64), (Str::from_static("yield"), 281i64), (Str::from_static("callable"), 345i64), (Str::from_static("insteadof"), 319i64), (Str::from_static("trait"), 337i64), (Str::from_static("__TRAIT__"), 350i64), (Str::from_static("__DIR__"), 348i64), (Str::from_static("goto"), 309i64), (Str::from_static("namespace"), 342i64), (Str::from_static("__NAMESPACE__"), 354i64)]);
    }
    pub fn assertSameTokens(&self, mut expectedTokens: Map<ArrayKey, Mixed>, mut tokens_v: Map<ArrayKey, Mixed>) -> Result<(), Throw> {
    let mut reducedTokens: List<(Mixed, Mixed)> = Default::default();
    let mut token: Mixed = Default::default();
    reducedTokens = List::<(Mixed, Mixed)>::new();
    'l1: for __kv1 in tokens_v.clone().into_iter() {
        token = __kv1.1;
        if (identical(&cast::<Mixed>(mixed_prop(&token.clone(), &Str::from_static("id"))), &cast::<Mixed>(0i64)) || truthy(&mixed_call(&token.clone(), &Str::from_static("isIgnorable"), vec![])?)) {
            { continue 'l1 };
        }
        reducedTokens.push((cast::<Mixed>(mixed_prop(&token.clone(), &Str::from_static("id"))), cast::<Mixed>(mixed_prop(&token.clone(), &Str::from_static("text")))));
    }
    { let _ = self; crate::phpunit::framework::Assert::assertSame(cast::<Mixed>(expectedTokens.clone()), cast::<Mixed>(reducedTokens.clone()), Str::from_static(""))? };
    #[allow(unreachable_code)] Ok(())
    }
    pub fn testLexNewFeatures(&self, mut code: Str, mut expectedTokens: Map<ArrayKey, Mixed>) -> Result<(), Throw> {
    let mut lexer: Late<crate::php_parser::lexer::Emulative> = Late::uninit();
    lexer.set(self.getLexer()?);
    self.assertSameTokens(expectedTokens.clone(), lexer.get().clone().tokenize(concat(Str::from_static("<?php "), code.clone()), { let _ = (); None::<crate::php_parser::ErrorHandler> })?)?;
    #[allow(unreachable_code)] Ok(())
    }
    pub fn testLeaveStuffAloneInStrings(&self, mut code: Str) -> Result<(), Throw> {
    let mut stringifiedToken: Str = Default::default();
    let mut lexer: Late<crate::php_parser::lexer::Emulative> = Late::uninit();
    let mut fullCode: Str = Default::default();
    stringifiedToken = concat(concat(Str::from_static("\""), addcslashes(&code.clone(), &Str::from_static("\"\\"))), Str::from_static("\""));
    lexer.set(self.getLexer()?);
    fullCode = concat(Str::from_static("<?php "), stringifiedToken.clone());
    { let _ = self; crate::phpunit::framework::Assert::assertEquals({ let __c1188 = (crate::php_parser::Token::new(394i64, Str::from_static("<?php "), 1i64, 0i64)?, crate::php_parser::Token::new(269i64, stringifiedToken.clone(), 1i64, 6i64)?, crate::php_parser::Token::new(0i64, Str::from_static_bytes(b"\x00"), (substr_count(&fullCode.clone(), &Str::from_static("\n"))).wrapping_add(1i64), strlen(&fullCode.clone()))?); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1188.0)); __m.push(cast::<Mixed>(__c1188.1)); __m.push(cast::<Mixed>(__c1188.2)); Mixed::Arr(__m) }, cast::<Mixed>(lexer.get().clone().tokenize(fullCode.clone(), { let _ = (); None::<crate::php_parser::ErrorHandler> })?), Str::from_static(""))? };
    #[allow(unreachable_code)] Ok(())
    }
    pub fn testErrorAfterEmulation(&self, mut code: Mixed) -> Result<(), Throw> {
    let mut errorHandler: Late<crate::php_parser::error_handler::Collecting> = Late::uninit();
    let mut lexer: Late<crate::php_parser::lexer::Emulative> = Late::uninit();
    let mut errors: Map<ArrayKey, crate::php_parser::Error> = Default::default();
    let mut error_v: Late<crate::php_parser::Error> = Late::uninit();
    let mut attrs: Map<Str, Mixed> = Default::default();
    let mut expPos: i64 = Default::default();
    let mut expLine: i64 = Default::default();
    errorHandler.set(crate::php_parser::error_handler::Collecting::new()?);
    lexer.set(self.getLexer()?);
    let _: Map<ArrayKey, Mixed> = lexer.get().clone().tokenize(concat(concat(Str::from_static("<?php "), cast::<Str>(code.clone())), Str::from_static_bytes(b"\x00")), Some(cast::<crate::php_parser::ErrorHandler>(errorHandler.get().clone())))?;
    errors = errorHandler.get().clone().getErrors()?;
    { let _ = self; crate::phpunit::framework::Assert::assertCount(1i64, cast::<Mixed>(errors.clone()), Str::from_static(""))? };
    error_v.set(errors.clone().idx(&to_key(&0i64)).clone());
    { let _ = self; crate::phpunit::framework::Assert::assertSame(cast::<Mixed>(Str::from_static("Unexpected null byte")), cast::<Mixed>(error_v.get().clone().getRawMessage()?), Str::from_static(""))? };
    attrs = error_v.get().clone().getAttributes()?;
    expPos = strlen(&concat(Str::from_static("<?php "), cast::<Str>(code.clone())));
    expLine = (1i64).wrapping_add(substr_count(&concat(Str::from_static("<?php "), cast::<Str>(code.clone())), &Str::from_static("\n")));
    { let _ = self; crate::phpunit::framework::Assert::assertSame(cast::<Mixed>(expPos), attrs.clone().idx(&Str::from_static("startFilePos")).clone(), Str::from_static(""))? };
    { let _ = self; crate::phpunit::framework::Assert::assertSame(cast::<Mixed>(expPos), attrs.clone().idx(&Str::from_static("endFilePos")).clone(), Str::from_static(""))? };
    { let _ = self; crate::phpunit::framework::Assert::assertSame(cast::<Mixed>(expLine), attrs.clone().idx(&Str::from_static("startLine")).clone(), Str::from_static(""))? };
    { let _ = self; crate::phpunit::framework::Assert::assertSame(cast::<Mixed>(expLine), attrs.clone().idx(&Str::from_static("endLine")).clone(), Str::from_static(""))? };
    #[allow(unreachable_code)] Ok(())
    }
    pub fn provideTestLexNewFeatures() -> Result<List<U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a>, Throw> {
    return Ok(list![U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("yield from"), ((282i64, Str::from_static("yield from")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("yield\r\nfrom"), ((282i64, Str::from_static("yield\r\nfrom")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("..."), ((404i64, Str::from_static("...")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("**"), ((406i64, Str::from_static("**")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("**="), ((407i64, Str::from_static("**=")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("??"), ((405i64, Str::from_static("??")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("<=>"), ((376i64, Str::from_static("<=>")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("0b1010110"), ((260i64, Str::from_static("0b1010110")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("0b1011010101001010110101010010101011010101010101101011001110111100"), ((261i64, Str::from_static("0b1011010101001010110101010010101011010101010101101011001110111100")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("\\"), ((403i64, Str::from_static("\\")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup3_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("<<<'NOWDOC'\nNOWDOC;\n"), ((398i64, Str::from_static("<<<'NOWDOC'\n")), (399i64, Str::from_static("NOWDOC")), (ord(&Str::from_static(";")), Str::from_static(";"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup4_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("<<<'NOWDOC'\nFoobar\nNOWDOC;\n"), ((398i64, Str::from_static("<<<'NOWDOC'\n")), (268i64, Str::from_static("Foobar\n")), (399i64, Str::from_static("NOWDOC")), (ord(&Str::from_static(";")), Str::from_static(";"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup3_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("<<<LABEL\nLABEL,"), ((398i64, Str::from_static("<<<LABEL\n")), (399i64, Str::from_static("LABEL")), (ord(&Str::from_static(",")), Str::from_static(","))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup3_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("<<<LABEL\n    LABEL,"), ((398i64, Str::from_static("<<<LABEL\n")), (399i64, Str::from_static("    LABEL")), (ord(&Str::from_static(",")), Str::from_static(","))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup4_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("<<<LABEL\n    Foo\n  LABEL;"), ((398i64, Str::from_static("<<<LABEL\n")), (268i64, Str::from_static("    Foo\n")), (399i64, Str::from_static("  LABEL")), (ord(&Str::from_static(";")), Str::from_static(";"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup6_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("<<<A\n A,<<<A\n A,"), ((398i64, Str::from_static("<<<A\n")), (399i64, Str::from_static(" A")), (ord(&Str::from_static(",")), Str::from_static(",")), (398i64, Str::from_static("<<<A\n")), (399i64, Str::from_static(" A")), (ord(&Str::from_static(",")), Str::from_static(","))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup3_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("<<<LABEL\nLABELNOPE\nLABEL\n"), ((398i64, Str::from_static("<<<LABEL\n")), (268i64, Str::from_static("LABELNOPE\n")), (399i64, Str::from_static("LABEL"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup3_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("<<<LABEL\n    LABEL\nLABEL\n"), ((398i64, Str::from_static("<<<LABEL\n")), (399i64, Str::from_static("    LABEL")), (262i64, Str::from_static("LABEL"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("??="), ((367i64, Str::from_static("??=")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("1_000"), ((260i64, Str::from_static("1_000")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("0x7AFE_F00D"), ((260i64, Str::from_static("0x7AFE_F00D")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("0b0101_1111"), ((260i64, Str::from_static("0b0101_1111")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("0137_041"), ((260i64, Str::from_static("0137_041")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("1_000.0"), ((261i64, Str::from_static("1_000.0")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("1_0.0"), ((261i64, Str::from_static("1_0.0")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("1_000_000_000.0"), ((261i64, Str::from_static("1_000_000_000.0")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("0e1_0"), ((261i64, Str::from_static("0e1_0")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("1_0e+10"), ((261i64, Str::from_static("1_0e+10")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("1_0e-10"), ((261i64, Str::from_static("1_0e-10")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("0b1011010101001010_110101010010_10101101010101_0101101011001_110111100"), ((261i64, Str::from_static("0b1011010101001010_110101010010_10101101010101_0101101011001_110111100")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("0xFFFF_FFFF_FFFF_FFFF"), ((261i64, Str::from_static("0xFFFF_FFFF_FFFF_FFFF")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup3_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("1_000+1"), ((260i64, Str::from_static("1_000")), (ord(&Str::from_static("+")), Str::from_static("+")), (260i64, Str::from_static("1"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("1_0abc"), ((260i64, Str::from_static("1_0")), (262i64, Str::from_static("abc"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("?->"), ((390i64, Str::from_static("?->")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup3_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("#[Attr]"), ((355i64, Str::from_static("#[")), (262i64, Str::from_static("Attr")), (ord(&Str::from_static("]")), Str::from_static("]"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup3_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("#[\nAttr\n]"), ((355i64, Str::from_static("#[")), (262i64, Str::from_static("Attr")), (ord(&Str::from_static("]")), Str::from_static("]"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup6_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("<<<LABEL\n    LABEL, #[Attr]"), ((398i64, Str::from_static("<<<LABEL\n")), (399i64, Str::from_static("    LABEL")), (ord(&Str::from_static(",")), Str::from_static(",")), (355i64, Str::from_static("#[")), (262i64, Str::from_static("Attr")), (ord(&Str::from_static("]")), Str::from_static("]"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup6_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("#[Attr] <<<LABEL\n    LABEL,"), ((355i64, Str::from_static("#[")), (262i64, Str::from_static("Attr")), (ord(&Str::from_static("]")), Str::from_static("]")), (398i64, Str::from_static("<<<LABEL\n")), (399i64, Str::from_static("    LABEL")), (ord(&Str::from_static(",")), Str::from_static(","))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup4_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("enum Foo {}"), ((339i64, Str::from_static("enum")), (262i64, Str::from_static("Foo")), (ord(&Str::from_static("{")), Str::from_static("{")), (ord(&Str::from_static("}")), Str::from_static("}"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup4_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("class Enum {}"), ((336i64, Str::from_static("class")), (262i64, Str::from_static("Enum")), (ord(&Str::from_static("{")), Str::from_static("{")), (ord(&Str::from_static("}")), Str::from_static("}"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup6_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("class Enum extends X {}"), ((336i64, Str::from_static("class")), (262i64, Str::from_static("Enum")), (340i64, Str::from_static("extends")), (262i64, Str::from_static("X")), (ord(&Str::from_static("{")), Str::from_static("{")), (ord(&Str::from_static("}")), Str::from_static("}"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup6_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("class Enum implements X {}"), ((336i64, Str::from_static("class")), (262i64, Str::from_static("Enum")), (341i64, Str::from_static("implements")), (262i64, Str::from_static("X")), (ord(&Str::from_static("{")), Str::from_static("{")), (ord(&Str::from_static("}")), Str::from_static("}"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("0o123"), ((260i64, Str::from_static("0o123")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("0O123"), ((260i64, Str::from_static("0O123")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("0o1_2_3"), ((260i64, Str::from_static("0o1_2_3")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("0o1000000000000000000000"), ((261i64, Str::from_static("0o1000000000000000000000")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("readonly class"), ((330i64, Str::from_static("readonly")), (336i64, Str::from_static("class"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup3_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("function readonly("), ((310i64, Str::from_static("function")), (330i64, Str::from_static("readonly")), (ord(&Str::from_static("(")), Str::from_static("("))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup3_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("function readonly ("), ((310i64, Str::from_static("function")), (330i64, Str::from_static("readonly")), (ord(&Str::from_static("(")), Str::from_static("("))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("private(set)"), ((327i64, Str::from_static("private(set)")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("PROTECTED(SET)"), ((328i64, Str::from_static("PROTECTED(SET)")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("Public(Set)"), ((329i64, Str::from_static("Public(Set)")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup4_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("public (set)"), ((326i64, Str::from_static("public")), (ord(&Str::from_static("(")), Str::from_static("(")), (262i64, Str::from_static("set")), (ord(&Str::from_static(")")), Str::from_static(")"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup5_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("->public(set)"), ((389i64, Str::from_static("->")), (262i64, Str::from_static("public")), (ord(&Str::from_static("(")), Str::from_static("(")), (262i64, Str::from_static("set")), (ord(&Str::from_static(")")), Str::from_static(")"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup5_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("?-> public(set)"), ((390i64, Str::from_static("?->")), (262i64, Str::from_static("public")), (ord(&Str::from_static("(")), Str::from_static("(")), (262i64, Str::from_static("set")), (ord(&Str::from_static(")")), Str::from_static(")"))))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("|>"), ((408i64, Str::from_static("|>")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("(void)"), ((388i64, Str::from_static("(void)")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("( \tvoid \t)"), ((388i64, Str::from_static("( \tvoid \t)")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup1_Tup2_Int_Str((Str::from_static("( vOiD)"), ((388i64, Str::from_static("( vOiD)")),))), U_Tup2_Str_Tup1_Tup2_Int_Str_or_Tup2_Str_Tup2_Tup2_Int_Str_Tup2_Int_St_c3a8549c8a::Tup2_Str_Tup3_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("(void\n)"), ((ord(&Str::from_static("(")), Str::from_static("(")), (262i64, Str::from_static("void")), (ord(&Str::from_static(")")), Str::from_static(")")))))]);
    }
    pub fn testTargetVersion(&self, mut phpVersion: Str, mut code: Str, mut expectedTokens: Map<ArrayKey, Mixed>) -> Result<(), Throw> {
    let mut lexer: Late<crate::php_parser::lexer::Emulative> = Late::uninit();
    lexer.set(crate::php_parser::lexer::Emulative::new(Some(crate::php_parser::PhpVersion::fromString(phpVersion.clone())?))?);
    self.assertSameTokens(expectedTokens.clone(), lexer.get().clone().tokenize(concat(Str::from_static("<?php "), code.clone()), { let _ = (); None::<crate::php_parser::ErrorHandler> })?)?;
    #[allow(unreachable_code)] Ok(())
    }
    pub fn provideTestTargetVersion() -> Result<List<U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3>, Throw> {
    return Ok(list![U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup1_Tup2_Int_Str((Str::from_static("8.0"), Str::from_static("match"), ((306i64, Str::from_static("match")),))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup1_Tup2_Int_Str((Str::from_static("7.4"), Str::from_static("match"), ((262i64, Str::from_static("match")),))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup1_Tup2_Int_Str((Str::from_static("8.0"), Str::from_static("MATCH"), ((306i64, Str::from_static("MATCH")),))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup1_Tup2_Int_Str((Str::from_static("7.4"), Str::from_static("MATCH"), ((262i64, Str::from_static("MATCH")),))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup5_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("8.0"), Str::from_static("\"$foo?->bar\""), ((ord(&Str::from_static("\"")), Str::from_static("\"")), (266i64, Str::from_static("$foo")), (390i64, Str::from_static("?->")), (262i64, Str::from_static("bar")), (ord(&Str::from_static("\"")), Str::from_static("\""))))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup6_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("8.0"), Str::from_static("\"$foo?->bar baz\""), ((ord(&Str::from_static("\"")), Str::from_static("\"")), (266i64, Str::from_static("$foo")), (390i64, Str::from_static("?->")), (262i64, Str::from_static("bar")), (268i64, Str::from_static(" baz")), (ord(&Str::from_static("\"")), Str::from_static("\""))))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup1_Tup2_Int_Str((Str::from_static("8.4"), Str::from_static("__PROPERTY__"), ((353i64, Str::from_static("__PROPERTY__")),))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup1_Tup2_Int_Str((Str::from_static("8.3"), Str::from_static("__PROPERTY__"), ((262i64, Str::from_static("__PROPERTY__")),))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup1_Tup2_Int_Str((Str::from_static("8.4"), Str::from_static("__property__"), ((353i64, Str::from_static("__property__")),))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup1_Tup2_Int_Str((Str::from_static("8.3"), Str::from_static("__property__"), ((262i64, Str::from_static("__property__")),))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup1_Tup2_Int_Str((Str::from_static("8.4"), Str::from_static("public(set)"), ((329i64, Str::from_static("public(set)")),))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup4_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("8.3"), Str::from_static("public(set)"), ((326i64, Str::from_static("public")), (ord(&Str::from_static("(")), Str::from_static("(")), (262i64, Str::from_static("set")), (ord(&Str::from_static(")")), Str::from_static(")"))))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup1_Tup2_Int_Str((Str::from_static("8.5"), Str::from_static("|>"), ((408i64, Str::from_static("|>")),))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("8.4"), Str::from_static("|>"), ((ord(&Str::from_static("|")), Str::from_static("|")), (ord(&Str::from_static(">")), Str::from_static(">"))))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup1_Tup2_Int_Str((Str::from_static("8.5"), Str::from_static("(void)"), ((388i64, Str::from_static("(void)")),))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup1_Tup2_Int_Str((Str::from_static("8.5"), Str::from_static("( \tvoid \t)"), ((388i64, Str::from_static("( \tvoid \t)")),))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup3_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("8.4"), Str::from_static("(void)"), ((ord(&Str::from_static("(")), Str::from_static("(")), (262i64, Str::from_static("void")), (ord(&Str::from_static(")")), Str::from_static(")"))))), U_Tup3_Str_Str_Tup1_Tup2_Int_Str_or_Tup3_Str_Str_Tup2_Tup2_Int_Str_Tup_60a2aa96e3::Tup3_Str_Str_Tup3_Tup2_Int_Str_Tup2_Int_Str_Tup2_Int_Str((Str::from_static("8.4"), Str::from_static("( \tVOID \t)"), ((ord(&Str::from_static("(")), Str::from_static("(")), (262i64, Str::from_static("VOID")), (ord(&Str::from_static(")")), Str::from_static(")")))))]);
    }
    pub fn testError(&self, mut code: Mixed, mut messages: Mixed) -> Result<(), Throw> { cast::<crate::php_parser::LexerTest>(self.clone()).testError__impl(code, messages) }
    pub fn provideTestError() -> Result<((Str, (Str,)), (Str, (Str,)), (Str, (Str,)), (Str, (Str,)), (Str, (Str,)), (Str, (Str, Str, Str))), Throw> { crate::php_parser::LexerTest::provideTestError() }
    pub fn testDefaultErrorHandler(&self) -> Result<(), Throw> { cast::<crate::php_parser::LexerTest>(self.clone()).testDefaultErrorHandler__impl() }
    pub fn testLex(&self, mut code: Mixed, mut expectedTokens: Mixed) -> Result<(), Throw> { cast::<crate::php_parser::LexerTest>(self.clone()).testLex__impl(code, expectedTokens) }
    pub fn provideTestLex() -> Result<((Str, ((i64, Str), (i64, Str), (i64, Str), (i64, Str), (i64, Str))), (Str, ((i64, Str), (i64, Str), (i64, Str), (i64, Str), (i64, Str)))), Throw> { crate::php_parser::LexerTest::provideTestLex() }
    pub fn testGetTokens(&self) -> Result<(), Throw> { cast::<crate::php_parser::LexerTest>(self.clone()).testGetTokens__impl() }
    pub fn magic__construct(&self, mut name: Str) -> Result<Mixed, Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).magic__construct__impl(name) }
    pub fn setUpBeforeClass() -> Result<(), Throw> { crate::phpunit::framework::TestCase::setUpBeforeClass() }
    pub fn tearDownAfterClass() -> Result<(), Throw> { crate::phpunit::framework::TestCase::tearDownAfterClass() }
    pub fn setUp(&self) -> Result<(), Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).setUp__impl() }
    pub fn tearDown(&self) -> Result<(), Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).tearDown__impl() }
    pub fn runSetUp(&self) -> Result<(), Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).runSetUp__impl() }
    pub fn runTearDown(&self) -> Result<(), Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).runTearDown__impl() }
    pub fn getName(&self) -> Result<Str, Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).getName__impl() }
    pub fn name(&self) -> Result<Str, Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).name__impl() }
    pub fn expectException(&self, mut exception: Str) -> Result<(), Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).expectException__impl(exception) }
    pub fn expectExceptionMessage(&self, mut message_v: Str) -> Result<(), Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).expectExceptionMessage__impl(message_v) }
    pub fn expectExceptionMessageMatches(&self, mut regularExpression: Str) -> Result<(), Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).expectExceptionMessageMatches__impl(regularExpression) }
    pub fn expectExceptionCode(&self, mut code: i64) -> Result<(), Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).expectExceptionCode__impl(code) }
    pub fn expectNotToPerformAssertions(&self) -> Result<(), Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).expectNotToPerformAssertions__impl() }
    pub fn expectsException(&self) -> Result<bool, Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).expectsException__impl() }
    pub fn verifyExpectedException(&self, mut e: crate::g::Throwable) -> Result<(), Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).verifyExpectedException__impl(e) }
    pub fn expectedExceptionDescription(&self) -> Result<Str, Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).expectedExceptionDescription__impl() }
    pub fn markTestSkipped(&self, mut message_v: Str) -> Result<Never, Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).markTestSkipped__impl(message_v) }
    pub fn markTestIncomplete(&self, mut message_v: Str) -> Result<Never, Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).markTestIncomplete__impl(message_v) }
    pub fn createMock(&self, mut originalClassName: Str) -> Result<Mixed, Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).createMock__impl(originalClassName) }
    pub fn createStub(&self, mut originalClassName: Str) -> Result<Mixed, Throw> { cast::<crate::phpunit::framework::TestCase>(self.clone()).createStub__impl(originalClassName) }
    pub fn getCount() -> Result<i64, Throw> { crate::phpunit::framework::Assert::getCount() }
    pub fn resetCount() -> Result<(), Throw> { crate::phpunit::framework::Assert::resetCount() }
    pub fn fail(mut message_v: Str) -> Result<Never, Throw> { crate::phpunit::framework::Assert::fail(message_v) }
    pub fn assertTrue(mut condition: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertTrue(condition, message_v) }
    pub fn assertFalse(mut condition: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertFalse(condition, message_v) }
    pub fn assertNull(mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertNull(actual, message_v) }
    pub fn assertNotNull(mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertNotNull(actual, message_v) }
    pub fn assertSame(mut expected: Mixed, mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertSame(expected, actual, message_v) }
    pub fn assertNotSame(mut expected: Mixed, mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertNotSame(expected, actual, message_v) }
    pub fn assertEquals(mut expected: Mixed, mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertEquals(expected, actual, message_v) }
    pub fn assertNotEquals(mut expected: Mixed, mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertNotEquals(expected, actual, message_v) }
    pub fn assertEqualsCanonicalizing(mut expected: Mixed, mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertEqualsCanonicalizing(expected, actual, message_v) }
    pub fn assertCount(mut expectedCount: i64, mut haystack: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertCount(expectedCount, haystack, message_v) }
    pub fn assertEmpty(mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertEmpty(actual, message_v) }
    pub fn assertNotEmpty(mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertNotEmpty(actual, message_v) }
    pub fn assertInstanceOf(mut expected: Str, mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertInstanceOf(expected, actual, message_v) }
    pub fn assertNotInstanceOf(mut expected: Str, mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertNotInstanceOf(expected, actual, message_v) }
    pub fn assertIsArray(mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertIsArray(actual, message_v) }
    pub fn assertIsString(mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertIsString(actual, message_v) }
    pub fn assertIsInt(mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertIsInt(actual, message_v) }
    pub fn assertIsBool(mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertIsBool(actual, message_v) }
    pub fn assertIsObject(mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertIsObject(actual, message_v) }
    pub fn assertIsCallable(mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertIsCallable(actual, message_v) }
    pub fn assertStringContainsString(mut needle: Str, mut haystack: Str, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertStringContainsString(needle, haystack, message_v) }
    pub fn assertStringNotContainsString(mut needle: Str, mut haystack: Str, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertStringNotContainsString(needle, haystack, message_v) }
    pub fn assertStringStartsWith(mut prefix: Str, mut string: Str, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertStringStartsWith(prefix, string, message_v) }
    pub fn assertStringEndsWith(mut suffix: Str, mut string: Str, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertStringEndsWith(suffix, string, message_v) }
    pub fn assertMatchesRegularExpression(mut pattern: Str, mut string: Str, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertMatchesRegularExpression(pattern, string, message_v) }
    pub fn assertDoesNotMatchRegularExpression(mut pattern: Str, mut string: Str, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertDoesNotMatchRegularExpression(pattern, string, message_v) }
    pub fn assertContains(mut needle: Mixed, mut haystack: Map<ArrayKey, Mixed>, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertContains(needle, haystack, message_v) }
    pub fn assertNotContains(mut needle: Mixed, mut haystack: Map<ArrayKey, Mixed>, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertNotContains(needle, haystack, message_v) }
    pub fn assertArrayHasKey(mut key_v: ArrayKey, mut array: Map<ArrayKey, Mixed>, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertArrayHasKey(key_v, array, message_v) }
    pub fn assertArrayNotHasKey(mut key_v: ArrayKey, mut array: Map<ArrayKey, Mixed>, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertArrayNotHasKey(key_v, array, message_v) }
    pub fn assertGreaterThan(mut expected: Mixed, mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertGreaterThan(expected, actual, message_v) }
    pub fn assertGreaterThanOrEqual(mut expected: Mixed, mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertGreaterThanOrEqual(expected, actual, message_v) }
    pub fn assertLessThan(mut expected: Mixed, mut actual: Mixed, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertLessThan(expected, actual, message_v) }
    pub fn assertFileExists(mut filename: Str, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertFileExists(filename, message_v) }
    pub fn assertFileDoesNotExist(mut filename: Str, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertFileDoesNotExist(filename, message_v) }
    pub fn assertDirectoryExists(mut directory: Str, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertDirectoryExists(directory, message_v) }
    pub fn assertStringEqualsFile(mut expectedFile: Str, mut actualString: Str, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertStringEqualsFile(expectedFile, actualString, message_v) }
    pub fn assertJsonStringEqualsJsonString(mut expectedJson: Str, mut actualJson: Str, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertJsonStringEqualsJsonString(expectedJson, actualJson, message_v) }
    pub fn assertObjectHasProperty(mut propertyName: Str, mut object: AnyObject, mut message_v: Str) -> Result<(), Throw> { crate::phpunit::framework::Assert::assertObjectHasProperty(propertyName, object, message_v) }
    pub fn new_same_class(&self, mut name: Str) -> Result<crate::php_parser::lexer::EmulativeTest, Throw> { Ok(Self::new(name)?) }
}
impl php_rt::PhpObject for EmulativeTest {
    fn class_name(&self) -> &'static str { "PhpParser\\Lexer\\EmulativeTest" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\lexer\\emulativetest", "phpparser\\lexertest", "phpunit\\framework\\testcase", "phpunit\\framework\\assert"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_expectedException_get()) { out.push((Str::from_static("expectedException"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_expectedExceptionMessage_get()) { out.push((Str::from_static("expectedExceptionMessage"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_expectedExceptionMessageRegExp_get()) { out.push((Str::from_static("expectedExceptionMessageRegExp"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_expectedExceptionCode_get()) { out.push((Str::from_static("expectedExceptionCode"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "expectedException" => { self.set_p_expectedException(value.to_option().map(|__m| cast::<Str>(__m))); true }, "expectedExceptionMessage" => { self.set_p_expectedExceptionMessage(value.to_option().map(|__m| cast::<Str>(__m))); true }, "expectedExceptionMessageRegExp" => { self.set_p_expectedExceptionMessageRegExp(value.to_option().map(|__m| cast::<Str>(__m))); true }, "expectedExceptionCode" => { self.set_p_expectedExceptionCode(value.to_option().map(|__m| cast::<i64>(__m))); true }, "name" => { self.set_p_name(cast::<Str>(value)); true }, _ => false } }
}
impl EmulativeTest { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Lexer\\EmulativeTest could not be converted to string"))) } }
impl php_rt::PhpClone for EmulativeTest { fn php_clone(&self) -> Self { let c = EmulativeTest(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for EmulativeTestObj { fn clone(&self) -> Self { EmulativeTestObj { expectedException: self.expectedException.clone(), expectedExceptionMessage: self.expectedExceptionMessage.clone(), expectedExceptionMessageRegExp: self.expectedExceptionMessageRegExp.clone(), expectedExceptionCode: self.expectedExceptionCode.clone(), name: self.name.clone() } } }
impl EmulativeTest {
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
impl php_rt::Truthy for EmulativeTest { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for EmulativeTest { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Lexer\\EmulativeTest")) } }
impl php_rt::Identical for EmulativeTest { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for EmulativeTest { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for EmulativeTest { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for EmulativeTest { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<EmulativeTest> for Mixed { fn cast_to(self) -> EmulativeTest { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::EmulativeTest>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Lexer\\EmulativeTest") } }
impl php_rt::TryDowncast for EmulativeTest { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::lexer::EmulativeTest>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for EmulativeTest { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<EmulativeTest> for AnyObject { fn cast_to(self) -> EmulativeTest { cast::<EmulativeTest>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<EmulativeTest> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\lexer\\emulativetest") } }
impl php_rt::InstanceOf<EmulativeTest> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\lexer\\emulativetest") } }
impl php_rt::InstanceOf<EmulativeTest> for EmulativeTest { fn is_instance(&self) -> bool { true } }
