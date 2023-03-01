use pulldepswithoverride::{
    config::refresh_app_configuration,
    fs::{foo_sfl, BAR_BF_CFG_DEPS, FOO_SFL_CFG_DEPS},
    fwk::{CfgDeps, RefreshMode},
};
use std::thread;

fn main() {
    CfgDeps::update_refresh_mode(&FOO_SFL_CFG_DEPS, RefreshMode::NoRefresh);
    CfgDeps::update_refresh_mode(&BAR_BF_CFG_DEPS, RefreshMode::NoRefresh);

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- there should be no difference in output.");

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
