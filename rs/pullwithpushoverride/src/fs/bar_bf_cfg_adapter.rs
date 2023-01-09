use crate::config::AppCfgInfo;
use crate::fs::BarBfCfgInfo;

pub fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo { z: app_cfg.y }
}
