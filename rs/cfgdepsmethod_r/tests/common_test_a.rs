use std::sync::OnceLock;

use cfgdepsmethods::fs::{BarABfCfg, BarABfS, FooASflCfg, FooASflDeps, FooASflS};
use common::fs_data::{BarABfCfgInfo, FooAIn, FooASflCfgInfo};
use common::fwk::{RefreshMode, Src};
use tokio;

pub async fn common_test(
    foo_a_sfl_cfg_info: FooASflCfgInfo,
    bar_a_bf_cfg_info: BarABfCfgInfo,
) -> Option<String> {
    let bar_cfg = BarABfCfg::new(
        Src::new_boxed(move || bar_a_bf_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );

    let bar_a_bf_s = {
        static BAR_A_BF_S: OnceLock<BarABfS> = OnceLock::new();
        BAR_A_BF_S.get_or_init(|| BarABfS { cfg: bar_cfg })
    };

    let foo_cfg = FooASflCfg::new(
        Src::new_boxed(move || foo_a_sfl_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );

    let foo_deps = FooASflDeps { bar_a_bf_s };

    let foo_a_sfl_s = FooASflS {
        cfg: foo_cfg,
        deps: foo_deps,
    };

    let handle = tokio::spawn(async move { foo_a_sfl_s.run(FooAIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
