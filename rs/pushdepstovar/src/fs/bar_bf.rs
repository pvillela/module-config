use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{
    cfg_global_to_thread_local, get_initialized_option, CfgArcSwapArc, CfgRefCellRc,
};

pub type BarBfCfg = CfgArcSwapArc<BarBfCfgInfo>;

pub type BarBfT = fn() -> String;

pub(crate) fn bar_bf() -> String {
    // This is to demonstrate calling get_my_cfg() as an alternative to using the thread-local..
    let _ = get_my_cfg().get_cfg();

    let cfg = BAR_BF_CFG_TL.with(|c| c.get_cfg());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

thread_local! {
    pub static BAR_BF_CFG_TL: CfgRefCellRc<BarBfCfgInfo> = unsafe{cfg_global_to_thread_local(&BAR_BF_CFG)};
}

pub static mut BAR_BF_CFG: Option<BarBfCfg> = None;

fn get_my_cfg() -> &'static BarBfCfg {
    unsafe { get_initialized_option(&BAR_BF_CFG) }
}
