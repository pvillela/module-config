use crate::fs::{bar_a_bf, BarABfCfg, BarABfT, BAR_A_BF_CFG};
use common::config::AppCfgInfo;
use common::fs_data::BarABfCfgInfo;
use common::fwk::{init_option, RefreshMode};
use common::pin_async_fn;
use std::sync::Arc;

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn get_bar_a_bf_raw(cfg: BarABfCfg) -> BarABfT {
    unsafe {
        init_option(&mut BAR_A_BF_CFG, cfg);
    }
    pin_async_fn!(bar_a_bf)
}

pub fn get_bar_a_bf(app_cfg_src: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> BarABfT {
    get_bar_a_bf_raw(BarABfCfg::new_boxed_with_cfg_adapter(
        app_cfg_src,
        bar_a_bf_cfg_adapter,
        refresh_mode,
    ))
}
