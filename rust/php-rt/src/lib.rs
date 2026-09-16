//! Runtime support library for PHP code transpiled to Rust by Psalm's transpiler.
#![allow(clippy::all)]
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, ambiguous_glob_reexports, hidden_glob_reexports)]

pub mod string;
pub mod key;
pub mod list;
pub mod map;
pub mod conv;
pub mod late;
pub mod refs;
pub mod mixed;
pub mod error;
pub mod traits;
pub mod ops;
pub mod cast;
pub mod output;
pub mod support;
pub mod containers;
pub mod registry;
pub mod consts;
pub mod tokenizer;
pub mod xml;
pub mod builtins;
pub mod data;
pub mod testing;

pub use string::Str;
pub use key::ArrayKey;
pub use list::List;
pub use map::Map;
pub use late::Late;
pub use refs::{PhpRef, Cell as PhpCell, new_cell, cell_of};
pub use mixed::{Mixed, AnyObj, PhpObject};
pub use error::{RtError, Flow, R, Never, never, dead};
pub use traits::*;
pub use ops::*;
pub use cast::{CastTo, cast};
pub use support::*;
pub use containers::*;
pub use conv::Num;
pub use output::*;
pub use builtins::*;

pub mod prelude {
    pub use crate::{Str, ArrayKey, List, Map, Late, Mixed, AnyObj, PhpObject, RtError, Flow, R, Never, never, dead, Num};
    pub use crate::refs::{PhpRef, Cell as PhpCell, new_cell, cell_of};
    pub use crate::key::MapKey;
    pub use crate::{list, map, cat, sfmt, sprintf, impl_enum_handle};
    pub use crate::traits::*;
    pub use crate::ops::*;
    pub use crate::cast::{CastTo, cast};
    pub use crate::support::*;
    pub use crate::containers::*;
    pub use crate::output::*;
    pub use crate::builtins::*;
    pub use crate::consts;
    pub use crate::registry;
    pub use std::sync::Arc as Rc;
    pub use crate::support::{RwCell as RefCell, CellRef as Ref, CellRefMut as RefMut};
}
