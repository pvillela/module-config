use common::fs_data::BarBfCfgInfo;
use common::fs_data::{FooAIn, FooSflCfgInfo};
use common::fwk::{box_pin_async_fn, CfgDeps, RefreshMode};
use pulldepswithoverride::fs::{
    bar_a_bf, foo_a_sfl, FooASflDeps, BAR_A_BF_CFG_DEPS, FOO_A_SFL_CFG_DEPS,
};
use tokio;

pub async fn common_test(
    foo_sfl_cfg_info: FooSflCfgInfo,
    bar_bf_cfg_info: BarBfCfgInfo,
) -> Option<String> {
    CfgDeps::set_static(
        &FOO_A_SFL_CFG_DEPS,
        move || foo_sfl_cfg_info.clone().into(),
        RefreshMode::NoRefresh,
        FooASflDeps {
            bar_a_bf: box_pin_async_fn(bar_a_bf),
        },
    );

    CfgDeps::set_static(
        &BAR_A_BF_CFG_DEPS,
        move || bar_bf_cfg_info.clone().into(),
        RefreshMode::NoRefresh,
        (),
    );

    let handle = tokio::spawn(async move { foo_a_sfl(FooAIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
