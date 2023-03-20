use common::fs_data::{BarABfCfgInfo, FooAIn, FooASflCfgInfo};
use common::fwk::{
    arc_pin_async_fn, set_once_cell, static_closure_0_thread_safe, CfgDef, RefreshMode,
};
use pushdepstovar::fs::{
    bar_a_bf, foo_a_sfl, FooASflDeps, BAR_A_BF_CFG_DEF, FOO_A_SFL_CFG_DEF, FOO_A_SFL_DEPS,
};
use tokio;

pub async fn common_test(
    foo_a_sfl_cfg_info: FooASflCfgInfo,
    bar_a_bf_cfg_info: BarABfCfgInfo,
) -> Option<String> {
    let _ = CfgDef::set_once_cell_with_cfg_src(
        &FOO_A_SFL_CFG_DEF,
        static_closure_0_thread_safe(move || foo_a_sfl_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );

    let _ = set_once_cell(
        &FOO_A_SFL_DEPS,
        FooASflDeps {
            bar_a_bf: arc_pin_async_fn(bar_a_bf),
        },
    );

    let _ = CfgDef::set_once_cell_with_cfg_src(
        &BAR_A_BF_CFG_DEF,
        static_closure_0_thread_safe(move || bar_a_bf_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );

    let handle = tokio::spawn(async move { foo_a_sfl(FooAIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
