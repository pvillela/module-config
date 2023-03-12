use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::CfgDepsDefault;

pub type BarBfT = Box<dyn Fn() -> String>;

pub type BarBfCfgDeps = CfgDepsDefault<BarBfCfgInfo, ()>;

pub fn bar_bf_c(cfg_deps: BarBfCfgDeps) -> BarBfT {
    let f = move || {
        let (cfg, _) = cfg_deps.get();
        let u = cfg.u;
        let v = cfg.v.clone();
        bar_core(u, v)
    };
    Box::new(f)
}
