use super::{bar_bf_boot, bar_bf_boot_lr};
use crate::fs::{foo_sfl_c, FooSflCfg, FooSflDeps, FooSflS, FooSflT};
use common::config::AppCfgInfo;
use common::fs_data::FooSflCfgInfo;
use common::fwk::{cfg_deps_boot, cfg_deps_boot_lr, RefreshMode};
use std::sync::Arc;

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

/// Coded without use of [cfg_deps_boot].
/// Returns a boxed foo_sfl closure.
pub fn foo_sfl_boot_by_hand(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<FooSflT> {
    let cfg =
        FooSflCfg::new_boxed_with_cfg_adapter(app_cfg, foo_sfl_cfg_adapter, refresh_mode.clone());
    let deps = FooSflDeps {
        bar_bf: bar_bf_boot(app_cfg, refresh_mode.clone()),
    };
    let foo_sfl_s = FooSflS { cfg, deps };
    let f = move |_| foo_sfl_c(&foo_sfl_s, ());
    Box::new(f)
}

/// Returns a boxed foo_sfl closure.
pub fn foo_sfl_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> Box<FooSflT> {
    let cfg_factory = FooSflCfg::new_boxed_with_cfg_adapter;
    let deps = FooSflDeps {
        bar_bf: bar_bf_boot(app_cfg, refresh_mode.clone()),
    };
    cfg_deps_boot(
        foo_sfl_c,
        cfg_factory,
        foo_sfl_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        deps,
    )
}

/// Coded without use of [cfg_deps_boot_lr].
/// Returns a leaked static reference to a foo_sfl closure.
/// Since bar_bf has no dependencies, there is no benefit over _boot.
pub fn foo_sfl_boot_lr_by_hand(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> &'static FooSflT {
    let cfg =
        FooSflCfg::new_boxed_with_cfg_adapter(app_cfg, foo_sfl_cfg_adapter, refresh_mode.clone());
    let deps = FooSflDeps {
        bar_bf: Box::new(bar_bf_boot_lr(app_cfg, refresh_mode.clone())),
    };
    let foo_sfl_s: &FooSflS = Box::leak(Box::new(FooSflS { cfg, deps }));
    let f = move |_| foo_sfl_c(foo_sfl_s, ());
    Box::leak(Box::new(f))
}
/// Returns a leaked static reference to a foo_sfl closure.
/// Since bar_bf has no dependencies, there is no benefit over _boot.
pub fn foo_sfl_boot_lr(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> &'static FooSflT {
    let cfg_factory = FooSflCfg::new_boxed_with_cfg_adapter;
    let deps = FooSflDeps {
        bar_bf: Box::new(bar_bf_boot_lr(app_cfg, refresh_mode.clone())),
    };
    cfg_deps_boot_lr(
        foo_sfl_c,
        cfg_factory,
        foo_sfl_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        deps,
    )
}
