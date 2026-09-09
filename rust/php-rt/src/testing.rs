//! Support for the generated PHPUnit test harness.

use std::fmt::Display;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use crate::mixed::PhpObject;
use crate::Str;

const STACK_SIZE: usize = 512 * 1024 * 1024;

/// Whether a thrown object is PHPUnit's "skipped"/"incomplete" marker (the test then counts as passed).
pub fn is_skip<E: PhpObject>(e: &E) -> bool {
    let name = e.class_name().to_ascii_lowercase();
    if name == "phpunit\\framework\\skippedtesterror" || name == "phpunit\\framework\\incompletetesterror" {
        return true;
    }
    e.class_ancestors().iter().any(|a| {
        *a == "phpunit\\framework\\skippedtesterror" || *a == "phpunit\\framework\\incompletetesterror"
    })
}

/// Runs one test method on a thread with a large stack; PHP exceptions become panics with the PHP message.
pub fn run<E: PhpObject + Display>(name: &str, root: &str, f: impl FnOnce() -> Result<(), E> + Send + 'static) {
    let root = root.to_string();
    let test_name = name.to_string();
    let handle = std::thread::Builder::new()
        .name(test_name.clone())
        .stack_size(STACK_SIZE)
        .spawn(move || -> Result<(), (bool, String)> {
            crate::support::set_src_root(&root);
            match f() {
                Ok(()) => Ok(()),
                Err(e) => Err((is_skip(&e), e.to_string())),
            }
        })
        .expect("spawn test thread");
    match handle.join() {
        Ok(Ok(())) => {}
        Ok(Err((true, msg))) => {
            eprintln!("[skipped] {}: {}", test_name, msg);
        }
        Ok(Err((false, msg))) => panic!("{}", msg),
        Err(payload) => resume_unwind(payload),
    }
}

/// Runs one data set of a test; skips are swallowed, failures are annotated with the data set name.
pub fn case<E: PhpObject + Display>(dataset: Str, f: impl FnOnce() -> Result<(), E>) -> Result<(), E> {
    let result = catch_unwind(AssertUnwindSafe(f));
    match result {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) if is_skip(&e) => {
            eprintln!("[skipped] data set \"{}\": {}", dataset, e);
            Ok(())
        }
        Ok(Err(e)) => {
            eprintln!("[failed] data set \"{}\": {}", dataset, e);
            Err(e)
        }
        Err(payload) => {
            eprintln!("[panicked] data set \"{}\"", dataset);
            resume_unwind(payload)
        }
    }
}
