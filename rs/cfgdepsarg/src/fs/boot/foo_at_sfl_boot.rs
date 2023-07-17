use super::bar_at_bf_boot;
use crate::fs::{foo_at_sfl_c, FooAtSflCfg, FooAtSflDeps};
use common::config::AppCfgInfo;
use common::fs_data::{FooAtIn, FooAtOut, FooAtSflCfgInfo};
use common::fwk::{cfg_deps_boot_at_free_tx_no_box, AppErr, RefreshMode, Tx};
use futures::Future;
use std::pin::Pin;
use std::sync::Arc;

fn foo_at_sfl_cfg_atdapter(app_cfg: &AppCfgInfo) -> FooAtSflCfgInfo {
    FooAtSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

/// Returns a boxed foo_at_sfl closure.
pub fn foo_at_sfl_boot(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<
    dyn for<'a> Fn(
            FooAtIn,
            &'a Tx,
        )
            -> Pin<Box<dyn Future<Output = Result<FooAtOut, AppErr>> + Send + Sync + 'a>>
        + Send
        + Sync,
> {
    let cfg_factory = FooAtSflCfg::new_boxed_with_cfg_adapter;
    let b = bar_at_bf_boot(app_cfg, refresh_mode.clone());
    let deps = FooAtSflDeps { bar_at_bf: b };

    // let foo_at_sfl_c = move |x, y, z| {
    //     let d: Pin<Box<dyn Future<Output = Result<FooAtOut, AppErr>> + Send + Sync>> =
    //         Box::pin(foo_at_sfl_c(x, y, z));
    //     d
    // };

    let x = cfg_deps_boot_at_free_tx_no_box(
        foo_at_sfl_c,
        cfg_factory,
        foo_at_sfl_cfg_atdapter,
        app_cfg,
        refresh_mode.clone(),
        deps,
    );
    Box::new(x)
}
