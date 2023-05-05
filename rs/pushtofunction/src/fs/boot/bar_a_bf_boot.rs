use crate::fs::{bar_a_bf_c, BarABfCfg, BarABfT};
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

pub fn bar_a_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> BarABfT {
    let bar_a_bf_cfg =
        BarABfCfg::new_boxed_with_cfg_adapter(app_cfg, bar_a_bf_cfg_adapter, refresh_mode);
    bar_a_bf_c(bar_a_bf_cfg)
}
