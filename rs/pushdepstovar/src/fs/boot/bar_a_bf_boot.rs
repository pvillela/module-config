use crate::fs::{BarABfCfg, BAR_A_BF_CFG};
use common::config::AppCfgInfo;
use common::fs_data::BarABfCfgInfo;
use common::fwk::{init_option, RefreshMode};
use std::sync::Arc;
use std::time::Duration;

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_a_bf_init(origin: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) {
    unsafe {
        init_option(
            &mut BAR_A_BF_CFG,
            BarABfCfg::new_boxed_with_cfg_adapter(origin, bar_a_bf_cfg_adapter, refresh_mode),
        );
    }
}

pub fn bar_a_bf_init_refreshable(app_cfg_src: fn() -> Arc<AppCfgInfo>, refresh_millis: u64) {
    bar_a_bf_init(
        app_cfg_src,
        RefreshMode::Refreshable(Duration::from_millis(refresh_millis)),
    );
}

pub fn bar_a_bf_init_no_refresh(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    bar_a_bf_init(app_cfg_src, RefreshMode::NoRefresh);
}
