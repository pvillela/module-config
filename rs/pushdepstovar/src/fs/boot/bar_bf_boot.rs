use crate::fs::BAR_BF_CFG_DEF;
use common::config::AppCfgInfo;
use common::fs_data::BarBfCfgInfo;
use common::fwk::{CfgDef, RefreshMode};
use std::sync::Arc;
use std::time::Duration;

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

fn bar_bf_adapt_cfg_src(
    origin: impl Fn() -> Arc<AppCfgInfo> + 'static + Send + Sync,
    refresh_mode: RefreshMode,
) {
    CfgDef::set_once_cell_with_cfg_adapter(
        &BAR_BF_CFG_DEF,
        origin,
        bar_bf_cfg_adapter,
        refresh_mode,
    );
}

pub fn bar_bf_init_refreshable(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    bar_bf_adapt_cfg_src(
        app_cfg_src,
        RefreshMode::Refreshable(Duration::from_millis(0)),
    );
}

pub fn bar_bf_init_cached(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    bar_bf_adapt_cfg_src(app_cfg_src, RefreshMode::NoRefresh);
}
