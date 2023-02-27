use pushcfgtovar::config::refresh_app_configuration;
use pushcfgtovar::fs::foo_sfl;
use pushcfgtovar::startup::init_no_cache;
use std::thread;

fn main() {
    init_no_cache();

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- output should be different.");

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
