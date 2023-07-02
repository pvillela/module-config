use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::CfgArcSwapArc;

pub type BarBfCfg = CfgArcSwapArc<BarBfCfgInfo>;

pub struct BarBfS {
    pub cfg: BarBfCfg,
}

impl BarBfS {
    pub fn run(&self) -> String {
        let cfg = self.cfg.get_cfg();
        let u = cfg.u;
        let v = cfg.v.clone();
        bar_core(u, v)
    }
}
