use common::fs_data::{BarABfCfgInfo, FooAIn, FooASflCfgInfo};
use common::fwk::{static_closure_0_thread_safe, CfgOvd, RefreshMode};
use pulldepswithoverride::fs::{foo_a_sfl, BAR_A_BF_CFG_OVERRIDE, FOO_A_SFL_CFG_OVERRIDE};
use tokio;

pub async fn common_test(
    foo_a_sfl_cfg_info: FooASflCfgInfo,
    bar_a_bf_cfg_info: BarABfCfgInfo,
) -> Option<String> {
    let _ = CfgOvd::set_once_cell(
        &FOO_A_SFL_CFG_OVERRIDE,
        Some(static_closure_0_thread_safe(move || {
            foo_a_sfl_cfg_info.clone()
        })),
        Some(RefreshMode::NoRefresh),
    );

    let _ = CfgOvd::set_once_cell(
        &BAR_A_BF_CFG_OVERRIDE,
        Some(static_closure_0_thread_safe(move || {
            bar_a_bf_cfg_info.clone()
        })),
        Some(RefreshMode::NoRefresh),
    );

    let handle = tokio::spawn(async move { foo_a_sfl(FooAIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
