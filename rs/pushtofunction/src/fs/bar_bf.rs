use common::config::AppCfgInfo;
use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{CfgRefCellRc, RefreshMode};
use std::rc::Rc;
use std::sync::Arc;

pub type BarBfT = Rc<dyn Fn() -> String>;

pub type BarBfCfg = CfgRefCellRc<BarBfCfgInfo>;

pub fn bar_bf_c(cfg: BarBfCfg) -> BarBfT {
    let f = move || {
        let cfg = cfg.get_cfg();
        let u = cfg.u;
        let v = cfg.v.clone();
        bar_core(u, v)
    };
    Rc::new(f)
}

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_bf_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> BarBfT {
    let bar_bf_cfg =
        BarBfCfg::new_boxed_with_cfg_adapter(app_cfg, bar_bf_cfg_adapter, refresh_mode);
    bar_bf_c(bar_bf_cfg)
}
