use crate::fs::{BarBfCfgInfo, BAR_BF_CFG_SRC};
use crate::fwk::{adapt_by_ref, RefreshMode};
use common::config::AppCfgInfo;
use std::sync::Arc;

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_bf_adapt_cfg_src(
    origin: impl Fn() -> Arc<AppCfgInfo> + 'static + Send + Sync,
    refresh_mode: RefreshMode,
) {
    adapt_by_ref(origin, bar_bf_cfg_adapter, refresh_mode, &BAR_BF_CFG_SRC);
}
