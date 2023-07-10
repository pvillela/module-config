use crate::fs::{bar_bf_c, BarBfCfg, BarBfS, BarBfT};
use common::config::AppCfgInfo;
use common::fs_data::BarBfCfgInfo;
use common::fwk::{cfg_deps_boot, cfg_deps_boot_lr, RefreshMode};
use std::sync::Arc;

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

/// Coded without use of [cfg_deps_boot].
/// Returns a boxed bar_bf closure.
pub fn bar_bf_boot_by_hand(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<BarBfT> {
    let cfg =
        BarBfCfg::new_boxed_with_cfg_adapter(app_cfg, bar_bf_cfg_adapter, refresh_mode.clone());
    let bar_bf_s = BarBfS { cfg, deps: () };
    let f = move |_| bar_bf_c(&bar_bf_s, ());
    Box::new(f)
}

/// Returns a boxed bar_bf closure.
pub fn bar_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> Box<BarBfT> {
    let cfg_factory = BarBfCfg::new_boxed_with_cfg_adapter;

    cfg_deps_boot(
        bar_bf_c,
        cfg_factory,
        bar_bf_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        (),
    )
}

/// Coded without use of [cfg_deps_boot_lr].
/// Returns a leaked static reference to a bar_bf closure.
/// Since bar_bf has no dependencies, there is no benefit over _boot.
pub fn bar_bf_boot_lr_by_hand(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> &'static BarBfT {
    Box::leak(Box::new(bar_bf_boot(app_cfg, refresh_mode)))
}

/// Returns a leaked static reference to a bar_bf closure.
/// Since bar_bf has no dependencies, there is no benefit over _boot.
pub fn bar_bf_boot_lr(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> &'static BarBfT {
    let cfg_factory = BarBfCfg::new_boxed_with_cfg_adapter;

    cfg_deps_boot_lr(
        bar_bf_c,
        cfg_factory,
        bar_bf_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        (),
    )
}
