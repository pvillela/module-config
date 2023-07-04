use crate::fs::{bar_i_bf_c, BarIBfS, BarIBfT};
use common::config::AppCfgInfo;
use common::fs_data::BarIBfCfgInfo;
use std::rc::Rc;
use std::sync::Arc;

fn bar_i_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarIBfCfgInfo {
    BarIBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_i_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> BarIBfT {
    let cfg = bar_i_bf_cfg_adapter(&app_cfg());
    let bar_i_bf_s = Rc::new(BarIBfS { cfg });
    let f = move || bar_i_bf_c(&bar_i_bf_s.clone());
    Box::new(f)
}
