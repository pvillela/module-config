use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{
    cfg_once_cell_to_thread_local, get_from_once_cell, set_once_cell, CfgArcSwapArc, CfgRefCellRc,
};
use std::sync::OnceLock;

pub type BarBfCfg = CfgArcSwapArc<BarBfCfgInfo>;

pub type BarBfT = fn() -> String;

fn bar_bf() -> String {
    // This is to demonstrate use of global config instead of thread-local.
    let _ = get_from_once_cell(&BAR_BF_CFG).get_cfg();

    let cfg = BAR_BF_CFG_TL.with(|c| c.get_cfg());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

thread_local! {
    pub static BAR_BF_CFG_TL: CfgRefCellRc<BarBfCfgInfo> = cfg_once_cell_to_thread_local(&BAR_BF_CFG);
}

static BAR_BF_CFG: OnceLock<BarBfCfg> = OnceLock::new();

pub fn get_bar_bf_raw(cfg: BarBfCfg) -> BarBfT {
    let _ = set_once_cell(&BAR_BF_CFG, cfg);
    bar_bf
}
