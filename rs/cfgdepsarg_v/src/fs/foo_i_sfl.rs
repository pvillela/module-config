use super::BarIBfT;
use common::fs_data::FooISflCfgInfo;
use common::fs_util::foo_core;

pub type FooISflT = Box<dyn Fn() -> String>;

pub struct FooISflDeps {
    pub bar_i_bf: BarIBfT,
}

pub struct FooISflS {
    pub cfg: FooISflCfgInfo,
    pub deps: FooISflDeps,
}

pub fn foo_i_sfl_c(s: &FooISflS) -> String {
    let FooISflDeps { bar_i_bf } = &s.deps;
    let cfg = &s.cfg;
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_i_bf();
    foo_core(a, b, bar_res)
}
