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

// an uninhabited value satisfies every value trait vacuously (a container of it is always empty)
impl crate::traits::Identical for Never {
    fn identical(&self, _o: &Self) -> bool {
        match *self {}
    }
}
impl crate::traits::PhpCmp for Never {
    fn php_cmp(&self, _o: &Self) -> std::cmp::Ordering {
        match *self {}
    }
}
impl crate::traits::Truthy for Never {
    fn truthy(&self) -> bool {
        match *self {}
    }
}
impl crate::traits::ToStr for Never {
    fn to_php_str(&self) -> crate::string::Str {
        match *self {}
    }
}
impl crate::traits::PhpKind for Never {
    fn php_kind(&self) -> crate::traits::Kind {
        match *self {}
    }
}
impl crate::traits::InstanceOfName for Never {
    fn php_instance_of(&self, _name: &[u8]) -> bool {
        match *self {}
    }
}

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

/// An uncaught PHP exception (a `throw` outside any `try` in the transpiled program). Under the panic-based
/// error model these do not need to be recovered from — they abort like a Rust panic, carrying the exception
/// message. Returns `!` so it fits any expression/return position (including bare-`T` non-Result functions).
pub fn uncaught<E: std::fmt::Display>(e: E) -> ! {
    panic!("Uncaught exception: {}", e)
}


/// PHP `throw`/`try`/`catch` under the panic-based error model. A PHP `throw` unwinds the Rust stack carrying
/// the thrown exception object (the program's `Throw` type — a closed enum over its Throwable classes) as the
/// panic payload; `try` boundaries catch_unwind and call take_thrown::<Throw>(). Nothing is dynamically typed:
/// the payload is downcast back to the one static type the program throws.
pub struct PhpThrow(pub Box<dyn std::any::Any + Send>);

/// The typed exception interface the generated `Throw` type implements (used by the test harness).
pub trait PhpThrowable: std::any::Any + Send + 'static {
    fn class_name(&self) -> &'static str;
    /// Lower-cased fully qualified names of the class and all its ancestors/interfaces.
    fn class_ancestors(&self) -> &'static [&'static str];
    fn message(&self) -> crate::Str;
}

/// Execute a PHP `throw`: unwind with the exception object as the payload. Returns `!` (fits any position).
pub fn do_throw<T: std::any::Any + Send>(e: T) -> ! {
    std::panic::panic_any(PhpThrow(Box::new(e)))
}

/// At a `try` boundary, given a caught panic payload: return the thrown PHP exception if it was a `throw`,
/// else resume unwinding (a genuine Rust panic = invariant violation, not catchable by PHP).
pub fn take_thrown<T: std::any::Any>(payload: Box<dyn std::any::Any + Send>) -> T {
    match payload.downcast::<PhpThrow>() {
        Ok(t) => match t.0.downcast::<T>() {
            Ok(e) => *e,
            Err(_) => panic!("PHP throw payload is not the program's Throw type"),
        },
        Err(p) => std::panic::resume_unwind(p),
    }
}

/// Like [`take_thrown`] but non-resuming: `Ok(exception)` if the payload is a PHP `throw`,
/// else `Err(payload)` so the caller can report a genuine Rust panic itself (used by the test harness).
pub fn take_thrown_opt<T: std::any::Any>(payload: Box<dyn std::any::Any + Send>) -> Result<T, Box<dyn std::any::Any + Send>> {
    match payload.downcast::<PhpThrow>() {
        Ok(t) => match t.0.downcast::<T>() {
            Ok(e) => Ok(*e),
            Err(_) => panic!("PHP throw payload is not the program's Throw type"),
        },
        Err(p) => Err(p),
    }
}

/// Install a panic hook that stays silent for PhpThrow (PHP exceptions in flight, usually caught) so caught
/// exceptions do not spam stderr with Rust panic backtraces; all other panics use the default hook.
pub fn install_throw_panic_hook() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let default = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if info.payload().is::<PhpThrow>() {
                return;
            }
            default(info);
        }));
    });
}
