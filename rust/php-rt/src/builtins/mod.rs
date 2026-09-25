//! Typed implementations of PHP builtin functions.

pub mod string;
pub mod array;
pub mod math;
pub mod var;
pub mod hash;
pub mod pcre;
pub mod json;
pub mod file;
pub mod misc;
pub mod eval;

pub use string::*;
pub use array::*;
pub use math::*;
pub use var::*;
pub use pcre::*;
pub use json::*;
pub use file::*;
pub use misc::*;
pub use eval::*;
