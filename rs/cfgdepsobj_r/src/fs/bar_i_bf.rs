use common::fs_data::BarIBfCfgInfo;
use common::fs_util::bar_core;

pub struct BarIBfS {
    pub cfg: BarIBfCfgInfo,
}

impl BarIBfS {
    pub fn run(&self) -> String {
        let cfg = &self.cfg;
        let u = cfg.u;
        let v = cfg.v.clone();
        bar_core(u, v)
    }
}
