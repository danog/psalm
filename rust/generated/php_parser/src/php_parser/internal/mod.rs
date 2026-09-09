use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub struct DiffElemObj {
    pub type_: Late<i64>,
    pub old: Mixed,
    pub new_: Mixed,
}
#[derive(Clone)]
pub struct DiffElem(pub Rc<RefCell<DiffElemObj>>);
impl DiffElem {
    pub fn p_type_(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| o.type_.get()) }
    pub fn p_type__get(&self) -> i64 { self.0.borrow().type_.get().clone() }
    pub fn p_type__opt(&self) -> Option<i64> { self.0.borrow().type_.as_option().cloned() }
    pub fn p_type__mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| o.type_.get_or_default_mut()) }
    pub fn set_p_type_(&self, v: i64) { self.0.borrow_mut().type_.set(v); }
    pub fn p_old(&self) -> Ref<'_, Mixed> { Ref::map(self.0.borrow(), |o| &o.old) }
    pub fn p_old_get(&self) -> Mixed { self.0.borrow().old.clone() }
    pub fn p_old_opt(&self) -> Option<Mixed> { Some(self.0.borrow().old.clone()) }
    pub fn p_old_mut(&self) -> RefMut<'_, Mixed> { RefMut::map(self.0.borrow_mut(), |o| &mut o.old) }
    pub fn set_p_old(&self, v: Mixed) { self.0.borrow_mut().old = v; }
    pub fn p_new_(&self) -> Ref<'_, Mixed> { Ref::map(self.0.borrow(), |o| &o.new_) }
    pub fn p_new__get(&self) -> Mixed { self.0.borrow().new_.clone() }
    pub fn p_new__opt(&self) -> Option<Mixed> { Some(self.0.borrow().new_.clone()) }
    pub fn p_new__mut(&self) -> RefMut<'_, Mixed> { RefMut::map(self.0.borrow_mut(), |o| &mut o.new_) }
    pub fn set_p_new_(&self, v: Mixed) { self.0.borrow_mut().new_ = v; }
    pub fn new_uninit() -> DiffElem {
        DiffElem(Rc::new(RefCell::new(DiffElemObj {
            type_: Late::uninit(),
            old: Default::default(),
            new_: Default::default(),
        })))
    }
    pub fn new(mut type_: i64, mut old: Mixed, mut new_v: Mixed) -> Result<DiffElem, Throw> {
        let this = DiffElem(Rc::new(RefCell::new(DiffElemObj {
            type_: Late::uninit(),
            old: Default::default(),
            new_: Default::default(),
        })));
        this.magic__construct(type_, old, new_v)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut type_: i64, mut old: Mixed, mut new_v: Mixed) -> Result<Mixed, Throw> {
    self.set_p_type_(type_);
    self.set_p_old(old.clone());
    self.set_p_new_(new_v.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn new_same_class(&self, mut type_: i64, mut old: Mixed, mut new_v: Mixed) -> Result<crate::php_parser::internal::DiffElem, Throw> { Ok(Self::new(type_, old, new_v)?) }
}
impl php_rt::PhpObject for DiffElem {
    fn class_name(&self) -> &'static str { "PhpParser\\Internal\\DiffElem" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\internal\\diffelem"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_type__opt() { out.push((Str::from_static("type"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_old_get()) { out.push((Str::from_static("old"), v)); } if let Some(v) = Some(self.p_new__get()) { out.push((Str::from_static("new"), v)); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_type__opt() { out.push((Str::from_static("type"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_old_get()) { out.push((Str::from_static("old"), v)); } if let Some(v) = Some(self.p_new__get()) { out.push((Str::from_static("new"), v)); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "type" => { self.set_p_type_(cast::<i64>(value)); true }, "old" => { self.set_p_old(value); true }, "new" => { self.set_p_new_(value); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(2) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Internal\\DiffElem", name)))) } }
}
impl DiffElem { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Internal\\DiffElem could not be converted to string"))) } }
impl php_rt::PhpClone for DiffElem { fn php_clone(&self) -> Self { let c = DiffElem(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for DiffElemObj { fn clone(&self) -> Self { DiffElemObj { type_: self.type_.clone(), old: self.old.clone(), new_: self.new_.clone() } } }
impl DiffElem {
    pub fn TYPE_KEEP() -> i64 { 0i64 }
    pub fn TYPE_REMOVE() -> i64 { 1i64 }
    pub fn TYPE_ADD() -> i64 { 2i64 }
    pub fn TYPE_REPLACE() -> i64 { 3i64 }
}
pub struct DifferObj {
    pub isEqual: Late<Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>>>,
}
#[derive(Clone)]
pub struct Differ(pub Rc<RefCell<DifferObj>>);
impl Differ {
    pub fn p_isEqual(&self) -> Ref<'_, Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>>> { Ref::map(self.0.borrow(), |o| o.isEqual.get()) }
    pub fn p_isEqual_get(&self) -> Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>> { self.0.borrow().isEqual.get().clone() }
    pub fn p_isEqual_opt(&self) -> Option<Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>>> { self.0.borrow().isEqual.as_option().cloned() }
    pub fn p_isEqual_mut(&self) -> RefMut<'_, Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>>> { RefMut::map(self.0.borrow_mut(), |o| o.isEqual.get_mut()) }
    pub fn set_p_isEqual(&self, v: Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>>) { self.0.borrow_mut().isEqual.set(v); }
    pub fn new_uninit() -> Differ {
        Differ(Rc::new(RefCell::new(DifferObj {
            isEqual: Late::uninit(),
        })))
    }
    pub fn new(mut isEqual: Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>>) -> Result<Differ, Throw> {
        let this = Differ(Rc::new(RefCell::new(DifferObj {
            isEqual: Late::uninit(),
        })));
        this.magic__construct(isEqual)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut isEqual: Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>>) -> Result<Mixed, Throw> {
    self.set_p_isEqual(isEqual.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn diff(&self, mut old: Map<ArrayKey, Mixed>, mut new_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, crate::php_parser::internal::DiffElem>, Throw> {
    let mut trace: Map<i64, Map<i64, i64>> = Default::default();
    let mut x: i64 = Default::default();
    let mut y: i64 = Default::default();
    old = cast::<Map<ArrayKey, Mixed>>(array_values_m(&old.clone()));
    new_v = cast::<Map<ArrayKey, Mixed>>(array_values_m(&new_v.clone()));
    let __d1 = self.calculateTrace(cast::<Map<ArrayKey, Mixed>>(cast::<List<Mixed>>(old.clone())), cast::<Map<ArrayKey, Mixed>>(cast::<List<Mixed>>(new_v.clone())))?; trace = __d1.0.clone(); x = __d1.1.clone(); y = __d1.2.clone(); 
    return Ok(self.extractDiff(trace.clone(), x, y, cast::<Map<ArrayKey, Mixed>>(cast::<List<Mixed>>(old.clone())), cast::<Map<ArrayKey, Mixed>>(cast::<List<Mixed>>(new_v.clone())))?);
    }
    pub fn diffWithReplacements(&self, mut old: Map<ArrayKey, Mixed>, mut new_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, crate::php_parser::internal::DiffElem>, Throw> {
    return Ok(self.coalesceReplacements(self.diff(old.clone(), new_v.clone())?)?);
    }
    pub fn calculateTrace(&self, mut old: Map<ArrayKey, Mixed>, mut new_v: Map<ArrayKey, Mixed>) -> Result<(Map<i64, Map<i64, i64>>, i64, i64), Throw> {
    let mut n: i64 = Default::default();
    let mut m: i64 = Default::default();
    let mut max: i64 = Default::default();
    let mut v: Map<i64, Mixed> = Default::default();
    let mut trace: List<Map<i64, Mixed>> = Default::default();
    let mut d: i64 = Default::default();
    let mut k: i64 = Default::default();
    let mut x: Mixed = Default::default();
    let mut y: Mixed = Default::default();
    n = old.clone().count();
    m = new_v.clone().count();
    max = (n).wrapping_add(m);
    v = { let mut __m1: Map<i64, Mixed> = Map::new(); __m1.insert(1i64, cast::<Mixed>(0i64)); __m1 };
    trace = List::<Map<i64, Mixed>>::new();
    {
        d = 0i64;
        'l1: loop {
            if !((d <= max)) { break; }
            'c2: {
                trace.push(v.clone());
                {
                    k = (d).wrapping_neg();
                    'l3: loop {
                        if !((k <= d)) { break; }
                        'c4: {
                            if ((k == (d).wrapping_neg()) || ((!(k == d)) && php_lt(&v.clone().idx(&(k).wrapping_sub(1i64)).clone(), &v.clone().idx(&(k).wrapping_add(1i64)).clone()))) {
                                x = v.clone().idx(&(k).wrapping_add(1i64)).clone();
                            } else {
                                x = num_add(to_num(&v.clone().idx(&(k).wrapping_sub(1i64)).clone()), Num::Int(1i64)).to_mixed();
                            }
                            y = num_sub(to_num(&x.clone()), Num::Int(k)).to_mixed();
                            'l5: loop {
                                if !((php_lt(&x.clone(), &cast::<Mixed>(n)) && php_lt(&y.clone(), &cast::<Mixed>(m))) && (self.p_isEqual_get())(old.clone().idx(&to_key(&x.clone())).clone(), new_v.clone().idx(&to_key(&y.clone())).clone())?) { break; }
                                let _ = { let __t2 = x.clone(); let __n = mixed_inc(&__t2); x = __n.clone(); __t2 };
                                let _ = { let __t3 = y.clone(); let __n = mixed_inc(&__t3); y = __n.clone(); __t3 };
                            }
                            v.insert(k, x.clone());
                            if (php_ge(&x.clone(), &cast::<Mixed>(n)) && php_ge(&y.clone(), &cast::<Mixed>(m))) {
                                return Ok((cast::<Map<i64, Map<i64, i64>>>(trace.clone().map_elems(|v| v.map_values(|v| cast::<i64>(v)))), cast::<i64>(x.clone()), cast::<i64>(y.clone())));
                            }
                        }
                        { let __t4 = 2i64; k = (k).wrapping_add(__t4); }
                    }
                }
            }
            let _ = { let __t5 = d; d = __t5.wrapping_add(1); __t5 };
        }
    }
    return Err(cast::<crate::g::Throwable>(crate::g::Exception::new(Str::from_static("Should not happen"), 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    pub fn extractDiff(&self, mut trace: Map<i64, Map<i64, i64>>, mut x: i64, mut y: i64, mut old: Map<ArrayKey, Mixed>, mut new_v: Map<ArrayKey, Mixed>) -> Result<Map<ArrayKey, crate::php_parser::internal::DiffElem>, Throw> {
    let mut result: List<crate::php_parser::internal::DiffElem> = Default::default();
    let mut d: i64 = Default::default();
    let mut v: Map<i64, i64> = Default::default();
    let mut k: i64 = Default::default();
    let mut prevK: i64 = Default::default();
    let mut prevX: i64 = Default::default();
    let mut prevY: i64 = Default::default();
    result = List::<crate::php_parser::internal::DiffElem>::new();
    {
        d = (trace.clone().count()).wrapping_sub(1i64);
        'l1: loop {
            if !((d >= 0i64)) { break; }
            'c2: {
                v = trace.clone().idx(&d).clone();
                k = (x).wrapping_sub(y);
                if ((k == (d).wrapping_neg()) || ((!(k == d)) && (v.clone().idx(&(k).wrapping_sub(1i64)).clone() < v.clone().idx(&(k).wrapping_add(1i64)).clone()))) {
                    prevK = (k).wrapping_add(1i64);
                } else {
                    prevK = (k).wrapping_sub(1i64);
                }
                prevX = v.clone().idx(&prevK).clone();
                prevY = (prevX).wrapping_sub(prevK);
                'l3: loop {
                    if !((x > prevX) && (y > prevY)) { break; }
                    { let __h1 = crate::php_parser::internal::DiffElem::new(crate::php_parser::internal::DiffElem::TYPE_KEEP(), old.clone().idx(&to_key(&(x).wrapping_sub(1i64))).clone(), new_v.clone().idx(&to_key(&(y).wrapping_sub(1i64))).clone())?; result.push(__h1); }
                    let _ = { let __t2 = x; x = __t2.wrapping_sub(1); __t2 };
                    let _ = { let __t3 = y; y = __t3.wrapping_sub(1); __t3 };
                }
                if (d == 0i64) {
                    { break 'l1 };
                }
                'l4: loop {
                    if !(x > prevX) { break; }
                    { let __h4 = crate::php_parser::internal::DiffElem::new(crate::php_parser::internal::DiffElem::TYPE_REMOVE(), old.clone().idx(&to_key(&(x).wrapping_sub(1i64))).clone(), Mixed::Null)?; result.push(__h4); }
                    let _ = { let __t5 = x; x = __t5.wrapping_sub(1); __t5 };
                }
                'l5: loop {
                    if !(y > prevY) { break; }
                    { let __h6 = crate::php_parser::internal::DiffElem::new(crate::php_parser::internal::DiffElem::TYPE_ADD(), Mixed::Null, new_v.clone().idx(&to_key(&(y).wrapping_sub(1i64))).clone())?; result.push(__h6); }
                    let _ = { let __t7 = y; y = __t7.wrapping_sub(1); __t7 };
                }
            }
            let _ = { let __t8 = d; d = __t8.wrapping_sub(1); __t8 };
        }
    }
    return Ok(cast::<Map<ArrayKey, crate::php_parser::internal::DiffElem>>(array_reverse_l(&result.clone())));
    }
    pub fn coalesceReplacements(&self, mut diff: Map<ArrayKey, crate::php_parser::internal::DiffElem>) -> Result<Map<ArrayKey, crate::php_parser::internal::DiffElem>, Throw> {
    let mut newDiff: List<crate::php_parser::internal::DiffElem> = Default::default();
    let mut c: i64 = Default::default();
    let mut i: i64 = Default::default();
    let mut diffType: i64 = Default::default();
    let mut j: i64 = Default::default();
    let mut k: i64 = Default::default();
    let mut len_v: i64 = Default::default();
    let mut n: i64 = Default::default();
    newDiff = List::<crate::php_parser::internal::DiffElem>::new();
    c = diff.clone().count();
    {
        i = 0i64;
        'l1: loop {
            if !((i < c)) { break; }
            'c2: {
                diffType = diff.clone().idx(&to_key(&i)).clone().p_type__get();
                if (!(diffType == crate::php_parser::internal::DiffElem::TYPE_REMOVE())) {
                    { let __h1 = diff.clone().idx(&to_key(&i)).clone(); newDiff.push(__h1); }
                    { break 'c2 };
                }
                j = i;
                'l3: loop {
                    if !((j < c) && (diff.clone().idx(&to_key(&j)).clone().p_type__get() == crate::php_parser::internal::DiffElem::TYPE_REMOVE())) { break; }
                    let _ = { let __t2 = j; j = __t2.wrapping_add(1); __t2 };
                }
                k = j;
                'l4: loop {
                    if !((k < c) && (diff.clone().idx(&to_key(&k)).clone().p_type__get() == crate::php_parser::internal::DiffElem::TYPE_ADD())) { break; }
                    let _ = { let __t3 = k; k = __t3.wrapping_add(1); __t3 };
                }
                if ((j).wrapping_sub(i) == (k).wrapping_sub(j)) {
                    len_v = (j).wrapping_sub(i);
                    {
                        n = 0i64;
                        'l5: loop {
                            if !((n < len_v)) { break; }
                            'c6: {
                                { let __h4 = crate::php_parser::internal::DiffElem::new(crate::php_parser::internal::DiffElem::TYPE_REPLACE(), diff.clone().idx(&to_key(&(i).wrapping_add(n))).clone().p_old_get(), diff.clone().idx(&to_key(&(j).wrapping_add(n))).clone().p_new__get())?; newDiff.push(__h4); }
                            }
                            let _ = { let __t5 = n; n = __t5.wrapping_add(1); __t5 };
                        }
                    }
                } else {
                    {
                        'l7: loop {
                            if !((i < k)) { break; }
                            'c8: {
                                { let __h6 = diff.clone().idx(&to_key(&i)).clone(); newDiff.push(__h6); }
                            }
                            let _ = { let __t7 = i; i = __t7.wrapping_add(1); __t7 };
                        }
                    }
                }
                i = (k).wrapping_sub(1i64);
            }
            let _ = { let __t8 = i; i = __t8.wrapping_add(1); __t8 };
        }
    }
    return Ok(cast::<Map<ArrayKey, crate::php_parser::internal::DiffElem>>(newDiff.clone()));
    }
    pub fn new_same_class(&self, mut isEqual: Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>>) -> Result<crate::php_parser::internal::Differ, Throw> { Ok(Self::new(isEqual)?) }
}
impl php_rt::PhpObject for Differ {
    fn class_name(&self) -> &'static str { "PhpParser\\Internal\\Differ" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\internal\\differ"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_isEqual_opt() { out.push((Str::from_static("isEqual"), Mixed::Closure(Rc::new({ let __c8 = v; DynCallable::new(2, move |__a: Vec<Mixed>| -> Result<Mixed, DynError> { (|| -> Result<Mixed, Throw> { Ok(cast::<Mixed>(__c8(__a[0].clone(), __a[1].clone())?)) })().map_err(|__e| DynError::Obj(cast::<Mixed>(__e))) }) })))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "isEqual" => { self.set_p_isEqual({ let __c9 = to_callable(&value); Rc::new(move |__p0: Mixed, __p1: Mixed| -> Result<bool, Throw> { Ok(cast::<bool>(__c9.call(vec![__p0, __p1])?)) }) as Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>> }); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => { let __c10 = to_callable(&__a.clone()); Rc::new(move |__p0: Mixed, __p1: Mixed| -> Result<bool, Throw> { Ok(cast::<bool>(__c10.call(vec![__p0, __p1])?)) }) as Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>> }, None => unreachable!("no default for Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>>") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "diff" => { let __r = self.diff((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "diffwithreplacements" => { let __r = self.diffWithReplacements((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "calculatetrace" => { let __r = self.calculateTrace((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let __c11 = __r; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c11.0)); __m.push(cast::<Mixed>(__c11.1)); __m.push(cast::<Mixed>(__c11.2)); Mixed::Arr(__m) }) }, "extractdiff" => { let __r = self.extractDiff((match args.get(0) { Some(__a) => cast::<Map<i64, Map<i64, i64>>>(__a.clone()), None => <Map<i64, Map<i64, i64>>>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(2) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(3) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() }), (match args.get(4) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "coalescereplacements" => { let __r = self.coalesceReplacements((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, crate::php_parser::internal::DiffElem>>(__a.clone()), None => <Map<ArrayKey, crate::php_parser::internal::DiffElem>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Internal\\Differ", name)))) } }
}
impl Differ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Internal\\Differ could not be converted to string"))) } }
impl php_rt::PhpClone for Differ { fn php_clone(&self) -> Self { let c = Differ(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for DifferObj { fn clone(&self) -> Self { DifferObj { isEqual: self.isEqual.clone() } } }
impl Differ {
}
pub struct PrintableNewAnonClassNodeObj {
    pub attributes: Late<Map<Str, Mixed>>,
    pub attrGroups: Late<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>,
    pub flags: Late<i64>,
    pub args: Late<Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>>,
    pub extends: Late<Option<crate::php_parser::node::Name>>,
    pub implements: Late<Map<ArrayKey, crate::php_parser::node::Name>>,
    pub stmts: Late<Map<ArrayKey, crate::php_parser::node::Stmt>>,
}
#[derive(Clone)]
pub struct PrintableNewAnonClassNode(pub Rc<RefCell<PrintableNewAnonClassNodeObj>>);
impl PrintableNewAnonClassNode {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| o.attributes.get()) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.get().clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { self.0.borrow().attributes.as_option().cloned() }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| o.attributes.get_or_default_mut()) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes.set(v); }
    pub fn p_attrGroups(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| o.attrGroups.get()) }
    pub fn p_attrGroups_get(&self) -> Map<ArrayKey, crate::php_parser::node::AttributeGroup> { self.0.borrow().attrGroups.get().clone() }
    pub fn p_attrGroups_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { self.0.borrow().attrGroups.as_option().cloned() }
    pub fn p_attrGroups_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| o.attrGroups.get_or_default_mut()) }
    pub fn set_p_attrGroups(&self, v: Map<ArrayKey, crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attrGroups.set(v); }
    pub fn p_flags(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| o.flags.get()) }
    pub fn p_flags_get(&self) -> i64 { self.0.borrow().flags.get().clone() }
    pub fn p_flags_opt(&self) -> Option<i64> { self.0.borrow().flags.as_option().cloned() }
    pub fn p_flags_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| o.flags.get_or_default_mut()) }
    pub fn set_p_flags(&self, v: i64) { self.0.borrow_mut().flags.set(v); }
    pub fn p_args(&self) -> Ref<'_, Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>> { Ref::map(self.0.borrow(), |o| o.args.get()) }
    pub fn p_args_get(&self) -> Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder> { self.0.borrow().args.get().clone() }
    pub fn p_args_opt(&self) -> Option<Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>> { self.0.borrow().args.as_option().cloned() }
    pub fn p_args_mut(&self) -> RefMut<'_, Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>> { RefMut::map(self.0.borrow_mut(), |o| o.args.get_or_default_mut()) }
    pub fn set_p_args(&self, v: Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>) { self.0.borrow_mut().args.set(v); }
    pub fn p_extends(&self) -> Ref<'_, Option<crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| o.extends.get()) }
    pub fn p_extends_get(&self) -> Option<crate::php_parser::node::Name> { self.0.borrow().extends.get().clone() }
    pub fn p_extends_opt(&self) -> Option<Option<crate::php_parser::node::Name>> { self.0.borrow().extends.as_option().cloned() }
    pub fn p_extends_mut(&self) -> RefMut<'_, Option<crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| o.extends.get_or_default_mut()) }
    pub fn set_p_extends(&self, v: Option<crate::php_parser::node::Name>) { self.0.borrow_mut().extends.set(v); }
    pub fn p_implements(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Name>> { Ref::map(self.0.borrow(), |o| o.implements.get()) }
    pub fn p_implements_get(&self) -> Map<ArrayKey, crate::php_parser::node::Name> { self.0.borrow().implements.get().clone() }
    pub fn p_implements_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Name>> { self.0.borrow().implements.as_option().cloned() }
    pub fn p_implements_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Name>> { RefMut::map(self.0.borrow_mut(), |o| o.implements.get_or_default_mut()) }
    pub fn set_p_implements(&self, v: Map<ArrayKey, crate::php_parser::node::Name>) { self.0.borrow_mut().implements.set(v); }
    pub fn p_stmts(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| o.stmts.get()) }
    pub fn p_stmts_get(&self) -> Map<ArrayKey, crate::php_parser::node::Stmt> { self.0.borrow().stmts.get().clone() }
    pub fn p_stmts_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Stmt>> { self.0.borrow().stmts.as_option().cloned() }
    pub fn p_stmts_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| o.stmts.get_or_default_mut()) }
    pub fn set_p_stmts(&self, v: Map<ArrayKey, crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts.set(v); }
    pub fn new_uninit() -> PrintableNewAnonClassNode {
        PrintableNewAnonClassNode(Rc::new(RefCell::new(PrintableNewAnonClassNodeObj {
            attributes: Late::uninit(),
            attrGroups: Late::uninit(),
            flags: Late::uninit(),
            args: Late::uninit(),
            extends: Late::uninit(),
            implements: Late::uninit(),
            stmts: Late::uninit(),
        })))
    }
    pub fn new(mut attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>, mut flags: i64, mut args: Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>, mut extends: Option<crate::php_parser::node::Name>, mut implements: Map<ArrayKey, crate::php_parser::node::Name>, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<PrintableNewAnonClassNode, Throw> {
        let this = PrintableNewAnonClassNode(Rc::new(RefCell::new(PrintableNewAnonClassNodeObj {
            attributes: Late::uninit(),
            attrGroups: Late::uninit(),
            flags: Late::uninit(),
            args: Late::uninit(),
            extends: Late::uninit(),
            implements: Late::uninit(),
            stmts: Late::uninit(),
        })));
        this.magic__construct(attrGroups, flags, args, extends, implements, stmts, attributes)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>, mut flags: i64, mut args: Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>, mut extends: Option<crate::php_parser::node::Name>, mut implements: Map<ArrayKey, crate::php_parser::node::Name>, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<Mixed, Throw> {
    let _: Mixed = cast::<crate::php_parser::NodeAbstract>(self.clone()).magic__construct__impl(attributes.clone())?;
    self.set_p_attrGroups(attrGroups.clone());
    self.set_p_flags(flags);
    self.set_p_args(args.clone());
    self.set_p_extends(extends.clone());
    self.set_p_implements(implements.clone());
    self.set_p_stmts(stmts.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn fromNewNode(mut newNode: crate::php_parser::node::expr::New_) -> Result<crate::php_parser::internal::PrintableNewAnonClassNode, Throw> {
    let mut class: Late<U_PhpParser_Node_Expr_or_PhpParser_Node_Name_or_PhpParser_Node_Stmt_Class_> = Late::uninit();
    class.set(newNode.clone().p_class_get());
    let _: bool = { if !(is_instance::<crate::php_parser::node::stmt::Class_>(&class.get().clone())) { return Err(Throw::assertion(Str::from_static("assert(Expr_Instanceof)"))); } true };
    return Ok(crate::php_parser::internal::PrintableNewAnonClassNode::new(cast::<crate::php_parser::node::stmt::Class_>(class.get().clone()).p_attrGroups_get(), cast::<crate::php_parser::node::stmt::Class_>(class.get().clone()).p_flags_get(), newNode.clone().p_args_get(), cast::<crate::php_parser::node::stmt::Class_>(class.get().clone()).p_extends_get(), cast::<crate::php_parser::node::stmt::Class_>(class.get().clone()).p_implements_get(), cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(cast::<List<crate::php_parser::node::Stmt>>(cast::<crate::php_parser::node::stmt::Class_>(class.get().clone()).p_stmts_get())), newNode.clone().getAttributes()?.map_entries(|k, v| (cast::<Str>(k), v)))?);
    }
    pub fn getType(&self) -> Result<Str, Throw> {
    return Ok(Str::from_static("Expr_PrintableNewAnonClass"));
    }
    pub fn getSubNodeNames(&self) -> Result<Map<ArrayKey, Mixed>, Throw> {
    return Ok({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.push(cast::<Mixed>(Str::from_static("attrGroups"))); __m1.push(cast::<Mixed>(Str::from_static("flags"))); __m1.push(cast::<Mixed>(Str::from_static("args"))); __m1.push(cast::<Mixed>(Str::from_static("extends"))); __m1.push(cast::<Mixed>(Str::from_static("implements"))); __m1.push(cast::<Mixed>(Str::from_static("stmts"))); __m1 });
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
    pub fn new_same_class(&self, mut attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>, mut flags: i64, mut args: Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>, mut extends: Option<crate::php_parser::node::Name>, mut implements: Map<ArrayKey, crate::php_parser::node::Name>, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<crate::php_parser::internal::PrintableNewAnonClassNode, Throw> { Ok(Self::new(attrGroups, flags, args, extends, implements, stmts, attributes)?) }
}
impl php_rt::PhpObject for PrintableNewAnonClassNode {
    fn class_name(&self) -> &'static str { "PhpParser\\Internal\\PrintableNewAnonClassNode" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\internal\\printablenewanonclassnode", "phpparser\\node\\expr", "phpparser\\nodeabstract", "phpparser\\node", "jsonserializable"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attributes_opt() { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = self.p_attrGroups_opt() { out.push((Str::from_static("attrGroups"), cast::<Mixed>(v))); } if let Some(v) = self.p_flags_opt() { out.push((Str::from_static("flags"), cast::<Mixed>(v))); } if let Some(v) = self.p_args_opt() { out.push((Str::from_static("args"), cast::<Mixed>(v))); } if let Some(v) = self.p_extends_opt() { out.push((Str::from_static("extends"), cast::<Mixed>(v))); } if let Some(v) = self.p_implements_opt() { out.push((Str::from_static("implements"), cast::<Mixed>(v))); } if let Some(v) = self.p_stmts_opt() { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_attrGroups_opt() { out.push((Str::from_static("attrGroups"), cast::<Mixed>(v))); } if let Some(v) = self.p_flags_opt() { out.push((Str::from_static("flags"), cast::<Mixed>(v))); } if let Some(v) = self.p_args_opt() { out.push((Str::from_static("args"), cast::<Mixed>(v))); } if let Some(v) = self.p_extends_opt() { out.push((Str::from_static("extends"), cast::<Mixed>(v))); } if let Some(v) = self.p_implements_opt() { out.push((Str::from_static("implements"), cast::<Mixed>(v))); } if let Some(v) = self.p_stmts_opt() { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "attrGroups" => { self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(value)); true }, "flags" => { self.set_p_flags(cast::<i64>(value)); true }, "args" => { self.set_p_args(cast::<Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>>(value)); true }, "extends" => { self.set_p_extends(value.to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m))); true }, "implements" => { self.set_p_implements(cast::<Map<ArrayKey, crate::php_parser::node::Name>>(value)); true }, "stmts" => { self.set_p_stmts(cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(__a.clone()), None => <Map<ArrayKey, crate::php_parser::node::AttributeGroup>>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(2) { Some(__a) => cast::<Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>>(__a.clone()), None => <Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>>::default() }), (match args.get(3) { Some(__a) => __a.clone().to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m)), None => <Option<crate::php_parser::node::Name>>::default() }), (match args.get(4) { Some(__a) => cast::<Map<ArrayKey, crate::php_parser::node::Name>>(__a.clone()), None => <Map<ArrayKey, crate::php_parser::node::Name>>::default() }), (match args.get(5) { Some(__a) => cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(__a.clone()), None => <Map<ArrayKey, crate::php_parser::node::Stmt>>::default() }), (match args.get(6) { Some(__a) => cast::<Map<Str, Mixed>>(__a.clone()), None => <Map<Str, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "fromnewnode" => { let __r = crate::php_parser::internal::PrintableNewAnonClassNode::fromNewNode((match args.get(0) { Some(__a) => cast::<crate::php_parser::node::expr::New_>(__a.clone()), None => unreachable!("no default for crate::php_parser::node::expr::New_") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettype" => { let __r = self.getType().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getsubnodenames" => { let __r = self.getSubNodeNames().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getline" => { let __r = self.getLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartline" => { let __r = self.getStartLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendline" => { let __r = self.getEndLine().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstarttokenpos" => { let __r = self.getStartTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendtokenpos" => { let __r = self.getEndTokenPos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getstartfilepos" => { let __r = self.getStartFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getendfilepos" => { let __r = self.getEndFilePos().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getcomments" => { let __r = self.getComments().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getdoccomment" => { let __r = self.getDocComment().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setdoccomment" => { let __r = self.setDocComment((match args.get(0) { Some(__a) => cast::<crate::php_parser::comment::Doc>(__a.clone()), None => unreachable!("no default for crate::php_parser::comment::Doc") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setattribute" => { let __r = self.setAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "hasattribute" => { let __r = self.hasAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getattribute" => { let __r = self.getAttribute((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getattributes" => { let __r = self.getAttributes().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "setattributes" => { let __r = self.setAttributes((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "jsonserialize" => { let __r = self.jsonSerialize().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Internal\\PrintableNewAnonClassNode", name)))) } }
}
impl PrintableNewAnonClassNode { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Internal\\PrintableNewAnonClassNode could not be converted to string"))) } }
impl php_rt::PhpClone for PrintableNewAnonClassNode { fn php_clone(&self) -> Self { let c = PrintableNewAnonClassNode(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for PrintableNewAnonClassNodeObj { fn clone(&self) -> Self { PrintableNewAnonClassNodeObj { attributes: self.attributes.clone(), attrGroups: self.attrGroups.clone(), flags: self.flags.clone(), args: self.args.clone(), extends: self.extends.clone(), implements: self.implements.clone(), stmts: self.stmts.clone() } } }
impl PrintableNewAnonClassNode {
}
pub struct TokenStreamObj {
    pub tokens: Late<Map<ArrayKey, crate::php_parser::Token>>,
    pub indentMap: Late<Map<ArrayKey, i64>>,
}
#[derive(Clone)]
pub struct TokenStream(pub Rc<RefCell<TokenStreamObj>>);
impl TokenStream {
    pub fn p_tokens(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::Token>> { Ref::map(self.0.borrow(), |o| o.tokens.get()) }
    pub fn p_tokens_get(&self) -> Map<ArrayKey, crate::php_parser::Token> { self.0.borrow().tokens.get().clone() }
    pub fn p_tokens_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::Token>> { self.0.borrow().tokens.as_option().cloned() }
    pub fn p_tokens_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::Token>> { RefMut::map(self.0.borrow_mut(), |o| o.tokens.get_or_default_mut()) }
    pub fn set_p_tokens(&self, v: Map<ArrayKey, crate::php_parser::Token>) { self.0.borrow_mut().tokens.set(v); }
    pub fn p_indentMap(&self) -> Ref<'_, Map<ArrayKey, i64>> { Ref::map(self.0.borrow(), |o| o.indentMap.get()) }
    pub fn p_indentMap_get(&self) -> Map<ArrayKey, i64> { self.0.borrow().indentMap.get().clone() }
    pub fn p_indentMap_opt(&self) -> Option<Map<ArrayKey, i64>> { self.0.borrow().indentMap.as_option().cloned() }
    pub fn p_indentMap_mut(&self) -> RefMut<'_, Map<ArrayKey, i64>> { RefMut::map(self.0.borrow_mut(), |o| o.indentMap.get_or_default_mut()) }
    pub fn set_p_indentMap(&self, v: Map<ArrayKey, i64>) { self.0.borrow_mut().indentMap.set(v); }
    pub fn new_uninit() -> TokenStream {
        TokenStream(Rc::new(RefCell::new(TokenStreamObj {
            tokens: Late::uninit(),
            indentMap: Late::uninit(),
        })))
    }
    pub fn new(mut tokens_v: Map<ArrayKey, crate::php_parser::Token>, mut tabWidth: i64) -> Result<TokenStream, Throw> {
        let this = TokenStream(Rc::new(RefCell::new(TokenStreamObj {
            tokens: Late::uninit(),
            indentMap: Late::uninit(),
        })));
        this.magic__construct(tokens_v, tabWidth)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>, mut tabWidth: i64) -> Result<Mixed, Throw> {
    self.set_p_tokens(tokens_v.clone());
    self.set_p_indentMap(self.calcIndentMap(tabWidth)?);
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn haveParens(&self, mut startPos: i64, mut endPos: i64) -> Result<bool, Throw> {
    return Ok((self.haveTokenImmediatelyBefore(startPos, cast::<ArrayKey>(Str::from_static("(")))? && self.haveTokenImmediatelyAfter(endPos, cast::<ArrayKey>(Str::from_static(")")))?));
    }
    pub fn haveBraces(&self, mut startPos: i64, mut endPos: i64) -> Result<bool, Throw> {
    return Ok(((self.haveTokenImmediatelyBefore(startPos, cast::<ArrayKey>(Str::from_static("{")))? || self.haveTokenImmediatelyBefore(startPos, cast::<ArrayKey>(401i64))?) && self.haveTokenImmediatelyAfter(endPos, cast::<ArrayKey>(Str::from_static("}")))?));
    }
    pub fn haveTokenImmediatelyBefore(&self, mut pos: i64, mut expectedTokenType: ArrayKey) -> Result<bool, Throw> {
    let mut tokens_v: Map<ArrayKey, crate::php_parser::Token> = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    tokens_v = self.p_tokens_get();
    let _ = { let __t1 = pos; pos = __t1.wrapping_sub(1); __t1 };
    {
        'l1: loop {
            if !((pos >= 0i64)) { break; }
            'c2: {
                token.set(tokens_v.clone().idx(&to_key(&pos)).clone());
                if token.get().clone().is(cast::<U_Int_or_Map_ArrayKey_ArrayKey_or_Str>(expectedTokenType.clone()))? {
                    return Ok(true);
                }
                if (!token.get().clone().isIgnorable()?) {
                    { break 'l1 };
                }
            }
            let _ = { let __t2 = pos; pos = __t2.wrapping_sub(1); __t2 };
        }
    }
    return Ok(false);
    }
    pub fn haveTokenImmediatelyAfter(&self, mut pos: i64, mut expectedTokenType: ArrayKey) -> Result<bool, Throw> {
    let mut tokens_v: Map<ArrayKey, crate::php_parser::Token> = Default::default();
    let mut c: i64 = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    tokens_v = self.p_tokens_get();
    let _ = { let __t1 = pos; pos = __t1.wrapping_add(1); __t1 };
    {
        c = tokens_v.clone().count();
        'l1: loop {
            if !((pos < c)) { break; }
            'c2: {
                token.set(tokens_v.clone().idx(&to_key(&pos)).clone());
                if token.get().clone().is(cast::<U_Int_or_Map_ArrayKey_ArrayKey_or_Str>(expectedTokenType.clone()))? {
                    return Ok(true);
                }
                if (!token.get().clone().isIgnorable()?) {
                    { break 'l1 };
                }
            }
            let _ = { let __t2 = pos; pos = __t2.wrapping_add(1); __t2 };
        }
    }
    return Ok(false);
    }
    pub fn skipLeft(&self, mut pos: i64, mut skipTokenType: U_Int_or_Map_ArrayKey_ArrayKey_or_Str) -> Result<i64, Throw> {
    let mut tokens_v: Map<ArrayKey, crate::php_parser::Token> = Default::default();
    tokens_v = self.p_tokens_get();
    pos = self.skipLeftWhitespace(pos)?;
    if identical(&skipTokenType.clone(), &U_Int_or_Map_ArrayKey_ArrayKey_or_Str::Int(397i64)) {
        return Ok(pos);
    }
    if (!tokens_v.clone().idx(&to_key(&pos)).clone().is(skipTokenType.clone())?) {
        return Err(cast::<crate::g::Throwable>(crate::g::Exception::new(Str::from_static("Encountered unexpected token"), 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    let _ = { let __t1 = pos; pos = __t1.wrapping_sub(1); __t1 };
    return Ok(self.skipLeftWhitespace(pos)?);
    }
    pub fn skipRight(&self, mut pos: i64, mut skipTokenType: U_Int_or_Map_ArrayKey_ArrayKey_or_Str) -> Result<i64, Throw> {
    let mut tokens_v: Map<ArrayKey, crate::php_parser::Token> = Default::default();
    tokens_v = self.p_tokens_get();
    pos = self.skipRightWhitespace(pos)?;
    if identical(&skipTokenType.clone(), &U_Int_or_Map_ArrayKey_ArrayKey_or_Str::Int(397i64)) {
        return Ok(pos);
    }
    if (!tokens_v.clone().idx(&to_key(&pos)).clone().is(skipTokenType.clone())?) {
        return Err(cast::<crate::g::Throwable>(crate::g::Exception::new(Str::from_static("Encountered unexpected token"), 0i64, { let _ = (); None::<crate::g::Throwable> })?));
    }
    let _ = { let __t1 = pos; pos = __t1.wrapping_add(1); __t1 };
    return Ok(self.skipRightWhitespace(pos)?);
    }
    pub fn skipLeftWhitespace(&self, mut pos: i64) -> Result<i64, Throw> {
    let mut tokens_v: Map<ArrayKey, crate::php_parser::Token> = Default::default();
    tokens_v = self.p_tokens_get();
    {
        'l1: loop {
            if !((pos >= 0i64)) { break; }
            'c2: {
                if (!tokens_v.clone().idx(&to_key(&pos)).clone().isIgnorable()?) {
                    { break 'l1 };
                }
            }
            let _ = { let __t1 = pos; pos = __t1.wrapping_sub(1); __t1 };
        }
    }
    return Ok(pos);
    }
    pub fn skipRightWhitespace(&self, mut pos: i64) -> Result<i64, Throw> {
    let mut tokens_v: Map<ArrayKey, crate::php_parser::Token> = Default::default();
    let mut count_v: i64 = Default::default();
    tokens_v = self.p_tokens_get();
    {
        count_v = tokens_v.clone().count();
        'l1: loop {
            if !((pos < count_v)) { break; }
            'c2: {
                if (!tokens_v.clone().idx(&to_key(&pos)).clone().isIgnorable()?) {
                    { break 'l1 };
                }
            }
            let _ = { let __t1 = pos; pos = __t1.wrapping_add(1); __t1 };
        }
    }
    return Ok(pos);
    }
    pub fn findRight(&self, mut pos: i64, mut findTokenType: U_Int_or_Map_ArrayKey_ArrayKey_or_Str) -> Result<i64, Throw> {
    let mut tokens_v: Map<ArrayKey, crate::php_parser::Token> = Default::default();
    let mut count_v: i64 = Default::default();
    tokens_v = self.p_tokens_get();
    {
        count_v = tokens_v.clone().count();
        'l1: loop {
            if !((pos < count_v)) { break; }
            'c2: {
                if tokens_v.clone().idx(&to_key(&pos)).clone().is(findTokenType.clone())? {
                    return Ok(pos);
                }
            }
            let _ = { let __t1 = pos; pos = __t1.wrapping_add(1); __t1 };
        }
    }
    return Ok((1i64).wrapping_neg());
    }
    pub fn haveTokenInRange(&self, mut startPos: i64, mut endPos: i64, mut tokenType: ArrayKey) -> Result<bool, Throw> {
    let mut tokens_v: Map<ArrayKey, crate::php_parser::Token> = Default::default();
    let mut pos: i64 = Default::default();
    tokens_v = self.p_tokens_get();
    {
        pos = startPos;
        'l1: loop {
            if !((pos < endPos)) { break; }
            'c2: {
                if tokens_v.clone().idx(&to_key(&pos)).clone().is(cast::<U_Int_or_Map_ArrayKey_ArrayKey_or_Str>(tokenType.clone()))? {
                    return Ok(true);
                }
            }
            let _ = { let __t1 = pos; pos = __t1.wrapping_add(1); __t1 };
        }
    }
    return Ok(false);
    }
    pub fn haveTagInRange(&self, mut startPos: i64, mut endPos: i64) -> Result<bool, Throw> {
    return Ok((self.haveTokenInRange(startPos, endPos, cast::<ArrayKey>(394i64))? || self.haveTokenInRange(startPos, endPos, cast::<ArrayKey>(396i64))?));
    }
    pub fn getIndentationBefore(&self, mut pos: i64) -> Result<i64, Throw> {
    return Ok(self.p_indentMap_get().idx(&to_key(&pos)).clone());
    }
    pub fn getTokenCode(&self, mut from: i64, mut to: i64, mut indent: i64) -> Result<Str, Throw> {
    let mut tokens_v: Map<ArrayKey, crate::php_parser::Token> = Default::default();
    let mut result: Str = Default::default();
    let mut pos: i64 = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    let mut id_v: i64 = Default::default();
    let mut text: Str = Default::default();
    tokens_v = self.p_tokens_get();
    result = Str::from_static("");
    {
        pos = from;
        'l1: loop {
            if !((pos < to)) { break; }
            'c2: {
                token.set(tokens_v.clone().idx(&to_key(&pos)).clone());
                id_v = token.get().clone().p_id_get();
                text = token.get().clone().p_text_get();
                if ((id_v == 269i64) || (id_v == 268i64)) {
                    { let __t1 = text.clone(); append(&mut result, __t1); }
                } else {
                    if (indent < 0i64) {
                        { let __t2 = str_replace(&concat(Str::from_static("\n"), str_repeat(&Str::from_static(" "), (indent).wrapping_neg())), &Str::from_static("\n"), &text.clone()); append(&mut result, __t2); }
                    } else if (indent > 0i64) {
                        { let __t3 = str_replace(&Str::from_static("\n"), &concat(Str::from_static("\n"), str_repeat(&Str::from_static(" "), indent)), &text.clone()); append(&mut result, __t3); }
                    } else {
                        { let __t4 = text.clone(); append(&mut result, __t4); }
                    }
                }
            }
            let _ = { let __t5 = pos; pos = __t5.wrapping_add(1); __t5 };
        }
    }
    return Ok(result.clone());
    }
    pub fn calcIndentMap(&self, mut tabWidth: i64) -> Result<Map<ArrayKey, i64>, Throw> {
    let mut indentMap: List<i64> = Default::default();
    let mut indent: i64 = Default::default();
    let mut i: ArrayKey = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    let mut content: Str = Default::default();
    let mut newlinePos: Late<U_Int_or___unit_False_> = Late::uninit();
    indentMap = List::<i64>::new();
    indent = 0i64;
    'l1: for __kv1 in self.p_tokens_get().into_iter() {
        i = __kv1.0;
        token.set(__kv1.1);
        indentMap.push(indent);
        if (token.get().clone().p_id_get() == 397i64) {
            content = token.get().clone().p_text_get();
            newlinePos.set((match strrpos(&content.clone(), &Str::from_static("\n"), 0) { Some(__o) => U_Int_or___unit_False_::Int(__o), None => U_Int_or___unit_False_::False }));
            if (!matches!(newlinePos.get().clone(), U_Int_or___unit_False_::False)) {
                indent = self.getIndent(substr(&content.clone(), (cast::<i64>(newlinePos.get().clone())).wrapping_add(1i64), None), tabWidth)?;
            } else if ((identical(&i.clone(), &cast::<ArrayKey>(1i64)) && (self.p_tokens_get().idx(&to_key(&0i64)).clone().p_id_get() == 394i64)) && identical(&str_index(&self.p_tokens_get().idx(&to_key(&0i64)).clone().p_text_get(), (strlen(&self.p_tokens_get().idx(&to_key(&0i64)).clone().p_text_get())).wrapping_sub(1i64)), &Str::from_static("\n"))) {
                indent = self.getIndent(content.clone(), tabWidth)?;
            }
        }
    }
    indentMap.push(indent);
    return Ok(cast::<Map<ArrayKey, i64>>(indentMap.clone()));
    }
    pub fn getIndent(&self, mut ws: Str, mut tabWidth: i64) -> Result<i64, Throw> {
    let mut spaces: i64 = Default::default();
    let mut tabs: i64 = Default::default();
    spaces = substr_count(&ws.clone(), &Str::from_static(" "));
    tabs = substr_count(&ws.clone(), &Str::from_static("\t"));
    let _: bool = { if !((strlen(&ws.clone()) == (spaces).wrapping_add(tabs))) { return Err(Throw::assertion(Str::from_static("assert(Expr_BinaryOp_Identical)"))); } true };
    return Ok((spaces).wrapping_add((tabs).wrapping_mul(tabWidth)));
    }
    pub fn new_same_class(&self, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>, mut tabWidth: i64) -> Result<crate::php_parser::internal::TokenStream, Throw> { Ok(Self::new(tokens_v, tabWidth)?) }
}
impl php_rt::PhpObject for TokenStream {
    fn class_name(&self) -> &'static str { "PhpParser\\Internal\\TokenStream" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\internal\\tokenstream"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_tokens_opt() { out.push((Str::from_static("tokens"), cast::<Mixed>(v))); } if let Some(v) = self.p_indentMap_opt() { out.push((Str::from_static("indentMap"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "tokens" => { self.set_p_tokens(cast::<Map<ArrayKey, crate::php_parser::Token>>(value)); true }, "indentMap" => { self.set_p_indentMap(cast::<Map<ArrayKey, i64>>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, crate::php_parser::Token>>(__a.clone()), None => <Map<ArrayKey, crate::php_parser::Token>>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "haveparens" => { let __r = self.haveParens((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "havebraces" => { let __r = self.haveBraces((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "havetokenimmediatelybefore" => { let __r = self.haveTokenImmediatelyBefore((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(1) { Some(__a) => cast::<ArrayKey>(__a.clone()), None => <ArrayKey>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "havetokenimmediatelyafter" => { let __r = self.haveTokenImmediatelyAfter((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(1) { Some(__a) => cast::<ArrayKey>(__a.clone()), None => <ArrayKey>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "skipleft" => { let __r = self.skipLeft((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(1) { Some(__a) => cast::<U_Int_or_Map_ArrayKey_ArrayKey_or_Str>(__a.clone()), None => unreachable!("no default for U_Int_or_Map_ArrayKey_ArrayKey_or_Str") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "skipright" => { let __r = self.skipRight((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(1) { Some(__a) => cast::<U_Int_or_Map_ArrayKey_ArrayKey_or_Str>(__a.clone()), None => unreachable!("no default for U_Int_or_Map_ArrayKey_ArrayKey_or_Str") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "skipleftwhitespace" => { let __r = self.skipLeftWhitespace((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "skiprightwhitespace" => { let __r = self.skipRightWhitespace((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "findright" => { let __r = self.findRight((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(1) { Some(__a) => cast::<U_Int_or_Map_ArrayKey_ArrayKey_or_Str>(__a.clone()), None => unreachable!("no default for U_Int_or_Map_ArrayKey_ArrayKey_or_Str") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "havetokeninrange" => { let __r = self.haveTokenInRange((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(2) { Some(__a) => cast::<ArrayKey>(__a.clone()), None => <ArrayKey>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "havetaginrange" => { let __r = self.haveTagInRange((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getindentationbefore" => { let __r = self.getIndentationBefore((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "gettokencode" => { let __r = self.getTokenCode((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(2) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "calcindentmap" => { let __r = self.calcIndentMap((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "getindent" => { let __r = self.getIndent((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Internal\\TokenStream", name)))) } }
}
impl TokenStream { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Internal\\TokenStream could not be converted to string"))) } }
impl php_rt::PhpClone for TokenStream { fn php_clone(&self) -> Self { let c = TokenStream(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for TokenStreamObj { fn clone(&self) -> Self { TokenStreamObj { tokens: self.tokens.clone(), indentMap: self.indentMap.clone() } } }
impl TokenStream {
}
pub struct DifferTestObj {
    pub expectedException: Option<Str>,
    pub expectedExceptionMessage: Option<Str>,
    pub expectedExceptionMessageRegExp: Option<Str>,
    pub expectedExceptionCode: Option<i64>,
    pub name: Str,
}
#[derive(Clone)]
pub struct DifferTest(pub Rc<RefCell<DifferTestObj>>);
impl DifferTest {
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
    pub fn new_uninit() -> DifferTest {
        DifferTest(Rc::new(RefCell::new(DifferTestObj {
            expectedException: { let _ = (); None::<Str> },
            expectedExceptionMessage: { let _ = (); None::<Str> },
            expectedExceptionMessageRegExp: { let _ = (); None::<Str> },
            expectedExceptionCode: { let _ = (); None::<i64> },
            name: Str::from_static(""),
        })))
    }
    pub fn new(mut name: Str) -> Result<DifferTest, Throw> {
        let this = DifferTest(Rc::new(RefCell::new(DifferTestObj {
            expectedException: { let _ = (); None::<Str> },
            expectedExceptionMessage: { let _ = (); None::<Str> },
            expectedExceptionMessageRegExp: { let _ = (); None::<Str> },
            expectedExceptionCode: { let _ = (); None::<i64> },
            name: Str::from_static(""),
        })));
        this.magic__construct(name)?;
        Ok(this)
    }
    pub fn formatDiffString(&self, mut diff: Map<ArrayKey, Mixed>) -> Result<Str, Throw> {
    let mut diffStr: Str = Default::default();
    let mut diffElem: Mixed = Default::default();
    diffStr = Str::from_static("");
    'l1: for __kv1 in diff.clone().into_iter() {
        diffElem = __kv1.1;
        'sw2: {
            let __sw2 = cast::<Mixed>(mixed_prop(&diffElem.clone(), &Str::from_static("type")));
            let __idx3: usize = if loose_eq(&__sw2.clone(), &cast::<Mixed>(crate::php_parser::internal::DiffElem::TYPE_KEEP())) { 0 } else if loose_eq(&__sw2.clone(), &cast::<Mixed>(crate::php_parser::internal::DiffElem::TYPE_REMOVE())) { 1 } else if loose_eq(&__sw2.clone(), &cast::<Mixed>(crate::php_parser::internal::DiffElem::TYPE_ADD())) { 2 } else if loose_eq(&__sw2.clone(), &cast::<Mixed>(crate::php_parser::internal::DiffElem::TYPE_REPLACE())) { 3 } else { 4 };
            if __idx3 <= 0 {
                { let __t4 = cast::<Str>(cast::<Mixed>(mixed_prop(&diffElem.clone(), &Str::from_static("old")))); append(&mut diffStr, __t4); }
                { break 'sw2 };
            }
            if __idx3 <= 1 {
                { let __t5 = concat(Str::from_static("-"), cast::<Str>(cast::<Mixed>(mixed_prop(&diffElem.clone(), &Str::from_static("old"))))); append(&mut diffStr, __t5); }
                { break 'sw2 };
            }
            if __idx3 <= 2 {
                { let __t6 = concat(Str::from_static("+"), cast::<Str>(cast::<Mixed>(mixed_prop(&diffElem.clone(), &Str::from_static("new"))))); append(&mut diffStr, __t6); }
                { break 'sw2 };
            }
            if __idx3 <= 3 {
                { let __t7 = concat(concat(Str::from_static("/"), cast::<Str>(cast::<Mixed>(mixed_prop(&diffElem.clone(), &Str::from_static("old"))))), cast::<Str>(cast::<Mixed>(mixed_prop(&diffElem.clone(), &Str::from_static("new"))))); append(&mut diffStr, __t7); }
                { break 'sw2 };
            }
            if __idx3 <= 4 {
                let _: bool = { if !(false) { return Err(Throw::assertion(Str::from_static("assert(Expr_ConstFetch)"))); } true };
                { break 'sw2 };
            }
        }
    }
    return Ok(diffStr.clone());
    }
    pub fn testDiff(&self, mut oldStr: Mixed, mut newStr: Mixed, mut expectedDiffStr: Mixed) -> Result<(), Throw> {
    let mut differ: Late<crate::php_parser::internal::Differ> = Late::uninit();
    let mut diff: Map<ArrayKey, crate::php_parser::internal::DiffElem> = Default::default();
    differ.set(crate::php_parser::internal::Differ::new({ let this = self.clone(); Rc::new(move |mut a: Mixed, mut b: Mixed| -> Result<bool, Throw> { 
    return Ok(identical(&a.clone(), &b.clone()));
    }) as Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>> })?);
    diff = differ.get().clone().diff(cast::<Map<ArrayKey, Mixed>>(str_split(&cast::<Str>(oldStr.clone()), 1).map_elems(|v| cast::<Mixed>(v))), cast::<Map<ArrayKey, Mixed>>(str_split(&cast::<Str>(newStr.clone()), 1).map_elems(|v| cast::<Mixed>(v))))?;
    { let _ = self; crate::phpunit::framework::Assert::assertSame(expectedDiffStr.clone(), cast::<Mixed>(self.formatDiffString(diff.clone().map_values(|v| cast::<Mixed>(v)))?), Str::from_static(""))? };
    #[allow(unreachable_code)] Ok(())
    }
    pub fn provideTestDiff() -> Result<((Str, Str, Str), (Str, Str, Str), (Str, Str, Str), (Str, Str, Str), (Str, Str, Str), (Str, Str, Str), (Str, Str, Str)), Throw> {
    return Ok(((Str::from_static("abc"), Str::from_static("abc"), Str::from_static("abc")), (Str::from_static("abc"), Str::from_static("abcdef"), Str::from_static("abc+d+e+f")), (Str::from_static("abcdef"), Str::from_static("abc"), Str::from_static("abc-d-e-f")), (Str::from_static("abcdef"), Str::from_static("abcxyzdef"), Str::from_static("abc+x+y+zdef")), (Str::from_static("axyzb"), Str::from_static("ab"), Str::from_static("a-x-y-zb")), (Str::from_static("abcdef"), Str::from_static("abxyef"), Str::from_static("ab-c-d+x+yef")), (Str::from_static("abcdef"), Str::from_static("cdefab"), Str::from_static("-a-bcdef+a+b"))));
    }
    pub fn testDiffWithReplacements(&self, mut oldStr: Mixed, mut newStr: Mixed, mut expectedDiffStr: Mixed) -> Result<(), Throw> {
    let mut differ: Late<crate::php_parser::internal::Differ> = Late::uninit();
    let mut diff: Map<ArrayKey, crate::php_parser::internal::DiffElem> = Default::default();
    differ.set(crate::php_parser::internal::Differ::new({ let this = self.clone(); Rc::new(move |mut a: Mixed, mut b: Mixed| -> Result<bool, Throw> { 
    return Ok(identical(&a.clone(), &b.clone()));
    }) as Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>> })?);
    diff = differ.get().clone().diffWithReplacements(cast::<Map<ArrayKey, Mixed>>(str_split(&cast::<Str>(oldStr.clone()), 1).map_elems(|v| cast::<Mixed>(v))), cast::<Map<ArrayKey, Mixed>>(str_split(&cast::<Str>(newStr.clone()), 1).map_elems(|v| cast::<Mixed>(v))))?;
    { let _ = self; crate::phpunit::framework::Assert::assertSame(expectedDiffStr.clone(), cast::<Mixed>(self.formatDiffString(diff.clone().map_values(|v| cast::<Mixed>(v)))?), Str::from_static(""))? };
    #[allow(unreachable_code)] Ok(())
    }
    pub fn provideTestDiffWithReplacements() -> Result<((Str, Str, Str), (Str, Str, Str), (Str, Str, Str), (Str, Str, Str)), Throw> {
    return Ok(((Str::from_static("abcde"), Str::from_static("axyze"), Str::from_static("a/bx/cy/dze")), (Str::from_static("abcde"), Str::from_static("xbcdy"), Str::from_static("/axbcd/ey")), (Str::from_static("abcde"), Str::from_static("axye"), Str::from_static("a-b-c-d+x+ye")), (Str::from_static("abcde"), Str::from_static("axyzue"), Str::from_static("a-b-c-d+x+y+z+ue"))));
    }
    pub fn testNonContiguousIndices(&self) -> Result<(), Throw> {
    let mut differ: Late<crate::php_parser::internal::Differ> = Late::uninit();
    let mut diff: Map<ArrayKey, crate::php_parser::internal::DiffElem> = Default::default();
    differ.set(crate::php_parser::internal::Differ::new({ let this = self.clone(); Rc::new(move |mut a: Mixed, mut b: Mixed| -> Result<bool, Throw> { 
    return Ok(identical(&a.clone(), &b.clone()));
    }) as Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>> })?);
    diff = differ.get().clone().diff({ let mut __m1: Map<ArrayKey, Mixed> = Map::new(); __m1.insert(to_key(&0i64), cast::<Mixed>(Str::from_static("a"))); __m1.insert(to_key(&2i64), cast::<Mixed>(Str::from_static("b"))); __m1 }, { let mut __m2: Map<ArrayKey, Mixed> = Map::new(); __m2.insert(to_key(&0i64), cast::<Mixed>(Str::from_static("a"))); __m2.insert(to_key(&3i64), cast::<Mixed>(Str::from_static("b"))); __m2 })?;
    { let _ = self; crate::phpunit::framework::Assert::assertEquals({ let __c1267 = (crate::php_parser::internal::DiffElem::new(crate::php_parser::internal::DiffElem::TYPE_KEEP(), cast::<Mixed>(Str::from_static("a")), cast::<Mixed>(Str::from_static("a")))?, crate::php_parser::internal::DiffElem::new(crate::php_parser::internal::DiffElem::TYPE_KEEP(), cast::<Mixed>(Str::from_static("b")), cast::<Mixed>(Str::from_static("b")))?); let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1267.0)); __m.push(cast::<Mixed>(__c1267.1)); Mixed::Arr(__m) }, cast::<Mixed>(diff.clone()), Str::from_static(""))? };
    #[allow(unreachable_code)] Ok(())
    }
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
    pub fn new_same_class(&self, mut name: Str) -> Result<crate::php_parser::internal::DifferTest, Throw> { Ok(Self::new(name)?) }
}
impl php_rt::PhpObject for DifferTest {
    fn class_name(&self) -> &'static str { "PhpParser\\Internal\\DifferTest" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\internal\\differtest", "phpunit\\framework\\testcase", "phpunit\\framework\\assert"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_expectedException_get()) { out.push((Str::from_static("expectedException"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_expectedExceptionMessage_get()) { out.push((Str::from_static("expectedExceptionMessage"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_expectedExceptionMessageRegExp_get()) { out.push((Str::from_static("expectedExceptionMessageRegExp"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_expectedExceptionCode_get()) { out.push((Str::from_static("expectedExceptionCode"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_name_get()) { out.push((Str::from_static("name"), cast::<Mixed>(v))); } out }
    fn public_props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "expectedException" => { self.set_p_expectedException(value.to_option().map(|__m| cast::<Str>(__m))); true }, "expectedExceptionMessage" => { self.set_p_expectedExceptionMessage(value.to_option().map(|__m| cast::<Str>(__m))); true }, "expectedExceptionMessageRegExp" => { self.set_p_expectedExceptionMessageRegExp(value.to_option().map(|__m| cast::<Str>(__m))); true }, "expectedExceptionCode" => { self.set_p_expectedExceptionCode(value.to_option().map(|__m| cast::<i64>(__m))); true }, "name" => { self.set_p_name(cast::<Str>(value)); true }, _ => false } }
    fn call_method(&self, name: &str, args: Vec<Mixed>) -> Result<Mixed, DynError> { match name { "formatdiffstring" => { let __r = self.formatDiffString((match args.get(0) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "testdiff" => { let __r = self.testDiff((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(2) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "providetestdiff" => { let __r = crate::php_parser::internal::DifferTest::provideTestDiff().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let __c1268 = __r; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push({ let __c1269 = __c1268.0; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1269.0)); __m.push(cast::<Mixed>(__c1269.1)); __m.push(cast::<Mixed>(__c1269.2)); Mixed::Arr(__m) }); __m.push({ let __c1270 = __c1268.1; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1270.0)); __m.push(cast::<Mixed>(__c1270.1)); __m.push(cast::<Mixed>(__c1270.2)); Mixed::Arr(__m) }); __m.push({ let __c1271 = __c1268.2; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1271.0)); __m.push(cast::<Mixed>(__c1271.1)); __m.push(cast::<Mixed>(__c1271.2)); Mixed::Arr(__m) }); __m.push({ let __c1272 = __c1268.3; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1272.0)); __m.push(cast::<Mixed>(__c1272.1)); __m.push(cast::<Mixed>(__c1272.2)); Mixed::Arr(__m) }); __m.push({ let __c1273 = __c1268.4; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1273.0)); __m.push(cast::<Mixed>(__c1273.1)); __m.push(cast::<Mixed>(__c1273.2)); Mixed::Arr(__m) }); __m.push({ let __c1274 = __c1268.5; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1274.0)); __m.push(cast::<Mixed>(__c1274.1)); __m.push(cast::<Mixed>(__c1274.2)); Mixed::Arr(__m) }); __m.push({ let __c1275 = __c1268.6; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1275.0)); __m.push(cast::<Mixed>(__c1275.1)); __m.push(cast::<Mixed>(__c1275.2)); Mixed::Arr(__m) }); Mixed::Arr(__m) }) }, "testdiffwithreplacements" => { let __r = self.testDiffWithReplacements((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(2) { Some(__a) => __a.clone(), None => <Mixed>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "providetestdiffwithreplacements" => { let __r = crate::php_parser::internal::DifferTest::provideTestDiffWithReplacements().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let __c1276 = __r; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push({ let __c1277 = __c1276.0; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1277.0)); __m.push(cast::<Mixed>(__c1277.1)); __m.push(cast::<Mixed>(__c1277.2)); Mixed::Arr(__m) }); __m.push({ let __c1278 = __c1276.1; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1278.0)); __m.push(cast::<Mixed>(__c1278.1)); __m.push(cast::<Mixed>(__c1278.2)); Mixed::Arr(__m) }); __m.push({ let __c1279 = __c1276.2; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1279.0)); __m.push(cast::<Mixed>(__c1279.1)); __m.push(cast::<Mixed>(__c1279.2)); Mixed::Arr(__m) }); __m.push({ let __c1280 = __c1276.3; let mut __m: Map<ArrayKey, Mixed> = Map::new(); __m.push(cast::<Mixed>(__c1280.0)); __m.push(cast::<Mixed>(__c1280.1)); __m.push(cast::<Mixed>(__c1280.2)); Mixed::Arr(__m) }); Mixed::Arr(__m) }) }, "testnoncontiguousindices" => { let __r = self.testNonContiguousIndices().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "__construct" => { let __r = self.magic__construct((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "setupbeforeclass" => { let __r = crate::phpunit::framework::TestCase::setUpBeforeClass().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "teardownafterclass" => { let __r = crate::phpunit::framework::TestCase::tearDownAfterClass().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "setup" => { let __r = self.setUp().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "teardown" => { let __r = self.tearDown().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "runsetup" => { let __r = self.runSetUp().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "runteardown" => { let __r = self.runTearDown().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "getname" => { let __r = self.getName().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "name" => { let __r = self.name().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "expectexception" => { let __r = self.expectException((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "expectexceptionmessage" => { let __r = self.expectExceptionMessage((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "expectexceptionmessagematches" => { let __r = self.expectExceptionMessageMatches((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "expectexceptioncode" => { let __r = self.expectExceptionCode((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "expectnottoperformassertions" => { let __r = self.expectNotToPerformAssertions().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "expectsexception" => { let __r = self.expectsException().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "verifyexpectedexception" => { let __r = self.verifyExpectedException((match args.get(0) { Some(__a) => cast::<crate::g::Throwable>(__a.clone()), None => unreachable!("no default for crate::g::Throwable") })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "expectedexceptiondescription" => { let __r = self.expectedExceptionDescription().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "marktestskipped" => { let __r = self.markTestSkipped((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(never(__r)) }, "marktestincomplete" => { let __r = self.markTestIncomplete((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(never(__r)) }, "createmock" => { let __r = self.createMock((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "createstub" => { let __r = self.createStub((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(__r) }, "getcount" => { let __r = crate::phpunit::framework::Assert::getCount().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(cast::<Mixed>(__r)) }, "resetcount" => { let __r = crate::phpunit::framework::Assert::resetCount().map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "fail" => { let __r = crate::phpunit::framework::Assert::fail((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok(never(__r)) }, "asserttrue" => { let __r = crate::phpunit::framework::Assert::assertTrue((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertfalse" => { let __r = crate::phpunit::framework::Assert::assertFalse((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertnull" => { let __r = crate::phpunit::framework::Assert::assertNull((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertnotnull" => { let __r = crate::phpunit::framework::Assert::assertNotNull((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertsame" => { let __r = crate::phpunit::framework::Assert::assertSame((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertnotsame" => { let __r = crate::phpunit::framework::Assert::assertNotSame((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertequals" => { let __r = crate::phpunit::framework::Assert::assertEquals((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertnotequals" => { let __r = crate::phpunit::framework::Assert::assertNotEquals((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertequalscanonicalizing" => { let __r = crate::phpunit::framework::Assert::assertEqualsCanonicalizing((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertcount" => { let __r = crate::phpunit::framework::Assert::assertCount((match args.get(0) { Some(__a) => cast::<i64>(__a.clone()), None => <i64>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertempty" => { let __r = crate::phpunit::framework::Assert::assertEmpty((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertnotempty" => { let __r = crate::phpunit::framework::Assert::assertNotEmpty((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertinstanceof" => { let __r = crate::phpunit::framework::Assert::assertInstanceOf((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertnotinstanceof" => { let __r = crate::phpunit::framework::Assert::assertNotInstanceOf((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertisarray" => { let __r = crate::phpunit::framework::Assert::assertIsArray((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertisstring" => { let __r = crate::phpunit::framework::Assert::assertIsString((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertisint" => { let __r = crate::phpunit::framework::Assert::assertIsInt((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertisbool" => { let __r = crate::phpunit::framework::Assert::assertIsBool((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertisobject" => { let __r = crate::phpunit::framework::Assert::assertIsObject((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertiscallable" => { let __r = crate::phpunit::framework::Assert::assertIsCallable((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertstringcontainsstring" => { let __r = crate::phpunit::framework::Assert::assertStringContainsString((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertstringnotcontainsstring" => { let __r = crate::phpunit::framework::Assert::assertStringNotContainsString((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertstringstartswith" => { let __r = crate::phpunit::framework::Assert::assertStringStartsWith((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertstringendswith" => { let __r = crate::phpunit::framework::Assert::assertStringEndsWith((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertmatchesregularexpression" => { let __r = crate::phpunit::framework::Assert::assertMatchesRegularExpression((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertdoesnotmatchregularexpression" => { let __r = crate::phpunit::framework::Assert::assertDoesNotMatchRegularExpression((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertcontains" => { let __r = crate::phpunit::framework::Assert::assertContains((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertnotcontains" => { let __r = crate::phpunit::framework::Assert::assertNotContains((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertarrayhaskey" => { let __r = crate::phpunit::framework::Assert::assertArrayHasKey((match args.get(0) { Some(__a) => cast::<ArrayKey>(__a.clone()), None => <ArrayKey>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertarraynothaskey" => { let __r = crate::phpunit::framework::Assert::assertArrayNotHasKey((match args.get(0) { Some(__a) => cast::<ArrayKey>(__a.clone()), None => <ArrayKey>::default() }), (match args.get(1) { Some(__a) => cast::<Map<ArrayKey, Mixed>>(__a.clone()), None => <Map<ArrayKey, Mixed>>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertgreaterthan" => { let __r = crate::phpunit::framework::Assert::assertGreaterThan((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertgreaterthanorequal" => { let __r = crate::phpunit::framework::Assert::assertGreaterThanOrEqual((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertlessthan" => { let __r = crate::phpunit::framework::Assert::assertLessThan((match args.get(0) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(1) { Some(__a) => __a.clone(), None => <Mixed>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertfileexists" => { let __r = crate::phpunit::framework::Assert::assertFileExists((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertfiledoesnotexist" => { let __r = crate::phpunit::framework::Assert::assertFileDoesNotExist((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertdirectoryexists" => { let __r = crate::phpunit::framework::Assert::assertDirectoryExists((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertstringequalsfile" => { let __r = crate::phpunit::framework::Assert::assertStringEqualsFile((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertjsonstringequalsjsonstring" => { let __r = crate::phpunit::framework::Assert::assertJsonStringEqualsJsonString((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, "assertobjecthasproperty" => { let __r = crate::phpunit::framework::Assert::assertObjectHasProperty((match args.get(0) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() }), (match args.get(1) { Some(__a) => cast::<AnyObject>(__a.clone()), None => unreachable!("no default for AnyObject") }), (match args.get(2) { Some(__a) => cast::<Str>(__a.clone()), None => <Str>::default() })).map_err(|e| DynError::Obj(cast::<Mixed>(e)))?; Ok({ let _ = __r; Mixed::Null }) }, _ => Err(DynError::Rt(RtError::error(format!("Call to undefined method {}::{}()", "PhpParser\\Internal\\DifferTest", name)))) } }
}
impl DifferTest { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Internal\\DifferTest could not be converted to string"))) } }
impl php_rt::PhpClone for DifferTest { fn php_clone(&self) -> Self { let c = DifferTest(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for DifferTestObj { fn clone(&self) -> Self { DifferTestObj { expectedException: self.expectedException.clone(), expectedExceptionMessage: self.expectedExceptionMessage.clone(), expectedExceptionMessageRegExp: self.expectedExceptionMessageRegExp.clone(), expectedExceptionCode: self.expectedExceptionCode.clone(), name: self.name.clone() } } }
impl DifferTest {
}
impl php_rt::Truthy for DiffElem { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for DiffElem { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for DiffElem { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Internal\\DiffElem")) } }
impl php_rt::PhpCmp for DiffElem { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for DiffElem { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for DiffElem { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<DiffElem> for Mixed { fn cast_to(self) -> DiffElem { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::internal::DiffElem>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Internal\\DiffElem") } }
impl php_rt::TryDowncast for DiffElem { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::internal::DiffElem>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for DiffElem { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<DiffElem> for AnyObject { fn cast_to(self) -> DiffElem { cast::<DiffElem>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<DiffElem> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\internal\\diffelem") } }
impl php_rt::InstanceOf<DiffElem> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\internal\\diffelem") } }
impl php_rt::InstanceOf<DiffElem> for DiffElem { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for Differ { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for Differ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for Differ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Internal\\Differ")) } }
impl php_rt::PhpCmp for Differ { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for Differ { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for Differ { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<Differ> for Mixed { fn cast_to(self) -> Differ { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::internal::Differ>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Internal\\Differ") } }
impl php_rt::TryDowncast for Differ { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::internal::Differ>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for Differ { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<Differ> for AnyObject { fn cast_to(self) -> Differ { cast::<Differ>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<Differ> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\internal\\differ") } }
impl php_rt::InstanceOf<Differ> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\internal\\differ") } }
impl php_rt::InstanceOf<Differ> for Differ { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for PrintableNewAnonClassNode { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for PrintableNewAnonClassNode { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for PrintableNewAnonClassNode { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Internal\\PrintableNewAnonClassNode")) } }
impl php_rt::PhpCmp for PrintableNewAnonClassNode { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for PrintableNewAnonClassNode { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for PrintableNewAnonClassNode { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<PrintableNewAnonClassNode> for Mixed { fn cast_to(self) -> PrintableNewAnonClassNode { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::internal::PrintableNewAnonClassNode>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Internal\\PrintableNewAnonClassNode") } }
impl php_rt::TryDowncast for PrintableNewAnonClassNode { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::internal::PrintableNewAnonClassNode>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for PrintableNewAnonClassNode { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<PrintableNewAnonClassNode> for AnyObject { fn cast_to(self) -> PrintableNewAnonClassNode { cast::<PrintableNewAnonClassNode>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<PrintableNewAnonClassNode> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\internal\\printablenewanonclassnode") } }
impl php_rt::InstanceOf<PrintableNewAnonClassNode> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\internal\\printablenewanonclassnode") } }
impl php_rt::InstanceOf<PrintableNewAnonClassNode> for PrintableNewAnonClassNode { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for TokenStream { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for TokenStream { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for TokenStream { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Internal\\TokenStream")) } }
impl php_rt::PhpCmp for TokenStream { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for TokenStream { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for TokenStream { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<TokenStream> for Mixed { fn cast_to(self) -> TokenStream { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::internal::TokenStream>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Internal\\TokenStream") } }
impl php_rt::TryDowncast for TokenStream { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::internal::TokenStream>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for TokenStream { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<TokenStream> for AnyObject { fn cast_to(self) -> TokenStream { cast::<TokenStream>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<TokenStream> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\internal\\tokenstream") } }
impl php_rt::InstanceOf<TokenStream> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\internal\\tokenstream") } }
impl php_rt::InstanceOf<TokenStream> for TokenStream { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for DifferTest { fn truthy(&self) -> bool { true } }
impl php_rt::Identical for DifferTest { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::ToStr for DifferTest { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Internal\\DifferTest")) } }
impl php_rt::PhpCmp for DifferTest { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for DifferTest { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for DifferTest { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<DifferTest> for Mixed { fn cast_to(self) -> DifferTest { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::internal::DifferTest>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\Internal\\DifferTest") } }
impl php_rt::TryDowncast for DifferTest { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::internal::DifferTest>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for DifferTest { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<DifferTest> for AnyObject { fn cast_to(self) -> DifferTest { cast::<DifferTest>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<DifferTest> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\internal\\differtest") } }
impl php_rt::InstanceOf<DifferTest> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\internal\\differtest") } }
impl php_rt::InstanceOf<DifferTest> for DifferTest { fn is_instance(&self) -> bool { true } }
