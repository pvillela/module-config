use crate::fs::{bar_ai_bf_c, BarAiBfS, BarAiBfT};
use common::config::AppCfgInfo;
use common::fs_data::BarAiBfCfgInfo;
use common::fwk::ref_pin_async_fn;
use std::sync::Arc;

fn bar_ai_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAiBfCfgInfo {
    BarAiBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_ai_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> BarAiBfT {
    let cfg = bar_ai_bf_cfg_adapter(&app_cfg());
    let bar_ai_bf_s = Arc::new(BarAiBfS { cfg, deps: () });
    let f = move |sleep_millis| bar_ai_bf_c(bar_ai_bf_s.clone(), sleep_millis);
    ref_pin_async_fn(f)
}
