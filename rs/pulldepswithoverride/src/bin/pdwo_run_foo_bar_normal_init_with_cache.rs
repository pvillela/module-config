use pulldepswithoverride::config::refresh_app_configuration;
use pulldepswithoverride::fs::foo_sfl;
use pulldepswithoverride::startup::init_no_refresh;
use std::thread;

fn main() {
    init_no_refresh();

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- there should be no difference in output.");

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
