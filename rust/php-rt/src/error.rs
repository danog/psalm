//! Error and control-flow plumbing.

use crate::string::Str;

/// An error raised by the runtime; the generated crate converts it into a PHP exception object.
#[derive(Debug, Clone)]
pub struct RtError {
    pub class: &'static str,
    pub message: Str,
}

impl RtError {
    pub fn new(class: &'static str, message: impl Into<Str>) -> RtError {
        RtError { class, message: message.into() }
    }
    pub fn error(message: impl Into<Str>) -> RtError {
        RtError::new("Error", message)
    }
    pub fn type_error(message: impl Into<Str>) -> RtError {
        RtError::new("TypeError", message)
    }
    pub fn value_error(message: impl Into<Str>) -> RtError {
        RtError::new("ValueError", message)
    }
    pub fn division_by_zero() -> RtError {
        RtError::new("DivisionByZeroError", "Division by zero")
    }
}

impl std::fmt::Display for RtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.class, self.message)
    }
}

/// Non-local control flow escaping a try block closure.
pub enum Flow<T> {
    Normal,
    Return(T),
    Break(usize),
    Continue(usize),
}

/// The uninhabited type for PHP `never`.
#[derive(Clone, Copy, Debug)]
pub enum Never {}

#[inline]
/// A value the transpiler proved unreachable (unsupported construct or external code): panics when evaluated.
pub fn dead<T>(msg: &str) -> T {
    panic!("{}", msg)
}

pub fn never(n: Never) -> ! {
    match n {}
}

/// Result alias used by builtins.
pub type R<T> = Result<T, RtError>;
