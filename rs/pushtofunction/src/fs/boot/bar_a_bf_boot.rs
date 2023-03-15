use crate::fs::{bar_a_bf_c, BarABfCfgDeps, BarABfT};
use common::config::AppCfgInfo;
use common::fs_data::BarABfCfgInfo;
use common::fwk::RefreshMode;
use once_cell::sync::OnceCell;
use std::sync::Arc;

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub static BAR_A_BF_CFG_INFO_OVERRIDE: OnceCell<BarABfCfgInfo> = OnceCell::new();

pub fn bar_a_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> BarABfT {
    let bar_a_bf_cfg_deps = BarABfCfgDeps::new_with_const_or_cfg_adapter(
        BAR_A_BF_CFG_INFO_OVERRIDE.get(),
        app_cfg,
        bar_a_bf_cfg_adapter,
        refresh_mode,
        (),
    );
    bar_a_bf_c(bar_a_bf_cfg_deps)
}
