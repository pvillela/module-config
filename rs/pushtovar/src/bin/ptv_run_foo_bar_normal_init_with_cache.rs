use pushtovar::fs::foo_sfl;
use pushtovar::startup::init_with_cache;
use std::thread;

fn main() {
    init_with_cache();

    let handle1 = thread::spawn(move || {
        foo_sfl();
    });

    let _ = handle1.join();

    foo_sfl();
}
