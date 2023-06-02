use common::config::refresh_app_configuration;
use common::fs_data::{BarBfCfgInfo, FooSflCfgInfo};
use common::fs_util::bar_core;
use common::fwk::{RefreshMode, Src};
use pushdepstovar::fs::get_foo_sfl_raw;
use pushdepstovar::fs::{FooSflCfg, FooSflDeps};
use std::thread;

fn bar_ovd_bf() -> String {
    let cfg = BarBfCfgInfo {
        u: 33,
        v: "bar_override".to_owned(),
    };
    let u = cfg.u * 1000;
    let v = cfg.v.clone() + "-bar_ovd_bf";
    bar_core(u, v)
}

fn main() {
    let foo_cfg = FooSflCfg::new(
        Src::new_boxed(|| FooSflCfgInfo {
            a: "foo_override".to_owned(),
            b: 11,
        }),
        RefreshMode::NoRefresh,
    );

    let foo_deps = FooSflDeps { bar_bf: bar_ovd_bf };

    let foo_sfl = get_foo_sfl_raw(foo_cfg, foo_deps);

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- there should be no difference in output.");

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
