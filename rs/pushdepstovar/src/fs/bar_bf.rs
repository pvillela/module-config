use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{cfg_global_to_thread_local, CfgArcSwapArc, CfgRefCellRc};

pub type BarBfCfg = CfgArcSwapArc<BarBfCfgInfo>;

pub fn bar_bf() -> String {
    let cfg = BAR_BF_CFG_TL.with(|c| c.get_cfg());
    // Below is an alternative to the above, using the global config directly.
    // let cfg = unsafe { get_initialized_option(&BAR_BF_CFG) }.get_cfg();
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

thread_local! {
    pub static BAR_BF_CFG_TL: CfgRefCellRc<BarBfCfgInfo> = unsafe{cfg_global_to_thread_local(&BAR_BF_CFG)};
}

pub static mut BAR_BF_CFG: Option<BarBfCfg> = None;
