use php_rt::prelude::*;
use crate::generated::*;
use crate::Throw;
use crate::AnyObject;
pub struct CloningVisitorObj {
}
#[derive(Clone)]
pub struct CloningVisitor(pub Rc<RefCell<CloningVisitorObj>>);
impl CloningVisitor {
    pub fn new() -> Result<CloningVisitor, Throw> {
        let this = CloningVisitor(Rc::new(RefCell::new(CloningVisitorObj {
        })));
        Ok(this)
    }
    pub fn enterNode(&self, mut origNode: crate::php_parser::Node) -> Result<Mixed, Throw> {
    let mut node: Late<crate::php_parser::Node> = Late::uninit();
    node.set(origNode.clone().php_clone());
    node.get().clone().setAttribute(Str::from_static("origNode"), cast::<Mixed>(origNode.clone()))?;
    return Ok(cast::<Mixed>(node.get().clone()));
    }
    pub fn beforeTraverse(&self, mut nodes: Map<ArrayKey, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeVisitorAbstract>(self.clone()).beforeTraverse__impl(nodes) }
    pub fn leaveNode(&self, mut node: crate::php_parser::Node) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeVisitorAbstract>(self.clone()).leaveNode__impl(node) }
    pub fn afterTraverse(&self, mut nodes: Map<ArrayKey, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeVisitorAbstract>(self.clone()).afterTraverse__impl(nodes) }
    pub fn new_same_class(&self) -> Result<crate::php_parser::node_visitor::CloningVisitor, Throw> { Ok(Self::new()?) }
}
impl php_rt::PhpObject for CloningVisitor {
    fn class_name(&self) -> &'static str { "PhpParser\\NodeVisitor\\CloningVisitor" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\nodevisitor\\cloningvisitor", "phpparser\\nodevisitorabstract", "phpparser\\nodevisitor"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new();  out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { _ => false } }
}
impl CloningVisitor { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\NodeVisitor\\CloningVisitor could not be converted to string"))) } }
impl php_rt::PhpClone for CloningVisitor { fn php_clone(&self) -> Self { let c = CloningVisitor(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for CloningVisitorObj { fn clone(&self) -> Self { CloningVisitorObj {  } } }
impl CloningVisitor {
}
pub struct CommentAnnotatingVisitorObj {
    pub pos: i64,
    pub tokens: Map<ArrayKey, crate::php_parser::Token>,
    pub commentPositions: List<i64>,
}
#[derive(Clone)]
pub struct CommentAnnotatingVisitor(pub Rc<RefCell<CommentAnnotatingVisitorObj>>);
impl CommentAnnotatingVisitor {
    pub fn p_pos(&self) -> Ref<'_, i64> { Ref::map(self.0.borrow(), |o| &o.pos) }
    pub fn p_pos_get(&self) -> i64 { self.0.borrow().pos.clone() }
    pub fn p_pos_opt(&self) -> Option<i64> { Some(self.0.borrow().pos.clone()) }
    pub fn p_pos_mut(&self) -> RefMut<'_, i64> { RefMut::map(self.0.borrow_mut(), |o| &mut o.pos) }
    pub fn set_p_pos(&self, v: i64) { self.0.borrow_mut().pos = v; }
    pub fn p_tokens(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::Token>> { Ref::map(self.0.borrow(), |o| &o.tokens) }
    pub fn p_tokens_get(&self) -> Map<ArrayKey, crate::php_parser::Token> { self.0.borrow().tokens.clone() }
    pub fn p_tokens_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::Token>> { Some(self.0.borrow().tokens.clone()) }
    pub fn p_tokens_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::Token>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.tokens) }
    pub fn set_p_tokens(&self, v: Map<ArrayKey, crate::php_parser::Token>) { self.0.borrow_mut().tokens = v; }
    pub fn p_commentPositions(&self) -> Ref<'_, List<i64>> { Ref::map(self.0.borrow(), |o| &o.commentPositions) }
    pub fn p_commentPositions_get(&self) -> List<i64> { self.0.borrow().commentPositions.clone() }
    pub fn p_commentPositions_opt(&self) -> Option<List<i64>> { Some(self.0.borrow().commentPositions.clone()) }
    pub fn p_commentPositions_mut(&self) -> RefMut<'_, List<i64>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.commentPositions) }
    pub fn set_p_commentPositions(&self, v: List<i64>) { self.0.borrow_mut().commentPositions = v; }
    pub fn new(mut tokens_v: Map<ArrayKey, crate::php_parser::Token>) -> Result<CommentAnnotatingVisitor, Throw> {
        let this = CommentAnnotatingVisitor(Rc::new(RefCell::new(CommentAnnotatingVisitorObj {
            pos: 0i64,
            tokens: Default::default(),
            commentPositions: List::<i64>::new(),
        })));
        this.magic__construct(tokens_v)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>) -> Result<Mixed, Throw> {
    let mut i: ArrayKey = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    self.set_p_tokens(tokens_v.clone());
    'l1: for __kv1 in tokens_v.clone().into_iter() {
        i = __kv1.0;
        token.set(__kv1.1);
        if ((token.get().clone().p_id_get() == 392i64) || (token.get().clone().p_id_get() == 393i64)) {
            (*self.p_commentPositions_mut()).push(cast::<i64>(i.clone()));
        }
    }
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn enterNode(&self, mut node: crate::php_parser::Node) -> Result<Mixed, Throw> {
    let mut nextCommentPos: Late<U_Int_or___unit_False_> = Late::uninit();
    let mut oldPos: i64 = Default::default();
    let mut pos: i64 = Default::default();
    let mut endPos: i64 = Default::default();
    let mut comments: List<crate::php_parser::Comment> = Default::default();
    let mut token: Late<crate::php_parser::Token> = Late::uninit();
    nextCommentPos.set(U_Int_or___unit_False_::Int(self.p_commentPositions_get().first().cloned().unwrap()));
    if matches!(nextCommentPos.get().clone(), U_Int_or___unit_False_::False) {
        return Ok(cast::<Mixed>(crate::php_parser::NodeVisitor::STOP_TRAVERSAL()));
    }
    oldPos = self.p_pos_get();
    self.set_p_pos({ let __t1 = node.clone().getStartTokenPos()?; pos = __t1.clone(); __t1 });
    if ((cast::<i64>(nextCommentPos.get().clone()) > oldPos) && (cast::<i64>(nextCommentPos.get().clone()) < pos)) {
        comments = List::<crate::php_parser::Comment>::new();
        'l1: loop {
            if !({ let __t2 = pos; pos = __t2.wrapping_sub(1); __t2.wrapping_sub(1) } >= oldPos) { break; }
            token.set(self.p_tokens_get().idx(&to_key(&pos)).clone());
            if (token.get().clone().p_id_get() == 393i64) {
                comments.push(cast::<crate::php_parser::Comment>(crate::php_parser::comment::Doc::new(token.get().clone().p_text_get(), token.get().clone().p_line_get(), token.get().clone().p_pos_get(), pos, token.get().clone().getEndLine()?, (token.get().clone().getEndPos()?).wrapping_sub(1i64), pos)?));
                { continue 'l1 };
            }
            if (token.get().clone().p_id_get() == 392i64) {
                comments.push(crate::php_parser::Comment::new(token.get().clone().p_text_get(), token.get().clone().p_line_get(), token.get().clone().p_pos_get(), pos, token.get().clone().getEndLine()?, (token.get().clone().getEndPos()?).wrapping_sub(1i64), pos)?);
                { continue 'l1 };
            }
            if (!(token.get().clone().p_id_get() == 397i64)) {
                { break 'l1 };
            }
        }
        if (!(!truthy(&comments.clone().map_elems(|v| U_PhpParser_Comment_or_PhpParser_Comment_Doc::PhpParser_Comment(v))))) {
            node.clone().setAttribute(Str::from_static("comments"), cast::<Mixed>(array_reverse_l(&comments.clone().map_elems(|v| U_PhpParser_Comment_or_PhpParser_Comment_Doc::PhpParser_Comment(v)))))?;
        }
        'l2: loop {
            'c3: {
                nextCommentPos.set(cast::<U_Int_or___unit_False_>(None::<Mixed>.unwrap()));
            }
            if !((!matches!(nextCommentPos.get().clone(), U_Int_or___unit_False_::False)) && (cast::<i64>(nextCommentPos.get().clone()) < self.p_pos_get())) { break; }
        }
    }
    endPos = node.clone().getEndTokenPos()?;
    if php_gt(&nextCommentPos.get().clone(), &U_Int_or___unit_False_::Int(endPos)) {
        self.set_p_pos(endPos);
        return Ok(cast::<Mixed>(crate::php_parser::NodeVisitor::DONT_TRAVERSE_CHILDREN()));
    }
    return Ok(Mixed::Null);
    }
    pub fn beforeTraverse(&self, mut nodes: Map<ArrayKey, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeVisitorAbstract>(self.clone()).beforeTraverse__impl(nodes) }
    pub fn leaveNode(&self, mut node: crate::php_parser::Node) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeVisitorAbstract>(self.clone()).leaveNode__impl(node) }
    pub fn afterTraverse(&self, mut nodes: Map<ArrayKey, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeVisitorAbstract>(self.clone()).afterTraverse__impl(nodes) }
    pub fn new_same_class(&self, mut tokens_v: Map<ArrayKey, crate::php_parser::Token>) -> Result<crate::php_parser::node_visitor::CommentAnnotatingVisitor, Throw> { Ok(Self::new(tokens_v)?) }
}
impl php_rt::PhpObject for CommentAnnotatingVisitor {
    fn class_name(&self) -> &'static str { "PhpParser\\NodeVisitor\\CommentAnnotatingVisitor" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\nodevisitor\\commentannotatingvisitor", "phpparser\\nodevisitorabstract", "phpparser\\nodevisitor"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_pos_get()) { out.push((Str::from_static("pos"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_tokens_get()) { out.push((Str::from_static("tokens"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_commentPositions_get()) { out.push((Str::from_static("commentPositions"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "pos" => { self.set_p_pos(cast::<i64>(value)); true }, "tokens" => { self.set_p_tokens(cast::<Map<ArrayKey, crate::php_parser::Token>>(value)); true }, "commentPositions" => { self.set_p_commentPositions(cast::<List<i64>>(value)); true }, _ => false } }
}
impl CommentAnnotatingVisitor { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\NodeVisitor\\CommentAnnotatingVisitor could not be converted to string"))) } }
impl php_rt::PhpClone for CommentAnnotatingVisitor { fn php_clone(&self) -> Self { let c = CommentAnnotatingVisitor(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for CommentAnnotatingVisitorObj { fn clone(&self) -> Self { CommentAnnotatingVisitorObj { pos: self.pos.clone(), tokens: self.tokens.clone(), commentPositions: self.commentPositions.clone() } } }
impl CommentAnnotatingVisitor {
}
pub struct FindingVisitorObj {
    pub filterCallback: Late<DynCallable>,
    pub foundNodes: List<crate::php_parser::Node>,
}
#[derive(Clone)]
pub struct FindingVisitor(pub Rc<RefCell<FindingVisitorObj>>);
impl FindingVisitor {
    pub fn p_filterCallback(&self) -> Ref<'_, DynCallable> { Ref::map(self.0.borrow(), |o| o.filterCallback.get()) }
    pub fn p_filterCallback_get(&self) -> DynCallable { self.0.borrow().filterCallback.get().clone() }
    pub fn p_filterCallback_opt(&self) -> Option<DynCallable> { self.0.borrow().filterCallback.as_option().cloned() }
    pub fn p_filterCallback_mut(&self) -> RefMut<'_, DynCallable> { RefMut::map(self.0.borrow_mut(), |o| o.filterCallback.get_mut()) }
    pub fn set_p_filterCallback(&self, v: DynCallable) { self.0.borrow_mut().filterCallback.set(v); }
    pub fn p_foundNodes(&self) -> Ref<'_, List<crate::php_parser::Node>> { Ref::map(self.0.borrow(), |o| &o.foundNodes) }
    pub fn p_foundNodes_get(&self) -> List<crate::php_parser::Node> { self.0.borrow().foundNodes.clone() }
    pub fn p_foundNodes_opt(&self) -> Option<List<crate::php_parser::Node>> { Some(self.0.borrow().foundNodes.clone()) }
    pub fn p_foundNodes_mut(&self) -> RefMut<'_, List<crate::php_parser::Node>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.foundNodes) }
    pub fn set_p_foundNodes(&self, v: List<crate::php_parser::Node>) { self.0.borrow_mut().foundNodes = v; }
    pub fn new(mut filterCallback: DynCallable) -> Result<FindingVisitor, Throw> {
        let this = FindingVisitor(Rc::new(RefCell::new(FindingVisitorObj {
            filterCallback: Late::uninit(),
            foundNodes: Default::default(),
        })));
        this.magic__construct(filterCallback)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut filterCallback: DynCallable) -> Result<Mixed, Throw> {
    self.set_p_filterCallback(filterCallback.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getFoundNodes(&self) -> Result<List<crate::php_parser::Node>, Throw> {
    return Ok(self.p_foundNodes_get());
    }
    pub fn beforeTraverse(&self, mut nodes: Map<ArrayKey, Mixed>) -> Result<Option<Map<ArrayKey, Mixed>>, Throw> {
    self.set_p_foundNodes(List::<crate::php_parser::Node>::new());
    return Ok({ let _ = (); None::<Map<ArrayKey, Mixed>> });
    }
    pub fn enterNode(&self, mut node: crate::php_parser::Node) -> Result<Mixed, Throw> {
    let mut filterCallback: Late<DynCallable> = Late::uninit();
    filterCallback.set(self.p_filterCallback_get());
    if truthy(&filterCallback.get().clone().call(vec![cast::<Mixed>(node.clone())])?) {
        (*self.p_foundNodes_mut()).push(node.clone());
    }
    return Ok(Mixed::Null);
    }
    pub fn leaveNode(&self, mut node: crate::php_parser::Node) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeVisitorAbstract>(self.clone()).leaveNode__impl(node) }
    pub fn afterTraverse(&self, mut nodes: Map<ArrayKey, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeVisitorAbstract>(self.clone()).afterTraverse__impl(nodes) }
    pub fn new_same_class(&self, mut filterCallback: DynCallable) -> Result<crate::php_parser::node_visitor::FindingVisitor, Throw> { Ok(Self::new(filterCallback)?) }
}
impl php_rt::PhpObject for FindingVisitor {
    fn class_name(&self) -> &'static str { "PhpParser\\NodeVisitor\\FindingVisitor" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\nodevisitor\\findingvisitor", "phpparser\\nodevisitorabstract", "phpparser\\nodevisitor"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_filterCallback_opt() { out.push((Str::from_static("filterCallback"), Mixed::Closure(Rc::new(v)))); } if let Some(v) = Some(self.p_foundNodes_get()) { out.push((Str::from_static("foundNodes"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "filterCallback" => { self.set_p_filterCallback(to_callable(&value)); true }, "foundNodes" => { self.set_p_foundNodes(cast::<List<crate::php_parser::Node>>(value)); true }, _ => false } }
}
impl FindingVisitor { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\NodeVisitor\\FindingVisitor could not be converted to string"))) } }
impl php_rt::PhpClone for FindingVisitor { fn php_clone(&self) -> Self { let c = FindingVisitor(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for FindingVisitorObj { fn clone(&self) -> Self { FindingVisitorObj { filterCallback: self.filterCallback.clone(), foundNodes: self.foundNodes.clone() } } }
impl FindingVisitor {
}
pub struct FirstFindingVisitorObj {
    pub filterCallback: Late<DynCallable>,
    pub foundNode: Option<crate::php_parser::Node>,
}
#[derive(Clone)]
pub struct FirstFindingVisitor(pub Rc<RefCell<FirstFindingVisitorObj>>);
impl FirstFindingVisitor {
    pub fn p_filterCallback(&self) -> Ref<'_, DynCallable> { Ref::map(self.0.borrow(), |o| o.filterCallback.get()) }
    pub fn p_filterCallback_get(&self) -> DynCallable { self.0.borrow().filterCallback.get().clone() }
    pub fn p_filterCallback_opt(&self) -> Option<DynCallable> { self.0.borrow().filterCallback.as_option().cloned() }
    pub fn p_filterCallback_mut(&self) -> RefMut<'_, DynCallable> { RefMut::map(self.0.borrow_mut(), |o| o.filterCallback.get_mut()) }
    pub fn set_p_filterCallback(&self, v: DynCallable) { self.0.borrow_mut().filterCallback.set(v); }
    pub fn p_foundNode(&self) -> Ref<'_, Option<crate::php_parser::Node>> { Ref::map(self.0.borrow(), |o| &o.foundNode) }
    pub fn p_foundNode_get(&self) -> Option<crate::php_parser::Node> { self.0.borrow().foundNode.clone() }
    pub fn p_foundNode_opt(&self) -> Option<Option<crate::php_parser::Node>> { Some(self.0.borrow().foundNode.clone()) }
    pub fn p_foundNode_mut(&self) -> RefMut<'_, Option<crate::php_parser::Node>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.foundNode) }
    pub fn set_p_foundNode(&self, v: Option<crate::php_parser::Node>) { self.0.borrow_mut().foundNode = v; }
    pub fn new(mut filterCallback: DynCallable) -> Result<FirstFindingVisitor, Throw> {
        let this = FirstFindingVisitor(Rc::new(RefCell::new(FirstFindingVisitorObj {
            filterCallback: Late::uninit(),
            foundNode: Default::default(),
        })));
        this.magic__construct(filterCallback)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut filterCallback: DynCallable) -> Result<Mixed, Throw> {
    self.set_p_filterCallback(filterCallback.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getFoundNode(&self) -> Result<Option<crate::php_parser::Node>, Throw> {
    return Ok(self.p_foundNode_get());
    }
    pub fn beforeTraverse(&self, mut nodes: Map<ArrayKey, Mixed>) -> Result<Option<Map<ArrayKey, Mixed>>, Throw> {
    self.set_p_foundNode({ let _ = (); None::<crate::php_parser::Node> });
    return Ok({ let _ = (); None::<Map<ArrayKey, Mixed>> });
    }
    pub fn enterNode(&self, mut node: crate::php_parser::Node) -> Result<Mixed, Throw> {
    let mut filterCallback: Late<DynCallable> = Late::uninit();
    filterCallback.set(self.p_filterCallback_get());
    if truthy(&filterCallback.get().clone().call(vec![cast::<Mixed>(node.clone())])?) {
        self.set_p_foundNode(Some(node.clone()));
        return Ok(cast::<Mixed>(crate::php_parser::NodeVisitor::STOP_TRAVERSAL()));
    }
    return Ok(Mixed::Null);
    }
    pub fn leaveNode(&self, mut node: crate::php_parser::Node) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeVisitorAbstract>(self.clone()).leaveNode__impl(node) }
    pub fn afterTraverse(&self, mut nodes: Map<ArrayKey, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeVisitorAbstract>(self.clone()).afterTraverse__impl(nodes) }
    pub fn new_same_class(&self, mut filterCallback: DynCallable) -> Result<crate::php_parser::node_visitor::FirstFindingVisitor, Throw> { Ok(Self::new(filterCallback)?) }
}
impl php_rt::PhpObject for FirstFindingVisitor {
    fn class_name(&self) -> &'static str { "PhpParser\\NodeVisitor\\FirstFindingVisitor" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\nodevisitor\\firstfindingvisitor", "phpparser\\nodevisitorabstract", "phpparser\\nodevisitor"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_filterCallback_opt() { out.push((Str::from_static("filterCallback"), Mixed::Closure(Rc::new(v)))); } if let Some(v) = Some(self.p_foundNode_get()) { out.push((Str::from_static("foundNode"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "filterCallback" => { self.set_p_filterCallback(to_callable(&value)); true }, "foundNode" => { self.set_p_foundNode(value.to_option().map(|__m| cast::<crate::php_parser::Node>(__m))); true }, _ => false } }
}
impl FirstFindingVisitor { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\NodeVisitor\\FirstFindingVisitor could not be converted to string"))) } }
impl php_rt::PhpClone for FirstFindingVisitor { fn php_clone(&self) -> Self { let c = FirstFindingVisitor(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for FirstFindingVisitorObj { fn clone(&self) -> Self { FirstFindingVisitorObj { filterCallback: self.filterCallback.clone(), foundNode: self.foundNode.clone() } } }
impl FirstFindingVisitor {
}
pub struct NameResolverObj {
    pub nameContext: Late<crate::php_parser::NameContext>,
    pub preserveOriginalNames: bool,
    pub replaceNodes: bool,
}
#[derive(Clone)]
pub struct NameResolver(pub Rc<RefCell<NameResolverObj>>);
impl NameResolver {
    pub fn p_nameContext(&self) -> Ref<'_, crate::php_parser::NameContext> { Ref::map(self.0.borrow(), |o| o.nameContext.get()) }
    pub fn p_nameContext_get(&self) -> crate::php_parser::NameContext { self.0.borrow().nameContext.get().clone() }
    pub fn p_nameContext_opt(&self) -> Option<crate::php_parser::NameContext> { self.0.borrow().nameContext.as_option().cloned() }
    pub fn p_nameContext_mut(&self) -> RefMut<'_, crate::php_parser::NameContext> { RefMut::map(self.0.borrow_mut(), |o| o.nameContext.get_mut()) }
    pub fn set_p_nameContext(&self, v: crate::php_parser::NameContext) { self.0.borrow_mut().nameContext.set(v); }
    pub fn p_preserveOriginalNames(&self) -> Ref<'_, bool> { Ref::map(self.0.borrow(), |o| &o.preserveOriginalNames) }
    pub fn p_preserveOriginalNames_get(&self) -> bool { self.0.borrow().preserveOriginalNames.clone() }
    pub fn p_preserveOriginalNames_opt(&self) -> Option<bool> { Some(self.0.borrow().preserveOriginalNames.clone()) }
    pub fn p_preserveOriginalNames_mut(&self) -> RefMut<'_, bool> { RefMut::map(self.0.borrow_mut(), |o| &mut o.preserveOriginalNames) }
    pub fn set_p_preserveOriginalNames(&self, v: bool) { self.0.borrow_mut().preserveOriginalNames = v; }
    pub fn p_replaceNodes(&self) -> Ref<'_, bool> { Ref::map(self.0.borrow(), |o| &o.replaceNodes) }
    pub fn p_replaceNodes_get(&self) -> bool { self.0.borrow().replaceNodes.clone() }
    pub fn p_replaceNodes_opt(&self) -> Option<bool> { Some(self.0.borrow().replaceNodes.clone()) }
    pub fn p_replaceNodes_mut(&self) -> RefMut<'_, bool> { RefMut::map(self.0.borrow_mut(), |o| &mut o.replaceNodes) }
    pub fn set_p_replaceNodes(&self, v: bool) { self.0.borrow_mut().replaceNodes = v; }
    pub fn new(mut errorHandler: Option<crate::php_parser::ErrorHandler>, mut options: Shape_preserveOriginalNamesq_Bool_replaceNodesq_Bool) -> Result<NameResolver, Throw> {
        let this = NameResolver(Rc::new(RefCell::new(NameResolverObj {
            nameContext: Late::uninit(),
            preserveOriginalNames: Default::default(),
            replaceNodes: Default::default(),
        })));
        this.magic__construct(errorHandler, options)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut errorHandler: Option<crate::php_parser::ErrorHandler>, mut options: Shape_preserveOriginalNamesq_Bool_replaceNodesq_Bool) -> Result<Mixed, Throw> {
    self.set_p_nameContext(crate::php_parser::NameContext::new(cast::<crate::php_parser::ErrorHandler>((match errorHandler.clone() { Some(__v) => U_PhpParser_ErrorHandler_or_PhpParser_ErrorHandler_Throwing::PhpParser_ErrorHandler(__v), None => U_PhpParser_ErrorHandler_or_PhpParser_ErrorHandler_Throwing::PhpParser_ErrorHandler_Throwing(crate::php_parser::error_handler::Throwing::new()?) })))?);
    self.set_p_preserveOriginalNames((match Some({ let __c107 = options.clone(); Shape_preserveOriginalNames_Bool_replaceNodesq_Bool { preserveOriginalNames: __c107.preserveOriginalNames.unwrap(), replaceNodes: __c107.replaceNodes } }).and_then(|__b| Some(__b.preserveOriginalNames)) { Some(__v) => __v, None => false }));
    self.set_p_replaceNodes((match Some({ let __c108 = options.clone(); Shape_preserveOriginalNamesq_Bool_replaceNodes_Bool { preserveOriginalNames: __c108.preserveOriginalNames, replaceNodes: __c108.replaceNodes.unwrap() } }).and_then(|__b| Some(__b.replaceNodes)) { Some(__v) => __v, None => true }));
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn getNameContext(&self) -> Result<crate::php_parser::NameContext, Throw> {
    return Ok(self.p_nameContext_get());
    }
    pub fn beforeTraverse(&self, mut nodes: Map<ArrayKey, Mixed>) -> Result<Option<Map<ArrayKey, Mixed>>, Throw> {
    self.p_nameContext_get().startNamespace({ let _ = (); None::<crate::php_parser::node::Name> })?;
    return Ok({ let _ = (); None::<Map<ArrayKey, Mixed>> });
    }
    pub fn enterNode(&self, mut node: crate::php_parser::Node) -> Result<Mixed, Throw> {
    let mut use_: Late<crate::php_parser::node::UseItem> = Late::uninit();
    let mut interface: Late<crate::php_parser::node::Name> = Late::uninit();
    let mut param: Late<crate::php_parser::node::Param> = Late::uninit();
    let mut const_: Late<crate::php_parser::node::Const_> = Late::uninit();
    let mut type_: Late<crate::php_parser::node::Name> = Late::uninit();
    let mut trait_: Late<crate::php_parser::node::Name> = Late::uninit();
    let mut adaptation: Late<crate::php_parser::node::stmt::TraitUseAdaptation> = Late::uninit();
    let mut insteadof: Late<crate::php_parser::node::Name> = Late::uninit();
    if is_instance::<crate::php_parser::node::stmt::Namespace_>(&node.clone()) {
        self.p_nameContext_get().startNamespace(cast::<crate::php_parser::node::stmt::Namespace_>(node.clone()).p_name_get())?;
    } else if is_instance::<crate::php_parser::node::stmt::Use_>(&node.clone()) {
        'l1: for __kv1 in cast::<crate::php_parser::node::stmt::Use_>(node.clone()).p_uses_get().into_iter() {
            use_.set(__kv1.1);
            self.addAlias(use_.get().clone(), cast::<Mixed>(cast::<i64>(cast::<crate::php_parser::node::stmt::Use_>(node.clone()).p_type__get())), { let _ = (); None::<crate::php_parser::node::Name> })?;
        }
    } else if is_instance::<crate::php_parser::node::stmt::GroupUse>(&node.clone()) {
        'l2: for __kv2 in cast::<crate::php_parser::node::stmt::GroupUse>(node.clone()).p_uses_get().into_iter() {
            use_.set(__kv2.1);
            self.addAlias(use_.get().clone(), cast::<Mixed>(cast::<i64>(cast::<crate::php_parser::node::stmt::GroupUse>(node.clone()).p_type__get())), Some(cast::<crate::php_parser::node::stmt::GroupUse>(node.clone()).p_prefix_get()))?;
        }
    } else if is_instance::<crate::php_parser::node::stmt::Class_>(&node.clone()) {
        if (!Some(cast::<crate::php_parser::node::stmt::Class_>(node.clone())).and_then(|__b| Some(__b.p_extends_get())).flatten().is_none()) {
            cast::<crate::php_parser::node::stmt::Class_>(node.clone()).set_p_extends(Some(self.resolveClassName(cast::<crate::php_parser::node::stmt::Class_>(node.clone()).p_extends_get().unwrap())?));
        }
        let __keys4: Vec<ArrayKey> = cast::<crate::php_parser::node::stmt::Class_>(node.clone()).p_implements_get().keys().cloned().collect();
        'l3: for __kv3 in __keys4 {
            interface.set(cast::<crate::php_parser::node::stmt::Class_>(node.clone()).p_implements_get().idx(&__kv3).clone());
            interface.set(self.resolveClassName(interface.get().clone())?);
            (*cast::<crate::php_parser::node::stmt::Class_>(node.clone()).p_implements_mut()).insert(__kv3.clone(), interface.get().clone());
        }
        self.resolveAttrGroups(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::stmt::Class_>(node.clone())))?;
        if (!Some(cast::<crate::php_parser::node::stmt::Class_>(node.clone())).and_then(|__b| Some(__b.p_name_get())).flatten().is_none()) {
            self.addNamespacedName(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::stmt::Class_>(node.clone())))?;
        } else {
            cast::<crate::php_parser::node::stmt::Class_>(node.clone()).set_p_namespacedName({ let _ = (); None::<crate::php_parser::node::Name> });
        }
    } else if is_instance::<crate::php_parser::node::stmt::Interface_>(&node.clone()) {
        let __keys6: Vec<ArrayKey> = cast::<crate::php_parser::node::stmt::Interface_>(node.clone()).p_extends_get().keys().cloned().collect();
        'l4: for __kv5 in __keys6 {
            interface.set(cast::<crate::php_parser::node::stmt::Interface_>(node.clone()).p_extends_get().idx(&__kv5).clone());
            interface.set(self.resolveClassName(interface.get().clone())?);
            (*cast::<crate::php_parser::node::stmt::Interface_>(node.clone()).p_extends_mut()).insert(__kv5.clone(), interface.get().clone());
        }
        self.resolveAttrGroups(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::stmt::Interface_>(node.clone())))?;
        self.addNamespacedName(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::stmt::Interface_>(node.clone())))?;
    } else if is_instance::<crate::php_parser::node::stmt::Enum_>(&node.clone()) {
        let __keys8: Vec<ArrayKey> = cast::<crate::php_parser::node::stmt::Enum_>(node.clone()).p_implements_get().keys().cloned().collect();
        'l5: for __kv7 in __keys8 {
            interface.set(cast::<crate::php_parser::node::stmt::Enum_>(node.clone()).p_implements_get().idx(&__kv7).clone());
            interface.set(self.resolveClassName(interface.get().clone())?);
            (*cast::<crate::php_parser::node::stmt::Enum_>(node.clone()).p_implements_mut()).insert(__kv7.clone(), interface.get().clone());
        }
        self.resolveAttrGroups(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::stmt::Enum_>(node.clone())))?;
        self.addNamespacedName(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::stmt::Enum_>(node.clone())))?;
    } else if is_instance::<crate::php_parser::node::stmt::Trait_>(&node.clone()) {
        self.resolveAttrGroups(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::stmt::Trait_>(node.clone())))?;
        self.addNamespacedName(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::stmt::Trait_>(node.clone())))?;
    } else if is_instance::<crate::php_parser::node::stmt::Function_>(&node.clone()) {
        self.resolveSignature(U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72::PhpParser_Node_Stmt_Function_(cast::<crate::php_parser::node::stmt::Function_>(node.clone())))?;
        self.resolveAttrGroups(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::stmt::Function_>(node.clone())))?;
        self.addNamespacedName(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::stmt::Function_>(node.clone())))?;
    } else if ((is_instance::<crate::php_parser::node::stmt::ClassMethod>(&node.clone()) || is_instance::<crate::php_parser::node::expr::Closure>(&node.clone())) || is_instance::<crate::php_parser::node::expr::ArrowFunction>(&node.clone())) {
        self.resolveSignature(cast::<U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72>(cast::<U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__99228c62df>(node.clone())))?;
        self.resolveAttrGroups(cast::<crate::php_parser::Node>(cast::<U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__99228c62df>(node.clone())))?;
    } else if is_instance::<crate::php_parser::node::stmt::Property>(&node.clone()) {
        if (!Some(cast::<crate::php_parser::node::stmt::Property>(node.clone())).and_then(|__b| Some(__b.p_type__get())).flatten().is_none()) {
            cast::<crate::php_parser::node::stmt::Property>(node.clone()).set_p_type_(self.resolveType(Some(cast::<crate::php_parser::node::stmt::Property>(node.clone()).p_type__get().unwrap()))?);
        }
        self.resolveAttrGroups(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::stmt::Property>(node.clone())))?;
    } else if is_instance::<crate::php_parser::node::PropertyHook>(&node.clone()) {
        'l6: for __kv9 in cast::<crate::php_parser::node::PropertyHook>(node.clone()).p_params_get().into_iter() {
            param.set(__kv9.1);
            param.get().clone().set_p_type_(self.resolveType(param.get().clone().p_type__get())?);
            self.resolveAttrGroups(cast::<crate::php_parser::Node>(param.get().clone()))?;
        }
        self.resolveAttrGroups(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::PropertyHook>(node.clone())))?;
    } else if is_instance::<crate::php_parser::node::stmt::Const_>(&node.clone()) {
        'l7: for __kv10 in cast::<crate::php_parser::node::stmt::Const_>(node.clone()).p_consts_get().into_iter() {
            const_.set(__kv10.1);
            self.addNamespacedName(cast::<crate::php_parser::Node>(const_.get().clone()))?;
        }
        self.resolveAttrGroups(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::stmt::Const_>(node.clone())))?;
    } else if is_instance::<crate::php_parser::node::stmt::ClassConst>(&node.clone()) {
        if (!Some(cast::<crate::php_parser::node::stmt::ClassConst>(node.clone())).and_then(|__b| Some(__b.p_type__get())).flatten().is_none()) {
            cast::<crate::php_parser::node::stmt::ClassConst>(node.clone()).set_p_type_(self.resolveType(Some(cast::<crate::php_parser::node::stmt::ClassConst>(node.clone()).p_type__get().unwrap()))?);
        }
        self.resolveAttrGroups(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::stmt::ClassConst>(node.clone())))?;
    } else if is_instance::<crate::php_parser::node::stmt::EnumCase>(&node.clone()) {
        self.resolveAttrGroups(cast::<crate::php_parser::Node>(cast::<crate::php_parser::node::stmt::EnumCase>(node.clone())))?;
    } else if ((((is_instance::<crate::php_parser::node::expr::StaticCall>(&node.clone()) || is_instance::<crate::php_parser::node::expr::StaticPropertyFetch>(&node.clone())) || is_instance::<crate::php_parser::node::expr::ClassConstFetch>(&node.clone())) || is_instance::<crate::php_parser::node::expr::New_>(&node.clone())) || is_instance::<crate::php_parser::node::expr::Instanceof_>(&node.clone())) {
        if is_instance::<crate::php_parser::node::Name>(&(match cast::<U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d>(node.clone()) { U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_ClassConstFetch(__o) => cast::<U_PhpParser_Node_Expr_or_PhpParser_Node_Name_or_PhpParser_Node_Stmt_Class_>(__o.p_class_get()), U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_Instanceof_(__o) => cast::<U_PhpParser_Node_Expr_or_PhpParser_Node_Name_or_PhpParser_Node_Stmt_Class_>(__o.p_class_get()), U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_New_(__o) => __o.p_class_get(), U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_StaticCall(__o) => cast::<U_PhpParser_Node_Expr_or_PhpParser_Node_Name_or_PhpParser_Node_Stmt_Class_>(__o.p_class_get()), U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_StaticPropertyFetch(__o) => cast::<U_PhpParser_Node_Expr_or_PhpParser_Node_Name_or_PhpParser_Node_Stmt_Class_>(__o.p_class_get()), _ => unreachable!() })) {
            { let __v = cast::<Mixed>(self.resolveClassName((match cast::<U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d>(node.clone()) { U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_ClassConstFetch(__o) => cast::<crate::php_parser::node::Name>(__o.p_class_get()), U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_Instanceof_(__o) => cast::<crate::php_parser::node::Name>(__o.p_class_get()), U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_New_(__o) => cast::<crate::php_parser::node::Name>(__o.p_class_get()), U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_StaticCall(__o) => cast::<crate::php_parser::node::Name>(__o.p_class_get()), U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_StaticPropertyFetch(__o) => cast::<crate::php_parser::node::Name>(__o.p_class_get()), _ => unreachable!() }))?); match cast::<U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d>(node.clone()) { U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_ClassConstFetch(__o) => __o.set_p_class(cast::<U_PhpParser_Node_Expr_or_PhpParser_Node_Name>(__v)), U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_Instanceof_(__o) => __o.set_p_class(cast::<U_PhpParser_Node_Expr_or_PhpParser_Node_Name>(__v)), U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_New_(__o) => __o.set_p_class(cast::<U_PhpParser_Node_Expr_or_PhpParser_Node_Name_or_PhpParser_Node_Stmt_Class_>(__v)), U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_StaticCall(__o) => __o.set_p_class(cast::<U_PhpParser_Node_Expr_or_PhpParser_Node_Name>(__v)), U_PhpParser_Node_Expr_ClassConstFetch_or_PhpParser_Node_Expr_Instanceo_714164646d::PhpParser_Node_Expr_StaticPropertyFetch(__o) => __o.set_p_class(cast::<U_PhpParser_Node_Expr_or_PhpParser_Node_Name>(__v)), _ => unreachable!() } }
        }
    } else if is_instance::<crate::php_parser::node::stmt::Catch_>(&node.clone()) {
        let __keys12: Vec<ArrayKey> = cast::<crate::php_parser::node::stmt::Catch_>(node.clone()).p_types_get().keys().cloned().collect();
        'l8: for __kv11 in __keys12 {
            type_.set(cast::<crate::php_parser::node::stmt::Catch_>(node.clone()).p_types_get().idx(&__kv11).clone());
            type_.set(self.resolveClassName(type_.get().clone())?);
            (*cast::<crate::php_parser::node::stmt::Catch_>(node.clone()).p_types_mut()).insert(__kv11.clone(), type_.get().clone());
        }
    } else if is_instance::<crate::php_parser::node::expr::FuncCall>(&node.clone()) {
        if is_instance::<crate::php_parser::node::Name>(&cast::<crate::php_parser::node::expr::FuncCall>(node.clone()).p_name_get()) {
            cast::<crate::php_parser::node::expr::FuncCall>(node.clone()).set_p_name(U_PhpParser_Node_Expr_or_PhpParser_Node_Name::PhpParser_Node_Name(self.resolveName(cast::<crate::php_parser::node::Name>(cast::<crate::php_parser::node::expr::FuncCall>(node.clone()).p_name_get()), cast::<Mixed>(crate::php_parser::node::stmt::Use_::TYPE_FUNCTION()))?));
        }
    } else if is_instance::<crate::php_parser::node::expr::ConstFetch>(&node.clone()) {
        cast::<crate::php_parser::node::expr::ConstFetch>(node.clone()).set_p_name(self.resolveName(cast::<crate::php_parser::node::expr::ConstFetch>(node.clone()).p_name_get(), cast::<Mixed>(crate::php_parser::node::stmt::Use_::TYPE_CONSTANT()))?);
    } else if is_instance::<crate::php_parser::node::stmt::TraitUse>(&node.clone()) {
        let __keys14: Vec<ArrayKey> = cast::<crate::php_parser::node::stmt::TraitUse>(node.clone()).p_traits_get().keys().cloned().collect();
        'l9: for __kv13 in __keys14 {
            trait_.set(cast::<crate::php_parser::node::stmt::TraitUse>(node.clone()).p_traits_get().idx(&__kv13).clone());
            trait_.set(self.resolveClassName(trait_.get().clone())?);
            (*cast::<crate::php_parser::node::stmt::TraitUse>(node.clone()).p_traits_mut()).insert(__kv13.clone(), trait_.get().clone());
        }
        'l10: for __kv15 in cast::<crate::php_parser::node::stmt::TraitUse>(node.clone()).p_adaptations_get().into_iter() {
            adaptation.set(__kv15.1);
            if (!Some(adaptation.get().clone()).and_then(|__b| Some(__b.p_trait__get())).flatten().is_none()) {
                adaptation.get().clone().set_p_trait_(Some(self.resolveClassName(adaptation.get().clone().p_trait__get().unwrap())?));
            }
            if is_instance::<crate::php_parser::node::stmt::trait_use_adaptation::Precedence>(&adaptation.get().clone()) {
                let __keys17: Vec<ArrayKey> = cast::<crate::php_parser::node::stmt::trait_use_adaptation::Precedence>(adaptation.get().clone()).p_insteadof_get().keys().cloned().collect();
                'l11: for __kv16 in __keys17 {
                    insteadof.set(cast::<crate::php_parser::node::stmt::trait_use_adaptation::Precedence>(adaptation.get().clone()).p_insteadof_get().idx(&__kv16).clone());
                    insteadof.set(self.resolveClassName(insteadof.get().clone())?);
                    (*cast::<crate::php_parser::node::stmt::trait_use_adaptation::Precedence>(adaptation.get().clone()).p_insteadof_mut()).insert(__kv16.clone(), insteadof.get().clone());
                }
            }
        }
    }
    return Ok(Mixed::Null);
    }
    pub fn addAlias(&self, mut use_: crate::php_parser::node::UseItem, mut type_: Mixed, mut prefix: Option<crate::php_parser::node::Name>) -> Result<(), Throw> {
    let mut name: Option<crate::php_parser::node::Name> = Default::default();
    name = (if truthy(&prefix.clone()) { crate::php_parser::node::Name::concat(Some(U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str::PhpParser_Node_Name(prefix.clone().unwrap())), Some(U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str::PhpParser_Node_Name(use_.clone().p_name_get())), Map::<Str, Mixed>::new())? } else { Some(use_.clone().p_name_get()) });
    { let __t1 = cast::<i64>(use_.clone().p_type__get()); type_ = cast::<Mixed>((cast::<i64>(type_.clone()) | __t1)); }
    self.p_nameContext_get().addAlias(name.clone().unwrap(), use_.clone().getAlias()?.to_php_string()?, cast::<Mixed>(cast::<i64>(type_.clone())), use_.clone().getAttributes()?.map_entries(|k, v| (cast::<Str>(k), v)))?;
    #[allow(unreachable_code)] Ok(())
    }
    pub fn resolveSignature(&self, mut node: U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72) -> Result<(), Throw> {
    let mut param: Late<crate::php_parser::node::Param> = Late::uninit();
    'l1: for __kv1 in (match node.clone() { U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72::PhpParser_Node_Expr_ArrowFunction(__o) => cast::<Map<ArrayKey, crate::php_parser::node::Param>>(__o.p_params_get()), U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72::PhpParser_Node_Expr_Closure(__o) => cast::<Map<ArrayKey, crate::php_parser::node::Param>>(__o.p_params_get()), U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72::PhpParser_Node_Stmt_ClassMethod(__o) => __o.p_params_get(), U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72::PhpParser_Node_Stmt_Function_(__o) => __o.p_params_get(), _ => unreachable!() }).into_iter() {
        param.set(__kv1.1);
        param.get().clone().set_p_type_(self.resolveType(param.get().clone().p_type__get())?);
        self.resolveAttrGroups(cast::<crate::php_parser::Node>(param.get().clone()))?;
    }
    { let __v = cast::<Mixed>(self.resolveType((match node.clone() { U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72::PhpParser_Node_Expr_ArrowFunction(__o) => __o.p_returnType_get(), U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72::PhpParser_Node_Expr_Closure(__o) => __o.p_returnType_get(), U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72::PhpParser_Node_Stmt_ClassMethod(__o) => __o.p_returnType_get(), U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72::PhpParser_Node_Stmt_Function_(__o) => __o.p_returnType_get(), _ => unreachable!() }))?); match node.clone() { U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72::PhpParser_Node_Expr_ArrowFunction(__o) => __o.set_p_returnType(__v.to_option().map(|__m| cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(__m))), U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72::PhpParser_Node_Expr_Closure(__o) => __o.set_p_returnType(__v.to_option().map(|__m| cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(__m))), U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72::PhpParser_Node_Stmt_ClassMethod(__o) => __o.set_p_returnType(__v.to_option().map(|__m| cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(__m))), U_PhpParser_Node_Expr_ArrowFunction_or_PhpParser_Node_Expr_Closure_or__0660be4e72::PhpParser_Node_Stmt_Function_(__o) => __o.set_p_returnType(__v.to_option().map(|__m| cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(__m))), _ => unreachable!() } }
    #[allow(unreachable_code)] Ok(())
    }
    pub fn resolveType(&self, mut node: Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>) -> Result<Option<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>, Throw> {
    let mut type_: Late<U_PhpParser_Node_Identifier_or_PhpParser_Node_IntersectionType_or_PhpParser_Node_Name> = Late::uninit();
    if node.clone().map_or(false, |__v| is_instance::<crate::php_parser::node::Name>(&__v)) {
        return Ok(Some(U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name::PhpParser_Node_Name(self.resolveClassName(cast::<crate::php_parser::node::Name>(node.clone().unwrap()))?)));
    }
    if node.clone().map_or(false, |__v| is_instance::<crate::php_parser::node::NullableType>(&__v)) {
        cast::<crate::php_parser::node::NullableType>(node.clone().unwrap()).set_p_type_(cast::<U_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(self.resolveType(Some(cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(cast::<crate::php_parser::node::NullableType>(node.clone().unwrap()).p_type__get())))?.unwrap()));
        return Ok(Some(U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name::PhpParser_Node_ComplexType(cast::<crate::php_parser::node::ComplexType>(cast::<crate::php_parser::node::NullableType>(node.clone().unwrap())))));
    }
    if (node.clone().map_or(false, |__v| is_instance::<crate::php_parser::node::UnionType>(&__v)) || node.clone().map_or(false, |__v| is_instance::<crate::php_parser::node::IntersectionType>(&__v))) {
        let __keys2: Vec<ArrayKey> = (match cast::<U_PhpParser_Node_IntersectionType_or_PhpParser_Node_UnionType>(node.clone().unwrap()) { U_PhpParser_Node_IntersectionType_or_PhpParser_Node_UnionType::PhpParser_Node_IntersectionType(__o) => __o.p_types_get().map_values(|v| cast::<U_PhpParser_Node_Identifier_or_PhpParser_Node_IntersectionType_or_PhpParser_Node_Name>(v)), U_PhpParser_Node_IntersectionType_or_PhpParser_Node_UnionType::PhpParser_Node_UnionType(__o) => __o.p_types_get(), _ => unreachable!() }).keys().cloned().collect();
        'l1: for __kv1 in __keys2 {
            type_.set((match cast::<U_PhpParser_Node_IntersectionType_or_PhpParser_Node_UnionType>(node.clone().unwrap()) { U_PhpParser_Node_IntersectionType_or_PhpParser_Node_UnionType::PhpParser_Node_IntersectionType(__o) => __o.p_types_get().map_values(|v| cast::<U_PhpParser_Node_Identifier_or_PhpParser_Node_IntersectionType_or_PhpParser_Node_Name>(v)), U_PhpParser_Node_IntersectionType_or_PhpParser_Node_UnionType::PhpParser_Node_UnionType(__o) => __o.p_types_get(), _ => unreachable!() }).idx(&__kv1).clone());
            type_.set(cast::<U_PhpParser_Node_Identifier_or_PhpParser_Node_IntersectionType_or_PhpParser_Node_Name>(self.resolveType(Some(cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(type_.get().clone())))?.unwrap()));
            { let mut __p1 = (match cast::<U_PhpParser_Node_IntersectionType_or_PhpParser_Node_UnionType>(node.clone().unwrap()) { U_PhpParser_Node_IntersectionType_or_PhpParser_Node_UnionType::PhpParser_Node_IntersectionType(__o) => __o.p_types_get().map_values(|v| cast::<U_PhpParser_Node_Identifier_or_PhpParser_Node_IntersectionType_or_PhpParser_Node_Name>(v)), U_PhpParser_Node_IntersectionType_or_PhpParser_Node_UnionType::PhpParser_Node_UnionType(__o) => __o.p_types_get(), _ => unreachable!() }); __p1.insert(__kv1.clone(), type_.get().clone()); { let __v = __p1; match cast::<U_PhpParser_Node_IntersectionType_or_PhpParser_Node_UnionType>(node.clone().unwrap()) { U_PhpParser_Node_IntersectionType_or_PhpParser_Node_UnionType::PhpParser_Node_IntersectionType(__o) => __o.set_p_types(__v.map_values(|v| cast::<U_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(v))), U_PhpParser_Node_IntersectionType_or_PhpParser_Node_UnionType::PhpParser_Node_UnionType(__o) => __o.set_p_types(__v), _ => unreachable!() } } }
        }
        return Ok(Some(cast::<U_PhpParser_Node_ComplexType_or_PhpParser_Node_Identifier_or_PhpParser_Node_Name>(cast::<U_PhpParser_Node_IntersectionType_or_PhpParser_Node_UnionType>(node.clone().unwrap()))));
    }
    return Ok(node.clone());
    }
    pub fn resolveName(&self, mut name: crate::php_parser::node::Name, mut type_: Mixed) -> Result<crate::php_parser::node::Name, Throw> {
    let mut resolvedName: Option<crate::php_parser::node::Name> = Default::default();
    let mut originalName: Late<crate::php_parser::node::Name> = Late::uninit();
    if (!self.p_replaceNodes_get()) {
        resolvedName = self.p_nameContext_get().getResolvedName(name.clone(), cast::<Mixed>(cast::<i64>(type_.clone())))?;
        if (!resolvedName.clone().is_none()) {
            name.clone().setAttribute(Str::from_static("resolvedName"), cast::<Mixed>(resolvedName.clone().unwrap()))?;
        } else {
            name.clone().setAttribute(Str::from_static("namespacedName"), cast::<Mixed>(crate::php_parser::node::Name::concat(self.p_nameContext_get().getNamespace()?.map(|v| U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str::PhpParser_Node_Name(v)), Some(U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str::PhpParser_Node_Name(name.clone())), name.clone().getAttributes()?.map_entries(|k, v| (cast::<Str>(k), v)))?))?;
        }
        return Ok(name.clone());
    }
    if self.p_preserveOriginalNames_get() {
        originalName.set(name.clone());
        name = originalName.get().clone().php_clone();
        name.clone().setAttribute(Str::from_static("originalName"), cast::<Mixed>(originalName.get().clone()))?;
    }
    resolvedName = self.p_nameContext_get().getResolvedName(name.clone(), cast::<Mixed>(cast::<i64>(type_.clone())))?;
    if (!resolvedName.clone().is_none()) {
        return Ok(resolvedName.clone().unwrap());
    }
    name.clone().setAttribute(Str::from_static("namespacedName"), cast::<Mixed>(crate::php_parser::node::Name::concat(self.p_nameContext_get().getNamespace()?.map(|v| U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str::PhpParser_Node_Name(v)), Some(U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str::PhpParser_Node_Name(name.clone())), name.clone().getAttributes()?.map_entries(|k, v| (cast::<Str>(k), v)))?))?;
    return Ok(name.clone());
    }
    pub fn resolveClassName(&self, mut name: crate::php_parser::node::Name) -> Result<crate::php_parser::node::Name, Throw> {
    return Ok(self.resolveName(name.clone(), cast::<Mixed>(crate::php_parser::node::stmt::Use_::TYPE_NORMAL()))?);
    }
    pub fn addNamespacedName(&self, mut node: crate::php_parser::Node) -> Result<(), Throw> {
    mixed_set_prop(&cast::<Mixed>(node.clone()), &Str::from_static("namespacedName"), cast::<Mixed>(crate::php_parser::node::Name::concat(self.p_nameContext_get().getNamespace()?.map(|v| U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str::PhpParser_Node_Name(v)), Some(U_Map_ArrayKey_Str_or_PhpParser_Node_Name_or_Str::Str(to_str(&mixed_prop(&cast::<Mixed>(node.clone()), &Str::from_static("name"))))), Map::<Str, Mixed>::new())?));
    #[allow(unreachable_code)] Ok(())
    }
    pub fn resolveAttrGroups(&self, mut node: crate::php_parser::Node) -> Result<(), Throw> {
    let mut attrGroup: Mixed = Default::default();
    let mut attr: Mixed = Default::default();
    'l1: for __kv1 in mixed_iter(cast::<Mixed>(mixed_prop(&cast::<Mixed>(node.clone()), &Str::from_static("attrGroups"))))? {
        attrGroup = __kv1.1;
        'l2: for __kv2 in mixed_iter(cast::<Mixed>(mixed_prop(&attrGroup.clone(), &Str::from_static("attrs"))))? {
            attr = __kv2.1;
            mixed_set_prop(&attr.clone(), &Str::from_static("name"), cast::<Mixed>(self.resolveClassName(cast::<crate::php_parser::node::Name>(cast::<Mixed>(mixed_prop(&attr.clone(), &Str::from_static("name")))))?));
        }
    }
    #[allow(unreachable_code)] Ok(())
    }
    pub fn leaveNode(&self, mut node: crate::php_parser::Node) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeVisitorAbstract>(self.clone()).leaveNode__impl(node) }
    pub fn afterTraverse(&self, mut nodes: Map<ArrayKey, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeVisitorAbstract>(self.clone()).afterTraverse__impl(nodes) }
    pub fn new_same_class(&self, mut errorHandler: Option<crate::php_parser::ErrorHandler>, mut options: Shape_preserveOriginalNamesq_Bool_replaceNodesq_Bool) -> Result<crate::php_parser::node_visitor::NameResolver, Throw> { Ok(Self::new(errorHandler, options)?) }
}
impl php_rt::PhpObject for NameResolver {
    fn class_name(&self) -> &'static str { "PhpParser\\NodeVisitor\\NameResolver" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\nodevisitor\\nameresolver", "phpparser\\nodevisitorabstract", "phpparser\\nodevisitor"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = self.p_nameContext_opt() { out.push((Str::from_static("nameContext"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_preserveOriginalNames_get()) { out.push((Str::from_static("preserveOriginalNames"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_replaceNodes_get()) { out.push((Str::from_static("replaceNodes"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "nameContext" => { self.set_p_nameContext(cast::<crate::php_parser::NameContext>(value)); true }, "preserveOriginalNames" => { self.set_p_preserveOriginalNames(cast::<bool>(value)); true }, "replaceNodes" => { self.set_p_replaceNodes(cast::<bool>(value)); true }, _ => false } }
}
impl NameResolver { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\NodeVisitor\\NameResolver could not be converted to string"))) } }
impl php_rt::PhpClone for NameResolver { fn php_clone(&self) -> Self { let c = NameResolver(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for NameResolverObj { fn clone(&self) -> Self { NameResolverObj { nameContext: self.nameContext.clone(), preserveOriginalNames: self.preserveOriginalNames.clone(), replaceNodes: self.replaceNodes.clone() } } }
impl NameResolver {
}
pub struct NodeConnectingVisitorObj {
    pub stack: Map<ArrayKey, crate::php_parser::Node>,
    pub previous: Option<crate::php_parser::Node>,
    pub weakReferences: bool,
}
#[derive(Clone)]
pub struct NodeConnectingVisitor(pub Rc<RefCell<NodeConnectingVisitorObj>>);
impl NodeConnectingVisitor {
    pub fn p_stack(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::Node>> { Ref::map(self.0.borrow(), |o| &o.stack) }
    pub fn p_stack_get(&self) -> Map<ArrayKey, crate::php_parser::Node> { self.0.borrow().stack.clone() }
    pub fn p_stack_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::Node>> { Some(self.0.borrow().stack.clone()) }
    pub fn p_stack_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::Node>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stack) }
    pub fn set_p_stack(&self, v: Map<ArrayKey, crate::php_parser::Node>) { self.0.borrow_mut().stack = v; }
    pub fn p_previous(&self) -> Ref<'_, Option<crate::php_parser::Node>> { Ref::map(self.0.borrow(), |o| &o.previous) }
    pub fn p_previous_get(&self) -> Option<crate::php_parser::Node> { self.0.borrow().previous.clone() }
    pub fn p_previous_opt(&self) -> Option<Option<crate::php_parser::Node>> { Some(self.0.borrow().previous.clone()) }
    pub fn p_previous_mut(&self) -> RefMut<'_, Option<crate::php_parser::Node>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.previous) }
    pub fn set_p_previous(&self, v: Option<crate::php_parser::Node>) { self.0.borrow_mut().previous = v; }
    pub fn p_weakReferences(&self) -> Ref<'_, bool> { Ref::map(self.0.borrow(), |o| &o.weakReferences) }
    pub fn p_weakReferences_get(&self) -> bool { self.0.borrow().weakReferences.clone() }
    pub fn p_weakReferences_opt(&self) -> Option<bool> { Some(self.0.borrow().weakReferences.clone()) }
    pub fn p_weakReferences_mut(&self) -> RefMut<'_, bool> { RefMut::map(self.0.borrow_mut(), |o| &mut o.weakReferences) }
    pub fn set_p_weakReferences(&self, v: bool) { self.0.borrow_mut().weakReferences = v; }
    pub fn new(mut weakReferences: bool) -> Result<NodeConnectingVisitor, Throw> {
        let this = NodeConnectingVisitor(Rc::new(RefCell::new(NodeConnectingVisitorObj {
            stack: Map::<ArrayKey, crate::php_parser::Node>::new(),
            previous: Default::default(),
            weakReferences: Default::default(),
        })));
        this.magic__construct(weakReferences)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut weakReferences: bool) -> Result<Mixed, Throw> {
    self.set_p_weakReferences(weakReferences);
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn beforeTraverse(&self, mut nodes: Map<ArrayKey, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_stack(Map::<ArrayKey, crate::php_parser::Node>::new());
    self.set_p_previous({ let _ = (); None::<crate::php_parser::Node> });
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn enterNode(&self, mut node: crate::php_parser::Node) -> Result<Mixed, Throw> {
    let mut parent: Late<crate::php_parser::Node> = Late::uninit();
    if (!(!truthy(&Some(self.clone()).and_then(|__b| Some(__b.p_stack_get()))))) {
        parent.set(self.p_stack_get().idx(&to_key(&(self.p_stack_get().count()).wrapping_sub(1i64))).clone());
        if self.p_weakReferences_get() {
            node.clone().setAttribute(Str::from_static("weak_parent"), cast::<Mixed>(WeakReference::create(parent.get().clone())))?;
        } else {
            node.clone().setAttribute(Str::from_static("parent"), cast::<Mixed>(parent.get().clone()))?;
        }
    }
    if (!Some(self.clone()).and_then(|__b| Some(__b.p_previous_get())).flatten().is_none()) {
        if self.p_weakReferences_get() {
            if identical(&self.p_previous_get().unwrap().getAttribute(Str::from_static("weak_parent"), Mixed::Null)?, &node.clone().getAttribute(Str::from_static("weak_parent"), Mixed::Null)?) {
                node.clone().setAttribute(Str::from_static("weak_previous"), cast::<Mixed>(WeakReference::create(self.p_previous_get().unwrap())))?;
                self.p_previous_get().unwrap().setAttribute(Str::from_static("weak_next"), cast::<Mixed>(WeakReference::create(node.clone())))?;
            }
        } else if identical(&self.p_previous_get().unwrap().getAttribute(Str::from_static("parent"), Mixed::Null)?, &node.clone().getAttribute(Str::from_static("parent"), Mixed::Null)?) {
            node.clone().setAttribute(Str::from_static("previous"), cast::<Mixed>(self.p_previous_get().unwrap()))?;
            self.p_previous_get().unwrap().setAttribute(Str::from_static("next"), cast::<Mixed>(node.clone()))?;
        }
    }
    (*self.p_stack_mut()).push(node.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn leaveNode(&self, mut node: crate::php_parser::Node) -> Result<Mixed, Throw> {
    self.set_p_previous(Some(node.clone()));
    let _: Option<crate::php_parser::Node> = { let __r = (*self.p_stack_mut()).pop(); __r };
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn afterTraverse(&self, mut nodes: Map<ArrayKey, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeVisitorAbstract>(self.clone()).afterTraverse__impl(nodes) }
    pub fn new_same_class(&self, mut weakReferences: bool) -> Result<crate::php_parser::node_visitor::NodeConnectingVisitor, Throw> { Ok(Self::new(weakReferences)?) }
}
impl php_rt::PhpObject for NodeConnectingVisitor {
    fn class_name(&self) -> &'static str { "PhpParser\\NodeVisitor\\NodeConnectingVisitor" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\nodevisitor\\nodeconnectingvisitor", "phpparser\\nodevisitorabstract", "phpparser\\nodevisitor"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_stack_get()) { out.push((Str::from_static("stack"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_previous_get()) { out.push((Str::from_static("previous"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_weakReferences_get()) { out.push((Str::from_static("weakReferences"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "stack" => { self.set_p_stack(cast::<Map<ArrayKey, crate::php_parser::Node>>(value)); true }, "previous" => { self.set_p_previous(value.to_option().map(|__m| cast::<crate::php_parser::Node>(__m))); true }, "weakReferences" => { self.set_p_weakReferences(cast::<bool>(value)); true }, _ => false } }
}
impl NodeConnectingVisitor { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\NodeVisitor\\NodeConnectingVisitor could not be converted to string"))) } }
impl php_rt::PhpClone for NodeConnectingVisitor { fn php_clone(&self) -> Self { let c = NodeConnectingVisitor(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for NodeConnectingVisitorObj { fn clone(&self) -> Self { NodeConnectingVisitorObj { stack: self.stack.clone(), previous: self.previous.clone(), weakReferences: self.weakReferences.clone() } } }
impl NodeConnectingVisitor {
}
pub struct ParentConnectingVisitorObj {
    pub stack: Map<ArrayKey, crate::php_parser::Node>,
    pub weakReferences: bool,
}
#[derive(Clone)]
pub struct ParentConnectingVisitor(pub Rc<RefCell<ParentConnectingVisitorObj>>);
impl ParentConnectingVisitor {
    pub fn p_stack(&self) -> Ref<'_, Map<ArrayKey, crate::php_parser::Node>> { Ref::map(self.0.borrow(), |o| &o.stack) }
    pub fn p_stack_get(&self) -> Map<ArrayKey, crate::php_parser::Node> { self.0.borrow().stack.clone() }
    pub fn p_stack_opt(&self) -> Option<Map<ArrayKey, crate::php_parser::Node>> { Some(self.0.borrow().stack.clone()) }
    pub fn p_stack_mut(&self) -> RefMut<'_, Map<ArrayKey, crate::php_parser::Node>> { RefMut::map(self.0.borrow_mut(), |o| &mut o.stack) }
    pub fn set_p_stack(&self, v: Map<ArrayKey, crate::php_parser::Node>) { self.0.borrow_mut().stack = v; }
    pub fn p_weakReferences(&self) -> Ref<'_, bool> { Ref::map(self.0.borrow(), |o| &o.weakReferences) }
    pub fn p_weakReferences_get(&self) -> bool { self.0.borrow().weakReferences.clone() }
    pub fn p_weakReferences_opt(&self) -> Option<bool> { Some(self.0.borrow().weakReferences.clone()) }
    pub fn p_weakReferences_mut(&self) -> RefMut<'_, bool> { RefMut::map(self.0.borrow_mut(), |o| &mut o.weakReferences) }
    pub fn set_p_weakReferences(&self, v: bool) { self.0.borrow_mut().weakReferences = v; }
    pub fn new(mut weakReferences: bool) -> Result<ParentConnectingVisitor, Throw> {
        let this = ParentConnectingVisitor(Rc::new(RefCell::new(ParentConnectingVisitorObj {
            stack: Map::<ArrayKey, crate::php_parser::Node>::new(),
            weakReferences: Default::default(),
        })));
        this.magic__construct(weakReferences)?;
        Ok(this)
    }
    pub fn magic__construct(&self, mut weakReferences: bool) -> Result<Mixed, Throw> {
    self.set_p_weakReferences(weakReferences);
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn beforeTraverse(&self, mut nodes: Map<ArrayKey, Mixed>) -> Result<Mixed, Throw> {
    self.set_p_stack(Map::<ArrayKey, crate::php_parser::Node>::new());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn enterNode(&self, mut node: crate::php_parser::Node) -> Result<Mixed, Throw> {
    let mut parent: Late<crate::php_parser::Node> = Late::uninit();
    if (!(!truthy(&Some(self.clone()).and_then(|__b| Some(__b.p_stack_get()))))) {
        parent.set(self.p_stack_get().idx(&to_key(&(self.p_stack_get().count()).wrapping_sub(1i64))).clone());
        if self.p_weakReferences_get() {
            node.clone().setAttribute(Str::from_static("weak_parent"), cast::<Mixed>(WeakReference::create(parent.get().clone())))?;
        } else {
            node.clone().setAttribute(Str::from_static("parent"), cast::<Mixed>(parent.get().clone()))?;
        }
    }
    (*self.p_stack_mut()).push(node.clone());
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn leaveNode(&self, mut node: crate::php_parser::Node) -> Result<Mixed, Throw> {
    let _: Option<crate::php_parser::Node> = { let __r = (*self.p_stack_mut()).pop(); __r };
    #[allow(unreachable_code)] Ok(Mixed::Null)
    }
    pub fn afterTraverse(&self, mut nodes: Map<ArrayKey, Mixed>) -> Result<Mixed, Throw> { cast::<crate::php_parser::NodeVisitorAbstract>(self.clone()).afterTraverse__impl(nodes) }
    pub fn new_same_class(&self, mut weakReferences: bool) -> Result<crate::php_parser::node_visitor::ParentConnectingVisitor, Throw> { Ok(Self::new(weakReferences)?) }
}
impl php_rt::PhpObject for ParentConnectingVisitor {
    fn class_name(&self) -> &'static str { "PhpParser\\NodeVisitor\\ParentConnectingVisitor" }
    fn class_ancestors(&self) -> &'static [&'static str] { &["phpparser\\nodevisitor\\parentconnectingvisitor", "phpparser\\nodevisitorabstract", "phpparser\\nodevisitor"] }
    fn obj_id(&self) -> usize { Rc::as_ptr(&self.0) as *const u8 as usize }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn props(&self) -> Vec<(Str, Mixed)> { let mut out = Vec::new(); if let Some(v) = Some(self.p_stack_get()) { out.push((Str::from_static("stack"), cast::<Mixed>(v))); } if let Some(v) = Some(self.p_weakReferences_get()) { out.push((Str::from_static("weakReferences"), cast::<Mixed>(v))); } out }
    fn set_prop(&self, name: &str, value: Mixed) -> bool { match name { "stack" => { self.set_p_stack(cast::<Map<ArrayKey, crate::php_parser::Node>>(value)); true }, "weakReferences" => { self.set_p_weakReferences(cast::<bool>(value)); true }, _ => false } }
}
impl ParentConnectingVisitor { pub fn to_php_string(&self) -> Result<Str, Throw> { Err(Throw::error(Str::from_static("Object of class PhpParser\\NodeVisitor\\ParentConnectingVisitor could not be converted to string"))) } }
impl php_rt::PhpClone for ParentConnectingVisitor { fn php_clone(&self) -> Self { let c = ParentConnectingVisitor(Rc::new(RefCell::new(self.0.borrow().clone()))); c } }
impl Clone for ParentConnectingVisitorObj { fn clone(&self) -> Self { ParentConnectingVisitorObj { stack: self.stack.clone(), weakReferences: self.weakReferences.clone() } } }
impl ParentConnectingVisitor {
}
impl php_rt::Truthy for CloningVisitor { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for CloningVisitor { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\NodeVisitor\\CloningVisitor")) } }
impl php_rt::Identical for CloningVisitor { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for CloningVisitor { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for CloningVisitor { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for CloningVisitor { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<CloningVisitor> for Mixed { fn cast_to(self) -> CloningVisitor { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node_visitor::CloningVisitor>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\NodeVisitor\\CloningVisitor") } }
impl php_rt::TryDowncast for CloningVisitor { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node_visitor::CloningVisitor>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for CloningVisitor { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<CloningVisitor> for AnyObject { fn cast_to(self) -> CloningVisitor { cast::<CloningVisitor>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<CloningVisitor> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\nodevisitor\\cloningvisitor") } }
impl php_rt::InstanceOf<CloningVisitor> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\nodevisitor\\cloningvisitor") } }
impl php_rt::InstanceOf<CloningVisitor> for CloningVisitor { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for CommentAnnotatingVisitor { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for CommentAnnotatingVisitor { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\NodeVisitor\\CommentAnnotatingVisitor")) } }
impl php_rt::Identical for CommentAnnotatingVisitor { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for CommentAnnotatingVisitor { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for CommentAnnotatingVisitor { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for CommentAnnotatingVisitor { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<CommentAnnotatingVisitor> for Mixed { fn cast_to(self) -> CommentAnnotatingVisitor { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node_visitor::CommentAnnotatingVisitor>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\NodeVisitor\\CommentAnnotatingVisitor") } }
impl php_rt::TryDowncast for CommentAnnotatingVisitor { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node_visitor::CommentAnnotatingVisitor>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for CommentAnnotatingVisitor { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<CommentAnnotatingVisitor> for AnyObject { fn cast_to(self) -> CommentAnnotatingVisitor { cast::<CommentAnnotatingVisitor>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<CommentAnnotatingVisitor> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\nodevisitor\\commentannotatingvisitor") } }
impl php_rt::InstanceOf<CommentAnnotatingVisitor> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\nodevisitor\\commentannotatingvisitor") } }
impl php_rt::InstanceOf<CommentAnnotatingVisitor> for CommentAnnotatingVisitor { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for FindingVisitor { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for FindingVisitor { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\NodeVisitor\\FindingVisitor")) } }
impl php_rt::Identical for FindingVisitor { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for FindingVisitor { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for FindingVisitor { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for FindingVisitor { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<FindingVisitor> for Mixed { fn cast_to(self) -> FindingVisitor { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node_visitor::FindingVisitor>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\NodeVisitor\\FindingVisitor") } }
impl php_rt::TryDowncast for FindingVisitor { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node_visitor::FindingVisitor>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for FindingVisitor { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<FindingVisitor> for AnyObject { fn cast_to(self) -> FindingVisitor { cast::<FindingVisitor>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<FindingVisitor> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\nodevisitor\\findingvisitor") } }
impl php_rt::InstanceOf<FindingVisitor> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\nodevisitor\\findingvisitor") } }
impl php_rt::InstanceOf<FindingVisitor> for FindingVisitor { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for FirstFindingVisitor { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for FirstFindingVisitor { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\NodeVisitor\\FirstFindingVisitor")) } }
impl php_rt::Identical for FirstFindingVisitor { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for FirstFindingVisitor { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for FirstFindingVisitor { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for FirstFindingVisitor { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<FirstFindingVisitor> for Mixed { fn cast_to(self) -> FirstFindingVisitor { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node_visitor::FirstFindingVisitor>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\NodeVisitor\\FirstFindingVisitor") } }
impl php_rt::TryDowncast for FirstFindingVisitor { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node_visitor::FirstFindingVisitor>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for FirstFindingVisitor { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<FirstFindingVisitor> for AnyObject { fn cast_to(self) -> FirstFindingVisitor { cast::<FirstFindingVisitor>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<FirstFindingVisitor> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\nodevisitor\\firstfindingvisitor") } }
impl php_rt::InstanceOf<FirstFindingVisitor> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\nodevisitor\\firstfindingvisitor") } }
impl php_rt::InstanceOf<FirstFindingVisitor> for FirstFindingVisitor { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for NameResolver { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for NameResolver { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\NodeVisitor\\NameResolver")) } }
impl php_rt::Identical for NameResolver { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for NameResolver { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for NameResolver { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for NameResolver { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<NameResolver> for Mixed { fn cast_to(self) -> NameResolver { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node_visitor::NameResolver>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\NodeVisitor\\NameResolver") } }
impl php_rt::TryDowncast for NameResolver { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node_visitor::NameResolver>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for NameResolver { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<NameResolver> for AnyObject { fn cast_to(self) -> NameResolver { cast::<NameResolver>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<NameResolver> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\nodevisitor\\nameresolver") } }
impl php_rt::InstanceOf<NameResolver> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\nodevisitor\\nameresolver") } }
impl php_rt::InstanceOf<NameResolver> for NameResolver { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for NodeConnectingVisitor { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for NodeConnectingVisitor { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\NodeVisitor\\NodeConnectingVisitor")) } }
impl php_rt::Identical for NodeConnectingVisitor { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for NodeConnectingVisitor { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for NodeConnectingVisitor { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for NodeConnectingVisitor { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<NodeConnectingVisitor> for Mixed { fn cast_to(self) -> NodeConnectingVisitor { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node_visitor::NodeConnectingVisitor>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\NodeVisitor\\NodeConnectingVisitor") } }
impl php_rt::TryDowncast for NodeConnectingVisitor { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node_visitor::NodeConnectingVisitor>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for NodeConnectingVisitor { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<NodeConnectingVisitor> for AnyObject { fn cast_to(self) -> NodeConnectingVisitor { cast::<NodeConnectingVisitor>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<NodeConnectingVisitor> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\nodevisitor\\nodeconnectingvisitor") } }
impl php_rt::InstanceOf<NodeConnectingVisitor> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\nodevisitor\\nodeconnectingvisitor") } }
impl php_rt::InstanceOf<NodeConnectingVisitor> for NodeConnectingVisitor { fn is_instance(&self) -> bool { true } }
impl php_rt::Truthy for ParentConnectingVisitor { fn truthy(&self) -> bool { true } }
impl php_rt::ToStr for ParentConnectingVisitor { fn to_php_str(&self) -> Str { self.php_to_string().unwrap_or_else(|| Str::from_static("PhpParser\\NodeVisitor\\ParentConnectingVisitor")) } }
impl php_rt::Identical for ParentConnectingVisitor { fn identical(&self, o: &Self) -> bool { self.obj_id() == o.obj_id() } }
impl php_rt::PhpCmp for ParentConnectingVisitor { fn php_cmp(&self, o: &Self) -> std::cmp::Ordering { cast::<Mixed>(self.clone()).php_cmp(&cast::<Mixed>(o.clone())) } }
impl std::fmt::Debug for ParentConnectingVisitor { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "object({})#{}", self.class_name(), self.obj_id()) } }
impl php_rt::CastTo<Mixed> for ParentConnectingVisitor { fn cast_to(self) -> Mixed { Mixed::Obj(Rc::new(self)) } }
impl php_rt::CastTo<ParentConnectingVisitor> for Mixed { fn cast_to(self) -> ParentConnectingVisitor { if let Mixed::Obj(o) = &self { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node_visitor::ParentConnectingVisitor>() { return v.clone(); } } panic!("Mixed value is not a PhpParser\\NodeVisitor\\ParentConnectingVisitor") } }
impl php_rt::TryDowncast for ParentConnectingVisitor { fn try_downcast(o: &AnyObj) -> Option<Self> { if let Some(v) = o.as_any().downcast_ref::<crate::php_parser::node_visitor::ParentConnectingVisitor>() { return Some(v.clone()); } None } }
impl php_rt::CastTo<AnyObject> for ParentConnectingVisitor { fn cast_to(self) -> AnyObject { AnyObject::from_mixed(cast::<Mixed>(self)) } }
impl php_rt::CastTo<ParentConnectingVisitor> for AnyObject { fn cast_to(self) -> ParentConnectingVisitor { cast::<ParentConnectingVisitor>(cast::<Mixed>(self)) } }
impl php_rt::InstanceOf<ParentConnectingVisitor> for AnyObject { fn is_instance(&self) -> bool { self.instance_of_name("phpparser\\nodevisitor\\parentconnectingvisitor") } }
impl php_rt::InstanceOf<ParentConnectingVisitor> for Mixed { fn is_instance(&self) -> bool { self.instance_of("phpparser\\nodevisitor\\parentconnectingvisitor") } }
impl php_rt::InstanceOf<ParentConnectingVisitor> for ParentConnectingVisitor { fn is_instance(&self) -> bool { true } }
