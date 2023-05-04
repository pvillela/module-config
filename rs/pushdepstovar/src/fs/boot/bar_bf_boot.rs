use crate::fs::{bar_bf, BarBfCfg, BarBfT, BAR_BF_CFG};
use common::config::AppCfgInfo;
use common::fs_data::BarBfCfgInfo;
use common::fwk::{init_option, RefreshMode};
use std::sync::Arc;

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn get_bar_bf_raw(cfg: BarBfCfg) -> BarBfT {
    unsafe {
        init_option(&mut BAR_BF_CFG, cfg);
    }
    bar_bf
}

pub fn get_bar_bf(app_cfg_src: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> BarBfT {
    get_bar_bf_raw(BarBfCfg::new_boxed_with_cfg_adapter(
        app_cfg_src,
        bar_bf_cfg_adapter,
        refresh_mode,
    ))
}
