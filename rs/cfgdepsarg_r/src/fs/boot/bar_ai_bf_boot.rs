use crate::fs::{bar_ai_bf_c, BarAiBfS, BarAiBfT};
use common::config::get_app_configuration;
use common::fs_data::BarAiBfCfgInfo;
use common::fwk::{box_pin_async_fn, cfg_deps_boot_ai_lr};
use common::{config::AppCfgInfo, fwk::ref_pin_async_fn};
use std::sync::{Arc, OnceLock};

fn bar_ai_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAiBfCfgInfo {
    BarAiBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_ai_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> Box<BarAiBfT> {
    let cfg = bar_ai_bf_cfg_adapter(&app_cfg());
    let bar_ai_bf_s = Arc::new(BarAiBfS { cfg, deps: () });
    let f = move |sleep_millis| bar_ai_bf_c(bar_ai_bf_s.clone(), sleep_millis);
    box_pin_async_fn(f)
}

// The only benefit of this version over _boot is that it saves an Arc clone for each call to the returned function.
pub fn bar_ai_bf_boot_xs(app_cfg: fn() -> Arc<AppCfgInfo>) -> Box<BarAiBfT> {
    static BAR_AI_BF_S_X: OnceLock<BarAiBfS> = OnceLock::new();
    let bar_ai_bf_s = BAR_AI_BF_S_X.get_or_init(|| {
        let cfg = bar_ai_bf_cfg_adapter(&app_cfg());
        BarAiBfS { cfg, deps: () }
    });
    let f = move |sleep_millis| bar_ai_bf_c(bar_ai_bf_s, sleep_millis);
    box_pin_async_fn(f)
}

pub fn bar_ai_bf_boot_lr(app_cfg: fn() -> Arc<AppCfgInfo>) -> &'static BarAiBfT {
    let cfg = bar_ai_bf_cfg_adapter(&app_cfg());
    let bar_ai_bf_s: &BarAiBfS = Box::leak(Box::new(BarAiBfS { cfg, deps: () }));
    let f = move |sleep_millis| bar_ai_bf_c(bar_ai_bf_s, sleep_millis);
    ref_pin_async_fn(f)
}

pub fn get_bar_ai_bf() -> &'static BarAiBfT {
    static BAR_AI_BF: OnceLock<&BarAiBfT> = OnceLock::new();
    BAR_AI_BF.get_or_init(|| {
        cfg_deps_boot_ai_lr(
            bar_ai_bf_c,
            bar_ai_bf_cfg_adapter,
            get_app_configuration(),
            (),
        )
    })
}
