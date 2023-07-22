use cfgdepsarg::fs::{
    bar_at_bf_c, foo_at_sfl_c, BarAtBfCfg, BarAtBfS, FooAtSflCfg, FooAtSflDeps, FooAtSflS,
};
use common::config::get_pool;
use common::fs_data::BarAtBfCfgInfo;
use common::fs_data::{FooAtIn, FooAtSflCfgInfo};
use common::fwk::{
    cfg_deps_at_partial_apply_free_tx_arc, cfg_deps_at_partial_apply_free_tx_box,
    fn2_arc_with_transaction, RefreshMode, Src,
};
use std::sync::Arc;
use tokio;

pub async fn common_test(
    foo_at_sfl_cfg_info: FooAtSflCfgInfo,
    bar_at_bf_cfg_info: BarAtBfCfgInfo,
) -> Option<String> {
    let bar_at_bf_s = {
        let bar_at_bf_cfg_info = bar_at_bf_cfg_info.clone();
        let bar_at_bf_cfg = BarAtBfCfg::new(
            Src::new_boxed(move || bar_at_bf_cfg_info.clone()),
            RefreshMode::NoRefresh,
        );
        Arc::new(BarAtBfS {
            cfg: bar_at_bf_cfg,
            deps: (),
        })
    };

    let bar_at_bf_tx = cfg_deps_at_partial_apply_free_tx_box(bar_at_bf_c, bar_at_bf_s);

    let foo_at_sfl_s = {
        let foo_at_sfl_cfg = FooAtSflCfg::new(
            Src::new_boxed(move || foo_at_sfl_cfg_info.clone()),
            RefreshMode::NoRefresh,
        );
        let foo_at_sfl_deps = FooAtSflDeps {
            bar_at_bf: bar_at_bf_tx,
        };
        Arc::new(FooAtSflS {
            cfg: foo_at_sfl_cfg,
            deps: foo_at_sfl_deps,
        })
    };

    let foo_at_sfl_tx_arc = cfg_deps_at_partial_apply_free_tx_arc(foo_at_sfl_c, foo_at_sfl_s);
    let foo_at_sfl = fn2_arc_with_transaction(get_pool(), foo_at_sfl_tx_arc);

    let handle = tokio::spawn(async move { foo_at_sfl(FooAtIn { sleep_millis: 0 }).await });
    let res = handle.await.ok().map(|x| format!("{:?}", x));
    println!("{:?}", res);
    res
}
