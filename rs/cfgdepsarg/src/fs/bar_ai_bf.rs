use common::config::AppCfgInfo;
use common::fs_data::BarAiBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{
    box_pin_async_fn, cfg_deps_ai_boot, cfg_deps_ai_boot_lr, ref_pin_async_fn, CfgDeps, PinFn,
};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub type BarAiBfT = PinFn<u64, String>;

pub type BarAiBfS = CfgDeps<BarAiBfCfgInfo, ()>;

pub async fn bar_ai_bf_c(s: impl Deref<Target = BarAiBfS>, sleep_millis: u64) -> String {
    let cfg = &s.cfg;
    sleep(Duration::from_millis(sleep_millis)).await;
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

fn bar_ai_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAiBfCfgInfo {
    BarAiBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

/// Coded without use of [cfg_deps_boot_ai].
/// Returns a boxed bar_ai_bf_closure.
pub fn bar_ai_bf_boot_by_hand(app_cfg: &AppCfgInfo) -> Box<BarAiBfT> {
    let cfg = bar_ai_bf_cfg_adapter(&app_cfg);
    let bar_ai_bf_s = Arc::new(BarAiBfS { cfg, deps: () });
    let f = move |sleep_millis| bar_ai_bf_c(bar_ai_bf_s.clone(), sleep_millis);
    box_pin_async_fn(f)
}

/// Returns a boxed bar_ai_bf_closure.
pub fn bar_ai_bf_boot(app_cfg: &AppCfgInfo) -> Box<BarAiBfT> {
    cfg_deps_ai_boot(bar_ai_bf_c, bar_ai_bf_cfg_adapter, app_cfg, ())
}

/// Coded without use of [cfg_deps_boot_ai_lr].
/// Returns a leaked static reference to a bar_ai_bf closure.
/// Since bar_ai_bf has no dependencies, there is no benefit over _boot.
pub fn bar_ai_bf_boot_lr_by_hand(app_cfg: &AppCfgInfo) -> &'static BarAiBfT {
    let cfg = bar_ai_bf_cfg_adapter(&app_cfg);
    let bar_ai_bf_s: &BarAiBfS = Box::leak(Box::new(BarAiBfS { cfg, deps: () }));
    let f = move |sleep_millis| bar_ai_bf_c(bar_ai_bf_s, sleep_millis);
    ref_pin_async_fn(f)
}

/// Returns a leaked static reference to a bar_ai_bf closure.
/// Since bar_ai_bf has no dependencies, there is no benefit over _boot.
pub fn bar_ai_bf_boot_lr(app_cfg: &AppCfgInfo) -> &'static BarAiBfT {
    cfg_deps_ai_boot_lr(bar_ai_bf_c, bar_ai_bf_cfg_adapter, app_cfg, ())
}
