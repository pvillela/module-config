use crate::fs::{bar_bf_c, BarBfCfgDeps, BarBfT};
use common::config::AppCfgInfo;
use common::fs_data::BarBfCfgInfo;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use std::time::Duration;

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub static BAR_BF_CFG_INFO_OVERRIDE: OnceCell<BarBfCfgInfo> = OnceCell::new();

pub fn bar_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> BarBfT {
    let bar_bf_cfg_deps = BarBfCfgDeps::new_with_const_or_cfg_adapter(
        BAR_BF_CFG_INFO_OVERRIDE.get(),
        app_cfg,
        bar_bf_cfg_adapter,
        common::fwk::RefreshMode::Refreshable(Duration::from_millis(0)),
        (),
    );
    bar_bf_c(bar_bf_cfg_deps)
}
