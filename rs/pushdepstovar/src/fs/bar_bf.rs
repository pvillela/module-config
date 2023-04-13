use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{cfg_once_cell_to_thread_local, CfgArcSwapArc, CfgRefCellRc};
use once_cell::sync::OnceCell;

pub type BarBfCfg = CfgArcSwapArc<BarBfCfgInfo>;

pub fn bar_bf() -> String {
    let cfg = BAR_BF_CFG_TL.with(|c| c.get_cfg());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

thread_local! {
    pub static BAR_BF_CFG_TL: CfgRefCellRc<BarBfCfgInfo> = cfg_once_cell_to_thread_local(&BAR_BF_CFG);
}

pub static BAR_BF_CFG: OnceCell<BarBfCfg> = OnceCell::new();
