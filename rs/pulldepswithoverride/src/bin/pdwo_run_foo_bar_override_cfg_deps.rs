use common::config::refresh_app_configuration;
use common::fs_data::{BarBfCfgInfo, FooSflCfgInfo};
use common::fs_util::bar_core;
use common::fwk::{RefreshMode, Src};
use pulldepswithoverride::fs::{
    foo_sfl, BarBfCfg, FooSflCfg, FooSflDeps, BAR_BF_CFG, BAR_BF_CFG_TL, FOO_SFL_CFG_DEPS,
};
use std::thread;

fn bar_ovd_bf() -> String {
    let cfg = BAR_BF_CFG_TL.with(|c| c.get_cfg());
    let u = cfg.u * 1000;
    let v = cfg.v.clone() + "-bar_ovd_bf";
    bar_core(u, v)
}

fn main() {
    FOO_SFL_CFG_DEPS.set_cfg_strict({
        let src = Src::Fn(|| FooSflCfgInfo {
            a: "a from foo_sfl_cfg_override".to_owned(),
            b: 4200,
        });
        FooSflCfg::new(src, RefreshMode::NoRefresh)
    });

    FOO_SFL_CFG_DEPS.set_deps_strict(FooSflDeps { bar_bf: bar_ovd_bf });

    BAR_BF_CFG.set_cfg_strict({
        let src = Src::Fn(|| BarBfCfgInfo {
            u: 1100,
            v: "u from bar_bf_cfg_override".to_owned(),
        });
        BarBfCfg::new(src, RefreshMode::NoRefresh)
    });

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- there should be no difference in output.");

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
