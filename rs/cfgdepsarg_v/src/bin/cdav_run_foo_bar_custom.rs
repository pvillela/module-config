use cfgdepsarg_v::fs::{boot::foo_sfl_boot, FooSflT};
use common::{
    config::{refresh_app_configuration, AppCfgInfo},
    fwk::RefreshMode,
};
use std::{sync::Arc, thread};

fn make_foo_sfl1() -> FooSflT {
    let app_cfg_src1 = move || {
        Arc::new(AppCfgInfo {
            x: "custom".to_owned(),
            y: 84,
            z: true,
        })
    };
    foo_sfl_boot(app_cfg_src1, RefreshMode::NoRefresh)
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
