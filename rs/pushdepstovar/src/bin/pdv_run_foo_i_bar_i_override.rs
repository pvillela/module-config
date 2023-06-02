use common::config::refresh_app_configuration;
use common::fs_data::{BarIBfCfgInfo, FooISflCfgInfo};
use common::fs_util::bar_core;
use pushdepstovar::fs::{get_foo_i_sfl_raw, FooISflDeps};
use std::thread;

fn bar_i_ovd_bf() -> String {
    let cfg = BarIBfCfgInfo {
        u: 33,
        v: "bar_i_override_cfg_info".to_owned(),
    };
    let u = cfg.u * 1000;
    let v = cfg.v.clone() + "-bar_i_ovd_bf";
    bar_core(u, v)
}

fn main() {
    println!("Running with immutable overridden configuration.");

    let foo_cfg = FooISflCfgInfo {
        a: "foo_i_override_cfg_info".to_owned(),
        b: 11,
    };

    let foo_deps = FooISflDeps {
        bar_i_bf: bar_i_ovd_bf,
    };

    let foo_i_sfl = get_foo_i_sfl_raw(foo_cfg, foo_deps);

    let handle = thread::spawn(move || foo_i_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- there should be no difference in output.");

    let handle = thread::spawn(move || foo_i_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
