fn _assert_send_sync<T: Send + Sync>() {}
fn main() {
    tiny_repro::init();
    _assert_send_sync::<php_rt::Str>();
    _assert_send_sync::<php_rt::Mixed>();          // axis-7 increment 4: Mixed is now Send+Sync
    _assert_send_sync::<php_rt::Map<php_rt::Str, php_rt::Mixed>>();
    _assert_send_sync::<tiny_repro::tiny::counter::Counter>();
    print!("{}", tiny_repro::g::run_all());
}
