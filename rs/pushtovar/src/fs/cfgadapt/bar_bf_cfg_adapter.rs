use crate::config::AppCfgInfo;
use crate::fs::{BarBfCfgInfo, BAR_BF_CFG_SRC};
use crate::fwk::{adapt_by_ref, ArcCache};
use std::sync::Arc;

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo { z: app_cfg.y }
}

pub fn bar_bf_adapt_cfg_src(
    origin: impl Fn() -> Arc<AppCfgInfo> + 'static + Send + Sync,
    cache_ref: &mut ArcCache<BarBfCfgInfo>,
) {
    adapt_by_ref(origin, bar_bf_cfg_adapter, cache_ref, &BAR_BF_CFG_SRC);
}
