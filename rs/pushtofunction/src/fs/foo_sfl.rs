use common::fs_data::FooSflCfgInfo;
use common::fs_util::foo_core;
use common::fwk::CfgDepsArcSwapArc;
use std::sync::Arc;

pub type FooSflT = Arc<dyn Fn() -> String + Send + Sync>;

pub type FooSflCfgDeps = CfgDepsArcSwapArc<FooSflCfgInfo, FooSflDeps>;

#[derive(Clone)]
pub struct FooSflDeps {
    pub bar_bf: Arc<dyn Fn() -> String + Send + Sync>,
}

pub fn foo_sfl_c(cfg_deps: FooSflCfgDeps) -> FooSflT {
    let f = move || {
        let (cfg, deps) = cfg_deps.get_cfg_deps();
        let a = cfg.a.clone();
        let b = cfg.b;
        let bar_bf = deps.bar_bf;
        let bar_ret = bar_bf();
        foo_core(a, b, bar_ret)
    };
    Arc::new(f)
}
