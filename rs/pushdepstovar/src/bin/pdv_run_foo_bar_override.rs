use common::config::refresh_app_configuration;
use common::fwk::{CfgDepsArc, RefreshMode};
use pushdepstovar::fs::{
    bar_bf, foo_sfl, BarBfCfgInfo, FooSflCfgInfo, FooSflDeps, BAR_BF_CFG_DEPS, FOO_SFL_CFG_DEPS,
};
use std::thread;

fn main() {
    CfgDepsArc::set(
        &FOO_SFL_CFG_DEPS,
        || FooSflCfgInfo {
            a: "foo_override".to_owned(),
            b: 11,
        },
        RefreshMode::NoRefresh,
        FooSflDeps { bar_bf },
    );

    CfgDepsArc::set(
        &BAR_BF_CFG_DEPS,
        || BarBfCfgInfo {
            u: 33,
            v: "bar_override".to_owned(),
        },
        RefreshMode::NoRefresh,
        (),
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
