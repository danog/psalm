use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub struct DiffElemObj {
    pub type_: i64,
    pub old: Mixed,
    pub new_: Mixed,
}
#[derive(Clone)]
pub struct DiffElem(pub Rc<RefCell<DiffElemObj>>);
impl DiffElem {
    pub fn p_type_(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.type_) }
    pub fn p_type__get(&self) -> i64 { self.0.borrow().type_.clone() }
    pub fn p_type__opt(&self) -> Option<i64> { Some(self.0.borrow().type_.clone()) }
    pub fn p_type__mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.type_) }
    pub fn set_p_type_(&self, v: i64) { self.0.borrow_mut().type_ = v; }
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
    pub fn new(mut type_: i64, mut old: Mixed, mut new_v: Mixed) -> Result<DiffElem, Throw> {
        let this = DiffElem(Rc::new(RefCell::new(DiffElemObj {
            type_: Default::default(),
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
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_type__get()) { out.push((Str::from_static("type"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_old_get()) { out.push((Str::from_static("old"), v)); } if let Some(v) = Some(self.p_new__get()) { out.push((Str::from_static("new"), v)); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "type" => { self.set_p_type_(cast::<i64>(value)); true }, "old" => { self.set_p_old(value); true }, "new" => { self.set_p_new_(value); true }, _ => false } }
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
                    result.push(crate::php_parser::internal::DiffElem::new(crate::php_parser::internal::DiffElem::TYPE_KEEP(), old.clone().idx(&to_key(&(x).wrapping_sub(1i64))).clone(), new_v.clone().idx(&to_key(&(y).wrapping_sub(1i64))).clone())?);
                    let _ = { let __t1 = x; x = __t1.wrapping_sub(1); __t1 };
                    let _ = { let __t2 = y; y = __t2.wrapping_sub(1); __t2 };
                }
                if (d == 0i64) {
                    { break 'l1 };
                }
                'l4: loop {
                    if !(x > prevX) { break; }
                    result.push(crate::php_parser::internal::DiffElem::new(crate::php_parser::internal::DiffElem::TYPE_REMOVE(), old.clone().idx(&to_key(&(x).wrapping_sub(1i64))).clone(), Mixed::Null)?);
                    let _ = { let __t3 = x; x = __t3.wrapping_sub(1); __t3 };
                }
                'l5: loop {
                    if !(y > prevY) { break; }
                    result.push(crate::php_parser::internal::DiffElem::new(crate::php_parser::internal::DiffElem::TYPE_ADD(), Mixed::Null, new_v.clone().idx(&to_key(&(y).wrapping_sub(1i64))).clone())?);
                    let _ = { let __t4 = y; y = __t4.wrapping_sub(1); __t4 };
                }
            }
            let _ = { let __t5 = d; d = __t5.wrapping_sub(1); __t5 };
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
                    newDiff.push(diff.clone().idx(&to_key(&i)).clone());
                    { break 'c2 };
                }
                j = i;
                'l3: loop {
                    if !((j < c) && (diff.clone().idx(&to_key(&j)).clone().p_type__get() == crate::php_parser::internal::DiffElem::TYPE_REMOVE())) { break; }
                    let _ = { let __t1 = j; j = __t1.wrapping_add(1); __t1 };
                }
                k = j;
                'l4: loop {
                    if !((k < c) && (diff.clone().idx(&to_key(&k)).clone().p_type__get() == crate::php_parser::internal::DiffElem::TYPE_ADD())) { break; }
                    let _ = { let __t2 = k; k = __t2.wrapping_add(1); __t2 };
                }
                if ((j).wrapping_sub(i) == (k).wrapping_sub(j)) {
                    len_v = (j).wrapping_sub(i);
                    {
                        n = 0i64;
                        'l5: loop {
                            if !((n < len_v)) { break; }
                            'c6: {
                                newDiff.push(crate::php_parser::internal::DiffElem::new(crate::php_parser::internal::DiffElem::TYPE_REPLACE(), diff.clone().idx(&to_key(&(i).wrapping_add(n))).clone().p_old_get(), diff.clone().idx(&to_key(&(j).wrapping_add(n))).clone().p_new__get())?);
                            }
                            let _ = { let __t3 = n; n = __t3.wrapping_add(1); __t3 };
                        }
                    }
                } else {
                    {
                        'l7: loop {
                            if !((i < k)) { break; }
                            'c8: {
                                newDiff.push(diff.clone().idx(&to_key(&i)).clone());
                            }
                            let _ = { let __t4 = i; i = __t4.wrapping_add(1); __t4 };
                        }
                    }
                }
                i = (k).wrapping_sub(1i64);
            }
            let _ = { let __t5 = i; i = __t5.wrapping_add(1); __t5 };
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
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_isEqual_opt() { out.push((Str::from_static("isEqual"), Mixed::Closure(Rc::new({ let __c9 = v; DynCallable::new(2, move |__a: Vec<Mixed>| -> Result<Mixed, DynError> { (|| -> Result<Mixed, Throw> { Ok(cast::<Mixed>(__c9(__a[0].clone(), __a[1].clone())?)) })().map_err(|__e| DynError::Obj(cast::<Mixed>(__e))) }) })))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "isEqual" => { self.set_p_isEqual({ let __c10 = to_callable(&value); Rc::new(move |__p0: Mixed, __p1: Mixed| -> Result<bool, Throw> { Ok(cast::<bool>(__c10.call(vec![__p0, __p1])?)) }) as Rc<dyn Fn(Mixed, Mixed) -> Result<bool, Throw>> }); true }, _ => false } }
}
impl Differ { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Internal\\Differ could not be converted to string"))) } }
impl php_rt::PhpClone for Differ { fn php_clone(&self) -> Self { let c = Differ(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for DifferObj { fn clone(&self) -> Self { DifferObj { isEqual: self.isEqual.clone() } } }
impl Differ {
}
pub struct PrintableNewAnonClassNodeObj {
    pub attributes: Map<Str, Mixed>,
    pub attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>,
    pub flags: i64,
    pub args: Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>,
    pub extends: Option<crate::php_parser::node::Name>,
    pub implements: Map<ArrayKey, crate::php_parser::node::Name>,
    pub stmts: Map<ArrayKey, crate::php_parser::node::Stmt>,
}
#[derive(Clone)]
pub struct PrintableNewAnonClassNode(pub Rc<RefCell<PrintableNewAnonClassNodeObj>>);
impl PrintableNewAnonClassNode {
    pub fn p_attributes(&self) -> Ref<'_, Map<Str, Mixed>> { Ref::map(self.0.borrow(), |o| &o.attributes) }
    pub fn p_attributes_get(&self) -> Map<Str, Mixed> { self.0.borrow().attributes.clone() }
    pub fn p_attributes_opt(&self) -> Option<Map<Str, Mixed>> { Some(self.0.borrow().attributes.clone()) }
    pub fn p_attributes_mut(&self) -> RefMut<'_, Map<Str, Mixed>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attributes) }
    pub fn set_p_attributes(&self, v: Map<Str, Mixed>) { self.0.borrow_mut().attributes = v; }
    pub fn p_attrGroups(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Ref::map(self.0.borrow(), |o| &o.attrGroups) }
    pub fn p_attrGroups_get(&self) -> Map<ArrayKey, crate::php_parser::node::AttributeGroup> { self.0.borrow().attrGroups.clone() }
    pub fn p_attrGroups_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { Some(self.0.borrow().attrGroups.clone()) }
    pub fn p_attrGroups_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::AttributeGroup>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.attrGroups) }
    pub fn set_p_attrGroups(&self, v: Map<ArrayKey, crate::php_parser::node::AttributeGroup>) { self.0.borrow_mut().attrGroups = v; }
    pub fn p_flags(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.flags) }
    pub fn p_flags_get(&self) -> i64 { self.0.borrow().flags.clone() }
    pub fn p_flags_opt(&self) -> Option<i64> { Some(self.0.borrow().flags.clone()) }
    pub fn p_flags_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.flags) }
    pub fn set_p_flags(&self, v: i64) { self.0.borrow_mut().flags = v; }
    pub fn p_args(&self) -> Ref<'_, Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>> { Ref::map(self.0.borrow(), |o| &o.args) }
    pub fn p_args_get(&self) -> Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder> { self.0.borrow().args.clone() }
    pub fn p_args_opt(&self) -> Option<Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>> { Some(self.0.borrow().args.clone()) }
    pub fn p_args_mut(&self) -> RefMut<'_, Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.args) }
    pub fn set_p_args(&self, v: Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>) { self.0.borrow_mut().args = v; }
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
    pub fn p_stmts(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { Ref::map(self.0.borrow(), |o| &o.stmts) }
    pub fn p_stmts_get(&self) -> Map<ArrayKey, crate::php_parser::node::Stmt> { self.0.borrow().stmts.clone() }
    pub fn p_stmts_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::node::Stmt>> { Some(self.0.borrow().stmts.clone()) }
    pub fn p_stmts_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::node::Stmt>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stmts) }
    pub fn set_p_stmts(&self, v: Map<ArrayKey, crate::php_parser::node::Stmt>) { self.0.borrow_mut().stmts = v; }
    pub fn new(mut attrGroups: Map<ArrayKey, crate::php_parser::node::AttributeGroup>, mut flags: i64, mut args: Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>, mut extends: Option<crate::php_parser::node::Name>, mut implements: Map<ArrayKey, crate::php_parser::node::Name>, mut stmts: Map<ArrayKey, crate::php_parser::node::Stmt>, mut attributes: Map<Str, Mixed>) -> Result<PrintableNewAnonClassNode, Throw> {
        let this = PrintableNewAnonClassNode(Rc::new(RefCell::new(PrintableNewAnonClassNodeObj {
            attributes: Default::default(),
            attrGroups: Default::default(),
            flags: Default::default(),
            args: Default::default(),
            extends: Default::default(),
            implements: Default::default(),
            stmts: Default::default(),
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
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_attributes_get()) { out.push((Str::from_static("attributes"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_attrGroups_get()) { out.push((Str::from_static("attrGroups"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_flags_get()) { out.push((Str::from_static("flags"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_args_get()) { out.push((Str::from_static("args"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_extends_get()) { out.push((Str::from_static("extends"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_implements_get()) { out.push((Str::from_static("implements"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_stmts_get()) { out.push((Str::from_static("stmts"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "attributes" => { self.set_p_attributes(cast::<Map<Str, Mixed>>(value)); true }, "attrGroups" => { self.set_p_attrGroups(cast::<Map<ArrayKey, crate::php_parser::node::AttributeGroup>>(value)); true }, "flags" => { self.set_p_flags(cast::<i64>(value)); true }, "args" => { self.set_p_args(cast::<Map<ArrayKey, U_PhpParser_Node_Arg_or_PhpParser_Node_VariadicPlaceholder>>(value)); true }, "extends" => { self.set_p_extends(value.to_option().map(|__m| cast::<crate::php_parser::node::Name>(__m))); true }, "implements" => { self.set_p_implements(cast::<Map<ArrayKey, crate::php_parser::node::Name>>(value)); true }, "stmts" => { self.set_p_stmts(cast::<Map<ArrayKey, crate::php_parser::node::Stmt>>(value)); true }, _ => false } }
}
impl PrintableNewAnonClassNode { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Internal\\PrintableNewAnonClassNode could not be converted to string"))) } }
impl php_rt::PhpClone for PrintableNewAnonClassNode { fn php_clone(&self) -> Self { let c = PrintableNewAnonClassNode(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for PrintableNewAnonClassNodeObj { fn clone(&self) -> Self { PrintableNewAnonClassNodeObj { attributes: self.attributes.clone(), attrGroups: self.attrGroups.clone(), flags: self.flags.clone(), args: self.args.clone(), extends: self.extends.clone(), implements: self.implements.clone(), stmts: self.stmts.clone() } } }
impl PrintableNewAnonClassNode {
}
pub struct TokenStreamObj {
    pub tokens: Map<ArrayKey, crate::php_parser::Token>,
    pub indentMap: Map<ArrayKey, i64>,
}
#[derive(Clone)]
pub struct TokenStream(pub Rc<RefCell<TokenStreamObj>>);
impl TokenStream {
    pub fn p_tokens(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::Token>> { Ref::map(self.0.borrow(), |o| &o.tokens) }
    pub fn p_tokens_get(&self) -> Map<ArrayKey, crate::php_parser::Token> { self.0.borrow().tokens.clone() }
    pub fn p_tokens_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::Token>> { Some(self.0.borrow().tokens.clone()) }
    pub fn p_tokens_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::Token>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.tokens) }
    pub fn set_p_tokens(&self, v: Map<ArrayKey, crate::php_parser::Token>) { self.0.borrow_mut().tokens = v; }
    pub fn p_indentMap(&self) -> Ref<'_, Map<ArrayKey, i64>> { Ref::map(self.0.borrow(), |o| &o.indentMap) }
    pub fn p_indentMap_get(&self) -> Map<ArrayKey, i64> { self.0.borrow().indentMap.clone() }
    pub fn p_indentMap_opt(&self) -> Option<Map<ArrayKey, i64>> { Some(self.0.borrow().indentMap.clone()) }
    pub fn p_indentMap_mut(&self) -> RefMut<'_, Map<ArrayKey, i64>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.indentMap) }
    pub fn set_p_indentMap(&self, v: Map<ArrayKey, i64>) { self.0.borrow_mut().indentMap = v; }
    pub fn new(mut tokens_v: Map<ArrayKey, crate::php_parser::Token>, mut tabWidth: i64) -> Result<TokenStream, Throw> {
        let this = TokenStream(Rc::new(RefCell::new(TokenStreamObj {
            tokens: Default::default(),
            indentMap: Default::default(),
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
                    append(&mut result, text.clone());
                } else {
                    if (indent < 0i64) {
                        append(&mut result, str_replace(&concat(Str::from_static("\n"), str_repeat(&Str::from_static(" "), (indent).wrapping_neg())), &Str::from_static("\n"), &text.clone()));
                    } else if (indent > 0i64) {
                        append(&mut result, str_replace(&Str::from_static("\n"), &concat(Str::from_static("\n"), str_repeat(&Str::from_static(" "), indent)), &text.clone()));
                    } else {
                        append(&mut result, text.clone());
                    }
                }
            }
            let _ = { let __t1 = pos; pos = __t1.wrapping_add(1); __t1 };
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
            newlinePos.set(U_Int_or___unit_False_::Int(strrpos(&content.clone(), &Str::from_static("\n"), 0).unwrap()));
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
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_tokens_get()) { out.push((Str::from_static("tokens"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_indentMap_get()) { out.push((Str::from_static("indentMap"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "tokens" => { self.set_p_tokens(cast::<Map<ArrayKey, crate::php_parser::Token>>(value)); true }, "indentMap" => { self.set_p_indentMap(cast::<Map<ArrayKey, i64>>(value)); true }, _ => false } }
}
impl TokenStream { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\Internal\\TokenStream could not be converted to string"))) } }
impl php_rt::PhpClone for TokenStream { fn php_clone(&self) -> Self { let c = TokenStream(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for TokenStreamObj { fn clone(&self) -> Self { TokenStreamObj { tokens: self.tokens.clone(), indentMap: self.indentMap.clone() } } }
impl TokenStream {
}
impl php_rt::Truthy for DiffElem { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for DiffElem { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Internal\\DiffElem")) } }
impl php_rt::Identical for DiffElem { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
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
impl php_rt::ToStr for Differ { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Internal\\Differ")) } }
impl php_rt::Identical for Differ { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
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
impl php_rt::ToStr for PrintableNewAnonClassNode { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Internal\\PrintableNewAnonClassNode")) } }
impl php_rt::Identical for PrintableNewAnonClassNode { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
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
impl php_rt::ToStr for TokenStream { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\Internal\\TokenStream")) } }
impl php_rt::Identical for TokenStream { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
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
