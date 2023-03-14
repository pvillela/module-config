use common::fs_data::BarBfCfgInfo;
use common::fs_data::{FooAIn, FooSflCfgInfo};
use common::fwk::RefreshMode;
use pushtofunction::fs::{bar_a_bf_c, foo_a_sfl_c, BarABfCfgDeps, FooASflCfgDeps, FooASflDeps};
use tokio;

pub async fn common_test(
    foo_sfl_cfg_info: FooSflCfgInfo,
    bar_bf_cfg_info: BarBfCfgInfo,
) -> Option<String> {
    let bar_cfg_deps = BarABfCfgDeps::new(
        move || bar_bf_cfg_info.clone().into(),
        RefreshMode::NoRefresh,
        (),
    );

    let bar_a_bf = bar_a_bf_c(bar_cfg_deps);

    let foo_cfg_deps = FooASflCfgDeps::new(
        move || foo_sfl_cfg_info.clone().into(),
        RefreshMode::NoRefresh,
        FooASflDeps { bar_a_bf },
    );

    let foo_a_sfl = foo_a_sfl_c(foo_cfg_deps);

    let handle = tokio::spawn(async move { foo_a_sfl(FooAIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
