use crate::config::app_cfg_info::AppCfgInfo;
use crate::fs::bar_bf::BarBfCfgInfo;

pub fn barBfCfgAdapter(appCfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo { z: appCfg.y }
}
