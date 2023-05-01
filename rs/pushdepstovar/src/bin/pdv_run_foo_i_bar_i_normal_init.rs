use common::config::refresh_app_configuration;
use pushdepstovar::fs::foo_i_sfl;
use pushdepstovar::startup::init_i;
use std::thread;

fn main() {
    init_i();

    let handle = thread::spawn(move || foo_i_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- output should be different.");

    let handle = thread::spawn(move || foo_i_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
