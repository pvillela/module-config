use crate::fs::{BarBfCfg, BarBfS};
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::BarBfCfgInfo;
use common::fwk::RefreshMode;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

fn get_bar_bf_s_with_app_cfg(
    app_cfg_src: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> BarBfS {
    BarBfS {
        cfg: BarBfCfg::new_boxed_with_cfg_adapter(app_cfg_src, bar_bf_cfg_adapter, refresh_mode),
    }
}

pub fn get_bar_bf_s_no_refresh() -> &'static BarBfS {
    static BAR_A_BF_S: OnceLock<BarBfS> = OnceLock::new();
    BAR_A_BF_S
        .get_or_init(|| get_bar_bf_s_with_app_cfg(get_app_configuration, RefreshMode::NoRefresh))
}

pub fn get_bar_bf_s_cached() -> &'static BarBfS {
    static BAR_A_BF_S_CACHED: OnceLock<BarBfS> = OnceLock::new();
    BAR_A_BF_S_CACHED.get_or_init(|| {
        get_bar_bf_s_with_app_cfg(
            get_app_configuration,
            RefreshMode::Refreshable(Duration::from_millis(150)),
        )
    })
}
