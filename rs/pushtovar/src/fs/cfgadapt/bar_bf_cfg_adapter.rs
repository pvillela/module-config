use crate::config::AppCfgInfo;
use crate::fs::{BarBfCfgInfo, BAR_BF_CFG_SRC};
use crate::fwk::CfgSrcAdaptation;
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use std::ops::Deref;

pub fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo { z: app_cfg.y }
}

pub static BAR_BF_CFG_ADAPTATION: Lazy<ArcSwap<CfgSrcAdaptation<AppCfgInfo, BarBfCfgInfo>>> =
    Lazy::new(|| {
        ArcSwap::from_pointee(CfgSrcAdaptation {
            target_src: BAR_BF_CFG_SRC.deref(),
            adapter: bar_bf_cfg_adapter,
        })
    });
