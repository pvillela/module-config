use crate::config::AppCfgInfo;
use crate::fs::bar_bf::{bar_bf_c, BarBfCfgInfo, BarBfCfgSrc, BarBfT};
use crate::fwk::const_or_adapt_by_ref;
use std::sync::{Arc, OnceLock};

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub static BAR_BF_CFG_INFO_OVERRIDE: OnceLock<BarBfCfgInfo> = OnceLock::new();

pub fn bar_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> BarBfT {
    let get = const_or_adapt_by_ref(BAR_BF_CFG_INFO_OVERRIDE.get(), app_cfg, bar_bf_cfg_adapter);
    let bar_bf_cfg_src = BarBfCfgSrc { get };
    bar_bf_c(bar_bf_cfg_src)
}
