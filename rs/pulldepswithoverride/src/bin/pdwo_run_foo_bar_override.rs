use common::config::refresh_app_configuration;
use common::fs_data::{BarBfCfgInfo, FooSflCfgInfo};
use common::fs_util::bar_core;
use common::fwk::{set_once_cell, CfgOvd, RefreshMode, Src};
use pulldepswithoverride::fs::{
    foo_sfl, BAR_BF_CFG, BAR_BF_CFG_OVERRIDE, FOO_SFL_CFG_OVERRIDE, FOO_SFL_DEPS_OVERRIDE,
};
use std::thread;

fn bar_ovd_bf() -> String {
    let cfg = BAR_BF_CFG.with(|c| c.get_cfg());
    let u = cfg.u * 1000;
    let v = cfg.v.clone() + "-bar_ovd_bf";
    bar_core(u, v)
}

fn main() {
    let _ = CfgOvd::set_once_cell(
        &FOO_SFL_CFG_OVERRIDE,
        Some(Src::new_boxed(|| FooSflCfgInfo {
            a: "foo_override".to_owned(),
            b: 11,
        })),
        Some(RefreshMode::NoRefresh),
    );

    let _ = set_once_cell(
        &FOO_SFL_DEPS_OVERRIDE,
        pulldepswithoverride::fs::FooSflDeps { bar_bf: bar_ovd_bf },
    );

    let _ = CfgOvd::set_once_cell(
        &BAR_BF_CFG_OVERRIDE,
        Some(Src::new_boxed(|| BarBfCfgInfo {
            u: 33,
            v: "bar_override".to_owned(),
        })),
        Some(RefreshMode::NoRefresh),
    );

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- there should be no difference in output.");

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
