use common::fs_data::BarIBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{get_from_once_cell, set_once_cell};
use std::sync::OnceLock;

pub type BarIBfT = fn() -> String;

fn bar_i_bf() -> String {
    let cfg = get_from_once_cell(&BAR_I_BF_CFG);
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

static BAR_I_BF_CFG: OnceLock<BarIBfCfgInfo> = OnceLock::new();

pub fn get_bar_i_bf_raw(cfg: BarIBfCfgInfo) -> BarIBfT {
    let _ = set_once_cell(&BAR_I_BF_CFG, cfg);
    bar_i_bf
}
