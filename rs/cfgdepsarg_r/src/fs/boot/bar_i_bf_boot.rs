use crate::fs::{bar_i_bf_c, BarIBfS, BarIBfT};
use common::config::AppCfgInfo;
use common::fs_data::BarIBfCfgInfo;
use std::sync::Arc;

fn bar_i_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarIBfCfgInfo {
    BarIBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_i_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> Box<BarIBfT> {
    let cfg = bar_i_bf_cfg_adapter(&app_cfg());
    let bar_i_bf_s = { BarIBfS { cfg, deps: () } };
    let f = move || bar_i_bf_c(&bar_i_bf_s);
    Box::new(f)
}

pub fn bar_i_bf_boot_lr(app_cfg: fn() -> Arc<AppCfgInfo>) -> &'static BarIBfT {
    Box::leak(Box::new(bar_i_bf_boot(app_cfg)))
}
