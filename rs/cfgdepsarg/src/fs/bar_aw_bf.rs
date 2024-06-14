use common::config::AppCfgInfo;
use common::fs_data::BarAwBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{
    box_pin_async_fn_wss, cfg_deps_aw_boot, CfgDeps, CfgRefCellId, PinFnWss, RefreshMode,
};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub type BarAwBfT = PinFnWss<u64, String>;

pub type BarAwBfCfg = CfgRefCellId<BarAwBfCfgInfo>;

pub type BarAwBfS = CfgDeps<BarAwBfCfg, ()>;

pub async fn bar_aw_bf_c(s: impl Deref<Target = BarAwBfS>, sleep_millis: u64) -> String {
    let cfg = s.cfg.get_cfg();
    sleep(Duration::from_millis(sleep_millis)).await;
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

fn bar_aw_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAwBfCfgInfo {
    BarAwBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

/// Coded without use of [cfg_deps_boot_aw].
/// Returns a bar_aw_bf stereotype instance.
pub fn bar_aw_bf_boot_by_hand(
    app_cfg: fn() -> AppCfgInfo,
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
pub fn bar_aw_bf_boot(app_cfg: fn() -> AppCfgInfo, refresh_mode: RefreshMode) -> Box<BarAwBfT> {
    cfg_deps_aw_boot(
        bar_aw_bf_c,
        BarAwBfCfg::new_boxed_with_cfg_adapter,
        bar_aw_bf_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        (),
    )
}
