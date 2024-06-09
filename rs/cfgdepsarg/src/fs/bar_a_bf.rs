use common::config::{AppCfg, AppCfgInfo};
use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{
    box_pin_async_fn, cfg_deps_a_boot, cfg_deps_a_boot_lr, ref_pin_async_fn, CfgArcSwapArc,
    CfgDeps, PinFn,
};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

pub type BarABfT = PinFn<u64, String>;

pub type BarABfCfg = CfgArcSwapArc<BarABfCfgInfo>;

pub type BarABfS = CfgDeps<BarABfCfg, ()>;

#[instrument(level = "trace", skip(s))]
pub async fn bar_a_bf_c(s: impl Deref<Target = BarABfS>, sleep_millis: u64) -> String {
    let cfg = s.cfg.get_cfg();
    sleep(Duration::from_millis(sleep_millis)).await;
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

/// Coded without use of [cfg_deps_boot_a].
pub fn bar_a_bf_boot_by_hand(app_cfg: AppCfg<AppCfgInfo>) -> Box<BarABfT> {
    let cfg = BarABfCfg::new_boxed_with_cfg_adapter(
        app_cfg.app_src,
        bar_a_bf_cfg_adapter,
        app_cfg.refresh_mode,
    );
    let bar_a_bf_s = Arc::new(BarABfS { cfg, deps: () });
    let f = move |sleep_millis| bar_a_bf_c(bar_a_bf_s.clone(), sleep_millis);
    box_pin_async_fn(f)
}

pub fn bar_a_bf_boot(app_cfg: AppCfg<AppCfgInfo>) -> Box<BarABfT> {
    let cfg_factory = BarABfCfg::new_boxed_with_cfg_adapter;

    cfg_deps_a_boot(bar_a_bf_c, cfg_factory, bar_a_bf_cfg_adapter, app_cfg, ())
}

/// Coded without use of [cfg_deps_boot_a_lr].
pub fn bar_a_bf_boot_lr_by_hand(app_cfg: AppCfg<AppCfgInfo>) -> &'static BarABfT {
    let cfg = BarABfCfg::new_boxed_with_cfg_adapter(
        app_cfg.app_src,
        bar_a_bf_cfg_adapter,
        app_cfg.refresh_mode,
    );
    let bar_a_bf_s: &BarABfS = Box::leak(Box::new(BarABfS { cfg, deps: () }));
    let f = move |sleep_millis| bar_a_bf_c(bar_a_bf_s, sleep_millis);
    ref_pin_async_fn(f)
}

/// Returns a leaked static reference to a bar_a_bf closure.
/// The benefit of this version over _boot is that it saves an Arc clone for each call to the returned function.
pub fn bar_a_bf_boot_lr(app_cfg: AppCfg<AppCfgInfo>) -> &'static BarABfT {
    let cfg_factory = BarABfCfg::new_boxed_with_cfg_adapter;

    cfg_deps_a_boot_lr(bar_a_bf_c, cfg_factory, bar_a_bf_cfg_adapter, app_cfg, ())
}
