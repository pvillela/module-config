use pushdepstovar::fs::{
    bar_a_bf, foo_a_sfl, BarABfCfgInfo, FooASflCfgInfo, FooASflDeps, BAR_A_BF_CFG_DEPS,
    FOO_A_SFL_CFG_DEPS,
};
use pushdepstovar::fwk::{box_pin_async_fn, CfgDeps};
use std::sync::Arc;
use tokio;

pub async fn common_test(
    foo_sfl_cfg_info: FooASflCfgInfo,
    bar_bf_cfg_info: BarABfCfgInfo,
) -> Option<String> {
    CfgDeps::set(
        &FOO_A_SFL_CFG_DEPS,
        move || Arc::new(foo_sfl_cfg_info.clone()),
        FooASflDeps {
            bar_a_bf: box_pin_async_fn(bar_a_bf),
        },
    );

    CfgDeps::set(
        &BAR_A_BF_CFG_DEPS,
        move || Arc::new(bar_bf_cfg_info.clone()),
        (),
    );

    let handle = tokio::spawn(async move { foo_a_sfl(0).await });
    let res = handle.await.ok();
    println!("{:?}", res);
    res
}
