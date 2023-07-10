use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{CfgArcSwapArc, CfgDeps};

pub type BarBfT = dyn Fn(()) -> String + Send + Sync;

pub type BarBfCfg = CfgArcSwapArc<BarBfCfgInfo>;

pub type BarBfS = CfgDeps<BarBfCfg, ()>;

pub fn bar_bf_c(s: &BarBfS, _: ()) -> String {
    let cfg = s.cfg.get_cfg();
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}
