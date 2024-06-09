use common::config::{AppCfg, AppCfgInfo};
use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::CfgRefCellRc;
use std::rc::Rc;

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

pub fn bar_bf_boot(app_cfg: AppCfg<AppCfgInfo>) -> BarBfT {
    let bar_bf_cfg = BarBfCfg::new_boxed_with_cfg_adapter(
        app_cfg.app_src,
        bar_bf_cfg_adapter,
        app_cfg.refresh_mode,
    );
    bar_bf_c(bar_bf_cfg)
}
