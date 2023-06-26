use common::config::refresh_app_configuration;
use pushtofunction_old::startup::{make_foo_sfl, make_foo_sfl1};
use std::thread;

fn main() {
    {
        println!("Running foo_sfl in 2 threads.");
        let foo_sfl = make_foo_sfl();
        let foo_sfl_clone = foo_sfl.clone();

        let handle = thread::spawn(move || foo_sfl());
        let res = handle.join().unwrap();
        println!("{}", res);

        refresh_app_configuration();
        println!("App configuration refreshed -- output should be different.");

        let handle = thread::spawn(move || foo_sfl_clone());
        let res = handle.join().unwrap();
        println!("{}", res);
    }

    {
        println!("Running foo_sfl1 in 2 threads.");
        let foo_sfl1 = make_foo_sfl1();
        let foo_sfl1_clone = foo_sfl1.clone();

        let handle = thread::spawn(move || foo_sfl1());
        let res = handle.join().unwrap();
        println!("{}", res);

        refresh_app_configuration();
        println!("App configuration refreshed -- there should be no difference in output.");

        let handle = thread::spawn(move || foo_sfl1_clone());
        let res = handle.join().unwrap();
        println!("{}", res);
    }
}
