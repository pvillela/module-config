use common::fwk::AppCfg;
use common::{
    config::{refresh_app_configuration, AppCfgInfo},
    fwk::RefreshMode,
};
use pushtofunction::fs;
use pushtofunction::fs::FooSflT;
use std::{sync::Arc, thread};

fn make_foo_sfl1() -> FooSflT {
    let app_cfg_src1 = move || {
        Arc::new(AppCfgInfo {
            x: "custom".to_owned(),
            y: 84,
            z: true,
        })
    };
    fs::foo_sfl_boot(AppCfg {
        app_src: app_cfg_src1,
        refresh_mode: RefreshMode::NoRefresh,
    })
}

fn main() {
    println!("Running custom foo_sfl in 2 threads.");

    let handle = thread::spawn(move || make_foo_sfl1()());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- there should be no difference in output.");

    let handle = thread::spawn(move || make_foo_sfl1()());
    let res = handle.join().unwrap();
    println!("{}", res);
}
