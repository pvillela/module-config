use crate::fs::{bar_aw_bf_c, BarAwBfCfgDeps, BarAwBfT};
use common::config::AppCfgInfo;
use common::fs_data::BarAwBfCfgInfo;
use common::fwk::RefreshMode;
use std::sync::Arc;

fn bar_aw_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAwBfCfgInfo {
    BarAwBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_aw_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> BarAwBfT {
    let bar_aw_bf_cfg_deps = BarAwBfCfgDeps::new_boxed_with_cfg_adapter(
        app_cfg,
        bar_aw_bf_cfg_adapter,
        refresh_mode,
        (),
    );
    bar_aw_bf_c(bar_aw_bf_cfg_deps)
}
