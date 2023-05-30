use common::fs_data::{BarABfCfgInfo, FooAIn, FooASflCfgInfo};
use common::fwk::{RefreshMode, Src};
use pushdepstovar::fs::boot::{get_bar_a_bf_raw, get_foo_a_sfl_raw};
use pushdepstovar::fs::{BarABfCfg, FooASflCfg, FooASflDeps};
use tokio;

pub async fn common_test(
    foo_a_sfl_cfg_info: FooASflCfgInfo,
    bar_a_bf_cfg_info: BarABfCfgInfo,
) -> Option<String> {
    let bar_cfg = BarABfCfg::new(
        Src::new_boxed(move || bar_a_bf_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );

    let bar_a_bf = get_bar_a_bf_raw(bar_cfg);

    let foo_cfg = FooASflCfg::new(
        Src::new_boxed(move || foo_a_sfl_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );

    let foo_deps = FooASflDeps { bar_a_bf };

    let foo_a_sfl = get_foo_a_sfl_raw(foo_cfg, foo_deps);

    let handle = tokio::spawn(async move { foo_a_sfl(FooAIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
