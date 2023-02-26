use pushdepstovar::fs::{
    bar_a_bf, foo_a_sfl, BarABfCfgInfo, FooAIn, FooASflCfgInfo, FooASflDeps, BAR_A_BF_CFG_DEPS,
    FOO_A_SFL_CFG_DEPS,
};
use pushdepstovar::fwk::{box_pin_async_fn, CfgDeps, RefreshMode};
use std::sync::Arc;
use tokio;

pub async fn common_test(
    foo_sfl_cfg_src: fn() -> Arc<FooASflCfgInfo>,
    bar_bf_cfg_src: fn() -> Arc<BarABfCfgInfo>,
) -> Option<String> {
    CfgDeps::set(
        &FOO_A_SFL_CFG_DEPS,
        foo_sfl_cfg_src,
        RefreshMode::NoRefresh,
        FooASflDeps {
            bar_a_bf: box_pin_async_fn(bar_a_bf),
        },
    );

    CfgDeps::set(
        &BAR_A_BF_CFG_DEPS,
        bar_bf_cfg_src,
        RefreshMode::NoRefresh,
        (),
    );

    let handle = tokio::spawn(async move { foo_a_sfl(FooAIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
