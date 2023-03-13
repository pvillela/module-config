use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::CfgDepsArcSwapArc;
use std::sync::Arc;

pub type BarBfT = Arc<dyn Fn() -> String + Send + Sync>;

pub type BarBfCfgDeps = CfgDepsArcSwapArc<BarBfCfgInfo, ()>;

pub fn bar_bf_c(cfg_deps: BarBfCfgDeps) -> BarBfT {
    let f = move || {
        let (cfg, _) = cfg_deps.get_cfg_deps();
        let u = cfg.u;
        let v = cfg.v.clone();
        bar_core(u, v)
    };
    Arc::new(f)
}
