use crate::fs::bar_bf::get_bar_bf_raw;
use crate::fs::{BarBfCfg, BarBfT};
use common::config::AppCfgInfo;
use common::fs_data::BarBfCfgInfo;
use common::fwk::RefreshMode;
use std::sync::Arc;

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn get_bar_bf_with_app_cfg(
    app_cfg_src: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> BarBfT {
    get_bar_bf_raw(BarBfCfg::new_boxed_with_cfg_adapter(
        app_cfg_src,
        bar_bf_cfg_adapter,
        refresh_mode,
    ))
}
