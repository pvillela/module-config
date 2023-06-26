use crate::fs::BarBfCfgInfo;
use common::config::AppCfgInfo;

pub fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo { z: app_cfg.y }
}
