use common::config::refresh_app_configuration;
use common::fs_data::{BarIBfCfgInfo, FooISflCfgInfo};
use common::fs_util::bar_core;
use pulldepswithoverride::fs::{foo_i_sfl, FooISflDeps, FOO_I_SFL_CFG_DEPS};
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

    FOO_I_SFL_CFG_DEPS.set_cfg_strict(FooISflCfgInfo {
        a: "foo_i_override_cfg_info".to_owned(),
        b: 11,
    });

    FOO_I_SFL_CFG_DEPS.set_deps_strict(FooISflDeps {
        bar_i_bf: bar_i_ovd_bf,
    });

    let handle = thread::spawn(move || foo_i_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- there should be no difference in output.");

    let handle = thread::spawn(move || foo_i_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
