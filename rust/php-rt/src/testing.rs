//! Support for the generated PHPUnit test harness.

use std::fmt::Display;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use crate::error::PhpThrowable;
use crate::Str;

const STACK_SIZE: usize = 512 * 1024 * 1024;

/// Whether a thrown exception is PHPUnit's "skipped"/"incomplete" marker (the test then counts as passed).
pub fn is_skip<E: PhpThrowable>(e: &E) -> bool {
    let name = e.class_name().to_ascii_lowercase();
    if name == "phpunit\\framework\\skippedtesterror" || name == "phpunit\\framework\\incompletetesterror" {
        return true;
    }
    e.class_ancestors().iter().any(|a| {
        *a == "phpunit\\framework\\skippedtesterror" || *a == "phpunit\\framework\\incompletetesterror"
    })
}

/// A human-readable message for a caught PHP exception: "Class: message".
pub fn exc_message<E: PhpThrowable>(e: &E) -> String {
    format!("{}: {}", e.class_name(), e.message().to_string_lossy())
}

/// Runs one test method on a thread with a large stack; PHP exceptions become panics with the PHP message.
pub fn run<E: PhpThrowable + Display>(name: &str, root: &str, f: impl FnOnce() -> Result<(), E> + Send + 'static) {
    let root = root.to_string();
    let test_name = name.to_string();
    let handle = std::thread::Builder::new()
        .name(test_name.clone())
        .stack_size(STACK_SIZE)
        .spawn(move || -> Result<(), (bool, String)> {
            crate::support::set_src_root(&root);
            // tests resolve fixtures and stubs relative to the repository root
            let _ = std::env::set_current_dir(&root);
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
pub fn case(dataset: Str, f: impl FnOnce()) {
    if let Err(payload) = catch_unwind(AssertUnwindSafe(f)) {
        eprintln!("[data set \"{}\"]", dataset);
        resume_unwind(payload);
    }
}

/// The message of a panic payload.
fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "panic".to_string()
    }
}

/// Runs `f` on a thread with a large stack (the source root set, the working directory at the root);
/// a PHP exception or a panic becomes the error message.
pub fn in_thread<T: Send + 'static, E: PhpThrowable>(root: &str, f: impl FnOnce() -> T + Send + 'static) -> Result<T, String> {
    let root = root.to_string();
    let handle = std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(move || -> Result<T, String> {
            crate::support::set_src_root(&root);
            let _ = std::env::set_current_dir(&root);
            match catch_unwind(AssertUnwindSafe(f)) {
                Ok(v) => Ok(v),
                Err(payload) => match crate::error::take_thrown_opt::<E>(payload) {
                    Ok(e) => Err(exc_message(&e)),
                    Err(p) => Err(panic_message(&*p)),
                },
            }
        })
        .expect("spawn test thread");
    match handle.join() {
        Ok(r) => r,
        Err(payload) => Err(panic_message(&*payload)),
    }
}

/// Runs one trial (a test method, or one data set of it): `Ok` when it passes or is skipped (the skip
/// is printed), `Err(message)` when it fails or panics.
pub fn run_trial<E: PhpThrowable + Display>(name: &str, root: &str, f: impl FnOnce() -> Result<(), E> + Send + 'static) -> Result<(), String> {
    let root = root.to_string();
    let test_name = name.to_string();
    let handle = std::thread::Builder::new()
        .name(test_name.clone())
        .stack_size(STACK_SIZE)
        .spawn(move || -> Result<(), (bool, String)> {
            crate::support::set_src_root(&root);
            let _ = std::env::set_current_dir(&root);
            match f() {
                Ok(()) => Ok(()),
                Err(e) => Err((is_skip(&e), e.to_string())),
            }
        })
        .expect("spawn test thread");
    match handle.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err((true, msg))) => {
            eprintln!("[skipped] {}: {}", test_name, msg);
            Ok(())
        }
        Ok(Err((false, msg))) => Err(msg),
        Err(payload) => Err(format!("panicked: {}", panic_message(&*payload))),
    }
}

/// Runs one data set on the current (worker) thread: `Ok` when it passes or is skipped, `Err(message)`
/// when it fails or panics (the panic is contained so the worker can serve the next data set).
pub fn run_row<E: PhpThrowable>(name: &str, f: impl FnOnce()) -> Result<(), String> {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(()) => Ok(()),
        Err(payload) => match crate::error::take_thrown_opt::<E>(payload) {
            Ok(e) if is_skip(&e) => {
                eprintln!("[skipped] {}: {}", name, exc_message(&e));
                Ok(())
            }
            Ok(e) => Err(exc_message(&e)),
            Err(p) => Err(format!("panicked: {}", panic_message(&*p))),
        },
    }
}

/// A request to a test method's worker thread: the data set to run and where to send the outcome.
pub type RowRequest = (usize, std::sync::mpsc::Sender<Result<(), String>>);

/// The lazily started worker threads of a test method with a data provider (PHP static state is
/// thread-local, so a worker keeps the state of the rows it ran): data set `i` runs on worker `i % n`,
/// which lets a thread pool run several rows of one method at a time.
pub type RowWorker = std::sync::Arc<std::sync::Mutex<Vec<Option<std::sync::mpsc::Sender<RowRequest>>>>>;

/// Number of worker threads per test method (each evaluates the data provider once).
pub const ROW_WORKERS: usize = 2;

pub fn new_row_worker() -> RowWorker {
    std::sync::Arc::new(std::sync::Mutex::new((0..ROW_WORKERS).map(|_| None).collect()))
}

