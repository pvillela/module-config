use common::fs_data::FooAIn;
use common::fwk::{arc_pin_async_fn, CfgDepsArc, RefreshMode};
use pushdepstovar::fs::{
    bar_a_bf, foo_a_sfl, BarABfCfgInfo, FooASflCfgInfo, FooASflDeps, BAR_A_BF_CFG_DEPS,
    FOO_A_SFL_CFG_DEPS,
};
use tokio;

pub async fn common_test(
    foo_sfl_cfg_info: FooASflCfgInfo,
    bar_bf_cfg_info: BarABfCfgInfo,
) -> Option<String> {
    CfgDepsArc::set(
        &FOO_A_SFL_CFG_DEPS,
        move || foo_sfl_cfg_info.clone(),
        RefreshMode::NoRefresh,
        FooASflDeps {
            bar_a_bf: arc_pin_async_fn(bar_a_bf),
        },
    );

    CfgDepsArc::set(
        &BAR_A_BF_CFG_DEPS,
        move || bar_bf_cfg_info.clone(),
        RefreshMode::NoRefresh,
        (),
    );

    let handle = tokio::spawn(async move { foo_a_sfl(FooAIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
