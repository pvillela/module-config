use cfgdepsarg_r::startup::make_foo_sfl_refreshable;
use common::config::refresh_app_configuration;
use std::{thread, time::Duration};

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
