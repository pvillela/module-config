use common::config::refresh_app_configuration;
use pushdepstovar::fs::foo_sfl;
use pushdepstovar::startup::init_refreshable;
use std::thread;

fn main() {
    init_refreshable(60);

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    thread::sleep(std::time::Duration::from_millis(30));
    refresh_app_configuration();
    println!("App configuration refreshed -- there should be no difference in output.");

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    thread::sleep(std::time::Duration::from_millis(30));
    refresh_app_configuration();
    println!("App configuration refreshed -- output should be different.");

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
