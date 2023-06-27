use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::BarIBfCfgInfo;
use common::fs_util::bar_core;
use std::rc::Rc;
use std::sync::OnceLock;

pub type BarIBfT = fn() -> String;

pub fn bar_i_bf() -> String {
    // This is to demonstrate use of global config instead of thread-local.
    let _cfg = get_cfg();

    let cfg = BAR_I_BF_CFG_TL.with(|c| c.clone());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

pub static BAR_I_BF_CFG: OnceLock<BarIBfCfgInfo> = OnceLock::new();

fn get_cfg() -> &'static BarIBfCfgInfo {
    BAR_I_BF_CFG.get_or_init(|| bar_i_bf_cfg_adapter(&get_app_configuration()))
}

thread_local! {
    pub static BAR_I_BF_CFG_TL: Rc<BarIBfCfgInfo> = Rc::new(get_cfg().clone());
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
pub fn bar_i_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarIBfCfgInfo {
    BarIBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}
