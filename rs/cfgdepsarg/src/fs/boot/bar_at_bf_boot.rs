use crate::fs::{bar_at_bf_c, BarAtBfCfg, BarAtBfT};
use common::config::AppCfgInfo;
use common::fs_data::{BarAtBfCfgInfo, FooAtIn, FooAtOut};
use common::fwk::{
    cfg_deps_boot_at_free_tx, cfg_deps_boot_at_free_tx_box, cfg_deps_boot_at_free_tx_box_no_ss,
    AppErr, RefreshMode, Tx,
};
use futures::Future;
use std::pin::Pin;
use std::sync::Arc;

fn bar_at_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAtBfCfgInfo {
    BarAtBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_at_bf_boot(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<
    dyn for<'a> Fn(
        u64,
        &'a Tx,
    ) -> Pin<Box<dyn Future<Output = Result<String, AppErr>> + Send + Sync>>,
> {
    let cfg_factory = BarAtBfCfg::new_boxed_with_cfg_adapter;

    let bar_at_bf_c = |x, y, z| {
        let d: Pin<Box<dyn Future<Output = Result<String, AppErr>> + Send + Sync>> =
            Box::pin(bar_at_bf_c(x, y, z));
        d
    };
    cfg_deps_boot_at_free_tx_box(
        bar_at_bf_c,
        cfg_factory,
        bar_at_bf_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        (),
    )
}
