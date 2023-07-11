use super::{bar_a_bf_boot, bar_a_bf_boot_lr};
use crate::fs::{foo_a_sfl_c, FooASflCfg, FooASflDeps, FooASflS, FooASflT};
use common::config::AppCfgInfo;
use common::fs_data::FooASflCfgInfo;
use common::fwk::{
    box_pin_async_fn, cfg_deps_boot_a, cfg_deps_boot_a_lr, ref_pin_async_fn, RefreshMode,
};
use std::sync::Arc;

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

/// Coded without use of [cfg_deps_boot_a].
/// Returns a boxed foo_a_sfl closure.
pub fn foo_a_sfl_boot_by_hand(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<FooASflT> {
    let cfg = FooASflCfg::new_boxed_with_cfg_adapter(
        app_cfg,
        foo_a_sfl_cfg_adapter,
        refresh_mode.clone(),
    );
    let deps = FooASflDeps {
        bar_a_bf: bar_a_bf_boot(app_cfg, refresh_mode.clone()),
    };
    let foo_a_sfl_s = Arc::new(FooASflS { cfg, deps });
    let f = move |input| foo_a_sfl_c(foo_a_sfl_s.clone(), input);
    box_pin_async_fn(f)
}

/// Returns a boxed foo_a_sfl closure.
pub fn foo_a_sfl_boot(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<FooASflT> {
    let cfg_factory = FooASflCfg::new_boxed_with_cfg_adapter;
    let deps = FooASflDeps {
        bar_a_bf: Box::new(bar_a_bf_boot(app_cfg, refresh_mode.clone())),
    };
    cfg_deps_boot_a(
        foo_a_sfl_c,
        cfg_factory,
        foo_a_sfl_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        deps,
    )
}

/// Coded without use of [cfg_deps_boot_a_lr].
/// Returns a leaked static reference to a foo_a_sfl closure.
/// The benefit of this version over _boot is that it saves an Arc clone for this and its dependencies
/// for each call to the returned function.
pub fn foo_a_sfl_boot_lr_by_hand(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> &'static FooASflT {
    let cfg = FooASflCfg::new_boxed_with_cfg_adapter(
        app_cfg,
        foo_a_sfl_cfg_adapter,
        refresh_mode.clone(),
    );
    let deps = FooASflDeps {
        bar_a_bf: Box::new(bar_a_bf_boot_lr(app_cfg, refresh_mode.clone())),
    };
    let foo_a_sfl_s: &FooASflS = Box::leak(Box::new(FooASflS { cfg, deps }));
    let f = move |input| foo_a_sfl_c(foo_a_sfl_s, input);
    ref_pin_async_fn(f)
}

/// Returns a leaked static reference to a foo_a_sfl closure.
/// The benefit of this version over _boot is that it saves an Arc clone for this and its dependencies
/// for each call to the returned function.
pub fn foo_a_sfl_boot_lr(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> &'static FooASflT {
    let cfg_factory = FooASflCfg::new_boxed_with_cfg_adapter;
    let deps = FooASflDeps {
        bar_a_bf: Box::new(bar_a_bf_boot_lr(app_cfg, refresh_mode.clone())),
    };
    cfg_deps_boot_a_lr(
        foo_a_sfl_c,
        cfg_factory,
        foo_a_sfl_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        deps,
    )
}
