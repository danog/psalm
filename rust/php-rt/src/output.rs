//! Output (echo/print) with output-buffering support.

use crate::string::Str;
use std::cell::RefCell;
use std::io::Write;

thread_local! {
    static BUFFERS: RefCell<Vec<Vec<u8>>> = RefCell::new(Vec::new());
}

pub fn echo(b: &[u8]) {
    BUFFERS.with(|bufs| {
        let mut bufs = bufs.borrow_mut();
        if let Some(top) = bufs.last_mut() {
            top.extend_from_slice(b);
        } else {
            let out = std::io::stdout();
            let mut lock = out.lock();
            let _ = lock.write_all(b);
            let _ = lock.flush();
        }
    });
}
pub fn echo_str<T: crate::traits::ToStr>(v: T) {
    echo(v.to_php_str().as_bytes());
}
pub fn ob_start() -> bool {
    BUFFERS.with(|b| b.borrow_mut().push(Vec::new()));
    true
}
pub fn ob_get_clean() -> Option<Str> {
    BUFFERS.with(|b| b.borrow_mut().pop().map(Str::from_vec))
}
pub fn ob_get_contents() -> Option<Str> {
    BUFFERS.with(|b| b.borrow().last().map(|v| Str::from_bytes(v)))
}
pub fn ob_end_clean() -> bool {
    BUFFERS.with(|b| b.borrow_mut().pop().is_some())
}
pub fn ob_get_level() -> i64 {
    BUFFERS.with(|b| b.borrow().len() as i64)
}
pub fn ob_end_flush() -> bool {
    let top = BUFFERS.with(|b| b.borrow_mut().pop());
    match top {
        Some(v) => {
            echo(&v);
            true
        }
        None => false,
    }
}
pub fn eprint(b: &[u8]) {
    let err = std::io::stderr();
    let mut lock = err.lock();
    let _ = lock.write_all(b);
}
