fn _assert_send_sync<T: Send + Sync>() {}
fn main() {
    tiny_repro::init();
    _assert_send_sync::<php_rt::Str>();
    _assert_send_sync::<php_rt::Mixed>();
    _assert_send_sync::<php_rt::Map<php_rt::Str, php_rt::Mixed>>();
    _assert_send_sync::<tiny_repro::tiny::counter::Counter>();

    // axis-7 increment 5 (mechanism): shared Send+Sync storage used across real OS threads.
    // Build a shared Map<Str,Mixed>, read it concurrently from 8 scoped threads.
    let mut m: php_rt::Map<php_rt::Str, php_rt::Mixed> = php_rt::Map::new();
    for i in 0..100i64 { m.insert(php_rt::Str::from_string(i.to_string()), php_rt::Mixed::Int(i)); }
    let shared = std::sync::Arc::new(m);
    let total = std::sync::atomic::AtomicI64::new(0);
    std::thread::scope(|s| {
        for _ in 0..8 {
            let sh = std::sync::Arc::clone(&shared);
            let tot = &total;
            s.spawn(move || {
                // concurrent reads of the shared map (Str interning + Map lookups across threads)
                let mut local = 0i64;
                for i in 0..100i64 {
                    if let Some(php_rt::Mixed::Int(v)) = sh.get(&php_rt::Str::from_string(i.to_string())) { local += v; }
                }
                tot.fetch_add(local, std::sync::atomic::Ordering::Relaxed);
            });
        }
    });
    // 8 threads * sum(0..100) = 8 * 4950 = 39600
    println!("threaded_reads={}", total.load(std::sync::atomic::Ordering::Relaxed));
    print!("{}", tiny_repro::g::run_all());
}
