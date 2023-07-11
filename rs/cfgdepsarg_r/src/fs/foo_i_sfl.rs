use super::BarIBfT;
use common::fs_data::FooISflCfgInfo;
use common::fs_util::foo_core;
use common::fwk::CfgDeps;

pub type FooISflT = dyn Fn(()) -> String + Send + Sync;

pub struct FooISflDeps {
    pub bar_i_bf: Box<BarIBfT>,
}

pub type FooISflS = CfgDeps<FooISflCfgInfo, FooISflDeps>;

pub fn foo_i_sfl_c(s: &FooISflS, _: ()) -> String {
    let FooISflDeps { bar_i_bf } = &s.deps;
    let cfg = &s.cfg;
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_i_bf(());
    foo_core(a, b, bar_res)
}
