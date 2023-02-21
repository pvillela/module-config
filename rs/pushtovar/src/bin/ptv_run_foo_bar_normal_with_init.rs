use pushtovar::fs::foo_sfl;
use pushtovar::startup::initialize;
use std::thread;

fn main() {
    initialize();

    let handle1 = thread::spawn(move || {
        foo_sfl();
    });

    let _ = handle1.join();

    foo_sfl();
}
