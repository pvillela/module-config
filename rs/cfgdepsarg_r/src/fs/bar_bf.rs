use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::CfgArcSwapArc;
use std::sync::Arc;

pub type BarBfT = Arc<dyn Fn() -> String + Send + Sync>;

pub type BarBfCfg = CfgArcSwapArc<BarBfCfgInfo>;

pub struct BarBfS {
    pub cfg: BarBfCfg,
}

pub fn bar_bf_c(s: &BarBfS) -> String {
    let cfg = s.cfg.get_cfg();
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}
