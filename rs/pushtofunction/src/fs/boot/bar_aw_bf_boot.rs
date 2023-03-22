use crate::fs::{bar_aw_bf_c, BarAwBfCfgDeps, BarAwBfT};
use common::config::AppCfgInfo;
use common::fs_data::BarAwBfCfgInfo;
use common::fwk::RefreshMode;
use once_cell::sync::OnceCell;
use std::sync::Arc;

fn bar_aw_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAwBfCfgInfo {
    BarAwBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub static BAR_AW_BF_CFG_INFO_OVERRIDE: OnceCell<BarAwBfCfgInfo> = OnceCell::new();

pub fn bar_aw_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> BarAwBfT {
    let bar_aw_bf_cfg_deps = BarAwBfCfgDeps::new_boxed_with_const_or_cfg_adapter(
        BAR_AW_BF_CFG_INFO_OVERRIDE.get(),
        app_cfg,
        bar_aw_bf_cfg_adapter,
        refresh_mode,
        (),
    );
    bar_aw_bf_c(bar_aw_bf_cfg_deps)
}
