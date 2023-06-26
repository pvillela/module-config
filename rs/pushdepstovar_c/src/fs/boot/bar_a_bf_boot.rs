use crate::fs::{BarABfCfgInfo, BAR_A_BF_CFG_DEPS};
use crate::fwk::{CfgDeps, RefreshMode};
use common::config::AppCfgInfo;
use std::sync::Arc;
use std::time::Duration;

fn bar_abf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

fn bar_abf_adapt_cfg_src(
    origin: impl Fn() -> Arc<AppCfgInfo> + 'static + Send + Sync,
    refresh_mode: RefreshMode,
    deps: (),
) {
    CfgDeps::set_with_cfg_adapter(
        &BAR_A_BF_CFG_DEPS,
        origin,
        bar_abf_cfg_adapter,
        refresh_mode,
        deps,
    );
}

pub fn bar_a_bf_init_refreshable(app_cfg_src: fn() -> Arc<AppCfgInfo>, cache_ttl: Duration) {
    bar_abf_adapt_cfg_src(app_cfg_src, RefreshMode::Refreshable(cache_ttl), ());
}

pub fn bar_a_bf_init_no_refresh(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    bar_abf_adapt_cfg_src(app_cfg_src, RefreshMode::NoRefresh, ());
}
