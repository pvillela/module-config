use common::fs_data::FooSflCfgInfo;
use common::fs_util::foo_core;
use common::fwk::CfgDepsRefCellRc;
use std::rc::Rc;

pub type FooSflT = Rc<dyn Fn() -> String>;

pub type FooSflCfgDeps = CfgDepsRefCellRc<FooSflCfgInfo, FooSflDeps>;

#[derive(Clone)]
pub struct FooSflDeps {
    pub bar_bf: Rc<dyn Fn() -> String>,
}

pub fn foo_sfl_c(cfg_deps: FooSflCfgDeps) -> FooSflT {
    let deps = cfg_deps.get_deps();
    let f = move || {
        let cfg = cfg_deps.get_cfg();
        let a = cfg.a.clone();
        let b = cfg.b;
        let bar_bf = &deps.bar_bf;
        let bar_ret = bar_bf();
        foo_core(a, b, bar_ret)
    };
    Rc::new(f)
}
