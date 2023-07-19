use crate::fs::{bar_at_bf_c, BarAtBfCfg, BarAtBfTxT};
use common::config::AppCfgInfo;
use common::fs_data::BarAtBfCfgInfo;
use common::fwk::{cfg_deps_boot_at_free_tx_no_box, RefreshMode};
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

    let x = cfg_deps_boot_at_free_tx_no_box(
        bar_at_bf_c,
        cfg_factory,
        bar_at_bf_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        (),
    );
    Box::new(x)
}
