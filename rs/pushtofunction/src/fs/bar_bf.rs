use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::CfgDepsRefCellRc;
use std::rc::Rc;

pub type BarBfT = Rc<dyn Fn() -> String>;

pub type BarBfCfgDeps = CfgDepsRefCellRc<BarBfCfgInfo, ()>;

pub fn bar_bf_c(cfg_deps: BarBfCfgDeps) -> BarBfT {
    let f = move || {
        let cfg = cfg_deps.get_cfg();
        let u = cfg.u;
        let v = cfg.v.clone();
        bar_core(u, v)
    };
    Rc::new(f)
}
