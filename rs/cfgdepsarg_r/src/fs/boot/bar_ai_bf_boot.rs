use crate::fs::{bar_ai_bf_c, BarAiBfS, BarAiBfT};
use common::config::AppCfgInfo;
use common::fs_data::BarAiBfCfgInfo;
use common::fwk::box_pin_async_fn;
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

// The only benefit of this version over the above is that it saves an Arc clone for each call to the returned function.
pub fn bar_ai_bf_boot_xr(app_cfg: fn() -> Arc<AppCfgInfo>) -> Box<BarAiBfT> {
    static BAR_AI_BF_S_X: OnceLock<BarAiBfS> = OnceLock::new();
    let bar_ai_bf_s = BAR_AI_BF_S_X.get_or_init(|| {
        let cfg = bar_ai_bf_cfg_adapter(&app_cfg());
        BarAiBfS { cfg, deps: () }
    });
    let f = move |sleep_millis| bar_ai_bf_c(bar_ai_bf_s, sleep_millis);
    box_pin_async_fn(f)
}
