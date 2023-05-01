use common::fs_data::BarIBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::get_initialized_option;

pub fn bar_i_bf() -> String {
    let cfg = get_my_cfg();
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

pub static mut BAR_I_BF_CFG: Option<BarIBfCfgInfo> = None;

fn get_my_cfg() -> &'static BarIBfCfgInfo {
    unsafe { get_initialized_option(&BAR_I_BF_CFG) }
}
