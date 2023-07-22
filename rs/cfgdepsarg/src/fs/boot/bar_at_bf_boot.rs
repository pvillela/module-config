use crate::fs::{bar_at_bf_c, BarAtBfCfg, BarAtBfTxT};
use common::config::AppCfgInfo;
use common::fs_data::BarAtBfCfgInfo;
use common::fwk::{cfg_deps_at_boot_free_tx_box, cfg_deps_at_boot_free_tx_lr, RefreshMode};
use std::sync::Arc;

fn bar_at_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAtBfCfgInfo {
    BarAtBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

/// Returns a boxed bar_at_bf closure with free Tx parameter.
pub fn bar_at_bf_boot(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<BarAtBfTxT> {
    let cfg_factory = BarAtBfCfg::new_boxed_with_cfg_adapter;

    cfg_deps_at_boot_free_tx_box(
        bar_at_bf_c,
        cfg_factory,
        bar_at_bf_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        (),
    )
}

/// Returns a leaked static reference to a bar_at_bf closure with free Tx parameter.
pub fn bar_at_bf_boot_lr(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> &'static BarAtBfTxT {
    let cfg_factory = BarAtBfCfg::new_boxed_with_cfg_adapter;

    cfg_deps_at_boot_free_tx_lr(
        bar_at_bf_c,
        cfg_factory,
        bar_at_bf_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        (),
    )
}
