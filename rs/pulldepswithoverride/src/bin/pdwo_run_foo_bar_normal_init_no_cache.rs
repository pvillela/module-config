use pulldepswithoverride::config::refresh_app_configuration;
use pulldepswithoverride::fs::foo_sfl;
use pulldepswithoverride::startup::init_refreshable;
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
