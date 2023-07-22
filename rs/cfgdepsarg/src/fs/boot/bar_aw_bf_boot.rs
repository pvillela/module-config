use crate::fs::{bar_aw_bf_c, BarAwBfCfg, BarAwBfS, BarAwBfT};
use common::config::AppCfgInfo;
use common::fs_data::BarAwBfCfgInfo;
use common::fwk::{box_pin_async_fn_wss, cfg_deps_aw_boot, RefreshMode};
use std::sync::Arc;

fn bar_aw_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAwBfCfgInfo {
    BarAwBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

/// Coded without use of [cfg_deps_boot_aw].
/// Returns a bar_aw_bf stereotype instance.
pub fn bar_aw_bf_boot_by_hand(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<BarAwBfT> {
    let cfg = BarAwBfCfg::new_boxed_with_cfg_adapter(
        app_cfg,
        bar_aw_bf_cfg_adapter,
        refresh_mode.clone(),
    );
    let bar_aw_bf_s = Arc::new(BarAwBfS { cfg, deps: () });
    let f = move |sleep_millis| bar_aw_bf_c(bar_aw_bf_s.clone(), sleep_millis);
    box_pin_async_fn_wss(f)
}

/// Returns a bar_aw_bf stereotype instance.
pub fn bar_aw_bf_boot(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<BarAwBfT> {
    let cfg_factory = BarAwBfCfg::new_boxed_with_cfg_adapter;
    cfg_deps_aw_boot(
        bar_aw_bf_c,
        cfg_factory,
        bar_aw_bf_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        (),
    )
}
