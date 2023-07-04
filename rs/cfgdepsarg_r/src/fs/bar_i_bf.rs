use common::fs_data::BarIBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::CfgDeps;

pub type BarIBfT = Box<dyn Fn() -> String>;

pub type BarIBfS = CfgDeps<BarIBfCfgInfo, ()>;

pub fn bar_i_bf_c(s: &BarIBfS) -> String {
    let cfg = &s.cfg;
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}
