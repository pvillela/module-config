use crate::fs::{bar_bf_c, BarBfCfgDeps, BarBfT};
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

pub fn bar_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> BarBfT {
    let bar_bf_cfg_deps =
        BarBfCfgDeps::new_boxed_with_cfg_adapter(app_cfg, bar_bf_cfg_adapter, refresh_mode, ());
    bar_bf_c(bar_bf_cfg_deps)
}
