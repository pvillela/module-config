use common::fs_util::foo_core;
use common::fwk::{CfgDepsDefault, CfgDepsMut};

type FooSflCfgInfo = common::fs_data::FooSflCfgInfo;

pub type FooSflT = Box<dyn FnMut() -> String>;

type FooSflCfgDeps = CfgDepsDefault<FooSflCfgInfo, FooSflDeps>;

#[derive(Clone, Debug)]
pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub fn foo_sfl_c(cfg_deps: FooSflCfgDeps) -> FooSflT {
    let mut x = cfg_deps.get_inner_clone();
    let f = move || {
        let (cfg, deps, _) = x.get_mut();
        let a = cfg.a.clone();
        let b = cfg.b;
        let bar_bf = deps.bar_bf;
        let bar_ret = bar_bf();
        foo_core(a, b, bar_ret)
    };
    Box::new(f)
}
