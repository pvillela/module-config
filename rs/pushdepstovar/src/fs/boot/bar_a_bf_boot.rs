use crate::fs::bar_a_bf::get_bar_a_bf_raw;
use crate::fs::{BarABfCfg, BarABfT};
use common::config::AppCfgInfo;
use common::fs_data::BarABfCfgInfo;
use common::fwk::RefreshMode;
use std::sync::Arc;

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn get_bar_a_bf_with_app_cfg(
    app_cfg_src: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> BarABfT {
    get_bar_a_bf_raw(BarABfCfg::new_boxed_with_cfg_adapter(
        app_cfg_src,
        bar_a_bf_cfg_adapter,
        refresh_mode,
    ))
}
