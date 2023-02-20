use crate::config::AppCfgInfo;
use crate::fs::bar_bf::{bar_bf_c, BarBfCfgInfo, BarBfCfgSrc, BarBfT};
use crate::fwk::const_or_adapt_by_ref;
use once_cell::sync::OnceCell;
use std::sync::Arc;

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo { z: app_cfg.y }
}

pub static BAR_BF_CFG_INFO_OVERRIDE: OnceCell<BarBfCfgInfo> = OnceCell::new();

pub fn bar_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> BarBfT {
    let get = const_or_adapt_by_ref(BAR_BF_CFG_INFO_OVERRIDE.get(), app_cfg, bar_bf_cfg_adapter);
    let bar_bf_cfg_src = BarBfCfgSrc { get };
    bar_bf_c(bar_bf_cfg_src)
}
