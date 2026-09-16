// axis-7 guard: core storage types must stay Send+Sync (compile-time).
fn _assert_send_sync<T: Send + Sync>() {}
fn main() {
    tiny_repro::init();
    _assert_send_sync::<php_rt::Str>();
    _assert_send_sync::<php_rt::Map<php_rt::Str, i64>>();
    _assert_send_sync::<php_rt::List<php_rt::Str>>();
    _assert_send_sync::<tiny_repro::tiny::counter::Counter>();
    print!("{}", tiny_repro::g::run_all());
}
