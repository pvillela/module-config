use common::config::refresh_app_configuration;
use common::fs_data::{BarBfCfgInfo, FooSflCfgInfo};
use common::fwk::RefreshMode;
use pushtofunction::fs::{bar_bf, foo_sfl, FooASflDeps, BAR_BF_CFG_DEPS, FOO_SFL_CFG_DEPS};
use std::thread;

fn main() {
    FOO_SFL_CFG_DEPS.with(|c| {
        c.update_all(
            || FooSflCfgInfo {
                a: "foo_override".to_owned(),
                b: 11,
            },
            RefreshMode::NoRefresh,
            FooASflDeps { bar_bf },
        )
    });

    BAR_BF_CFG_DEPS.with(|c| {
        c.update_all(
            || BarBfCfgInfo {
                u: 33,
                v: "bar_override".to_owned(),
            },
            RefreshMode::NoRefresh,
            (),
        )
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