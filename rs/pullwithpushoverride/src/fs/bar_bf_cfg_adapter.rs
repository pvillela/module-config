use crate::config::app_cfg_info::AppCfgInfo;
use crate::fs::bar_bf::BarBfCfgInfo;
use std::sync::Arc;

pub fn barBfCfgAdapter_arc(appCfg: &AppCfgInfo) -> Arc<BarBfCfgInfo> {
    Arc::new(BarBfCfgInfo { z: appCfg.y })
}

pub fn barBfCfgAdapter(appCfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo { z: appCfg.y }
}
