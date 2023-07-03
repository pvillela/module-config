use crate::fs::bar_bf::{bar_bf_c, BarBfD};
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

pub fn get_bar_bf_d_no_refresh() -> BarBfD {
    static BAR_BF_S: OnceLock<BarBfS> = OnceLock::new();
    let bar_bf_s = BAR_BF_S
        .get_or_init(|| get_bar_bf_s_with_app_cfg(get_app_configuration, RefreshMode::NoRefresh));
    BarBfD {
        s: bar_bf_s,
        f: bar_bf_c,
    }
}

pub fn get_bar_bf_d_cached() -> BarBfD {
    static BAR_BF_S_CACHED: OnceLock<BarBfS> = OnceLock::new();
    let bar_bf_s = BAR_BF_S_CACHED.get_or_init(|| {
        get_bar_bf_s_with_app_cfg(
            get_app_configuration,
            RefreshMode::Refreshable(Duration::from_millis(150)),
        )
    });
    BarBfD {
        s: bar_bf_s,
        f: bar_bf_c,
    }
}
