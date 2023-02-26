use pushdepstovar_o::config::refresh_app_configuration;
use pushdepstovar_o::fs::{
    bar_bf, foo_sfl, BarBfCfgInfo, FooSflCfgInfo, FooSflDeps, BAR_BF_CFG_DEPS, FOO_SFL_CFG_DEPS,
};
use pushdepstovar_o::fwk::{CfgDeps, RefreshMode};
use std::sync::Arc;
use std::thread;

fn main() {
    CfgDeps::set(
        &FOO_SFL_CFG_DEPS,
        || {
            Arc::new(FooSflCfgInfo {
                a: "foo_override".to_owned(),
                b: 11,
            })
        },
        RefreshMode::NoRefresh,
        FooSflDeps { bar_bf },
    );

    CfgDeps::set(
        &BAR_BF_CFG_DEPS,
        || {
            Arc::new(BarBfCfgInfo {
                u: 33,
                v: "bar_override".to_owned(),
            })
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
