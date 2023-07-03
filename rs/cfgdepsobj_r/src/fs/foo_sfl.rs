use super::bar_bf::BarBfD;
use common::{
    fs_data::FooSflCfgInfo,
    fs_util::foo_core,
    fwk::{CfgArcSwapArc, Dep0},
};

pub type FooSflCfg = CfgArcSwapArc<FooSflCfgInfo>;

pub struct FooSflDeps {
    pub bar_bf_d: BarBfD,
}

pub struct FooSflS {
    pub cfg: FooSflCfg,
    pub deps: FooSflDeps,
}

pub type FooSflD = Dep0<FooSflS, String>;

pub fn foo_sfl_c(s: &FooSflS) -> String {
    let FooSflDeps { bar_bf_d } = &s.deps;
    let cfg = s.cfg.get_cfg();
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_bf_d.run();
    foo_core(a, b, bar_res)
}
