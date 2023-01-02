use crate::config::app_cfg_info::AppCfgInfo;
use crate::fs::bar_bf::BarBfCfgInfo;
use std::sync::Arc;

pub fn barBfCfgAdapter(appCfg: &AppCfgInfo) -> Arc<BarBfCfgInfo> {
    Arc::new(BarBfCfgInfo { z: appCfg.y })
}
