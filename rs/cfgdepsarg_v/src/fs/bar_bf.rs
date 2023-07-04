use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{CfgDeps, CfgRefCellRc};

pub type BarBfT = Box<dyn Fn() -> String>;

pub type BarBfCfg = CfgRefCellRc<BarBfCfgInfo>;

pub type BarBfS = CfgDeps<BarBfCfg, ()>;

pub fn bar_bf_c(s: &BarBfS) -> String {
    let cfg = s.cfg.get_cfg();
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}
