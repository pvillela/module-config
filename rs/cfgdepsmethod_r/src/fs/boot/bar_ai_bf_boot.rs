use crate::fs::{get_bar_ai_bf_raw, BarAiBfT};
use common::config::AppCfgInfo;
use common::fs_data::BarAiBfCfgInfo;
use std::sync::Arc;

fn bar_ai_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAiBfCfgInfo {
    BarAiBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn get_bar_ai_bf_with_app_cfg(app_cfg_src: fn() -> Arc<AppCfgInfo>) -> BarAiBfT {
    get_bar_ai_bf_raw(bar_ai_bf_cfg_adapter(&app_cfg_src()))
}
