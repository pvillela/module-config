use common::fs_data::{BarABfCfgInfo, FooAIn, FooASflCfgInfo};
use common::fwk::{RefreshMode, Src};
use pulldepswithoverride::fs::{
    foo_a_sfl, BarABfCfg, FooASflCfg, BAR_A_BF_CFG, FOO_A_SFL_CFG_DEPS,
};
use tokio;

pub async fn common_test(
    foo_a_sfl_cfg_info: FooASflCfgInfo,
    bar_a_bf_cfg_info: BarABfCfgInfo,
) -> Option<String> {
    let foo_a_sfl_cfg = {
        let src = Src::new_boxed(move || foo_a_sfl_cfg_info.clone());
        FooASflCfg::new(src, RefreshMode::NoRefresh)
    };
    FOO_A_SFL_CFG_DEPS.set_cfg_strict(foo_a_sfl_cfg);

    let bar_a_bf_cfg = {
        let src = Src::new_boxed(move || bar_a_bf_cfg_info.clone());
        BarABfCfg::new(src, RefreshMode::NoRefresh)
    };
    assert!(
        BAR_A_BF_CFG.set(bar_a_bf_cfg).is_ok(),
        "BAR_BF_CFG already initialized"
    );

    let handle = tokio::spawn(async move { foo_a_sfl(FooAIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
