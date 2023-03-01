use common::config::refresh_app_configuration;
use common::fwk::{CfgDeps, RefreshMode};
use pulldepswithoverride::fs::{foo_sfl, BAR_BF_CFG_DEPS, FOO_SFL_CFG_DEPS};
use std::thread;
use std::time::Duration;

fn main() {
    CfgDeps::update_static_refresh_mode(
        &FOO_SFL_CFG_DEPS,
        RefreshMode::Refreshable(Duration::from_millis(0)),
    );

    CfgDeps::update_static_refresh_mode(
        &BAR_BF_CFG_DEPS,
        RefreshMode::Refreshable(Duration::from_millis(0)),
    );

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- output should be different.");

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
