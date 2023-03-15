use std::{thread, time::Duration};

use common::config::refresh_app_configuration;
use pushtofunction::startup::make_foo_sfl_refreshable;

fn main() {
    let foo_sfl = make_foo_sfl_refreshable();
    let res = foo_sfl();
    println!("{}", res);

    refresh_app_configuration();
    thread::sleep(Duration::from_millis(70));
    println!("App configuration refreshed -- output should be different.");

    let res = foo_sfl();
    println!("{}", res);
}
