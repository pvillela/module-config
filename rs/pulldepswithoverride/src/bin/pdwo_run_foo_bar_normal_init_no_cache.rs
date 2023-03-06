use common::config::refresh_app_configuration;
use common::fwk::RefreshMode;
use pulldepswithoverride::fs::{foo_sfl, BAR_BF_CFG_DEPS, FOO_SFL_CFG_DEPS};
use std::thread;
use std::time::Duration;

fn main() {
    FOO_SFL_CFG_DEPS
        .with(|c| c.update_refresh_mode(RefreshMode::Refreshable(Duration::from_millis(0))));
    BAR_BF_CFG_DEPS
        .with(|c| c.update_refresh_mode(RefreshMode::Refreshable(Duration::from_millis(0))));

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- output should be different.");

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
