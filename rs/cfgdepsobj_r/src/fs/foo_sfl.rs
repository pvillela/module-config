use super::BarBfS;
use common::{fs_data::FooSflCfgInfo, fs_util::foo_core, fwk::CfgArcSwapArc};

pub type FooSflCfg = CfgArcSwapArc<FooSflCfgInfo>;

pub struct FooSflDeps {
    pub bar_bf_s: &'static BarBfS,
}

pub struct FooSflS {
    pub cfg: FooSflCfg,
    pub deps: FooSflDeps,
}

impl FooSflS {
    pub fn run(&self) -> String {
        let FooSflDeps { bar_bf_s } = self.deps;
        let cfg = self.cfg.get_cfg();
        let a = cfg.a.clone();
        let b = cfg.b;
        let bar_res = bar_bf_s.run();
        foo_core(a, b, bar_res)
    }
}
