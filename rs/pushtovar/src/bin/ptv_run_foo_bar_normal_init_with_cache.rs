use pushtovar::config::refresh_app_configuration;
use pushtovar::fs::foo_sfl;
use pushtovar::startup::init_with_cache;
use std::thread;

fn main() {
    init_with_cache();

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- there should be no difference in output.");

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
