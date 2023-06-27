use common::config::refresh_app_configuration;
use pulldepswithoverride::fs::foo_i_sfl;
use std::thread;

fn main() {
    println!("Running with immutable configuration.");

    let handle = thread::spawn(move || foo_i_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- output should be identical.");

    let handle = thread::spawn(move || foo_i_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
