use common::fs_data::FooSflCfgInfo;
use common::fs_util::foo_core;
use common::fwk::CfgRefCellRc;
use std::rc::Rc;

pub type FooSflT = Rc<dyn Fn() -> String>;

pub type FooSflCfg = CfgRefCellRc<FooSflCfgInfo>;

#[derive(Clone)]
pub struct FooSflDeps {
    pub bar_bf: Rc<dyn Fn() -> String>,
}

pub fn foo_sfl_c(cfg: FooSflCfg, deps: FooSflDeps) -> FooSflT {
    let f = move || {
        let cfg = cfg.get_cfg();
        let a = cfg.a.clone();
        let b = cfg.b;
        let bar_bf = &deps.bar_bf;
        let bar_ret = bar_bf();
        foo_core(a, b, bar_ret)
    };
    Rc::new(f)
}
