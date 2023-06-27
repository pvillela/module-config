use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{
    cfg_to_thread_local, get_from_once_lock, set_once_lock, CfgArcSwapArc, CfgRefCellRc,
};
use std::sync::OnceLock;

pub type BarBfCfg = CfgArcSwapArc<BarBfCfgInfo>;

pub type BarBfT = fn() -> String;

fn bar_bf() -> String {
    // This is to demonstrate use of global config instead of thread-local.
    let _cfg = get_cfg().get_cfg();

    let cfg = BAR_BF_CFG_TL.with(|c| c.get_cfg());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

static BAR_BF_CFG: OnceLock<BarBfCfg> = OnceLock::new();

fn get_cfg() -> &'static BarBfCfg {
    get_from_once_lock(&BAR_BF_CFG)
}

thread_local! {
    pub static BAR_BF_CFG_TL: CfgRefCellRc<BarBfCfgInfo> = cfg_to_thread_local(get_cfg());
}

pub fn get_bar_bf_raw(cfg: BarBfCfg) -> BarBfT {
    let _ = set_once_lock(&BAR_BF_CFG, cfg);
    bar_bf
}
