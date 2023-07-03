use crate::fs::{bar_a_bf_c, BarABfCfg, BarABfD, BarABfS};
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::BarABfCfgInfo;
use common::fwk::RefreshMode;
use common::pin_async_fn_2;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

fn get_bar_a_bf_s_with_app_cfg(
    app_cfg_src: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> BarABfS {
    BarABfS {
        cfg: BarABfCfg::new_boxed_with_cfg_adapter(app_cfg_src, bar_a_bf_cfg_adapter, refresh_mode),
    }
}

pub fn get_bar_a_bf_d_no_refresh() -> BarABfD {
    static BAR_A_BF_S: OnceLock<BarABfS> = OnceLock::new();
    let bar_a_bf_s = BAR_A_BF_S
        .get_or_init(|| get_bar_a_bf_s_with_app_cfg(get_app_configuration, RefreshMode::NoRefresh));
    BarABfD {
        s: bar_a_bf_s,
        f: pin_async_fn_2!(bar_a_bf_c),
    }
}

pub fn get_bar_a_bf_d_cached() -> BarABfD {
    static BAR_A_BF_S_CACHED: OnceLock<BarABfS> = OnceLock::new();
    let bar_a_bf_s = BAR_A_BF_S_CACHED.get_or_init(|| {
        get_bar_a_bf_s_with_app_cfg(
            get_app_configuration,
            RefreshMode::Refreshable(Duration::from_millis(150)),
        )
    });
    BarABfD {
        s: bar_a_bf_s,
        f: pin_async_fn_2!(bar_a_bf_c),
    }
}
