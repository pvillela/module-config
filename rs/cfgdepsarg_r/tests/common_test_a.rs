use std::sync::Arc;

use cfgdepsarg_r::fs::{
    bar_a_bf_c, foo_a_sfl_c, BarABfCfg, BarABfS, FooASflCfg, FooASflDeps, FooASflS,
};
use common::fs_data::BarABfCfgInfo;
use common::fs_data::{FooAIn, FooASflCfgInfo};
use common::fwk::{RefreshMode, Src};
use common::ref_pin_async_fn;
use tokio;

pub async fn common_test(
    foo_a_sfl_cfg_info: FooASflCfgInfo,
    bar_a_bf_cfg_info: BarABfCfgInfo,
) -> Option<String> {
    let bar_a_bf = move |sleep_millis| {
        let bar_a_bf_cfg_info = bar_a_bf_cfg_info.clone();
        let bar_a_bf_cfg = BarABfCfg::new(
            Src::new_boxed(move || bar_a_bf_cfg_info.clone()),
            RefreshMode::NoRefresh,
        );
        let bar_a_bf_s = Arc::new(BarABfS {
            cfg: bar_a_bf_cfg,
            deps: (),
        });
        bar_a_bf_c(bar_a_bf_s, sleep_millis)
    };

    let foo_a_sfl_cfg = FooASflCfg::new(
        Src::new_boxed(move || foo_a_sfl_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );
    let foo_a_sfl_deps = FooASflDeps {
        bar_a_bf: ref_pin_async_fn!(bar_a_bf),
    };
    let foo_a_sfl_s = Arc::new(FooASflS {
        cfg: foo_a_sfl_cfg,
        deps: foo_a_sfl_deps,
    });

    let foo_a_sfl = |input| foo_a_sfl_c(foo_a_sfl_s, input);

    let handle = tokio::spawn(async move { foo_a_sfl(FooAIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| x.res);
    println!("{:?}", res);
    res
}
