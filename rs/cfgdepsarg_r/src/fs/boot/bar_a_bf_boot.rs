use crate::fs::{bar_a_bf_c, BarABfCfg, BarABfS, BarABfT};
use common::config::AppCfgInfo;
use common::fs_data::BarABfCfgInfo;
use common::fwk::{box_pin_async_fn, RefreshMode};
use std::sync::{Arc, OnceLock};

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_a_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> Box<BarABfT> {
    let cfg =
        BarABfCfg::new_boxed_with_cfg_adapter(app_cfg, bar_a_bf_cfg_adapter, refresh_mode.clone());
    let bar_a_bf_s = Arc::new(BarABfS { cfg, deps: () });
    let f = move |sleep_millis| bar_a_bf_c(bar_a_bf_s.clone(), sleep_millis);
    box_pin_async_fn(f)
}

// The only benefit of this version over the above is that it saves an Arc clone for each call to the returned function.
pub fn bar_a_bf_boot_xr(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<BarABfT> {
    static BAR_A_BF_S_X: OnceLock<BarABfS> = OnceLock::new();
    let bar_a_bf_s = BAR_A_BF_S_X.get_or_init(|| {
        let cfg = BarABfCfg::new_boxed_with_cfg_adapter(
            app_cfg,
            bar_a_bf_cfg_adapter,
            refresh_mode.clone(),
        );
        BarABfS { cfg, deps: () }
    });
    let f = move |sleep_millis| bar_a_bf_c(bar_a_bf_s, sleep_millis);
    box_pin_async_fn(f)
}
