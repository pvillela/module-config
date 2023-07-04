use super::BarBfT;
use common::fs_data::FooSflCfgInfo;
use common::fs_util::foo_core;
use common::fwk::CfgRefCellRc;

pub type FooSflT = Box<dyn Fn() -> String>;

pub type FooSflCfg = CfgRefCellRc<FooSflCfgInfo>;

pub struct FooSflDeps {
    pub bar_bf: BarBfT,
}

pub struct FooSflS {
    pub cfg: FooSflCfg,
    pub deps: FooSflDeps,
}

pub fn foo_sfl_c(s: &FooSflS) -> String {
    let FooSflDeps { bar_bf } = &s.deps;
    let cfg = s.cfg.get_cfg();
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_bf();
    foo_core(a, b, bar_res)
}
