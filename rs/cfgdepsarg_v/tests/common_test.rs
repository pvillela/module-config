use common::fs_data::BarBfCfgInfo;
use common::fs_data::{FooAIn, FooSflCfgInfo};
use common::fwk::{RefreshMode, Src};
use cfgdepsarg::fs::{bar_a_bf_c, foo_a_sfl_c, BarABfCfg, FooASflCfg, FooASflDeps};
use tokio;

pub async fn common_test(
    foo_sfl_cfg_info: FooSflCfgInfo,
    bar_bf_cfg_info: BarBfCfgInfo,
) -> Option<String> {
    let bar_cfg = BarABfCfg::new(
        Src::new_boxed(move || bar_bf_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );

    let bar_a_bf = bar_a_bf_c(bar_cfg);

    let foo_cfg = FooASflCfg::new(
        Src::new_boxed(move || foo_sfl_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );

    let foo_deps = FooASflDeps { bar_a_bf };

    let foo_a_sfl = foo_a_sfl_c(foo_cfg, foo_deps);

    let handle = tokio::spawn(async move { foo_a_sfl(FooAIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
