use super::bar_at_bf_boot;
use crate::fs::{foo_at_sfl_c, FooAtSflCfg, FooAtSflDeps, FooAtSflS};
use common::config::AppCfgInfo;
use common::fs_data::{FooAIn, FooAtIn, FooAtOut, FooAtSflCfgInfo};
use common::fwk::{
    box_pin_async_fn, cfg_deps_boot_at_free_tx, cfg_deps_boot_at_free_tx_box, ref_pin_async_fn,
    AppErr, RefreshMode, Tx,
};
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
    ) -> Pin<Box<dyn Future<Output = Result<FooAtOut, AppErr>> + Send + Sync>>,
> {
    let cfg_factory = FooAtSflCfg::new_boxed_with_cfg_adapter;
    let deps = FooAtSflDeps {
        bar_at_bf: bar_at_bf_boot(app_cfg, refresh_mode.clone()),
    };

    let foo_at_sfl_c = |x, y, z| {
        let d: Pin<Box<dyn Future<Output = Result<FooAtOut, AppErr>> + Send + Sync>> =
            Box::pin(foo_at_sfl_c(x, y, z));
        d
    };

    cfg_deps_boot_at_free_tx_box(
        foo_at_sfl_c,
        cfg_factory,
        foo_at_sfl_cfg_atdapter,
        app_cfg,
        refresh_mode.clone(),
        deps,
    );
    // x
    todo!()
}
