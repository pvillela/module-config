use pushtovar::config::refresh_app_configuration;
use pushtovar::fs::foo_sfl;
use pushtovar::startup::init_no_cache;
use std::thread;

fn main() {
    init_no_cache();

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- output sould be different.");

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
