use common::config::{AppCfg, AppCfgInfo};
use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{cfg_deps_boot, cfg_deps_boot_lr, CfgArcSwapArc, CfgDeps};

pub type BarBfT = dyn Fn(()) -> String + Send + Sync;

pub type BarBfCfg = CfgArcSwapArc<BarBfCfgInfo>;

pub type BarBfS = CfgDeps<BarBfCfg, ()>;

pub fn bar_bf_c(s: &BarBfS, _: ()) -> String {
    let cfg = s.cfg.get_cfg();
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

/// Coded without use of [cfg_deps_boot].
/// Returns a boxed bar_bf closure.
pub fn bar_bf_boot_by_hand(app_cfg: AppCfg<AppCfgInfo>) -> Box<BarBfT> {
    let cfg = BarBfCfg::new_boxed_with_cfg_adapter(
        app_cfg.app_src,
        bar_bf_cfg_adapter,
        app_cfg.refresh_mode,
    );
    let bar_bf_s = BarBfS { cfg, deps: () };
    let f = move |_| bar_bf_c(&bar_bf_s, ());
    Box::new(f)
}

/// Returns a boxed bar_bf closure.
pub fn bar_bf_boot(app_cfg: AppCfg<AppCfgInfo>) -> Box<BarBfT> {
    let cfg_factory = BarBfCfg::new_boxed_with_cfg_adapter;
    cfg_deps_boot(bar_bf_c, cfg_factory, bar_bf_cfg_adapter, app_cfg, ())
}

/// Coded without use of [cfg_deps_boot_lr].
/// Returns a leaked static reference to a bar_bf closure.
/// Since bar_bf has no dependencies, there is no benefit over _boot.
pub fn bar_bf_boot_lr_by_hand(app_cfg: AppCfg<AppCfgInfo>) -> &'static BarBfT {
    Box::leak(Box::new(bar_bf_boot(app_cfg)))
}

/// Returns a leaked static reference to a bar_bf closure.
/// Since bar_bf has no dependencies, there is no benefit over _boot.
pub fn bar_bf_boot_lr(app_cfg: AppCfg<AppCfgInfo>) -> &'static BarBfT {
    let cfg_factory = BarBfCfg::new_boxed_with_cfg_adapter;
    cfg_deps_boot_lr(bar_bf_c, cfg_factory, bar_bf_cfg_adapter, app_cfg, ())
}
