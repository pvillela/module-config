use crate::fs::{bar_a_bf_c, BarABfCfg, BarABfS, BarABfT};
use common::config::AppCfgInfo;
use common::fs_data::BarABfCfgInfo;
use common::fwk::{
    box_pin_async_fn, cfg_deps_boot_a, cfg_deps_boot_a_lr, ref_pin_async_fn, RefreshMode,
};
use std::sync::Arc;

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

/// Coded without use of [cfg_deps_boot_a].
pub fn bar_a_bf_boot_by_hand(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<BarABfT> {
    let cfg =
        BarABfCfg::new_boxed_with_cfg_adapter(app_cfg, bar_a_bf_cfg_adapter, refresh_mode.clone());
    let bar_a_bf_s = Arc::new(BarABfS { cfg, deps: () });
    let f = move |sleep_millis| bar_a_bf_c(bar_a_bf_s.clone(), sleep_millis);
    box_pin_async_fn(f)
}

pub fn bar_a_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> Box<BarABfT> {
    let cfg_factory = BarABfCfg::new_boxed_with_cfg_adapter;
    let deps = ();

    cfg_deps_boot_a(
        bar_a_bf_c,
        cfg_factory,
        bar_a_bf_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        deps,
    )
}

/// Coded without use of [cfg_deps_boot_a_lr].
pub fn bar_a_bf_boot_lr_by_hand(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> &'static BarABfT {
    let cfg =
        BarABfCfg::new_boxed_with_cfg_adapter(app_cfg, bar_a_bf_cfg_adapter, refresh_mode.clone());
    let bar_a_bf_s: &BarABfS = Box::leak(Box::new(BarABfS { cfg, deps: () }));
    let f = move |sleep_millis| bar_a_bf_c(bar_a_bf_s, sleep_millis);
    ref_pin_async_fn(f)
}

/// Returns a leaked static reference to a bar_a_bf closure.
/// The benefit of this version over _boot is that it saves an Arc clone for each call to the returned function.
pub fn bar_a_bf_boot_lr(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> &'static BarABfT {
    let cfg_factory = BarABfCfg::new_boxed_with_cfg_adapter;
    let deps = ();

    cfg_deps_boot_a_lr(
        bar_a_bf_c,
        cfg_factory,
        bar_a_bf_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        deps,
    )
}