/// Spawns the worker of a test method (`serve` receives the requests, runs on a large stack at the root).
pub fn spawn_row_worker(root: &str, serve: impl FnOnce(std::sync::mpsc::Receiver<RowRequest>) + Send + 'static) -> std::sync::mpsc::Sender<RowRequest> {
    let (tx, rx) = std::sync::mpsc::channel::<RowRequest>();
    let root = root.to_string();
    std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(move || {
            crate::support::set_src_root(&root);
            let _ = std::env::set_current_dir(&root);
            serve(rx);
        })
        .expect("spawn test worker thread");
    tx
}

/// Runs data set `i` on the method's worker (started on first use), returning the outcome.
pub fn run_on_worker(worker: &RowWorker, root: &str, i: usize, start: impl FnOnce(std::sync::mpsc::Receiver<RowRequest>) + Send + 'static) -> Result<(), String> {
    let tx = {
        let mut guard = worker.lock().unwrap_or_else(|e| e.into_inner());
        let slot = i % guard.len().max(1);
        if guard[slot].is_none() {
            guard[slot] = Some(spawn_row_worker(root, start));
        }
        guard[slot].as_ref().unwrap().clone()
    };
    let (reply_tx, reply_rx) = std::sync::mpsc::channel();
    if tx.send((i, reply_tx)).is_err() {
        return Err("test worker thread is gone".to_string());
    }
    reply_rx.recv().unwrap_or_else(|_| Err("test worker thread died".to_string()))
}

/// Orders the trials of many test methods so that a thread pool running them in order works on
/// different methods at the same time: the groups are taken `width` at a time and their trials
/// interleaved round-robin (data sets of one method serialize on that method's worker thread).
pub fn interleave<T>(groups: Vec<Vec<T>>, width: usize) -> Vec<T> {
    let mut out = Vec::new();
    let mut groups: Vec<std::collections::VecDeque<T>> = groups.into_iter().map(|g| g.into_iter().collect()).collect();
    for chunk in groups.chunks_mut(width.max(1)) {
        loop {
            let mut any = false;
            for g in chunk.iter_mut() {
                if let Some(t) = g.pop_front() {
                    out.push(t);
                    any = true;
                }
            }
            if !any {
                break;
            }
        }
    }
    out
}


// ---------------------------------------------------------------- warm worker pool

/// A trial job: runs a test method or one of its data sets, `Err(message)` on failure.
pub type Job = Box<dyn FnOnce() -> Result<(), String> + Send>;

type JobRequest = (Job, std::sync::mpsc::Sender<Result<(), String>>);

static POOL: std::sync::OnceLock<std::sync::Mutex<std::sync::mpsc::Sender<JobRequest>>> = std::sync::OnceLock::new();

/// Number of warm worker threads: like PHPUnit processes, each keeps its PHP static state across the
/// tests it runs (tests reset what they need in setUp), so nothing is re-scanned per test. One worker per
/// core by default (each holds its own copy of that state, ~700 MB once the stubs are scanned);
/// `PSALM_TEST_WORKERS` overrides it for a memory-constrained machine.
fn pool_workers() -> usize {
    if let Ok(v) = std::env::var("PSALM_TEST_WORKERS") {
        if let Ok(n) = v.parse::<usize>() {
            if n > 0 {
                return n;
            }
        }
    }
    std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4)
}

fn pool() -> &'static std::sync::Mutex<std::sync::mpsc::Sender<JobRequest>> {
    POOL.get_or_init(|| {
        let (tx, rx) = std::sync::mpsc::channel::<JobRequest>();
        let rx = std::sync::Arc::new(std::sync::Mutex::new(rx));
        for _ in 0..pool_workers() {
            let rx = rx.clone();
            std::thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(move || loop {
                    let next = rx.lock().unwrap_or_else(|e| e.into_inner()).recv();
                    let Ok((job, reply)) = next else { break };
                    let outcome = match catch_unwind(AssertUnwindSafe(job)) {
                        Ok(r) => r,
                        Err(payload) => Err(format!("panicked: {}", panic_message(&*payload))),
                    };
                    let _ = reply.send(outcome);
                })
                .expect("spawn test worker thread");
        }
        std::sync::Mutex::new(tx)
    })
}

/// Runs a job on the warm pool and returns its outcome.
pub fn run_on_pool(root: &str, job: Job) -> Result<(), String> {
    let root = root.to_string();
    let job: Job = Box::new(move || {
        crate::support::set_src_root(&root);
        let _ = std::env::set_current_dir(&root);
        job()
    });
    let (reply_tx, reply_rx) = std::sync::mpsc::channel();
    if pool().lock().unwrap_or_else(|e| e.into_inner()).send((job, reply_tx)).is_err() {
        return Err("test worker pool is gone".to_string());
    }
    reply_rx.recv().unwrap_or_else(|_| Err("test worker thread died".to_string()))
}

thread_local! {
    static ROWS: std::cell::RefCell<crate::FastMap<&'static str, std::sync::Arc<dyn std::any::Any>>> = std::cell::RefCell::new(crate::fast_map());
}

/// The data-provider rows of a test method on this worker thread, computed once per thread.
pub fn cached_rows<T: 'static, R>(key: &'static str, compute: impl FnOnce() -> T, f: impl FnOnce(&T) -> R) -> R {
    let cached = ROWS.with(|r| r.borrow().get(key).cloned());
    let rows: std::sync::Arc<dyn std::any::Any> = match cached {
        Some(r) => r,
        None => {
            let r: std::sync::Arc<dyn std::any::Any> = std::sync::Arc::new(compute());
            ROWS.with(|c| c.borrow_mut().insert(key, r.clone()));
            r
        }
    };
    f(rows.downcast_ref::<T>().expect("data provider row type"))
}
