use pushdepstovar::config::refresh_app_configuration;
use pushdepstovar::fs::{
    bar_bf, foo_sfl, BarBfCfgInfo, FooSflCfgInfo, FooSflDeps, BAR_BF_CFG_SRC, FOO_SFL_CFG_SRC,
};
use pushdepstovar::fwk::update_cfg_src_with_fn;
use std::sync::Arc;
use std::thread;

fn main() {
    update_cfg_src_with_fn(
        &FOO_SFL_CFG_SRC,
        || {
            Arc::new(FooSflCfgInfo {
                a: "foo_override".to_owned(),
                b: 11,
            })
        },
        FooSflDeps { bar_bf },
    );

    update_cfg_src_with_fn(
        &BAR_BF_CFG_SRC,
        || {
            Arc::new(BarBfCfgInfo {
                u: 33,
                v: "bar_override".to_owned(),
            })
        },
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
