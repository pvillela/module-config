use crate::config::app_cfg_info::AppCfgInfo;
use crate::fs::bar_bf::BarBfCfgInfo;

pub fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo { z: app_cfg.y }
}
