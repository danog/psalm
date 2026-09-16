fn _assert_send_sync<T: Send + Sync>() {}
fn main() {
    tiny_repro::init();
    _assert_send_sync::<php_rt::Str>();
    _assert_send_sync::<php_rt::Mixed>();
    _assert_send_sync::<php_rt::Map<php_rt::Str, php_rt::Mixed>>();
    _assert_send_sync::<tiny_repro::tiny::counter::Counter>();
    // threaded read guard
    let mut m: php_rt::Map<php_rt::Str, php_rt::Mixed> = php_rt::Map::new();
    for i in 0..100i64 { m.insert(php_rt::Str::from_string(i.to_string()), php_rt::Mixed::Int(i)); }
    let shared = std::sync::Arc::new(m);
    let total = std::sync::atomic::AtomicI64::new(0);
    std::thread::scope(|s| {
        for _ in 0..8 {
            let sh = std::sync::Arc::clone(&shared);
            let tot = &total;
            s.spawn(move || {
                let mut local = 0i64;
                for i in 0..100i64 { if let Some(php_rt::Mixed::Int(v)) = sh.get(&php_rt::Str::from_string(i.to_string())) { local += v; } }
                tot.fetch_add(local, std::sync::atomic::Ordering::Relaxed);
            });
        }
    });
    assert_eq!(total.load(std::sync::atomic::Ordering::Relaxed), 39600);
    // threaded mutation guard
    let c = std::sync::Arc::new(php_rt::support::RwCell::new(0i64));
    std::thread::scope(|s| {
        for _ in 0..8 { let c = std::sync::Arc::clone(&c); s.spawn(move || { for _ in 0..1000 { *c.borrow_mut() += 1; } }); }
    });
    assert_eq!(*c.borrow(), 8000);
    print!("{}", tiny_repro::g::run_all());
}
