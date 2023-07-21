use super::{bar_at_bf_boot, bar_at_bf_boot_lr};
use crate::fs::{foo_at_sfl_c, FooAtSflCfg, FooAtSflDeps, FooAtSflTxT};
use common::config::AppCfgInfo;
use common::fs_data::FooAtSflCfgInfo;
use common::fwk::{cfg_deps_boot_at_free_tx, cfg_deps_boot_at_free_tx_lr, RefreshMode};
use std::sync::Arc;

fn foo_at_sfl_cfg_atdapter(app_cfg: &AppCfgInfo) -> FooAtSflCfgInfo {
    FooAtSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

/// Returns a boxed foo_at_sfl closure with free Tx parameter.
pub fn foo_at_sfl_boot(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<FooAtSflTxT> {
    let cfg_factory = FooAtSflCfg::new_boxed_with_cfg_adapter;
    let b = bar_at_bf_boot(app_cfg, refresh_mode.clone());
    let deps = FooAtSflDeps { bar_at_bf: b };

    cfg_deps_boot_at_free_tx(
        foo_at_sfl_c,
        cfg_factory,
        foo_at_sfl_cfg_atdapter,
        app_cfg,
        refresh_mode.clone(),
        deps,
    )
}

/// Returns a leaked static reference to a foo_at_sfl closure with free Tx parameter.
pub fn foo_at_sfl_boot_lr(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> &'static FooAtSflTxT {
    let cfg_factory = FooAtSflCfg::new_boxed_with_cfg_adapter;
    let b = Box::new(bar_at_bf_boot_lr(app_cfg, refresh_mode.clone()));
    let deps = FooAtSflDeps { bar_at_bf: b };

    cfg_deps_boot_at_free_tx_lr(
        foo_at_sfl_c,
        cfg_factory,
        foo_at_sfl_cfg_atdapter,
        app_cfg,
        refresh_mode.clone(),
        deps,
    )
}
