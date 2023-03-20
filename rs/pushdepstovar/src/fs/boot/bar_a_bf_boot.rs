use crate::fs::BAR_A_BF_CFG_DEF;
use common::config::AppCfgInfo;
use common::fs_data::BarABfCfgInfo;
use common::fwk::{CfgDef, RefreshMode};
use std::sync::Arc;
use std::time::Duration;

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

fn bar_a_bf_adapt_cfg_src(origin: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) {
    CfgDef::set_once_cell_with_cfg_adapter(
        &BAR_A_BF_CFG_DEF,
        origin,
        bar_a_bf_cfg_adapter,
        refresh_mode,
    );
}

pub fn bar_a_bf_init_refreshable(app_cfg_src: fn() -> Arc<AppCfgInfo>, cache_ttl: Duration) {
    bar_a_bf_adapt_cfg_src(app_cfg_src, RefreshMode::Refreshable(cache_ttl));
}

pub fn bar_a_bf_init_no_refresh(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    bar_a_bf_adapt_cfg_src(app_cfg_src, RefreshMode::NoRefresh);
}
