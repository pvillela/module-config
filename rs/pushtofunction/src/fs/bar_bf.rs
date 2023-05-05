use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::CfgRefCellRc;
use std::rc::Rc;

pub type BarBfT = Rc<dyn Fn() -> String>;

pub type BarBfCfg = CfgRefCellRc<BarBfCfgInfo>;

pub fn bar_bf_c(cfg: BarBfCfg) -> BarBfT {
    let f = move || {
        let cfg = cfg.get_cfg();
        let u = cfg.u;
        let v = cfg.v.clone();
        bar_core(u, v)
    };
    Rc::new(f)
}
