use crate::fs::{bar_i_bf_c, BarIBfS, BarIBfT};
use common::config::AppCfgInfo;
use common::fs_data::BarIBfCfgInfo;
use common::fwk::{cfg_deps_boot_i, cfg_deps_boot_i_lr};

fn bar_i_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarIBfCfgInfo {
    BarIBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

/// Coded without use of [cfg_deps_boot_i].
/// Returns a boxed bar_i_bf_closure.
pub fn bar_i_bf_boot_by_hand(app_cfg: &AppCfgInfo) -> Box<BarIBfT> {
    let cfg = bar_i_bf_cfg_adapter(app_cfg);
    let bar_i_bf_s = { BarIBfS { cfg, deps: () } };
    let f = move |_| bar_i_bf_c(&bar_i_bf_s, ());
    Box::new(f)
}

/// Returns a boxed bar_i_bf_closure.
pub fn bar_i_bf_boot(app_cfg: &AppCfgInfo) -> Box<BarIBfT> {
    cfg_deps_boot_i(bar_i_bf_c, bar_i_bf_cfg_adapter, app_cfg, ())
}

/// Coded without use of [cfg_deps_boot_i_lr].
/// Returns a leaked static reference to a bar_i_bf closure.
/// Since bar_i_bf has no dependencies, there is no benefit over _boot.
pub fn bar_i_bf_boot_lr_manual(app_cfg: &AppCfgInfo) -> &'static BarIBfT {
    Box::leak(Box::new(bar_i_bf_boot(app_cfg)))
}

/// Returns a leaked static reference to a bar_i_bf closure.
/// Since bar_i_bf has no dependencies, there is no benefit over _boot.
pub fn bar_i_bf_boot_lr(app_cfg: &AppCfgInfo) -> &'static BarIBfT {
    cfg_deps_boot_i_lr(bar_i_bf_c, bar_i_bf_cfg_adapter, app_cfg, ())
}
