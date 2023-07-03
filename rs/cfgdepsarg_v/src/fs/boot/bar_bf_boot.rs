use crate::fs::{bar_bf_c, BarBfCfg, BarBfS, BarBfT};
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
    let f = move || {
        let cfg =
            BarBfCfg::new_boxed_with_cfg_adapter(app_cfg, bar_bf_cfg_adapter, refresh_mode.clone());
        let bar_bf_s = BarBfS { cfg };
        bar_bf_c(bar_bf_s)
    };
    Box::new(f)
}
