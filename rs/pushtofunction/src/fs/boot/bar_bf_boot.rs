use crate::config::app_cfg_info::AppCfgInfo;
use crate::fs::bar_bf::{bar_bf_c, BarBfCfgInfo, BarBfCfgSrc, BarBfT};
use crate::fwk::cfg_adapter::lift_to_nullary;
use crate::fwk::cfg_adapter::DressedCfgAdapter;
use once_cell::sync::Lazy;
use std::sync::Arc;

fn bar_bf_cfg_adapter0(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo { z: app_cfg.y }
}

pub static BAR_BF_CFG_ADAPTER: Lazy<DressedCfgAdapter<AppCfgInfo, BarBfCfgInfo>> =
    Lazy::new(|| lift_to_nullary(bar_bf_cfg_adapter0));

pub fn bar_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> BarBfT {
    let bar_bf_cfg_src = BarBfCfgSrc {
        get: (BAR_BF_CFG_ADAPTER.load())(app_cfg),
    };
    bar_bf_c(bar_bf_cfg_src)
}
