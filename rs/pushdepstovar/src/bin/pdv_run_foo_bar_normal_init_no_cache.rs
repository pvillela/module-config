use common::config::refresh_app_configuration;
use pushdepstovar::fs::foo_sfl;
use pushdepstovar::startup::init_refreshable;
use std::thread;

fn main() {
    init_refreshable();

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- output should be different.");

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
