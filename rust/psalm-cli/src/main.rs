//! The transpiled Psalm as a command-line program, so it can analyse a project the way the
//! interpreted one does. Psalm recurses deeply, so the work runs on a thread with the same
//! large stack the test harness gives its workers.
//!
//! `PSALM_SRC_ROOT` points at the PHP tree the crate was transpiled from: that is where
//! `__DIR__` resolves, so the dictionaries and stubs Psalm loads relative to its own source
//! are found. It defaults to the checkout this crate was built in (two levels above the
//! crate), so it only has to be set when the workspace is built outside the PHP tree. The
//! project under analysis is simply the working directory, as with the interpreted CLI.

// Proving the generated exception type `Send` walks the whole object graph the program declares.
#![recursion_limit = "1024"]

use php_rt::error::PhpThrowable;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let src_root = std::env::var("PSALM_SRC_ROOT")
        .unwrap_or_else(|_| concat!(env!("CARGO_MANIFEST_DIR"), "/../..").to_string());
    let src_root = match std::fs::canonicalize(&src_root) {
        Ok(path) if path.join("src/Psalm").is_dir() => path.to_string_lossy().into_owned(),
        _ => {
            eprintln!("psalm-rs: no Psalm sources (src/Psalm) under {src_root}: set PSALM_SRC_ROOT to the PHP tree this binary was transpiled from");
            std::process::exit(2);
        }
    };

    let handle = std::thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(move || {
            php_rt::support::set_src_root(&src_root);
            // census printed when this thread ends (php-rt `stats` feature; no-op otherwise)
            let _census = php_rt::stats::PrintOnDrop;
            psalm_mono::init();

            // `PSALM_RS_TRACE_EXIT=1`: print where a PHP exit() with a non-zero status is raised,
            // which the interpreter shows through its own frames and the compiled program cannot.
            if std::env::var_os("PSALM_RS_TRACE_EXIT").is_some() {
                let previous = std::panic::take_hook();
                std::panic::set_hook(Box::new(move |info| {
                    if let Some(thrown) = info.payload().downcast_ref::<php_rt::error::PhpThrow>() {
                        if let Some(t) = thrown.0.downcast_ref::<psalm_mono::Throw>() {
                            if matches!(t.exit_status(), Some(status) if status != 0) {
                                eprintln!("exit({}) raised at:\n{}", t.exit_status().unwrap(), std::backtrace::Backtrace::force_capture());
                            }
                        }
                    }
                    previous(info);
                }));
            }

            let argv: php_rt::list::List<php_rt::string::Str> = php_rt::list::List::from_vec(
                args.iter().map(|a| php_rt::string::Str::from_string(a.clone())).collect(),
            );
            // the owned/borrowed analysis takes read-only static parameters by reference
            psalm_mono::psalm::internal::cli::psalm::Psalm::run(&argv);
        })
        .expect("spawn");

    // A PHP `exit()` and an uncaught exception both unwind out of the program as a panic
    // carrying the thrown value; report them the way the interpreter would.
    if let Err(payload) = handle.join() {
        match php_rt::error::take_thrown_opt::<psalm_mono::Throw>(payload) {
            Ok(thrown) => {
                if let Some(status) = thrown.exit_status() {
                    std::process::exit(status as i32);
                }
                eprintln!("Uncaught {}: {}", thrown.class_name(), thrown.message().to_string_lossy());
                let mut previous = thrown.getPrevious();
                while let Some(cause) = previous {
                    eprintln!("  caused by {}: {}", cause.class_name(), cause.message().to_string_lossy());
                    previous = cause.getPrevious();
                }
                std::process::exit(255);
            }
            Err(panic) => {
                let message = panic
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| panic.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_else(|| "non-string panic payload".to_string());
                eprintln!("internal error: {message}");
                std::process::exit(101);
            }
        }
    }
}
