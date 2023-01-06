use std::ops::Deref;

use crate::config::app_cfg_info::AppCfgInfo;
use crate::fs::bar_bf::{BarBfCfgInfo, BAR_BF_CFG_SRC};
use crate::fwk::cfg_src::CfgSrcAdaptation;
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

pub fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo { z: app_cfg.y }
}

pub static barBfCfgAdaptation: Lazy<ArcSwap<CfgSrcAdaptation<AppCfgInfo, BarBfCfgInfo>>> =
    Lazy::new(|| {
        ArcSwap::from_pointee(CfgSrcAdaptation {
            targetSrc: BAR_BF_CFG_SRC.deref(),
            adapter: bar_bf_cfg_adapter,
        })
    });
