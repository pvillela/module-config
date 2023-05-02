use crate::fs::{BarBfCfg, BAR_BF_CFG};
use common::config::AppCfgInfo;
use common::fs_data::BarBfCfgInfo;
use common::fwk::{init_option, RefreshMode};
use std::sync::Arc;
use std::time::Duration;

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_bf_init(origin: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) {
    unsafe {
        init_option(
            &mut BAR_BF_CFG,
            BarBfCfg::new_boxed_with_cfg_adapter(origin, bar_bf_cfg_adapter, refresh_mode),
        );
    }
}

pub fn bar_bf_init_refreshable(app_cfg_src: fn() -> Arc<AppCfgInfo>, refresh_millis: u64) {
    bar_bf_init(
        app_cfg_src,
        RefreshMode::Refreshable(Duration::from_millis(refresh_millis)),
    );
}

pub fn bar_bf_init_no_refresh(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    bar_bf_init(app_cfg_src, RefreshMode::NoRefresh);
}
