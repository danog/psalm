// The tiny suite's driver: initialise the transpiled crate and print every case's verdict. (The
// runtime is single-threaded by design now -- reference counts and cells are plain -- so the
// cross-thread guards this once asserted are gone with the atomics they exercised.)
fn main() {
    tiny_repro::init();
    print!("{}", tiny_repro::g::run_all());
}
